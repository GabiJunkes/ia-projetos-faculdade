use std::fs;

use rand::Rng;

#[derive(Debug)]
pub struct TSP {
    pub cities: Vec<City>,
}

impl TSP {
    pub fn new(file_path: &str) -> Self {
        let cnf = fs::read_to_string(file_path).expect("Should have been able to read the file");

        let lines: Vec<&str> = cnf.lines().filter(|x| x.len() > 3).collect();

        let mut tsp = Self {
            cities: Vec::new(),
        };

        tsp.build_cities(lines);
        
        tsp
    }

    fn build_cities(&mut self, lines: Vec<&str>) {
        for line in lines.iter() {
            let line = String::from(*line);
            if line.len() <= 4 {
                continue;
            }

            let vars: Vec<usize> = line
                .split_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .collect();

            let city = City {
                x: vars[1],
                y: vars[2],
            };

            self.cities.push(city);
        }
    }

    pub fn evaluate(&self, path: &TSPPath) -> f64 {
        let mut iter = path.cities.iter().cycle().peekable();
        let mut tsp_value = 0.0;
        loop {
            let current_city = iter.next().unwrap();
            let next_city = iter.peek().unwrap();

            tsp_value += current_city.distance(&next_city);

            if *next_city == path.cities.first().unwrap() {
                break;
            }
        }
        tsp_value
    }
}

#[derive(Debug, Clone)]
pub struct TSPPath {
    pub cities: Vec<City>,
}

impl TSPPath {
    pub fn new(cities: &Vec<City>) -> Self {
        let mut tspPath = TSPPath {
            cities: Vec::with_capacity(cities.len())
        };

        let mut rng = rand::rng();

        for _ in 0..cities.len() {    
            loop {
                let random_index = rng.random_range(0..cities.len());

                if let Ok(_) = tspPath.insert(cities.get(random_index).unwrap().clone()) {
                    break;
                }
            }
        }

        tspPath
    }

    pub fn insert(&mut self, city: City) -> Result<(), ()>{
        if self.cities.contains(&city) {
            return Err(());
        }
        
        self.cities.push(city);
        Ok(())
    }

    pub fn swap(&self, index_a: usize, index_b: usize) -> Self {
        let mut new_tsp_path = self.clone();
        
        new_tsp_path.cities.swap(index_a, index_b);
        new_tsp_path
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct City {
    x: usize,
    y: usize,
}

impl City {
    pub fn distance(&self, city: &City) -> f64 {
        ((((city.x as i64) - (self.x as i64)).pow(2) + ((city.y as i64) - (self.y as i64)).pow(2)) as f64).sqrt()
    }
}
