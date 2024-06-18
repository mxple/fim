use std::{collections::HashMap, mem::swap};

use font_kit::{
    family_name::FamilyName, handle::Handle, properties::Properties, source::SystemSource,
};
use ft::{ffi::FT_OUTLINE_REVERSE_FILL, Face, Outline, Vector};
use glam::Vec2;

// Loads fonts from the system
pub struct FontManager {
    handles: Vec<Handle>,
    ft_lib: ft::Library,
    system_source: SystemSource,
}

#[allow(dead_code)]
pub struct Curve {
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    x3: f32,
    y3: f32,
}

impl Curve {
    fn new(a: &glam::Vec2, b: &glam::Vec2, c: &glam::Vec2) -> Self {
        Curve {
            x1: a.x,
            y1: a.y,
            x2: b.x,
            y2: b.y,
            x3: c.x,
            y3: c.y,
        }
    }
}

// data regarding a glyph and pointer+size into the curve buffer
pub struct GlyphData {
    // glyph_index: u32,
    pub start: usize,
    pub count: usize,

    pub width: f32,
    pub height: f32,
    pub bearing_x: f32,
    pub bearing_y: f32,
    pub advance: f32,
}

impl FontManager {
    pub fn new() -> Self {
        Self {
            handles: Vec::new(),
            ft_lib: ft::Library::init().expect("Freetype lib init failed"),
            system_source: SystemSource::new(),
        }
    }

    pub fn load_main_font(
        &mut self,
        name: &str,
        curve_data: &mut Vec<Curve>,
        glyph_map: &mut HashMap<u32, GlyphData>,
        advance: &mut f32,
        height: &mut f32,
    ) {
        let handle = self
            .system_source
            .select_best_match(&[FamilyName::Title(name.into())], &Properties::new())
            .unwrap_or(
                SystemSource::new()
                    .select_best_match(&[FamilyName::Monospace], &Properties::new())
                    .unwrap_or(Handle::Memory {
                        bytes: (Vec::from(include_bytes!("../../assets/fonts/FreeMono.otf"))
                            .into()),
                        font_index: (0),
                    }),
            );

        let face;
        match handle {
            font_kit::handle::Handle::Path { ref path, .. } => {
                face = self.ft_lib.new_face(path, 0).unwrap();
            }
            font_kit::handle::Handle::Memory { ref bytes, .. } => {
                face = self.ft_lib.new_memory_face((**bytes).clone(), 0).unwrap();
                // panic!("we should not be loading memory");
            }
        }

        let em_size = face.em_size() as f32;
        *height = face.height() as f32 / em_size;

        let load_flags = ft::face::LoadFlag::NO_SCALE
            | ft::face::LoadFlag::NO_HINTING
            | ft::face::LoadFlag::NO_BITMAP;

        match face.load_glyph(0, load_flags) {
            Ok(_) => {
                Self::build_glyph(&face, curve_data, glyph_map, 0);
            }
            Err(e) => {
                println!("Unable to load the undefined glyph. {}", e);
            }
        }

        for char_code in 32..2560000 {
            let Some(glyph_index) = face.get_char_index(char_code) else {
                continue;
            };
            match face.load_glyph(glyph_index, load_flags) {
                Ok(_) => {
                    Self::build_glyph(&face, curve_data, glyph_map, char_code as u32);
                }
                Err(e) => {
                    println!(
                        "Unable to load the glyph for {}. {}",
                        std::char::from_u32(char_code as u32).unwrap(),
                        e
                    );
                }
            }
        }

        *advance = glyph_map.get(&(' ' as u32)).unwrap_or(glyph_map.get(&0).unwrap()).advance;
        self.handles.push(handle);
    }

    pub fn prepare_fallback_fonts(&mut self) {
        self.handles.extend(self.system_source.all_fonts().expect("Failed to load fonts"));
    }

