use crate::variables::{Variable1D};

impl Variable1D {
    pub fn mean_standard_deviation(&self) -> f64 {
        // calculates the mean standard deviation

        if valid_parameter(self) == false {
            return 0.0 as f64;
        }

        let n: f64 = self.values.len() as f64;
        let mut sum: f64 = 0.0 as f64;
        let dev: Variable1D = self.deviation_from_mean();

        for val in &dev.values {
            sum = sum + val * val;
        }

        let dev_mean: f64 = (sum / (n * (n - 1.0))).sqrt();
        return dev_mean;
    }
}

fn error_message() {
    println!("\nCode example of how to use it correctly: \n\
    let x: Vec<f64> = vec![1.0, 2.0, 3.0];\n\
    let mean_dev: f64 = mean_standard_deviation(&x); \n"
    );
}

fn valid_parameter(var: &Variable1D) -> bool {

    if var.values.len() == 0 {
        eprintln!("Err: empty array to compute the mean standard deviation.");
        error_message();
        return false;
    }

    return true;
}
