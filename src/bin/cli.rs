use clap::{Parser, Subcommand};
use gitignore_builder_rs::telemetry::prepare_logging;
use gitignore_builder_rs::{available_ignores_from_file, get_matching_ignores, Gitignore};
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
    Fetch {
        lang: Option<String>,
    },
    Merge {
        lang: Vec<String>,
    },
    ListAll {
        source: Option<Source>,
        lang: Option<Vec<String>>,
    },
}

// impl Into<Gitignore> for Command::Merge {
//     fn into(self) -> Gitignore {
//         Gitignore { langs: self.langs }
//     }
// }

#[tokio::main]
async fn main() {
    prepare_logging();
    let args = Cli::parse();
    match &args.command {
        Command::Fetch { lang } => {
            let lang = lang.clone().expect("No language specified");
            let igs = available_ignores_from_file();
            let m: Vec<_> = get_matching_ignores(igs, &vec![lang.clone()])
                .into_iter()
                .filter_map(|x| x.ok())
                .collect();
            match m.len() {
                0 => {
                    panic!("No matching gitignore found for {}", lang)
                }
                1 => {
                    let res = gitignore_builder_rs::fetch_ignores(Gitignore { lang: m }).await;
                    println!("{}", res)
                }
                x => {
                    panic!(
                        "Too many matching gitignores found for {}. Found {} matches",
                        lang, x
                    )
                }
            }
        }
        Command::Merge { lang } => {
            let res = gitignore_builder_rs::fetch_ignores(Gitignore { lang: lang.clone() }).await;
            println!("{}", res)
        }
        Command::ListAll { source, lang } => {
            println!("Fetching the list of gitignores from GitHub");
            let source = source.unwrap_or(Source::Disk);
            match source {
                Source::Disk => {
                    let igs = available_ignores_from_file();
                    match lang {
                        Some(langs) => {
                            // TODO: Work out what the sensible return is for an unmatched language
                            let matching: Vec<String> = get_matching_ignores(igs, langs)
                                .into_iter()
                                .flatten()
                                .collect();
                            println!("{}", matching.join("\n"))
                        }
                        None => {
                            println!(
                                "{}",
                                igs.iter()
                                    .map(|x| x.path.clone())
                                    .collect::<Vec<String>>()
                                    .join("\n")
                            )
                        }
                    }

                    //println!("{}", serde_json::to_string_pretty(&j).unwrap())
                }
                Source::API => {
                    unimplemented!()
                }
                Source::DB => unimplemented!(),
            }
        }
    }
}
