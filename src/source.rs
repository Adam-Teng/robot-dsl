use anyhow::Result;
use log::info;
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
    pub lines: Rc<Vec<String>>,
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

        let lines = Self::analyze_lines(buf.as_str());
        Ok(SourceFile {
            path: path,
            src: Rc::new(buf),
            lines: Rc::new(lines),
        })
    }

    /// Split lines and Remove unused things.
    pub fn analyze_lines(src: &str) -> Vec<String> {
        let mut lines = src
            .lines()
            .map(|line| line.to_string())
            .collect::<Vec<String>>();
        for line in lines.iter_mut() {
            // Remove comments
            if let Some(pos) = line.find("#") {
                line.truncate(pos);
            }
            // Remove spaces
            *line = line.trim().to_string();
        }
        // Remove empty lines
        let new_lines = lines
            .iter()
            .filter(|line| !line.is_empty())
            .map(|line| line.to_string())
            .collect::<Vec<String>>();
        return new_lines;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_analyze_lines() {
        let src = "  # comment\n\n\n   aaa";
        let lines = SourceFile::analyze_lines(src);
        assert_eq!(lines, vec!["aaa".to_string()]);
    }
}
