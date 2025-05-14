use functions::{FunctionType, ackley};
use pso::{PSO, Result};

use std::{
    sync::{Arc, Barrier, Mutex},
    thread,
};

mod functions;
mod pso;

const DIM: usize = 2;
const NUM_PARTICLES: usize = 30;
const ITERATIONS: usize = 50000;

fn main() {
    let c = [1.4, 1.4]; // cognitive/social coefficients
    let domain = FunctionType::Ackley as usize as f64;

    // Initialize particles
    let mut particles = Vec::with_capacity(NUM_PARTICLES);
    for _ in 0..NUM_PARTICLES {
        particles.push(PSO::<DIM>::new(c, domain, ackley));
    }

    let global_best = Arc::new(Mutex::new(particles[0].local_best.clone()));
    let barrier = Arc::new(Barrier::new(NUM_PARTICLES));

    let mut handles = vec![];

    for mut particle in particles {
        let global_best = Arc::clone(&global_best);
        let barrier = Arc::clone(&barrier);

        let handle = thread::spawn(move || {
            for _ in 0..ITERATIONS {
                particle.update_local_best();

                // dbg!(particle.translation, particle.velocity, particle.local_best);

                // Global best update section
                {
                    let mut global = global_best.lock().unwrap();
                    if particle.local_best.value < global.value {
                        global.set_new(particle.local_best.clone());
                    }
                }

                barrier.wait(); // wait for all particles to finish updating global

                // Then update velocity and position
                let global = global_best.lock().unwrap().clone();
                particle.update_velocity(global);
                particle.update_translation();

                barrier.wait(); // wait for all particles to finish updating velocity and translation
            }

            // Optionally return the final result of this particle
            particle.local_best
        });

        handles.push(handle);
    }

    // Wait for all threads and get their results
    let mut best = Result::<DIM> {
        value: f64::INFINITY,
        translation: [0.0; DIM],
    };

    for handle in handles {
        let local_best = handle.join().unwrap();
        if local_best.value < best.value {
            best = local_best;
        }
    }

    let handle = global_best.lock().unwrap();

    println!("Global best value: {}", handle.value);
    println!("Global best position: {:?}", handle.translation);
}
