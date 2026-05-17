# MasterFormat is one of many classification systems

Common confusion: people say "MasterFormat" when they mean "spec." MasterFormat is the *classification* — the codebook that organises specs into divisions. NBS Chorus + Avitru SpecLink author the actual *content* against that classification (or its UK equivalent, Uniclass).

## Classifications by region

| System | Source | Region |
|---|---|---|
| **CSI MasterFormat** | Construction Specifications Institute | US, Canada |
| **Uniclass** | NBS | UK |
| **OmniClass** | OmniClass Construction Classification System | US (broader scope — includes processes, services, not just products) |
| **CAWS** | Common Arrangement of Work Sections | UK (legacy, mostly superseded by Uniclass) |
| **OCCS** | Open Construction Classification System | International (BSI/CSI joint) |

This agent normalises across all of them — `translate` is the cross-walk verb.

## Cross-walk confidence

Translation between systems is **lossy**. Some MasterFormat sections have:

- Direct 1:1 mapping to Uniclass (confidence ~1.0): e.g. `03 30 00` (Cast-in-Place Concrete) ↔ `Pr_20_70_15_15`
- Many-to-one collapse (confidence ~0.7): e.g. several `09 65 xx` (resilient flooring) sections collapse to one Uniclass `Ss_30_40_75_60`
- No equivalent (confidence ~0.3): rare — happens for region-specific items like US-specific stormwater management

The transport returns the best match with explicit confidence. App authors should check confidence before trusting the translation for contractually-binding documents.

## What this agent does NOT do

- It does **not** author specs (use `nbs-chorus` for UK, `avitru-speclink` for US)
- It does **not** validate spec content (use `solibri` for BEP-level checking, including the project-classification check)
- It does **not** create custom classifications — only the 5 standard ones above

## Where the data comes from

The agent embeds the published CSI MasterFormat 2020 codebook + NBS Uniclass 2024 tables + the OmniClass + CAWS + OCCS public releases. Single Rust binary, ~3 MB self-contained. When CSI / NBS release updates, the agent ships a new version (no live API dependency).
