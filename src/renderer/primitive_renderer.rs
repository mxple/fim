use std::ffi::CString;

use super::index_buffer::IndexBuffer;
use super::shader::Program;
use super::shader::Shader;
use super::vertex_array::VertexArray;

const MAX_CIRCLES: usize = 100;
const CIRCLE_RESOLUTION: usize = 30;

const MAX_BEZIERS: usize = 100;
const BEZIER_RESOLUTION: usize = 4;

#[derive(Debug)]
pub struct CircleData {
    pos_rad: glam::Vec4,
    col: glam::Vec4,
}

#[derive(Debug)]
pub struct BezierData {
    p12: glam::Vec4,
    p3: glam::Vec4,  // 8bytes padding
    col: glam::Vec4,
}

pub struct PrimitiveRenderer {
    circle_vao: VertexArray,
    circle_data: Vec<CircleData>,
    circle_shader: Program,
    circle_ssbo: gl::types::GLuint,
    // quad_vao: VertexArray,
    // texture_shader: Program,
    // curr_texture: i32,

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
            &CString::new(include_str!("../shaders/circle.frag")).unwrap(),
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

        let mut circle_sbbo = 0;
        unsafe {
            gl::GenBuffers(1, &mut circle_sbbo);
            gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, circle_sbbo);
            gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, 0, circle_sbbo);
            gl::BufferData(
                gl::SHADER_STORAGE_BUFFER,
                (MAX_CIRCLES * std::mem::size_of::<CircleData>()) as gl::types::GLsizeiptr,
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
            &CString::new(include_str!("../shaders/bezier.frag")).unwrap(),
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

        let mut bezier_sbbo = 0;
        unsafe {
            gl::GenBuffers(1, &mut bezier_sbbo);
            gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, bezier_sbbo);
            gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, 0, bezier_sbbo);
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
            circle_ssbo: circle_sbbo,

            bezier_vao,
            bezier_data: Vec::new(),
            bezier_shader,
            bezier_ssbo: bezier_sbbo,
        }
    }

    pub fn begin_scene(&mut self) {
        self.circle_data.clear();
        self.bezier_data.clear();
    }

    pub fn end_scene(&mut self) {
        self.flush();
    }

    pub fn flush(&mut self) {
        self.circle_vao.bind();
        self.circle_shader.bind();
        unsafe {
            let uniform_name = CString::new("CircleResolution").unwrap();
            let uniform_location =
                gl::GetUniformLocation(self.circle_shader.id(), uniform_name.as_ptr());
            if uniform_location == -1 {
                println!("Failed to find uniform location for CircleResolution");
            } else {
                gl::Uniform1ui(uniform_location, CIRCLE_RESOLUTION as gl::types::GLuint);
            }
        }
        unsafe {
            gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, self.circle_ssbo);
            gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, 0, self.circle_ssbo);
            gl::BufferSubData(
                gl::SHADER_STORAGE_BUFFER,
                0,
                (self.circle_data.len() * std::mem::size_of::<CircleData>())
                    as gl::types::GLsizeiptr,
                self.circle_data.as_ptr() as *const gl::types::GLvoid,
            );
            gl::DrawElements(
                gl::TRIANGLES,
                (self.circle_data.len() * CIRCLE_RESOLUTION * 3) as gl::types::GLsizei,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );
        }

        self.bezier_vao.bind();
        self.bezier_shader.bind();
        unsafe {
            let uniform_name = CString::new("BezierResolution").unwrap();
            let uniform_location =
                gl::GetUniformLocation(self.bezier_shader.id(), uniform_name.as_ptr());
            if uniform_location == -1 {
                println!("Failed to find uniform location for bezierResolution");
            } else {
                gl::Uniform1ui(uniform_location, BEZIER_RESOLUTION as gl::types::GLuint);
            }
        }
        unsafe {
            gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, self.bezier_ssbo);
            gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, 0, self.bezier_ssbo);
            gl::BufferSubData(
                gl::SHADER_STORAGE_BUFFER,
                0,
                (self.bezier_data.len() * std::mem::size_of::<BezierData>())
                    as gl::types::GLsizeiptr,
                self.bezier_data.as_ptr() as *const gl::types::GLvoid,
            );
            gl::DrawElements(
                gl::LINES,
                (self.bezier_data.len() * BEZIER_RESOLUTION * 2) as gl::types::GLsizei,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );
        }
    }

    // TODO consider references mayhaps
    pub fn draw_circle(&mut self, pos: glam::Vec2, col: glam::Vec3, rad: f32) {
        if self.circle_data.len() >= MAX_CIRCLES {
            self.end_scene();
            self.begin_scene();
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
            self.end_scene();
            self.begin_scene();
        }
        self.bezier_data.push(BezierData {
            p12: glam::Vec4::new(
                p1.x, p1.y, p2.x, p2.y
            ),
            p3: glam::Vec4::new(
                p3.x, p3.y, 0.0, 0.0
            ),
            col: glam::Vec4::new(
                col.x, col.y, col.z, 1.0
            )
        });
    }

    pub fn draw_bezier3(&mut self, p1: glam::Vec2, p2: glam::Vec2, p3: glam::Vec2, p4: glam::Vec2, col: glam::Vec3) {
        let c0 = p1 + (p2 - p1) * 0.75; 
        let c1 = p4 + (p3 - p4) * 0.75; 
        let d = (c0 + c1) * 0.5;
        self.draw_bezier(p1, c0, d, col);
        self.draw_bezier(d, c1, p4, col);
    }
}
