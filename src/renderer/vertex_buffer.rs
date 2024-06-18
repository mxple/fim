use crate::renderer::buffer_utils::BufferLayout;

#[derive(Debug)]
pub struct VertexBuffer {
    id: gl::types::GLuint,
    size: usize,
    layout: Option<BufferLayout>,
}

impl VertexBuffer {
    pub fn from(vertices: &[f32]) -> Self {
        let mut vbo = Self {
            id: 0,
            size: vertices.len(),
            layout: None,
        };
        unsafe {
            gl::GenBuffers(1, &mut vbo.id);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo.id);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                vertices.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
        vbo
    }

    pub fn new(size: usize) -> Self {
        let mut vbo = Self {
            id: 0,
            size, 
            layout: None,
        };
        unsafe {
            gl::GenBuffers(1, &mut vbo.id);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo.id);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (size * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                std::ptr::null(),
                gl::STATIC_DRAW,
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
        vbo
    }

    pub fn set_data(&mut self, vertices: &[f32]) {
        debug_assert!(vertices.len() <= self.size);
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                vertices.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }

    pub fn set_layout(&mut self, layout: BufferLayout) {
        self.layout = Some(layout)
    }

    pub fn get_layout(&self) -> Option<&BufferLayout> {
        self.layout.as_ref()
    }

    pub fn get_layout_mut(&mut self) -> Option<&mut BufferLayout> {
        self.layout.as_mut()
    }

    pub fn bind(&self) {
        unsafe { gl::BindBuffer(gl::ARRAY_BUFFER, self.id); }
    }

    pub fn unbind(&self) {
        unsafe { gl::BindBuffer(gl::ARRAY_BUFFER, 0) };
    }
}

impl Drop for VertexBuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}
