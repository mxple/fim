#[derive(Debug, Clone, Copy)]
pub struct Cursor {
    pub line_pos: usize,
    pub char_pos: usize,
}

#[derive(Clone)]
pub struct Line {
    pub content: String,
}

pub struct Buffer {
    // name of file
    name: String,

    pub lines: Vec<Line>,
    cursor: Cursor,
    pub want_cursor: usize,

    range_start: Cursor,
    range_end: Cursor,

    // save dest
    file_path: String,
    is_modified: bool,

    // EXTERNAL STATE
    pub editor_mode: Mode,
}

pub struct Buffers {
    buffers: Vec<Buffer>,
    current_buffer: usize,
}

impl Cursor {
    fn new() -> Self {
        Cursor {
            line_pos: 1,
            char_pos: 0,
        }
    }
}

impl From<String> for Line {
    fn from(data: String) -> Self {
        Self {
            content: data.to_owned(),
        }
    }
}

impl Line {
    pub fn new() -> Self {
        Line {
            content: String::new(),
        }
    }

    pub fn insert(&mut self, pos: usize, text: &str) {
        debug_assert!(pos <= self.content.chars().count());
        if let Some(offset) = self.content.char_indices().nth(pos) {
            self.content.insert_str(offset.0, text);
        } else {
            self.content.insert_str(self.content.len(), text);
        }
    }

    pub fn delete_char(&mut self, pos: usize) {
        if self.content.len() == 0 {
            return;
        }
        // debug_assert!(pos < self.content.len());
        if let Some(offset) = self.content.char_indices().nth(pos) {
            self.content.remove(offset.0);
        } else {
            println!("dleeete feailed")
        }
    }

    pub fn delete_range<R>(&mut self, range: R)
    where
        R: RangeBounds<usize>,
    {
        let start = range.start_bound();
        let new_start = match start {
            std::ops::Bound::Included(&n) => {
                let offset = self.content.char_indices().nth(n).unwrap_or((0, 0 as char)).0;
                std::ops::Bound::Included(offset)
            }
            std::ops::Bound::Excluded(&n) => {
                let offset = self.content.char_indices().nth(n + 1).unwrap().0;
                std::ops::Bound::Included(offset)
            }
            _ => std::ops::Bound::Unbounded,
        };
        let end = range.end_bound();
        let new_end = match end {
            std::ops::Bound::Included(&n) => {
                if let Some(offset) = self.content.char_indices().nth(n + 1) {
                    std::ops::Bound::Excluded(offset.0)
                } else {
                    std::ops::Bound::Unbounded
                }
            }
            std::ops::Bound::Excluded(&n) => {
                if let Some(offset) = self.content.char_indices().nth(n) {
                    std::ops::Bound::Excluded(offset.0)
                } else {
                    std::ops::Bound::Unbounded
                }
            }
            _ => std::ops::Bound::Unbounded,
        };
        self.content.replace_range((new_start, new_end), "");
    }

    pub fn len(&self) -> usize {
        self.content.chars().count()
    }
}

impl Buffer {
    pub fn new() -> Self {
        Buffer {
            name: String::from(""),

            lines: Vec::from([Line::new()]),
            cursor: Cursor::new(),
            want_cursor: 0,

            range_start: Cursor::new(),
            range_end: Cursor::new(),

            file_path: String::from(""),
            is_modified: false,

            editor_mode: Mode::Insert,
        }
    }

    pub fn from_file(file_path: &str) -> Self {
        let path = Path::new(file_path);
        let file = match std::fs::File::open(file_path) {
            Ok(file) => file,
            Err(_) => std::fs::File::create(file_path).unwrap(),
        };
        let reader = BufReader::new(file);
        let lines: Vec<Line> = reader
            .lines()
            .map(|line| Line::from(line.expect("Failed to read line"))).collect();

        Buffer {
            name: String::from(path.file_name().and_then(|name| name.to_str()).unwrap()),

            lines,
            cursor: Cursor::new(),
            want_cursor: 0,

            range_start: Cursor::new(),
            range_end: Cursor::new(),

            file_path: file_path.to_string(),
            is_modified: false,

            editor_mode: Mode::Insert,
        }
    }

    pub fn line_count(&self) -> usize {
        self.lines.len()
    }

    pub fn cursor(&self) -> &Cursor {
        &self.cursor
    }

