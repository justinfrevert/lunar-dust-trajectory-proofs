# Moon Landing Proofs

I created this repo after noting lack of trust in expert opinion on the information in *[Tracking Lunar Dust - Analysis of Apollo Footage
](https://www.researchgate.net/publication/258468670_Tracking_Lunar_Dust_-_Analysis_of_Apollo_Footage)*. I attempt to recreate some of the computations here, and prove them in zero-knowledge. The hope is that doing so in such a trustless way might improve trust in the research. The lack of trust was around the computations performed in the math, so I will limit this work to minimally cover that. This work is intended for people who do not have an understanding of the original paper. The ultimate goal for this is that you *should be able to check the simple calculations for x and y, and see that we've copied them plainly into the code.*

## Structure
`graphs` - hosts the dust trajectory graph we plot with our data, as well as the matching graph from the original paper.
`host` - host-side code for preparing the RISC Zero ZKVM guest as well as code for plotting graphs.
`methods` - This code uses RISC Zero to generate ZK-STARK proofs of the correct execution of the code we care about proving.

## Description
I provide expected values for the dust trajectory of clip 1. I've gotten these by simply trying to perform the calculations in the paper(See `Introduction`). Within the context of code which generates a proof, I simply check that the math produces these same values. If they do, we generate a proof that it is the case.

Those expected values fall along the areas of dust shown in the paper, so if our calculations match theirs, we can believe that their math is working as expected(for clip 1), as the point of their paper is that their estimated lunar dust trajectory lines up with the actual movement of the dust on film.

To compare, check the produced graph in `graphs/simulated.png` with the original graph in `graphs/source-graph.png`

# Zk Proofs and RISC Zero

# How to use
To run the code to generate and verify the proof, and generate the graph:
1. Install [rust](https://www.rust-lang.org/tools/install)
2. run `cargo run`

# Proofs
