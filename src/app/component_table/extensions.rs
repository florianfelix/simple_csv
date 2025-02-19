#![allow(unused)]
use std::path::PathBuf;

use text_buffer::Buffer;

pub trait BufferExt {
    fn to_cursor_string(&self) -> String;
}

impl BufferExt for Buffer {
    fn to_cursor_string(&self) -> String {
        let mut res = self.to_string();
        res.insert(self.cursor().chars(), '|');
        res
    }
}

impl BufferExt for Option<PathBuf> {
    fn to_cursor_string(&self) -> String {
        match self {
            None => String::new(),
            Some(path) => path.to_string_lossy().into_owned(),
        }
    }
}
