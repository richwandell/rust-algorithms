extern crate ndarray;

use std::ops::Mul;

use ndarray::{arr2, Array, Array2, ArrayBase, Ix, OwnedRepr};
use ndarray::Ix2;
use ndarray_rand::rand_distr::Uniform;
use ndarray_rand::RandomExt;
use ndarray_stats::QuantileExt;

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
    fn cost_function_prime(&mut self, X: &Array2<f64>, y: &Array2<f64>) -> (Array2<f64>, Array2<f64>);
    fn cost_function(&mut self, X: &Array2<f64>, y: &Array2<f64>) -> f64;
    fn sigmoid(&mut self, z: &Array2<f64>) -> Array2<f64>;
    fn sigmoid_prime(&mut self, z: &Array2<f64>) -> Array2<f64>;
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

    fn sigmoid_prime(&mut self, z: &Array2<f64>) -> Array2<f64> {
        let sig = self.sigmoid(z);
        let ones = Array2::<f64>::ones(sig.shape());
        let one_minus = ones.sub(&sig);
        let mul = &sig * one_minus;
        return mul;
    }

    fn train(&mut self, X: &Array2<f64>, y: &Array2<f64>) {
        let alpha = 0.02;
        let pcost = 9999999999999999.;
        let mut cost_diff = pcost;
        let momentum = 0.9;
        let learning_rate = 0.3;


        loop {

            let s = Array::random((1, X.shape()[0]), Uniform::new(0., X.shape()[0]));

            for n in 0..s.len() as i32 {
                let i = s[n as usize];
                let x = X.row(i as usize).reshape((1, X.shape()[1]));
                let yt = y.row(i as usize);
                let (djdw1, djdw2) = self.cost_function_prime(&x, &yt);
                let d1 = djdw1 * alpha;
                self.W1 = &self.W1 - d1;
                let d2 = djdw2 * alpha;
                self.W2 = &self.W2 - d2;
            }

            let cost = self.cost_function();
            cost_diff = pcost - cost;

            if cost_diff <= 0.0001 {
                break
            }
        }
    }

    fn cost_function(&mut self, X: &Array2<f64>, y: &Array2<f64>) -> f64 {
        self.y_hat = self.forward(X);
        let ymyhat = y - &self.y_hat;
        let squared_summed = ymyhat.powi(2).sum();

        let mut J = 1. / X.shape()[1] as f64;
        J = J.mul(squared_summed);
        return J;
    }

    fn cost_function_prime(&mut self, X: &Array2<f64>, y: &Array2<f64>) -> (Array2<f64>, Array2<f64>) {
        let mut y_hat = self.forward(&X);
        self.y_hat = y_hat;

        let ymyh = y - &y_hat;
        let rsub = -1 * ymyh;

        let mut delta3 = self.sigmoid_prime(&self.Z3);
        delta3 = rsub.mul(delta3);

        let djdw2 = self.A2.t().dot(&delta3);
        let d3mw2 = delta3.dot(&self.W2.t());

        let sig = self.sigmoid_prime(&self.Z2);
        let delta2 = d3mw2 * sig;

        let djdw1 = X.t().dot(delta2);

        return (djdw1, djdw2);
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
