use std::{ffi::CString, time::Instant};

use circular_buffer::CircularBuffer;

use crate::{
    configuration::CONFIG, renderer::{
        buffer_utils::{BufferElement, BufferLayout, ShaderDataType},
        index_buffer::IndexBuffer,
        shader::Shader,
    }, START_TIME
};

use super::{
    camera::Camera, primitive_renderer::PrimitiveRenderer, shader::Program,
    vertex_array::VertexArray, vertex_buffer::VertexBuffer,
};

pub struct CursorRenderer {
    cursor_vao: VertexArray,
    cursor_shader: Program,

    cursor_prev: glam::Vec2,
    cursor_prev_prev: glam::Vec2,
    trail_loc: glam::Vec2,
}

impl CursorRenderer {
    pub fn new() -> Self {
        let vert_shader = Shader::from_source(
            &CString::new(include_str!("../shaders/cursor.vert")).unwrap(),
            gl::VERTEX_SHADER,
        )
        .unwrap();

        let frag_shader = Shader::from_source(
            &CString::new(include_str!("../shaders/cursor.frag")).unwrap(),
            gl::FRAGMENT_SHADER,
        )
        .unwrap();

        let cursor_shader = Program::from_shaders(&[vert_shader, frag_shader]).unwrap();

        let mut cursor_vbo = VertexBuffer::new(16);
        cursor_vbo.set_layout(BufferLayout::new(&[BufferElement::new(
            ShaderDataType::Float2,
            true,
        )]));

        // lol
        let indices = [
            4, 5, 6, 6, 5, 7, 0, 2, 4, 4, 2, 6, 0, 1, 4, 4, 1, 5, 1, 3, 5, 5, 3, 7, 2, 3, 6, 6, 3,
            7,
        ];

        let mut cursor_vao = VertexArray::new();
        cursor_vao.add_vbo(cursor_vbo);
        cursor_vao.set_ibo(IndexBuffer::from(&indices));

        Self {
            cursor_vao,
            cursor_shader,

            cursor_prev: glam::vec2(0., 0.),
            cursor_prev_prev: glam::vec2(0., 0.),
            trail_loc: glam::vec2(0., 0.),
        }
    }

    pub unsafe fn draw(&self, cam: &Camera) {
        self.cursor_shader.bind();
        self.cursor_vao.bind();

        let uniform_name = CString::new("uView").unwrap();
        let uniform_location =
            gl::GetUniformLocation(self.cursor_shader.id(), uniform_name.as_ptr());
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
            gl::GetUniformLocation(self.cursor_shader.id(), uniform_name.as_ptr());
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

        let uniform_name = CString::new("uTime").unwrap();
        let uniform_location =
            gl::GetUniformLocation(self.cursor_shader.id(), uniform_name.as_ptr());
        if uniform_location == -1 {
            println!("Failed to find uniform location for uTime");
        } else {
            gl::Uniform1f(
                uniform_location,
                (Instant::now() - *START_TIME).as_secs_f32(),
            );
        }
        gl::BlendEquation(gl::FUNC_SUBTRACT);
        gl::BlendFunc(gl::ONE, gl::ONE_MINUS_DST_COLOR);

        gl::DrawElements(gl::TRIANGLES, 30, gl::UNSIGNED_INT, std::ptr::null());
    }

    pub fn draw_cursor_at(&mut self, x: f32, y: f32, w: f32, h: f32) {
        let cursor_curr = glam::vec2(x + w / 2., y + h / 2.);
        if self.cursor_prev != cursor_curr {
            self.cursor_prev_prev = self.cursor_prev;
            let dist = (cursor_curr).distance(self.cursor_prev);
            self.trail_loc = cursor_curr + (self.cursor_prev - cursor_curr).normalize() * dist.powf(unsafe { CONFIG.get().unwrap_unchecked() }.cursor.trail_length);
        }
        self.cursor_prev = cursor_curr;

        let thickness = 2.;
        let length = 0.5;
        let xf = cursor_curr.x + thickness * (self.trail_loc.x - cursor_curr.x);
        let yf = cursor_curr.y + thickness * (self.trail_loc.y - cursor_curr.y);

        // let dist2 = (self.trail_loc.0 - cursor_curr.0) * (self.trail_loc.0 - cursor_curr.0)
        //     + (self.trail_loc.1 - cursor_curr.1) * (self.trail_loc.1 - cursor_curr.1);
        // let lerp_factor = (dist2 / 10.).clamp(0., 0.9);
        let lerp_factor = unsafe { CONFIG.get().unwrap_unchecked() }.cursor.lerp_factor;
        self.trail_loc -= (self.trail_loc - cursor_curr) * lerp_factor;

        self.cursor_vao.bind();
        let vbo = self.cursor_vao.get_vbo_mut().get_mut(0).unwrap();
        vbo.set_data(&[
            x,
            y,
            x + w,
            y,
            x,
            y + h,
            x + w,
            y + h,
            x + length * (xf - x),
            y + length * (yf - y),
            (x + w) + length * (xf - (x + w)),
            y + length * (yf - y),
            x + length * (xf - x),
            (y + h) + length * (yf - (y + h)),
            (x + w) + length * (xf - (x + w)),
            (y + h) + length * (yf - (y + h)),
        ]);
    }
}
