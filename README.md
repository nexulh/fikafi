# FikaFi

FikaFi is a personal finance planning project for exploring long-term financial outcomes in a Swedish context.

Its goal is to make it easier to reason about how income, pensions, investments, debt, living costs, taxes, inflation, and market uncertainty interact over a lifetime. Rather than producing a single deterministic forecast, FikaFi is intended to support Monte Carlo simulations that show a range of possible outcomes and the trade-offs behind different financial decisions.

The project is designed around the realities of Swedish personal finance, including:

- Pension income and retirement drawdown
- Taxable investment accounts (*capital gains*) and ISK accounts (*schablonbeskattning*), and their fees and tax implications
- Mortgage interest and interest deductions
- Annual reconciliation of taxes incurred by annual Income/Assets/Liabilities
- Inflation, interest rates, and changing market conditions

FikaFi aims to keep financial mechanics transparent and composable. Cash flow, asset allocation, debt servicing, withdrawal priorities, and investment decisions are treated as separate concerns so that different planning approaches can be compared clearly.

## Status

FikaFi is under active development. The current focus is on establishing a reliable financial domain model and simulation foundation before adding a complete end-user planning workflow.

For the architectural direction and design decisions, see [docs/DESIGN.md](docs/DESIGN.md).
