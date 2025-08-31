use std::fs;
use std::path::{Path, PathBuf};

use chrono::{DateTime, Utc};

use crate::spec;

#[derive(Debug, PartialEq)]
pub struct Page {
    name: String,
    desc: String,
    path: PathBuf,
    href: String,
    date: String,
}

impl Page {
    pub fn new(spec: spec::PageSpec, location: &Path, root: &Path) -> Self {
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

#[cfg(test)]
mod tests {
    use super::*;

    use crate::spec;
    use std::path::{Path, PathBuf};

    #[test]
    fn page_new() {
        let spec = spec::PageSpec {
            name: "Imaginary Name".to_string(),
            desc: "Hello!".to_string(),
            path: "imaginary_file.md".to_string(),
        };

        let page = Page::new(spec.clone(), Path::new("in/s1/"), Path::new("in/"));

        let exp = Page {
            name: "Imaginary Name".to_string(),
            desc: "Hello!".to_string(),
            path: PathBuf::from("in/s1/imaginary_file.md"),
            href: "/s1/imaginary_file.md".to_string(),
            date: "Unknown".to_string(),
        };

        assert_eq!(page, exp);

        let page = Page::new(spec.clone(), Path::new(""), Path::new(""));

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
