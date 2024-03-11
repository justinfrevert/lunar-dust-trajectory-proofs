#![no_main]
#![no_std] // std support is experimental

use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

const APPROXIMATE_CLIP_ONE_TRAJECTORY: [(f64, f64); 27] = [
    // x, z
     (0.07900000000000003, 0.175),
     (0.23800000000000004, 0.374),
     (0.3970000000000001, 0.5569999999999999),
     (0.5560000000000002, 0.724),
     (0.7150000000000001, 0.875),
     (0.874, 1.0099999999999998),
     (1.033, 1.129),
     (1.192, 1.2319999999999998),
     (1.3509999999999998, 1.3189999999999997),
     (1.5099999999999998, 1.3899999999999997),
     (1.6689999999999998, 1.4449999999999998),
     (1.8279999999999998, 1.4839999999999998),
     (1.987, 1.5069999999999997),
     (2.1460000000000004, 1.5139999999999998),
     (2.3050000000000006, 1.505),
     (2.4640000000000004, 1.4799999999999995),
     (2.6230000000000007, 1.4389999999999996),
     (2.782000000000001, 1.3819999999999997),
     (2.941000000000001, 1.3089999999999993),
     (3.100000000000001, 1.2199999999999993),
     (3.259000000000001, 1.1149999999999989),
     (3.418000000000001, 0.9939999999999993),
     (3.5770000000000013, 0.8569999999999984),
     (3.7360000000000015, 0.7039999999999988),
     (3.8950000000000014, 0.5349999999999984),
     (4.054000000000002, 0.349999999999997),
     (4.213000000000002, 0.14899999999999824),
];

const APPROXIMATE_CLIP_TWO_TRAJECTORY: [(f64, f64); 27] = [
    // x, z
    (0.09900000000000003, 0.29100000000000004),
    (0.3980000000000001, 0.5660000000000001),
    (0.6970000000000003, 0.8250000000000002),
    (0.9960000000000002, 1.068),
    (1.2950000000000002, 1.2950000000000002),
    (1.594, 1.506),
    (1.893, 1.701),
    (2.1919999999999997, 1.88),
    (2.4909999999999997, 2.043),
    (2.7899999999999996, 2.19),
    (3.0889999999999995, 2.3209999999999997),
    (3.388, 2.436),
    (3.6870000000000003, 2.535),
    (3.9860000000000007, 2.6180000000000003),
    (4.285000000000001, 2.6850000000000005),
    (4.584000000000001, 2.7360000000000007),
    (4.883000000000002, 2.771000000000001),
    (5.182000000000001, 2.79),
    (5.481000000000002, 2.793),
    (5.780000000000001, 2.78),
    (6.0790000000000015, 2.751),
    (6.378000000000002, 2.706),
    (6.677000000000002, 2.6449999999999996),
    (6.976000000000003, 2.5679999999999996),
    (7.275000000000003, 2.4749999999999996),
    (7.574000000000003, 2.3659999999999988),
    (7.873000000000004, 2.2409999999999997),
];

// The value we use for moon's gravity
const GM: f64 = 1.6;

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

fn calculate_trajectory(
    // Time
    t: f64,
    // Parameters which fit the ballistic equation, see: Table I.
    // $x_0$ parameter
    x0: f64,
    // $z_0$ parameter
    z0: f64,
    // $V_{x_0}$ parameter
    vx0: f64,
    // $V_{z_0}$ parameter 
    vz0: f64
) -> (f64, f64) {
    // Value for a given x
    // $x = X_0 + V_{X_0}t$
    let x = x0 + vx0 * t;
    // Value for a given y
    // $Z_0 + V_{Z_0}t - 1/2g_{M}t^2$
    let z = z0 + vz0 * t - (0.5 * GM * (t * t));
    (x, z)
}
