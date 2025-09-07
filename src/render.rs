// Copyright 2025 Szymon Sztuka (lysolaka)
// Distributed under the terms of the GNU General Public License v3 or later

use minijinja::{Environment, context};

use crate::tree::*;

impl Tree {
    pub fn pages(&self) -> impl Iterator<Item = &Page> {
        let root_pages = self.iter().filter_map(|e| match e {
            TreeEntry::Page(page) => Some(page),
            _ => None,
        });

        let sec_pages = self
            .iter()
            .filter_map(|e| match e {
                TreeEntry::Section(section) => Some(section.iter()),
                _ => None,
            })
            .flatten()
            .filter_map(|e| match e {
                SectionEntry::Page(page) => Some(page),
                _ => None,
            });

        let sub_pages = self
            .iter()
            .filter_map(|e| match e {
                TreeEntry::Section(section) => Some(section.iter()),
                _ => None,
            })
            .flatten()
            .filter_map(|e| match e {
                SectionEntry::Subsection(subsection) => Some(subsection.iter()),
                _ => None,
            })
            .flatten();

        root_pages.chain(sec_pages).chain(sub_pages)
    }
}

fn is_empty(value: String) -> bool {
    value.is_empty()
}

fn is_section(value: String) -> bool {
    value == "Section"
}

fn is_page(value: String) -> bool {
    value == "Page"
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree::Tree;

    use minijinja::{Environment, context};

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
