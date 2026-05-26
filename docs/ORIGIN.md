# Origin

We already had a strong clause-graph surface for LegalTech buyers, but the same primitive was at risk of getting duplicated when the language atlas moved into Rust.

The right split was not “another clause dashboard in another language.” The right split was:

- clause graph and ownership visibility in TypeScript
- append-only obligation ledger and evidence durability in Rust

That makes the Rust repo a real systems complement instead of a portfolio clone.
