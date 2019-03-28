use {io, Ready, Poll, PollOpt, Token};
use event::Evented;
use unix::EventedFd;
use std::fmt;
use std::net::{self, Ipv4Addr, Ipv6Addr, SocketAddr};
use std::os::unix::io::{RawFd, IntoRawFd, AsRawFd, FromRawFd};

// #[allow(unused_imports)] // only here for Rust 1.8
// use net2::UdpSocketExt;
// use unix::net_icmp;
use super::net_icmp;



pub struct IcmpSocket {
    io: net_icmp::IcmpSocket,
}

impl IcmpSocket {
    pub fn new(socket: net_icmp::IcmpSocket) -> io::Result<IcmpSocket> {
        // socket.set_nonblocking(1)?;
        Ok(IcmpSocket {
            io: socket,
        })
    }

    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        self.io.local_addr()
    }

    pub fn try_clone(&self) -> io::Result<IcmpSocket> {
        self.io.try_clone().map(|io| {
            IcmpSocket {
                io,
            }
        })
    }

    pub fn send_to(&self, buf: &[u8], target: &SocketAddr) -> io::Result<usize> {
        self.io.send_to(buf, target)
    }

    pub fn recv_from(&self, buf: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
        self.io.recv_from(buf)
    }

    pub fn send(&self, buf: &[u8]) -> io::Result<usize> {
        self.io.send(buf)
    }

    pub fn recv(&self, buf: &mut [u8]) -> io::Result<usize> {
        self.io.recv(buf)
    }

    pub fn connect(&self, addr: SocketAddr)
                     -> io::Result<()> {
        self.io.connect(addr)
    }

    pub fn broadcast(&self) -> io::Result<bool> {
        self.io.broadcast()
    }

    pub fn set_broadcast(&self, on: bool) -> io::Result<()> {
        self.io.set_broadcast(on)
    }

    pub fn multicast_loop_v4(&self) -> io::Result<bool> {
        // self.io.multicast_loop_v4()
        Ok(true)
    }

    pub fn set_multicast_loop_v4(&self, on: bool) -> io::Result<()> {
        // self.io.set_multicast_loop_v4(on)
        Ok(())
    }

    pub fn multicast_ttl_v4(&self) -> io::Result<u32> {
        // self.io.multicast_ttl_v4()
        Ok(1)
    }

    pub fn set_multicast_ttl_v4(&self, ttl: u32) -> io::Result<()> {
        // self.io.set_multicast_ttl_v4(ttl)
        Ok(())
    }

    pub fn multicast_loop_v6(&self) -> io::Result<bool> {
        // self.io.multicast_loop_v6()
        Ok(true)
    }

    pub fn set_multicast_loop_v6(&self, on: bool) -> io::Result<()> {
        // self.io.set_multicast_loop_v6(on)
        Ok(())
    }

    pub fn ttl(&self) -> io::Result<u32> {
        self.io.ttl()
    }

    pub fn set_nonblocking(&self,opt: i32) -> io::Result<()> {
        self.io.set_nonblocking(opt)
    }

    pub fn set_ttl(&self, ttl: u32) -> io::Result<()> {
        self.io.set_ttl(ttl)
    }

    pub fn join_multicast_v4(&self,
                             multiaddr: &Ipv4Addr,
                             interface: &Ipv4Addr) -> io::Result<()> {
        // self.io.join_multicast_v4(multiaddr, interface)
        Ok(())

    }

    pub fn join_multicast_v6(&self,
                             multiaddr: &Ipv6Addr,
                             interface: u32) -> io::Result<()> {
        // self.io.join_multicast_v6(multiaddr, interface)
        Ok(())

    }

    pub fn leave_multicast_v4(&self,
                              multiaddr: &Ipv4Addr,
                              interface: &Ipv4Addr) -> io::Result<()> {
        // self.io.leave_multicast_v4(multiaddr, interface)
        Ok(())

    }

    pub fn leave_multicast_v6(&self,
                              multiaddr: &Ipv6Addr,
                              interface: u32) -> io::Result<()> {
        // self.io.leave_multicast_v6(multiaddr, interface)
        Ok(())

    }

    pub fn set_only_v6(&self, only_v6: bool) -> io::Result<()> {
        // self.io.set_only_v6(only_v6)
        Ok(())

    }

    pub fn only_v6(&self) -> io::Result<bool> {
        // self.io.only_v6()
        Ok(true)

    }

    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        // self.io.take_error()
        Ok(Some(io::Error::new(io::ErrorKind::Other, "oh no!")))
    }
}

impl Evented for IcmpSocket {
    fn register(&self, poll: &Poll, token: Token, interest: Ready, opts: PollOpt) -> io::Result<()> {
        EventedFd(&self.as_raw_fd()).register(poll, token, interest, opts)
    }

    fn reregister(&self, poll: &Poll, token: Token, interest: Ready, opts: PollOpt) -> io::Result<()> {
        EventedFd(&self.as_raw_fd()).reregister(poll, token, interest, opts)
    }

    fn deregister(&self, poll: &Poll) -> io::Result<()> {
        EventedFd(&self.as_raw_fd()).deregister(poll)
    }
}

impl fmt::Debug for IcmpSocket {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.io, f)
    }
}

impl FromRawFd for IcmpSocket {
    unsafe fn from_raw_fd(fd: RawFd) -> IcmpSocket {
        IcmpSocket {
            io: net_icmp::IcmpSocket::from_raw_fd(fd),
        }
    }
}

impl IntoRawFd for IcmpSocket {
    fn into_raw_fd(self) -> RawFd {
        self.io.into_raw_fd()
    }
}

impl AsRawFd for IcmpSocket {
    fn as_raw_fd(&self) -> RawFd {
        self.io.as_raw_fd()
    }
}