    pub fn char_pos(&self) -> usize {
        self.cursor().char_pos
    }

    pub fn line_pos(&self) -> usize {
        self.cursor().line_pos
    }
}

// Line accessor methods
impl Buffer {
    pub fn curr_line_offset_mut(&mut self, offset: isize) -> Option<&mut Line> {
        let offset_adj: isize = isize::try_from(self.cursor.line_pos).unwrap() + offset - 1;

        if offset_adj >= 0 {
            Some(
                self.lines
                    .get_mut(offset_adj as usize)
                    .expect("requested line out of bounds"),
            )
        } else {
            None
        }
    }

    pub fn curr_line_offset(&self, offset: isize) -> Option<&Line> {
        let offset_adj: isize = isize::try_from(self.cursor.line_pos).unwrap() + offset - 1;

        if offset_adj >= 0 {
            Some(
                self.lines
                    .get(offset_adj as usize)
                    .expect("requested line out of bounds"),
            )
        } else {
            None
        }
    }

    // pub fn curr_line_offset_mut_clamped(&mut self, offset: isize) -> &mut Line {
    //     let offset_adj: isize = isize::try_from(self.cursor.line_pos).unwrap() + offset - 1;
    //
    //     let len: isize = self.line_count().try_into().unwrap();
    //     unsafe {
    //         self.lines
    //             .get_unchecked_mut(offset_adj.clamp(0, len) as usize)
    //     }
    // }
    //
    // pub fn curr_line_offset_clamped(&self, offset: isize) -> &Line {
    //     let offset_adj: isize = isize::try_from(self.cursor.line_pos).unwrap() + offset - 1;
    //
    //     let len: isize = self.lines.len().try_into().unwrap();
    //     unsafe { self.lines.get_unchecked(offset_adj.clamp(0, len) as usize) }
    // }

    pub fn curr_line_mut(&mut self) -> &mut Line {
        self.lines
            .get_mut(self.cursor.line_pos - 1)
            .expect("requested line out of bounds")
    }

    pub fn curr_line(&self) -> &Line {
        self.lines
            .get(self.cursor.line_pos - 1)
            .expect("requested line out of bounds")
    }

    pub fn get_line_mut(&mut self, i: usize) -> &mut Line {
        self.lines.get_mut(i).expect("requested line out of bounds")
    }

    pub fn get_line(&self, i: usize) -> &Line {
        self.lines.get(i).expect("requested line out of bounds")
    }
}

impl Buffer {
    pub fn insert_text(&mut self, text: &str, move_cursor: bool) {
        let old_cursor = self.cursor().clone();
        let old_want = self.want_cursor;
        for (offset, to_insert) in text.split("\n").enumerate() {
            if offset != 0 {
                self.insert_lines(1, true);
            }
            let char_pos = self.char_pos();
            self.curr_line_mut().insert(char_pos, to_insert);
            self.move_cursor_by(to_insert.chars().count() as isize, 0, true);
        }
        self.is_modified = true;
        if !move_cursor {
            self.cursor = old_cursor;
            self.want_cursor = old_want;
        }
    }

    pub fn delete_char_cursor(&mut self) {
        let char_pos = self.char_pos();
        self.curr_line_mut().delete_char(char_pos);
        self.is_modified = true;
    }

    pub fn delete_text<R>(&mut self, range: R)
    where
        R: RangeBounds<usize>,
    {
        self.curr_line_mut().delete_range(range);
        self.is_modified = true;
    }

    pub fn insert_lines_above(&mut self, amount: usize, move_cursor: bool) {
        self.lines.splice(
            (self.line_pos()-1)..(self.line_pos()-1),
            iter::repeat(Line::new()).take(amount),
        );
        if !move_cursor {
            self.move_cursor_by(0, amount as isize, true);
        }
        self.move_cursor_by(0, 0, true);
    }

    pub fn insert_lines(&mut self, amount: usize, move_cursor: bool) {
        self.lines.splice(
            self.line_pos()..self.line_pos(),
            iter::repeat(Line::new()).take(amount),
        );
        if move_cursor {
            self.move_cursor_by(0, amount as isize, true);
        }
    }

    pub fn delete_line_curr(&mut self) {
        let idx = self.line_pos() - 1;
        self.lines.remove(idx);
        self.move_cursor_by(0, 0, true); // sync cursors and clamps
    }

