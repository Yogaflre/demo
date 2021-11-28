use crate::{Interest, Token};
use std::{
    io,
    net::TcpStream,
    os::unix::{io::RawFd, prelude::AsRawFd},
    time::Duration,
};

#[derive(Debug)]
pub struct Registry {
    pub selector: Selector,
}

impl Registry {
    pub fn new() -> io::Result<Self> {
        return Selector::new().map(|selector| Self { selector });
    }

    /*
     * register event into epoll queue.
     */
    pub fn register(
        &mut self,
        stream: &TcpStream,
        token: Token,
        interest: Interest,
    ) -> io::Result<()> {
        let fd = stream.as_raw_fd();
        let mut event = Event::new(interest, token);
        return epoll_ctl(self.selector.epfd, ffi::EPOLL_CTL_ADD, fd, &mut event);
    }
}

#[derive(Debug)]
pub struct Selector {
    epfd: RawFd, // epoll queue
}

impl Drop for Selector {
    fn drop(&mut self) {
        close(self.epfd).unwrap();
    }
}

impl Selector {
    pub fn new() -> io::Result<Self> {
        return epoll_create().map(|epfd| Self { epfd });
    }

    pub fn select(&self, events: &mut Vec<Event>, timeout: Option<Duration>) -> io::Result<()> {
        events.clear(); // make sure events is empty array.
        return epoll_wait(self.epfd, events, 1024, timeout).map(|active_event_size| {
            unsafe { events.set_len(active_event_size) }; // ffi might have some magic in array, reset size.
        });
    }
}

/*
 * ffi wapper
 */
pub type Event = ffi::Event;
impl Event {
    fn new(interest: Interest, token: Token) -> Self {
        if interest.is_readable() {
            return Self {
                events: ffi::EPOLLIN | ffi::EPOLLONESHOT,
                epoll_data: token,
            };
        } else if interest.is_writable() {
            // TODO more events
            unimplemented!();
        } else {
            unimplemented!();
        }
    }

    pub fn token(&self) -> Token {
        return self.epoll_data;
    }
}

fn epoll_create() -> io::Result<i32> {
    match unsafe { ffi::epoll_create(1) } {
        res if res >= 0 => Ok(res),
        res => Err(io::Error::from_raw_os_error(res)),
    }
}
fn close(epfd: RawFd) -> io::Result<i32> {
    match unsafe { ffi::close(epfd) } {
        res if res >= 0 => Ok(res),
        res => Err(io::Error::from_raw_os_error(res)),
    }
}
fn epoll_ctl(epfd: RawFd, op: i32, fd: RawFd, events: *mut ffi::Event) -> io::Result<()> {
    match unsafe { ffi::epoll_ctl(epfd, op, fd, events) } {
        res if res >= 0 => Ok(()),
        res => Err(io::Error::from_raw_os_error(res)),
    }
}
fn epoll_wait(
    epfd: RawFd,
    events: &mut Vec<Event>,
    maxevents: i32,
    timeout: Option<Duration>,
) -> io::Result<usize> {
    let timeout = timeout.map(|t| t.as_millis() as i32).unwrap_or(-1);
    match unsafe { ffi::epoll_wait(epfd, events.as_mut_ptr(), maxevents, timeout) } {
        res if res >= 0 => Ok(res as usize),
        res => Err(io::Error::from_raw_os_error(res)),
    }
}

/*
 * FFI
 */
mod ffi {
    pub const EPOLL_CTL_ADD: i32 = 1;
    pub const EPOLLIN: u32 = 0x001; // read event.
    pub const EPOLLONESHOT: u32 = 0x40000000; // remove fd from epoll queue after notified.

    #[derive(Debug, Clone, Copy)]
    #[repr(C, packed)]
    pub struct Event {
        pub events: u32,
        pub epoll_data: usize, // user flag.
    }

    #[link(name = "c")]
    extern "C" {
        pub(super) fn epoll_create(size: i32) -> i32;
        pub(super) fn close(fd: i32) -> i32;
        pub(super) fn epoll_ctl(epfd: i32, op: i32, fd: i32, event: *mut Event) -> i32;
        pub(super) fn epoll_wait(
            epfd: i32,
            events: *mut Event,
            maxevents: i32,
            timeout: i32,
        ) -> i32;
    }
}
