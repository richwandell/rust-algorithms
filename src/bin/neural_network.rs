extern crate ndarray;

use ndarray::{arr2, ArrayBase, Array2, OwnedRepr, Ix};
use ndarray::Ix2;
use ndarray_stats::QuantileExt;

struct NeuralNetwork {
    w1: Array2<f64>,
    w2: Array2<f64>
}

trait Works {
    fn new(input_layer_size: i32, hidden_layer_size: i32, output_layer_size: i32) -> NeuralNetwork;
    fn forward(&mut self, X: ArrayBase<OwnedRepr<f64>, Ix2>);
    fn train(&mut self, X: ArrayBase<OwnedRepr<f64>, Ix2>, y: ArrayBase<OwnedRepr<f64>, Ix2>);
    fn cost_function_prime(&mut self, X: Array2<f64>, y: Array2<f64>);
}

impl Works for NeuralNetwork {

    fn new(input_layer_size: i32, hidden_layer_size: i32, output_layer_size: i32) -> NeuralNetwork {
        let mut w1: Array2<f64> = Array2::<f64>::zeros((input_layer_size as usize, hidden_layer_size as usize));
        let mut w2: Array2<f64> = Array2::<f64>::zeros((hidden_layer_size as usize, output_layer_size as usize));
        NeuralNetwork{
            w1,
            w2
        }
    }

    fn forward(&mut self, X: ArrayBase<OwnedRepr<f64>, Ix2>) {
//        let new = &self.w1.dot(X);

//        println!("{:?}", new);
    }

    fn train(&mut self,  X: Array2<f64>, y: Array2<f64>) {
        let pcost = 9999999999999999.;

        loop {
            let cost_diff = 0.000000000001;
//            let grad = self.compute_gradients(X, y);
            if cost_diff <= 0.000000000001 {
                break
            }
        }
    }

    fn cost_function_prime(&mut self, X: Array2<f64>, y: Array2<f64>) {

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
    let x = training_x / max;
    let y = training_y / 100.;

    let mut n = NeuralNetwork::new(2, 3, 1);
    n.train(x, y);

    println!("{}", "hi");
}
