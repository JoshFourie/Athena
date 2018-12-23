extern crate zksnark;
extern crate tiny_keccak;
extern crate itertools;
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
pub mod zero_orb;

#[test]
fn test_athenian_knowledge() {
    for _ in 0..100 {
        let test = |input: [u8; 7], paths: [&Path; 4]| -> bool {
            AthenianOrb::new(
                vec![input[0]],
                vec![input[1], input[2]],
                b"Athenian".to_vec(),
                paths
            ).check(
                vec![input[3], input[4]],
                vec![input[5], input[6]],
                paths
            )
        };
        let paths = [
            Path::new("src/zero_orb/source/athenian/athena_statement.zk"),
            Path::new("src/zero_orb/source/athenian/athena_qap.json"),
            Path::new("src/zero_orb/source/athenian/athena_sg1.json"),
            Path::new("src/zero_orb/source/athenian/athena_sg2.json")
        ]; 
        assert_eq!(
            true, 
            test(
                [10, 5, 15, 1, 0, 5, 15],
                paths.clone()
            )
        );
        assert_eq!(
            false, 
            test(
                [20, 5, 15, 1, 0, 5, 15],
                paths.clone()
            )
        );
        assert_eq!(
            false, 
            test(
                [0, 5, 15, 1, 0, 5, 15],
                paths.clone()
            )
        );
        assert_eq!(
            false, 
            test(
                [10, 5, 20, 1, 0, 5, 15],
                paths.clone()
            )
        );
        assert_eq!(
            true, 
            test(
                [12, 10, 15, 1, 0, 5, 15],
                paths.clone()
            )
        );
    }
}