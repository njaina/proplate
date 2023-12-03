use clap::{arg, Command};
use cmd::create::create;

mod cmd;
mod errors;
mod settings;
mod shell;
mod template;
mod ui;
mod util;

fn cli() -> Command {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    Command::new("proplate")
        .version(VERSION)
        .author("Yume Saiko <yumii.saiko@gmail.com>")
        .about(
            r#"
    ▄▄▄▄▄▄▄ ▄▄▄▄▄▄   ▄▄▄▄▄▄▄ ▄▄▄▄▄▄▄ ▄▄▄     ▄▄▄▄▄▄ ▄▄▄▄▄▄▄ ▄▄▄▄▄▄▄ 
    █       █   ▄  █ █       █       █   █   █      █       █       █
    █    ▄  █  █ █ █ █   ▄   █    ▄  █   █   █  ▄   █▄     ▄█    ▄▄▄█
    █   █▄█ █   █▄▄█▄█  █ █  █   █▄█ █   █   █ █▄█  █ █   █ █   █▄▄▄ 
    █    ▄▄▄█    ▄▄  █  █▄█  █    ▄▄▄█   █▄▄▄█      █ █   █ █    ▄▄▄█
    █   █   █   █  █ █       █   █   █       █  ▄   █ █   █ █   █▄▄▄ 
    █▄▄▄█   █▄▄▄█  █▄█▄▄▄▄▄▄▄█▄▄▄█   █▄▄▄▄▄▄▄█▄█ █▄▄█ █▄▄▄█ █▄▄▄▄▄▄▄█
    
Any Project starter in one tool"#,
        )
        .subcommand(
            Command::new("create")
                .about("create project from template")
                .args(&[
                    arg!(--template <template> "Template id to start from").required(true),
                    arg!(--dest <dest> "Destination path").required(true),
                ]),
        )
}

fn main() -> Result<(), clap::Error> {
    let matches = cli().get_matches();
    let subcommands = matches.subcommand();

    match subcommands {
        Some(cmd) => match cmd {
            ("create", args) => {
                let template_id = args.get_one::<String>("template").unwrap().as_str();
                let dest = args.get_one::<String>("dest").unwrap().as_str();
                create(template_id, dest).expect(
                    format!(
                        "Unable to create boilerplate from Template [{}]",
                        template_id
                    )
                    .as_str(),
                )
            }
            _ => {}
        },
        _ => cli().print_help()?,
    }

    Ok(())
}
