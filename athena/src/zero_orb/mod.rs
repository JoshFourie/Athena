use zksnark::*;
use zksnark::field::z251::Z251;

use std::path::Path;
use std::fs::{File, read_to_string};
use std::io::Write;
use std::result::Result;

use itertools::Itertools;
use serde_json::{to_string, from_str};
use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct PilotOrb {
    pub tag: Vec<u8>,
    pub proof: Proof<Z251, Z251>,
}

pub struct CommonReference {
    pub code: Vec<u8>,
    pub qap: QAP<CoefficientPoly<Z251>>,
    pub sg1: SigmaG1<Z251>,
    pub sg2: SigmaG2<Z251>,
}

impl CommonReference {
    pub fn new(paths: [&Path; 4]) {
        let qap: QAP<CoefficientPoly<Z251>> = ASTParser::try_parse(
            &read_to_string(paths[0]).unwrap()
        ).unwrap().into();
        let (sg1, sg2) = groth16::setup(&qap);
        File::create(paths[1]).unwrap().write_all(
            to_string(&qap).unwrap().as_bytes()
        ).unwrap();
        File::create(paths[2]).unwrap().write_all(
            to_string(&sg1).unwrap().as_bytes()
        ).unwrap();
        File::create(paths[3]).unwrap().write_all(
            to_string(&sg2).unwrap().as_bytes()
        ).unwrap();
    }
    pub fn read(paths: [&Path; 4]) -> Self {
        let code = read_to_string(paths[0]).unwrap().as_bytes().to_vec();
        let qap : QAP<CoefficientPoly<Z251>> = from_str(
            &read_to_string(paths[1]).unwrap()
        ).unwrap();
        let sg1 : SigmaG1<Z251> = from_str(
            &read_to_string(paths[2]).unwrap()
        ).unwrap();
        let sg2 : SigmaG2<Z251> = from_str(
            &read_to_string(paths[3]).unwrap()
        ).unwrap();
        Self {
            code,
            qap,
            sg1,
            sg2
        }
    }
}

pub trait IntoInnerField {
    type Field;
    fn collect_as_field(self) -> Result<Self::Field, ()>;
} 

impl IntoInnerField for Vec<u8> {
    type Field = Vec<Z251>;
    fn collect_as_field(self) -> Result<Self::Field, ()> {
        let bit_array = self.into_iter().map(|mut num| {
            let mut bits: [u8; 8] = [0; 8];
            for i in 0..8 {
                bits[i] = num % 2;
                num = num >> 1;
            }
            bits
        }).collect::<Vec<_>>();
        Ok(
            bit_array.into_iter()
                .map(|a| {
                a.iter().map(|&n| {
                    assert!(n < 251);
                    Z251 { inner: (n) as u8 }
                }).collect::<Vec<_>>()        
            }).concat()
        )
    }
}

impl IntoInnerField for Option<Vec<u8>> {
    type Field = Vec<Z251>;
    fn collect_as_field(self) -> Result<Self::Field, ()> {
        match self {
            Some(x) => {
                let bit_array = x.into_iter()
                    .map(|mut num| {
                        let mut bits: [u8; 8] = [0; 8];
                        for i in 0..8 {
                            bits[i] = num % 2;
                            num = num >> 1;
                        }
                        bits
                    }).collect::<Vec<_>>();
                return Ok(
                    bit_array.into_iter()
                        .map(|a| {
                        a.iter().map(|&n| {
                            assert!(n < 251);
                            Z251 { inner: (n) as u8 }
                        }).collect::<Vec<_>>()        
                    }).concat()
                )
            },
            None => Err(())
        }
    }
}

pub trait Knowledgeable {
    fn new(
        witness_bits: Vec<u8>, variable_bits: Vec<u8>, 
        tag: Vec<u8>, paths: [&Path; 4]) -> Self;
    fn check(self, verify_num: Vec<u8>, verify_bits: Vec<u8>, paths: [&Path; 4]) -> bool;
    fn as_bits(&self) -> Vec<u8>;
}

impl Knowledgeable for PilotOrb {
    fn new(
        witness_bits: Vec<u8>, 
        variable_bits: Vec<u8>, 
        tag: Vec<u8>, 
        paths: [&Path; 4]
    ) -> Self {
        let crs = CommonReference::read(paths);
        let mut assignments = witness_bits.collect_as_field().unwrap();
        assignments.append(&mut variable_bits.collect_as_field().unwrap());
        let weights = groth16::weights(
            std::str::from_utf8(
                crs.code.as_slice()
            ).unwrap(), 
            &assignments
        ).unwrap();    
        Self {
            tag: tag,
            proof: groth16::prove(
                &crs.qap,
                (&crs.sg1, &crs.sg2),
                &weights
            ) 
        }
    }
    fn check(self, verify_num: Vec<u8>, verify_bits: Vec<u8>, paths: [&Path; 4]) -> bool {
        let crs = CommonReference::read(paths);
        let mut inputs = verify_num.into_iter()
                .map(|num: u8| Z251::from(num as usize))
                .collect::<Vec<_>>();
        inputs.append(&mut verify_bits.collect_as_field().unwrap());
        groth16::verify::<CoefficientPoly<Z251>, _, _, _, _>(
            (crs.sg1, crs.sg2),
            &inputs,
            self.proof
        )
    }
    fn as_bits(&self) -> Vec<u8> {
        let string = to_string(&self).unwrap();
        string.as_bytes().to_vec()
    }
}