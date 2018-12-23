# NOTES

The circuit is currently constructed only for a > b when var. input for groth16::verify == 1, and a < b when var. input == 0.

We require the 'check' in the common-reference struct to confirm that the chosen CRS is not faulty as a work-around for a difficulties we had in zkSNARK-rs when testing the comparator. Console commands are used for logging purposes at the moment.

We are maintaining a fork of zkSNARK-rs available here: https://github.com/republicprotocol/zksnark-rs until the lib is production ready. We have implemented some traits from testing units directly to the lib and included the SERDE ser/de macro. We are also maintaining Hackwork's BN crate: https://github.com/zcash-hackworks/bn, for SERDE compatability.
