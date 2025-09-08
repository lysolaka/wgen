// Copyright 2025 Szymon Sztuka (lysolaka)
// Distributed under the terms of the GNU General Public License v3 or later

#![allow(unused)]

use simplelog::*;

use minijinja::{AutoEscape, Environment};

mod discover;
mod render;
mod spec;
mod tree;

use crate::render::{is_empty, is_page, is_section};

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

    let mut env = Environment::new();
    env.set_auto_escape_callback(|_| AutoEscape::None);
    env.add_test("page", is_page);
    env.add_test("section", is_section);
    env.add_test("empty", is_empty);
    env.add_template("base.html", include_str!("./templates/base.html"))?;
    env.add_template("index.html", include_str!("./templates/index.html"))?;
    env.add_template("content.html", include_str!("./templates/content.html"))?;

    let structure = discover::Structure::collect_from(std::path::Path::new("example/"))?;
    let tree = structure.into_tree()?;
    tree.render_pages(std::path::Path::new("/tmp/wgen_out"), &env)?;
    tree.render_indexes(std::path::Path::new("/tmp/wgen_out"), &env)?;

    Ok(())
}
