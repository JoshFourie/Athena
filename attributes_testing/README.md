# NOTES

The circuit is currently constructed only for a > b when var. input for groth16::verify == 1, and a < b when var. input == 0.

We require the 'check' in the common-reference builder to confirm that the chosen CRS is not faulty as a work-around for a current error in zkSNARK-rs.