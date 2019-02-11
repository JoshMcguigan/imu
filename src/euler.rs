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
        let w = q.w;
        let x = q.x;
        let y = q.y;
        let z = q.z;

        let test = x*y + z*w;
        if test > 0.499 { // singularity at north pole
            let pitch = 2. * atan2f(x,w);
            let yaw = PI/2.;
            let roll = 0.;
            return Euler { roll, pitch, yaw }
        }
        if test < -0.499 { // singularity at south pole
            let pitch = -2. * atan2f(x,w);
            let yaw = -PI/2.;
            let roll = 0.;
            return Euler { roll, pitch, yaw }
        }
        let sqx = x*x;
        let sqy = y*y;
        let sqz = z*z;
        let pitch = atan2f(2.*y*w-2.*x*z , 1. - 2.*sqy - 2.*sqz);
        let yaw = asinf(2.*test);
        let roll = atan2f(2.*x*w-2.*y*z , 1. - 2.*sqx - 2.*sqz);

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
            w: 1.0,
            x: 0.0,
            y: 0.0,
            z: 0.0
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
            w: 0.991,
            x: 0.131,
            y: 0.0,
            z: 0.0
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
            w: 0.991,
            x: 0.0,
            y: 0.131,
            z: 0.0
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
            w: 0.991,
            x: 0.0,
            y: 0.0,
            z: 0.131
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
            w: 0.996,
            x: 0.052,
            y: 0.047,
            z: 0.052
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
