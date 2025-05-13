use std::{fs::File, io::Write};
use std::sync::{Arc, Mutex};
use std::thread;

use rand::Rng;

use crate::sat::SAT;

fn generate_neighbor(vector: &Vec<bool>) -> Vec<bool> {
    let mut rng = rand::rng();

    let mut neighbor_vector = vector.clone();

    // change 10% of variables
    for _ in 0..(vector.len() as f32 * 0.01).ceil() as i32 {
        let index = rng.random_range(0..vector.len());
        let elem = neighbor_vector.get_mut(index).unwrap();
        *elem = !*elem;
    }

    neighbor_vector
}

pub fn run(sat: &SAT, sa_max: usize, starting_temp: f32, vector: Vec<bool>, index: usize) -> Vec<bool> {
    let mut rng = rand::rng();
    let mut best_vector = vector.clone();
    let mut best_result = sat.evaluate(&best_vector);

    let mut temp = starting_temp;

    let mut current_vector = vector;
    let mut current_result = sat.evaluate(&current_vector);

    let mut log_data: Vec<String> = Vec::new();
    let mut iter = 0;
    let mut generation = 0;

    log_data.push(format!("{},{}", generation, best_result.false_count));

    let max_generation = 8000;

    while generation < max_generation {

        while iter < sa_max {
            iter += 1;
            let neighbor_vector = generate_neighbor(&current_vector);
            let neighbor_result = sat.evaluate(&neighbor_vector);

            let delta = neighbor_result.false_count as i32 - current_result.false_count as i32;

            if delta < 0 {
                current_vector = neighbor_vector.clone();
                current_result = neighbor_result;

                if current_result.false_count < best_result.false_count {
                    best_vector = current_vector.clone();
                    best_result = current_result;
                }
            } else {
                let x = rng.random_range(0.0..1.0);
                let p = (-delta as f32 / temp).exp();
                if x < p {
                    current_vector = neighbor_vector.clone();
                    current_result = neighbor_result;
                }
            }
            log_data.push(format!("{},{},{}", iter + sa_max * generation, current_result.false_count, temp));
        }

        // log_data.push(format!("{},{},{}", generation, best_result.false_count, temp));

        temp = temp * 0.995;
        
        iter = 0;
        generation +=1;
    }

    let mut file = File::create(format!("data/convergence_data_{}_{}_5.csv", sat.vars_count, index)).expect("Failed to create file");
    file.write_all(b"Generation,False Clauses,Temperature\n").unwrap();
    for line in log_data {
        file.write_all(line.as_bytes()).unwrap();
        file.write_all(b"\n").unwrap();
    }

    return best_vector;
}

pub fn run_multiple_threads(sat: Arc<SAT>, sa_max: usize, starting_temp: f32, iterations: usize) {
    let results: Arc<Mutex<Vec<(usize, u32)>>> = Arc::new(Mutex::new(Vec::new()));

    let mut handles = Vec::new();

    for run_id in 0..iterations {
        let sat_clone = Arc::clone(&sat);
        let results_clone = Arc::clone(&results);

        let handle = thread::spawn(move || {
            let mut rng = rand::rng();
            let vector: Vec<bool> = (0..sat_clone.vars_count).map(|_| rng.random::<bool>()).collect();

            let result_vector = run(&sat_clone, sa_max, starting_temp, vector, run_id);
            let final_result = sat_clone.evaluate(&result_vector);

            let mut results = results_clone.lock().unwrap();
            results.push((run_id + 1, final_result.false_count));
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let mut file = File::create(format!("data/boxplot_data_{}_10.csv", sat.vars_count)).expect("Failed to create file");
    file.write_all(b"Run,False Clauses\n").unwrap();

    let results = results.lock().unwrap();
    for (run_id, true_count) in results.iter() {
        file.write_all(format!("{},{}\n", run_id, true_count).as_bytes()).unwrap();
    }
}