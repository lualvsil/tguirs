use tguirs::*;
use nix::libc;
use std::ffi::CString;

mod egl;
mod gl;
use gl::types::*;

macro_rules! check_egl {
	($expr:expr, $err:expr) => {
		if $expr == egl::NO_CONTEXT || $expr == egl::NO_SURFACE || $expr == egl::NO_IMAGE_KHR {
			return Err(TguiErr::Msg($err));
		}
	};
}

fn main() -> Res<()> {
	let c = Connection::new()?;
	let a = c.new_activity(-1, false)?;
	
	let sv = a.new_surface_view()?;
	let hb1 = c.new_hardware_buffer(800, 800)?;
	let hb2 = c.new_hardware_buffer(800, 800)?;

	initialize_egl(&hb1, &hb2)?;
	
	unsafe {
		gl::Viewport(0,0, 800,800);
		triangle_init();
	}

	let mut current_buffer = 1;
	let mut set_hb: &HardwareBuffer;
	loop {
		let ev = c.recv_event()?;
		if let Some(ref event) = ev.event {
			match event {
				items::event::Event::Destroy(_) => break,
				items::event::Event::FrameComplete(_) => {},
				_ => println!("{:?}", ev.event.unwrap()),
			}
		}
		
		unsafe {
			gl::ClearColor(0.0, 1.0, 1.0, 1.0);
			gl::Clear(gl::COLOR_BUFFER_BIT);
			
			gl::DrawArrays(gl::TRIANGLES, 0, 3);
			
			gl::Flush();
			gl::BindFramebuffer(gl::FRAMEBUFFER, current_buffer);
		}
		
		if current_buffer == 1 {
			set_hb = &hb2;
			current_buffer = 2;
		} else {
			set_hb = &hb1;
			current_buffer = 1;
		}
		sv.set_hb(set_hb)?;
	}

	Ok(())
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
			egl::RENDERABLE_TYPE, egl::OPENGL_ES2_BIT,
			egl::NONE,
		];
		let mut config: egl::types::EGLConfig = std::ptr::null_mut();
		let mut config_size: i32 = 0;
		egl::ChooseConfig(d, configs.as_ptr() as *const i32, &mut config, 1, &mut config_size);
		if config_size == 0 {
			return Err(TguiErr::Msg("config_size == 0"));
		}

		egl::BindAPI(egl::OPENGL_ES_API);

		let context_attribs = [egl::CONTEXT_CLIENT_VERSION, 2, egl::NONE];
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

static VERTEX_SHADER: &'static str = "
attribute vec4 position;

void main()
{
	gl_Position = vec4(position.xyz, 1.0);
}";

static FRAGMENT_SHADER: &'static str = "
precision mediump float;

void main()
{
	gl_FragColor = vec4(1.0);
}
";
static VERTEX_DATA: [GLfloat; 6] = [0.0, 0.5, 0.5, -0.5, -0.5, -0.5];

unsafe fn triangle_init() {
	let vertex = gl::CreateShader(gl::VERTEX_SHADER);
	gl::ShaderSource(vertex, 1, &CString::new(VERTEX_SHADER).unwrap().as_ptr(), std::ptr::null());
	gl::CompileShader(vertex);
	
	let fragment = gl::CreateShader(gl::FRAGMENT_SHADER);
	gl::ShaderSource(fragment, 1, &CString::new(FRAGMENT_SHADER).unwrap().as_ptr(), std::ptr::null());
	gl::CompileShader(fragment);
	
	let program = gl::CreateProgram();
	gl::AttachShader(program, vertex);
	gl::AttachShader(program, fragment);
	gl::LinkProgram(program);
	
	gl::UseProgram(program);
	
	let position = gl::GetAttribLocation(program, CString::new("position").unwrap().as_ptr()) as u32;
	gl::EnableVertexAttribArray(position);
	gl::VertexAttribPointer(position, 2, gl::FLOAT, gl::FALSE as GLboolean, 0, VERTEX_DATA.as_ptr() as *const _);
}