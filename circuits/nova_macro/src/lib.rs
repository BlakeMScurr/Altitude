use pasta_curves::Fq;
use std::collections::HashMap;

pub trait NovaRoundInput {
    fn circuit_input(&self) -> std::collections::HashMap<String, serde_json::Value>;
}

pub trait NovaInput {
    fn initial_inputs(&self) -> Vec<Fq>;
    fn round_inputs(&self) -> Vec<HashMap<String, serde_json::Value>>;
}
