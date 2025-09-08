// Copyright 2025 Szymon Sztuka (lysolaka)
// Distributed under the terms of the GNU General Public License v3 or later

use markdown::{Constructs, Options, ParseOptions};
use minijinja::{AutoEscape, Environment, context};
use std::fs;
use std::path::Path;
use std::io::prelude::*;

use crate::tree::*;

const SCRIPT_JS: &'static str = include_str!("./templates/script.js");
const STYLE_CSS: &'static str = include_str!("./templates/style.css");

impl Tree {
    pub fn sections(&self) -> impl Iterator<Item = &Section> {
        self.iter().filter_map(|e| match e {
            TreeEntry::Section(section) => Some(section),
            _ => None,
        })
    }

    pub fn subsections(&self) -> impl Iterator<Item = &Subsection> {
        self.sections()
            .flat_map(|s| s.iter())
            .filter_map(|e| match e {
                SectionEntry::Subsection(subsection) => Some(subsection),
                _ => None,
            })
    }

    pub fn pages(&self) -> impl Iterator<Item = &Page> {
        let root_pages = self.iter().filter_map(|e| match e {
            TreeEntry::Page(page) => Some(page),
            _ => None,
        });

        let sec_pages = self
            .sections()
            .flat_map(|s| s.iter())
            .filter_map(|e| match e {
                SectionEntry::Page(page) => Some(page),
                _ => None,
            });

        let sub_pages = self.subsections().flat_map(|s| s.iter());

        root_pages.chain(sec_pages).chain(sub_pages)
    }

    pub fn render(&self, outdir: &Path) -> anyhow::Result<()> {
        let md_opts = Options {
            parse: ParseOptions {
                constructs: Constructs {
                    math_flow: true,
                    math_text: true,
                    ..Constructs::gfm()
                },
                ..ParseOptions::gfm()
            },
            ..Options::gfm()
        };

        let mut env = Environment::new();
        env.set_auto_escape_callback(|_| AutoEscape::None);
        env.add_test("page", is_page);
        env.add_test("section", is_section);
        env.add_test("empty", is_empty);
        env.add_template("base.html", include_str!("./templates/base.html"))?;
        env.add_template("sec_index.html", include_str!("./templates/sec_index.html"))?;
        env.add_template("sub_index.html", include_str!("./templates/sub_index.html"))?;
        env.add_template("sidebar.html", include_str!("./templates/sidebar.html"))?;
        env.add_template("content.html", include_str!("./templates/content.html"))?;

        self.render_sections(outdir, &env)?;
        self.render_subsections(outdir, &env)?;
        self.render_pages(outdir, &env)?;

        // render sidebar
        let outpath = outdir.join("sidebar.html");
        log::info!("Rendering sidebar to {}", outpath.display());
        let outfile = fs::File::create(outpath)?;
        let tmpl = env.get_template("sidebar.html")?;
        tmpl.render_to_write(context! { tree => self, ctx => self.context() }, outfile)?;

        // render the main page
        let outpath = outdir.join("index.html");
        log::trace!(
            "Reading main page contents from {}",
            self.main_page().file().display()
        );
        let page_content = fs::read_to_string(self.main_page().file())?;

        let page_content = match markdown::to_html_with_options(&page_content, &md_opts) {
            Ok(s) => s,
            Err(e) => anyhow::bail!("{}", e),
        };

        log::info!("Rendering main page to {}", outpath.display());
        let outfile = fs::File::create(outpath)?;

        let tmpl = env.get_template("content.html")?;
        tmpl.render_to_write(
                context! { ctx => self.context(), page => self.main_page(), script => SCRIPT_JS, page_content },
                outfile,
            )?;

        // todo make style.css a template as well (allow for themes)
        let outpath = outdir.join("style.css");
        log::info!("Writing style.css to {}", outpath.display());
        let mut outfile = fs::File::create(outpath)?;
        outfile.write_all(STYLE_CSS.as_bytes())?;

        Ok(())
    }

