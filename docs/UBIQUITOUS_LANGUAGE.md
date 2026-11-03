# UBIQUITOUS_LANGUAGE.md: Swedish Retirement & Personal Finance Domain

This document establishes the Ubiquitous Language for the retirement simulation domain. In Domain-Driven Design (DDD), this vocabulary serves as the single source of truth—bridging real-world Swedish financial rules, tax structures, and personal financial strategies into a rigorous, shared conceptual model.

## Core Balance Sheet & Liquidity

* **Portfolio:** The complete financial standing of an individual at any given point in time, defined as the net sum of all long-term assets minus liabilities, anchored by a daily operational cash buffer.

* **Asset (*Tillgång*):** A long-term wealth container or investment wrapper held for capital appreciation or future consumption. In the Swedish context, the container (*konto*) dictates the governing tax rules rather than individual assets inside it.

* **Liability (*Skuld*):** An outstanding financial obligation (such as a mortgage) that incurs interest and carries mandatory repayment rules. Liabilities are managed distinctly from assets to maintain a true balance-sheet model.

* **Cash Buffer (*Likvida medel* / *Likviditetskonto*):** The primary operational clearinghouse. All day-to-day financial inflows (salaries, pension disbursements) land here first, and all mandatory outflows (living expenses, debt servicing, tax settlements) are paid from it. It insulates long-term capital from short-term volatility.

* **Tax Account (*Skattekonto*):** The dedicated ledger managed by the Swedish Tax Agency (*Skatteverket*) for an individual. It tracks all tax-related transactions throughout the year—receiving preliminary tax withholdings (*preliminärskatt*) and ISK quarterly tax accruals, while serving as the settlement point for final tax assessments (*slutlig skatt*), residual tax payments (*kvarskatt*), or tax refunds (*skatteåterbäring*).

* **Net Worth (*Nettoförmögenhet*):** The absolute economic value of the portfolio, calculated as Total Assets minus Total Liabilities.


## Swedish Account Wrappers & Investment Vehicles

* **ISK (*Investeringssparkonto*):** A tax-favored investment account wrapper in Sweden. Instead of being taxed on realized capital gains, it is subject to an annual flat-rate wealth tax (*schablonskatt*) calculated from quarterly balances and annual deposits.

* **Standard Depot (*Aktiedepå*):** A traditional taxable brokerage account where capital gains and losses are realized and taxed upon the sale of securities (typically at a 30% capital gains tax rate).

* **Public Pension (*Allmän pension*):** The state-managed statutory pension system comprising **Income Pension (*Inkomstpension*)** and **Premium Pension (*Premiepension*)**. During the accumulation phase, it functions as a tax-deferred entitlement.

* **Occupational Pension (*Tjänstepension*):** The workplace-based pension tier provided by employers. It operates as a tax-deferred asset container during working years and converts into gross earned income upon disbursement.

* **Collective Agreement Pension (*Kollektivavtalad tjänstepension*):** Standardized occupational schemes (such as ITP1 or ITP2) where employer contributions are governed by salary tiers (e.g., 4.5% up to 7.5 IBB, and 30% on salary above 7.5 IBB).


## Tax & Fiscal Domain Concepts

* **Tax Base (*Skatteunderlag* / *Schablonunderlag*):** The specific monetary base upon which a tax rate is applied. For an ISK, this is the average of the four quarterly balance snapshots plus total annual deposits.

* **Earned Income (*Tjänsteinkomst*):** Income derived from labor or pension disbursements, subject to progressive municipal income taxes (*kommunalskatt*) and statutory deductions.

* **Basic Allowance (*Grundavdrag* / *Förhöjt grundavdrag*):** A statutory tax deduction applied to earned income, which increases automatically upon reaching statutory tax age (*riktålder*), lowering overall tax liability.

* **State Income Tax Threshold (*Skiktgräns för statlig skatt*):** The annual earned income limit above which a 20% national tax (*statlig inkomstskatt*) is levied on top of municipal tax.

* **Interest Deduction (*Ränteavdrag*):** The Swedish tax reduction system allowing individuals to deduct a percentage (historically 30%) of mortgage interest payments from their final tax burden.

* **Preliminary Tax (*Preliminärskatt*):** Tax withheld at source on monthly income streams (salaries and pensions) and credited directly to the individual's *skattekonto* to continuously cover estimated tax obligations.

* **Year-End Tax Reconciliation (*Slutlig skatt* / *Taxering*):** The formal fiscal settlement at year-end where total accrued liabilities on the *skattekonto* (earned income taxes, ISK *schablonskatt*, capital gains) are offset against credits (preliminary taxes paid, *ränteavdrag* reductions). Any net deficit results in residual tax debt (*kvarskatt*) payable from `cash`, while a surplus yields a tax refund (*skatteåterbäring*) credited back to `cash`.

* **Government Borrowing Rate (*Statslåneränta* / SLR):** The benchmark interest rate established on November 30 each year, which directly determines the ISK flat-rate tax rate ($\text{SLR} + 1\%$).



## Statutory Indexation & Base Units

* **Price Base Amount (*Prisbasbelopp* / PBB):** The annual inflation-indexed benchmark unit used to determine social benefits, tax deductions, and occupational pension tiers (e.g., ITP salary brackets).

* **Income Base Amount (*Inkomstbasbelopp* / IBB):** The annual wage-indexed unit governing public pension contributions. Earned pension credits (*pensionsrätt*) are capped at an income threshold of 8.07 IBB.

* **Target Pension Age (*Riktålder*):** The statutory, dynamically adjusting retirement age defined by Swedish law that controls eligibility for public pension payouts and the enhanced basic deduction (*förhöjt grundavdrag*).



## Debt & Servicing Mechanics

* **Amortization Requirement (*Amorteringskrav*):** Statutory Swedish rules mandating periodic principal paydowns on mortgages based on the property's Loan-to-Value (LTV) ratio and the borrower's debt-to-income ratio.

* **Debt Servicing (*Låneutgifter*):** The combined monthly outflow required to maintain a liability, consisting of mandatory interest payments and principal amortization.



## Lifecycle Phases & Strategic Dynamics

* **Accumulation Phase (*Sparfasen*):** The working years characterized by active income, external pension contributions, continuous savings, and portfolio growth.

* **Drawdown Phase (*Uttagsfasen*):** The retirement phase where earned income ceases and living expenses are funded via portfolio liquidations, ISK withdrawals, and pension disbursements.

* **Surplus:** The state in which monthly operational cash inflows exceed baseline living expenses and debt servicing, prompting capital allocation strategies.

* **Deficit:** The state in which operational cash inflows are insufficient to cover mandatory expenses or liabilities, triggering liquidity drawdown and asset liquidation strategies.


## Individual Taxation

* **Individual Taxation (*Särbeskattning*):** The Swedish legal principle where income, tax deductions (*grundavdrag*), pension rights, and tax thresholds (*skiktgräns*) are calculated strictly per individual person, regardless of marital or household status.

* **Tax Record (*Deklarationsunderlag*):** The individual tax ledger for a specific person that accumulates earned income, preliminary taxes withheld, ISK tax bases, and deductible interest throughout the fiscal year.

* **Ownership Share (*Ägarandel*):** The explicit percentage distribution of an asset or liability assigned to one or more individuals for tax attribution and drawdown rights.