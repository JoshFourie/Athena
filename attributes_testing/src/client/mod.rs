use zksnark::field::z251::Z251;
use rand::Rng;

type Bits = [u8; 8];

pub struct TargetRange {
    pub lower: u8,
    pub upper: u8,
}

impl TargetRange {
    pub fn new(lower: u8, upper: u8) -> Self {
        Self {
            lower: lower,
            upper: upper
        }
    }
    pub fn new_random(lower: [u8; 2], upper: [u8; 2]) -> Self {
        Self {
            lower: rand::thread_rng().gen_range(lower[0], lower[1]),
            upper: rand::thread_rng().gen_range(upper[0], upper[1])
        }
    }
    pub fn to_bits(mut num: u8) -> Bits {
        let mut bits: Bits = [0; 8];
        for i in 0..8 {
            bits[i] = num % 2;
            num = num >> 1;
        }
        bits 
    }
}

pub struct Attributes {
    pub age: u8,
    pub health_rating: u8,
    pub credit_score: u8,
}

impl Attributes {
    pub fn new(age: u8, health_rating: u8, credit_score: u8) -> Self {
        Self {
            age: age,
            health_rating: health_rating,
            credit_score: credit_score
        }
    }
    pub fn new_random() -> Self {
        Self {
            age: rand::thread_rng().gen_range(18, 85),
            health_rating: rand::thread_rng().gen_range(30, 100),
            credit_score: rand::thread_rng().gen_range(30, 120)
        }
    }
    pub fn to_bits(mut num: u8) -> Bits {
        let mut bits: Bits = [0; 8];
        for i in 0..8 {
            bits[i] = num % 2;
            num = num >> 1;
        }
        bits 
    }
}