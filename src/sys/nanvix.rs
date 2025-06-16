#![allow(unused_variables)]

use crate::{SockAddr, TcpKeepalive};
use ::std::marker::PhantomData;
use ::std::mem::MaybeUninit;
use ::std::net::Shutdown;
use ::std::net::{Ipv4Addr, Ipv6Addr};
use ::std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd};
use ::std::path::Path;
use ::std::time::Duration;
use ::std::{io, slice};

// The location of the following re-exports should not change.
pub use ::std::os::nanvix::ffi::c_int;

// The location of the following re-exports are likely to change.
pub use ::std::os::nanvix::syscall::arpa::inet::in_addr;
// pub use ::std::os::nanvix::syscall::arpa::inet::in_addr6;
pub use ::std::os::nanvix::syscall::netinet::in_::bindings::in6_addr;
pub use ::std::os::nanvix::syscall::netinet::in_::bindings::ipproto::IPPROTO_ICMP;
pub use ::std::os::nanvix::syscall::netinet::in_::bindings::sockaddr_in6;
pub use ::std::os::nanvix::syscall::netinet::in_::bindings::IP_MULTICAST_IF;
pub use ::std::os::nanvix::syscall::netinet::in_::bindings::IP_MULTICAST_LOOP;
pub use ::std::os::nanvix::syscall::netinet::in_::bindings::IP_MULTICAST_TTL;
pub use ::std::os::nanvix::syscall::netinet::in_::bindings::IP_TTL;
// pub use ::std::os::nanvix::syscall::netinet::in_::bindings::ipproto::IPPROTO_ICMPV6;
pub use ::std::os::nanvix::syscall::netinet::in_::bindings::ipproto::IPPROTO_IP;
pub use ::std::os::nanvix::syscall::netinet::in_::bindings::ipproto::IPPROTO_TCP;
pub use ::std::os::nanvix::syscall::netinet::in_::bindings::ipproto::IPPROTO_UDP;
pub use ::std::os::nanvix::syscall::netinet::in_::bindings::sockaddr_in;
pub use ::std::os::nanvix::syscall::netinet::tcp::TCP_NODELAY;
pub use ::std::os::nanvix::syscall::sys::socket::family::AF_INET;
pub use ::std::os::nanvix::syscall::sys::socket::family::AF_INET6;
pub use ::std::os::nanvix::syscall::sys::socket::family::AF_UNIX;
pub use ::std::os::nanvix::syscall::sys::socket::linger;
pub use ::std::os::nanvix::syscall::sys::socket::sa_family_t;
pub use ::std::os::nanvix::syscall::sys::socket::sockaddr_storage;
pub use ::std::os::nanvix::syscall::sys::socket::socklen_t;
pub use ::std::os::nanvix::syscall::sys::socket::MSG_OOB;
pub use ::std::os::nanvix::syscall::sys::socket::MSG_PEEK;
pub use ::std::os::nanvix::syscall::sys::socket::SOCK_DGRAM;
pub use ::std::os::nanvix::syscall::sys::socket::SOCK_STREAM;
pub use ::std::os::nanvix::syscall::sys::socket::SOL_SOCKET;
pub use ::std::os::nanvix::syscall::sys::socket::SO_BROADCAST;
pub use ::std::os::nanvix::syscall::sys::socket::SO_ERROR;
pub use ::std::os::nanvix::syscall::sys::socket::SO_KEEPALIVE;
pub use ::std::os::nanvix::syscall::sys::socket::SO_LINGER;
pub use ::std::os::nanvix::syscall::sys::socket::SO_RCVBUF;
pub use ::std::os::nanvix::syscall::sys::socket::SO_RCVTIMEO;
pub use ::std::os::nanvix::syscall::sys::socket::SO_REUSEADDR;
pub use ::std::os::nanvix::syscall::sys::socket::SO_SNDBUF;
pub use ::std::os::nanvix::syscall::sys::socket::SO_SNDTIMEO;
pub use ::std::os::nanvix::syscall::sys::socket::SO_TYPE;
pub use ::std::os::nanvix::syscall::sys::uio::iovec;

pub(crate) type Bool = c_int;

#[repr(transparent)]
pub struct MaybeUninitSlice<'a> {
    vec: iovec,
    _lifetime: PhantomData<&'a mut [MaybeUninit<u8>]>,
}

