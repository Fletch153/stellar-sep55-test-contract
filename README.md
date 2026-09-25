# Stellar SEP-55 verification experiment

This repository isolates StellarExpert's SEP-55 verification path using its official reusable workflow and a throwaway Stellar testnet deployment.

## Fresh official-workflow build

- Source tag: [`v0.1.0`](https://github.com/Fletch153/stellar-sep55-test-contract/tree/v0.1.0)
- Workflow run: [36188911595](https://github.com/Fletch153/stellar-sep55-test-contract/actions/runs/36188911595)
- Release: [`v0.1.0_contracts_sep55_test_sep55-test_cli27.0.0`](https://github.com/Fletch153/stellar-sep55-test-contract/releases/tag/v0.1.0_contracts_sep55_test_sep55-test_cli27.0.0)
- Released WASM SHA-256: `851b1520cb55ae6c481e42367542215c7da722124da53001644e435df96d24f1`
- Testnet contract: [`CDLSU5W4XQQDR3FTK7CNU624XMYGO7GYMFXGLU2RQ3M7HLN2R2BK2SC5`](https://stellar.expert/explorer/testnet/contract/CDLSU5W4XQQDR3FTK7CNU624XMYGO7GYMFXGLU2RQ3M7HLN2R2BK2SC5)
- Deploy transaction: [`bbc27ba36bef820234ea151e2af9b9bf42ef9388d3d803dd9fa44d62b1d1c9ac`](https://stellar.expert/explorer/testnet/tx/bbc27ba36bef820234ea151e2af9b9bf42ef9388d3d803dd9fa44d62b1d1c9ac)

The release artifact contains `source_repo=github:Fletch153/stellar-sep55-test-contract`. The deployed bytes are byte-for-byte identical to the release artifact, and `gh attestation verify` succeeds when the reusable signer repository is specified:

```bash
gh attestation verify sep55-test_v0.1.0.wasm \
  --repo Fletch153/stellar-sep55-test-contract \
  --signer-repo stellar-expert/soroban-build-workflow
```

Despite this, StellarExpert reports `validation.status: unverified`.

The official reusable workflow submitted the expected payload to `/explorer/public/contract-validation/match`, but the endpoint returned `{}`. Historical successful intake returns `{"ok":1}`.

## Known-verified control

To test whether StellarExpert supports verified hashes on testnet, the already-verified Reflector WASM hash `8ecd1857496df2c15aaab4d18d2d7689542a62814245e9b2c613c609b86bd11c` was deployed as a new testnet instance:

- Control contract: [`CDCWOY3PFWZRIGFLVCUWLOXQNA42DG6GR4HP4Z3IW2ZXZ3IBW53KNYNO`](https://stellar.expert/explorer/testnet/contract/CDCWOY3PFWZRIGFLVCUWLOXQNA42DG6GR4HP4Z3IW2ZXZ3IBW53KNYNO)
- Deploy transaction: [`2d2c21dbd0fa969f56ac27aa0871f2c1592491571b14ae585e78a38913814cfd`](https://stellar.expert/explorer/testnet/tx/2d2c21dbd0fa969f56ac27aa0871f2c1592491571b14ae585e78a38913814cfd)
- StellarExpert status: `verified`

This proves that StellarExpert displays globally verified WASM hashes on testnet. The fresh contract remains unverified because its official-workflow notification was not inserted into StellarExpert's validation queue, not because of deployment, metadata, attestation, or testnet support.

## Security

Deployment used a newly generated, Friendbot-funded testnet identity stored only in an ephemeral temporary configuration directory. No secret key is present in this repository or workflow.