    pub fn load_glyphs_in_str(
        &self,
        curve_data: &mut Vec<Curve>,
        glyph_map: &mut HashMap<u32, GlyphData>,
        s: &str,
    ) -> bool {
        let mut modified: bool = false;

        let load_flags = ft::face::LoadFlag::NO_SCALE
            | ft::face::LoadFlag::NO_HINTING
            | ft::face::LoadFlag::NO_BITMAP;

        for c in s.chars() {
            if glyph_map.get(&(c as u32)).is_some() {
                continue;
            }
            if c == '\r' || c == '\n' {
                continue;
            }

            for handle in &self.handles {
                let face;
                match handle {
                    font_kit::handle::Handle::Path { path, .. } => {
                        face = self.ft_lib.new_face(path, 0).unwrap();
                    }
                    font_kit::handle::Handle::Memory { .. } => {
                        // face = self.ft_lib.new_memory_face((**bytes), 0).unwrap();
                        panic!("why we loading memory");
                    }
                }

                if let Some(glyph_index) = face.get_char_index(c as usize) {
                    match face.load_glyph(glyph_index, load_flags) {
                        Ok(_) => {
                            Self::build_glyph(
                                &face,
                                curve_data,
                                glyph_map,
                                c as u32,
                                // glyph_index,
                            );
                            modified = true;
                        }
                        Err(e) => {
                            println!(
                                "Unable to load the glyph for {}. {}",
                                std::char::from_u32(c as u32).unwrap(),
                                e
                            );
                        }
                    }
                    break;
                }
            }
        }

        // make sure to reload shader buffers
        modified
    }

    fn build_glyph(
        face: &Face,
        curve_data: &mut Vec<Curve>,
        glyph_map: &mut HashMap<u32, GlyphData>,
        char_code: u32,
        // glyph_index: u32,
    ) {
        let start = curve_data.len();
        let em_size = face.em_size() as f32;

        // contours is a list of points where a contour, a collection of bezier curves, ends
        // 0-c_1, c_1-c_2, ... , c_(n-1)-c_n
        Self::process_contour(curve_data, face.glyph().outline().expect(""), em_size);

        let g: GlyphData = GlyphData {
            // glyph_index,
            start,
            count: curve_data.len() - start,
            width: face.glyph().metrics().width as f32 / em_size,
            height: face.glyph().metrics().height as f32 / em_size,
            bearing_x: face.glyph().metrics().horiBearingX as f32 / em_size,
            bearing_y: face.glyph().metrics().horiBearingY as f32 / em_size,
            advance: face.glyph().metrics().horiAdvance as f32 / em_size,
        };
        glyph_map.insert(char_code, g);
    }

    fn process_contour(curve_data: &mut Vec<Curve>, outline: Outline, em_size: f32) {
        let convert = |v: &Vector| glam::Vec2 {
            x: v.x as f32 / em_size,
            y: v.y as f32 / em_size,
        };

        let start_idx = curve_data.len();

        let contours = outline.contours_iter();
        for curve in contours {
            let mut p1: Vector = *curve.start();
            for c in curve {
                match c {
                    ft::outline::Curve::Line(p2) => {
                        curve_data.push(Curve::new(
                            &convert(&p1),
                            &midpoint(&convert(&p1), &convert(&p2)),
                            &convert(&p2),
                        ));
                        p1 = p2;
                    }
                    ft::outline::Curve::Bezier2(p2, p3) => {
                        curve_data.push(Curve::new(&convert(&p1), &convert(&p2), &convert(&p3)));
                        p1 = p3;
                    }
                    ft::outline::Curve::Bezier3(p2, p3, p4) => {
                        // Quadratic Approximation of Cubic Curves
                        // https://ttnghia.github.io/pdf/QuadraticApproximation.pdf
                        // https://doi.org/10.1145/3406178
                        let c0 = glam::Vec2::new(
                            p1.x as f32 + (p2.x - p1.x) as f32 * 0.75,
                            p1.y as f32 + (p2.y - p1.y) as f32 * 0.75,
                        ) / em_size;
                        let c1 = glam::Vec2::new(
                            p4.x as f32 + (p3.x - p4.x) as f32 * 0.75,
                            p4.y as f32 + (p3.y - p4.y) as f32 * 0.75,
                        ) / em_size;
                        let d = (c0 + c1) * 0.5;
                        curve_data.push(Curve::new(&convert(&p1), &c0, &d));
                        curve_data.push(Curve::new(&d, &c1, &convert(&p4)));
                        p1 = p4;
                    }
                }
            }
        }

        if outline.flags() & FT_OUTLINE_REVERSE_FILL == 0 {
            for i in curve_data.iter_mut().skip(start_idx) {
                swap(&mut i.x1, &mut i.x3);
                swap(&mut i.y1, &mut i.y3);
            }
        }
    }
}

fn midpoint(a: &Vec2, b: &Vec2) -> Vec2 {
    (*a + *b) * 0.5
}
