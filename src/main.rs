// Copyright 2025 Szymon Sztuka (lysolaka)
// Distributed under the terms of the GNU General Public License v3 or later

#![allow(unused)]

use simplelog::*;

mod discover;
mod spec;
mod tree;

fn main() -> anyhow::Result<()> {
    let logger_cfg = ConfigBuilder::new()
        .set_time_level(LevelFilter::Off)
        .set_location_level(LevelFilter::Off)
        .set_target_level(LevelFilter::Off)
        .set_thread_level(LevelFilter::Off)
        .build();

    TermLogger::init(
        LevelFilter::Trace,
        logger_cfg,
        TerminalMode::Mixed,
        ColorChoice::Auto,
    );

    let structure = discover::Structure::collect_from(std::path::Path::new("spec2"))?;
    let tree = structure.into_tree()?;

    // println!("{:#?}", tree);

    Ok(())
}
