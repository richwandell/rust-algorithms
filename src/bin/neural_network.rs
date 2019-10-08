extern crate ndarray;

use ndarray::{arr2, ArrayBase, Array2, OwnedRepr, Ix};
use ndarray::Ix2;
use ndarray_stats::QuantileExt;
use ndarray_rand::RandomExt;
use ndarray_rand::rand_distr::Uniform;

struct NeuralNetwork {
    W1: Array2<f64>,
    W2: Array2<f64>,
//    y_hat: Array2<f64>,
//    Z2: Array2<f64>,
//    A2: Array2<f64>
}

trait Works {
    fn new(input_layer_size: i32, hidden_layer_size: i32, output_layer_size: i32) -> NeuralNetwork;
    fn forward(&mut self, X: ArrayBase<OwnedRepr<f64>, Ix2>);
    fn train(&mut self, X: ArrayBase<OwnedRepr<f64>, Ix2>, y: ArrayBase<OwnedRepr<f64>, Ix2>);
    fn cost_function_prime(&mut self, X: Array2<f64>, y: Array2<f64>);
    fn compute_gradients(&mut self, X: Array2<f64>, y: Array2<f64>);
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
        NeuralNetwork{
            W1,
            W2
        }
    }

    fn forward(&mut self, X: ArrayBase<OwnedRepr<f64>, Ix2>) {
//        self.Z2 = math.multiply(X, this.W1);
//        self.Z2 = math.add(1, this.Z2);
//        self.A2 = this.sigmoid(this.Z2);
//        this.A2 = math.add(1, this.A2);
//        this.Z3 = math.multiply(this.A2, this.W2);
//        this.Z3 = math.add(1, this.Z3);
//        var yHat = this.sigmoid(this.Z3);
//        return yHat;
    }

    fn train(&mut self,  &mut X: Array2<f64>, &mut y: Array2<f64>) {
        let alpha = 0.02;
        let pcost = 9999999999999999.;
        let cost_diff = pcost;
        let momentum = 0.9;
        loop {
            let grad = self.compute_gradients(X, y);
            if cost_diff <= 0.000000000001 {
                break
            }
        }
    }

    fn compute_gradients(&mut self, X: Array2<f64>, y: Array2<f64>) {
        let djdwa = self.cost_function_prime(X, y);
        return djdwa;
    }

    fn cost_function_prime(&mut self, X: Array2<f64>, y: Array2<f64>) {
//        self.y_hat = self.forward(X);
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
    n.train(X, y);



    println!("{}", "hi");
}
