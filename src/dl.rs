use ndarray::Array2;

// sigmoid activation function
pub fn sigmoid(x: &Array2<f64>) -> Array2<f64> {
    1.0 / (1.0 + x.map(|y| (-y).exp()))
}

// derivative of sigmoid activation function
pub fn deriv_sigmoid(x: &Array2<f64> ) -> Array2<f64> {
    let sig: Array2<f64> = sigmoid(&x);
    &sig * (1.0 - &sig)
}


// relu activation function
pub fn relu(x: &Array2<f64>) -> Array2<f64> {
    x.mapv(|y| y.max(0.0))
}

// derivative of relu activation function
pub fn deriv_relu(x: &Array2<f64>) -> Array2<f64> {
    x.map(|y| if *y <= 0.0 { 0.0} else { 1.0 })
}