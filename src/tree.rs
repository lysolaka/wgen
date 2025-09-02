use std::fs;
use std::path::{Path, PathBuf};

use chrono::{DateTime, Utc};

use crate::spec::*;

#[derive(Debug, PartialEq)]
pub struct Page {
    name: String,
    desc: String,
    path: PathBuf,
    href: String,
    date: String,
}

impl Page {
    pub fn from_spec(spec: PageSpec, location: &Path, root: &Path) -> Self {
        let path = location.join(&spec.path);
        let href = if let Ok(p) = path.strip_prefix(root) {
            format!("/{}", p.display())
        } else {
            format!("/{}", path.display())
        };

        let date = match fs::metadata(&path) {
            Ok(metadata) => match metadata.modified() {
                Ok(mtime) => {
                    let mtime: DateTime<Utc> = mtime.into();
                    mtime.format("%-d.%m.%Y %H:%M").to_string()
                }
                Err(_) => "Unknown".to_string(),
            },
            Err(_) => "Unknown".to_string(),
        };

        Self {
            name: spec.name,
            desc: spec.desc,
            path,
            href,
            date,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Subsection {
    name: String,
    desc: String,
    path: PathBuf,
    href: String,
    pages: Vec<Page>,
}

impl Subsection {
    pub fn from_spec(spec: SubsectionSpec, location: &Path, root: &Path) -> Self {
        let href = if let Ok(p) = location.strip_prefix(root) {
            format!("/{}/", p.display())
        } else {
            format!("/{}/", location.display())
        };

        let pages: Vec<Page> = spec
            .pages
            .into_iter()
            .map(|p| Page::from_spec(p, location, root))
            .collect();

        Self {
            name: spec.subsection.name,
            desc: spec.subsection.desc,
            path: location.to_path_buf(),
            href,
            pages,
        }
    }

    #[cfg(test)]
    pub fn subsection_read_spec1_expected() -> Self {
        Self {
            name: "subsection s1".to_string(),
            desc: "".to_string(),
            path: PathBuf::from("spec1/d1/s1"),
            href: "/d1/s1/".to_string(),
            pages: vec![
                Page {
                    name: "D1-S1-1".to_string(),
                    desc: "this is the most 1's".to_string(),
                    path: PathBuf::from("spec1/d1/s1/1.md"),
                    href: "/d1/s1/1.md".to_string(),
                    date: "1.09.2025 20:34".to_string(),
                },
                Page {
                    name: "Second page in the d1/s1 subsection".to_string(),
                    desc: "".to_string(),
                    path: PathBuf::from("spec1/d1/s1/2.md"),
                    href: "/d1/s1/2.md".to_string(),
                    date: "1.09.2025 20:34".to_string(),
                },
            ],
        }
    }

    #[cfg(test)]
    pub fn subsection_read_spec2_expected() -> Self {
        Self {
            name: "d1/S1 subsection".to_string(),
            desc: "SZAJSE".to_string(),
            path: PathBuf::from("spec2/d2/s1"),
            href: "/spec2/d2/s1/".to_string(),
            pages: vec![Page {
                name: "1 MD".to_string(),
                desc: "The first and only page here".to_string(),
                path: PathBuf::from("spec2/d2/s1/1.md"),
                href: "/spec2/d2/s1/1.md".to_string(),
                date: "1.09.2025 20:34".to_string(),
            }],
        }
    }
}

#[derive(Debug, PartialEq)]
enum SectionEntry {
    Page(Page),
    Subsection(Subsection),
}

#[derive(Debug, PartialEq)]
pub struct Section {
    name: String,
    desc: String,
    path: PathBuf,
    href: String,
    entries: Vec<SectionEntry>,
}

impl Section {
    pub fn from_spec(
        spec: SectionSpec,
        subsections: Vec<Subsection>,
        location: &Path,
        root: &Path,
    ) -> Self {
        let href = if let Ok(p) = location.strip_prefix(root) {
            format!("/{}/", p.display())
        } else {
            format!("/{}/", location.display())
        };

        let pages = spec
            .pages
            .into_iter()
            .map(|p| SectionEntry::Page(Page::from_spec(p, location, root)));

        let entries: Vec<SectionEntry> = subsections
            .into_iter()
            .map(|s| SectionEntry::Subsection(s))
            .chain(pages)
            .collect();

        Self {
            name: spec.section.name,
            desc: spec.section.desc,
            path: location.to_path_buf(),
            href,
            entries,
        }
    }
}

enum TreeEntry {
    Page(Page),
    Section(Section),
}

pub struct Tree {
    root: PathBuf,
    title: String,
    append_title: bool,
    href_prepend: String,
    entries: Vec<TreeEntry>,
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::spec;
    use std::path::{Path, PathBuf};

    #[test]
    fn page_from_spec() {
        let spec = spec::PageSpec {
            name: "Imaginary Name".to_string(),
            desc: "Hello!".to_string(),
            path: "imaginary_file.md".to_string(),
        };

        let page = Page::from_spec(spec.clone(), Path::new("in/s1/"), Path::new("in/"));

        let exp = Page {
            name: "Imaginary Name".to_string(),
            desc: "Hello!".to_string(),
            path: PathBuf::from("in/s1/imaginary_file.md"),
            href: "/s1/imaginary_file.md".to_string(),
            date: "Unknown".to_string(),
        };

        assert_eq!(page, exp);

        let page = Page::from_spec(spec.clone(), Path::new(""), Path::new(""));

        let exp = Page {
            name: "Imaginary Name".to_string(),
            desc: "Hello!".to_string(),
            path: PathBuf::from("imaginary_file.md"),
            href: "/imaginary_file.md".to_string(),
            date: "Unknown".to_string(),
        };

        assert_eq!(page, exp);
    }
}
