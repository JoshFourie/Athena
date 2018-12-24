# Dev-Notes
We require the 'check' in the common-reference struct to confirm that the chosen CRS is not faulty as a work-around for a difficulties we had in zkSNARK-rs when testing the comparator. Not implemented at the moment.

We are maintaining a fork of zkSNARK-rs available here: https://github.com/republicprotocol/zksnark-rs until the lib is production ready. We have implemented some traits from testing units directly to the lib and included the SERDE ser/de macro. We are also maintaining Hackwork's BN crate: https://github.com/zcash-hackworks/bn, for SERDE compatability.

The current objective is to build a struct/handler that scales as best as possible for the client/server which is currently done using Vec<u8> values that are mapped to the in, out and verify options native to the .zk dsl. Vec<u8> is currently used in favour of a generic as we require a substitute for the .concat() and .append() methods before we can implement generics. Traits are named for fun rather than information: we say an orb is knowledgeable when a zksnark can be generated.

TO DO:
  [ ] Create 'check' for crs.
  [ ] Fall-back for absent data: Programmatic/Systems level.
  [ ] Create handler.

##Progress: 
Abandoning baking matching into the Orb at the moment, there are a lot of complexities unnecessary for the MVP discussed below and, instead, I will focus on the handler and building fall-backs for absent data-values. Currently 'AthenianOrb' denotes an orb that takes in a vector of arbitrary bits (witness_bits, variable_bits) checked against an arbitrary number of u8 (verify_num) values and an arbitrary number of bits (verify_bits). The pilot should extend to include the health rating with a number of proofs baked into the verification at the prover's discretion. 

##Problem: 
We reqiure the orb to handle a specific .zk program for the duration of the pilot, but we also need to handle instances where an input is not provided. Currently,
a malformed run of inputs returns a StructureError from within the .zk. We either need to provide a matching solution which pulls the correct .zk program for the data provided, or to bake a solution for absent values into the .zk, which is not a known solution at the moent. A generic orb will only function correctly if the prover can communicate the .zk that was inputted, and that this maps to a .zk issued by the verifier.

Note that we don't pass 'code' around, the prover requires the code and then the verifier merely requires g1/g2, but that we should probably match the crs to the code through a bit comparison.

### The Attack Vector 
There could be a moonshot attack where an attacker places a .zk in the verifier's repo and then calls that through the API in the generic orb instance. This would require the attacker to request that the verifier reads a pre-agreed code file to string that is then parsed by the ASTParser and embedded in the QAP. Alternatively and potentially more realistically, a legitimate API call to a malformed code and crs file hiding malciious code could expose the verifier to attack. Potentially storing the code on the blockchain might assist us but, again, that opens the verifier to a man in the middle attack at each call.

In the iterim, a generic proof seems to raise more issues than it solves so we should only consider implementing a massively pared down version.

##Expected Behaviour: Pilot Orb
The Pilot Orb is expected to handle everything for the wellness program such that the Android env can just pass a correctly formed string that spits back a proof or 
sends it to the appropriate verifier. We then need to build in guards on the verifier side against malicious attacks. The more interaction and commands we accept from the 
prover, the more vectors we expose.