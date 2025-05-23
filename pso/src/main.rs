use functions::{ackley, griewank, FunctionType};
use pso::{PSO, Result};

use std::{
    fs::File, io::Write, sync::{Arc, Barrier, Mutex}, thread
};

mod functions;
mod pso;

const DIM: usize = 10;
const NUM_PARTICLES: usize = 30;
const ITERATIONS: usize = 500;
const W: f64 = 0.9;

// considerar PSOs, PSOw, PSOk
// menor q 10e-10 = 0
// resolver para 5 e 10
// Considerar 10 runs para media e desvio-padrao e boxplot.
// trazer graficos de convergencia
// 30 particulas

fn main() {
    let c = [2.05, 2.05]; // cognitive/social coefficients
    let domain = FunctionType::Ackley as usize as f64;

    // Initialize particles
    let mut particles = Vec::with_capacity(NUM_PARTICLES);
    for _ in 0..NUM_PARTICLES {
        particles.push(PSO::<DIM>::new(c, W, domain, ackley));
    }

    let global_best = Arc::new(Mutex::new(particles[0].local_best.clone()));
    let barrier = Arc::new(Barrier::new(NUM_PARTICLES));

    let mut handles = vec![];

    let mut count = 0;
    for mut particle in particles {
        count += 1;
        let global_best = Arc::clone(&global_best);
        let barrier = Arc::clone(&barrier);

        let handle = thread::spawn(move || {
            let index = count;
            let mut history = Vec::new();

            for i in 0..ITERATIONS {
                particle.update_local_best();

                // dbg!(index, particle.translation, particle.velocity, "  ---- ");

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

                if index == 1 {
                  // dbg!(global.value);
                  history.push(global.value);
                }

                barrier.wait(); // wait for all particles to finish updating velocity and translation
            }

            // Optionally return the final result of this particle
            (particle.local_best, history)
        });

        handles.push(handle);
    }

    for handle in handles {
        let result = handle.join().unwrap();
        if result.1.len() > 1 {
          let mut file = File::create(format!("data/data.csv")).expect("Failed to create file");
          file.write_all(b"Geracao,BestValue\n").unwrap();
          let mut content = String::new();
          for (i, value) in result.1.iter().enumerate() {
            content.push_str(format!("{},{}\n", i, value).as_str());
          }
          file.write_all(content.as_bytes()).unwrap();

        }
    }
    


    let mut handle = global_best.lock().unwrap();

    if handle.value <= 1e-10 {
        handle.value = 0.0;
    }

    println!("Global best value: {}", handle.value);
    println!("Global best position: {:?}", handle.translation);
}
