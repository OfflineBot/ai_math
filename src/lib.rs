use ndarray::Array1;
use std::f64::consts::E;

pub mod dl;
pub mod rl;

pub fn softmax(x: &Vec<f64>) -> Vec<f64> {
    let mut e: Vec<f64> = x.clone().iter_mut().map(|y| E.powf(*y)).collect();
    let sum: f64 = e.iter().sum();
    e = e.iter_mut().map(|z| *z / sum).collect();
    e
}

pub fn nd_softmax(x: &Array1<f64>) -> Array1<f64> {
    let mut e: Array1<f64> = x.mapv(|item| E.powf(item));
    let sum: f64 = e.sum();
    e = e.mapv(|item| item / sum);
    e
}