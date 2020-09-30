mod cli;
mod util;
use cli::{CompletionOpt, Opt, StructOpt, Subcommand};
use code_minimap::{print_minimap, Opt as MinimapOpt};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    util::reset_signal_pipe_handler();
    let opt: Opt = Opt::from_args();
    match &opt.subcommand {
        Some(Subcommand::Completion(CompletionOpt { shell })) => {
            Opt::clap().gen_completions_to(env!("CARGO_PKG_NAME"), *shell, &mut std::io::stdout());
        }
        None => {
            let reader: Box<dyn BufRead> = match &opt.file {
                Some(path) => Box::new(BufReader::new(File::open(path)?)),
                None => Box::new(BufReader::new(io::stdin())),
            };
            let minimap_opt = MinimapOpt {
                file:    opt.file,
                hscale:  opt.hscale,
                vscale:  opt.vscale,
                padding: opt.padding,
            };
            print_minimap(reader, &minimap_opt).unwrap();
        }
    }
    Ok(())
}