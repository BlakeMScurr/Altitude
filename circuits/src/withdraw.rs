use std::{collections::HashMap, env::current_dir};

use crate::nova::nova;
use ff::PrimeField;
use nova_macro::{NovaInput, NovaRoundInput};
use nova_macro_derive::{NovaInput, NovaRoundInput};
use nova_scotia::{circom::reader::load_r1cs, FileLocation, F1};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, NovaInput)]
#[allow(non_snake_case)]
struct Withdraw {
    step_in: [String; 1],
    private_inputs: Vec<WithdrawRound>,
}

#[derive(Serialize, Deserialize, Debug, Clone, NovaRoundInput)]
#[allow(non_snake_case)]
struct WithdrawRound {
    sender: String,
    recipient: String,
    leaf_coins: [String; 2],
    pathElements: Vec<String>,
    pathIndices: Vec<String>,
    r: Vec<String>,
    s: Vec<String>,
    msghash: Vec<String>,
    pubkey: [[String; 4]; 2],
}

pub fn withdraw(iteration_count: usize) {
    let root = current_dir().unwrap();

    let circuit_file = root.join("circuits/build/compiled_circuit/withdraw.r1cs");
    let r1cs = load_r1cs(&FileLocation::PathBuf(circuit_file));
    let witness_generator_wasm =
        root.join("circuits/build/compiled_circuit/withdraw_js/withdraw.wasm");

    let withdraw_data: Withdraw =
        serde_json::from_str(include_str!("../inputs/withdraw.json")).unwrap();
    nova(
        iteration_count,
        false,
        r1cs,
        withdraw_data,
        witness_generator_wasm,
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    // use std::env;
    #[test]
    fn test_nova() {
        withdraw(2);
    }
}
