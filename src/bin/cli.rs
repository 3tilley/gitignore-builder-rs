use clap::{Parser, Subcommand};
use gitignore_builder_rs::Gitignore;

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

#[derive(Subcommand, Debug)]
enum Command {
    Fetch {
        lang: String,
    },
    Merge {
        lang: Vec<String>
    },
    ListAll,
}


/// Derive Clap subcommands to be used in the CLI
/// Merge, diff, fetch-all, fetch
impl Into<Gitignore> for Command::Merge {
    fn into(self) -> Gitignore {
        Gitignore { langs: self.langs }
    }
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    match args.command {
        Command::Fetch { lang } => {
            let res = gitignore_builder_rs::fetch_ignores(args.into()).await;
            println!("{}", res)
        },
        Command::Merge { lang } => {
            let res = gitignore_builder_rs::fetch_ignores(args.into()).await;
            println!("{}", res)
        }
        Command::ListAll => {
            println!("Fetching the list of gitignores from GitHub")
        }
    }


}
