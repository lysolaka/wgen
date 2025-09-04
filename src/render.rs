// Copyright 2025 Szymon Sztuka (lysolaka)
// Distributed under the terms of the GNU General Public License v3 or later

use minijinja::{Environment, context};

fn is_empty(value: String) -> bool {
    value.is_empty()
}

fn is_section(value: String) -> bool {
    value == "Section"
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
