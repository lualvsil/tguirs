use crate::{items::*, *};
use nix::sys::socket::{
	recv, MsgFlags
};

pub struct HardwareBuffer {
	pub id: i32,
	pub buffer: *mut hb::AHardwareBuffer,
	pub width: i32,
	pub height: i32,
}

impl Connection {
	pub fn new_hardware_buffer(&self, width: i32, height: i32) -> Res<HardwareBuffer> {
		let req = CreateHardwareBufferRequest {
			width,
			height,
			format: create_hardware_buffer_request::Format::Rgba8888.into(),
			cpu_read: create_hardware_buffer_request::CpuUsage::Rarely.into(),
			cpu_write: create_hardware_buffer_request::CpuUsage::Rarely.into(),
		};
		
		self.send_msg(items::method::Method::CreateHardwareBuffer(req))?;
		
		// Receive id
		let mut bytes = [0u8; 4];
		let mut start = 0;
		while start < bytes.len() {
			let n = recv(self.main, &mut bytes[start..], MsgFlags::empty())?;
			start += n;
		}
		let id = i32::from_be_bytes(bytes);
		if id <= 0 {
			return Err(TguiErr::Msg("new_hardware_buffer"));
		}
		
		// Receive buffer
		let buffer = hb::Lib::recv(self.main);
		if buffer.is_none() {
			return Err(TguiErr::Msg("failed to receive AHardwareBuffer"));
		}
		
		Ok(HardwareBuffer{
			id,
			buffer: buffer.unwrap(),
			width,
			height,
		})
	}
}