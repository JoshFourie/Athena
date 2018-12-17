extern crate zksnark;
extern crate tiny_keccak;
extern crate itertools;
#[macro_use] extern crate log;

pub mod client;
pub mod common_reference;

use self::client::{Attributes, TargetRange};
use self::common_reference::CommonReference;

fn main () {
    use std::fs::read_to_string;
    use zksnark::field::z251::Z251;
    use zksnark::groth16;
    use zksnark::*;

    // Prove honest_person > target_age.
    let honest_person = Attributes::new(
        22 as u8, 
        150 as u8,
        220 as u8);
    let target_age = TargetRange::new(21, 24);
    let crs = CommonReference::read();
    let lower_age = target_age.lower;
    let lower_bits = TargetRange::to_bits(lower_age);
    let age_bits = Attributes::to_bits(honest_person.age);
    
    // building the proof.
    let assignments = age_bits.iter().chain(
        lower_bits.iter()
        ).map(|&bit| Z251::from(bit as usize)).collect::<Vec<_>>();
    let code = read_to_string("src/common_reference/8bit_comparator.zk").unwrap();
    let weights = groth16::weights(&code, &assignments).unwrap();
    let proof = groth16::prove(&crs.qap, (&crs.sg1, &crs.sg2), &weights);

    // building the verification query.
    // verifying that the comparator puts out a 1 for age > target_age.lower.
    let mut inputs = vec![Z251::from(1)];
    inputs.append(
        &mut lower_bits.iter()
        .map(|&bit| Z251::from(bit as usize))
        .collect::<Vec<_>>()
    );
    let outcome = groth16::verify::<CoefficientPoly<Z251>, _, _, _, _>(
        (crs.sg1, crs.sg2),
        &inputs,
        proof
    );
    println!("{}", outcome);
}




fn new_reference() {
    CommonReference::new();
    loop {
        match CommonReference::check_comparator() {
            Ok(true) => {
                println!("Found operable CRS.. Breaking loop.");
                break   
            },
            Ok(false) => {
                CommonReference::new();
                println!("Retrying CRS...");
            },
            _ => panic!()
        }
    }
}