    fn render_sections(&self, outdir: &Path, env: &Environment) -> anyhow::Result<()> {
        let tmpl = env.get_template("sec_index.html")?;

        let sec_count = self.sections().count();
        for (i, s) in self.sections().enumerate() {
            let outpath = {
                let p = Path::new(s.href()).strip_prefix("/").unwrap();
                outdir.join(p).join("index.html")
            };
            let outdir = outpath.parent().unwrap_or_else(|| Path::new(""));
            fs::create_dir_all(outdir)?;

            log::info!(
                "[{}/{}] Rendering section index to {}",
                i + 1,
                sec_count,
                outpath.display()
            );

            let outfile = fs::File::create(outpath)?;

            tmpl.render_to_write(
                context! { ctx => self.context(), sec => s, script => SCRIPT_JS },
                outfile,
            )?;
        }

        Ok(())
    }

    fn render_subsections(&self, outdir: &Path, env: &Environment) -> anyhow::Result<()> {
        let tmpl = env.get_template("sub_index.html")?;

        let sub_count = self.subsections().count();
        for (i, s) in self.subsections().enumerate() {
            let outpath = {
                let p = Path::new(s.href()).strip_prefix("/").unwrap();
                outdir.join(p).join("index.html")
            };
            let outdir = outpath.parent().unwrap_or_else(|| Path::new(""));
            fs::create_dir_all(outdir)?;

            log::info!(
                "[{}/{}] Rendering subsection index to {}",
                i + 1,
                sub_count,
                outpath.display()
            );

            let outfile = fs::File::create(outpath)?;

            tmpl.render_to_write(
                context! { ctx => self.context(), sec => s, script => SCRIPT_JS },
                outfile,
            )?;
        }

        Ok(())
    }

    fn render_pages(&self, outdir: &Path, env: &Environment) -> anyhow::Result<()> {
        let md_opts = Options {
            parse: ParseOptions {
                constructs: Constructs {
                    math_flow: true,
                    math_text: true,
                    ..Constructs::gfm()
                },
                ..ParseOptions::gfm()
            },
            ..Options::gfm()
        };

        let tmpl = env.get_template("content.html")?;

        let page_count = self.pages().count();
        for (i, p) in self.pages().enumerate() {
            // can unwrap because all hrefs start with a slash
            let outpath = Path::new(p.href()).strip_prefix("/").unwrap();
            let outpath = outdir.join(outpath);
            let outdir = outpath.parent().unwrap_or_else(|| Path::new(""));
            fs::create_dir_all(outdir)?;

            log::trace!("[{}/{}] Reading {}", i + 1, page_count, p.file().display());

            let page_content = fs::read_to_string(p.file())?;

            log::info!(
                "[{}/{}] Rendering {} to {}",
                i + 1,
                page_count,
                p.file().display(),
                outpath.display()
            );

            let page_content = match markdown::to_html_with_options(&page_content, &md_opts) {
                Ok(s) => s,
                Err(e) => anyhow::bail!("{}", e),
            };

            let outfile = fs::File::create(outpath)?;

            tmpl.render_to_write(
                context! { ctx => self.context(), page => p, script => SCRIPT_JS, page_content},
                outfile,
            )?;
        }
        Ok(())
    }
}

pub fn is_empty(value: String) -> bool {
    value.is_empty()
}

pub fn is_section(value: String) -> bool {
    value == "Section"
}

pub fn is_page(value: String) -> bool {
    value == "Page"
}

#[cfg(test)]
mod tests {
    use super::*;

    use minijinja::{Environment, context};

    #[test]
    fn sections_iter() {
        let tree = Tree::example_tree();
        let sections: Vec<&str> = tree.sections().map(|s| s.name()).collect();
        let exp = vec!["Projects", "Tutorials"];

        assert_eq!(sections, exp);
    }

    #[test]
    fn subsections_iter() {
        let tree = Tree::structure_into_tree_expect();
        let subsections: Vec<&str> = tree.subsections().map(|s| s.name()).collect();
        let exp = vec!["S1 subsection", "S2 sub", "d1/S1 subsection"];

        assert_eq!(subsections, exp);
    }

