use std::collections::HashMap;
use std::path::{Path, PathBuf};

use anyhow::Result;
use minijinja::value::Value;

use crate::{builtin_templates, compute_template, define_name_and_license, BuildTemplate};

static YARN_TEMPLATES: &[(&str, &str)] = &builtin_templates!["yarn" =>
    ("md.README", "README.md"),
    ("ci.gitlab", ".gitlab-ci.yml"),
    ("ci.github", "github.yml")
];

/// A yarn project data.
pub struct Yarn;

impl Yarn {
    /// Creates a new CI for a yarn project.
    pub fn create_ci(project_path: &Path, license: &str) -> Result<()> {
        let (project_name, license) = define_name_and_license(project_path, license)?;
        let template = Yarn.build(project_path, project_name, license.id());
        compute_template(template, license)
    }

    fn project_structure(
        project_path: &Path,
        name: &str,
    ) -> (HashMap<PathBuf, &'static str>, Vec<PathBuf>) {
        let root = project_path.to_path_buf();
        let github = project_path.join(".github/workflows");

        let mut template_files = HashMap::new();

        // README
        template_files.insert(root.join("README.md"), "md.README");

        // Continuous Integration
        template_files.insert(root.join(".gitlab-ci.yml"), "ci.gitlab");
        template_files.insert(github.join(format!("{}.yml", name)), "ci.github");

        (template_files, vec![root, github])
    }
}

impl BuildTemplate for Yarn {
    fn define(
        &self,
        project_path: &Path,
        project_name: &str,
        license: &str,
    ) -> (
        HashMap<PathBuf, &'static str>,
        Vec<PathBuf>,
        HashMap<&'static str, Value>,
    ) {
        let mut context = HashMap::new();

        context.insert("name", Value::from_serializable(&project_name));
        context.insert("license_id", Value::from_serializable(&license));

        let (files, dirs) = Yarn::project_structure(project_path, project_name);

        (files, dirs, context)
    }

    fn get_templates() -> &'static [(&'static str, &'static str)] {
        YARN_TEMPLATES
    }
}
