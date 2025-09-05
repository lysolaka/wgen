// Copyright 2025 Szymon Sztuka (lysolaka)
// Distributed under the terms of the GNU General Public License v3 or later

use std::fs;
use std::path::{Path, PathBuf};

use chrono::{DateTime, Utc};

use crate::spec::*;

#[derive(Debug, serde::Serialize)]
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
            format!("/{}", p.with_extension("html").display())
        } else {
            format!("/{}", path.with_extension("html").display())
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

    pub fn new_main(path: &str, title: &str, root: &Path) -> Self {
        let path = root.join(path);

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
            name: title.to_string(),
            desc: "".to_string(),
            path,
            href: "/".to_string(),
            date,
        }
    }
}

impl PartialEq for Page {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.desc == other.desc
            && self.path == other.path
            && self.href == other.href
    }
}

#[derive(Debug, PartialEq, serde::Serialize)]
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

        log::trace!("Found {} pages in {}", pages.len(), location.display());

        Self {
            name: spec.subsection.name,
            desc: spec.subsection.desc,
            path: location.to_path_buf(),
            href,
            pages,
        }
    }

    pub fn iter(&self) -> std::slice::Iter<Page> {
        self.pages.iter()
    }
}

#[derive(Debug, PartialEq, serde::Serialize)]
#[serde(tag = "type")]
pub enum SectionEntry {
    Page(Page),
    Subsection(Subsection),
}

#[derive(Debug, PartialEq, serde::Serialize)]
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
        subsections: impl Iterator<Item = Subsection>,
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

        log::trace!("Found {} pages in {}", pages.len(), location.display());

        let entries: Vec<SectionEntry> = subsections
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

    pub fn iter(&self) -> std::slice::Iter<SectionEntry> {
        self.entries.iter()
    }
}

#[derive(Debug, PartialEq, serde::Serialize)]
#[serde(tag = "type")]
pub enum TreeEntry {
    Page(Page),
    Section(Section),
}

#[derive(Debug, PartialEq)]
pub struct Tree {
    root: PathBuf,
    title: String,
    append_title: bool,
    href_prepend: String,
    footer_content: String,
    main_page: Page,
    entries: Vec<TreeEntry>,
}

impl Tree {
    pub fn from_spec(
        spec: ManifestSpec,
        root: PathBuf,
        sections: impl Iterator<Item = Section>,
    ) -> Self {
        let pages = spec
            .pages
            .into_iter()
            .map(|p| TreeEntry::Page(Page::from_spec(p, &root, &root)));

        log::trace!("Found {} pages in {}", pages.len(), root.display());

        let main_page = Page::new_main(&spec.main_page, &spec.title, &root);

        let entries: Vec<TreeEntry> = sections
            .map(|s| TreeEntry::Section(s))
            .chain(pages)
            .collect();

        Self {
            root,
            title: spec.title,
            append_title: spec.append_title,
            href_prepend: spec.href_prepend,
            footer_content: spec.footer_content,
            main_page,
            entries,
        }
    }

    pub fn context(&self) -> Context<'_> {
        Context {
            title: &self.title,
            append_title: self.append_title,
            href_prepend: &self.href_prepend,
            footer_content: &self.footer_content,
        }
    }

    pub fn iter(&self) -> std::slice::Iter<TreeEntry> {
        self.entries.iter()
    }
}

pub struct Context<'a> {
    title: &'a str,
    append_title: bool,
    href_prepend: &'a str,
    footer_content: &'a str,
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
            href: "/s1/imaginary_file.html".to_string(),
            date: "Unknown".to_string(),
        };

        assert_eq!(page, exp);

        let page = Page::from_spec(spec.clone(), Path::new(""), Path::new(""));

        let exp = Page {
            name: "Imaginary Name".to_string(),
            desc: "Hello!".to_string(),
            path: PathBuf::from("imaginary_file.md"),
            href: "/imaginary_file.html".to_string(),
            date: "Unknown".to_string(),
        };

        assert_eq!(page, exp);
    }
}

