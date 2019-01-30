#![no_std]

#[derive(Clone, Copy)]
#[repr(C)]
pub struct V {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Q {
    pub q1: f32,
    pub q2: f32,
    pub q3: f32,
    pub q4: f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    extern {
        fn hello(c: i32) -> i32;
        fn filterUpdate(w: V, a: V, q: Q) -> Q;
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + unsafe { hello(2) }, 6);
    }

    #[test]
    fn it_works2() {
        let w = V {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };
        let a = V {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };
        let q_init = Q {
            q1: 1.0,
            q2: 2.0,
            q3: 3.0,
            q4: 4.0
        };
        let result = unsafe { filterUpdate(w, a, q_init) };
        assert_eq!(result.q1, 1.0);
    }
}
