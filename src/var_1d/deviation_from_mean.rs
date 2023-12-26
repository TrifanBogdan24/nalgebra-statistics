use crate::variables::{Variable1D};

fn error_message() {
    println!("\nCode example of how to use it correctly.`: \n\
    let x: Vec<f64> = vec![1.0, 2.0, 3.0];\n\
    let deviation: Vec<f64> = deviation_from_mean(&x); \n"
    );
}

fn valid_parameter(var: &Variable1D) -> bool {
    if var.values.len() == 0 {
        eprintln!("Err: empty array to compute the deviation of each element from the mean value.");
        error_message();
        return false;
    }

    return true;
}


impl Variable1D {
    pub fn deviation_from_mean(&self) -> Self {
        // calculates the difference between each value of the vector and the mean

        let mut dev = Variable1D {
            name: format!("{}'s deviation from mean", self.name),
            values: Vec::new(),
        };

        if valid_parameter(self) == false {
            return dev;
        }

        let mean: f64 = self.average();

        for val in &self.values {
            dev.values.push(val - mean);
        }


        return dev;
    }
}
