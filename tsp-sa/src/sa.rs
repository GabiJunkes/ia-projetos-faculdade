use std::{fs::File, io::Write};
use std::sync::{Arc, Mutex};
use std::thread;

use rand::Rng;

use crate::tsp::{TSPPath, TSP};

fn generate_neighbor(tsp_path: &TSPPath) -> TSPPath {
    let mut rng = rand::rng();
    
    let mut tsp_path = tsp_path.clone();

    for _ in 0..(tsp_path.cities.len() as f32 *0.01).ceil() as i32 {
      let (index_a, index_b) = loop {
          let index_a = rng.random_range(0..tsp_path.cities.len());
          let index_b = rng.random_range(0..tsp_path.cities.len());
  
          if index_a != index_b {
              break (index_a, index_b);
          }
      };
      tsp_path = tsp_path.swap(index_a, index_b);
    }

    tsp_path
}

pub fn run(tsp: &TSP, sa_max: usize, starting_temp: f32, vector: TSPPath, index: usize) -> TSPPath {
    let mut rng = rand::rng();
    let mut best_vector = vector.clone();
    let mut best_result = tsp.evaluate(&best_vector);

    let mut temp = starting_temp;

    let mut current_vector = vector;
    let mut current_result = tsp.evaluate(&current_vector);

    let mut log_data: Vec<String> = Vec::new();
    let mut iter = 0;
    let mut generation = 0;

    log_data.push(format!("{},{}", generation, best_result));

    let max_generation = 15000;

    while generation < max_generation {

        while iter < sa_max {
            iter += 1;
            let neighbor_vector = generate_neighbor(&current_vector);
            let neighbor_result = tsp.evaluate(&neighbor_vector);

            let delta = neighbor_result - current_result;

            if delta < 0.0 {
                current_vector = neighbor_vector.clone();
                current_result = neighbor_result;

                if current_result < best_result {
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
            log_data.push(format!("{},{},{}", iter + sa_max * generation, current_result, temp));
        }

        // log_data.push(format!("{},{},{}", generation, best_result, temp));

        temp = temp * 0.999;
        // temp = starting_temp / (1.0 + (generation as f32).ln());
        // temp = starting_temp * (1.0 - (generation as f32 / max_generation as f32));
        
        iter = 0;
        generation +=1;
    }

    let mut file = File::create(format!("data/convergence_data_{}_{}.csv", tsp.cities.len(), index)).expect("Failed to create file");
    file.write_all(b"Generation,False Clauses,Temperature\n").unwrap();
    for line in log_data {
        file.write_all(line.as_bytes()).unwrap();
        file.write_all(b"\n").unwrap();
    }

    return best_vector;
}

pub fn run_multiple_threads(tsp: Arc<TSP>, sa_max: usize, starting_temp: f32, iterations: usize) {
    let results: Arc<Mutex<Vec<(usize, f64)>>> = Arc::new(Mutex::new(Vec::new()));

    let mut handles = Vec::new();

    for run_id in 0..iterations {
        let tsp_clone = Arc::clone(&tsp);
        let results_clone = Arc::clone(&results);

        let handle = thread::spawn(move || {
            let tsp_path = TSPPath::new(&tsp_clone.cities);

            let result_vector = run(&tsp_clone, sa_max, starting_temp, tsp_path, run_id);
            let final_result = tsp_clone.evaluate(&result_vector);

            let mut results = results_clone.lock().unwrap();
            results.push((run_id + 1, final_result));
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let mut file = File::create(format!("data/boxplot_data_{}_10.csv", tsp.cities.len())).expect("Failed to create file");
    file.write_all(b"Run,False Clauses\n").unwrap();

    let results = results.lock().unwrap();
    for (run_id, true_count) in results.iter() {
        file.write_all(format!("{},{}\n", run_id, true_count).as_bytes()).unwrap();
    }
}