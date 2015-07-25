use std::{error, io};
use std::result::Result;

use {EventLoop, EventSet, Token};

#[allow(unused_variables)]
pub trait HandlerExt {
    type Timeout;
    type Message: Send;
    type Error: error::Error + From<io::Error> = io::Error;

    /// Invoked when the socket represented by `token` is ready to be operated
    /// on. `events` indicates the specific operations that are
    /// ready to be performed.
    ///
    /// For example, when a TCP socket is ready to be read from, `events` will
    /// have `readable` set. When the socket is ready to be written to,
    /// `events` will have `writable` set.
    ///
    /// This function will only be invoked a single time per socket per event
    /// loop tick.
    fn ready(&mut self, event_loop: &mut EventLoop<Self>, token: Token, events: EventSet) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Invoked when a message has been received via the event loop's channel.
    fn notify(&mut self, event_loop: &mut EventLoop<Self>, msg: Self::Message) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Invoked when a timeout has completed.
    fn timeout(&mut self, event_loop: &mut EventLoop<Self>, timeout: Self::Timeout) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Invoked when `EventLoop` has been interrupted by a signal interrupt.
    fn interrupted(&mut self, event_loop: &mut EventLoop<Self>) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[allow(unused_variables)]
pub trait Handler: Sized {
    type Timeout;
    type Message: Send;

    fn ready(&mut self, event_loop: &mut EventLoop<Self>, token: Token, events: EventSet) {
    }

    fn notify(&mut self, event_loop: &mut EventLoop<Self>, msg: Self::Message) {
    }

    fn timeout(&mut self, event_loop: &mut EventLoop<Self>, timeout: Self::Timeout) {
    }

    fn interrupted(&mut self, event_loop: &mut EventLoop<Self>) {
    }
}

impl <T, M, S> HandlerExt for S
where M: Send, S: Handler<Timeout=T, Message=M> {
    type Timeout = T;
    type Message = M;

    fn ready(&mut self, event_loop: &mut EventLoop<Self>, token: Token, events: EventSet) -> io::Result<()> {
        Handler::ready(self, event_loop, token, events);
        Ok(())
    }

    fn notify(&mut self, event_loop: &mut EventLoop<Self>, msg: M) -> io::Result<()> {
        Handler::notify(self, event_loop, msg);
        Ok(())
    }

    fn timeout(&mut self, event_loop: &mut EventLoop<Self>, timeout: T) -> io::Result<()> {
        Handler::timeout(self, event_loop, timeout);
        Ok(())
    }

    fn interrupted(&mut self, event_loop: &mut EventLoop<Self>) -> io::Result<()> {
        Handler::interrupted(self, event_loop);
        Ok(())
    }
}
