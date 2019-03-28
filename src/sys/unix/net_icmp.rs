
// use std::net::IpAddr;
use std::io::{Result, ErrorKind};
use std::mem;
use std::fmt;
use std::net::{self, IpAddr,Ipv4Addr, Ipv6Addr, SocketAddr};

use libc as c;
use std::os::unix::io::{RawFd, IntoRawFd, AsRawFd, FromRawFd};

use super::compat::{IntoInner, FromInner, AsInner, cvt, setsockopt, getsockopt, ioctl};

// Following constants are not defined in libc (as for 0.2.17 version)
const IPPROTO_ICMP: c::c_int = 1;
// Ipv4
const IP_TOS: c::c_int = 1;
// Ipv6
const IPV6_UNICAST_HOPS: c::c_int = 16;
const IPV6_TCLASS: c::c_int = 67;

#[cfg(target_os = "linux")]
use libc::SOCK_CLOEXEC;
#[cfg(not(target_os = "linux"))]
const SOCK_CLOEXEC: c::c_int = 0;


pub struct IcmpSocket {
  pub  fd: c::c_int,
    family: c::c_int,
    peer: c::sockaddr,
    local_addr: IpAddr,
}

impl IcmpSocket {

    pub fn connect(&self,sock_addr: SocketAddr) -> Result<()> {
      Ok(())
      // Ok((sock_addr,sock_addr.ip()))
        // let addr = sock_addr.ip();
        // let family = match addr {
        //     IpAddr::V4(..) => c::AF_INET,
        //     IpAddr::V6(..) => c::AF_INET6,
        // };

        // let fd = unsafe {
        //     cvt(c::socket(family, c::SOCK_RAW | SOCK_CLOEXEC, IPPROTO_ICMP))?
        // };
        // Ok(IcmpSocket {
        //     fd: fd,
        //     family: family,
        //     peer: addr.into_inner(),
        //     local_addr: addr,
        // })
    }
    pub fn bind(sock_addr: &SocketAddr) -> Result<IcmpSocket> {
        let addr = sock_addr.ip();

        let family = match addr {
            IpAddr::V4(..) => c::AF_INET,
            IpAddr::V6(..) => c::AF_INET6,
        };

        let fd = unsafe {
            cvt(c::socket(family, c::SOCK_RAW | SOCK_CLOEXEC, IPPROTO_ICMP))?
        };
        let peer = addr.into_inner();
        unsafe {
            cvt(c::bind(fd, &peer,mem::size_of_val(&peer) as c::socklen_t));
        }

        Ok(IcmpSocket {
            fd: fd,
            family: family,
            peer: addr.into_inner(),
            local_addr: addr,
        })
    }
    pub fn local_addr(&self) -> Result<SocketAddr>{
      Ok(SocketAddr::new(self.local_addr,1))
    }
    pub fn try_clone(&self) -> Result<IcmpSocket> {
        Ok(IcmpSocket {
            fd: self.fd,
            family: self.family,
            peer: self.peer,
            local_addr: self.local_addr,
        })
    }

    pub fn recv(&self, buf: &mut [u8]) -> Result<usize> {
        let ret = unsafe {
            cvt(c::recv(
                    self.fd,
                    buf.as_mut_ptr() as *mut c::c_void,
                    buf.len() as c::size_t,
                    0,
            ))
        };

        match ret {
            Ok(size) => Ok(size as usize),
            Err(ref err) if err.kind() == ErrorKind::Interrupted => Ok(0),
            Err(err) => Err(err),
        }
    }

    pub fn recv_from(&self, buf: &mut [u8]) -> Result<(usize, SocketAddr)> {
        let mut peer: c::sockaddr = unsafe { mem::uninitialized() };
        let ret = unsafe {
            cvt(c::recvfrom(
                    self.fd,
                    buf.as_mut_ptr() as *mut c::c_void,
                    buf.len() as c::size_t,
                    0,
                    &mut peer,
                    &mut (mem::size_of_val(&peer) as c::socklen_t)
                )
            )
        };

        match ret { 
            Ok(size) => Ok((size as usize,SocketAddr::new(IpAddr::from_inner(peer),0) )),
            Err(ref err) if err.kind() == ErrorKind::Interrupted => Ok((0, SocketAddr::new(IpAddr::from_inner(peer),0))),
            Err(err) => Err(err),
        }
    }

