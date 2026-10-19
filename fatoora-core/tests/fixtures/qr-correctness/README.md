# QR comparison report

`report.json` was captured using `scripts/qr_parity.py` with SDK `238-R3.4.8` and the repository's public dummy signing credentials. It records six invoice/note types, eight decimal spellings, the dual-currency advance-payment sample and the equivalent-amount Tag 4 mutation. Input and source hashes identify the capture; randomized invoice signatures make encoded QR payloads run-specific.

The offline `qr_evidence` test regenerates deterministic fields and checks the input hashes. Raw XML and process evidence are produced by rerunning the capture command. Source hashes describe the capture and need not match unrelated future edits.

The recorded foreign-currency sample has SDK Tag 5 `1068.75` (SAR accounting VAT) versus Fatoority `285` (USD invoice VAT). `SDK-QR-005` explains the published-term interpretation. `SDK-QR-004` separates monetary equivalence from the additional SDK-profile Tag 4 spelling check. Full validation results remain separate from QR-field equality; decimal mutation inputs may fail business rules.

These are local signing and validation comparisons. They contain no production clearance result or evidence of certificate issuer trust. See `docs/development/qr-correctness-evidence.md` for scope and commands.

`development-api.json` records 15 live development compliance probes, including negative controls. It contains no private key, credential token or secret. Those response values are observations: the service accepted an incorrect Tag 5 amount and an unexpected standard Tag 9, so they must not become permissive native validation expectations. No actual reporting/clearance endpoint was called and no platform-stamped XML was returned.
