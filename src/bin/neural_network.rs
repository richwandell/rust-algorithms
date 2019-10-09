extern crate ndarray;

use ndarray::{arr2, ArrayBase, Array2, OwnedRepr, Ix};
use ndarray::Ix2;
use ndarray_stats::QuantileExt;
use ndarray_rand::RandomExt;
use ndarray_rand::rand_distr::Uniform;

struct NeuralNetwork {
    W1: Array2<f64>,
    W2: Array2<f64>,
    y_hat: Array2<f64>,
    Z2: Array2<f64>,
    Z3: Array2<f64>,
    A2: Array2<f64>
}

trait Works {
    fn new(input_layer_size: i32, hidden_layer_size: i32, output_layer_size: i32) -> NeuralNetwork;
    fn forward(&mut self, X: &Array2<f64>) -> Array2<f64>;
    fn train(&mut self, X: &Array2<f64>, y: &Array2<f64>);
    fn cost_function_prime(&mut self, X: &Array2<f64>, y: &Array2<f64>);
    fn compute_gradients(&mut self, X: &Array2<f64>, y: &Array2<f64>);
    fn sigmoid(&mut self, z: &Array2<f64>) -> Array2<f64>;
}

impl Works for NeuralNetwork {

    fn new(input_layer_size: i32, hidden_layer_size: i32, output_layer_size: i32) -> NeuralNetwork {
        let mut W1: Array2<f64> = Array2::<f64>::random(
            (input_layer_size as usize, hidden_layer_size as usize),
            Uniform::new(0., 1.)
        );
        let mut W2: Array2<f64> = Array2::<f64>::random(
            (hidden_layer_size as usize, output_layer_size as usize),
            Uniform::new(0., 1.)
        );
        let y_hat: Array2<f64> = Array2::<f64>::random(
            (hidden_layer_size as usize, output_layer_size as usize),
            Uniform::new(0., 1.)
        );
        let mut Z2: Array2<f64> = Array2::<f64>::random(
            (hidden_layer_size as usize, output_layer_size as usize),
            Uniform::new(0., 1.)
        );
        let mut Z3: Array2<f64> = Array2::<f64>::random(
            (hidden_layer_size as usize, output_layer_size as usize),
            Uniform::new(0., 1.)
        );
        let mut A2: Array2<f64> = Array2::<f64>::random(
            (hidden_layer_size as usize, output_layer_size as usize),
            Uniform::new(0., 1.)
        );
        NeuralNetwork{
            W1,
            W2,
            y_hat,
            Z2,
            Z3,
            A2
        }
    }

    fn forward(&mut self, X: &Array2<f64>) -> Array2<f64> {
        let mut Z2 = X.dot(&self.W1);
        Z2 = Z2 + 1.;
        let mut A2 = self.sigmoid(&Z2);
        A2 = A2 + 1.;
        self.A2 = A2;
        let mut Z3 = self.A2.dot(&self.W2);
        Z3 = Z3 + 1.;
        let y_hat = self.sigmoid(&Z3);
        self.Z2 = Z2;
        self.Z3 = Z3;
        return y_hat;
    }

    fn sigmoid(&mut self, z: &Array2<f64>) -> Array2<f64> {
        let mut bottom = z * -1.;
        bottom.mapv(|a| {
            1. / (1. + std::f64::consts::E.powf(a))
        })
    }

    fn train(&mut self, X: &Array2<f64>, y: &Array2<f64>) {
        let alpha = 0.02;
        let pcost = 9999999999999999.;
        let cost_diff = pcost;
        let momentum = 0.9;

        loop {
            self.compute_gradients(&X, &y);
//            let grad = self.compute_gradients(X, y);
            if cost_diff <= 0.000000000001 {
                break
            }
        }
    }

    fn compute_gradients(&mut self, X: &Array2<f64>, y: &Array2<f64>) {
        self.cost_function_prime(&X, &y);
//        return djdwa;
    }

    fn cost_function_prime(&mut self, X: &Array2<f64>, y: &Array2<f64>) {
        self.y_hat = self.forward(&X);
    }
}

fn main () {
    //training x values
    let training_x: Array2<f64> = arr2(&[
        [3., 5.],
        [5., 1.],
        [10., 2.],
        [1., 12.],
        [1., 10.]
    ]);
    //training y values
    let training_y: Array2<f64> = arr2(&[[75.], [82.], [93.], [10.], [10.]]);

    //normalize
    let max: f64 = *training_x.max().unwrap();
    let X = training_x / max;
    let y = training_y / 100.;

    let mut n = NeuralNetwork::new(2, 3, 1);
    n.train(&X, &y);



    println!("{}", "hi");
}
