use crate::store::OUTPUT;
use crate::store::VCF;
use crate::store::{GenCodeExon, GenCodeGene, GenCodeTranscript};
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::Command;

/*
Gaurav Sablok,
codeprog@icloud.com
*/

pub fn varaltanno(pathfile: &str, variant: &str) -> Result<String, Box<dyn Error>> {
    let pathnew = Path::new("gencode.v48.chr_patch_hapl_scaff.annotation.gtf");
    if !pathnew.exists() {
        let _ = Command::new("wget").
        arg("https://ftp.ebi.ac.uk/pub/databases/gencode/Gencode_human/release_48/gencode.v48.chr_patch_hapl_scaff.annotation.gtf.gz")
        .output()
        .expect("command failed");
        let _ = Command::new("gunzip")
            .arg("gencode.v48.chr_patch_hapl_scaff.annotation.gtf.gz")
            .output()
            .expect("command failed");
        let fileopen = File::open(pathfile).expect("file not present");
        let fileread = BufReader::new(fileopen);
        let gtfresults_gene: Vec<GenCodeGene> =
            gtfread_gene_annotation("gencode.v48.chr_patch_hapl_scaff.annotation.gtf").unwrap();
        let gtfresults_exon: Vec<GenCodeExon> =
            gtfread_exon_annotation("gencode.v48.chr_patch_hapl_scaff.annotation.gtf").unwrap();
        let gtfresults_transcript: Vec<GenCodeTranscript> =
            gtfread_transcript_annotation("gencode.v48.chr_patch_hapl_scaff.annotation.gtf")
                .unwrap();
        let mut vcstring_file: Vec<VCF> = Vec::new();
        for i in fileread.lines() {
            let linevcf = i.expect("file not present");
            if !linevcf.starts_with("#") {
                let linevec: Vec<String> = linevcf
                    .split("\t")
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>();
                vcstring_file.push(VCF {
                    chrom: linevec[0].to_string(),
                    pos: linevec[1].parse::<usize>().unwrap(),
                    id: linevec[2].to_string(),
                    refnuc: linevec[3].to_string(),
                    altnuc: linevec[4].to_string(),
                    qual: linevec[5].to_string(),
                });
            }
        }

        let mut output_gene: Vec<OUTPUT> = Vec::new();
        let mut output_exon: Vec<OUTPUT> = Vec::new();
        let mut output_transcript: Vec<OUTPUT> = Vec::new();

        for i in vcstring_file.iter() {
            for j in gtfresults_gene.iter() {
                if i.pos > j.start && i.pos <= j.stop && i.altnuc == variant {
                    output_gene.push(OUTPUT {
                        chrom: i.chrom.clone(),
                        pos: i.pos.clone().to_string(),
                        id: i.id.clone(),
                        refnuc: i.refnuc.clone(),
                        altnuc: i.altnuc.clone(),
                        typeannotate: j.typeannotate.clone(),
                        geneid: j.geneid.clone(),
                        genename: j.genename.clone(),
                    });
                }
            }
        }

        for i in vcstring_file.iter() {
            for j in gtfresults_exon.iter() {
                if i.pos > j.start && i.pos <= j.stop && i.altnuc == variant {
                    output_exon.push(OUTPUT {
                        chrom: i.chrom.clone(),
                        pos: i.pos.clone().to_string(),
                        id: i.id.clone(),
                        refnuc: i.refnuc.clone(),
                        altnuc: i.altnuc.clone(),
                        typeannotate: j.typeannotate.clone(),
                        geneid: j.geneid.clone(),
                        genename: j.genename.clone(),
                    });
                }
            }
        }

        for i in vcstring_file.iter() {
            for j in gtfresults_transcript.iter() {
                if i.pos > j.start && i.pos <= j.stop && i.altnuc == variant {
                    output_transcript.push(OUTPUT {
                        chrom: i.chrom.clone(),
                        pos: i.pos.clone().to_string(),
                        id: i.id.clone(),
                        refnuc: i.refnuc.clone(),
                        altnuc: i.altnuc.clone(),
                        typeannotate: j.typeannotate.clone(),
                        geneid: j.geneid.clone(),
                        genename: j.genename.clone(),
                    });
                }
            }
        }

        let mut mutwrite_gene = File::create("annotationfile-gene.txt").expect("file not present");
        let mut mutwrite_exon = File::create("annotationfile-exon.txt").expect("file not present");
        let mut mutwrite_transcript =
            File::create("annotationfile-transcript.txt").expect("file not present");

        for i in output_gene.iter() {
            writeln!(
                mutwrite_gene,
                "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
                i.chrom, i.pos, i.id, i.refnuc, i.altnuc, i.typeannotate, i.geneid, i.genename
            )
            .expect("line not found");
        }

        for i in output_exon.iter() {
            writeln!(
                mutwrite_exon,
                "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
                i.chrom, i.pos, i.id, i.refnuc, i.altnuc, i.typeannotate, i.geneid, i.genename
            )
            .expect("line not found");
        }

        for i in output_transcript.iter() {
            writeln!(
                mutwrite_transcript,
                "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
                i.chrom, i.pos, i.id, i.refnuc, i.altnuc, i.typeannotate, i.geneid, i.genename
            )
            .expect("line not found");
        }
    }
    Ok("The regions have been annotated".to_string())
}

