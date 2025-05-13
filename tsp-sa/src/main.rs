mod sa;
mod tsp;

use sa::{run, run_multiple_threads};
use tsp::{TSPPath, TSP};

// 3-SAT SA.
// Obj: Max ou Min
// Vetor de solução: vetor binario
// S.A: Unica trajetoria, melhoria
// Funcao Obj: Quantidade de clausulas
// Ler funcao booleanas
// Vizinho: bit-flip
// Definicao da queda de temperatura
// multiplas execucoes: 30 vzs (boxplot)

fn main() {
    let sa_max = 20;
    let starting_temp = 30000.0;
    let iterations = 30;

    let files = [
        "cnfs/eil51-tsp.cnf",
        "cnfs/kroA100-tsp.cnf",
    ];

    // 426
    // 21282


    for (index, file_path) in files.iter().enumerate() {
        println!("Running {}/{}", index+1, files.len());
        let tsp = TSP::new(file_path);
        if false {
            let tsp_path = TSPPath::new(&tsp.cities);
    
            let result_vector = run(&tsp, sa_max, starting_temp, tsp_path, 1);
    
            let result = tsp.evaluate(&result_vector);
    
            dbg!(result);
        } else {
            run_multiple_threads(tsp.into(), sa_max, starting_temp, iterations);
        }

    }
}
