#![no_main]
#![no_std] // std support is experimental

use risc0_zkvm::guest::env;
risc0_zkvm::guest::entry!(main);
use trajectory_core::{APPROXIMATE_CLIP_ONE_TRAJECTORY, APPROXIMATE_CLIP_TWO_TRAJECTORY, GM,calculate_trajectory };

pub fn main() {
    let mut t = 0.1;
    let mut i = 0;

    // Values for clip 1
    // $x_0$ parameter for the first clip
    let clip_one_x0: f64 = -0.08;
    /// $z_0$ parameter for the first clip
    let clip_one_z0: f64 = -0.04;
    /// $V_{x_0}$ parameter for the first clip
    let clip_one_vx0: f64 = 1.59;
    /// $V_{z_0}$ parameter for the first clip
    let clip_one_vz0: f64 = 2.23;

    // Values for clip 2
    // $x_0$ parameter for the second clip
    let clip_two_x0: f64 = -0.20;
    /// $z_0$ parameter for the second clip
    let clip_two_z0: f64 = 0.0;
    /// $V_{x_0}$ parameter for the second clip
    let clip_two_vx0: f64 = 2.99;
    /// $V_{z_0}$ parameter for the second clip
    let clip_two_vz0: f64 = 2.99;

    while t < 2.8 {
        // Calculate x and z for clip 1
        let (clip_one_x, clip_one_z) = calculate_trajectory(t, clip_one_x0, clip_one_z0, clip_one_vx0, clip_one_vz0);
        let (clip_two_x, clip_two_z) = calculate_trajectory(t, clip_two_x0, clip_two_z0, clip_two_vx0, clip_two_vz0);
        // x is correct
        assert_eq!(APPROXIMATE_CLIP_ONE_TRAJECTORY[i as usize].0, clip_one_x);
        assert_eq!(APPROXIMATE_CLIP_TWO_TRAJECTORY[i as usize].0, clip_two_x);
        // z is correct
        assert_eq!(APPROXIMATE_CLIP_ONE_TRAJECTORY[i as usize].1, clip_one_z);
        assert_eq!(APPROXIMATE_CLIP_TWO_TRAJECTORY[i as usize].1, clip_two_z);
        
        t += 0.1;
        i += 1;
    }

    // TODO: Get values out
    env::commit(&true);
}
