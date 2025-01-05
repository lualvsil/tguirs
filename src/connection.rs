use crate::*;

use std::{
	os::{fd::AsRawFd, unix::io::{RawFd}},
	process::{Command, Stdio}
};
use nix::sys::socket::{
    accept, bind, listen, recv, send, socket, AddressFamily, MsgFlags, SockFlag, SockType, UnixAddr, Backlog,
};
use rand::{distributions::{Alphanumeric, DistString}, thread_rng};
use prost::Message;

#[derive(Debug, Clone)]
pub struct Connection {
	pub main: RawFd,
	pub event: RawFd,
}

impl Connection {
	pub fn new() -> Res<Self> {
		let main_addr = Alphanumeric.sample_string(&mut thread_rng(), 50);
		let event_addr = Alphanumeric.sample_string(&mut thread_rng(), 50);
		
		let main_sockaddr = UnixAddr::new_abstract(main_addr.as_bytes())?;
		let event_sockaddr = UnixAddr::new_abstract(event_addr.as_bytes())?;
		
		let main_socket = socket(
			AddressFamily::Unix,
			SockType::Stream,
			SockFlag::empty(),
			None
		)?;
		let event_socket = socket(
			AddressFamily::Unix,
			SockType::Stream,
			SockFlag::empty(),
			None
		)?;
		
		bind(main_socket.as_raw_fd(), &main_sockaddr)?;
		bind(event_socket.as_raw_fd(), &event_sockaddr)?;
		
		listen(&main_socket, Backlog::new(1)?)?;
		listen(&event_socket, Backlog::new(1)?)?;
		
		Command::new("am")
			.stdout(Stdio::null())
			.stderr(Stdio::null())
			.args([
				"broadcast", "-n", "com.termux.gui/.GUIReceiver",
				"--es", "mainSocket", &main_addr,
				"--es", "eventSocket", &event_addr,
			])
			.status()?;
		
		let main = accept(main_socket.as_raw_fd())?;
		let event = accept(event_socket.as_raw_fd())?;
		
		// Protocol
		//   0: Protobuf
		//   1: Json
		let mut protocol = [0u8];
		
		while send(main, &protocol, MsgFlags::empty())? == 0 {}
		protocol = [0u8];
		while recv(main, &mut protocol, MsgFlags::empty())? == 0 {}
		
		Ok(Self{
			main,
			event,
		})
	}
	
	pub fn send_msg(&self, method: items::method::Method) -> Res<()> {
		let msg = items::Method{method: Some(method)}.encode_length_delimited_to_vec();
		
		let bytes = msg.as_slice();
		
		let mut total = 0;
		while total < bytes.len() {
			let n = send(self.main, &bytes[total..], MsgFlags::empty())?;
			total += n;
		}
		
		Ok(())
	}
	pub fn recv_msg<T: Message+Default>(&self) -> Res<T> {
		let msg_size = recv_size(self.main)?;
		let mut msg = vec![0u8; msg_size];
		let mut total = 0;
		while total < msg_size {
			let n = recv(self.main, &mut msg[total..], MsgFlags::empty())?;
			total += n;
		}
		Ok(Message::decode(msg.as_slice())?)
	}
	
	pub fn send_recv_msg<T: Message+Default>(&self, method: items::method::Method) -> Res<T> {
		self.send_msg(method)?;
		self.recv_msg()
	}
	
	pub fn recv_event(&self) -> Res<items::Event> {
		let msg_size = recv_size(self.event)?;
		let mut msg = vec![0u8; msg_size];
		let mut total = 0;
		while total < msg_size {
			let n = recv(self.event, &mut msg[total..], MsgFlags::empty())?;
			total += n;
		}
		Ok(Message::decode(msg.as_slice())?)
	}
	
	pub fn toast(&self, text: String, long: bool) -> Res<()> {
		let req = items::ToastRequest {
			text,
			long,
		};
		self.send_msg(items::method::Method::Toast(req))?;
		Ok(())
	}
}

// REFS: https://docs.rs/delimited-protobuf/latest/src/delimited_protobuf/lib.rs.html#5-14
//       https://github.com/rsuu/tgui
#[inline]
fn recv_size(fd: RawFd) -> Res<usize> {
	let mut buf = [0_u8; 1];
	let mut num_bits_read = 0;
	let mut val: u32 = 0;
	let mut is_last: bool = false;
	let mut byte: u32;

	while !is_last {
		'l2: loop {
			if recv(fd, &mut buf, MsgFlags::empty()).is_ok() {
				break 'l2;
			}
		}

		byte = buf[0] as u32;
		is_last = byte >> 7 == 0;
		byte &= 0b0111_1111;

		byte = byte
			.checked_shl(num_bits_read)
			.expect("too many bytes for u32");
		val |= byte;
		num_bits_read += 7;
	}

	if val == 0 {
		Err(TguiErr::ProtoZeroLen)
	} else {
		Ok(val as usize)
	}
}