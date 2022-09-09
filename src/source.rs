use anyhow::Result;
use log::{debug, error, info, log_enabled, Level};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::rc::Rc;

pub struct SourceFile {
    /// The Path of the file
    pub path: PathBuf,
    /// The complete source code of the file
    pub src: Rc<String>,
    /// Locations of line beginnings
    pub lines: Rc<Vec<usize>>,
}

impl SourceFile {
    pub fn open(path: PathBuf) -> Result<Self> {
        let mut buf = String::new();
        File::open(&path)?.read_to_string(&mut buf)?;

        // Remove UTF-8 BOM, if any.
        if buf.starts_with("\u{feff}") {
            buf.drain(..3);
        }

        info!("Append \n if the file does not end with it");
        if !buf.ends_with("\n") {
            buf.push_str("\n");
        }

        info!("Count the number of lines");
        let lines = Self::analyze_lines(buf.as_str());
        Ok(SourceFile {
            path: path,
            src: Rc::new(buf),
            lines: Rc::new(lines),
        })
    }

    /// Find out all line breaks.
    pub fn analyze_lines(src: &str) -> Vec<usize> {
        let mut lines = vec![0];
        lines.extend(src.chars().enumerate().filter_map(|(i, c)| {
            if c == '\n' {
                Some(i + 1)
            } else {
                None
            }
        }));
        lines
    }
}
