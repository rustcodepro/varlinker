use crate::annotation::ontologyannotate;
use crate::args::CommandParse;
use crate::args::Commands;
use crate::clinicvar::clinvarmapper;
use crate::clinvarlinker::clinvarvcf;
use crate::databases::databasedownload;
use crate::medgen::cuiparallel;
use crate::ncbigeneid::ncbiannotate;
use crate::omim::omimevidence;
use crate::phenotype::phenotypeall;
use clap::Parser;

/*
 Author Gaurav Sablok,
 Email: codeprog@icloud.com
 Date: 2025-4-8
*/

fn main() {
    let argsparse = CommandParse::parse();
    match &argsparse.command {
        Commands::CUIGENERATE {
            medgenhpo,
            medgen_omim,
            medgenmapping,
            medgenpubmed,
        } => {
            let command = cuiparallel(medgenhpo, medgen_omim, medgenmapping, medgenpubmed).unwrap();
            println!("The command has been completed:{:?}", command);
        }
        Commands::OMIM {
            omimfile,
            evidencenumber,
            hpomapping,
            hpomedgen,
        } => {
            let command = omimevidence(omimfile, evidencenumber, hpomapping, hpomedgen).unwrap();
            println!("The links for the given evidence are:{:?}", command);
        }
        Commands::CLINVAROMIMEVIDENCE {
            clinvar,
            medgen,
            medgenhpo,
            omim,
        } => {
            let command = clinvarmapper(clinvar, medgen, medgenhpo, omim).unwrap();
            println!("The command has completed:{:?}", command);
        }
        Commands::NCBIANNOTATE {
            ncbigeneid,
            clinvar,
            medgenomim,
            medgenhpo,
            omimsearch,
        } => {
            let command =
                ncbiannotate(ncbigeneid, clinvar, medgenomim, medgenhpo, omimsearch).unwrap();
            println!("The command has completed:{:?}", command);
        }
        Commands::ANNOTATOR {
            pathncbimaxo,
            medgenomim,
            medgenhpo,
            evidence,
        } => {
            let command = ontologyannotate(pathncbimaxo, medgenomim, medgenhpo, evidence).unwrap();
            println!("The command has been completed:{:?}", command)
        }
        Commands::VCFCLINVARANNOTATE { vcffile, clinvar } => {
            let command = clinvarvcf(vcffile, clinvar).unwrap();
            println!(
                "The command has finished and the annotated vcf file has been written:{:?}",
                command
            );
        }
        Commands::PHENOTYPELINKER {
            genesdisease,
            genesphenotype,
            phenotypehpoa,
            phenotypesgenes,
        } => {
            let command =
                phenotypeall(genesdisease, genesphenotype, phenotypehpoa, phenotypesgenes).unwrap();
            println!("The command has finished: {:?}", command);
        }
        Commands::Databases { databaseoption } => {
            let command = databasedownload(*databaseoption).unwrap();
            println!(
                "The command has been finished and all the files have been downloaded:{}",
                command
            );
        }
    }
}
