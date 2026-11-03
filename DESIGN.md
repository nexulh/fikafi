# Swedish Retirement Monte Carlo Engine: Architecture & Design Rationale

## Nailed-Down Constraints & Fixed Decisions

* **Separate `assets` and `liabilities` Vectors:** `Portfolio` explicitly maintains `pub assets: Vec<Asset>` and `pub liabilities: Vec<Liability>`. Assets and liabilities have inverted operational lifecycles; forcing them under a single trait/umbrella introduced friction without benefit. Net worth remains $TotalAssets - TotalLiabilities$.

* **Dedicated `liquid_cash` Clearinghouse:** `liquid_cash` lives directly on `Portfolio` as a first-class field. All operational inflows (salary, pensions, liquidations) and outflows (living expenses, debt service, deposits) pass through `liquid_cash`. Long-term capital growth is completely isolated from daily liquidity management.

* **Singular Module Naming:** Modules are strictly named in the singular (`asset.rs`, `liability.rs`) per Rust API guidelines and namespace ergonomics (`asset::Asset` vs `assets::Asset`).

* **Explicit Multi-Phase Chronological Pipeline:** The monthly simulation loop is managed by the orchestrator in `simulator.rs` using a multi-phase sequential strategy execution pipeline. `Portfolio` acts as an anemic state container while external strategies generate atomic, ordered `FinancialPlan` actions.

  $$
  \text{Phase 1: Inflows \& Expenses} \longrightarrow \text{Phase 2: Deficit Liquidation} \longrightarrow \text{Phase 3: Surplus Sweeping} \longrightarrow \text{Phase 4: Market Step}
  $$

* **`step_month` Runs LAST:** Market returns and quarter-end snapshots execute at the *end* of the monthly loop iteration (Phase 4). This ensures all monthly cash flows participate in that month's compounding and quarter-end ISK snapshots catch fully settled balances.


* **Mid-Year Startup via Pre-Filled Quarters:** When instantiating an ISK mid-year, all four quarter slots in `quarter_values` are pre-filled with the starting balance (`[starting_balance; 4]`). Elapsed quarters retain this baseline proxy, while future quarter-ends during the simulation naturally overwrite their respective slots.

* **Direct Semantic Asset Tagging (`AssetRole`):** Individual asset structs (e.g., `IskAccount`) contain an explicit `role: AssetRole` field (e.g., `CashBuffer`, `GrowthEquity`, `BondStabilizer`). External strategies filter and target portfolio assets via these semantic roles rather than relying on fragile vector indices or shadow ledgers.

## Decoupled Tax Architecture & Swedish *Skattekonto* Model

To reflect real-world Swedish tax law accurately and avoid tight coupling between asset classes and macro tax logic, the tax engine follows an explicit three-phase data flow:

$$
\text{1. Raw Tax Events } (\texttt{TaxBase}) \longrightarrow \text{2. Annual Ledger } (\texttt{TaxRecord}) \longrightarrow \text{3. Fiscal Output } (\texttt{TaxResult})
$$

### Data Flow Components

* **Event Emission (`TaxBase` & `TaxBaseType`):** Assets, liabilities, and income streams do not calculate tax. Instead, they emit lightweight `TaxBase` event records categorized by `TaxBaseType` (e.g., `IskCapitalBase`, `DeductibleInterest`, `RealizedCapitalGain`, `EarnedIncome`, `WithheldTax`).

* **Per-Person Annual Ledger (`TaxRecord`):** Each individual (`PersonId`) holds a dedicated `TaxRecord` that accumulates `TaxBase` events across the fiscal year.

* **Decoupled Engine Pass (`TaxResult`):** At Month 12 (year-end), the central `TaxEngine` ingests the `TaxRecord` alongside statutory macro constants (e.g., *statslåneränta*, municipal tax rates, *grundavdrag*, and *ränteavdrag* thresholds) to calculate a final `TaxResult`.

* **Reconciliation against *Skattekonto*:** The net variance from the `TaxResult` (total statutory tax liability minus preliminary tax paid) is settled against each individual's *Skattekonto*, resulting in a single net refund or tax payment cleared through `portfolio.liquid_cash`.

### Individual Taxation (*Särbeskattning*) & Household Scalability

* Income taxes, *grundavdrag*, *statlig inkomstskatt*, pension allocations, and *ränteavdrag* caps (30% reduction up to 100,000 SEK per person) are evaluated strictly per individual (`PersonId`).

* Assets and liabilities carry an `Ownership` tag (`Sole(PersonId)` or `Joint(Vec<(PersonId, Decimal)>)`), allowing shared obligations (like mortgages) to cleanly split their emitted `TaxBase` events across individual `TaxRecord` ledgers before reconciliation.

## CQRS-Lite Strategy Architecture & Shared Observability

Rather than mutating state directly or hardcoding cash routing inside `Portfolio`, operations are driven by external, stateful strategy modules that emit atomic action plans based on read-only portfolio evaluations.

