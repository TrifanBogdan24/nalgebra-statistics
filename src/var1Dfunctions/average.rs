impl Validate {
    fn error_message() {
        println!("\nCode example of how to use it correctly.`: \n\
        let x: Vec<f64> = vec![1.0, 2.0, 3.0];\n\
        let avg: f64 = average(&x); \n"
        );
    }

    fn valid_parameter(x: &Variable1D) -> bool {

        if x.values.len() == 0 {
            eprintln!("Err: empty array to compute average.");
            error_message();
            return false;
        }

        return true;
    }
}

impl Variable1D {
    pub fn average(&self) -> f64 {
        // calculates the average of all vetor's values

        if valid_parameter(self) == false {
            return 0.0 as f64;
        }

        let n: f64 = self.values.len() as f64;
        let mut sum: f64 = 0.0;

        for val in &self.values {
            sum = sum + val;
        }

        return sum / n;
    }
}
