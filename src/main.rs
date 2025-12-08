mod args;
mod store;
mod varaltannot;
mod variantlinker;
mod varrefannot;
use crate::args::CommandParse;
use crate::args::Commands;
use crate::varaltannot::varaltanno;
use crate::variantlinker::varlinker;
use crate::varrefannot::varrefanno;
use clap::Parser;
use figlet_rs::FIGfont;

/*
Author Gaurav Sablok,
Email: codeprog@icloud.com
*/

fn main() {
    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert("varLinker");
    assert!(figure.is_some());
    println!("{}", figure.unwrap());
    let argsparse = CommandParse::parse();
    match &argsparse.command {
        Commands::VariantLINKER { vcfile, thread } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(thread.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = varlinker(vcfile).unwrap();
                println!("The command has been completed:{:?}", command);
            });
        }
        Commands::VariantTALTANNO {
            vcffile,
            altallel,
            thread,
        } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(thread.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = varaltanno(vcffile, altallel).unwrap();
                println!("The command has been completed:{:?}", command);
            });
        }
        Commands::VariantTREFANNO {
            vcffile,
            refallele,
            thread,
        } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(thread.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = varrefanno(vcffile, refallele).unwrap();
                println!("The command has been completed:{:?}", command);
            });
        }
    }
}
