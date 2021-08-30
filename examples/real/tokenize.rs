extern crate clap;
use clap::Clap;

#[allow(dead_code)]
use super::helpers::*;

#[derive(Debug, Clap)]
struct Args {
    #[clap(about = "file/dir to parse")]
    pattern: Option<String>,

    #[clap(short = 'e', about = "code to evaluate")]
    code_to_eval: Option<String>,

    #[clap(short, long, about = "don't print anything except OK/Error per file")]
    quiet: bool,

    #[clap(
        long,
        about = "comma-separated list of debug levels (parser, lexer, buffer)"
    )]
    debug_level: Option<DebugLevel>,
}

#[allow(dead_code)]
pub(crate) fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Args = Args::parse();

    fn print_nothing(_: TokenList) {}

    fn print_all(tokens: TokenList) {
        println!("{:?}", tokens)
    }
    let callback = if args.quiet { print_nothing } else { print_all };
    let debug_level = args.debug_level.unwrap_or_default();

    let files = InputFiles::new(&args.code_to_eval, &args.pattern, &None);

    for file in files.into_iter() {
        let tokens = tokenize(file, debug_level)?;
        println!("OK");
        callback(TokenList { tokens });
    }

    return Ok(());
}