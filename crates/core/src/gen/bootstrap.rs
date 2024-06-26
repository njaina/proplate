use std::{collections::HashMap, fs, path::Path};

use crate::{
  fs as pfs,
  template::{config::analyze_dyn_files, interpolation::Interpolate, op::Execute, Template},
};

use proplate_errors::{ProplateError, ProplateErrorKind, ProplateResult};
use proplate_tui::logger;

/// typealias for template ctx
pub type Context = HashMap<String, String>;

/// Processes the given `template` using the `ctx` and outputs the result to `dest` directory
pub fn bootstrap(template: &mut Template, dest: &str, ctx: &Context) -> ProplateResult<()> {
  (|| -> ProplateResult<()> {
    process_template(template, ctx)?;
    prepare_dest(dest)?;
    copy_files(template, dest)?;
    cleanup(template)?;
    Ok(())
  })()
  .map_err(|e| -> ProplateError {
    if let Err(_) = cleanup(template) {
      logger::warn("Unable to cleanup");
    }
    e
  })
}

/// Executes hook and bind ctx onto dynamic_files.
pub fn process_template(template: &mut Template, ctx: &Context) -> ProplateResult<()> {
  println!("{}", logger::step("Running additional operations..."));

  // run "additional_operations" in order to process the dynamically
  // added file in the extra operation.
  for op in &template.conf.additional_operations {
    op.execute(&ctx)?;
  }

  println!(
    "{}",
    logger::step("Verifying whether analysis of dyn files is necessary...")
  );
  if template.conf.require_dyn_file_analysis {
    analyze_dyn_files(&mut template.conf, &template.base_path);
  }

  println!("{}", logger::step("Binding ctx to dynamic_files..."));

  for filepath in &template.conf.dynamic_files {
    println!("      {}", logger::step(&format!("processing...")));
    bind_ctx_to_file(Path::new(&filepath), ctx);
  }

  Ok(())
}

/// Replaces dynamic var "$var" with their actual value
pub fn bind_ctx_to_file(path: &Path, ctx: &Context) {
  match pfs::map_file(path, |s| s.to_string().interpolate(ctx)) {
    Err(_) => {
      // TODO: warn if not found but wasn't removed in additional_op either
    }
    _ => (),
  }
}

/// Create project dest dir
fn prepare_dest(dest: &str) -> ProplateResult<()> {
  println!("{}", logger::title("Finalizing"));
  fs::create_dir_all(dest).map_err(|e| {
    ProplateError::create(ProplateErrorKind::Fs {
      concerned_paths: vec![dest.into()],
      operation: "create_dir_all".into(),
    })
    .with_ctx("gen:bootstrap:prepare")
    .with_cause(&e.to_string())
  })?;
  Ok(())
}

/// Copies template file to the provided dest
/// Files under "meta.exclude" won't be copied
pub fn copy_files(template: &Template, dest: &str) -> ProplateResult<()> {
  let src = &template.base_path;
  let dest = Path::new(dest);

  println!("{}", logger::step("Copying..."));

  pfs::copy_fdir(
    src,
    dest,
    Some(
      template
        .conf
        .exclude
        .iter()
        .map(|s| s.into())
        .collect::<Vec<_>>(),
    ),
  )
  .map_err(|e| {
    ProplateError::create(ProplateErrorKind::Fs {
      concerned_paths: vec![src.display().to_string(), dest.display().to_string()],
      operation: "copy_fdir".into(),
    })
    .with_ctx("gen:bootstrap:copy_files")
    .with_cause(&e.to_string())
  })
}

pub fn cleanup(template: &Template) -> ProplateResult<()> {
  println!("{}", logger::step("cleaning up..."));
  fs::remove_dir_all(&template.base_path).map_err(|e| {
    ProplateError::create(ProplateErrorKind::Fs {
      concerned_paths: vec![template.base_path.display().to_string()],
      operation: "remove_dir_all".into(),
    })
    .with_ctx("gen:bootstrap:cleanup")
    .with_cause(&e.to_string())
  })?;
  Ok(())
}