pub fn gtfread_gene_annotation(gtffile: &str) -> Result<Vec<GenCodeGene>, Box<dyn Error>> {
    let fileopen = File::open(gtffile).expect("file not found");
    let fileread = BufReader::new(fileopen);
    let mut gtf_vector: Vec<GenCodeGene> = Vec::new();
    for i in fileread.lines() {
        let linegtf = i.expect("line not found");
        if !linegtf.starts_with("#") {
            let linevec: Vec<String> = linegtf
                .split("\t")
                .map(|x| x.to_string())
                .collect::<Vec<String>>();
            if linevec[2] == "gene" {
                let linecollect: String = linevec[8].split(";").collect::<Vec<_>>()[2]
                    .replace(" ", "-")
                    .split("-")
                    .collect::<Vec<_>>()[2]
                    .to_string()
                    .replace("\"", "");
                let genecollect: String = linevec[8].split(";").collect::<Vec<_>>()[0]
                    .replace(" ", "-")
                    .split("-")
                    .collect::<Vec<_>>()[1]
                    .to_string()
                    .replace("\"", "");
                gtf_vector.push(GenCodeGene {
                    chrom: linevec[0].clone(),
                    typeannotate: linevec[2].clone(),
                    start: linevec[3].parse::<usize>().unwrap(),
                    stop: linevec[4].parse::<usize>().unwrap(),
                    geneid: genecollect,
                    genename: linecollect,
                })
            }
        }
    }
    Ok(gtf_vector)
}

pub fn gtfread_exon_annotation(gtffile: &str) -> Result<Vec<GenCodeExon>, Box<dyn Error>> {
    let fileopen = File::open(gtffile).expect("file not found");
    let fileread = BufReader::new(fileopen);
    let mut gtf_vector: Vec<GenCodeExon> = Vec::new();
    for i in fileread.lines() {
        let linegtf = i.expect("line not found");
        if !linegtf.starts_with("#") {
            let linevec: Vec<String> = linegtf
                .split("\t")
                .map(|x| x.to_string())
                .collect::<Vec<String>>();
            if linevec[2] == "exon" {
                let linecollect: String = linevec[8].split(";").collect::<Vec<_>>()[3]
                    .replace(" ", "-")
                    .split("-")
                    .collect::<Vec<_>>()[2]
                    .to_string()
                    .replace("\"", "");
                let genecollect: String = linevec[8].split(";").collect::<Vec<_>>()[0]
                    .replace(" ", "-")
                    .split("-")
                    .collect::<Vec<_>>()[1]
                    .to_string()
                    .replace("\"", "");

                gtf_vector.push(GenCodeExon {
                    chrom: linevec[0].clone(),
                    typeannotate: linevec[2].clone(),
                    start: linevec[3].parse::<usize>().unwrap(),
                    stop: linevec[4].parse::<usize>().unwrap(),
                    geneid: genecollect,
                    genename: linecollect,
                })
            }
        }
    }
    Ok(gtf_vector)
}

pub fn gtfread_transcript_annotation(
    gtffile: &str,
) -> Result<Vec<GenCodeTranscript>, Box<dyn Error>> {
    let fileopen = File::open(gtffile).expect("file not found");
    let fileread = BufReader::new(fileopen);
    let mut gtf_vector: Vec<GenCodeTranscript> = Vec::new();
    for i in fileread.lines() {
        let linegtf = i.expect("line not found");
        if !linegtf.starts_with("#") {
            let linevec: Vec<String> = linegtf
                .split("\t")
                .map(|x| x.to_string())
                .collect::<Vec<String>>();
            if linevec[2] == "transcript" {
                let linecollect: String = linevec[8].split(";").collect::<Vec<_>>()[3]
                    .replace(" ", "-")
                    .split("-")
                    .collect::<Vec<_>>()[2]
                    .to_string()
                    .replace("\"", "");
                let genecollect: String = linevec[8].split(";").collect::<Vec<_>>()[0]
                    .replace(" ", "-")
                    .split("-")
                    .collect::<Vec<_>>()[1]
                    .to_string()
                    .replace("\"", "");
                gtf_vector.push(GenCodeTranscript {
                    chrom: linevec[0].clone(),
                    typeannotate: linevec[2].clone(),
                    start: linevec[3].parse::<usize>().unwrap(),
                    stop: linevec[4].parse::<usize>().unwrap(),
                    geneid: genecollect,
                    genename: linecollect,
                })
            }
        }
    }
    Ok(gtf_vector)
}
