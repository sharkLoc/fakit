use anyhow::{Error, Ok};
use clap::Parser;
use log::error;

mod cli;
use cli::*;
mod logger;
use logger::*;
mod top;
use top::*;
mod tail;
use tail::*;
mod fa2fq;
use fa2fq::*;
mod faidx;
use faidx::*;
mod flatten;
use flatten::*;
mod range;
use range::*;
mod rename;
use rename::*;
mod reverse;
use reverse::*;
mod search;
use search::*;
mod grep;
use grep::*;
mod seq;
use seq::*;
mod shuffle;
use shuffle::*;
mod slide;
use slide::*;
mod sort;
use sort::*;
mod split;
use split::*;
mod split2;
use split2::*;
mod subfa;
use subfa::*;
mod summ;
use summ::*;
mod size;
use size::*;
mod codon;
mod wrap;
use codon::*;
mod utils;

fn main() -> Result<(), Error> {
    let args = cli::Args::parse();
    logger(args.verbose, args.logfile, args.quiet)?;

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
        Subcli::tail { input, num, output } => {
            tail_n_records(
                num,
                input.as_ref(),
                output.as_ref(),
                args.width,
                args.compression_level,
            )?;
        }
        Subcli::fa2fq {
            input,
            qual,
            output,
        } => {
            fake_quality(
                input.as_ref(),
                qual,
                output.as_ref(),
                args.compression_level,
            )?;
        }
        Subcli::faidx { input, region } => {
            if let Some(input) = input {
                if let Some(region) = region {
                    index_reader(&input, region, args.compression_level)?;
                } else {
                    index_fasta(Some(&input), args.compression_level)?;
                }
            } else {
                error!("usage: fakit faidx -h/--help, get more help");
                std::process::exit(1);
            }
        }
        Subcli::flatten {
            input,
            keep,
            output,
        } => {
            flatten_fa(
                input.as_ref(),
                output.as_ref(),
                keep,
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
        Subcli::size { input, all, output } => {
            size_fasta(input.as_ref(), all, output.as_ref(), args.compression_level)?;
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
        Subcli::summ { file, all } => {
            let buf = file.iter().map(|x| x.as_str()).collect();
            summary_fa(buf, all)?;
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

    Ok(())
}
