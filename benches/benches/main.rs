use criterion::Criterion;

mod bench_rank;
mod bench_select;

// Select9 DOES NOT EXIST

pub fn main() {
    let mut criterion = Criterion::default()
        .without_plots()
        .configure_from_args()
        .with_filter("");

    let filter = std::env::args().nth(1).unwrap_or_default();

    match filter.as_str() {
        "rank9" => bench_rank::bench_rank9(&mut criterion),
        "rank10_256" => bench_rank::bench_rank10::<256>(&mut criterion),
        "rank10_512" => bench_rank::bench_rank10::<512>(&mut criterion),
        "rank10_1024" => bench_rank::bench_rank10::<1024>(&mut criterion),
        "rank11" => bench_rank::bench_rank11(&mut criterion),
        "rank12" => bench_rank::bench_rank12(&mut criterion),
        "rank16" => bench_rank::bench_rank16(&mut criterion),
        "poppy" => bench_rank::bench_poppy(&mut criterion),
        "simple_select" => bench_select::bench_simple_select(&mut criterion, true),
        "simple_select_non_uniform" => bench_select::bench_simple_select(&mut criterion, false),
        "rank9sel" => bench_select::bench_rank9sel(&mut criterion, true),
        "rank9sel_non_uniform" => bench_select::bench_rank9sel(&mut criterion, false),
        "rank10sel_1024_11" => bench_select::bench_rank10sel::<1024, 11>(&mut criterion, true),
        "rank10sel_1024_11_non_uniform" => {
            bench_select::bench_rank10sel::<1024, 11>(&mut criterion, false)
        }
        "cs_poppy" => bench_select::bench_cs_poppy(&mut criterion, true),
        "cs_poppy_non_uniform" => bench_select::bench_cs_poppy(&mut criterion, false),
        _ => {
            eprintln!("Unknown benchmark: {}", filter);
        }
    }
}
