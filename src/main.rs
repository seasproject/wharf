use std::path::PathBuf;

use clap::Args;
use clap::Parser;
use clap::Subcommand;
use wharf::*;

fn main() {
    let cli = Cli::parse();
    use Command::*;
    match cli.command {
        Dock(args) => {
            run(args.path);
        }
        Reverse(args) => {
            reverse(args.path);
        }
        Package(args) => {
            package(args.path);
        }
    }
}

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Will run the target "ship" in a forwards manner. Useful for installing.
    Dock(DockArgs),
    /// Will reverse the target "ship"'s commands and execute them, effectively undoing all the work. Useful for uninstalling.
    Reverse(ReverseArgs),
    /// Will package a "ship" into a single file.
    /// This is useful for creating a "ship" that can be distributed to other machines.
    Package(PackageArgs),
}

#[derive(Args)]
struct DockArgs {
    path: PathBuf,
}

#[derive(Args)]
struct ReverseArgs {
    path: PathBuf,
}

#[derive(Args)]
struct PackageArgs {
    path: PathBuf,
}
