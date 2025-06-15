// tgui_wrapper.rs
use tguirs::*;
use std::sync::Arc;

use crate::gl;
use crate::egl;

use nix::libc;
use std::ffi::CString;

pub struct TguiManager {
	pub connection: Arc<Connection>,
	activity: Activity,
	surface_view: SurfaceView,
	buffers: Vec<HardwareBuffer>,
	current_buffer: u32,
}

impl TguiManager {
	pub fn new(width: i32, height: i32, buffer_count: usize) -> Res<Self> {
		let connection = Arc::new(Connection::new()?);
		let activity = connection.new_activity(-1, false)?;
		let surface_view = activity.new_surface_view()?;
		surface_view.send_touch_event(true)?;
		
		let mut buffers = Vec::new();
		for _ in 0..buffer_count {
			buffers.push(connection.new_hardware_buffer(width, height)?);
		}
		
		initialize_egl(&buffers[0], &buffers[1])?;
		unsafe { gl::Viewport(0,0, width, height); }

		Ok(Self {
			connection,
			activity,
			surface_view,
			buffers,
			current_buffer: 1,
		})
	}

	pub fn get_buffers(&self) -> &Vec<HardwareBuffer> {
		&self.buffers
	}

	pub fn set_buffer(&self, index: usize) -> Res<()> {
		if let Some(buffer) = self.buffers.get(index) {
			self.surface_view.set_hb(buffer)?;
		}
		Ok(())
	}
	
	pub fn swap_buffers(&mut self) -> Res<()> {
		unsafe {
			gl::BindFramebuffer(gl::FRAMEBUFFER, self.current_buffer+1);
			gl::Flush();
		}
		self.set_buffer(self.current_buffer as usize)?;
		self.current_buffer = (self.current_buffer + 1) % 2;
		Ok(())
	}

	pub fn recv_event(&self) -> Res<items::Event> {
		self.connection.recv_event()
	}
}



macro_rules! check_egl {
	($expr:expr, $err:expr) => {
		if $expr == egl::NO_CONTEXT || $expr == egl::NO_SURFACE || $expr == egl::NO_IMAGE_KHR {
			return Err(TguiErr::Msg($err));
		}
	};
}

fn initialize_egl(hb1: &HardwareBuffer, hb2: &HardwareBuffer) -> Res<()> {
	egl::load_with(|name| {
		unsafe {
			let egllibname = CString::new("libEGL.so").unwrap();
			let egllib = libc::dlopen(egllibname.as_ptr(), libc::RTLD_LAZY | libc::RTLD_GLOBAL);
			let cname = CString::new(name).unwrap();
			libc::dlsym(egllib, cname.as_ptr())
		}
	});

	unsafe {
		let d = egl::GetDisplay(egl::DEFAULT_DISPLAY);
		let mut major: i32 = 0;
		let mut minor: i32 = 0;
		egl::Initialize(d, &mut major, &mut minor);
		println!("EGL {}.{}", major, minor);

		let configs = [
			egl::RED_SIZE, 8,
			egl::GREEN_SIZE, 8,
			egl::BLUE_SIZE, 8,
			egl::COLOR_BUFFER_TYPE, egl::RGB_BUFFER,
			egl::SURFACE_TYPE, egl::PBUFFER_BIT,
			egl::RENDERABLE_TYPE, egl::OPENGL_ES3_BIT,
			egl::NONE,
		];
		let mut config: egl::types::EGLConfig = std::ptr::null_mut();
		let mut config_size: i32 = 0;
		egl::ChooseConfig(d, configs.as_ptr() as *const i32, &mut config, 1, &mut config_size);
		if config_size == 0 {
			return Err(TguiErr::Msg("config_size == 0"));
		}

		egl::BindAPI(egl::OPENGL_ES_API);

		let context_attribs = [
			egl::CONTEXT_MAJOR_VERSION, 3,
			egl::CONTEXT_MINOR_VERSION, 2,
			egl::NONE];
		let ctx = egl::CreateContext(
			d,
			config,
			egl::NO_CONTEXT,
			context_attribs.as_ptr() as *const i32,
		);
		check_egl!(ctx, "context null");

		let pbuffer_attribs = [egl::WIDTH, 1, egl::HEIGHT, 1, egl::NONE];
		let dummy_surface = egl::CreatePbufferSurface(
			d,
			config,
			pbuffer_attribs.as_ptr() as *const i32,
		);
		check_egl!(dummy_surface, "dummy_surface null");

		egl::MakeCurrent(d, dummy_surface, dummy_surface, ctx);
		
		gl::load_with(|name| {
			let cname = CString::new(name).unwrap();
			egl::GetProcAddress(cname.as_ptr()) as *const std::ffi::c_void
		});
		
		configure_framebuffer(d, &hb1, 1)?;
		configure_framebuffer(d, &hb2, 2)?;
	}

	Ok(())
}

fn configure_framebuffer(d: egl::types::EGLDisplay, hbuffer: &HardwareBuffer, bufnumber: u32) -> Res<()> {
	unsafe {
		let image_attribs = [egl::IMAGE_PRESERVED_KHR, egl::TRUE, egl::NONE];
		let cb = egl::GetNativeClientBufferANDROID(hbuffer.buffer as *const _);
		let img = egl::CreateImageKHR(
			d,
			egl::NO_CONTEXT,
			egl::NATIVE_BUFFER_ANDROID,
			cb,
			image_attribs.as_ptr() as *const i32,
		);
		check_egl!(img, "img NO_IMAGE_KHR");
		
		gl::BindFramebuffer(gl::FRAMEBUFFER, bufnumber);
		gl::BindRenderbuffer(gl::RENDERBUFFER, bufnumber);
		gl::EGLImageTargetRenderbufferStorageOES(gl::FRAMEBUFFER, img);
		gl::FramebufferRenderbuffer(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::RENDERBUFFER, bufnumber);
	}
	Ok(())
}
