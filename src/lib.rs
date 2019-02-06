#![no_std]
use libm::{sqrtf, atan2, fabs, asin};

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct V {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Q {
    pub q1: f32,
    pub q2: f32,
    pub q3: f32,
    pub q4: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct Euler {
    pub roll: f32,
    pub pitch: f32,
    pub yaw: f32,
}

const GYRO_MEAS_ERROR: f32 = 3.14159265358979 * (5.0 / 180.0); // using hardcoded value of pi to match paper

// w - gyroscope measurements in rad/s
// a - accelerometer measurements
// q - orientation quaternion elements initial conditions
// delta_t - sampling period in seconds
pub fn filter_update(w: V, mut a: V, mut q: Q, delta_t: f32) -> Q {
    let beta: f32 = sqrtf(3.0 / 4.0) * GYRO_MEAS_ERROR;

    let half_seq_1 = 0.5 * q.q1;
    let half_seq_2 = 0.5 * q.q2;
    let half_seq_3 = 0.5 * q.q3;
    let half_seq_4 = 0.5 * q.q4;
    let two_seq_1 = 2.0 * q.q1;
    let two_seq_2 = 2.0 * q.q2;
    let two_seq_3 = 2.0 * q.q3;

    let mut norm = sqrtf(a.x * a.x + a.y * a.y + a.z * a.z);
    a.x /= norm;
    a.y /= norm;
    a.z /= norm;

    let f_1 = two_seq_2 * q.q4 - two_seq_1 * q.q3 - a.x;
    let f_2 = two_seq_1 * q.q2 + two_seq_3 * q.q4 - a.y;
    let f_3 = 1.0 - two_seq_2 * q.q2 - two_seq_3 * q.q3 - a.z;
    let j_11_or_24 = two_seq_3;
    let j_12_or_23 = 2.0 * q.q4;
    let j_13_or_22 = two_seq_1;
    let j_14_or_21 = two_seq_2;
    let j_32 = 2.0 * j_14_or_21;
    let j_33 = 2.0 * j_11_or_24;

    let mut seq_hat_dot_1 = j_14_or_21 * f_2 - j_11_or_24 * f_1;
    let mut seq_hat_dot_2 = j_12_or_23 * f_1 + j_13_or_22 * f_2 - j_32 * f_3;
    let mut seq_hat_dot_3 = j_12_or_23 * f_2 - j_33 * f_3 - j_13_or_22 * f_1;
    let mut seq_hat_dot_4 = j_14_or_21 * f_1 + j_11_or_24 * f_2;

    norm = sqrtf(
        seq_hat_dot_1 * seq_hat_dot_1 +
            seq_hat_dot_2 * seq_hat_dot_2 +
            seq_hat_dot_3 * seq_hat_dot_3 +
            seq_hat_dot_4 * seq_hat_dot_4
    );
    seq_hat_dot_1 /= norm;
    seq_hat_dot_2 /= norm;
    seq_hat_dot_3 /= norm;
    seq_hat_dot_4 /= norm;

    let seq_dot_omega_1 = -half_seq_2 * w.x - half_seq_3 * w.y - half_seq_4 * w.z;
    let seq_dot_omega_2 = half_seq_1 * w.x + half_seq_3 * w.z - half_seq_4 * w.y;
    let seq_dot_omega_3 = half_seq_1 * w.y + half_seq_2 * w.z + half_seq_4 * w.x;
    let seq_dot_omega_4 = half_seq_1 * w.z + half_seq_2 * w.y - half_seq_3 * w.x;

    q.q1 += (seq_dot_omega_1 - (beta * seq_hat_dot_1)) * delta_t;
    q.q2 += (seq_dot_omega_2 - (beta * seq_hat_dot_2)) * delta_t;
    q.q3 += (seq_dot_omega_3 - (beta * seq_hat_dot_3)) * delta_t;
    q.q4 += (seq_dot_omega_4 - (beta * seq_hat_dot_4)) * delta_t;

    norm = sqrtf(q.q1 * q.q1 + q.q2 * q.q2 + q.q3 * q.q3 + q.q4 * q.q4);
    q.q1 /= norm;
    q.q2 /= norm;
    q.q3 /= norm;
    q.q4 /= norm;

    q
}

// returns the absolute value of x with the sign of y
fn copysign(x: f32, y: f32) -> f32 {
    if y > 0. { x } else { -x }
}

fn to_euler_angle(q: &Q) -> Euler {
    // roll (x-axis rotation)
    let sinr_cosp = 2.0 * (q.q1 * q.q2 + q.q3 * q.q4);
    let cosr_cosp = 1.0 - 2.0 * (q.q2 * q.q2 + q.q3 * q.q3);
    let roll = atan2(sinr_cosp as f64, cosr_cosp as f64) as f32;

    // pitch (y-axis rotation)
    let sinp = 2.0 * (q.q1 * q.q3 - q.q4 * q.q1);

    let pitch = if fabs(sinp as f64) >= 1. {
        copysign(core::f32::consts::PI / 2., sinp)
    } else {
        asin(sinp as f64) as f32
    };

    // yaw (z-axis rotation)
    let siny_cosp = 2.0 * (q.q1 * q.q3 + q.q2 * q.q3);
    let cosy_cosp = 1.0 - 2.0 * (q.q3 * q.q3 + q.q4 * q.q4);
    let yaw = atan2(siny_cosp as f64, cosy_cosp as f64) as f32;

    Euler { roll, pitch, yaw }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::quickcheck;
    use quickcheck::Gen;
    use quickcheck::Arbitrary;
    use rand::Rng;

    extern {
        fn filterUpdateC(w: V, a: V, q: Q, delta_t: f32) -> Q;
    }

    const DELTA_T: f32 = 0.001;

    fn compare_float(a: f32, b: f32) -> bool {
        // consider two NaN equal for purposes of comparing the output of the C algorithm
        //     to the Rust algorithm
        (a - b) < 0.01 || (a.is_nan() && b.is_nan())
    }

    impl PartialEq<Q> for Q {
        fn eq(&self, other: &Q) -> bool {
            compare_float(self.q1, other.q1) &&
            compare_float(self.q2, other.q2) &&
            compare_float(self.q3, other.q3) &&
            compare_float(self.q4, other.q4)
        }
    }

    impl Arbitrary for V {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            V {
                x: g.gen_range(-10., 10.),
                y: g.gen_range(-10., 10.),
                z: g.gen_range(-10., 10.),
            }
        }
    }

    impl Arbitrary for Q {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            Q {
                q1: g.gen_range(-1., 1.),
                q2: g.gen_range(-1., 1.),
                q3: g.gen_range(-1., 1.),
                q4: g.gen_range(-1., 1.),
            }
        }
    }

    #[test]
    fn simple_check() {
        let w = V {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let a = V {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        };
        let q_init = Q {
            q1: 1.0,
            q2: 0.0,
            q3: 0.0,
            q4: 0.0
        };
        let result_c = unsafe { filterUpdateC(w.clone(), a.clone(), q_init.clone(), DELTA_T) };
        let result = filter_update(w, a, q_init, DELTA_T);
        assert_eq!(result_c, result);
    }

    quickcheck!{
        fn c_implementation_matches_rust(w: V, a: V, q_init: Q) -> bool {
            let result_c = unsafe { filterUpdateC(w.clone(), a.clone(), q_init.clone(), DELTA_T) };
            let result = filter_update(w, a, q_init, DELTA_T);
            result_c == result
        }
    }

    #[test]
    fn copysign_pos() {
        assert_eq!(2., copysign(2.,3.));
    }

    #[test]
    fn copysign_neg() {
        assert_eq!(-2., copysign(2.,-3.));
    }
}
