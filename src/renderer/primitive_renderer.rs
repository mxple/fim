use std::ffi::CString;

use super::camera::Camera;
use super::index_buffer::IndexBuffer;
use super::shader::Program;
use super::shader::Shader;
use super::vertex_array::VertexArray;

const MAX_CIRCLES: usize = 1000;
const CIRCLE_RESOLUTION: usize = 40;

const MAX_QUADS: usize = 1000;

const MAX_BEZIERS: usize = 1000;
const BEZIER_RESOLUTION: usize = 4;

#[derive(Debug)]
struct CircleData {
    pos_rad: glam::Vec4,
    col: glam::Vec4,
}

#[derive(Debug)]
struct QuadData {
    col : glam::Vec4,
    p0 : glam::Vec2,
    p1 : glam::Vec2,
    p2 : glam::Vec2,
    p3 : glam::Vec2,
}

#[derive(Debug)]
struct BezierData {
    p12: glam::Vec4,
    p3: glam::Vec4, // 8bytes padding
    col: glam::Vec4,
}

pub struct PrimitiveRenderer {
    circle_vao: VertexArray,
    circle_data: Vec<CircleData>,
    circle_shader: Program,
    circle_ssbo: gl::types::GLuint,
    
    quad_vao: VertexArray,
    quad_data: Vec<QuadData>,
    quad_shader: Program,
    quad_ssbo: gl::types::GLuint,
    
    bezier_vao: VertexArray,
    bezier_data: Vec<BezierData>,
    bezier_shader: Program,
    bezier_ssbo: gl::types::GLuint,
}

