use std::process::exit;

use criterion::Criterion;

mod bench_rank;
mod bench_select;

enum BenchedStruct {
    Rank9,
    Rank10,
    Rank11,
    Rank12,
    Rank16,
    Poppy,
    CsPoppy,
    SimpleSelect,
    Rank9Sel,
    Rank10Sel,
}

pub fn main() {
    let mut criterion = Criterion::default()
        .without_plots()
        .configure_from_args()
        .with_filter("");

    let mut filter = std::env::args().nth(1).unwrap_or_default();

    if filter.is_empty() || filter.starts_with("-") {
        eprintln!("Usage: cargo bench --bench benches -- <filter>");
        exit(1);
    }

    let uniform = if filter.contains("-nu") {
        filter = filter.replace("-nu", "");
        false
    } else {
        true
    };

    let list_benches = filter.split("+").collect::<Vec<&str>>();

    for bench in list_benches {
        let params = bench.split("_").collect::<Vec<&str>>();
        let benched_struct = match params[0] {
            "rank9" => BenchedStruct::Rank9,
            "rank10" => BenchedStruct::Rank10,
            "rank11" => BenchedStruct::Rank11,
            "rank12" => BenchedStruct::Rank12,
            "rank16" => BenchedStruct::Rank16,
            "poppy" => BenchedStruct::Poppy,
            "cs_poppy" => BenchedStruct::CsPoppy,
            "simple" => BenchedStruct::SimpleSelect,
            "rank9sel" => BenchedStruct::Rank9Sel,
            "rank10sel" => BenchedStruct::Rank10Sel,
            _ => {
                eprintln!("Unknown struct: {}", bench);
                exit(1);
            }
        };

        match benched_struct {
            BenchedStruct::Rank9 => bench_rank::bench_rank9(&mut criterion),
            BenchedStruct::Rank10 => {
                if params.len() != 2 {
                    eprintln!("Usage: rank10_<log2_upper_block_size>");
                    exit(1);
                }
                let log2_upper_block_size = params[1].parse::<usize>().unwrap();
                match log2_upper_block_size {
                    8 => bench_rank::bench_rank10::<8>(&mut criterion),
                    9 => bench_rank::bench_rank10::<9>(&mut criterion),
                    10 => bench_rank::bench_rank10::<10>(&mut criterion),
                    _ => {
                        eprintln!("Invalid log2_upper_block_size: {}", log2_upper_block_size);
                        eprintln!("Valid values: 8, 9, 10");
                        exit(1);
                    }
                }
            }
            BenchedStruct::Rank11 => bench_rank::bench_rank11(&mut criterion),
            BenchedStruct::Rank12 => bench_rank::bench_rank12(&mut criterion),
            BenchedStruct::Rank16 => bench_rank::bench_rank16(&mut criterion),
            BenchedStruct::Poppy => bench_rank::bench_poppy(&mut criterion),
            BenchedStruct::CsPoppy => bench_select::bench_cs_poppy(&mut criterion, uniform),
            BenchedStruct::SimpleSelect => {
                bench_select::bench_simple_select(&mut criterion, uniform)
            }
            BenchedStruct::Rank9Sel => bench_select::bench_rank9sel(&mut criterion, uniform),
            BenchedStruct::Rank10Sel => {
                if params.len() != 3 {
                    eprintln!("Usage: rank10sel_<log2_upper_block_size>_<log2_ones_per_inventory>");
                    exit(1);
                }
                let log2_upper_block_size = params[1].parse::<usize>().unwrap();
                let log2_ones_per_inventory = params[2].parse::<usize>().unwrap();
                match log2_ones_per_inventory {
                    10 => match log2_upper_block_size {
                        8 => bench_select::bench_rank10sel::<8, 10>(&mut criterion, uniform),
                        9 => bench_select::bench_rank10sel::<9, 10>(&mut criterion, uniform),
                        10 => bench_select::bench_rank10sel::<10, 10>(&mut criterion, uniform),
                        _ => {
                            eprintln!("Invalid log2_upper_block_size: {}", log2_upper_block_size);
                            eprintln!("Valid values: 8, 9, 10");
                            exit(1);
                        }
                    },
                    11 => match log2_upper_block_size {
                        8 => bench_select::bench_rank10sel::<8, 11>(&mut criterion, uniform),
                        9 => bench_select::bench_rank10sel::<9, 11>(&mut criterion, uniform),
                        10 => bench_select::bench_rank10sel::<10, 11>(&mut criterion, uniform),
                        _ => {
                            eprintln!("Invalid log2_upper_block_size: {}", log2_upper_block_size);
                            eprintln!("Valid values: 8, 9, 10");
                            exit(1);
                        }
                    },
                    12 => match log2_upper_block_size {
                        8 => bench_select::bench_rank10sel::<8, 12>(&mut criterion, uniform),
                        9 => bench_select::bench_rank10sel::<9, 12>(&mut criterion, uniform),
                        10 => bench_select::bench_rank10sel::<10, 12>(&mut criterion, uniform),
                        _ => {
                            eprintln!("Invalid log2_upper_block_size: {}", log2_upper_block_size);
                            eprintln!("Valid values: 8, 9, 10");
                            exit(1);
                        }
                    },
                    13 => match log2_upper_block_size {
                        8 => bench_select::bench_rank10sel::<8, 13>(&mut criterion, uniform),
                        9 => bench_select::bench_rank10sel::<9, 13>(&mut criterion, uniform),
                        10 => bench_select::bench_rank10sel::<10, 13>(&mut criterion, uniform),
                        _ => {
                            eprintln!("Invalid log2_upper_block_size: {}", log2_upper_block_size);
                            eprintln!("Valid values: 8, 9, 10");
                            exit(1);
                        }
                    },
                    _ => {
                        eprintln!(
                            "Invalid log2_ones_per_inventory: {}",
                            log2_ones_per_inventory
                        );
                        eprintln!("Valid values: 10, 11, 12, 13");
                        exit(1);
                    }
                }
            }
        }
    }
}
