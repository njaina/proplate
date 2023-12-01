use std::{
    fs,
    io::{Error, ErrorKind::AlreadyExists},
    path::{Path, PathBuf},
};
use uuid::Uuid;

use crate::{
    colors::{error, step, success, title},
    shell,
    template::{find::find_template_by_id, ForkTemplate},
};

pub fn create(template_id: &str, dest: &str) -> Result<(), Error> {
    println!("{}", title("Setup template"));
    let fork = fork_template(template_id, dest)?;

    let cleanup = || {
        println!("{}", step("removing tmp..."));
        fs::remove_dir_all(&fork.tmp_dir).expect(&error("unable to remove tmp dir"));
    };

    fs::create_dir_all(dest).map_err(|_| {
        cleanup();
        Error::new(
            AlreadyExists,
            error(&format!("out dir already exists: {dest}")),
        )
    })?;

    println!("{}", step("Copying..."));
    shell::copy_directory(&fork.tmp_dir, Path::new(dest)).map_err(|e| {
        cleanup();
        e
    })?;

    println!(
        "{}",
        success("Done, wait a lil moment while we remove temporary files")
    );
    cleanup();

    Ok(())
}

fn fork_template<'a>(id: &'a str, dest: &'a str) -> Result<ForkTemplate<'a>, Error> {
    println!("{}", step("Finding template..."));
    let template = match find_template_by_id(id) {
        Ok(t) => t,
        Err(e) => panic!("{}", e),
    };

    let path_str = format!(".temp/{}-{}", dest, Uuid::new_v4());
    let path_buf = PathBuf::from(path_str);

    fs::create_dir_all(&path_buf)?;

    println!("{}", step("Forking template..."));
    shell::copy_directory(&template.base_path, path_buf.as_path())?;

    Ok(ForkTemplate::new(template, path_buf))
}