#[cfg(test)]
impl Subsection {
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
                    href: "/d1/s1/1.html".to_string(),
                    date: "1.09.2025 20:34".to_string(),
                },
                Page {
                    name: "Second page in the d1/s1 subsection".to_string(),
                    desc: "".to_string(),
                    path: PathBuf::from("spec1/d1/s1/2.md"),
                    href: "/d1/s1/2.html".to_string(),
                    date: "1.09.2025 20:34".to_string(),
                },
            ],
        }
    }

    pub fn subsection_read_spec2_expected() -> Self {
        Self {
            name: "d1/S1 subsection".to_string(),
            desc: "SZAJSE".to_string(),
            path: PathBuf::from("spec2/d2/s1"),
            href: "/d2/s1/".to_string(),
            pages: vec![Page {
                name: "1 MD".to_string(),
                desc: "The first and only page here".to_string(),
                path: PathBuf::from("spec2/d2/s1/1.md"),
                href: "/d2/s1/1.html".to_string(),
                date: "1.09.2025 20:34".to_string(),
            }],
        }
    }
}

#[cfg(test)]
impl Section {
    pub fn section_read_spec_expected() -> Self {
        Self {
            name: "D1 section".to_string(),
            desc: "Shit section".to_string(),
            path: PathBuf::from("spec2/d1"),
            href: "/d1/".to_string(),
            entries: vec![
                SectionEntry::Subsection(Subsection {
                    name: "S1 subsection".to_string(),
                    desc: "Shit shit shit".to_string(),
                    path: PathBuf::from("spec2/d1/s1"),
                    href: "/d1/s1/".to_string(),
                    pages: vec![
                        Page {
                            name: "1 MD".to_string(),
                            desc: "The first page here".to_string(),
                            path: PathBuf::from("spec2/d1/s1/1.md"),
                            href: "/d1/s1/1.html".to_string(),
                            date: "1.09.2025 20:34".to_string(),
                        },
                        Page {
                            name: "2nd md".to_string(),
                            desc: "".to_string(),
                            path: PathBuf::from("spec2/d1/s1/2.md"),
                            href: "/d1/s1/2.html".to_string(),
                            date: "1.09.2025 20:34".to_string(),
                        },
                    ],
                }),
                SectionEntry::Subsection(Subsection {
                    name: "S2 sub".to_string(),
                    desc: "Shit".to_string(),
                    path: PathBuf::from("spec2/d1/s2"),
                    href: "/d1/s2/".to_string(),
                    pages: vec![
                        Page {
                            name: "1 EMDE".to_string(),
                            desc: "A page".to_string(),
                            path: PathBuf::from("spec2/d1/s2/1.md"),
                            href: "/d1/s2/1.html".to_string(),
                            date: "1.09.2025 20:34".to_string(),
                        },
                        Page {
                            name: "second md".to_string(),
                            desc: "".to_string(),
                            path: PathBuf::from("spec2/d1/s2/2.md"),
                            href: "/d1/s2/2.html".to_string(),
                            date: "1.09.2025 20:34".to_string(),
                        },
                    ],
                }),
                SectionEntry::Page(Page {
                    name: "one MD".to_string(),
                    desc: "The first page here".to_string(),
                    path: PathBuf::from("spec2/d1/1.md"),
                    href: "/d1/1.html".to_string(),
                    date: "1.09.2025 20:34".to_string(),
                }),
                SectionEntry::Page(Page {
                    name: "2nd markdown".to_string(),
                    desc: "".to_string(),
                    path: PathBuf::from("spec2/d1/2.md"),
                    href: "/d1/2.html".to_string(),
                    date: "1.09.2025 20:34".to_string(),
                }),
            ],
        }
    }
}

