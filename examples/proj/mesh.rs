use crate::gl;
use crate::gl::types::*;
use nix::libc;

use std::ffi::CString;
use std::ptr;
use nix::libc::{c_void, size_t};

// Definição de Vertex
pub type Pos = [f32; 3];
pub type Color = [f32; 4];

#[derive(Debug, Clone)]
#[repr(C, packed)]
pub struct Vertex(pub Pos, pub Color);

// Mesh que representa a malha
pub struct Mesh {
    vao: u32,
    vbo: u32,
    ebo: u32,
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
}

impl Mesh {
    // Cria uma nova malha e configura os buffers necessários
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>) -> Mesh {
        let mut vao: u32 = 0;
        let mut vbo: u32 = 0;
        let mut ebo: u32 = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::GenBuffers(1, &mut ebo);

            gl::BindVertexArray(vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<Vertex>()) as isize,
                vertices.as_ptr() as *const c_void,
                gl::STATIC_DRAW,
            );

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * std::mem::size_of::<u32>()) as isize,
                indices.as_ptr() as *const c_void,
                gl::STATIC_DRAW,
            );

            // Configura o atributo de posição
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                (std::mem::size_of::<Vertex>()) as i32,
                ptr::null(),
            );
            gl::EnableVertexAttribArray(0);

            // Configura o atributo de cor
            gl::VertexAttribPointer(
                1,
                4,
                gl::FLOAT,
                gl::FALSE,
                (std::mem::size_of::<Vertex>()) as i32,
                (3 * std::mem::size_of::<f32>()) as *const c_void,
            );
            gl::EnableVertexAttribArray(1);

            gl::BindVertexArray(0); // Desvincula o VAO
        }

        Mesh {
            vao,
            vbo,
            ebo,
            vertices,
            indices,
        }
    }

    // Desenha a malha
    pub fn draw(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawElements(
                gl::TRIANGLES,
                self.indices.len() as i32,
                gl::UNSIGNED_INT,
                ptr::null(),
            );
            gl::BindVertexArray(0);
        }
    }

    // Libera os recursos
    pub fn delete(&self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteBuffers(1, &self.ebo);
        }
    }
}

// fn main() {
//     // Exemplo de como criar uma malha com dados fictícios
//     let vertices = vec![
//         Vertex { x: 0.0, y: 0.0, z: 0.0, r: 1.0, g: 0.0, b: 0.0, a: 1.0 },
//         Vertex { x: 1.0, y: 0.0, z: 0.0, r: 0.0, g: 1.0, b: 0.0, a: 1.0 },
//         Vertex { x: 0.0, y: 1.0, z: 0.0, r: 0.0, g: 0.0, b: 1.0, a: 1.0 },
//     ];
//     let indices = vec![0, 1, 2];

//     // Cria a malha
//     let mesh = Mesh::new(vertices, indices);

//     // Desenha a malha
//     mesh.draw();

//     // Libera os recursos
//     mesh.delete();
// }