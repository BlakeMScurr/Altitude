use crate::nova::nova;
use ff::PrimeField;
use nova_macro::{NovaInput, NovaRoundInput};
use nova_macro_derive::{NovaInput, NovaRoundInput};
use nova_scotia::{circom::reader::load_r1cs, FileLocation, F1};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env::current_dir};

#[derive(Serialize, Deserialize, Debug, Clone, NovaInput)]
#[allow(non_snake_case)]
struct Mint {
    step_in: [String; 2],
    private_inputs: Vec<MintRound>,
}

#[derive(Serialize, Deserialize, Debug, Clone, NovaRoundInput)]
#[allow(non_snake_case)]
struct MintRound {
    sender: String,
    recipient: String,
    leaf_coins: [String; 2],
    mintPathElements: Vec<String>,
    mintPathIndices: Vec<String>,
    pathElements: Vec<String>,
    pathIndices: Vec<String>,
}

pub fn mint(iteration_count: usize) {
    let root = current_dir().unwrap();

    let circuit_file = root.join("circuits/build/compiled_circuit/mint.r1cs");
    let r1cs = load_r1cs(&FileLocation::PathBuf(circuit_file));
    let witness_generator_wasm = root.join("circuits/build/compiled_circuit/mint_js/mint.wasm");

    let mint_data: Mint = serde_json::from_str(include_str!("../inputs/mint.json")).unwrap();
    nova(
        iteration_count,
        true,
        r1cs,
        mint_data,
        witness_generator_wasm,
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    // use std::env;
    #[test]
    fn test_nova() {
        mint(2);
    }
}
