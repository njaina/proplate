use std::fs::read_dir;
use std::path::{Path, PathBuf};

use super::Template;
use super::error::Error;

const BUILT_IN_TEMPLATE_DIR: &str = "built_in";

pub fn find_template_by_id<'a>(id: &'a str) -> Result<Template<'a>, Error> {
  let path = get_template_path_by_id(id);

  if !path.exists() {
    return Err(Error::not_found(id.to_string()));
  }

  match read_dir(&path) {
    Ok(entries) => {
      let file_list = entries.into_iter().filter_map(|e| {
        match e {
          Ok(entry) => Some(entry.file_name()),
          _ => None
        }
      }).collect::<Vec<_>>();
      Ok(Template::build(id, path, file_list))
    },
    _ => Err(Error::not_found(id.to_string()))
  }
}

fn get_template_path_by_id(id: &str) -> PathBuf {
  Path::new(BUILT_IN_TEMPLATE_DIR)
    .join(Path::new(id))
}