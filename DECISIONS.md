# FikaFi: Design Rationale & Architectural Decision Records (ADR)

This document records the design rationale, evaluated trade-offs, discarded technical paths, and active exploratory vectors underlying FikaFi's system design. For the primary system topology and domain contracts, see [`ARCHITECTURE.md`](ARCHITECTURE.md).

---

## 1. Key Design Rationale & Trade-offs

### Decoupled Tax Engine vs. Account Self-Calculation
* **Decision:** Tax calculations are extracted into an independent `TaxEngine` pass driven by an annual `TaxRecord` rather than evaluated directly inside individual `Asset` or `Liability` structs.
* **Rationale:** Real-world Swedish tax law (*Skatteverket*) evaluates income tax brackets, interest deduction caps (*ränteavdrag*), and statutory basic deductions (*grundavdrag*) at the individual (*PersonId*) level across all financial activities. Having account instances compute their own tax liabilities would require passing global macro rates, income brackets, and cross-account balances into primitive asset types, causing tight coupling and fragile circular dependencies.

### CQRS-Lite Strategy Architecture
* **Decision:** Strategies operate as read-only evaluators emitting atomic `FinancialPlan` action commands instead of receiving mutable `&mut Portfolio` references.
* **Rationale:** Direct mutable access inside strategy logic led to complex Rust borrow-checker conflicts, made strategies difficult to unit test in isolation, and risked out-of-order state mutations during multi-phase operations. Emitting plan actions enforces strict plan validation, explicit execution ordering, and seamless audit logging.

### Shared `MarketIndicator` vs. Independent Strategy Tracking
* **Decision:** All strategy modules observe a centralized `MarketIndicator` provided within `EvaluationContext`.
* **Rationale:** Allowing strategies to independently calculate market trends created "strategy schizophrenia"—where one module interpreted a market dip as a bear regime and sold defensively while another interpreted it as a temporary drawdown and bought aggressively. Centralizing historical market facts ensures coherent household-level strategy coordination.

---

## 2. Rejected Alternatives & Discarded Approaches

| Discarded Approach | Reason for Rejection | Better Alternative Adopted |
| :--- | :--- | :--- |
| **Unified "Account" Enum**<br>*(Combining Assets & Debt)* | Liabilities carry mandatory amortization (*amorteringskrav*) and interest expense logic; assets carry yield and tax-wrapper rules. Combining them required awkward default trait implementations. | Explicit balance-sheet split (`Vec<Asset>` and `Vec<Liability>`) on `Portfolio`. |
| **Monolithic `step_month` Orchestrator** | Embedding cash flow routing, liquidation waterfalls, and surplus sweeps inside `Portfolio` recreated a untestable "God Method" anti-pattern. | Anemic `Portfolio` container driven by priority-ordered external strategy modules. |
| **Embedding Cash in `Vec<Asset>`** | Treating liquid cash as an asset variant required constant array filtering and searching during routine cash flow routing. | Promoted `liquid_cash` to a first-class field acting as the primary clearinghouse. |
| **Banking Naming (`Account` vs `LiabilityAccount`)** | Asymmetric and clumsy naming convention. | Established textbook balance-sheet symmetry (`Asset` / `Liability`). |

---

## 3. Active Explorations & Open Design Vectors

* **Stochastic Macro Generator:** Designing a correlated multi-variable generator to model realistic joint distributions of equity returns, inflation indices, *statslåneränta*, and mortgage interest rates (`&MacroStep`). `MarketSampler` currently models equity market returns; expanding correlation models for macro interest rates remains work in progress.

* **Liability Servicing Engine:** Modeling detailed Swedish mortgage regulations inside the `liability` module, including dynamic LTV (Loan-to-Value) tiering, debt-to-income limits, and mandatory *amorteringskrav* amortization rules.

* **`FinancialPlan` Transaction Processor:** Implementing transaction validation and state application logic inside `Portfolio` to process `FinancialPlan` action items safely and detect invalid operations (e.g., overdrafts or illegal liquidations).
