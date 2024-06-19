use std::{env, io, sync::LazyLock, time::Instant};
use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};

use editor::Editor;
use renderer::{
    camera::Camera, cursor_renderer::CursorRenderer, primitive_renderer::PrimitiveRenderer,
    text_renderer::TextRenderer,
};
use sdl2::rect::Rect;
extern crate freetype as ft;
extern crate gl;
extern crate sdl2;

pub mod editor;
pub mod renderer;

static START_TIME: LazyLock<Instant> = LazyLock::new(|| Instant::now());

fn main() {
    let _ = *START_TIME;

    let args: Vec<String> = env::args().collect();
    let no_path = String::from("");
    let file_path = args.get(1).unwrap_or(&no_path);

    // load sdl + gl
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    //
    // set up window + context
    let window = video_subsystem
        .window("fim", 1200, 800)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);

    video_subsystem.gl_set_swap_interval(1).unwrap();
    video_subsystem.text_input().start();
    video_subsystem
        .text_input()
        .set_rect(Rect::new(0, 0, 300, 100));

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 6);
    gl_attr.set_double_buffer(true); // Enable double buffering
    gl_attr.set_multisample_buffers(1); // Enable multisampling if desired

    // let display_mode = video_subsystem.current_display_mode(0).unwrap();
    // let sw2 = display_mode.w / 2;
    // let sh2 = display_mode.h / 2;

    // load these once at the start of your program
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let syntax = ps.find_syntax_by_extension("rs").unwrap();
    let mut h = HighlightLines::new(syntax, &ts.themes["base16-mocha.dark"]);

    // load renderer(s)
    // let mut r2d = Renderer::new();
    let mut txr = TextRenderer::new("Free Mono");
    let mut cur = CursorRenderer::new();

    let mut camera = Camera::new(
        glam::Vec3::new(0., 0., 4.),
        glam::Vec3::new(0., 0., 0.),
        glam::Vec3::new(0., 1., 0.),
    );

    // load editor
    let mut editor = Editor::new(file_path);

    let mut cursor_prev: (f32, f32) = (0., 0.);
    let mut cam_z = 20.;

    let mut start = Instant::now();
    let mut event_pump = sdl_context.event_pump().unwrap();
    'main_loop: loop {
        let mut skip_events = false;
        for event in event_pump.poll_iter() {
            if skip_events {
                continue;
            }

            // dbg!(&event);
            match event {
                sdl2::event::Event::Quit { .. } => break 'main_loop,
                sdl2::event::Event::TextInput { text, .. } => {
                    editor.handle_text_input(&text);
                }
                sdl2::event::Event::KeyDown {
                    keycode, keymod, ..
                } => {
                    editor.handle_keypress(keycode.unwrap(), keymod, &mut skip_events);
                }
                sdl2::event::Event::MouseWheel { precise_y, .. } => {
                    cam_z += cam_z * precise_y * 0.1;
                    camera.update_view();
                }
                sdl2::event::Event::Window { win_event, .. } => match win_event {
                    sdl2::event::WindowEvent::Resized(w, h) => {
                        camera.set_perspective(3.14 / 4., w as f32 / h as f32)
                    }
                    _ => (),
                },
                _ => (),
            }
        }

        let (win_x, win_y) = window.position();
        let (ww, wh) = window.size();
        // let aspect = sw2 as f32 / sh2 as f32;
        unsafe { gl::Viewport(0, 0, ww as i32, wh as i32) }
        // camera.set_perspective(3.14 / 4., ww as f32 / wh as f32);

        // print!("\x1b[2J\x1b[H");
        // print!("{}", editor.buffers.curr_buffer());
        txr.begin_scene();
        // portal effect
        // let (x, y, w, h) = txr.draw_text(
        //     -win_x as f32 / sh2 as f32 * 8.,
        //     win_y as f32 / sh2 as f32 * 8.,
        //     &editor.get_text(),
        //     20., Some(editor.get_cursor()));
        // let (mut x, mut y, w, h) =
        //     txr.draw_text(0., 0., &editor.get_text_newlines(), f32::MAX, Some(editor.get_cursor()));
        // dbg!(&ts.themes.keys());
        for line in editor.get_text() {
            // LinesWithEndings enables use of newlines mode
            let ranges: Vec<(Style, &str)> = h.highlight_line(&line.content, &ps).unwrap();
            for r in &ranges {
                txr.type_writer(r.1, r.0.foreground);
            }
            txr.typewriter_new_line();
        }
        txr.reset_typewriter();

        let x = editor.get_cursor().1 as f32 * txr.advance;
        let y = (editor.get_cursor().0 - 1) as f32 * -txr.height;
        let w = txr.advance;
        let h = txr.height;

        cursor_prev = cur.draw_cursor_at(x, y - 0.2, w, h, cursor_prev.0, cursor_prev.1);

        let orig = glam::Vec3::new(x + w / 2., y + h / 2. - 0.2, 0.);

        // camera.pos.x -= (camera.pos.x - (orig.x + (win_x  as f32 + ww as f32/2.)/aspect)) * 0.03;
        camera.pos.x -= (camera.pos.x - orig.x) * 0.03;
        camera.pos.y -= (camera.pos.y - orig.y) * 0.03;
        camera.pos.z = cam_z;

        // dbg!(win_x, win_y);
        // camera.dx(win_x as f32 / sh2 as f32 - aspect + orig.x/2., (win_x + ww as i32) as f32 / sh2 as f32 - aspect + orig.x/2.,
        //           -win_y as f32/ sh2 as f32 + 1. + orig.y/2., -(win_y + wh as i32) as f32/ sh2 as f32 + 1. + orig.y/2.,
        //           4., 150.);

        camera.to -= (camera.to - orig) * 0.05;
        camera.update_view();

        unsafe {
            gl::Enable(gl::BLEND);
            gl::BlendEquation(gl::FUNC_ADD);
            gl::BlendFunc(gl::ONE, gl::ONE);

            // gl::ClearColor(0., 0., 0., 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            // r2d.end_scene();
            txr.flush(&camera);
            cur.draw(&camera);
        }

        let end = Instant::now();
        window.gl_swap_window();
        println!("Time taken: {}", (end - start).as_micros());
        start = Instant::now();
    }
    // free resources
}
