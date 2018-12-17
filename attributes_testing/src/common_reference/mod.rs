use zksnark::*;
use zksnark::groth16::*;
use zksnark::groth16::fr::*;
use zksnark::field::z251::Z251;

use std::fs::{File, read_to_string};
use std::io::Write;
use std::result::Result::{Ok, Err};
use std::thread::spawn;
use std::path::Path;

use serde_json::{to_string, from_str};

pub struct CommonReference {
    pub qap: QAP<CoefficientPoly<Z251>>,
    pub sg1: SigmaG1<Z251>,
    pub sg2: SigmaG2<Z251>,
}

impl CommonReference {

    pub fn new() {
        println!("Building dependencies for a new common-reference string...");
        let code = read_to_string("src/common_reference/8bit_comparator.zk").unwrap();
        let qap: QAP<CoefficientPoly<Z251>> = ASTParser::try_parse(&code).unwrap().into();
        let (sg1, sg2) = groth16::setup(&qap);
        println!("Ok. Built CRS... Writing to file.");
        let mut qap_file = File::create("src/common_reference/QAP.json").unwrap();
        qap_file.write_all(
            to_string(
                &qap
            ).unwrap().as_bytes()
        );
        let mut g1_file = File::create("src/common_reference/SigmaG1.json").unwrap();
        g1_file.write_all(
            to_string(
                &sg1
            ).unwrap().as_bytes()
        );
        let mut g2_file = File::create("src/common_reference/SigmaG2.json").unwrap();
        g2_file.write_all(
            to_string(
                &sg2
            ).unwrap().as_bytes()
        );
        println!("Written CRS to file.");
    }

    pub fn read() -> Self {
        println!("Reading CRS...");
        let qap : QAP<CoefficientPoly<Z251>> = from_str(
            &read_to_string("src/common_reference/QAP.json").unwrap()
        ).unwrap();
        let sg1 : SigmaG1<Z251> = from_str(
            &read_to_string("src/common_reference/SigmaG1.json").unwrap()
        ).unwrap();
        let sg2 : SigmaG2<Z251> = from_str(
            &read_to_string("src/common_reference/SigmaG2.json").unwrap()
        ).unwrap();
        println!("Returning CRS...");
        Self {
            qap,
            sg1,
            sg2
        }
    }

    pub fn check_comparator() -> Result<bool, ()> {
        
        fn to_bits(mut num: u8) -> [u8; 8] {
            let mut bits: [u8; 8] = [0; 8];
            for i in 0..8 {
                bits[i] = num % 2;
                num = num >> 1;
            }
            bits
        }

        let code = read_to_string("src/common_reference/8bit_comparator.zk").unwrap();
        let qap : QAP<CoefficientPoly<Z251>> = from_str(
            &read_to_string("src/common_reference/QAP.json").unwrap()
        ).unwrap();
        println!("Built 'comparator_test' dependencies... Testing...");
        let mut passed = 0;
        for _ in 0..1000 {
            let sigmag1: SigmaG1<Z251> = from_str(
                &read_to_string("src/common_reference/SigmaG1.json").unwrap()
            ).unwrap();
            let sigmag2: SigmaG2<Z251> = from_str(
                &read_to_string("src/common_reference/SigmaG2.json").unwrap()
            ).unwrap();
            let (a_bits, b_bits) = (to_bits(1 as u8), to_bits(2 as u8));
            let assignments = a_bits
                .iter()
                .chain(b_bits.iter())
                .map(|&bit| Z251::from(bit as usize))
                .collect::<Vec<_>>();
            let weights = groth16::weights(&code, &assignments).unwrap();
            let proof = groth16::prove(&qap, (&sigmag1, &sigmag2), &weights);
            // a > b
            let mut inputs = vec![Z251::from(1)];
            inputs.append(
                &mut b_bits
                    .iter()
                    .map(|&bit| Z251::from(bit as usize))
                    .collect::<Vec<_>>(),
            );
            let outcome = groth16::verify::<CoefficientPoly<Z251>, _, _, _, _>(
                (sigmag1, sigmag2),
                &inputs,
                proof
            );
            match outcome {
                false => passed = passed + 1,
                true => {}
            };
        }
        match passed == 1000 {
            true => Ok(true),
            false => Ok(false),
            _ => Err(())
        }
    }
}
