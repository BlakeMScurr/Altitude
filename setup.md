# Repo navigation

- At the root, it has the structure of a foundry repo.
- All circuit specific code is in [./circuits](./circuits) directory. Circuits are written in circom.

## Prerequisite installations
- Install custom circom fork
  -  We are using Nova Scotia which needs a custom Circom fork: https://github.com/nalinbhardwaj/circom/tree/pasta
  - Checkout the `pasta` branch locally and follow https://docs.circom.io/getting-started/installation/.
  - This replaces your global circom binary.

## Smart contracts

To run and test smart contracts:
- forge test

## Circuits
- Run `yarn install` in [./circuits/circuits](./circuits/circuits) and [./circuits/scripts](./circuits/scripts).
- Install git submodule [circom-ecdsa@v0.0.1](https://github.com/0xPARC/circom-ecdsa/releases/tag/v0.0.1) by running `git submodule init` and `git submodule update`.
- Navigate to [./circuits/circuits/git_modules/circom-ecdsa](./circuits/circuits/git_modules/circom-ecdsa), and run `ln -s  ../../node_modules node_modules`.
  - Make sure to not commit the updated submodule.
- Navigate to [./circuits/scripts/gen_keys.sh](./circuits/scripts/gen_keys.sh), set `CIRCUIT_NAME` to the name of the circuit you want to compile.
- Run `gen_keys.sh`. This creates the necessary files read by Nova Scotia code.
- Run the tests in [`mint.rs`](./circuits/src/mint.rs) with `cargo test` which reads input data from [`mint.json`](./circuits/inputs/mint.json`) and runs the mint circuit twice through Nova Scotia. You can also run the test directly from VS Code if you have the right setup.

If you want to generate a new input data for `mint.json`:
- Run `node mint.js` in [`circuits/scripts`](./circuits/scripts/) which prints the data for 2 iterations and copy the output to [`mint.json`](./circuits/inputs/mint.json`).
