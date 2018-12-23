# NOTES

We require the 'check' in the common-reference struct to confirm that the chosen CRS is not faulty as a work-around for a difficulties we had in zkSNARK-rs when testing the comparator. Console commands are used for logging purposes at the moment.

We are maintaining a fork of zkSNARK-rs available here: https://github.com/republicprotocol/zksnark-rs until the lib is production ready. We have implemented some traits from testing units directly to the lib and included the SERDE ser/de macro. We are also maintaining Hackwork's BN crate: https://github.com/zcash-hackworks/bn, for SERDE compatability.

The objective is to build a struct/handler that scales as best as possible for the client/server which is currently done using Vec<u8> values that are mapped to the in, out and verify options native to the .zk dsl. Vec<u8> is currently used in favour of a generic as we require a substitute for the .concat() and .append() methods before
we can implement generics. Traits are named for fun rather than information: we say an orb is knowledgeable when a zksnark can be generated.