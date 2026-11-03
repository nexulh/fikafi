# FikaFi: System Architecture & Architectural Mental Model

## 1. High-Level Architecture & Domain Layering

FikaFi is built around a clean separation between **anemic state containers** and **decoupled execution engines**. The balance sheet, household demographic data, and macro indicators represent pure state, while financial planning, tax reconciliation, and simulation steps are driven by external execution modules.

```
                           +------------------------+
                           |   MarketSampler /      |
                           |   Macro Engine         |
                           +-----------+------------+
                                       |
                                       v
                               MonthlyMarketContext
                                       |
  +-------------------+                v                +--------------------+
  | External          | ----> EvaluationContext ------> | Strategy Pipeline  |
  | Demographics      |               |                 | (Priority-Ordered) |
  | (Household/Person)|               |                 +---------+----------+
  +-------------------+               v                           |
                               +--------------+                   v
                               |  Portfolio   | <------- FinancialPlan (Actions)
                               | - Assets     |
                               | - Liabilities|
                               | - liquid_cash|
                               +--------------+
                                       |
                                       v (Month 12)
                               +--------------+
                               |  Tax Engine  |
                               +--------------+
```

### Module Responsibilities & Ergonomics

* **Anemic Domain State (`portfolio.rs`, `asset.rs`, `liability.rs`, `household.rs`):**

  * `Portfolio`, `Asset`, `Liability`, and `Person` are plain data containers enforcing basic structural invariants (e.g., non-negative balances, valid dates, ownership shares).

  * Polymorphic dispatch across account types (e.g., `IskAccount`, `TjanstepensionAccount`) is achieved via the `enum_dispatch` crate over the `AssetOperations` and `LiabilityOperations` traits.

  * Modules follow Rust API guidelines using singular naming (`asset.rs` containing `Asset`, `liability.rs` containing `Liability`).

* **Explicit Assets & Liabilities Separation:**

  * `Portfolio` explicitly maintains `assets: Vec<Asset>` and `liabilities: Vec<Liability>` as distinct collections. Assets and liabilities have inverted operational lifecycles (capital appreciation vs. mandatory debt servicing/amortization), preserving a true balance-sheet structure. Net worth is strictly calculated as $\text{Total Asset Value} - \text{Total Liabilities Principal} + \text{liquid\_cash}$.

* **Operational Clearinghouse (`liquid_cash`):**

  * `Portfolio::liquid_cash` serves as the primary operational clearinghouse.

  * Operational cash inflows (income, pension disbursements) land directly in `liquid_cash`, while outflows (living expenses, debt servicing, tax settlements, investment sweeps) clear from it. Long-term capital growth is completely isolated from daily liquidity management.

## 2. Priority-Ordered Strategy & Cash Flow Pipeline

Rather than relying on a rigid, hardcoded chronological loop, monthly operations are governed by a **Priority-Ordered Strategy Pipeline**. This allows modeling complex financial behaviors such as "Pay Yourself First" investment strategies alongside baseline living expenses.

### Strategy Evaluation & Plan Emission (CQRS-Lite)

1. **Read-Only Inspection:** Strategies inspect the simulation state via a read-only `EvaluationContext` (exposing `Portfolio`, `MarketIndicator`, and household demographic details).

2. **Atomic Intent (`FinancialPlan`):** Strategies generate an ordered list of atomic `PlanAction` items (e.g., `Withdraw`, `Deposit`, `Transfer`, `PayLiability`).

3. **Incremental Execution:** Strategies run in order of defined priority (e.g., `Priority 1: Mandatory Expenses`, `Priority 2: Pay Yourself First Investments`, `Priority 3: Secondary Buffer Sweeps`). After each strategy emits a plan, the orchestrator validates and applies the plan against `Portfolio` immediately before evaluating the next strategy in line.

### Multi-Tiered Liquidity Cascades & `AssetRole` Tagging