    // clamps
    pub fn delete_lines(&mut self, range: Range<usize>) {
        let mut range_adj = range;
        range_adj.start -= 1;
        range_adj.end = (range_adj.end - 1).clamp(0, self.line_count());
        self.lines.drain(range_adj);
        self.is_modified = true;
    }

    pub fn join_line_below(&mut self) {
        let next_idx = self.line_pos();
        let s: String = mem::take(&mut self.lines[next_idx].content);
        self.insert_text(&s, false);
        self.delete_lines((self.line_pos() + 1)..self.line_pos() + 2);
    }

    pub fn split_line_below(&mut self) {
        self.insert_lines(1, false);
        if let Some(offset) = self.curr_line().content.char_indices().nth(self.char_pos()) {
            let s: String =
                mem::take(&mut self.curr_line().content.split_at(offset.0).1.to_owned());
            self.curr_line_offset_mut(1).unwrap().content.push_str(&s);
            let range = self.char_pos()..self.curr_line().len();
            self.curr_line_mut().delete_range(range);
        }
    }
}

// cursor movement
impl Buffer {
    pub fn move_cursor_by(&mut self, offset_char: isize, offset_line: isize, sync: bool) {
        self.cursor.line_pos = (self.cursor.line_pos as isize + offset_line)
            .clamp(1, self.lines.len() as isize) as usize;

        if offset_char != 0 {
            self.want_cursor =
                (self.char_pos() as isize + offset_char).clamp(0, isize::MAX) as usize;
        }

        self.move_to_want();

        if sync {
            self.want_cursor = self.cursor.char_pos;
        }
    }

    pub fn move_cursor_line_to(&mut self, dest_line: usize) {
        self.cursor.line_pos = dest_line.clamp(1, self.lines.len());
    }

    pub fn move_cursor_char_to(&mut self, dest_char: usize) {
        self.want_cursor = dest_char;
        self.move_to_want();
    }

    pub fn move_cursor_char_w(&mut self) {}
    pub fn move_cursor_char_b(&mut self) {}
    pub fn move_cursor_char_bigw(&mut self) {}
    pub fn move_cursor_char_bigb(&mut self) {}

    pub fn move_cursor_to_last_line(&mut self) {
        self.cursor.line_pos = usize::MAX.clamp(0, self.lines.len());
        self.move_to_want();
    }

    pub fn move_cursor_to_last_char(&mut self) {
        self.want_cursor = isize::MAX as usize;
        self.move_to_want();
    }

    fn move_to_want(&mut self) {
        if self.editor_mode == Mode::Insert {
            self.cursor.char_pos = self.want_cursor.clamp(0, self.curr_line().len());
        } else {
            self.cursor.char_pos = self
                .want_cursor
                .clamp(0, (self.curr_line().len().max(1)) - 1);
        }
    }
}

impl Buffers {
    pub fn new() -> Self {
        Buffers {
            buffers: Vec::from([Buffer::new()]),
            current_buffer: 0,
        }
    }

    pub fn from_file(file_path: &str) -> Self {
        Buffers {
            buffers: Vec::from([Buffer::from_file(file_path)]),
            current_buffer: 0,
        }
    }

    pub fn curr_buffer(&self) -> &Buffer {
        let buf_idx = self.current_buffer;
        self.buffers
            .get(buf_idx)
            .expect("Current buffer index out of bounds")
    }

    pub fn curr_buffer_mut(&mut self) -> &mut Buffer {
        let buf_idx = self.current_buffer;
        self.buffers
            .get_mut(buf_idx)
            .expect("Current buffer index out of bounds")
    }
}

use std::{
    fmt,
    fs::File,
    io::{BufRead, BufReader, IsTerminal, Read, Write},
    iter, mem,
    ops::{Range, RangeBounds}, path::Path,
};

use super::Mode;

impl fmt::Display for Buffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (line, num) in self.lines.iter().zip(1..=self.lines.len()) {
            if num == self.cursor.line_pos {
                writeln!(
                    f,
                    "{} \t| {}{}{}",
                    num,
                    line.content.get(..self.char_pos()).unwrap(),
                    "█",
                    line.content.get(self.char_pos() + 1..).unwrap_or("")
                )
                .unwrap();
            } else {
                writeln!(f, "{} \t| {}", num, line.content).unwrap();
            }
        }
        writeln!(f, "{}, {}", self.cursor.char_pos, self.cursor.line_pos)
    }
}
