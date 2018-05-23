extern crate rusty_machine;
use rusty_machine::linalg::{Matrix,Vector};
use rusty_machine::learning::gp::{GaussianProcess,ConstMean};
use rusty_machine::learning::toolkit::kernel;
use rusty_machine::learning::SupModel;

fn main() {
    let inputs = Matrix::new(3,3,vec![1.1,1.2,1.3,2.1,2.2,2.3,3.1,3.2,3.3]);
    let targets = Vector::new(vec![0.1,0.8,0.3]);

    let test_inputs = Matrix::new(2,3, vec![1.2,1.3,1.4,2.2,2.3,2.4]);

    let ker = kernel::SquaredExp::new(2., 1.);
    let zero_mean = ConstMean::default();
    let mut gp = GaussianProcess::new(ker, zero_mean, 0.5);

    gp.train(&inputs, &targets).unwrap();
    let _ = gp.predict(&test_inputs).unwrap();
}
