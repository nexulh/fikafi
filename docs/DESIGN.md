# Swedish Retirement Monte Carlo Engine: Architecture & Design Rationale

## 1. Nailed-Down Constraints & Fixed Decisions

* **Separate `assets` and `liabilities` Vectors:** `Portfolio` explicitly maintains `pub assets: Vec<Asset>` and `pub liabilities: Vec<Liability>`. Assets and liabilities have inverted operational lifecycles; forcing them under a single trait/umbrella introduced friction without benefit. Net worth remains $TotalAssets - TotalLiabilities$.
* **Dedicated `cash` Clearinghouse:** `cash: Currency` lives directly on `Portfolio` as a first-class field. All operational inflows (salary, pensions, liquidations) and outflows (living expenses, debt service, deposits) pass through `cash`. Long-term capital growth is completely isolated from daily liquidity management.
* **Singular Module Naming:** Modules are strictly named in the singular (`asset.rs`, `liability.rs`) per Rust API guidelines and namespace ergonomics (`asset::Asset` vs `assets::Asset`).
* **Explicit Chronological Pipeline:** The monthly simulation loop is an explicit pipeline in the simulator orchestrator, not hidden inside heavy account methods:
  $$\text{Inflows} \longrightarrow \text{Debt Service \& Expenses} \longrightarrow \text{Strategy Execution (Surplus/Deficit)} \longrightarrow \text{Step Market Returns}$$
* **`step_month` Runs LAST:** Market returns and quarter-end snapshots execute at the *end* of the monthly loop iteration. This ensures all monthly cash flows participate in that month's compounding and quarter-end ISK snapshots catch fully settled balances.
* **Encapsulated ISK Tax Tracking:** `IskAccount` tracks its own 4 quarterly snapshots (`[Currency; 4]`) and annual deposits (`Currency`).
* **Mid-Year Startup via Pre-Filled Quarters:** When instantiating an ISK mid-year, all four quarter slots in `quarter_values` are pre-filled with the starting balance (`[starting_balance; 4]`). Elapsed quarters retain this baseline proxy, while future quarter-ends during the simulation naturally overwrite their respective slots.

---

## 2. Swedish Tax Reconciliation & Cash Flow Model

* **Preliminary Monthly Deductions vs. Year-End Settlement:** 
  * Inflows (salaries, pension disbursements) are processed monthly with **preliminary tax withheld** at source, mirroring real-world Swedish tax mechanics.
  * At Month 12 (December year-end), the `TaxEngine` aggregates all `TaxBase` items emitted across the portfolio (ISK *schablonunderlag*, realized gains, mortgage *ränteavdrag*) alongside total annual income taxes paid.
  * The net tax variance (tax debt or refund/credit) is settled in `portfolio.cash` as a true fiscal year-end reconciliation.
* **Capital Tax vs. Income Tax Distinction:**
  * **`tax_base()`** handles **wrapper & debt taxation** (ISK *schablonskatt*, mortgage interest deductions).
  * **Income logic** handles **earned/pension income** (*kommunalskatt*, *statlig skatt*, *grundavdrag* / *förhöjt grundavdrag*).

---

## 3. Pluggable Strategy Architecture (Extensibility Framework)

Rather than hardcoding cash routing and liquidation rules into the simulation loop, operations will be driven by **Pluggable Strategy Patterns**:

* **`ExpenseStrategy` / Liquidation Priority:** Defines how deficits in `portfolio.cash` are covered. `cash` is always drawn down first as the primary clearing buffer, followed by a configurable asset liquidation chain (e.g., Cash $\rightarrow$ Savings $\rightarrow$ ISK $\rightarrow$ Taxable Depot $\rightarrow$ Pension Drawdown).
* **`InvestmentStrategy` / Surplus Sweeping:** Defines how month-end cash surplus exceeding an emergency buffer is allocated across long-term asset wrappers.
* **`AssetAllocationStrategy` / Rebalancing:** Handles periodic portfolio rebalancing across macro asset classes.

---

## 4. Rejected Alternatives & Discarded Approaches

* **Unified "Account" Umbrella for Assets and Debt:**
  * *Discarded:* Trying to make `Liability` implement an asset-centric `AccountOps` trait or putting debt into an `Account` enum.
  * *Reason:* Liabilities carry mandatory amortization (*amorteringskrav*) and interest expense logic; assets carry yield and tax-wrapper rules. Combining them required awkward default trait implementations.
* **Banking-Centric Naming (`Account` vs `LiabilityAccount`):**
  * *Discarded:* Keeping `Account` for assets and naming debt `LiabilityAccount`.
  * *Reason:* Asymmetric and clumsy. Renaming the asset enum to `Asset` established textbook balance-sheet symmetry (`Asset` / `Liability`).
* **Monolithic "God Method" `step_month`:**
  * *Discarded:* Having `step_month` internally trigger payouts, fee deductions, cash routing, and tax checks.
  * *Reason:* Caused implicit ordering bugs, bloated method signatures, and obscured exact event chronology.
* **Embedding Cash inside `Vec<Asset>`:**
  * *Discarded:* Treating cash as just another asset variant.
  * *Reason:* Required constant array searching/filtering during monthly cash-flow execution and complicated surplus/deficit routing.
* **Asset-Level Pension Tax Calculation:**
  * *Discarded:* Forcing pension variants (*Inkomstpension*, *Premiepension*, *Tjänstepension*) to return a capital tax base via `tax_base()`.
  * *Reason:* Pension accounts during accumulation are tax-deferred containers with zero annual capital tax base. During drawdown, payouts are gross earned income (*tjänsteinkomst*), which belong in the income tax engine.

---

## 5. Active Explorations & Open Architecture Vectors

1. **Stochastic Macro Generator:** Designing the correlated multi-variable engine generating monthly equity returns, inflation indices, *statslåneränta*, and mortgage interest rates (`&MacroStep`).
2. **Liability Servicing Engine:** Modeling specific Swedish mortgage rules inside `liability.rs` (calculating LTV tiers, income-to-loan ratios, and mandatory *amorteringskrav* deductions).
3. **Trait / Interface Definitions for Pluggable Strategies:** Defining the Rust trait signatures for `ExpenseStrategy` and `InvestmentStrategy` so they can manipulate `Portfolio` balances cleanly without violating borrowing rules.

---

## 6. Domain Terminology & Core Mental Model

* **Wrapper vs. Asset:** In Swedish retirement planning, the container (e.g., ISK, *Aktiedepå*, *Tjänstepension*) dictates tax rules and liquidity, while the internal balance represents capital. The model treats wrapper accounts as discrete `Asset` variants.
* **`TaxBase` & `TaxBaseType`:** A lightweight reporting struct emitted by assets and liabilities via `tax_base(&mut self) -> Option<TaxBase>`:
  * `TaxBaseType::IskCapitalBase`: Raw *schablonunderlag* ($\frac{\sum Q}{4} + \text{Deposits}$).
  * `TaxBaseType::DeductibleInterest`: Mortgage interest paid during the fiscal year for Swedish *ränteavdrag*.
  * `TaxBaseType::RealizedCapitalGain`: Capital gains/losses from standard taxable depots.