use nova_macro::NovaInput;
use nova_scotia::{
    circom::circuit::R1CS, create_public_params, create_recursive_circuit, FileLocation, G2, S1, S2,
};
use nova_snark::{traits::Group, CompressedSNARK};
use pasta_curves::Fq;
use std::{path::PathBuf, time::Instant};

pub fn nova(
    iteration_count: usize,
    compress: bool,
    r1cs: R1CS<Fq>,
    input: impl NovaInput,
    witness_generator_wasm: PathBuf,
) {
    let start_public_input = input.initial_inputs();
    let private_inputs = input.round_inputs();

    let pp = create_public_params(r1cs.clone());

    println!(
        "Number of constraints per step (primary circuit): {}",
        pp.num_constraints().0
    );
    println!(
        "Number of constraints per step (secondary circuit): {}",
        pp.num_constraints().1
    );

    println!(
        "Number of variables per step (primary circuit): {}",
        pp.num_variables().0
    );
    println!(
        "Number of variables per step (secondary circuit): {}",
        pp.num_variables().1
    );

    println!("Creating a RecursiveSNARK...");
    let start = Instant::now();
    let recursive_snark = create_recursive_circuit(
        FileLocation::PathBuf(witness_generator_wasm),
        r1cs,
        private_inputs,
        start_public_input.clone(),
        &pp,
    )
    .unwrap();
    println!("RecursiveSNARK creation took {:?}", start.elapsed());

    // TODO: empty?
    let z0_secondary = vec![<G2 as Group>::Scalar::zero()];

    // verify the recursive SNARK
    println!("Verifying a RecursiveSNARK...");
    println!("z0_primary: {start_public_input:?}");
    println!("z0_secondary: {z0_secondary:?}");
    let start = Instant::now();
    let res = recursive_snark.verify(
        &pp,
        iteration_count,
        start_public_input.clone(),
        z0_secondary.clone(),
    );
    println!(
        "RecursiveSNARK::verify: {:?}, took {:?}",
        res,
        start.elapsed()
    );
    assert!(res.is_ok());

    // produce a compressed SNARK
    if compress {
        println!("Generating a CompressedSNARK using Spartan with IPA-PC...");
        let start = Instant::now();
        let (pk, vk) = CompressedSNARK::<_, _, _, _, S1, S2>::setup(&pp).unwrap();
        let res = CompressedSNARK::<_, _, _, _, S1, S2>::prove(&pp, &pk, &recursive_snark);
        println!(
            "CompressedSNARK::prove: {:?}, took {:?}",
            res.is_ok(),
            start.elapsed()
        );
        assert!(res.is_ok());
        let compressed_snark = res.unwrap();

        // verify the compressed SNARK
        println!("Verifying a CompressedSNARK...");
        println!("z0_primary: {start_public_input:?}");
        println!("z0_secondary: {z0_secondary:?}");
        let start = Instant::now();
        let res = compressed_snark.verify(
            &vk,
            iteration_count,
            start_public_input.clone(),
            z0_secondary,
        );
        println!(
            "CompressedSNARK::verify: {:?}, took {:?}",
            res.is_ok(),
            start.elapsed()
        );
        assert!(res.is_ok());
    }
}
