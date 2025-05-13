use std::fs;

#[derive(Debug)]
pub struct SAT {
    pub vars_count: usize,
    pub clauses: Vec<Clause>,
}

#[derive(Debug, Clone, Copy)]
pub struct EvaluationResult {
    pub true_count: u32,
    pub false_count: u32,
}

impl SAT {
    pub fn evaluate(&self, result_vector: &Vec<bool>) -> EvaluationResult {
        let mut result = EvaluationResult {
            true_count: 0,
            false_count: 0,
        };

        for clause in self.clauses.iter() {
            if clause.evaluate(result_vector) {
                result.true_count += 1;
            }else {
                result.false_count += 1;
            }
        }

        result
    }

    pub fn new(file_path: &str) -> Self {
        let cnf = fs::read_to_string(file_path).expect("Should have been able to read the file");

        let mut lines: Vec<&str> = cnf.lines().filter(|x| x.len() > 3).collect();

        let first_line = lines.remove(0);

        let config: Vec<&str> = first_line.split_whitespace().collect();

        let (vars_count, clause_number) = (config[2].parse().unwrap(), config[3].parse().unwrap());

        let mut sat = Self {
            clauses: Vec::with_capacity(clause_number),
            vars_count,
        };

        sat.build_clauses(lines);
        
        sat
    }

    fn build_clauses(&mut self, lines: Vec<&str>) {
        for line in lines.iter() {
            let line = String::from(*line);
            if line.len() <= 4 {
                continue;
            }

            let vars: Vec<i32> = line
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect();

            self.clauses.push(Clause {
                first: Var::new(vars[0]),
                second: Var::new(vars[1]),
                third: Var::new(vars[2]),
            });
        }
    }
}

#[derive(Debug)]
pub struct Clause {
    first: Var,
    second: Var,
    third: Var,
}

impl Clause {
    pub fn evaluate(&self, vector: &Vec<bool>) -> bool {
        self.first.evaluate(vector)
            || self.second.evaluate(vector)
            || self.third.evaluate(vector)
    }
}

#[derive(Debug)]
struct Var {
    pub literal: bool,
    pub index: usize,
}

impl Var {
    pub fn new(value: i32) -> Self {
        Self {
            literal: value > 0,
            index: value.abs() as usize - 1,
        }
    }

    pub fn evaluate(&self, vector: &Vec<bool>) -> bool {
        if self.literal {
            vector[self.index]
        } else {
            !vector[self.index]
        }
    }
}