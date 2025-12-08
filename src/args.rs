use clap::{Parser, Subcommand};
#[derive(Debug, Parser)]
#[command(
    name = "varlinker",
    version = "1.0",
    about = "specific position annotator for human genomics.
       ************************************************
       Author Gaurav Sablok,
       Email: codeprog@icloud.com
      ************************************************"
)]
pub struct CommandParse {
    /// subcommands for the specific actions
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// annotate the specific coordinate
    VariantLINKER {
        /// variant VCF file
        vcfile: String,
        /// threads for the analysis
        thread: String,
    },
    /// extract the annotation of the specific ref allele
    VariantTREFANNO {
        /// variant VCF file
        vcffile: String,
        /// ref allele
        refallele: String,
        /// threads for the analysis
        thread: String,
    },
    /// extract the annotation of the specific alt allele
    VariantTALTANNO {
        /// variant VCF file
        vcffile: String,
        /// alt allele
        altallel: String,
        /// threads for the analysis
        thread: String,
    },
}
