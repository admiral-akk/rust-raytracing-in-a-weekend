pub struct Rand {
    vals: Vec<f32>,
    units: Vec<Vec3>,
    index: usize,
    modulus: usize,
}

use rand::{thread_rng, Rng};

use crate::Vec3;

impl Rand {
    pub fn new(len: usize) -> Rand {
        let mut vals: Vec<f32> = Vec::new();
        let mut rng = thread_rng();
        for _i in 0..len {
            vals.push(rng.gen_range(0.0..=1.0));
        }
        let mut units: Vec<Vec3> = Vec::new();
        for _i in 0..len {
            units.push(Rand::precompute_unit());
        }
        Rand {
            vals: vals,
            units: units,
            index: 0,
            modulus: 1,
        }
    }

    fn precompute_unit() -> Vec3 {
        let mut rng = thread_rng();
        loop {
            let v = Vec3 {
                x: rng.gen_range(-1.0..1.0),
                y: rng.gen_range(-1.0..1.0),
                z: rng.gen_range(-1.0..1.0),
            };
            if v.length_squared() <= 1.0 {
                return v.normalized();
            }
        }
    }

    fn get_index(&mut self) -> usize {
        self.index += self.modulus;
        let index = self.index % self.vals.len();
        if index == 0 {
            self.modulus = (self.modulus % self.vals.len()) + 1;
        }
        return index;
    }

    pub fn rand(&mut self) -> f32 {
        let index = self.get_index();
        return self.vals[index];
    }

    pub fn random_range(&mut self, min: f32, max: f32) -> f32 {
        return min + (max - min) * self.rand();
    }

    pub fn random_unit(&mut self) -> Vec3 {
        let index = self.get_index();
        return self.units[index];
    }
}
