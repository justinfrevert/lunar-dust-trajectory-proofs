# Lunar Dust Trajectory proofs

I created this repo after noting lack of trust in expert opinion on the information in *[Tracking Lunar Dust - Analysis of Apollo Footage
](https://www.researchgate.net/publication/258468670_Tracking_Lunar_Dust_-_Analysis_of_Apollo_Footage)*. I attempt to recreate some of the computations here, and prove them using RISC Zero's tooling. The hope is that doing so in such a trustless way might improve trust in the research. The lack of trust I noticed was around the computations performed in the math, so I will limit this work to minimally cover that portion. This work is intended for people who do not have an understanding of the original paper, but maybe trust the math wtihout knowing how to verify it themselves. The ultimate goal for this is that anyone *should be able to note the original calculations for x and y, and see that we've copied them plainly into this code.* To add, if anyone can see that the math in the code is correct, we prove that it is correct.

## Structure
`core` - This contains constants of expected values, and a function for performing the trajectory calculations.
`graphs` - hosts the dust trajectory graph we plot with our data, as well as the matching graph from the original paper.

`host` - host-side code for preparing the RISC Zero ZKVM guest as well as code for plotting graphs.

`methods` - This code uses RISC Zero to generate STARK proofs of the correct execution of the code we care about proving. **The core functionality for the code we care about is in (https://github.com/justinfrevert/lunar-dust-trajectory-proofs/blob/main/methods/guest/src/main.rs))**

## Description
I provide expected values for the dust trajectory of both clips. I've gotten these by simply trying to perform the calculations in the paper(See `Introduction`). Within the context of code which generates a proof, I simply check that the math produces these same values. If they do, we generate a proof that it is the case.

Those expected values fall along the areas of dust shown in the paper, so if our calculations match theirs, we can believe that their math is working as expected, as the point of their paper is that their estimated lunar dust trajectory lines up with the actual movement of the dust on film.

To compare, check the produced graph in `graphs/simulated-1.png` with the original graph in `graphs/source-graph.png`

# ZK Proofs and RISC Zero
If you're unfamiliar with the concept of Zero-knowledge, see the [wikipedia article](https://en.wikipedia.org/wiki/Zero-knowledge_proof).
This code uses RISC Zero to write STARK proofs from plain rust. [See more](https://www.risczero.com/).

# How to use
To run the code to generate and verify the proof, and generate the graph:
1. Install [rust](https://www.rust-lang.org/tools/install)
2. run `cargo run`

# Proofs
For transparency, the proof is stored in `proof`. If you know how to verify the proof, note the image id: `[1213147617, 2391554405, 1722749995, 1891971243, 1767211547, 4236494893, 2449305051, 298188669]`.