    pub fn send(&self, buf: &[u8]) -> Result<usize> {
        let ret = unsafe {
            cvt(c::sendto(
                    self.fd,
                    buf.as_ptr() as *mut c::c_void,
                    buf.len() as c::size_t,
                    0,
                    &self.peer,
                    mem::size_of_val(&self.peer) as c::socklen_t,
                )
            )?
        };

        Ok(ret as usize)
    }
    pub fn send_to(&self, buf: &[u8],target: &SocketAddr) -> Result<usize> {
        let mut peer = target.ip().into_inner();
        
        let ret = unsafe {
            cvt(c::sendto(
                    self.fd,
                    buf.as_ptr() as *mut c::c_void,
                    buf.len() as c::size_t,
                    0,
                    &peer,
                    mem::size_of_val(&peer) as c::socklen_t,
                )
            )?
        };

        Ok(ret as usize)
    }

    pub fn set_ttl(&self, ttl: u32) -> Result<()> {
        match self.family {
            c::AF_INET => setsockopt(self, c::IPPROTO_IP, c::IP_TTL, ttl as c::c_int),
            c::AF_INET6 => setsockopt(self, c::IPPROTO_IPV6, IPV6_UNICAST_HOPS, ttl as c::c_int),
            _ => unreachable!(),
        }
    }
    pub fn set_nonblocking(&self, flag: i32) -> Result<()> {
        match self.family {
            c::AF_INET => ioctl(self, c::FIONBIO as u64, flag as c::c_int),
            c::AF_INET6 => ioctl(self, c::FIONBIO as u64, flag as c::c_int),
            _ => unreachable!(),
        }
    }

    pub fn ttl(&self) -> Result<u32> {
        match self.family {
            c::AF_INET => getsockopt(self, c::IPPROTO_IP, c::IP_TTL),
            c::AF_INET6 => getsockopt(self, c::IPPROTO_IPV6, IPV6_UNICAST_HOPS),
            _ => unreachable!(),
        }
    }

    pub fn set_broadcast(&self, broadcast: bool) -> Result<()> {
        setsockopt(&self, c::SOL_SOCKET, c::SO_BROADCAST, broadcast as c::c_int)
    }

    pub fn broadcast(&self) -> Result<bool> {
        let raw: c::c_int = getsockopt(&self, c::SOL_SOCKET, c::SO_BROADCAST)?;
        Ok(raw != 0)
    }

    pub fn set_qos(&self, qos: u8) -> Result<()> {
        match self.family {
            c::AF_INET => setsockopt(&self, c::IPPROTO_IP, IP_TOS, qos as c::c_int),
            c::AF_INET6 => setsockopt(&self, c::IPPROTO_IPV6, IPV6_TCLASS, qos as c::c_int),
            _ => unreachable!(),
        }
    }

    pub fn qos(&self) -> Result<u8> {
        match self.family {
            c::AF_INET => getsockopt(&self, c::IPPROTO_IP, IP_TOS),
            c::AF_INET6 => getsockopt(&self, c::IPPROTO_IPV6, IPV6_TCLASS),
            _ => unreachable!(),
        }
    }

}

impl Drop for IcmpSocket {
    fn drop(&mut self) {
        let _ = unsafe {
            c::close(self.fd)
        };
    }
}

impl AsInner<c::c_int> for IcmpSocket {
    fn as_inner(&self) -> &c::c_int {
        &self.fd
    }
}

impl fmt::Debug for IcmpSocket {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.fd, f)
    }
}

impl FromRawFd for IcmpSocket {
    unsafe fn from_raw_fd(fd: RawFd) -> IcmpSocket {
        IcmpSocket {
            fd: fd,
            family: c::AF_INET,
            peer: IpAddr::V4(Ipv4Addr::new(127,0,0,1)).into_inner(),
            local_addr: IpAddr::V4(Ipv4Addr::new(127,0,0,1)),
        }
    }
}

impl IntoRawFd for IcmpSocket {
    fn into_raw_fd(self) -> RawFd {
        self.fd
    }
}

impl AsRawFd for IcmpSocket {
    fn as_raw_fd(&self) -> RawFd {
        self.fd
    }
}