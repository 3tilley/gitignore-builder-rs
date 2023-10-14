use clap::Parser;
use gitignore_builder_rs::Gitignore;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    langs: Vec<String>,
}

impl Into<Gitignore> for Args {
    fn into(self) -> Gitignore {
        Gitignore { langs: self.langs }
    }
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let res = gitignore_builder_rs::fetch_ignores(args.into()).await;
    println!("{}", res)

}
