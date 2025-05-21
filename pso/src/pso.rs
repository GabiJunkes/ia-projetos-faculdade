use std::sync::Arc;

use rand::{distr::{Distribution, Uniform}, Rng};


pub struct PSO<const DIM: usize> {
    pub translation: [f64; DIM],
    pub velocity: [f64; DIM],
    pub local_best: Result::<DIM>,
    c: [f64; 2],
    k: f64,
    w: f64,
    func_domain: f64,
    func: Arc<dyn Fn(Vec<f64>) -> f64 + 'static + Sync + Send>,
}

#[derive(Clone, Debug, Copy)]
pub struct Result<const DIM: usize> {
    pub value: f64,
    pub translation: [f64; DIM],
}

impl <const DIM: usize> Result<DIM> {
    pub fn set_new(&mut self, result: Result<DIM>) {
        if self.value > result.value {
            self.value = result.value;
            self.translation = result.translation;
        }
    }
}

impl <const DIM: usize> PSO<DIM> {
    pub fn new(c: [f64; 2], w: f64, func_domain: f64, func: impl Fn(Vec<f64>) -> f64 + 'static + Sync + Send) -> Self {
        let range = -func_domain..func_domain;
        let range_div = -func_domain / 8.0..func_domain / 8.0;

        let between_translation = Uniform::try_from(range.clone()).unwrap();
        let between_velocity = Uniform::try_from(range_div).unwrap();
        let mut rng = rand::rng();

        let mut translation = [0.0; DIM];
        let mut velocity = [0.0; DIM];

        for i in 0..DIM {
            translation[i] = between_translation.sample(&mut rng);
            velocity[i] = between_velocity.sample(&mut rng);
        }

        let local_best = Result {
            translation,
            value: f64::INFINITY,
        };

        let phi = c[0] + c[1];

        let k = 2.0 / (2.0 - phi - (phi.powi(2) - 4.0 * phi).sqrt()).abs();

        let mut pso = PSO::<DIM> {
            translation,
            velocity,
            local_best,
            c,
            k,
            w,
            func_domain,
            func: Arc::new(func),
        };

        pso.local_best.value = pso.evaluate();

        pso
    }

    pub fn update_local_best(&mut self) {
        let current = self.evaluate();

        if current < self.local_best.value {
            self.local_best.value = current;
            self.local_best.translation = self.translation;
        }
    }

    pub fn update_velocity(&mut self, global_best: Result<DIM>) {
        for i in 0..DIM {
            if self.translation[i] < -(self.func_domain) || self.translation[i] > self.func_domain {
                self.velocity[i] = 0.0;
                continue;
            }

            let mut r: [[f64; DIM]; 2] = [[0.0; DIM]; 2];
            let mut rng = rand::rng();
    
            for i in 0..DIM {    
                r[0][i] = rng.random_range(0.0..=1.0);
                r[1][i] = rng.random_range(0.0..=1.0);
            }

            let cognitive = self.c[0] * r[0][i] * (self.local_best.translation[i] - self.translation[i]);
            let social = self.c[1] * r[1][i] * (global_best.translation[i] - self.translation[i]);
            self.velocity[i] = self.k * (self.velocity[i] * self.w + cognitive + social);
        }
    }

    pub fn update_translation(&mut self) {
        for i in 0..DIM {
            let value =  self.translation[i] + self.velocity[i];

            if value > self.func_domain {
                self.translation[i] = self.func_domain;
            }else if value < -self.func_domain {
                self.translation[i] = -self.func_domain;
            }else {
                self.translation[i] = value;
            }
        }
    }

    pub fn evaluate(&mut self) -> f64 {
        (self.func)(self.translation.to_vec())
    }
}