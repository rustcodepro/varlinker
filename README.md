# varlinker

- Parallel threaded variant linker.
- sample vcf file from [vcflib](https://github.com/vcflib/vcflib/blob/master/samples/sample.vcf)

```
__   __   __ _   _ __  | |     (_)  _ __   | | __   ___   _ __
__   __   __ _   _ __  | |     (_)  _ __   | | __   ___   _ __
\ \ / /  / _` | | '__| | |     | | | '_ \  | |/ /  / _ \ | '__|
 \ V /  | (_| | | |    | |___  | | | | | | |   <  |  __/ | |
  \_/    \__,_| |_|    |_____| |_| |_| |_| |_|\_\  \___| |_|


specific position annotator for human genomics.
      ************************************************
      Author Gaurav Sablok,
      Email: codeprog@icloud.com
     ************************************************

Usage: varlinker <COMMAND>

Commands:
 variant-linker    annotate the specific coordinate
 variant-trefanno  extract the annotation of the specific ref allele
 variant-taltanno  extract the annotation of the specific alt allele
 help              Print this message or the help of the given subcommand(s)

Options:
 -h, --help     Print help
 -V, --version  Print version
```

- Annotate only the ref variant with A
```
./target/debug/varlinker variant-trefanno ./sample-files/sample.vcf A
```

- Annotate only the alt variant with A
```
./target/debug/varlinker variant-taltanno ./sample-files/sample.vcf T
```
- Annotate all the variants in the vcf
```
./target/debug/varlinker variant-linker ./sample-files/sample.vcf
```

- it will produce three output files classifying variant annotation to gene, exon and transcript level.

```
==> annotationfile-exon.txt <==
19      111     .       A       C       exon    ENSG00000284900.2       KBTBD4
19      111     .       A       C       exon    ENSG00000284900.2       KBTBD4
19      111     .       A       C       exon    ENSG00000284900.2       KBTBD4
19      111     .       A       C       exon    ENSG00000284900.2       KBTBD4

==> annotationfile-gene.txt <==
19      111     .       A       C       gene    ENSG00000284900.2       KBTBD4
19      111     .       A       C       gene    ENSG00000278550.4       SLC43A2
19      111     .       A       C       gene    ENSG00000264450.1       ENSG00000264450
19      111     .       A       C       gene    ENSG00000273929.1       U6

==> annotationfile-transcript.txt <==
19      111     .       A       C       transcript      ENSG00000284900.2       KBTBD4
19      111     .       A       C       transcript      ENSG00000284900.2       KBTBD4
19      111     .       A       C       transcript      ENSG00000284900.2       KBTBD4
19      111     .       A       C       transcript      ENSG00000284900.2       KBTBD4
```

- To install windows version:
```
rustup component add llvm-tools
rustup target add x86_64-pc-windows-msvc
git clone https://github.com/IBCHgenomic/varlinker.git
cd ensemblcov
cargo xwin build --target x86_64-pc-windows-msvc
```

Gaurav Sablok \
codeprog@icloud.com
