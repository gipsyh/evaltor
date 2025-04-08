use clap::{Args, Parser, ValueEnum};
use shadow_rs::shadow;
use std::path::PathBuf;

/// Evaltor
#[derive(Parser, Debug, Clone)]
#[command(
    version,
    about,
    after_help = "Copyright (C) 2023 - Present, Yuheng Su <gipsyh.icu@gmail.com>. All rights reserved."
)]
#[clap(long_version = build::CLAP_LONG_VERSION)]
pub struct Options {
   
}
