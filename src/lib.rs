pub mod connection;
pub mod activity;
pub mod terr;
pub mod views;
pub mod hardwarebuffer;
pub mod ffi;

pub mod items {
    include!(concat!(env!("OUT_DIR"), "/tgui.proto0.rs"));
}

pub use {
	connection::Connection,
	activity::Activity,
	ffi::hb,
	hardwarebuffer::HardwareBuffer,
	terr::{Res, TguiErr},
	views::*,
};