### Plan-Emission Pattern (`FinancialPlan`)
Strategies receive a read-only `EvaluationContext` and generate an ordered list of atomic commands:
* **`EvaluationContext`:** Exposes `Portfolio`, `MarketIndicator`, and household demographic data.
* **`FinancialPlan` (list of `PlanAction` items):** Expresses concrete intents such as `Withdraw { from_asset_id, amount }`, `Deposit { to_asset_id, amount }`, Transfer { from_asset_id, to_asset_id, amount }, or `PayLiability { LiabilityId, amount }`.
* **Plan implementation:** The orchestrator sends the plans to the `Portfolio` (which implements it), preventing state races while allowing strategies to stay pure and read-only during evaluation.

### Centralized Observability (`MarketIndicator`)
To prevent "strategy schizophrenia" (e.g., an expense strategy selling defensively during a "Bear market" while an investment strategy buys aggressively during it's notion of a "Bull market"), all strategies observe a shared `MarketIndicator`.
* **Role:** Acts as a centralized, objective history tracker updated at the start of each month.
* **Observable Facts:** Calculates High-Water Marks (ATH), current percentage drawdowns, drawdown duration, and multi-temporal metrics (trailing 12-month returns, previous calendar year returns).
* **Isolation:** The indicator is maintained by the simulator orchestrator in `EvaluationContext` and does not pollute the `Portfolio` struct. Explicitly distinct from the `MarketSampler` to prevent "insider information". 

## Rejected Alternatives & Discarded Approaches

* **Asset-Level Tax Self-Calculation:**
  * *Discarded:* Having individual `Asset` and `Liability` implementations compute their own tax liabilities internally.
  * *Reason:* Required asset classes to hold references to macro rates (*statslåneränta*), personal income brackets, and other accounts' states, causing tangled dependencies and unscalable code.

* **Unified "Account" Umbrella for Assets and Debt:**
  * *Discarded:* Trying to make `Liability` implement an asset-centric `AccountOps` trait or putting debt into an `Account` enum.
  * *Reason:* Liabilities carry mandatory amortization (*amorteringskrav*) and interest expense logic; assets carry yield and tax-wrapper rules. Combining them required awkward default trait implementations.

* **Banking-Centric Naming (`Account` vs `LiabilityAccount`):**
  * *Discarded:* Keeping `Account` for both assets and liabilities, and naming debt `LiabilityAccount`.
  * *Reason:* Asymmetric and clumsy. Renaming the asset enum to `Asset` established textbook balance-sheet symmetry (`Asset` / `Liability`).

* **Monolithic "God Method" `step_month` or State-Heavy `Portfolio`:**
  * *Discarded:* Having `Portfolio` or `step_month` internally make decisions about liquidation waterfalls, bucket refilling, or market-driven withdrawals.
  * *Reason:* Re-creates the monolithic class anti-pattern. `Portfolio` should remain an anemic state container with primitive safety methods, while strategic logic lives in external modules.

* **Direct Mutable Access in Strategies:**
  * *Discarded:* Passing `&mut Portfolio` directly into strategy traits.
  * *Reason:* Created complex borrow checker constraints, made strategies hard to unit test, and allowed unconstrained state mutations out of chronological order.

* **Independent Strategy Market Tracking:**
  * *Discarded:* Letting each strategy maintain its own local heuristics for detecting bull/bear markets.
  * *Reason:* Led to conflicting, contradictory strategy behaviors during market shifts.

* **Embedding Cash inside `Portfolio::Vec<Asset>`:**
  * *Discarded:* Treating cash as just another asset variant.
  * *Reason:* Required constant array searching/filtering during monthly cash-flow execution and complicated surplus/deficit routing.

## Active Explorations & Open Architecture Vectors

* **Stochastic Macro Generator:** Designing a correlated multi-variable engine generating monthly equity returns, inflation indices, *statslåneränta*, and mortgage interest rates (`&MacroStep`). `MarketSampler` currently implements a statistical model of monthly market returns; but inflation, *statslåneränta* and mortgage interest rates depending on the market are still work in progress. 

* **Liability Servicing Engine:** Modeling specific Swedish mortgage rules inside  the `liability` module (calculating LTV tiers, income-to-loan ratios, and mandatory *amorteringskrav* amortization).

* **`FinancialPlan` Execution Engine:** Implementing the transaction processor inside `Portfolio` that validates and applies `FinancialPlan` items emitted by strategies.

## Domain Terminology & Core Mental Model

* **Wrapper vs. Asset:** In Swedish retirement planning, the container (e.g., ISK, *Aktiedepå*, *Tjänstepension*) dictates tax rules and liquidity, while the internal balance represents capital. The model treats wrapper accounts as discrete `Asset` variants.

* **`TaxBase` & `TaxBaseType`:** A lightweight reporting struct emitted by assets, liabilities, and income streams via `tax_base(&mut self) -> Option<TaxBase>`.

* **`TaxRecord` & `TaxResult`:** Per-person ledgers that aggregate raw `TaxBase` events throughout the fiscal year and store calculated tax breakdown outputs after the `TaxEngine` pass.

* **`FinancialPlan` & `PlanAction`:** The atomic units emitted by external strategies during read-only evaluation and executed sequentially by the `Portfolio` as part of the simulation orchestration.
