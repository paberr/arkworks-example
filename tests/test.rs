use ark_crypto_primitives::crh::{pedersen, CRHScheme, CRHSchemeGadget};
use ark_mnt6_753::{constraints::G1Var, Fq, G1Projective};
use ark_r1cs_std::prelude::*;
use ark_relations::r1cs::{ConstraintSystem, ConstraintSystemRef};
use ark_std::rand::Rng;
use ark_std::test_rng;

type TestCRH = pedersen::CRH<G1Projective, Window>;
type TestCRHGadget = pedersen::constraints::CRHGadget<G1Projective, G1Var, Window>;
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Window;

impl pedersen::Window for Window {
    const WINDOW_SIZE: usize = 127;
    const NUM_WINDOWS: usize = 9;
}

fn generate_u8_input<R: Rng>(
    cs: ConstraintSystemRef<Fq>,
    size: usize,
    rng: &mut R,
) -> (Vec<u8>, Vec<UInt8<Fq>>) {
    let mut input = vec![1u8; size];
    rng.fill_bytes(&mut input);

    let mut input_bytes = vec![];
    for byte in input.iter() {
        input_bytes.push(UInt8::new_witness(cs.clone(), || Ok(byte)).unwrap());
    }
    (input, input_bytes)
}

#[test]
fn test_native_equality() {
    let rng = &mut test_rng();
    let cs = ConstraintSystem::<Fq>::new_ref();

    let (input, input_var) = generate_u8_input(cs.clone(), 128, rng);

    let parameters = TestCRH::setup(rng).unwrap();
    let primitive_result = TestCRH::evaluate(&parameters, input.as_slice()).unwrap();

    let parameters_var = pedersen::constraints::CRHParametersVar::new_constant(
        ark_relations::ns!(cs, "CRH Parameters"),
        &parameters,
    )
    .unwrap();

    let result_var = TestCRHGadget::evaluate(&parameters_var, &input_var).unwrap();

    let primitive_result = primitive_result;
    assert_eq!(primitive_result, result_var.value().unwrap());
    assert!(cs.is_satisfied().unwrap());
}
