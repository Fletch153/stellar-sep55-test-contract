# SEP-55 test contract design

## Goal

Create an isolated, reproducible end-to-end test of Stellar contract build verification on testnet using StellarExpert's official workflow.

## Build and provenance

- Public repository: `Fletch153/stellar-sep55-test-contract`.
- Minimal Rust Soroban contract with no constructor arguments.
- Build from tag `v0.1.0` through `stellar-expert/soroban-build-workflow/.github/workflows/release.yml@main`.
- Grant `id-token: write`, `contents: write`, and `attestations: write`.
- Treat the generated GitHub release WASM as the only deployable artifact.

## Deployment

- Download the generated release WASM rather than rebuilding locally.
- Verify its SHA-256, `source_repo` metadata, and GitHub attestation before deployment.
- Generate an ephemeral Stellar testnet identity in a temporary config directory.
- Fund it through Friendbot and deploy the exact release bytes to testnet.
- Keep no secret key in Git, workflow logs, or durable project files.

## Verification

- Fetch the deployed WASM from testnet and compare it byte-for-byte with the release artifact.
- Verify the fetched bytes using `gh attestation verify`.
- Inspect the official workflow's StellarExpert notification response; `{"ok":1}` means queue insertion, while `{}` does not.
- Query the StellarExpert testnet contract record for validation status.
- If the fresh contract remains unverified, deploy a known verified WASM as a control to distinguish fresh-intake failure from testnet display behavior.

## Success criteria

- Release artifact, attestation subject, deployed WASM, and on-chain hash are identical.
- StellarExpert returns `{"ok":1}` and displays the testnet instance as verified.
- If StellarExpert fails, evidence identifies the first failing boundary without changing production Chainlink contracts.
