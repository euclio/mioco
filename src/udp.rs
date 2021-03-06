use super::{RW, RcEvented, Evented};
use super::prv::EventedPrv;
use super::mio_orig;
use std::io;
use std::net::SocketAddr;
use std::os::unix::io::{RawFd, FromRawFd, AsRawFd};

pub use mio_orig::IpAddr;

/// Udp Socket
pub struct UdpSocket(RcEvented<mio_orig::udp::UdpSocket>);

impl EventedPrv for UdpSocket {
    type Raw = mio_orig::udp::UdpSocket;

    fn shared(&self) -> &RcEvented<Self::Raw> {
        &self.0
    }
}

impl Evented for UdpSocket {}

impl UdpSocket {
    /// Return a new unbound IPv4 UDP Socket.
    pub fn v4() -> io::Result<Self> {
        mio_orig::udp::UdpSocket::v4().map(|t| UdpSocket(RcEvented::new(t)))
    }

    /// Return a new unbound IPv6 UDP Socket.
    pub fn v6() -> io::Result<Self> {
        mio_orig::udp::UdpSocket::v6().map(|t| UdpSocket(RcEvented::new(t)))
    }

    /// Return a new bound UDP Socket.
    pub fn bound(addr: &SocketAddr) -> io::Result<Self> {
        mio_orig::udp::UdpSocket::bound(addr).map(|t| UdpSocket(RcEvented::new(t)))
    }

    /// Bind the unbound UDP Socket.
    pub fn bind(&self, addr: &SocketAddr) -> io::Result<()> {
        self.shared().0.borrow().io.bind(addr)

    }

    /// Local address of the Socket.
    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        self.shared().0.borrow().io.local_addr()
    }

    /// Try cloning the socket.
    pub fn try_clone(&self) -> io::Result<UdpSocket> {
        self.shared().0.borrow().io.try_clone().map(|t| UdpSocket(RcEvented::new(t)))
    }

    /// Block on read.
    pub fn read(&mut self, buf: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
        loop {
            let res = self.try_read(buf);

            match res {
                Ok(None) => self.block_on(RW::read()),
                Ok(Some(r)) => {
                    return Ok(r);
                }
                Err(e) => return Err(e),
            }
        }
    }

    /// Try reading data into a buffer.
    ///
    /// This will not block.
    pub fn try_read(&mut self, buf: &mut [u8]) -> io::Result<Option<(usize, SocketAddr)>> {
        self.shared().0.borrow().io.recv_from(buf)
    }

    /// Block on write.
    pub fn write(&mut self, buf: &[u8], target: &SocketAddr) -> io::Result<usize> {
        loop {
            let res = self.try_write(buf, target);

            match res {
                Ok(None) => self.block_on(RW::write()),
                Ok(Some(r)) => {
                    return Ok(r);
                }
                Err(e) => return Err(e),
            }
        }
    }

    /// Try writing a data from the buffer.
    ///
    /// This will not block.
    pub fn try_write(&self, buf: &[u8], target: &SocketAddr) -> io::Result<Option<usize>> {
        self.shared().0.borrow().io.send_to(buf, target)
    }

    /// Set broadcast flag.
    pub fn set_broadcast(&self, on: bool) -> io::Result<()> {
        self.shared().0.borrow().io.set_broadcast(on)
    }

    /// Set multicast loop flag.
    pub fn set_multicast_loop(&self, on: bool) -> io::Result<()> {
        self.shared().0.borrow().io.set_multicast_loop(on)
    }

    /// Join multicast.
    pub fn join_multicast(&self, multi: &IpAddr) -> io::Result<()> {
        self.shared().0.borrow().io.join_multicast(multi)
    }

    /// Leave multicast.
    pub fn leave_multicast(&self, multi: &IpAddr) -> io::Result<()> {
        self.shared().0.borrow().io.leave_multicast(multi)
    }

    /// Set multicast TTL.
    pub fn set_multicast_time_to_live(&self, ttl: i32) -> io::Result<()> {
        self.shared().0.borrow().io.set_multicast_time_to_live(ttl)
    }
}

impl FromRawFd for UdpSocket {
    unsafe fn from_raw_fd(fd: RawFd) -> UdpSocket {
        UdpSocket(RcEvented::new(mio_orig::udp::UdpSocket::from_raw_fd(fd)))
    }
}

impl AsRawFd for UdpSocket {
    fn as_raw_fd(&self) -> RawFd {
        self.shared().0.borrow_mut().io.as_raw_fd()
    }
}

unsafe impl Send for UdpSocket {}
