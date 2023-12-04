use std::env::current_dir;
use std::fs::read_dir;
use std::io::Error;
use std::path::{Path, PathBuf};

use uuid::Uuid;

use proplate_errors::{ProplateError, ProplateResult};
use proplate_integration::git;

use super::types::Template;

pub fn find_template(location: &str) -> ProplateResult<Template> {
    match is_remote_loc(location) {
        true => clone_git_template(location),
        false => get_local_template(location),
    }
}

fn get_local_template(dir: &str) -> ProplateResult<Template> {
    let path = Path::new(dir);
    if !path.exists() {
        return Err(ProplateError::local_template_not_found(dir));
    }
    explore_meta(path.try_into().unwrap(), &dir, None)
        .map_err(|_| ProplateError::local_template_not_found(dir))
}

fn clone_git_template(url: &str) -> ProplateResult<Template> {
    let path = url.strip_prefix("https://github.com/").unwrap();
    let id = path.split("/").collect::<Vec<_>>().join("-");
    let path = format!(".temp/{}-{}", id, Uuid::new_v4());
    git::exec_cmd(["clone", url, &path], &current_dir().unwrap())
        .map_err(|_| ProplateError::remote_template_not_found(url))?;
    explore_meta(path.try_into().unwrap(), &id, Some(url.to_string()))
        .map_err(|e| ProplateError::fs(&e.to_string()))
}

fn explore_meta(path: PathBuf, id: &str, source: Option<String>) -> Result<Template, Error> {
    let file_list = read_dir(&path)?
        .into_iter()
        .filter_map(|e| match e {
            Ok(entry) => Some(entry.file_name()),
            _ => None,
        })
        .collect::<Vec<_>>();
    Ok(Template::build(id.to_string(), path, file_list, source))
}

fn is_remote_loc(location: &str) -> bool {
    location.starts_with("https://github.com/")
}