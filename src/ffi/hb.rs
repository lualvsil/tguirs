#![allow(non_camel_case_types)]

use nix::libc;
use std::ffi::*;
use std::ptr::null_mut;

const LIBANDROID_SO: &str = "libandroid.so";

#[repr(C)]
pub struct AHardwareBuffer {}

// #[link(name="android")]
// extern "C" {
// 	fn android_get_device_api_level() -> c_int;
// }

pub struct Lib {
	handle: *mut c_void,
	fn_recv: Option<AHardwareBuffer_recvHandleFromUnixSocket_type>,
}

type AHardwareBuffer_recvHandleFromUnixSocket_type = unsafe extern "C" fn(socketFd: c_int, outBuffer: *mut *mut AHardwareBuffer) -> c_int;

static mut ANDROID_LIB: Lib = Lib{
	handle: null_mut::<c_void>(),
	fn_recv: None,
};

impl Lib {
	pub fn init() {
		unsafe {
			if !ANDROID_LIB.handle.is_null() {
				return;
			}
			
			// let apilevel = android_get_device_api_level();
			// if apilevel < 26 {
			// 	panic!("Android API {apilevel} unsupported");
			// }
			
			let libname = CString::new(LIBANDROID_SO).unwrap();
			let handle = libc::dlopen(libname.as_ptr(), libc::RTLD_LAZY | libc::RTLD_LOCAL);
			
			if handle.is_null() {
				panic!("libandroid.so open failed");
			}
			
			let fname = CString::new("AHardwareBuffer_recvHandleFromUnixSocket").unwrap();
			let recv_fn = libc::dlsym(handle, fname.as_ptr());
			if recv_fn.is_null() {
				panic!("failed to load AHardwareBuffer_recvHandleFromUnixSocket");
			}
			
			ANDROID_LIB.handle = handle;
			ANDROID_LIB.fn_recv = Some(std::mem::transmute(recv_fn));
		}
	}
	
	pub fn recv(socket: c_int) -> Option<*mut AHardwareBuffer> {
		Self::init();
			
		let mut buffer: *mut AHardwareBuffer = null_mut();
		unsafe {
			if ANDROID_LIB.fn_recv.unwrap()(socket, &mut buffer as *mut *mut AHardwareBuffer) != 0 {
				buffer = null_mut();
			}
		}
		if buffer == null_mut() {
			return None;
		}
		Some(buffer)
	}
}

impl Drop for Lib {
	fn drop(&mut self) {
		unsafe {
			if !self.handle.is_null() {
				libc::dlclose(self.handle);
			}
		}
	}
}