impl<'a> MaybeUninitSlice<'a> {
    pub(crate) fn new(buf: &'a mut [MaybeUninit<u8>]) -> MaybeUninitSlice<'a> {
        MaybeUninitSlice {
            vec: iovec {
                iov_base: buf.as_mut_ptr().cast(),
                iov_len: buf.len() as u32,
            },
            _lifetime: PhantomData,
        }
    }

    pub(crate) fn as_slice(&self) -> &[MaybeUninit<u8>] {
        unsafe { slice::from_raw_parts(self.vec.iov_base.cast(), self.vec.iov_len as usize) }
    }

    pub(crate) fn as_mut_slice(&mut self) -> &mut [MaybeUninit<u8>] {
        unsafe { slice::from_raw_parts_mut(self.vec.iov_base.cast(), self.vec.iov_len as usize) }
    }
}

pub(crate) type Socket = c_int;

pub(crate) fn getsockopt<T>(fd: Socket, opt: c_int, val: c_int) -> io::Result<T> {
    todo!();
}

pub(crate) unsafe fn setsockopt<T>(
    fd: Socket,
    opt: c_int,
    val: c_int,
    payload: T,
) -> io::Result<()> {
    todo!();
}

pub(crate) fn from_in_addr(in_addr: in_addr) -> Ipv4Addr {
    Ipv4Addr::from(in_addr.s_addr.to_ne_bytes())
}
// pub(crate) fn from_in6_addr(addr: in6_addr) -> Ipv6Addr {
//     Ipv6Addr::from(addr.s6_addr)
// }

impl FromRawFd for crate::Socket {
    unsafe fn from_raw_fd(fd: c_int) -> crate::Socket {
        crate::Socket::from_raw(fd)
    }
}
pub(crate) fn listen(fd: Socket, backlog: c_int) -> io::Result<()> {
    todo!()
}

pub(crate) fn accept(fd: Socket) -> io::Result<(Socket, SockAddr)> {
    todo!()
}

pub(crate) fn getsockname(fd: Socket) -> io::Result<SockAddr> {
    todo!()
}

pub(crate) const fn to_in_addr(addr: &Ipv4Addr) -> in_addr {
    // `s_addr` is stored as BE on all machines, and the array is in BE order.
    // So the native endian conversion method is used so that it's never
    // swapped.
    in_addr {
        s_addr: u32::from_ne_bytes(addr.octets()),
    }
}

pub(crate) fn socket_as_raw(socket: &crate::socket::Inner) -> Socket {
    socket.as_raw_fd()
}

impl IntoRawFd for crate::Socket {
    fn into_raw_fd(self) -> c_int {
        self.into_raw()
    }
}

pub(crate) unsafe fn socket_from_raw(socket: Socket) -> crate::socket::Inner {
    crate::socket::Inner::from_raw_fd(socket)
}
pub(crate) fn socket_into_raw(socket: crate::socket::Inner) -> Socket {
    socket.into_raw_fd()
}

pub(crate) fn socket(family: c_int, ty: c_int, protocol: c_int) -> io::Result<Socket> {
    todo!()
}

pub(crate) fn bind(fd: Socket, addr: &SockAddr) -> io::Result<()> {
    todo!()
}
pub(crate) fn peek_sender(fd: Socket) -> io::Result<SockAddr> {
    todo!()
}

pub(crate) fn poll_connect(socket: &crate::Socket, timeout: Duration) -> io::Result<()> {
    todo!()
}

pub(crate) fn getpeername(fd: Socket) -> io::Result<SockAddr> {
    todo!()
}

pub(crate) fn connect(fd: Socket, addr: &SockAddr) -> io::Result<()> {
    todo!()
}

pub(crate) fn recv(fd: Socket, buf: &mut [MaybeUninit<u8>], flags: c_int) -> io::Result<usize> {
    todo!()
}

pub(crate) fn try_clone(fd: Socket) -> io::Result<Socket> {
    todo!()
}

pub(crate) fn unix_sockaddr(path: &Path) -> io::Result<SockAddr> {
    todo!()
}

pub(crate) fn recv_from(
    fd: Socket,
    buf: &mut [MaybeUninit<u8>],
    flags: c_int,
) -> io::Result<(usize, SockAddr)> {
    todo!()
}

pub(crate) fn from_in6_addr(addr: in6_addr) -> Ipv6Addr {
    Ipv6Addr::from(addr.s6_addr)
}

pub(crate) fn shutdown(fd: Socket, how: Shutdown) -> io::Result<()> {
    todo!()
}

pub(crate) fn send_to(fd: Socket, buf: &[u8], addr: &SockAddr, flags: c_int) -> io::Result<usize> {
    todo!()
}

pub(crate) fn timeout_opt(fd: Socket, opt: c_int, val: c_int) -> io::Result<Option<Duration>> {
    todo!()
}
pub(crate) fn set_timeout_opt(
    fd: Socket,
    opt: c_int,
    val: c_int,
    duration: Option<Duration>,
) -> io::Result<()> {
    todo!()
}

pub(crate) fn set_nonblocking(fd: Socket, nonblocking: bool) -> io::Result<()> {
    todo!()
}

pub(crate) fn send(fd: Socket, buf: &[u8], flags: c_int) -> io::Result<usize> {
    todo!()
}

pub(crate) fn set_tcp_keepalive(fd: Socket, keepalive: &TcpKeepalive) -> io::Result<()> {
    if let Some(interval) = keepalive.interval {
        todo!()
    }

    if let Some(retries) = keepalive.retries {
        todo!()
    }
    todo!()
}

pub(crate) const fn to_in6_addr(addr: &Ipv6Addr) -> in6_addr {
    in6_addr {
        s6_addr: addr.octets(),
    }
}