#[cfg(test)]
impl Tree {
    pub fn structure_into_tree_expect() -> Self {
        Self {
            root: PathBuf::from("spec2"),
            title: "WGEN Webpage".to_string(),
            append_title: false,
            href_prepend: "/~home".to_string(),
            footer_content: "&copy; lysolaka. Contact me: +69 420 024 420".to_string(),
            main_page: Page {
                name: "WGEN Webpage".to_string(),
                desc: "".to_string(),
                path: PathBuf::from("spec2/index.md"),
                href: "/".to_string(),
                date: "3.09.2025 16:14".to_string(),
            },
            entries: vec![
                TreeEntry::Section(Section {
                    name: "D1 section".to_string(),
                    desc: "Shit section".to_string(),
                    path: PathBuf::from("spec2/d1"),
                    href: "/d1/".to_string(),
                    entries: vec![
                        SectionEntry::Subsection(Subsection {
                            name: "S1 subsection".to_string(),
                            desc: "Shit shit shit".to_string(),
                            path: PathBuf::from("spec2/d1/s1"),
                            href: "/d1/s1/".to_string(),
                            pages: vec![
                                Page {
                                    name: "1 MD".to_string(),
                                    desc: "The first page here".to_string(),
                                    path: PathBuf::from("spec2/d1/s1/1.md"),
                                    href: "/d1/s1/1.html".to_string(),
                                    date: "29.08.2025 08:45".to_string(),
                                },
                                Page {
                                    name: "2nd md".to_string(),
                                    desc: "".to_string(),
                                    path: PathBuf::from("spec2/d1/s1/2.md"),
                                    href: "/d1/s1/2.html".to_string(),
                                    date: "29.08.2025 08:45".to_string(),
                                },
                            ],
                        }),
                        SectionEntry::Subsection(Subsection {
                            name: "S2 sub".to_string(),
                            desc: "Shit".to_string(),
                            path: PathBuf::from("spec2/d1/s2"),
                            href: "/d1/s2/".to_string(),
                            pages: vec![
                                Page {
                                    name: "1 EMDE".to_string(),
                                    desc: "A page".to_string(),
                                    path: PathBuf::from("spec2/d1/s2/1.md"),
                                    href: "/d1/s2/1.html".to_string(),
                                    date: "29.08.2025 08:45".to_string(),
                                },
                                Page {
                                    name: "second md".to_string(),
                                    desc: "".to_string(),
                                    path: PathBuf::from("spec2/d1/s2/2.md"),
                                    href: "/d1/s2/2.html".to_string(),
                                    date: "29.08.2025 08:45".to_string(),
                                },
                            ],
                        }),
                        SectionEntry::Page(Page {
                            name: "one MD".to_string(),
                            desc: "The first page here".to_string(),
                            path: PathBuf::from("spec2/d1/1.md"),
                            href: "/d1/1.html".to_string(),
                            date: "29.08.2025 08:45".to_string(),
                        }),
                        SectionEntry::Page(Page {
                            name: "2nd markdown".to_string(),
                            desc: "".to_string(),
                            path: PathBuf::from("spec2/d1/2.md"),
                            href: "/d1/2.html".to_string(),
                            date: "29.08.2025 08:45".to_string(),
                        }),
                    ],
                }),
                TreeEntry::Section(Section {
                    name: "D2 section".to_string(),
                    desc: "Shittier section".to_string(),
                    path: PathBuf::from("spec2/d2"),
                    href: "/d2/".to_string(),
                    entries: vec![
                        SectionEntry::Subsection(Subsection {
                            name: "d1/S1 subsection".to_string(),
                            desc: "SZAJSE".to_string(),
                            path: PathBuf::from("spec2/d2/s1"),
                            href: "/d2/s1/".to_string(),
                            pages: vec![Page {
                                name: "1 MD".to_string(),
                                desc: "The first and only page here".to_string(),
                                path: PathBuf::from("spec2/d2/s1/1.md"),
                                href: "/d2/s1/1.html".to_string(),
                                date: "29.08.2025 08:45".to_string(),
                            }],
                        }),
                        SectionEntry::Page(Page {
                            name: "one MD".to_string(),
                            desc: "The first page here".to_string(),
                            path: PathBuf::from("spec2/d2/1.md"),
                            href: "/d2/1.html".to_string(),
                            date: "29.08.2025 08:45".to_string(),
                        }),
                        SectionEntry::Page(Page {
                            name: "2nd markdown".to_string(),
                            desc: "".to_string(),
                            path: PathBuf::from("spec2/d2/2.md"),
                            href: "/d2/2.html".to_string(),
                            date: "29.08.2025 08:45".to_string(),
                        }),
                    ],
                }),
                TreeEntry::Page(Page {
                    name: "First page 1.md".to_string(),
                    desc: "Generic description".to_string(),
                    path: PathBuf::from("spec2/1.md"),
                    href: "/1.html".to_string(),
                    date: "29.08.2025 08:45".to_string(),
                }),
                TreeEntry::Page(Page {
                    name: "Second page in the root".to_string(),
                    desc: "".to_string(),
                    path: PathBuf::from("spec2/2.md"),
                    href: "/2.html".to_string(),
                    date: "29.08.2025 08:45".to_string(),
                }),
            ],
        }
    }
}
