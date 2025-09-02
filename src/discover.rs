use std::fs;
use std::path::{Path, PathBuf};

use crate::spec::*;
use crate::tree;

#[derive(Debug, PartialEq)]
pub struct Structure {
    pub root: PathBuf,
    sections: Vec<Section>,
}

#[derive(Debug, PartialEq)]
struct Section {
    spec: PathBuf,
    subsections: Vec<Subsection>,
}

#[derive(Debug, PartialEq)]
struct Subsection(PathBuf);

impl Structure {
    pub fn collect_from(root: &Path) -> anyhow::Result<Self> {
        match collect_structure(root) {
            Ok(sections) => Ok(Self {
                root: root.to_path_buf(),
                sections,
            }),
            Err(e) => {
                anyhow::bail!("fetching the tree failed: {}", e)
            }
        }
    }

    pub fn into_tree(self) -> anyhow::Result<tree::Tree> {
        todo!()
    }
}

impl Section {
    fn read_spec(self, root: &Path) -> anyhow::Result<tree::Section> {
        let spec = fs::read_to_string(&self.spec)?;
        let spec: SectionSpec = toml::from_str(&spec)?;

        let subsections: Vec<tree::Subsection> = self
            .subsections
            .into_iter()
            .map(|s| {
                let path = s.0.clone();
                match s.read_spec(root) {
                    Ok(sub) => Ok(sub),
                    Err(e) => {
                        log::warn!("Could not read the spec {}: {}", path.display(), e);
                        Err(())
                    }
                }
            })
            .flatten()
            .collect();
        let location = self.spec.parent().unwrap_or(Path::new(""));
        Ok(tree::Section::from_spec(spec, subsections, location, root))
    }
}

impl Subsection {
    fn read_spec(self, root: &Path) -> anyhow::Result<tree::Subsection> {
        let spec = fs::read_to_string(&self.0)?;
        let spec: SubsectionSpec = toml::from_str(&spec)?;
        let location = self.0.parent().unwrap_or(Path::new(""));
        Ok(tree::Subsection::from_spec(spec, location, root))
    }
}

fn collect_structure(root: &Path) -> anyhow::Result<Vec<Section>> {
    log::info!("Collecting entries from root at {}", root.display());

    let spec = root.join("manifest.toml");
    if spec.try_exists()? && spec.is_file() {
        let mut sections = Vec::new();
        if let Ok(read_dir) = root.read_dir() {
            for entry in read_dir {
                match entry {
                    Ok(entry) => {
                        let path = entry.path();
                        if path.is_dir() {
                            match collect_dir(&path) {
                                Ok(sec) => sections.push(sec),
                                Err(e) => log::warn!("Skipping {}: {}", path.display(), e),
                            }
                        }
                    }
                    Err(e) => log::warn!("Skipping a file in {}: {}", root.display(), e),
                }
            }
        }
        Ok(sections)
    } else {
        anyhow::bail!("no manifest.toml found in the root, aborting...");
    }
}

fn collect_dir(root: &Path) -> anyhow::Result<Section> {
    log::info!("Entering directory {}", root.display());

    let spec = root.join("section.toml");
    if spec.try_exists()? && spec.is_file() {
        let mut subsections = Vec::new();
        if let Ok(read_dir) = root.read_dir() {
            for entry in read_dir {
                match entry {
                    Ok(entry) => {
                        let path = entry.path();
                        if path.is_dir() {
                            match collect_subdir(&path) {
                                Ok(sub) => subsections.push(sub),
                                Err(e) => log::warn!("Skipping {}: {}", path.display(), e),
                            }
                        }
                    }
                    Err(e) => log::warn!("Skipping a file in {}: {}", root.display(), e),
                }
            }
        }
        Ok(Section { spec, subsections })
    } else {
        anyhow::bail!("section.toml not found or is not a file")
    }
}

fn collect_subdir(root: &Path) -> anyhow::Result<Subsection> {
    log::info!("Entering subdirectory {}", root.display());

    let spec = root.join("subsection.toml");
    if spec.try_exists()? && spec.is_file() {
        Ok(Subsection(spec))
    } else {
        anyhow::bail!("subsection.toml not found or is not a file")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::{Path, PathBuf};

    #[test]
    fn collect_spec1() -> anyhow::Result<()> {
        let structure = Structure::collect_from(std::path::Path::new("spec1"))?;
        let expect = Structure {
            root: PathBuf::from("spec1"),
            sections: vec![
                Section {
                    spec: PathBuf::from("spec1/d1/section.toml"),
                    subsections: vec![Subsection(PathBuf::from("spec1/d1/s1/subsection.toml"))],
                },
                Section {
                    spec: PathBuf::from("spec1/d2/section.toml"),
                    subsections: vec![],
                },
            ],
        };

        assert_eq!(structure, expect);
        Ok(())
    }

    #[test]
    fn collect_spec2() -> anyhow::Result<()> {
        let structure = Structure::collect_from(std::path::Path::new("spec2"))?;
        let expect = Structure {
            root: PathBuf::from("spec2"),
            sections: vec![
                Section {
                    spec: PathBuf::from("spec2/d1/section.toml"),
                    subsections: vec![
                        Subsection(PathBuf::from("spec2/d1/s1/subsection.toml")),
                        Subsection(PathBuf::from("spec2/d1/s2/subsection.toml")),
                    ],
                },
                Section {
                    spec: PathBuf::from("spec2/d2/section.toml"),
                    subsections: vec![Subsection(PathBuf::from("spec2/d2/s1/subsection.toml"))],
                },
            ],
        };

        assert_eq!(structure, expect);
        Ok(())
    }

    #[test]
    fn subsection_read_spec1() -> anyhow::Result<()> {
        let root = Path::new("spec1");
        let subsection = Subsection(PathBuf::from("spec1/d1/s1/subsection.toml"));
        let subsection = subsection.read_spec(root)?;
        let expect = tree::Subsection::subsection_read_spec1_expected();

        assert_eq!(subsection, expect);
        Ok(())
    }

    #[test]
    fn subsection_read_spec2() -> anyhow::Result<()> {
        let root = Path::new("spec1");
        let subsection = Subsection(PathBuf::from("spec2/d2/s1/subsection.toml"));
        let subsection = subsection.read_spec(root)?;
        let expect = tree::Subsection::subsection_read_spec2_expected();

        assert_eq!(subsection, expect);

        Ok(())
    }
}
