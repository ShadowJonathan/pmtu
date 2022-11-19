use std::{net::UdpSocket, os::unix::prelude::AsRawFd};

use nix::sys::socket::setsockopt;

pub enum PTMUEstimate {
    PathMTU(u16),
    LocalMTU(u16),
    CannotDetermine,
}

pub fn get_estimate(socket: &UdpSocket) -> PTMUEstimate {
    todo!()
}

#[test]
fn garbage() {
    use assert_matches::assert_matches;

    let sock = UdpSocket::bind("0.0.0.0:0").unwrap();
    dbg!(&sock);

    // assert_matches!(setsockopt(sock.as_raw_fd(), IpDontFrag, &true), Ok(()));

    sock.connect("1.0.0.1:52").unwrap();

    const PACKET: [u8; 2000] = [0; 2000];

    assert_matches!(sock.send(&PACKET), Ok(2000));
}
