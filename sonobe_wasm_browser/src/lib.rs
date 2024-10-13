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

#[wasm_bindgen]
pub unsafe fn single_fold(r1cs_bytes: Vec<u8>, witness_js: Uint8Array) -> String {
    pub type N =
        Nova<G1, GVar, G2, GVar2, CircomFCircuit<Fr>, KZG<'static, Bn254>, Pedersen<G2>, false>;

    // Set the panic hook to provide better error messages
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    let poseidon_config = poseidon_canonical_config::<Fr>();
    let mut rng = rand::rngs::OsRng;

    println!("Start folding");

    // Deserialize witness
    let witness = load_witness_from_bin_reader(&witness_js.to_vec()[..]);
    let read_witness = format!("{:?}", witness.clone());
    alert(Box::leak(Box::new(read_witness)));

    alert("external_inputs created");

    // TODO: Add fn to include initial state.
    // Else default to a default values fn.
    // initialize z_0
    let w_0 = vec![Fr::from(0_u32)];
    // initialize external inputs
    let external_inputs = vec![Fr::from(0u32), Fr::from(0u32)];

    let z_0 = [w_0, external_inputs].concat();

    // (r1cs_bytes, state_len, external_inputs_len)
    let f_circuit_params = (r1cs_bytes.into(), witness.clone(), 1, 2);
    alert("circuit params created");

    let mut f_circuit = CircomFCircuit::<Fr>::new(f_circuit_params).unwrap();
    alert("created circuit!");

    // prepare the Nova prover & verifier params
    let nova_preprocess_params = PreprocessorParam::new(poseidon_config, f_circuit.clone());
    alert("Nova preprocess params created");

    let nova_params = N::preprocess(&mut rng, &nova_preprocess_params).unwrap();
    alert("prepared nova prover and verifier params!");

    // initialize the folding scheme engine, in our case we use Nova
    let mut nova = N::init(&nova_params, f_circuit.clone(), z_0).unwrap();
    alert("initialized folding scheme!");

    // run 1 step of the folding iteration
    nova.prove_step(rng, witness.clone(), None).unwrap();
    alert(&format!("Nova::prove_step"));

    alert("finished folding!");

    web_sys::console::log_1(&format!("public_inputs_serialized: ").into());

    alert("🎉");
    format!("{:?}", nova.F.witness)
}

/// load witness from u8 array by a reader
pub(crate) fn load_witness_from_bin_reader<R: Read>(mut reader: R) -> Vec<Fr> {
    let mut wtns_header = [0u8; 4];
    reader.read_exact(&mut wtns_header).expect("read_error");
    if wtns_header != [119, 116, 110, 115] {
        // ruby -e 'p "wtns".bytes' => [119, 116, 110, 115]
        alert("invalid file header");
    }
    let version = reader.read_u32::<LittleEndian>().expect("read_error");
    // println!("wtns version {}", version);
    if version > 2 {
        alert("unsupported file version");
    }
    let num_sections = reader.read_u32::<LittleEndian>().expect("read_error");
    if num_sections != 2 {
        alert("invalid num sections");
    }
    // read the first section
    let sec_type = reader.read_u32::<LittleEndian>().expect("read_error");
    if sec_type != 1 {
        alert("invalid section type");
    }
    let sec_size = reader.read_u64::<LittleEndian>().expect("read_error");
    if sec_size != 4 + 32 + 4 {
        alert("invalid section len")
    }
    let field_size: u32 = reader.read_u32::<LittleEndian>().expect("read_error");
    if field_size != 32 {
        alert("invalid field byte size");
    }
    let mut prime = vec![0u8; field_size as usize];
    reader.read_exact(&mut prime).expect("read_error");
    // if prime != hex!("010000f093f5e1439170b97948e833285d588181b64550b829a031e1724e6430") {
    //     alert("invalid curve prime {:?}", prime);
    // }
    let witness_len: u32 = reader.read_u32::<LittleEndian>().expect("read_error");
    // println!("witness len {}", witness_len);
    let sec_type = reader.read_u32::<LittleEndian>().expect("read_error");
    if sec_type != 2 {
        alert("invalid section type");
    }
    let sec_size = reader.read_u64::<LittleEndian>().expect("read_error");
    if sec_size != (witness_len * field_size) as u64 {
        alert("invalid witness section size");
    }
    let mut result = Vec::with_capacity(witness_len as usize);
    for _ in 0..witness_len {
        result.push(read_field::<&mut R, Fr>(&mut reader).expect("read_error"));
    }
    result
}

fn read_field<R: Read, F: PrimeField>(
    mut reader: R,
) -> Result<F, ark_serialize::SerializationError> {
    let mut repr: Vec<u8> = F::ZERO.into_bigint().to_bytes_le();
    for digit in repr.iter_mut() {
        *digit = reader.read_u8()?;
    }
    Ok(F::from_le_bytes_mod_order(&repr[..]))
}
