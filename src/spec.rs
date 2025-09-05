// Copyright 2025 Szymon Sztuka (lysolaka)
// Distributed under the terms of the GNU General Public License v3 or later

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct PageSpec {
    pub name: String,
    #[serde(default)]
    pub desc: String,
    pub path: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct ManifestSpec {
    pub title: String,
    #[serde(default)]
    pub append_title: bool,
    pub main_page: String,
    #[serde(default)]
    pub footer_content: String,
    #[serde(default)]
    pub href_prepend: String,
    #[serde(rename = "page")]
    pub pages: Vec<PageSpec>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct SectionSpec {
    pub section: SectionField,
    #[serde(rename = "page")]
    pub pages: Vec<PageSpec>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct SubsectionSpec {
    pub subsection: SectionField,
    #[serde(rename = "page")]
    pub pages: Vec<PageSpec>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct SectionField {
    pub name: String,
    #[serde(default)]
    pub desc: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;
    use toml;

    #[test]
    fn read_manifest() -> anyhow::Result<()> {
        let manifest = fs::read_to_string("spec1/manifest.toml")?;
        let manifest: ManifestSpec = toml::from_str(&manifest)?;

        let spec = ManifestSpec {
            title: "WGEN Webpage".to_string(),
            append_title: false,
            main_page: "index.md".to_string(),
            footer_content: "&copy; lysolaka. Contact me: +69 420 024 420".to_string(),
            href_prepend: "/~home".to_string(),
            pages: vec![
                PageSpec {
                    name: "First page 1.md".to_string(),
                    desc: "Generic description".to_string(),
                    path: "1.md".to_string(),
                },
                PageSpec {
                    name: "Second page in the root".to_string(),
                    desc: "".to_string(),
                    path: "2.md".to_string(),
                },
            ],
        };

        assert_eq!(manifest, spec);
        Ok(())
    }

    #[test]
    fn read_section() -> anyhow::Result<()> {
        let section = fs::read_to_string("spec1/d1/section.toml")?;
        let section: SectionSpec = toml::from_str(&section)?;

        let spec = SectionSpec {
            section: SectionField { name: "D1 section".to_string(), desc: "Could be no description".to_string() },
            pages: vec![
                PageSpec {
                    name: "D1-1".to_string(),
                    desc: "there will be more ones".to_string(),
                    path: "1.md".to_string(),
                },
                PageSpec {
                    name: "Second page in the d1 section".to_string(),
                    desc: "".to_string(),
                    path: "2.md".to_string(),
                },
            ],
        };

        assert_eq!(section, spec);
        Ok(())
    }

    #[test]
    fn read_subsection() -> anyhow::Result<()> {
        let subsection = fs::read_to_string("spec1/d1/s1/subsection.toml")?;
        let subsection: SubsectionSpec = toml::from_str(&subsection)?;

        let spec = SubsectionSpec {
            subsection: SectionField { name: "subsection s1".to_string(), desc: "".to_string() },
            pages: vec![
                PageSpec {
                    name: "D1-S1-1".to_string(),
                    desc: "this is the most 1's".to_string(),
                    path: "1.md".to_string(),
                },
                PageSpec {
                    name: "Second page in the d1/s1 subsection".to_string(),
                    desc: "".to_string(),
                    path: "2.md".to_string(),
                },
            ],
        };

        assert_eq!(subsection, spec);
        Ok(())
    }
}
