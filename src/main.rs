#![allow(unused)]

use simplelog::*;

mod discover;
mod spec;
mod tree;

fn main() -> anyhow::Result<()> {
    TermLogger::init(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    );

    let structure = discover::Structure::collect_from(std::path::Path::new("spec2"))?;
    let tree = structure.into_tree()?;

    println!("{:#?}", tree);

    Ok(())
}
