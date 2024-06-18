#[derive(Debug)]
pub struct IndexBuffer {
    id: gl::types::GLuint,
}

impl IndexBuffer {
    pub fn from(indices: &[u32]) -> Self {
        let mut ebo = Self { id: 0 };
        unsafe {
            gl::GenBuffers(1, &mut ebo.id); 
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo.id);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * std::mem::size_of::<u32>()) as gl::types::GLsizeiptr,
                indices.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW
            );
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }
        ebo
    }

    pub fn new(size: usize) -> Self {
        let mut ebo = Self { id: 0 };
        unsafe {
            gl::GenBuffers(1, &mut ebo.id); 
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo.id);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (size * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                std::ptr::null(),
                gl::STATIC_DRAW
            );
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }
        ebo
    }

    pub fn set_data(&mut self, indices: &[u32]) {
        unsafe {
            gl::GenBuffers(1, &mut self.id); 
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.id);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * std::mem::size_of::<u32>()) as gl::types::GLsizeiptr,
                indices.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW
            );
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }
    }

    pub fn bind(&self) {
        unsafe { gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.id); }
    }

    pub fn unbind(&self) {
        unsafe { gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0) };
    } 
}

impl Drop for IndexBuffer {
    fn drop(&mut self) {
        unsafe { gl::DeleteBuffers(1, &self.id); }
    }
}
