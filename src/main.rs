use rayon::prelude::*;
use rust_decimal::Decimal;
use rust_decimal::MathematicalOps;
use rust_decimal::dec;

use fikafi::portfolio::Portfolio;

mod market_sampler;
use market_sampler::MarketSampler;

fn run_market_sampler_test(sim_runs: usize) -> Decimal {
    let sim_months = 12 * 30; // 30 years of monthly returns
    let sampler = MarketSampler::new();

    let cagr_values: Vec<Decimal> = (0..sim_runs)
        .into_par_iter()
        .map(|_| {
            let mut rng = rand::rng();

            // Initialize variables for this simulation run
            let mut sampler = sampler.clone();
            let mut total_return = dec!(1);

            for _ in 0..sim_months {
                let monthly_return = sampler.sample_month(&mut rng);
                total_return *= dec!(1) + monthly_return;
            }
            //let mean_cagr =
            //    ((total_return).powd(dec!(12) / Decimal::from(sim_months)) - dec!(1)) * dec!(100);
            //cagr_values.push(mean_cagr);
            ((total_return).powd(dec!(12) / Decimal::from(sim_months)) - dec!(1)) * dec!(100)
        })
        .collect();
    cagr_values.iter().sum::<Decimal>() / Decimal::from(cagr_values.len())
}

fn main() {
    println!("Starting portfolio calculation...");
    let portfolio = Portfolio {
        assets: vec![],
        liabilities: vec![],
        liquid_cash: dec!(100_000.0),
    };
    println!("{}", portfolio.total_nominal_value().round_dp(2));

    let sim_runs = 10_000;
    let start = std::time::Instant::now();
    let avg_cagr = run_market_sampler_test(sim_runs);
    let duration = start.elapsed();

    println!("Average CAGR: {avg_cagr:.3}%");
    println!("Simulation duration: {duration:?}");
}
