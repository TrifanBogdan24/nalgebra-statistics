use crate::variables::{Variable1D, Variable2D};

fn error_message() {
    println!("\nCode example of how to use it correctly: \n\
    let x: Vec<f64> = vec![1.0, 2.0, 3.0];\n\
    let y: Vec<f64> = vec![10.0, 20.0, 30.0];\n\
    let r: f64 = correlation(&x, &y); \n"
    );
}


fn valid_parameter(var: &Variable2D) -> bool {
    let x: &Variable1D = &var.x;
    let y: &Variable1D = &var.y;

    if x.values.len() == 0 && y.values.len() == 0 {
        eprintln!("Err: empty arrays to compute the correlation.");
        error_message();
        return false;
    }

    if x.values.len() == 0 {
        eprintln!("Err: empty abscissa (OX) array to compute the correlation.");
        error_message();
        return false;
    }

    if y.values.len() == 0 {
        eprintln!("Err: empty ordinate (OY) array to compute the covariance.");
        error_message();
        return false;
    }

    if x.values.len() != y.values.len() {
        eprintln!("Err: correlation cannot be computed for vectors with different lengths.");
        error_message();
        return false;
    }

    return true;
}

impl Variable2D {
    pub fn correlation(&self) -> f64 {
        // calculates the correlation between two variables (vectors)
        // x = a vector representing the abcissas of the points (OX) 
        // y = a vector representing the ordinates of the points (OY)

        if valid_parameter(self) == false {
            return 0.0 as f64;
        }

        let x: &Variable1D = &self.x;
        let y: &Variable1D = &self.y;

        let x_avg: f64 = x.average();
        let y_avg: f64 = y.average();

        let mut sum_of_prods: f64 = 1.0 as f64;   // for numerator
        let mut x_sum: f64 = 0.0 as f64;        // for denumitor
        let mut y_sum: f64 = 0.0 as f64;        // for denumitor

        let n = x.values.len();

        for i in 0..=(n - 1) {
            sum_of_prods = sum_of_prods + (x.values[i] - x_avg) * (y.values[i] - y_avg);
            x_sum = x_sum + (x.values[i] - x_avg) * (x.values[i] - x_avg);
            y_sum = y_sum + (y.values[i] - y_avg) * (y.values[i] - y_avg);
        }


        if x_sum == 0.0 || y_sum == 0.0 {
            // no correlation between the given variables
            // one of them has constant values
            return 0.0 as f64
        }

        let r: f64 = sum_of_prods / (x_sum.sqrt() * y_sum.sqrt());
        return r;
    }
}

pub enum CorrelationType {
    // r = corr(x, y) = correlation coefficient

    Invalid,            // r < -1 or r > 1

    NoCorr,             // r = 0.0

    NegVeryWeak,        // r ∈ (-0.0, -0.2]
    NegWeak,            // r ∈ (-0.2, -0.4]
    NegReasonable,      // r ∈ (-0.4, -0.6]
    NegHigh,            // r ∈ (-0.6, -0.8]
    NegVeryHigh,        // r ∈ (-0.8, -1.0)
    NegCorr,            // r = -1.0

    PosVeryWeak,        // r ∈ (0.0, 0.2]
    PosWeak,            // r ∈ (0.2, 0.4]
    PosReasonable,      // r ∈ (0.4, 0.6]
    PosHigh,            // r ∈ (0.6, 0.8]
    PosVeryHigh,        // r ∈ (0.8, 1.0)
    PosCorr             // r = 1.0
}

impl Variable2D {
    pub fn correlation_value_interpretation(&self) -> CorrelationType {

        let r: f64 = self.correlation();

        if r == 0.0 {
            return CorrelationType::NoCorr;
        } else if -0.0 < r && r <= -0.2 {
            return CorrelationType::NegVeryWeak;
        } else if -0.2 < r && r <= -0.4 {
            return CorrelationType::NegWeak;
        } else if -0.4 < r && r <= -0.6 {
            return CorrelationType::NegReasonable
        } else if -0.6 < r && r <= -0.8 {
            return CorrelationType::NegHigh;
        } else if -0.8 < r && r < -1.0 {
            return CorrelationType::NegVeryHigh;
        } else if r == -1.0 {
            return CorrelationType::NegCorr;
        } else if 0.0 < r && r <= 0.2 {
            return CorrelationType::PosVeryWeak;
        } else if 0.2 < r && r <= 0.4 {
            return CorrelationType::PosWeak;
        } else if 0.4 < r && r <= 0.6 {
            return CorrelationType::PosReasonable;
        } else if 0.6 < r && r <= 0.8 {
            return CorrelationType::PosHigh;
        } else if 0.8 < r && r < 1.0 {
            return CorrelationType::PosVeryHigh;
        } else if r == 1.0 {
            return CorrelationType::PosCorr;
        }

        return CorrelationType::Invalid;
    }
}

impl Variable2D {
    pub fn correlation_value_interpretation_tostring(&self) -> String {

        let r: f64 = self.correlation();
        let x: &Variable1D = &self.x;
        let y: &Variable1D = &self.y;

        if r == 0.0 {
            return format!("There is no correlation between '{}' and '{}'."
                           , x.name, y.name);
        } else if -0.0 < r && r <= -0.2 {
            return format!("There is an indirect dependancy \
                            and a very week correlation between '{}' and '{}'."
                           , x.name, y.name);
        } else if -0.2 < r && r <= -0.4 {
            return format!("There is an indirect dependancy \
                            and a weak correlation between '{}' and '{}'."
                           , x.name, y.name);
        } else if -0.4 < r && r <= -0.6 {
            return format!("There is an indirect dependancy \
                            and a reasonable correlation between '{}' and '{}'."
                           , x.name, y.name);
        } else if -0.6 < r && r <= -0.8 {
            return format!("There is an indirect dependancy \
                            and a high correlation between '{}' and '{}'."
                           , x.name, y.name);
        } else if -0.8 < r && r < -1.0 {
            return format!("There is an indirect dependancy \
                            and a very high correlation between '{}' and '{}'."
                           , x.name, y.name);
        } else if r == -1.0 {
            return format!("There is an indirect dependancy \
                            bewtween the variables '{}' and '{}'.\
                            They are for sure correlated and depend one on another."
                           , x.name, y.name);
        } else if 0.0 < r && r <= 0.2 {
            return format!("There is a direct dependancy \
                            and a very weak correlation between '{}' and '{}'."
                           , x.name, y.name);
        } else if 0.2 < r && r <= 0.4 {
            return format!("There is a direct dependency \
                            and a weak correlation between '{}' and '{}'."
                           , x.name, y.name);
        } else if 0.4 < r && r <= 0.6 {
            return format!("There is a direct dependency \
                            and a reasonable correlation between '{}' and '{}'."
                           , x.name, y.name);
        } else if 0.6 < r && r <= 0.8 {
            return format!("There is a direct dependency \
                            and a high correlation between '{}' and '{}'."
                           , x.name, y.name);
        } else if 0.8 < r && r < 1.0 {
            return format!("There is a direct dependency \
                            and a very high correlation between '{}' and '{}'."
                           , x.name, y.name);
        } else if r == 1.0 {
            return format!("There is a direct dependency \
                            between the variables '{}' and '{}'. \
                            They are for sure correlated and depend one on another."
                           , x.name, y.name);
        }

        return format!("There was a mistake in calculating the correlation between '{}' and '{}'."
                       , x.name, y.name);
    }
}
