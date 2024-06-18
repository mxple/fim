use std::{collections::HashMap, ffi::CString, os::raw::c_void, time::Instant};

use crate::{
    renderer::{
        index_buffer::IndexBuffer,
        shader::Shader,
    },
    START_TIME,
};

use super::{
    camera::Camera,
    font_manager::{Curve, FontManager, GlyphData},
    shader::Program,
    vertex_array::VertexArray,
};

#[allow(dead_code)]
struct QuadData {
    pos: glam::Vec2,
    uv0: glam::Vec2,
    uv1: glam::Vec2,
    start: u32,
    count: u32,
}

pub struct TextRenderer {
    font_manager: FontManager,
    text_vao: VertexArray,
    text_shader: Program,
    quad_data: Vec<QuadData>,
    curve_ssbo: gl::types::GLuint,
    quad_ssbo: gl::types::GLuint,

    curve_ssbo_capacity: usize,

    curve_data: Vec<Curve>,
    glyph_map: HashMap<u32, GlyphData>,

    pub advance: f32,
    pub height: f32,
}

const MAX_QUADS: usize = 100000;

impl TextRenderer {
    pub fn new(main_font_name: &str) -> Self {
        let mut curve_data: Vec<Curve> = Vec::new();
        let mut glyph_map: HashMap<u32, GlyphData> = HashMap::new();
        let mut advance = 0.;
        let mut height = 0.;

        let mut font_manager = FontManager::new();
        font_manager.load_main_font(main_font_name, &mut curve_data, &mut glyph_map, &mut advance, &mut height);
        font_manager.prepare_fallback_fonts();

        let vert_shader = Shader::from_source(
            &CString::new(include_str!("../shaders/font.vert")).unwrap(),
            gl::VERTEX_SHADER,
        )
        .unwrap();

        let frag_shader = Shader::from_source(
            &CString::new(include_str!("../shaders/font.frag")).unwrap(),
            gl::FRAGMENT_SHADER,
        )
        .unwrap();

        let font_shader = Program::from_shaders(&[vert_shader, frag_shader]).unwrap();

        let mut font_vao = VertexArray::new();

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
        font_vao.set_ibo(IndexBuffer::from(&indices));

        let curve_ssbo_capacity = curve_data.len().next_power_of_two();
        let mut curve_ssbo = 0;
        unsafe {
            gl::GenBuffers(1, &mut curve_ssbo);
            gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, curve_ssbo);
            gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, 1, curve_ssbo);
            gl::BufferData(
                gl::SHADER_STORAGE_BUFFER,
                (curve_ssbo_capacity * std::mem::size_of::<Curve>()) as gl::types::GLsizeiptr,
                curve_data.as_ptr() as *const c_void,
                gl::DYNAMIC_DRAW,
            );
        }

        let mut quad_ssbo = 0;
        unsafe {
            gl::GenBuffers(1, &mut quad_ssbo);
            gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, quad_ssbo);
            gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, 0, quad_ssbo);
            gl::BufferData(
                gl::SHADER_STORAGE_BUFFER,
                (MAX_QUADS * std::mem::size_of::<QuadData>()) as gl::types::GLsizeiptr,
                std::ptr::null(),
                gl::DYNAMIC_DRAW,
            );
        }

        Self {
            font_manager,
            text_vao: font_vao,
            text_shader: font_shader,
            quad_data: Vec::new(),
            curve_ssbo,
            quad_ssbo,

            curve_ssbo_capacity,

            curve_data,
            glyph_map,

            advance,
            height,
        }
    }

    pub fn begin_scene(&mut self) {
        self.quad_data.clear();
    }

    pub fn flush(&mut self, cam: &Camera) {
        // dbg!(self.curve_data.len());
        self.text_vao.bind();
        self.text_shader.bind();
        unsafe {
            let uniform_name = CString::new("uView").unwrap();
            let uniform_location =
                gl::GetUniformLocation(self.text_shader.id(), uniform_name.as_ptr());
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
                gl::GetUniformLocation(self.text_shader.id(), uniform_name.as_ptr());
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
                gl::GetUniformLocation(self.text_shader.id(), uniform_name.as_ptr());
            if uniform_location == -1 {
                println!("Failed to find uniform location for uTime");
            } else {
                gl::Uniform1f(
                    uniform_location,
                    (Instant::now() - *START_TIME).as_secs_f32(),
                );
            }
        }
        unsafe {
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
        }

    }

    pub fn prep_text(&mut self, text: &str) {
        if self
            .font_manager
            .load_glyphs_in_str(&mut self.curve_data, &mut self.glyph_map, text)
        {
            self.reload_curve_buffer();
        }
    }

    fn reload_curve_buffer(&mut self) {
        if self.curve_data.len() > self.curve_ssbo_capacity {
            self.curve_ssbo_capacity = self.curve_data.len().next_power_of_two();
            unsafe {
                gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, self.curve_ssbo);
                gl::DeleteBuffers(1, &self.curve_ssbo);

                gl::GenBuffers(1, &mut self.curve_ssbo);
                gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, self.curve_ssbo);
                gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, 1, self.curve_ssbo);
                gl::BufferData(
                    gl::SHADER_STORAGE_BUFFER,
                    (self.curve_ssbo_capacity * std::mem::size_of::<Curve>())
                        as gl::types::GLsizeiptr,
                    self.curve_data.as_ptr() as *const c_void,
                    gl::DYNAMIC_DRAW,
                );
            }
        }
        else {
            unsafe {
                gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, self.curve_ssbo);
                gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, 1, self.curve_ssbo);
                gl::BufferSubData(
                    gl::SHADER_STORAGE_BUFFER,
                    0,
                    (self.curve_data.len() * std::mem::size_of::<Curve>()) as gl::types::GLsizeiptr,
                    self.curve_data.as_ptr() as *const c_void,
                );
            }
        }
    }

    // additionally returns cursor position and size if given the line pos and char pos of the cursor
    pub fn draw_text(
        &mut self,
        mut x: f32,
        mut y: f32,
        text: &str,
        wrap: f32,
        cursor_pos: Option<(u32, u32)>,
    ) -> (f32, f32, f32, f32) {
        self.prep_text(text);

        let original_x = x;
        let mut found_char = false;

        // cursor width is space width or null char width if space not found
        let mut cursor_w = self
            .glyph_map
            .get(&(' ' as u32))
            .unwrap_or(self.glyph_map.get(&0_u32).unwrap())
            .advance;

        let mut cursor_x = x;
        let mut cursor_y = y;
        let mut last_y = y;

        let mut line = 1;
        let mut char = 0;
        let cursor_line = cursor_pos.unwrap_or((u32::MAX, u32::MAX)).0;
        let cursor_char = cursor_pos.unwrap_or((u32::MAX, u32::MAX)).1;

        // let mut prev_glyph_index: u32 = 0;
        for c in text.chars() {
            // we check for unknown characters behorehand
            let glyph = self
                .glyph_map
                .get(&(c as u32))
                .unwrap_or(self.glyph_map.get(&0_u32).unwrap());

            // FOUND yay
            if !found_char && line == cursor_line && char == cursor_char {
                found_char = true;
                cursor_x = x;
                cursor_y = y;
                cursor_w = glyph.advance;
            }

            if !found_char && line == cursor_line + 1 {
                found_char = true; // pseudo true
                cursor_x = original_x;
                cursor_y = last_y;
                cursor_w = glyph.advance;
            }

            if c == '\r' {
                continue;
            }

            if c == '\n' {
                x = original_x;
                last_y = y;
                y -= self.height;
                char = 0;
                line += 1;
                continue;
            }

            // kerning not going to be supported

            char += 1;

            if x > wrap && glyph.count == 0 {
                x = original_x;
                last_y = y;
                y -= self.height;
                continue;
            }

            if glyph.count != 0 {
                // neccessary for glyphs like g p q
                self.quad_data.push(QuadData {
                    pos: glam::Vec2 { x, y },
                    uv0: glam::Vec2 {
                        x: glyph.bearing_x,
                        y: (glyph.bearing_y - glyph.height),
                    },
                    uv1: glam::Vec2 {
                        x: (glyph.bearing_x + glyph.width),
                        y: glyph.bearing_y,
                    },
                    start: glyph.start as u32,
                    count: glyph.count as u32,
                });
            }
            // x += glyph.advance;
            x += self.advance;
        }
        if !found_char {
            cursor_x = x;
            cursor_y = y;
        }
        // (cursor_x, cursor_y, cursor_w, self.height)
        (cursor_x, cursor_y, self.advance, self.height)
    }

}
