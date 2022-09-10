mod source;

use anyhow::Result;
use log::info;
use std::path::PathBuf;
use structopt::StructOpt;

use source::SourceFile;

#[derive(StructOpt, Debug)]
#[structopt(
    name = env!("CARGO_PKG_NAME"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION"),
)]
struct Opt {
    #[structopt(parse(from_os_str), help = "The source code file")]
    source: PathBuf,
}

fn main() -> Result<()> {
    // Initialize the logger
    env_logger::init();
    // Parse the command line arguments
    let opt = Opt::from_args();
    // Read dsl file
    info!("Reading dsl file");
    let source = SourceFile::open(opt.source)?;
    info!("Source file: {}", source.src.as_str());
    info!("Lines: {:?}", source.lines);
    Ok(())
}
