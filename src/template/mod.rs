use std::{ffi::OsString, fmt::Display, path::PathBuf};

use crate::colors::error;

pub mod error;
pub mod find;
pub mod meta;

#[derive(Debug)]
pub struct Template {
    pub id: String,
    pub base_path: PathBuf,
    pub base_file_list: Vec<OsString>,
    pub fork_source: Option<String>,
}

pub const META_CONF: &str = "meta.json";

impl Display for Template {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "TEMPLATE [{}], base_path: {:?}",
            self.id, self.base_path
        ))
    }
}

impl Template {
    pub fn build(
        id: String,
        base_path: PathBuf,
        base_file_list: Vec<OsString>,
        fork_source: Option<String>,
    ) -> Template {
        let new = Self {
            id,
            base_path,
            base_file_list,
            fork_source,
        };
        Template::validate(&new);
        new
    }

    pub fn validate(template: &Template) {
        let filelist = template.base_file_list.clone();
        let mut violations = Vec::<String>::new();

        if !filelist.contains(&OsString::from(META_CONF)) {
            violations.push(String::from("No `meta_json` conf file"));
        }

        if !violations.is_empty() {
            panic!("Error\n{}", error(&violations.join("\n")))
        }
    }
}
