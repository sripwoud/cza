# Circom App Template

> [!NOTE]
> Bootstraped from [sripwoud/dapp-template](https://github.com/sripwoud/dapp-template)

This template includes:

- a sha256 circom circuit (from [erhant/circomkit](https://github.com/erhant/circomkit/tree/main/examples/bun-sha256))
- scripts to:
  - perform a dummy setup for this circuit and build the corresponding solidity verifier contract
  - start a local test ethereum node, deploy the verifier contract to it, start the web client that includes a `\proof` page to demo generating and verifying proof

## Develop

To start setup and start everything:

```commandline
./setup
mise r d
```
