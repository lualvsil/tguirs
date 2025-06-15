use crate::gl;
use crate::gl::types::*;

use crate::mesh::*;

use std::ffi::{CString, CStr};
use std::ptr;


pub struct Renderer {
	color: f32,
	m: Mesh,
	ms: Vec<Mesh>,
	pub touches: Vec<(i32, i32)>,
}

impl Renderer {
	pub fn new() -> Self {
		// Shader
		let vertex_shader_source = r#"
            #version 320 es
            layout(location = 0) in vec3 a_Position;
            layout(location = 1) in vec4 a_Color;
            
            out vec4 color;
            
            void main() {
                color = a_Color;
                gl_Position = vec4(a_Position, 1.0);
            }
		"#;
		let fragment_shader_source = r#"
            #version 320 es
            precision mediump float;
            
            in vec4 color;
            out vec4 fragColor;
            
            void main() {
                fragColor = color;
            }
		"#;
		
		let prog = init_shader(vertex_shader_source, fragment_shader_source);
		unsafe { gl::UseProgram(prog) };
		
		// Vertices
		let vertices = vec![
            Vertex {0: [0.0, 0.0, 0.0], 1: [1.0, 0.0, 0.0, 1.0] },
            Vertex {0: [1.0, 0.0, 0.0], 1: [0.0, 1.0, 0.0, 1.0] },
            Vertex {0: [0.0, 1.0, 0.0], 1: [0.0, 0.0, 1.0, 1.0] },
        ];
        let indices = vec![0, 1, 2];
        
        // Cria a malha
        let mesh = Mesh::new(vertices.clone(), indices.clone());
		
		unsafe {
		gl::Enable(gl::CULL_FACE);
		}
		
		Self {
			color: 0.0,
			m: mesh,
			ms: (0..500).map(|_| Mesh::new(vertices.clone(), indices.clone())).collect(),
			touches: Vec::new(),
		}
	}
	
	pub fn update(&mut self) {
		if self.touches.len() != 0 {
			self.color = (1.0 / 1080.0) * self.touches[0].0 as f32;
		}
	}
	
	pub unsafe fn draw(&self) {
		gl::ClearColor(self.color, 1.0, 1.0, 1.0);
		gl::Clear(gl::COLOR_BUFFER_BIT);
		
		// for m in &self.ms {
		for i in 0..500 {
		    self.m.draw();
		}
		// }
	}
}

pub fn init_shader(vertex_src: &str, fragment_src: &str) -> u32 {
    unsafe {
        // Criação do vertex shader
        let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
        let vertex_c_str = CString::new(vertex_src).unwrap();
        gl::ShaderSource(vertex_shader, 1, &vertex_c_str.as_ptr(), ptr::null());
        gl::CompileShader(vertex_shader);

        // Verifica erros de compilação no vertex shader
        let mut success: i32 = 1;
        gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut log = vec![0; 512];
            gl::GetShaderInfoLog(vertex_shader, 512, ptr::null_mut(), log.as_mut_ptr() as *mut u8);
            panic!("Erro ao compilar vertex shader: {}", CStr::from_ptr(log.as_ptr() as *const u8).to_string_lossy());
        }

        // Criação do fragment shader
        let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        let fragment_c_str = CString::new(fragment_src).unwrap();
        gl::ShaderSource(fragment_shader, 1, &fragment_c_str.as_ptr(), ptr::null());
        gl::CompileShader(fragment_shader);

        // Verifica erros de compilação no fragment shader
        gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut log = vec![0; 512];
            gl::GetShaderInfoLog(fragment_shader, 512, ptr::null_mut(), log.as_mut_ptr() as *mut u8);
            panic!("Erro ao compilar fragment shader: {}", CStr::from_ptr(log.as_ptr() as *const u8).to_string_lossy());
        }

        // Criação do programa de shader
        let shader_program = gl::CreateProgram();
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);

        // Verifica erros de linkagem no programa
        gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
        if success == 0 {
            let mut log = vec![0; 512];
            gl::GetProgramInfoLog(shader_program, 512, ptr::null_mut(), log.as_mut_ptr() as *mut u8);
            panic!("Erro ao linkar programa de shader: {}", CStr::from_ptr(log.as_ptr() as *const u8).to_string_lossy());
        }

        // Libera os shaders após linká-los
        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        shader_program
    }
}