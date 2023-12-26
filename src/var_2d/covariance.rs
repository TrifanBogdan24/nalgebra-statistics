use crate::variables::{Variable1D, Variable2D};

impl Variable2D {
    pub fn covariance(&self) -> f64 {
        // calculates the covariance of two variables (vectors)
        // x = a vector representing the abcissas of the points (OX)
        // y = a vector representing the ordinates of the points (OY)

        if valid_parameter(self) == false {
            return 0.0 as f64;
        }

        let x: &Variable1D = &self.x;
        let y: &Variable1D = &self.y;

        let x_avg: f64 = x.average();
        let y_avg: f64 = y.average();

        let mut cov: f64 = 0.0 as f64;

        for i in 0..=(x.values.len() - 1) {
            cov = cov + (x.values[i] - x_avg) * (y.values[i] - y_avg)
        }

        cov = cov / (x.values.len() as f64);
        return cov;
    }
}


fn error_message() {
    println!("\nCode example of how to use it correctly: \n\
        let x: Vec<f64> = vec![1.0, 2.0, 3.0];\n\
        let y: Vec<f64> = vec![10.0, 20.0, 30.0];\n\
        let cov: f64 = covariance(&x, &y); \n"
    );
}

fn valid_parameter(var: &Variable2D) -> bool {
    let x: &Variable1D = &var.x;
    let y: &Variable1D = &var.y;

    if x.values.len() == 0 && y.values.len() == 0 {
        eprintln!("Err: empty arrays to compute the covariance.");
        error_message();
        return false;
    }

    if x.values.len() == 0 {
        eprintln!("Err: empty abscissa (OX) array to compute the covariance.");
        error_message();
        return false;
    }

    if y.values.len() == 0 {
        eprintln!("Err: empty ordinate (OY) array to compute the covariance.");
        error_message();
        return false;
    }

    if x.values.len() != y.values.len() {
        eprintln!("Err: the covariance cannot be computed for vectors with different lengths.");
        error_message();
        return false;
    }

    return true;
}
