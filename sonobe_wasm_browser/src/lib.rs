use ark_relations::r1cs;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

use ark_bn254::{constraints::GVar, Bn254, Fr, G1Projective as G1};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};

use ark_groth16::Groth16;
use ark_grumpkin::{constraints::GVar as GVar2, Projective as G2};

use ark_ff::{BigInteger, BigInteger256};
use ark_ff::{Field, PrimeField};
use byteorder::{LittleEndian, ReadBytesExt};
use folding_schemes::{
    commitment::{kzg::KZG, pedersen::Pedersen},
    folding::nova::{
        decider_eth::{prepare_calldata, Decider as DeciderEth},
        Nova, PreprocessorParam,
    },
    frontend::{circom::CircomFCircuit, FCircuit},
    transcript::poseidon::poseidon_canonical_config,
    Decider, FoldingScheme,
};
use num_bigint::BigInt;
use std::collections::HashMap;
use std::io::Read;
use std::path::PathBuf;
use web_sys::js_sys::Uint8Array;
use web_time::Instant;

const STATE_LEN: usize = 1usize;
const EXTERNAL_INPUTS_LEN: usize = 2usize;

#[wasm_bindgen]
pub unsafe fn single_fold(r1cs_bytes: Vec<u8>, witness_js: Uint8Array) {
    pub type N =
        Nova<G1, GVar, G2, GVar2, CircomFCircuit<Fr>, KZG<'static, Bn254>, Pedersen<G2>, false>;

    // Set the panic hook to provide better error messages
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    // Initialize needed stuff.
    let poseidon_config = poseidon_canonical_config::<Fr>();
    let mut rng = rand::rngs::OsRng;

    // Deserialize witness
    let witness = load_witness_from_bin_reader(&witness_js.to_vec()[..]);
    let read_witness = format!("{:?}", witness.clone());
    web_sys::console::log_1(&format!("{:?}", Box::leak(Box::new(read_witness))).into());

    // initialize z_0
    let w_0 = vec![Fr::from(0_u32)];
    // initialize external inputs
    let external_inputs = vec![Fr::from(0_u32), Fr::from(0_u32)];

    // (r1cs_bytes, state_len, external_inputs_len)
    let f_circuit_params = (r1cs_bytes.into(), 1, 2);
    web_sys::console::log_1(&"circuit params created".into());

    let f_circuit = CircomFCircuit::<Fr>::new(f_circuit_params).unwrap();
    web_sys::console::log_1(&"created circuit!".into());

    // prepare the Nova prover & verifier params
    let nova_preprocess_params = PreprocessorParam::new(poseidon_config, f_circuit.clone());
    web_sys::console::log_1(&"Nova preprocess params created".into());

    let nova_params = N::preprocess(&mut rng, &nova_preprocess_params).unwrap();
    web_sys::console::log_1(&"prepared nova prover and verifier params!".into());

    // initialize the folding scheme engine, in our case we use Nova
    let mut nova = N::init(&nova_params, f_circuit.clone(), w_0).unwrap();
    web_sys::console::log_1(&"initialized folding scheme!".into());

    // Now we need to start
    let now = Instant::now();
    nova.prove_step(rng, witness.clone(), None).unwrap();
    let duration = now.elapsed();
    alert(Box::leak(Box::new(format!("Fold done in {:?}", duration))));

    alert("🎉");
}
