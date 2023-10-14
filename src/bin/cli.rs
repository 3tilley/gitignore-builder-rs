use clap::{Parser, Subcommand};
use gitignore_builder_rs::Gitignore;
use std::path::Path;
use strum::EnumString;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(arg_required_else_help = true)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
    #[command(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}

#[derive(EnumString, Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[strum(ascii_case_insensitive)]
enum Source {
    Disk,
    API,
    DB,
}

#[derive(Subcommand, Debug)]
enum Command {
    Fetch { lang: String },
    Merge { lang: Vec<String> },
    ListAll { source: Source },
}

// impl Into<Gitignore> for Command::Merge {
//     fn into(self) -> Gitignore {
//         Gitignore { langs: self.langs }
//     }
// }

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    match &args.command {
        Command::Fetch { lang } => {
            let res = gitignore_builder_rs::fetch_ignores(Gitignore {
                lang: vec![lang.clone()],
            })
            .await;
            println!("{}", res)
        }
        Command::Merge { lang } => {
            let res = gitignore_builder_rs::fetch_ignores(Gitignore { lang: lang.clone() }).await;
            println!("{}", res)
        }
        Command::ListAll { source } => {
            println!("Fetching the list of gitignores from GitHub");
            match source {
                Source::Disk => {
                    // Use fs_err to read a file from disk and deserialise with serde
                    let path = Path::new(file!())
                        .parent()
                        .unwrap()
                        .parent()
                        .unwrap()
                        .parent()
                        .unwrap()
                        .join("data")
                        .join("schema.json");
                    let f = fs_err::File::open(path).unwrap();
                    let j: gitignore_builder_rs::github::Root = serde_json::from_reader(f).unwrap();
                    println!("{:?}", j.title)
                }
                Source::API => {
                    unimplemented!()
                }
                Source::DB => unimplemented!(),
            }
        }
    }
}
