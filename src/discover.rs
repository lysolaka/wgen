// Copyright 2025 Szymon Sztuka (lysolaka)
// Distributed under the terms of the GNU General Public License v3 or later

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
        let spec = self.root.join("manifest.toml");
        log::info!("Reading {}", spec.display());
        let spec = fs::read_to_string(spec)?;
        let spec: ManifestSpec = toml::from_str(&spec)?;

        let sections = self
            .sections
            .into_iter()
            .flat_map(|s| {
                let path = s.spec.clone();
                match s.read_spec(&self.root) {
                    Ok(sub) => Ok(sub),
                    Err(e) => {
                        log::warn!("Could not read the spec {}: {}", path.display(), e);
                        Err(())
                    }
                }
            });
        
        Ok(tree::Tree::from_spec(spec, self.root.clone(), sections))
    }
}

impl Section {
    fn read_spec(self, root: &Path) -> anyhow::Result<tree::Section> {
        log::info!("Reading section specfile {}", self.spec.display());
        let spec = fs::read_to_string(&self.spec)?;
        let spec: SectionSpec = toml::from_str(&spec)?;

        let subsections = self
            .subsections
            .into_iter()
            .flat_map(|s| {
                let path = s.0.clone();
                match s.read_spec(root) {
                    Ok(sub) => Ok(sub),
                    Err(e) => {
                        log::warn!("Could not read the spec {}: {}", path.display(), e);
                        Err(())
                    }
                }
            });

        let location = self.spec.parent().unwrap_or(Path::new(""));
        Ok(tree::Section::from_spec(spec, subsections, location, root))
    }
}

impl Subsection {
    fn read_spec(self, root: &Path) -> anyhow::Result<tree::Subsection> {
        log::info!("Reading subsection specfile {}", self.0.display());
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
        let subsection = Subsection(PathBuf::from("spec1/d1/s1/subsection.toml"));
        let subsection = subsection.read_spec(Path::new("spec1"))?;
        let expect = tree::Subsection::subsection_read_spec1_expected();

        assert_eq!(subsection, expect);
        Ok(())
    }

    #[test]
    fn subsection_read_spec2() -> anyhow::Result<()> {
        let subsection = Subsection(PathBuf::from("spec2/d2/s1/subsection.toml"));
        let subsection = subsection.read_spec(Path::new("spec2"))?;
        let expect = tree::Subsection::subsection_read_spec2_expected();

        assert_eq!(subsection, expect);
        Ok(())
    }

    #[test]
    fn section_read_spec() -> anyhow::Result<()> {
        let section = Section {
            spec: PathBuf::from("spec2/d1/section.toml"),
            subsections: vec![
                Subsection(PathBuf::from("spec2/d1/s1/subsection.toml")),
                Subsection(PathBuf::from("spec2/d1/s2/subsection.toml")),
            ],
        };
        let section = section.read_spec(Path::new("spec2"))?;
        let expect = tree::Section::section_read_spec_expected();

        assert_eq!(section, expect);
        Ok(())
    }

    #[test]
    fn structure_into_tree() -> anyhow::Result<()> {
        let structure = Structure {
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

        let tree = structure.into_tree()?;
        let expect = tree::Tree::structure_into_tree_expect();

        assert_eq!(tree, expect);
        Ok(())
    }
}
