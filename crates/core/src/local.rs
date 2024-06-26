use std::{
  env::current_exe,
  path::{Path, PathBuf},
};

use proplate_errors::{ProplateError, ProplateErrorKind, ProplateResult, TemplateErrorKind};

pub fn local_template_path() -> PathBuf {
  proplate_dir().join("builtins").join("templates")
}

pub fn get_local_template<P>(path: P) -> ProplateResult<PathBuf>
where
  P: AsRef<Path> + Copy,
{
  let tpath = local_template_path().join(path);
  match tpath.exists() {
    true => Ok(tpath),
    _ => Err(
      ProplateError::create(ProplateErrorKind::Template {
        kind: TemplateErrorKind::NotFound { is_remote: false },
        location: path.as_ref().display().to_string(),
      })
      .with_ctx("template:get_local"),
    ),
  }
}

pub fn proplate_dir() -> PathBuf {
  let exe = current_exe().expect("Unable to resolve proplate path");
  exe.parent().unwrap().to_owned()
}
