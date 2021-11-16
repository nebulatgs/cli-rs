
use clap::Parser;
use ignore::WalkBuilder;
use synchronized_writer::SynchronizedWriter;
use std::{sync::{Arc, Mutex}};

use gzp::{deflate::Gzip, ZBuilder};
use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};
use reqwest::Client;
use tar::Builder;

#[derive(Parser)]
pub struct Args {
    dir: Option<String>,
}

pub async fn command(args: Args) -> super::CommandResult {
    let bytes = Vec::<u8>::new();
    let arc = Arc::new(Mutex::new(bytes));
    let mut parz = ZBuilder::<Gzip, _>::new()
        .num_threads(64)
        .from_writer(SynchronizedWriter::new(arc.clone()));
    {
        let mut archive = Builder::new(&mut parz);
        let mut builder = WalkBuilder::new(args.dir.unwrap_or(".".to_string()));
        let walker = builder.follow_links(true).hidden(false);
        let walked = walker.build().collect::<Vec<_>>();
        let pg = ProgressBar::new(walked.len() as u64).with_message("Compressing");
        pg.enable_steady_tick(100);
        pg.set_style(
            ProgressStyle::default_bar()
                .template("    {msg:.cyan} [{bar:20}] {percent}% {spinner}")
                .progress_chars("=> ")
                .tick_chars("/-\\|"),
        );
        for entry in walked.into_iter().progress_with(pg) {
            archive.append_path(entry?.path())?;
        }
    }
    parz.finish()?;
    let client = Client::new();
    let builder = client.post(format!(
        "https://backboard.railway.app/project/{}/environment/{}/up",
        "799d37b8-6c66-435f-a5af-facb54dfda0d", "6af31a80-aa95-4c4d-bc4a-a3db80cd931f"
    ));

    let res = builder
        .header(
            "project-access-token",
            "<your token here>",
        )
        .header("x-source", "cli")
        .header("Content-Type", "multipart/form-data")
        .body(arc.lock().unwrap().clone())
        .send()
        .await?;
    Ok(())
}
