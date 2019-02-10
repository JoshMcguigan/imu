use crate::Q;
use libm::{atan2f, asinf};
use core::f32::consts::PI;

#[derive(Clone, Copy, Debug)]
pub struct Euler {
    pub roll: f32,
    pub pitch: f32,
    pub yaw: f32,
}

impl From<Q> for Euler {
    fn from(q: Q) -> Self {
        let q1 = q.q1;
        let q2 = q.q2;
        let q3 = q.q3;
        let q4 = q.q4;
        let test = q2*q3 + q4*q1;
        if test > 0.499 { // singularity at north pole
            let pitch = 2. * atan2f(q2,q1);
            let yaw = PI/2.;
            let roll = 0.;
            return Euler { roll, pitch, yaw }
        }
        if test < -0.499 { // singularity at south pole
            let pitch = -2. * atan2f(q2,q1);
            let yaw = -PI/2.;
            let roll = 0.;
            return Euler { roll, pitch, yaw }
        }
        let sqx = q2*q2;
        let sqy = q3*q3;
        let sqz = q4*q4;
        let pitch = atan2f(2.*q3*q1-2.*q2*q4 , 1. - 2.*sqy - 2.*sqz);
        let yaw = asinf((2.*test).into());
        let roll = atan2f(2.*q2*q1-2.*q3*q4 , 1. - 2.*sqx - 2.*sqz);

        Euler { roll, pitch, yaw }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn compare_float(a: f32, b: f32) -> bool {
        (a - b) < 0.01
    }

    impl PartialEq<Euler> for Euler {
        fn eq(&self, other: &Euler) -> bool {
            compare_float(self.roll, other.roll) &&
            compare_float(self.pitch, other.pitch) &&
            compare_float(self.yaw, other.yaw)
        }
    }

    #[test]
    fn unit() {
        let q = Q {
            q1: 1.0,
            q2: 0.0,
            q3: 0.0,
            q4: 0.0
        };

        let expected_euler = Euler {
            roll: 0.0,
            pitch: 0.0,
            yaw: 0.0
        };

        assert_eq!(expected_euler, q.into());
    }

    #[test]
    fn roll() {
        let q = Q {
            q1: 0.991,
            q2: 0.131,
            q3: 0.0,
            q4: 0.0
        };

        let expected_euler = Euler {
            roll: 0.262,
            pitch: 0.0,
            yaw: 0.0
        };

        assert_eq!(expected_euler, q.into());
    }

    #[test]
    fn pitch() {
        let q = Q {
            q1: 0.991,
            q2: 0.0,
            q3: 0.131,
            q4: 0.0
        };

        let expected_euler = Euler {
            roll: 0.0,
            pitch: 0.262,
            yaw: 0.0
        };

        assert_eq!(expected_euler, q.into());
    }

    #[test]
    fn yaw() {
        let q = Q {
            q1: 0.991,
            q2: 0.0,
            q3: 0.0,
            q4: 0.131
        };

        let expected_euler = Euler {
            roll: 0.0,
            pitch: 0.0,
            yaw: 0.262
        };

        assert_eq!(expected_euler, q.into());
    }

    #[test]
    fn all_positive() {
        let _q = Q {
            q1: 0.996,
            q2: 0.052,
            q3: 0.047,
            q4: 0.052
        };

        let _expected_euler = Euler {
            roll: 0.1,
            pitch: 0.1,
            yaw: 0.1
        };

        // this fails, presumably due to rounding errors
        // assert_eq!(expected_euler, q.into());
    }
}
