use anyhow::{Error, Ok};
use clap::Parser;
use log::info;
use std::time::Instant;

mod cli;
use cli::*;
mod logger;
use logger::*;
mod errors;

mod cmd;
mod utils;
use cmd::{
    codon::*, fa2fq::*, faidx::*, flatten::*, grep::*, kmer::*, range::*, rename::*, reverse::*,
    search::*, seq::*, shuffle::*, size::*, slide::*, sort::*, split::*, split2::*, subfa::*,
    summ::*, tail::*, top::*,
};

fn main() -> Result<(), Error> {
    let args = cli::Args::parse();
    logger(args.verbose, args.logfile, args.quiet)?;
    let start = Instant::now();
    info!("version: {}", env!("CARGO_PKG_AUTHORS"));

    match args.command {
        Subcli::topn { input, num, output } => {
            top_n_records(
                num,
                input.as_ref(),
                output.as_ref(),
                args.width,
                args.compression_level,
            )?;
        }
        Subcli::tail {
            input,
            num,
            two_pass,
            output,
        } => {
            tail_n_records(
                num,
                input.as_ref(),
                two_pass,
                output.as_ref(),
                args.width,
                args.compression_level,
            )?;
        }
        Subcli::fa2fq {
            input,
            qual,
            keep,
            output,
        } => {
            fake_quality(
                input.as_ref(),
                qual,
                keep,
                output.as_ref(),
                args.compression_level,
            )?;
        }
        Subcli::faidx {
            input,
            region,
            output,
        } => {
            faidx_fasta(
                input.as_ref(),
                region,
                output.as_ref(),
                args.compression_level,
            )?;
        }
        Subcli::kmer {
            input,
            size,
            header,
            out,
        } => {
            kmer_count(
                input.as_ref(),
                size,
                header,
                out.as_ref(),
                args.compression_level,
            )?;
        }
        Subcli::flatten {
            input,
            keep,
            gap,
            len,
            gc,
            sep,
            output,
        } => {
            flatten_fa(
                input.as_ref(),
                keep,
                gap,
                len,
                gc,
                sep,
                output.as_ref(),
                args.compression_level,
            )?;
        }
        Subcli::range {
            input,
            skip,
            take,
            out,
        } => {
            range_fasta(
                input.as_ref(),
                skip,
                take,
                out.as_ref(),
                args.width,
                args.compression_level,
            )?;
        }
        Subcli::rename {
            input,
            keep,
            prefix,
            output,
        } => {
            rename_fa(
                input.as_ref(),
                keep,
                prefix,
                output.as_ref(),
                args.width,
                args.compression_level,
            )?;
        }
        Subcli::reverse { input, rev, out } => {
            reverse_comp_seq(
                input.as_ref(),
                out.as_ref(),
                rev,
                args.width,
                args.compression_level,
            )?;
        }
        Subcli::window {
            input,
            wind,
            step,
            keep,
            output,
        } => {
            silding_window(
                step,
                wind,
                input.as_ref(),
                output.as_ref(),
                keep,
                args.compression_level,
            )?;
        }
        Subcli::grep {
            input,
            pat,
            name,
            seq,
            ignore,
            output,
        } => {
            grep_fasta(
                input.as_ref(),
                output.as_ref(),
                &pat,
                ignore,
                name,
                seq,
                args.width,
                args.compression_level,
            )?;
        }
        Subcli::seq {
            input,
            lower,
            upper,
            sequence,
            min,
            max,
            gc_min,
            gc_max,
            out,
        } => {
            seq_fa(
                input.as_ref(),
                lower,
                upper,
                min,
                max,
                gc_min,
                gc_max,
                sequence,
                out.as_ref(),
                args.width,
                args.compression_level,
            )?;
        }
        Subcli::sort {
            input,
            name,
            seq,
            gc,
            length,
            reverse,
            out,
        } => {
            sort_fasta(
                input.as_ref(),
                name,
                seq,
                gc,
                length,
                reverse,
                out.as_ref(),
                args.width,
                args.compression_level,
            )?;
        }
        Subcli::shuffle { input, seed, out } => {
            shuffle_fasta(
                input.as_ref(),
                seed,
                out.as_ref(),
                args.width,
                args.compression_level,
            )?;
        }
        Subcli::search {
            input,
            pat,
            Header,
            output,
        } => {
            search_fa(
                input.as_ref(),
                output.as_ref(),
                &pat,
                Header,
                args.compression_level,
            )?;
        }
        Subcli::size {
            input,
            all,
            keep,
            noheader,
            output,
        } => {
            size_fasta(
                input.as_ref(),
                all,
                keep,
                noheader,
                output.as_ref(),
                args.compression_level,
            )?;
        }
        Subcli::subfa {
            input,
            seed,
            num,
            rdc,
            output,
        } => {
            if rdc {
                select_fasta(
                    input.as_ref(),
                    num,
                    seed,
                    output.as_ref(),
                    args.width,
                    args.compression_level,
                )?;
            } else {
                select_fasta2(
                    input.as_ref(),
                    num,
                    seed,
                    output.as_ref(),
                    args.width,
                    args.compression_level,
                )?;
            }
        }
        Subcli::summ { file, all, output } => {
            let buf = file.iter().map(|x| x.as_str()).collect();
            if let Some(path) = output {
                summary_fa(buf, all, Some(&path), args.compression_level)?;
            } else {
                summary_fa(buf, all, None, args.compression_level)?;
            }
        }
        Subcli::split { input, ext, outdir } => {
            split_fa(
                input.as_ref(),
                ext,
                outdir.as_ref(),
                args.width,
                args.compression_level,
            )?;
        }
        Subcli::split2 {
            input,
            num,
            gzip,
            bzip2,
            xz,
            name,
        } => {
            split_chunk(
                input.as_ref(),
                num,
                gzip,
                bzip2,
                xz,
                &name,
                args.width,
                args.compression_level,
            )?;
        }
        Subcli::codon { name } => {
            show_codon(name)?;
        }
    }

    info!("time elapsed is: {:?}", start.elapsed());
    Ok(())
}