impl PrimitiveRenderer {
    pub fn new() -> Self {
        use std::ffi::CString;

        let vert_shader = Shader::from_source(
            &CString::new(include_str!("../shaders/circle.vert")).unwrap(),
            gl::VERTEX_SHADER,
        )
        .unwrap();

        let frag_shader = Shader::from_source(
            &CString::new(include_str!("../shaders/primitive.frag")).unwrap(),
            gl::FRAGMENT_SHADER,
        )
        .unwrap();

        let circle_shader = Program::from_shaders(&[vert_shader, frag_shader]).unwrap();

        let mut circle_vao = VertexArray::new();

        let mut indices: [u32; MAX_CIRCLES * CIRCLE_RESOLUTION * 3] =
            [0; MAX_CIRCLES * CIRCLE_RESOLUTION * 3];
        let mut offset: usize = 0;
        for circ in 0..MAX_CIRCLES {
            for tri in 0..CIRCLE_RESOLUTION {
                let index = (circ * CIRCLE_RESOLUTION + tri) * 3;
                indices[index + 0] = (offset + 0) as u32;
                indices[index + 1] = (offset + tri + 1) as u32;
                indices[index + 2] = (offset + tri + 2) as u32;
            }
            indices[(circ * CIRCLE_RESOLUTION + CIRCLE_RESOLUTION) * 3 - 1] = (offset + 1) as u32;
            offset += CIRCLE_RESOLUTION + 1;
        }
        circle_vao.set_ibo(IndexBuffer::from(&indices));

        let mut circle_ssbo = 0;
        unsafe {
            gl::GenBuffers(1, &mut circle_ssbo);
            gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, circle_ssbo);
            gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, 0, circle_ssbo);
            gl::BufferData(
                gl::SHADER_STORAGE_BUFFER,
                (MAX_CIRCLES * std::mem::size_of::<CircleData>()) as gl::types::GLsizeiptr,
                std::ptr::null(),
                gl::DYNAMIC_DRAW,
            );
            gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, 0, 0);
        }

        let vert_shader = Shader::from_source(
            &CString::new(include_str!("../shaders/quad.vert")).unwrap(),
            gl::VERTEX_SHADER,
        )
        .unwrap();

        let frag_shader = Shader::from_source(
            &CString::new(include_str!("../shaders/primitive.frag")).unwrap(),
            gl::FRAGMENT_SHADER,
        )
        .unwrap();

        let quad_shader = Program::from_shaders(&[vert_shader, frag_shader]).unwrap();

        let mut quad_vao = VertexArray::new();

        let mut indices: [u32; MAX_QUADS * 6] = [0; MAX_QUADS * 6];
        let mut offset: usize = 0;
        for i in (0..MAX_QUADS).step_by(6) {
            indices[i + 0] = (offset + 0) as u32;
            indices[i + 1] = (offset + 1) as u32;
            indices[i + 2] = (offset + 2) as u32;

            indices[i + 3] = (offset + 2) as u32;
            indices[i + 4] = (offset + 3) as u32;
            indices[i + 5] = (offset + 0) as u32;

            offset += 4;
        }
        quad_vao.set_ibo(IndexBuffer::from(&indices));

        let mut quad_ssbo = 0;
        unsafe {
            gl::GenBuffers(1, &mut quad_ssbo);
            gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, quad_ssbo);
            gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, 0, quad_ssbo);
            gl::BufferData(
                gl::SHADER_STORAGE_BUFFER,
                (MAX_QUADS * std::mem::size_of::<CircleData>()) as gl::types::GLsizeiptr,
                std::ptr::null(),
                gl::DYNAMIC_DRAW,
            );
            gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, 0, 0);
        }

        let vert_shader = Shader::from_source(
            &CString::new(include_str!("../shaders/bezier.vert")).unwrap(),
            gl::VERTEX_SHADER,
        )
        .unwrap();

        let frag_shader = Shader::from_source(
            &CString::new(include_str!("../shaders/primitive.frag")).unwrap(),
            gl::FRAGMENT_SHADER,
        )
        .unwrap();

        let bezier_shader = Program::from_shaders(&[vert_shader, frag_shader]).unwrap();

        let mut bezier_vao = VertexArray::new();

        let mut indices: [u32; MAX_BEZIERS * BEZIER_RESOLUTION * 2] =
            [0; MAX_BEZIERS * BEZIER_RESOLUTION * 2];
        let mut offset: usize = 0;
        for bezier in 0..MAX_BEZIERS {
            for i in 0..BEZIER_RESOLUTION {
                let index = (bezier * BEZIER_RESOLUTION + i) * 2;
                indices[index] = (offset + i) as u32;
                indices[index + 1] = (offset + i + 1) as u32;
            }
            offset += BEZIER_RESOLUTION + 1;
        }
        bezier_vao.set_ibo(IndexBuffer::from(&indices));

        let mut bezier_ssbo = 0;
        unsafe {
            gl::GenBuffers(1, &mut bezier_ssbo);
            gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, bezier_ssbo);
            gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, 0, bezier_ssbo);
            gl::BufferData(
                gl::SHADER_STORAGE_BUFFER,
                (MAX_BEZIERS * std::mem::size_of::<BezierData>()) as gl::types::GLsizeiptr,
                std::ptr::null(),
                gl::STATIC_DRAW,
            );
            gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, 0, 0);
        }

        Self {
            circle_vao,
            circle_data: Vec::new(),
            circle_shader,
            circle_ssbo,

            quad_vao,
            quad_data: Vec::new(),
            quad_shader,
            quad_ssbo,

            bezier_vao,
            bezier_data: Vec::new(),
            bezier_shader,
            bezier_ssbo,
        }
    }

    pub fn begin_scene(&mut self) {
        self.circle_data.clear();
        self.quad_data.clear();
        self.bezier_data.clear();
    }

    pub fn end_scene(&mut self, cam: &Camera) {
        unsafe { self.flush(cam) };
    }

    pub unsafe fn flush(&mut self, cam: &Camera) {
        self.circle_vao.bind();
        self.circle_shader.bind();

        let uniform_name = CString::new("uView").unwrap();
        let uniform_location =
            gl::GetUniformLocation(self.circle_shader.id(), uniform_name.as_ptr());
        if uniform_location == -1 {
            println!("Failed to find uniform location for uView");
        } else {
            gl::UniformMatrix4fv(
                uniform_location,
                1,
                false as u8,
                &cam.view as *const glam::Mat4 as *const gl::types::GLfloat,
            );
        }

        let uniform_name = CString::new("uProj").unwrap();
        let uniform_location =
            gl::GetUniformLocation(self.circle_shader.id(), uniform_name.as_ptr());
        if uniform_location == -1 {
            println!("Failed to find uniform location for uProj");
        } else {
            gl::UniformMatrix4fv(
                uniform_location,
                1,
                false as u8,
                &cam.proj as *const glam::Mat4 as *const gl::types::GLfloat,
            );
        }

        let uniform_name = CString::new("CircleResolution").unwrap();
        let uniform_location =
            gl::GetUniformLocation(self.circle_shader.id(), uniform_name.as_ptr());
        if uniform_location == -1 {
            println!("Failed to find uniform location for CircleResolution");
        } else {
            gl::Uniform1ui(uniform_location, CIRCLE_RESOLUTION as gl::types::GLuint);
        }
        gl::BlendEquation(gl::FUNC_SUBTRACT);
        gl::BlendFunc(gl::ONE, gl::ONE_MINUS_DST_COLOR);

        gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, self.circle_ssbo);
        gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, 0, self.circle_ssbo);
        gl::BufferSubData(
            gl::SHADER_STORAGE_BUFFER,
            0,
            (self.circle_data.len() * std::mem::size_of::<CircleData>()) as gl::types::GLsizeiptr,
            self.circle_data.as_ptr() as *const gl::types::GLvoid,
        );
        gl::DrawElements(
            gl::TRIANGLES,
            (self.circle_data.len() * CIRCLE_RESOLUTION * 3) as gl::types::GLsizei,
            gl::UNSIGNED_INT,
            std::ptr::null(),
        );

        self.quad_vao.bind();
        self.quad_shader.bind();

        let uniform_name = CString::new("uView").unwrap();
        let uniform_location =
            gl::GetUniformLocation(self.quad_shader.id(), uniform_name.as_ptr());
        if uniform_location == -1 {
            println!("Failed to find uniform location for uView");
        } else {
            gl::UniformMatrix4fv(
                uniform_location,
                1,
                false as u8,
                &cam.view as *const glam::Mat4 as *const gl::types::GLfloat,
            );
        }

        let uniform_name = CString::new("uProj").unwrap();
        let uniform_location =
            gl::GetUniformLocation(self.quad_shader.id(), uniform_name.as_ptr());
        if uniform_location == -1 {
            println!("Failed to find uniform location for uProj");
        } else {
            gl::UniformMatrix4fv(
                uniform_location,
                1,
                false as u8,
                &cam.proj as *const glam::Mat4 as *const gl::types::GLfloat,
            );
        }

        gl::BlendEquation(gl::FUNC_SUBTRACT);
        gl::BlendFunc(gl::ONE, gl::ONE_MINUS_DST_COLOR);

        gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, self.quad_ssbo);
        gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, 0, self.quad_ssbo);
        gl::BufferSubData(
            gl::SHADER_STORAGE_BUFFER,
            0,
            (self.quad_data.len() * std::mem::size_of::<QuadData>()) as gl::types::GLsizeiptr,
            self.quad_data.as_ptr() as *const gl::types::GLvoid,
        );
        gl::DrawElements(
            gl::TRIANGLES,
            (self.quad_data.len() * 6) as gl::types::GLsizei,
            gl::UNSIGNED_INT,
            std::ptr::null(),
        );

        self.bezier_vao.bind();
        self.bezier_shader.bind();
        let uniform_name = CString::new("BezierResolution").unwrap();
        let uniform_location =
            gl::GetUniformLocation(self.bezier_shader.id(), uniform_name.as_ptr());
        if uniform_location == -1 {
            println!("Failed to find uniform location for bezierResolution");
        } else {
            gl::Uniform1ui(uniform_location, BEZIER_RESOLUTION as gl::types::GLuint);
        }
        gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, self.bezier_ssbo);
        gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, 0, self.bezier_ssbo);
        gl::BufferSubData(
            gl::SHADER_STORAGE_BUFFER,
            0,
            (self.bezier_data.len() * std::mem::size_of::<BezierData>()) as gl::types::GLsizeiptr,
            self.bezier_data.as_ptr() as *const gl::types::GLvoid,
        );
        gl::DrawElements(
            gl::LINES,
            (self.bezier_data.len() * BEZIER_RESOLUTION * 2) as gl::types::GLsizei,
            gl::UNSIGNED_INT,
            std::ptr::null(),
        );
    }

    // TODO consider references mayhaps
    pub fn draw_circle(&mut self, pos: glam::Vec2, col: glam::Vec3, rad: f32) {
        if self.circle_data.len() >= MAX_CIRCLES {
            // self.end_scene();
            // self.begin_scene();
            return;
        }
        self.circle_data.push(CircleData {
            pos_rad: glam::Vec4::new(pos.x, pos.y, 0.0, rad),
            col: glam::Vec4::new(col.x, col.y, col.z, 1.0),
        });
    }

    pub fn draw_line(&mut self, p1: glam::Vec2, p2: glam::Vec2, col: glam::Vec3) {
        let pm = (p1 + p2) * 0.5;
        self.draw_bezier(p1, pm, p2, col);
    }

    pub fn draw_bezier(&mut self, p1: glam::Vec2, p2: glam::Vec2, p3: glam::Vec2, col: glam::Vec3) {
        if self.bezier_data.len() >= MAX_BEZIERS {
            // self.end_scene();
            // self.begin_scene();
            return;
        }
        self.bezier_data.push(BezierData {
            p12: glam::Vec4::new(p1.x, p1.y, p2.x, p2.y),
            p3: glam::Vec4::new(p3.x, p3.y, 0.0, 0.0),
            col: glam::Vec4::new(col.x, col.y, col.z, 1.0),
        });
    }

    pub fn draw_bezier3(
        &mut self,
        p1: glam::Vec2,
        p2: glam::Vec2,
        p3: glam::Vec2,
        p4: glam::Vec2,
        col: glam::Vec3,
    ) {
        let c0 = p1 + (p2 - p1) * 0.75;
        let c1 = p4 + (p3 - p4) * 0.75;
        let d = (c0 + c1) * 0.5;
        self.draw_bezier(p1, c0, d, col);
        self.draw_bezier(d, c1, p4, col);
    }

    pub fn draw_quad(&mut self, pos: &[glam::Vec2 ; 4], col: glam::Vec3) {
        if self.quad_data.len() >= MAX_QUADS {
            // self.end_scene();
            // self.begin_scene();
            return;
        }
        self.quad_data.push(QuadData {
            col: glam::Vec4::new(col.x, col.y, col.z, 1.0),
            p0: pos[0],
            p1: pos[1],
            p2: pos[2],
            p3: pos[3],
        });
    }
}
