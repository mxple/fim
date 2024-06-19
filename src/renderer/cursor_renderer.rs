use std::{ffi::CString, time::Instant};

use crate::{
    renderer::{
        buffer_utils::{BufferElement, BufferLayout, ShaderDataType},
        index_buffer::IndexBuffer,
        shader::Shader,
    },
    START_TIME,
};

use super::{
    camera::Camera, primitive_renderer::PrimitiveRenderer, shader::Program, vertex_array::VertexArray, vertex_buffer::VertexBuffer
};

pub struct CursorRenderer {
    cursor_vao: VertexArray,
    cursor_shader: Program,
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
        }
    }

    pub unsafe fn draw(&self, cam: &Camera) {
        self.cursor_shader.bind();
        self.cursor_vao.bind();

        let uniform_name = CString::new("uView").unwrap();
        let uniform_location = gl::GetUniformLocation(self.cursor_shader.id(), uniform_name.as_ptr());
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
        let uniform_location = gl::GetUniformLocation(self.cursor_shader.id(), uniform_name.as_ptr());
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

    pub fn draw_cursor_at(
        &mut self,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        xp: f32,
        yp: f32,
    ) -> (f32, f32) {
        let thickness = 10.;
        let length = 0.1;
        let duration = 0.85;
        let xf = (x + w / 2.) + thickness * (xp - (x + w / 2.));
        let yf = (y + h / 2.) + thickness * (yp - (y + h / 2.));
        // yp = (y+h/2.) + s * (yp - (y+h/2.));
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
        // vbo.set_data(&[0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0]);
        (
            (x + w / 2.) + duration * (xp - (x + w / 2.)),
            (y + h / 2.) + duration * (yp - (y + h / 2.)),
        )
    }

    // pub fn draw_circular_cursor1(&mut self, &mut pr: PrimitiveRenderer) {
    //     let mut prev = glam::vec2(cb.front().unwrap().0, cb.front().unwrap().1);
    //     let mut iter = (&cb).into_iter();
    //     iter.next();
    //     for (ww, (xx, yy)) in iter.enumerate() {
    //         let curr = glam::vec2(*xx, *yy);
    //         let rad = w/2. * (1.-ww as f32 / 10.);
    //         
    //         let unit = (prev - curr).normalize().yx() * rad;
    //         pr.draw_quad(&[curr + unit, prev + unit, prev - unit, curr - unit], glam::Vec3::ONE);
    //         pr.draw_circle(curr, glam::Vec3::ONE, rad);
    //
    //
    //         prev = curr;
    //     }
    // }
    //
    // pub fn draw_circular_cursor2(&mut self, &mut pr: PrimitiveRenderer) {
    //     trail[0] = (x + w/2., y + h/4.);
    //     for i in 0..trail.len() - 1 {
    //         let (first, second) = trail.split_at_mut(i + 1);
    //         let a = &mut first[i];
    //         let b = &mut second[0];
    //         b.0 -= (b.0 - a.0) * 0.6;
    //         b.1 -= (b.1 - a.1) * 0.6;
    //
    //         let curr = glam::vec2(a.0, a.1);
    //         let next = glam::vec2(b.0, b.1);
    //         let radc = w/2. * (1.- i as f32 / 30.);
    //         let radn = w/2. * (1.- (i+1) as f32 / 30.);
    //         let unitc = (next-curr).normalize().yx() * radc;
    //         let unitn = (next-curr).normalize().yx() * radn;
    //
    //         pr.draw_quad(&[curr + unitc, next + unitn, next - unitn, curr - unitc], glam::Vec3::ONE);
    //         pr.draw_circle(curr, glam::Vec3::ONE, radc);
    //     }
    // }
}
