use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct PageSpec {
    pub name: String,
    #[serde(default)]
    pub desc: String,
    pub path: String,
}

impl PageSpec {
    pub fn path(&self) -> &str {
        &self.path
    }
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct ManifestSpec {
    title: String,
    #[serde(default)]
    append_title: bool,
    main_page: String,
    #[serde(default)]
    href_prepend: String,
    #[serde(rename = "page")]
    pages: Vec<PageSpec>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct SectionSpec {
    section: SectionField,
    #[serde(rename = "page")]
    pages: Vec<PageSpec>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct SubectionSpec {
    subsection: SectionField,
    #[serde(rename = "page")]
    pages: Vec<PageSpec>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct SectionField {
    name: String,
    #[serde(default)]
    desc: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;
    use toml;

    #[test]
    fn read_manifest() -> anyhow::Result<()> {
        let manifest = fs::read_to_string("spec/manifest.toml")?;
        let manifest: ManifestSpec = toml::from_str(&manifest)?;

        let spec = ManifestSpec {
            title: "WGEN Webpage".to_string(),
            append_title: false,
            main_page: "index.md".to_string(),
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
        let section = fs::read_to_string("spec/d1/section.toml")?;
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
        let subsection = fs::read_to_string("spec/d1/s1/subsection.toml")?;
        let subsection: SubectionSpec = toml::from_str(&subsection)?;

        let spec = SubectionSpec {
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