    #[test]
    fn pages_iter() {
        let tree = Tree::example_tree();
        let pages: Vec<&str> = tree.pages().map(|p| p.name()).collect();
        let expect = vec![
            "Schedule",
            "Contact Information",
            "Catalyst improvements",
            "wgen - the very \"limited\" website generator",
            "Gentoo + LLVM",
            "Learning maths",
            "Toxic videogames",
            "Choosing your distribution",
            "Why Gentoo is the best?",
            "Online Linux tutorials",
        ];

        assert_eq!(pages, expect);
    }

    #[test]
    fn test_is_empty() -> anyhow::Result<()> {
        let mut env = Environment::new();
        env.add_test("empty", is_empty);
        env.add_template("test1.in", include_str!("./test_templates/test1.in"))?;

        let strs = ["not empty", "next one is", "", "again not", ""];
        let tmpl = env.get_template("test1.in")?;
        let res = tmpl.render(context! {strs})?;
        let exp = "not emptynext one isI was empty.again notI was empty.";

        assert_eq!(res, exp);
        Ok(())
    }

    #[test]
    fn test_is_section() -> anyhow::Result<()> {
        let mut env = Environment::new();
        env.add_test("section", is_section);
        env.add_template("test2.in", include_str!("./test_templates/test2.in"))?;

        let tree = Tree::structure_into_tree_expect();
        let tmpl = env.get_template("test2.in")?;
        let res = tmpl.render(context! {tree})?;
        let exp = r"
D1 section

D2 section
";

        assert_eq!(res, exp);
        Ok(())
    }

    #[test]
    fn test_is_page() -> anyhow::Result<()> {
        let mut env = Environment::new();
        env.add_test("page", is_page);
        env.add_template("test3.in", include_str!("./test_templates/test3.in"))?;

        let tree = Tree::structure_into_tree_expect();
        let tmpl = env.get_template("test3.in")?;
        let res = tmpl.render(context! {tree})?;
        let exp = r"
First page 1.md

Second page in the root
";

        assert_eq!(res, exp);
        Ok(())
    }

    #[test]
    fn test_inherit() -> anyhow::Result<()> {
        let exp = r"[title]
Hello, world!

[body]
This is the body

[footer]
I exist!
";
        let mut env = Environment::new();
        env.add_test("empty", is_empty);
        env.add_template("base.in", include_str!("./test_templates/base.in"))?;
        env.add_template("inherit.in", include_str!("./test_templates/inherit.in"))?;

        let tmpl = env.get_template("inherit.in")?;
        let ctx = context! {
            title_content => "Hello, world!",
            body_content => "This is the body",
            footer_content => "I exist!",
        };
        let res = tmpl.render(ctx)?;

        assert_eq!(res, exp);
        Ok(())
    }
}

// pub fn render_page_test(page: &Page) -> anyhow::Result<String> {
//     let mut env = Environment::new();
//     env.add_template("test_page", include_str!("test_page.in"))?;
//     let tmpl = env.get_template("test_page")?;
//     Ok(tmpl.render(context!(page => page))?)
// }
//
// pub fn render_test() -> anyhow::Result<()> {
//     let section_spec = fs::read_to_string("./spec1/d1/section.toml")?;
//     let section_spec: SectionSpec = toml::from_str(&section_spec)?;
//     let page = section_spec.pages[0].clone();
//     let page = Page::from_spec(page, Path::new("spec1/d1/"), Path::new("spec1/"));
//     let test_str = render_page_test(&page)?;
//     println!("{}", test_str);
//     Ok(())
// }
//
// pub fn render_section_test() -> anyhow::Result<String> {
//     let section = Section::section_read_spec_expected();
//
//     let mut env = Environment::new();
//     env.add_template("entrytest", include_str!("templates/entrytest.in"))?;
//     let tmpl = env.get_template("entrytest")?;
//     Ok(tmpl.render(context!(sec => section))?)
// }
