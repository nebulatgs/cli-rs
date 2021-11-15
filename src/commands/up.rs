use std::io::Write;
use std::path::Path;

use clap::Parser;
use ignore::WalkBuilder;

use flate2::write::GzEncoder;
use flate2::Compression;
use tar::Builder;

#[derive(Parser)]
pub struct Args {
    dir: Option<String>,
}

pub fn command(args: Args) -> super::CommandResult {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    {
        let mut archive = Builder::new(&mut encoder);
        for entry in WalkBuilder::new(args.dir.unwrap_or(".".to_string()))
            .follow_links(true)
            .hidden(false)
            .build()
        {
            archive.append_path(entry?.path())?;
        }
    }
    let tar = encoder.finish()?;
    Ok(())
}
