use std::collections::HashMap;
use anyhow::Error;
use log::*;
use colored::*;

pub fn show_codon(
    name: Option<String>,
) -> Result<(),Error>{
    let codons = vec![
        "UUU (Phe/F)", "UUC (Phe/F)", "UUA (Leu/L)", "UUG (Leu/L)", "UCU (Ser/S)", "UCC (Ser/S)", "UCA (Ser/S)", "UCG (Ser/S)",
        "UAU (Tyr/Y)", "UAC (Tyr/Y)", "UAA (Stop)",  "UAG (Stop)",  "UGU (Cys/C)", "UGC (Cys/C)", "UGA (Stop)",  "UGG (Trp/W)",
        "CUU (Leu/L)", "CUC (Leu/L)", "CUA (Leu/L)", "CUG (Leu/L)", "CCU (Pro/P)", "CCC (Pro/P)", "CCA (Pro/P)", "CCG (Pro/P)",
        "CAU (His/H)", "CAC (His/H)", "CAA (Gln/Q)", "CAG (Gln/Q)", "CGU (Arg/R)", "CGC (Arg/R)", "CGA (Arg/R)", "CGG (Arg/R)",
        "AUU (Ile/I)", "AUC (Ile/I)", "AUA (Ile/I)", "AUG (Start)", "ACU (Thr/T)", "ACC (Thr/T)", "ACA (Thr/T)", "ACG (Thr/T)",
        "AAU (Asn/N)", "AAC (Asn/N)", "AAA (Lys/K)", "AAG (Lys/K)", "AGU (Ser/S)", "AGC (Ser/S)", "AGA (Arg/R)", "AGG (Arg/R)",
        "GUU (Val/V)", "GUC (Val/V)", "GUA (Val/V)", "GUG (Val/V)", "GCU (Ala/A)", "GCC (Ala/A)", "GCA (Ala/A)", "GCG (Ala/A)",
        "GAU (Asp/D)", "GAC (Asp/D)", "GAA (Glu/E)", "GAG (Glu/E)", "GGU (Gly/G)", "GGC (Gly/G)", "GGA (Gly/G)", "GGG (Gly/G)",
    ];
    let aa: HashMap<String,String> = vec![
            ("M".to_string(),"AUG".to_string()), ("W".to_string(),"UGG".to_string()),
            ("N".to_string(),"AAU,AAC".to_string()), ("D".to_string(),"GAU,GAC".to_string()), ("C".to_string(),"UGU,UGC".to_string()),
            ("E".to_string(),"GAA,GAG".to_string()), ("Q".to_string(),"CAA,CAG".to_string()), ("H".to_string(),"CAU,CAC".to_string()),
            ("K".to_string(),"AAA,AAG".to_string()), ("F".to_string(),"UUU,UUC".to_string()), ("Y".to_string(),"UAU,UAC".to_string()),
            ("I".to_string(),"AUU,AUC,AUA".to_string()),
            ("A".to_string(),"GCU,GCC,GCA,GCG".to_string()), ("G".to_string(),"GGU,GGC,GGA,GGG".to_string()),
            ("P".to_string(),"CCU,CCC,CCA,CCG".to_string()), ("T".to_string(),"ACU,ACC,ACA,ACG".to_string()), ("V".to_string(),"GUU,GUC,GUA,GUG".to_string()),
            ("R".to_string(),"CGU,CGC,CGA,CGG,AGA,AGG".to_string()), ("S".to_string(),"UCU,UCC,UCA,UCG,AGU,AGC".to_string()), ("L".to_string(),"UUA,UUG,CUU,CUC,CUA,CUG".to_string()),
        ].into_iter().collect();
    
    if let Some(name) = name {
        if aa.contains_key(&name){
            println!("{}\t{}",name,aa.get(&name).unwrap());
        } else {
            error!("no amino acid named {}",name);
            std::process::exit(1);
        }
    } else {
        let mut i = 0;
        for iterm in codons {
            i +=1;
            print!("{}\t",iterm);
            if i % 4 == 0 {
                println!();
            }
            if i % 16 == 0 {
                println!();
            }
        }
        println!("Iniation codon: {}", "AUG".green());
        println!("Termination codon: {}", "UAG,UAA,UGA".red())
    }
    
    Ok(())
}