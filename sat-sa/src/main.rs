mod sa;
mod sat;

use rand::Rng;
use sa::{run, run_multiple_threads};
use sat::SAT;

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
    let sa_max = 1;
    let starting_temp = 10000.0;
    let iterations = 30;

    let files = [
        "cnfs/uf20-01.cnf",
        "cnfs/uf100-01.cnf",
        "cnfs/uf250-01.cnf",
    ];


    for (index, file_path) in files.iter().enumerate() {
        println!("Running {}/{}", index+1, files.len());
        let sat = SAT::new(file_path);
        if false {
            let mut vector: Vec<bool> = Vec::with_capacity(sat.vars_count);
    
            let mut rng = rand::rng();
            for _ in 0..sat.vars_count {
                vector.push(rng.random::<bool>());
            }
    
            let result_vector = run(&sat, sa_max, starting_temp, vector, 1);
    
            let result = sat.evaluate(&result_vector);
    
            dbg!(result);
        } else {
            run_multiple_threads(sat.into(), sa_max, starting_temp, iterations);
        }

    }
}
