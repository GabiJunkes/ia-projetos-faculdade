//https://www.sfu.ca/~ssurjano/griewank.html
//Griewank
// top=0;
// top1=0;
// top2=1;
// for (j=0;j<DIM;j++) {
//     top1=top1+pow((sol[j]),(double)2);
//     top2=top2*cos((((sol[j])/sqrt((double)(j+1)))*M_PI)/180);
// }
// top=(1/(double)4000)*top1-top2+1;

// return top;

#[derive(Copy, Clone)]
pub enum FunctionType {
    Griewank = 600,
    Ackley = 32,
}

use std::f64::consts::PI;

pub fn griewank(params: Vec<f64>) -> f64 {
    let mut top_1 = 0.0;
    let mut top_2 = 1.0;

    for j in 0..params.len() {
        // top1 = top1 + pow( (sol[j]), 2 );
        top_1 = top_1 + params[j].powi(2);

        // top2 = top2 * cos( ( (sol[j] / sqrt(j+1)) * M_PI ) / 180 );
        top_2 = top_2 * ( (((params[j] / (j as f64+1.0).sqrt()) * PI)) / 180.0 ).cos();
    }

    // top=(1/(double)4000)*top1-top2+1;
    (1.0 / 4000.0) * top_1 - top_2 + 1.0
}

// https://www.sfu.ca/~ssurjano/ackley.html
// Ackley
/*
-       Dimension: n arbitrary
-       Domain:   -32 <= | x_i | <= 32.0
-       Minimum 0 at x_i = 0.0
*/
// for (i = 0; i < DIM; i++) {
//     aux += sol[i]*sol[i];
// }
// for (i = 0; i < DIM; i++){
//     aux1 += cos(2.0*M_PI*sol[i]);
// }

// return (-20.0*(exp(-0.2*sqrt(1.0/(float)DIM*aux)))-exp(1.0/(float)DIM*aux1)+20.0+exp(1));

pub fn ackley(params: Vec<f64>) -> f64 {
    let mut aux = 0.0;
    let mut aux_1 = 0.0;

    let dim = params.len();

    for i in 0..dim {
        // aux += sol[i]*sol[i];
        aux += params[i]*params[i];

        // aux1 += cos(2.0*M_PI*sol[i]);
        aux_1 += (2.0 * PI * params[i]).cos()
    }

    // (-20.0 * ( exp( -0.2 * sqrt( 1.0/ DIM * aux ) ) ) - exp( 1.0 / DIM * aux1 ) + 20.0 + exp(1) );
    -20.0 * ( -0.2 * (1.0 / dim as f64 * aux).sqrt() ).exp() - (1.0 / dim as f64 * aux_1).exp() + 20.0 + 1.0_f64.exp()
}

pub fn ackley_iter(params: Vec<f64>) -> f64 {
    let dim = params.len();
    let aux: f64 = params.iter().map(|x| x.powi(2)).sum();
    let aux_1: f64 = params.iter().map(|x| (2.0*PI * *x).cos()).sum();


    -20.0 * (-0.2 * ((1.0 / dim as f64) * aux).sqrt()).exp()
        - ((1.0 / dim as f64) * aux_1).exp()
        + 20.0
        + 1.0_f64.exp()
}