#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use linux::{Event, Registry, Selector};

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub use macos::{Event, Registry, Selector};

use std::{io, time::Duration};

/*
 * Base type
 */
pub type Token = usize;

/*
 * Interest event by user
 */
pub struct Interest(u8);
impl Interest {
    pub const READABLE: Interest = Interest(0b0000_0001);
    pub const WRITABLE: Interest = Interest(0b0000_0010);

    pub fn is_readable(&self) -> bool {
        return self.0 == Self::READABLE.0;
    }
    pub fn is_writable(&self) -> bool {
        return self.0 == Self::WRITABLE.0;
    }
}

/*
 * Base api
 */
#[derive(Debug)]
pub struct Poll {
    registry: Registry,
}

impl Poll {
    pub fn new() -> io::Result<Poll> {
        return Registry::new().map(|registry| Self { registry });
    }

    pub fn registry(&mut self) -> &mut Registry {
        return &mut self.registry;
    }

    pub fn poll(&mut self, events: &mut Vec<Event>, timeout: Option<Duration>) -> io::Result<()> {
        return self.registry.selector.select(events, timeout);
    }
}
