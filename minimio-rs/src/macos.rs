use crate::{Interest, Token};
use std::{
    io,
    net::TcpStream,
    os::unix::{io::RawFd, prelude::AsRawFd},
    ptr,
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
        let events = [Event::new(fd, interest, token)];
        kevent(self.selector.kqfd, &events, &mut [], 0, None)?;
        return Ok(());
    }
}

#[derive(Debug)]
pub struct Selector {
    pub kqfd: RawFd, // epoll ueue
}

impl Drop for Selector {
    fn drop(&mut self) {
        close(self.kqfd).unwrap();
    }
}

impl Selector {
    pub fn new() -> io::Result<Self> {
        return kqueue().map(|kqfd| Self { kqfd });
    }

    pub fn select(&self, events: &mut Vec<Event>, timeout: Option<Duration>) -> io::Result<()> {
        let n_events = events.capacity();
        events.clear(); // make sure events is empty array.
        return kevent(self.kqfd, &[], events, n_events as i32, timeout)
            .map(|active_event_size| unsafe { events.set_len(active_event_size) });
    }
}

/*
 * ffi wapper
 */
pub type Event = ffi::Kevent;
impl Event {
    fn new(fd: RawFd, interest: Interest, token: Token) -> Self {
        if interest.is_readable() {
            return Self {
                ident: fd as u64,
                filter: ffi::EVFILT_READ,
                flags: ffi::EV_ADD | ffi::EV_ENABLE | ffi::EV_ONESHOT,
                fflags: 0,
                data: 0,
                udata: token as u64,
            };
        } else if interest.is_writable() {
            // TODO more events
            unimplemented!();
        } else {
            unimplemented!();
        }
    }

    pub fn token(&self) -> Token {
        return self.udata as usize;
    }
}

fn kqueue() -> io::Result<i32> {
    match unsafe { ffi::kqueue() } {
        res if res >= 0 => Ok(res),
        res => Err(io::Error::from_raw_os_error(res)),
    }
}
fn close(kqfd: RawFd) -> io::Result<i32> {
    match unsafe { ffi::close(kqfd) } {
        res if res >= 0 => Ok(res),
        res => Err(io::Error::from_raw_os_error(res)),
    }
}

fn kevent(
    kqfd: RawFd,
    changelist: &[Event],
    eventlist: &mut [Event],
    nevents: i32,
    timeout: Option<Duration>,
) -> io::Result<usize> {
    let kqfd = kqfd as i32;
    let nchanges = changelist.len() as i32;
    let timespec = match timeout {
        Some(t) => &ffi::Timespec {
            tv_sec: t.as_secs() as isize,
            v_nsec: t.as_millis() as usize,
        },
        None => ptr::null(),
    };

    return match unsafe {
        ffi::kevent(
            kqfd,
            changelist.as_ptr(),
            nchanges,
            eventlist.as_mut_ptr(),
            nevents,
            timespec,
        )
    } {
        res if res >= 0 => Ok(res as usize),
        res => Err(io::Error::from_raw_os_error(res)),
    };
}

/*
 * FFI
 */
mod ffi {
    pub const EV_ADD: u16 = 0x1;
    pub const EV_ENABLE: u16 = 0x4;
    pub const EVFILT_READ: i16 = -1;
    pub const EV_ONESHOT: u16 = 0x10;

    #[derive(Debug, Clone, Default)]
    #[repr(C)]
    pub struct Kevent {
        pub ident: u64,
        pub filter: i16,
        pub flags: u16,
        pub fflags: u32,
        pub data: i64,
        pub udata: u64,
    }

    #[derive(Debug)]
    #[repr(C)]
    pub(super) struct Timespec {
        pub tv_sec: isize,
        pub v_nsec: usize,
    }

    #[link(name = "c")]
    extern "C" {
        pub(super) fn kqueue() -> i32;
        pub(super) fn kevent(
            kq: i32,
            changelist: *const Kevent,
            nchanges: i32,
            eventlist: *mut Kevent,
            nevents: i32,
            timeout: *const Timespec,
        ) -> i32;
        pub(super) fn close(d: i32) -> i32;
    }
}