Individual asset structs contain an explicit semantic role tag (`role: AssetRole`, e.g., `CashBuffer`, `GrowthEquity`, `BondStabilizer`). External strategies use these semantic tags to filter and target portfolio assets directly without relying on fragile array indices or shadow ledgers.

When `liquid_cash` is insufficient to satisfy an operational requirement, the liquidity cascade evaluates these semantic roles:

* **Tier 1:** Primary clearinghouse (`liquid_cash`).

* **Tier 2:** Secondary cash buffers (`AssetRole::CashBuffer`, e.g., high-yield savings or money market funds).

* **Tier 3:** Long-term capital growth investments (`AssetRole::GrowthEquity`).

## 3. Earned Income, Pension Mechanics & `TaxCalculator` Service

To isolate the `Portfolio` from payroll and statutory tax complexity, an external `TaxCalculator` service processes gross taxable earned income (*Tjänsteinkomst*)—encompassing both employment salary during accumulation and pension disbursements during drawdown. It acts as an automated pipeline translating gross income into net operational cash flows and statutory allocations.

```
 Gross Earned Income / Pension Disbursement (Tjänsteinkomst)
                            |
                            v
                  +--------------------+
                  |   TaxCalculator    |
                  |      Service       |
                  +---------+----------+
                            |
                            +---> Preliminary Tax (Withheld) ------> Emits TaxBase (WithheldTax)
                            |
                            +---> Net Operational Income ---------> Portfolio.liquid_cash
                            |     (Nettoinkomst)
                            |
                            +---> Public Pension (18.5%) ----------> Assets: Inkomstpension (16%)
                            |     (Labor/Salary ONLY)                      Premiepension (2.5%)
                            |
                            +---> Occupational Pension ------------> Assets: Tjänstepension
                                  (Labor/Salary ONLY; ITP Tiers)
```

### Statutory Swedish Income & Pension Handling

1. **Earned Income Alignment (*Tjänsteinkomst*):**

   * Employment salary and taxable pension disbursements (Inkomstpension, Premiepension, Tjänstepension) are both classified as *Tjänsteinkomst* for income taxation.

   * The `TaxCalculator` differentiates between labor income and pension income when applying statutory deductions (such as standard *Grundavdrag* vs. *Förhöjt grundavdrag* for individuals meeting the statutory target age / *riktålder*).

2. **PGI Distinction (Pension-Earning Income / *Pensionsgrundande Inkomst*):**

   * **Labor/Salary Income = PGI:** Only active employment earnings (salaried income) qualify as *PGI*, triggering automated statutory contributions to state pensions (*Allmän pension*) and employer occupational pensions (*Tjänstepension*).

   * **Pension Disbursements = Non-PGI:** Pension payouts during retirement are taxable *Tjänsteinkomst*, but strictly **do not** generate *PGI*. Consequently, pension disbursements incur zero ongoing contributions to *Inkomstpension*, *Premiepension*, or *Tjänstepension*.

3. **Public & Occupational Pension Contributions (Accumulation Phase):**

   * **Public Pension (*Allmän pension*):** Derived from gross salary up to $8.07 \times \text{IBB}$, automatically partitioned into `InkomstpensionAccount` ($16\%$) and `PremiepensionAccount` ($2.5\%$).

   * **Occupational Pension (*Tjänstepension*):** Routed to `TjanstepensionAccount` based on collective agreement brackets (e.g., ITP1/ITP2 tiers).

4. **Net Cash & Preliminary Tax Withholding:**

   * Preliminary taxes (*preliminärskatt*) are withheld at source on gross income streams and recorded in the individual's `TaxRecord`.

   * The remaining **net operational income** (*nettoinkomst*) lands in `Portfolio::liquid_cash`.

## 4. Tax Engine & Annual Reconciliation (*Särbeskattning*)

Tax calculations adhere strictly to Swedish tax law (*Skatteverket* standards), which mandates individual taxation (*särbeskattning*) and annual fiscal reconciliation.

### Data Flow & Tracking

