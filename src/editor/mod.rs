use std::{env, path::PathBuf};

use sdl2::keyboard::{Keycode, Mod};

pub mod buffer;

use buffer::Buffers;

#[derive(Eq, PartialEq)]
pub enum Mode {
    Normal,
    Insert,
    Visual,
    VisualLine,
    Replace,
    Command,
}

pub struct Editor {
    working_dir: PathBuf,

    mode: Mode,
    pub buffers: Buffers,

    yank_buffer: String,
    insert_buffer: String,
}

impl Editor {
    pub fn new(file_path: &str) -> Self {
        let buffers;
        if file_path.is_empty() {
            buffers = Buffers::new();
        } else {
            buffers = Buffers::from_file(file_path);
        }
        Editor {
            // TODO handle possible error
            working_dir: env::current_dir().unwrap(),

            mode: Mode::Normal,
            buffers,

            yank_buffer: String::from(""),
            insert_buffer: String::from(""),
        }
    }

    fn handle_keypress_normal(&mut self, keycode: Keycode, keymod: Mod, skip_events: &mut bool) {
        let buf = self.buffers.curr_buffer_mut();
        match keycode {
            Keycode::Return => {
                buf.insert_lines(1, true);
            }
            Keycode::Up => {
                buf.move_cursor_by(0, -1, false);
            }
            Keycode::Down => {
                buf.move_cursor_by(0, 1, false);
            }
            Keycode::Right => {
                buf.move_cursor_by(1, 0, true);
            }
            Keycode::Left => {
                buf.move_cursor_by(-1, 0, true);
            }
            Keycode::A => {
                self.mode = Mode::Insert;
                buf.editor_mode = Mode::Insert;
                *skip_events = true;
                buf.move_cursor_by(1, 0, true);
            }
            Keycode::D => {
                if buf.line_count() == 1 {
                    buf.delete_text(0..);
                    buf.move_cursor_by(0, 0, true);
                } else {
                    buf.delete_line_curr();
                }
            }
            Keycode::H => {
                buf.move_cursor_by(-1, 0, true);
            }
            Keycode::I => {
                self.mode = Mode::Insert;
                buf.editor_mode = Mode::Insert;
                *skip_events = true;
            }
            Keycode::J => {
                buf.move_cursor_by(0, 1, false);
            }
            Keycode::K => {
                buf.move_cursor_by(0, -1, false);
            }
            Keycode::L => {
                buf.move_cursor_by(1, 0, true);
            }
            Keycode::O => {
                if is_upper(keymod) {
                    self.mode = Mode::Insert;
                    buf.editor_mode = Mode::Insert;
                    *skip_events = true;
                    buf.insert_lines_above(1, true);
                }
                if is_lower(keymod) {
                    self.mode = Mode::Insert;
                    buf.editor_mode = Mode::Insert;
                    *skip_events = true;
                    buf.insert_lines(1, true);
                }
            }
            Keycode::X => {
                buf.delete_char_cursor();
                buf.move_cursor_by(0, 0, true); // update cursor keep in bounds
            }
            Keycode::Num0 => {
                buf.move_cursor_char_to(0);
            }
            Keycode::Num4 => {
                if (keymod & (Mod::LSHIFTMOD | Mod::RSHIFTMOD)).bits() != 0 {
                    buf.move_cursor_to_last_char();
                }
            }
            _ => (),
        }
    }

    fn handle_keypress_insert(&mut self, keycode: Keycode, keymod: Mod, skip_events: &mut bool) {
        let buf = self.buffers.curr_buffer_mut();
        match keycode {
            Keycode::Escape => {
                self.mode = Mode::Normal;
                buf.editor_mode = Mode::Normal;
                buf.move_cursor_by(-1, 0, true);
            }
            Keycode::Backspace => {
                if buf.char_pos() == 0 {
                    if buf.curr_line_offset_mut(-1).is_some() {
                        buf.move_cursor_by(0, -1, true);
                        buf.move_cursor_to_last_char();
                        buf.move_cursor_by(0, 0, true);
                        buf.join_line_below();
                    }
                } else {
                    buf.move_cursor_by(-1, 0, true);
                    buf.delete_char_cursor();
                }
            }
            Keycode::Return => {
                buf.split_line_below();
                buf.move_cursor_by(0, 1, true);
                buf.move_cursor_char_to(0);
            }
            Keycode::Up => {
                buf.move_cursor_by(0, -1, false);
            }
            Keycode::Down => {
                buf.move_cursor_by(0, 1, false);
            }
            Keycode::Right => {
                buf.move_cursor_by(1, 0, true);
            }
            Keycode::Left => {
                buf.move_cursor_by(-1, 0, true);
            }
            Keycode::Tab => {
                buf.insert_text("    ", true);
            }
            _ => (),
        }
    }

    pub fn handle_keypress(&mut self, keycode: Keycode, keymod: Mod, skip_events: &mut bool) {
        match self.mode {
            Mode::Normal => self.handle_keypress_normal(keycode, keymod, skip_events),
            Mode::Insert => self.handle_keypress_insert(keycode, keymod, skip_events),
            _ => (),
        }
    }

    pub fn handle_text_input(&mut self, text: &str) {
        if self.mode == Mode::Insert {
            self.buffers.curr_buffer_mut().insert_text(text, true);
        }
        // if self.mode == Mode::Replace {
        //     self.buffers.curr_buffer_mut().replace_text(text);
        // }
    }

    pub fn get_text(&self) -> String {
        let mut result = String::new();
        for s in &self.buffers.curr_buffer().lines {
            result.push_str(&s.content);
            result.push('\n');
        }
        result.pop();
        result
    }

    pub fn get_cursor(&self) -> (u32, u32) {
        let c = self.buffers.curr_buffer().cursor();
        (c.line_pos as u32, c.char_pos as u32)
    }
}

// false does not mean uppercase
fn is_lower(keymod: Mod) -> bool {
    ((keymod.intersects(Mod::CAPSMOD)) && is_shift(keymod - Mod::CAPSMOD)) || keymod == Mod::NOMOD
}

// false does not mean lowercase
fn is_upper(keymod: Mod) -> bool {
    is_shift(keymod) || keymod == Mod::CAPSMOD
}

// only shift
fn is_shift(keymod: Mod) -> bool {
    keymod == Mod::LSHIFTMOD
        || keymod == Mod::RSHIFTMOD
        || keymod == (Mod::RSHIFTMOD | Mod::LSHIFTMOD)
}

// only ctrl
fn is_ctrl(keymod: Mod) -> bool {
    keymod == Mod::LCTRLMOD || keymod == Mod::RCTRLMOD || keymod == (Mod::RCTRLMOD & Mod::LCTRLMOD)
}

// only alt
fn is_alt(keymod: Mod) -> bool {
    keymod == Mod::LALTMOD || keymod == Mod::RALTMOD || keymod == (Mod::RALTMOD & Mod::LALTMOD)
}

// shift + ctrl only (no super alt or whatev)
fn is_shift_ctrl(keymod: Mod) -> bool {
    is_shift(keymod & !(Mod::LCTRLMOD | Mod::RCTRLMOD))
        || is_ctrl(keymod & !(Mod::LSHIFTMOD | Mod::RSHIFTMOD))
}
