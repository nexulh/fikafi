use rand::Rng;
use rand_distr::{Distribution, StudentT, Uniform};
use rust_decimal::Decimal;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarketRegime {
    Bull,
    Bear,
}

#[derive(Debug, Clone)]
pub struct MarketSampler {
    state: MarketRegime,
    // Transition probabilities per month
    p_bull_to_bear: f64,
    p_bear_to_bull: f64,
    // Bull Regime Parameters (Monthly)
    bull_mean: f64,
    bull_scale: f64,
    // Bear Regime Parameters (Monthly)
    bear_mean: f64,
    bear_scale: f64,

    bull_dist: StudentT<f64>,
    bear_dist: StudentT<f64>,
    uniform_dist: Uniform<f64>,
}

impl MarketSampler {
    /// Creates a calibrated sampler with realistic monthly defaults:
    /// - Bull: ~8% annualized geometric return, low volatility (~12% annual scale)
    /// - Bear: Sharp negative drift, high volatility (~30% annual scale, heavy tails)
    pub fn new() -> Self {
        let bull_df = 8.0;
        let bear_df = 4.0;

        let bull_target_annual_vol = 0.14;
        let bear_target_annual_vol = 0.24;

        let bull_monthly_vol = bull_target_annual_vol / (12.0f64).sqrt();
        let bear_monthly_vol = bear_target_annual_vol / (12.0f64).sqrt();

        let bull_scale = bull_monthly_vol * (bull_df / (bull_df - 2.0f64)).sqrt();
        let bear_scale = bear_monthly_vol * (bear_df / (bear_df - 2.0f64)).sqrt();

        #[allow(clippy::unwrap_used)]
        let uniform_dist = Uniform::new(0.0, 1.0).unwrap();
        #[allow(clippy::unwrap_used)]
        let bull_dist = StudentT::new(bull_df).unwrap();
        #[allow(clippy::unwrap_used)]
        let bear_dist = StudentT::new(bear_df).unwrap();

        Self {
            state: MarketRegime::Bull,
            // ~1.5% monthly chance to switch to bear (~5.5 year average bull duration)
            p_bull_to_bear: 0.01,
            // ~15% monthly chance to recover (~6.7 month average bear duration)
            p_bear_to_bull: 0.20,

            // Bull Parameters
            bull_mean: 0.0095, // ~8.9 annualized arithmetic drift in bull
            bull_scale,

            // Bear Parameters
            bear_mean: 0.0, // Mild drag during bear regime
            bear_scale,

            bull_dist,
            bear_dist,
            uniform_dist,
        }
    }

    /// Evaluates the Markov transition matrix and samples a monthly return percentage.
    pub fn sample_month(&mut self, rng: &mut impl Rng) -> Decimal {
        // 1. State Transition Check (Markov Chain)
        #[allow(clippy::unwrap_used)]
        let roll = self.uniform_dist.sample(rng);

        match self.state {
            MarketRegime::Bull => {
                if roll < self.p_bull_to_bear {
                    self.state = MarketRegime::Bear;
                }
            }
            MarketRegime::Bear => {
                if roll < self.p_bear_to_bull {
                    self.state = MarketRegime::Bull;
                }
            }
        }

        // 2. Select distribution parameters based on current active regime
        let (mean, scale, dist) = match self.state {
            MarketRegime::Bull => (self.bull_mean, self.bull_scale, self.bull_dist),
            MarketRegime::Bear => (self.bear_mean, self.bear_scale, self.bear_dist),
        };

        // 3. Draw from Student's t-distribution
        #[allow(clippy::unwrap_used)]
        let t_sample: f64 = dist.sample(rng);

        // 4. Scale and shift to match desired mean and volatility
        let raw_return = mean + scale * t_sample;

        // 5. Convert to high-precision Decimal
        Decimal::from_f64_retain(raw_return).unwrap_or(Decimal::ZERO)
    }

    #[allow(dead_code)]
    pub fn current_regime(&self) -> MarketRegime {
        self.state
    }
}