$$
\text{Monthly Asset/Liability Tracking } (\texttt{TaxBase}) \longrightarrow \text{Month 12 Engine Pass } (\texttt{TaxRecord}) \longrightarrow \text{Net Settlement } (\texttt{TaxResult})
$$

1. **Internal Monthly Base Tracking:** Individual assets (e.g., ISK quarterly balances) and liabilities (deductible mortgage interest) internally track and compute their tax bases across monthly time steps. No intermediate tax records are emitted during months 1–11.

2. **Mid-Year Startup Proxy:** When instantiating an ISK account mid-year during simulation startup, missing prior quarter slots (`quarter_values`) are pre-filled with the starting balance (`[starting_balance; 4]`). Elapsed quarters retain this proxy, while subsequent quarter-ends naturally overwrite their respective slots.

3. **Month 12 Statutory Engine Pass:** At the end of the fiscal year (Month 12), the central `TaxEngine` ingests the accumulated `TaxBase` totals for each individual (`PersonId`), combining them with statutory macro parameters (*statslåneränta*, municipal tax rates, *grundavdrag*, and *ränteavdrag* thresholds).

4. **Joint Ownership & Tax Attribution (`Ownership`):** Assets and liabilities carry an explicit ownership tag (`Sole(PersonId)` or `Joint(Vec<(PersonId, Decimal)>)`). Emitted tax bases for shared obligations (like joint mortgages) are cleanly split according to registered ownership percentages prior to the annual engine evaluation.

5. **Skattekonto Clearing:** The net variance from the final `TaxResult` (total tax obligation minus preliminary taxes withheld) clears as a single net refund or payment transaction against `Portfolio::liquid_cash`.

## 5. Macro Environment & Observability

### Unified Macro Context (`MonthlyMarketContext`)

* Macroeconomic variables—including inflation, *statslåneränta* (government borrowing rate), *prisbasbelopp* (PBB), and *inkomstbasbelopp* (IBB)—are inferred directly from historical market returns inside the stochastic `MarketSampler`.

* Every monthly iteration packages these variables into an immutable `MonthlyMarketContext` injected into the `Portfolio` and its constituent assets/liabilities.

* The domain models (`Portfolio`, `Asset`, `Liability`) remain pure, passive containers with zero global mutable state.

### Centralized Observability (`MarketIndicator`)

* To prevent "strategy schizophrenia" (e.g., an expense strategy selling defensively during a market downturn while an investment strategy buys aggressively based on a conflicting heuristic), all strategies observe a shared `MarketIndicator`.

* Maintained by the simulation orchestrator within `EvaluationContext`, `MarketIndicator` tracks centralized market metrics:
  * High-Water Marks (ATH)
  * Current percentage drawdowns and drawdown durations
  * Multi-period trailing returns (12-month trailing, prior calendar year returns)

* `MarketIndicator` provides shared, objective historical facts without leaking stochastic future data from the `MarketSampler`.

## 6. Architectural Invariants

1. **Balance Sheet Integrity:** Total Portfolio Value is strictly defined as $\text{Total Asset Value} - \text{Total Liabilities Principal} + \text{liquid\_cash}$.

2. **Passive Domain Models:** Assets and liabilities never initiate self-directed cash transfers or direct mutations outside of their explicit `AssetOperations` / `LiabilityOperations` step contracts.

3. **Strict Individual Taxation:** Tax liabilities, basic deductions (*grundavdrag*), interest deduction caps (*ränteavdrag* 30% reduction up to 100,000 SEK), and progressive brackets (*statlig inkomstskatt*) are evaluated strictly per `PersonId`.

4. **Non-PGI Pension Invariant:** Pension payouts are non-PGI transactions and will never generate new contributions or deposits into public or occupational pension accounts.

5. **Deterministic Time-Stepping Order:** Monthly return compounding and quarter-end snapshots (`step_month`) execute at the end of the monthly loop iteration. This ensures all monthly cash flows participate in compounding, and quarter-end ISK snapshots capture fully settled balances.
