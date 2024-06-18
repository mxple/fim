use std::ffi::c_void;

use crate::renderer::index_buffer::IndexBuffer;
use crate::renderer::vertex_buffer::VertexBuffer;

use crate::renderer::buffer_utils::ShaderDataType;

#[derive(Debug)]
pub struct VertexArray {
    id: gl::types::GLuint,
    layout_location: u32,
    vbos: Vec<VertexBuffer>,
    ibo: Option<IndexBuffer>,
}

impl VertexArray {
    pub fn new() -> Self {
        let mut vao = Self {
            id: 0,
            layout_location: 0,
            vbos: Vec::new(),
            ibo: None,
        };
        unsafe {
            gl::GenVertexArrays(1, &mut vao.id);
            gl::BindVertexArray(vao.id);
        }
        vao
    }

    pub fn add_vbo(&mut self, vbo: VertexBuffer) {
        unsafe { gl::BindVertexArray(self.id); }
        vbo.bind();
        if let Some(layout) = vbo.get_layout() {
            for e in layout.iter() {
                match e.shader_type {
                    ShaderDataType::Float
                    | ShaderDataType::Float2
                    | ShaderDataType::Float3
                    | ShaderDataType::Float4 => unsafe {
                        gl::EnableVertexAttribArray(self.layout_location);
                        gl::VertexAttribPointer(
                            self.layout_location,
                            e.component_count as i32,
                            e.gl_type,
                            e.is_normalized as u8,
                            layout.get_stride() as i32,
                            e.offset as *const c_void,
                        );
                        self.layout_location += 1;
                    },
                    ShaderDataType::Int
                    | ShaderDataType::Int2
                    | ShaderDataType::Int3
                    | ShaderDataType::Int4
                    | ShaderDataType::Bool => unsafe {
                        gl::EnableVertexAttribArray(self.layout_location);
                        gl::VertexAttribIPointer(
                            self.layout_location,
                            e.component_count as i32,
                            e.gl_type,
                            layout.get_stride() as i32,
                            e.offset as *const c_void,
                        );
                        self.layout_location += 1;
                    }
                    ShaderDataType::Mat3 | ShaderDataType::Mat4 => {
                        panic!("Matrix shader types not yet supported!")
                    }
                }
            }
        }
        self.vbos.push(vbo);
    }

    pub fn set_ibo(&mut self, ibo: IndexBuffer) {
        unsafe { gl::BindVertexArray(self.id); }
        ibo.bind();
        self.ibo = Some(ibo);
    }

    pub fn get_vbo(&self) -> &Vec<VertexBuffer> {
        &self.vbos
    }

    pub fn get_vbo_mut(&mut self) -> &mut Vec<VertexBuffer> {
        &mut self.vbos
    }

    pub fn get_ibo(&self) -> Option<&IndexBuffer> {
        self.ibo.as_ref()
    }

    pub fn bind(&self) {
        unsafe { gl::BindVertexArray(self.id); }
    }

    pub fn unbind(&self) {
        unsafe { gl::BindVertexArray(0); }
    }
}
