extern crate phf_codegen;

use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

fn main() {
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("keywords.rs");
    let mut file = BufWriter::new(File::create(&path).unwrap());

    write!(
        &mut file,
        "pub static KEYWORDS: phf::Map<&'static str, TokenType> 
="
    )
    .unwrap();
    phf_codegen::Map::new()
        .entry("false", "TokenType::False")
        .entry("speak", "TokenType::Speak")
        .entry("listen", "TokenType::Listen")
        .entry("branch", "TokenType::Branch")
        .entry("step", "TokenType::Step")
        .entry("true", "TokenType::True")
        .entry("var", "TokenType::Var")
        .entry("exit", "TokenType::Exit")
        .entry("input", "TokenType::Input")
        .entry("loop", "TokenType::Loop")
        .entry("nil", "TokenType::Nil")
        .build(&mut file)
        .unwrap();
    write!(&mut file, ";\n").unwrap();
}
