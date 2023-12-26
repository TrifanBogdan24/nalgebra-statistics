
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub mod var_1d {
    pub mod average;
    pub mod deviation_from_mean;
    pub mod mean_standard_deviation;
}

pub mod var_2d {
    pub mod correlation;
    pub mod covariance;
}

pub mod variables;

#[cfg(test)]
mod tests {
    use super::*;

    use crate::variables::{Variable1D, Variable2D};
    use crate::var_1d::{average, mean_standard_deviation, deviation_from_mean};
    use crate::var_2d::{correlation, covariance};

    use approx::abs_diff_eq;        // since I compute integrals,
    // I am interested in comparing only the first 5 decimals


    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }



    #[test]
    fn var_1d_structure() {
        let var: Variable1D = Variable1D{
            name: String::from("X"),
            values: vec![5.1, 5.2, 5.9, 6.2],
        };

        if var.name != String::from("X") {
            assert!(false);
        }

        if var.values.len() != 4 {
            assert!(false);
        }
        
        if var.values != vec![5.1, 5.2, 5.9, 6.2] {
            assert!(false);
        }

        assert!(true);
    }


    #[test]
    fn var_1d_copy_constructor() {
        let var: Variable1D = Variable1D::new(String::from("X"), &[5.1, 5.2, 5.9, 6.2]);

        if var.name != String::from("X") {
            assert!(false);
        }

        if var.values.len() != 4 {
            assert!(false);
        }

        if var.values != &[5.1, 5.2, 5.9, 6.2] {
            assert!(false);
        }

        assert!(true);
    }

    #[test]
    fn var_1d_macro_constructor() {
        let var: Variable1D = var1d!("X", 5.1, 5.2, 5.9, 6.2);

        if var.name != "X" {
            assert!(false);
        }

        if var.values.len() != 4 {
            assert!(false);
        }

        if var.values != vec![5.1, 5.2, 5.9, 6.2] {
            assert!(false);
        }

        assert!(true);
    }


    #[test]
    fn var_2d_structure() {
        let x_var: Variable1D = var1d!("X", 5.1, 5.2, 5.9, 6.2);
        let y_var: Variable1D = var1d!("Y", 10.0, 90.0, 250.0, 450.0);

        let xy_var: Variable2D =  Variable2D {
            x: x_var,
            y: y_var,
        };

        if xy_var.x.name != "X" || xy_var.y.name != "Y" {
            assert!(false);
        }

        if xy_var.x.values.len() != 4 || xy_var.y.values.len() != 4 {
            assert!(false);
        }

        if xy_var.x.values != &[5.1, 5.2, 5.9, 6.2] || xy_var.y.values != &[10.0, 90.0, 250.0, 450.0] {
            assert!(false);
        }

        assert!(true);
    }

    #[test]
    fn var_2d_copy_constructor() {
        let x_var: Variable1D = var1d!("X", 5.1, 5.2, 5.9, 6.2);
        let y_var: Variable1D = var1d!("Y", 10.0, 90.0, 250.0, 450.0);

        let xy_var: Variable2D =  Variable2D::new(&x_var, &y_var);

        if xy_var.x.name != "X" || xy_var.y.name != "Y" {
            assert!(false);
        }

        if xy_var.x.values.len() != 4 || xy_var.y.values.len() != 4 {
            assert!(false);
        }

        if xy_var.x.values != &[5.1, 5.2, 5.9, 6.2] || xy_var.y.values != &[10.0, 90.0, 250.0, 450.0] {
            assert!(false);
        }

        assert!(true);
    }

    #[test]
    fn var_2d_macro_constructor() {
        let x_var: Variable1D = var1d!("X", 5.1, 5.2, 5.9, 6.2);
        let y_var: Variable1D = var1d!("Y", 10.0, 90.0, 250.0, 450.0);

        let xy_var: Variable2D =  var2d!(&x_var, &y_var);

        if xy_var.x.name != "X" || xy_var.y.name != "Y" {
            assert!(false);
        }

        if xy_var.x.values.len() != 4 || xy_var.y.values.len() != 4 {
            assert!(false);
        }

        if xy_var.x.values != &[5.1, 5.2, 5.9, 6.2] || xy_var.y.values != &[10.0, 90.0, 250.0, 450.0] {
            assert!(false);
        }

        assert!(true);
    }

    #[test]
    fn test_average() {
        let x_var: Variable1D = var1d!("X", 5.1, 5.2, 5.9, 6.2);
        let avg: f64 = x_var.average();

        // 3 fixed decimals
        let precision: f64 = 1e-3;

        assert_eq!(abs_diff_eq!(avg, 5.600, epsilon = precision), true);
    }

    #[test]
    fn test_deviation_from_mean_value() {
        let x_var: Variable1D = var1d!("X", 5.1, 5.2, 5.9, 6.2);
        let dev: Variable1D = x_var.deviation_from_mean();

        // 3 fixed decimals
        let precision: f64 = 1e-3;

        if dev.values.len() != 4 {
            assert!(false);
        }
        
        if abs_diff_eq!(dev.values[0], -0.500, epsilon = precision) == false {
            assert!(false);
        }
        
        if abs_diff_eq!(dev.values[1], -0.400, epsilon = precision) == false {
            assert!(false);
        }
        
        if abs_diff_eq!(dev.values[2], 0.300, epsilon = precision) == false {
            assert!(false);
        }

        if abs_diff_eq!(dev.values[3], 0.600, epsilon = precision) == false {
            assert!(false);
        }

        assert!(true);
    }


    #[test]
    fn test_mean_standard_deviation() {
        let x_var: Variable1D = var1d!("X", 5.1, 5.2, 5.9, 6.2);
        let avg_dev: f64 = x_var.mean_standard_deviation();

        // 3 fixed decimals
        let precision: f64 = 1e-3;
        assert_eq!(abs_diff_eq!(avg_dev, 0.268, epsilon = precision), true);

    }

    #[test]
    fn test_covariance() {
        let x_var: Variable1D = var1d!("X", 5.1, 5.2, 5.9, 6.2);
        let y_var: Variable1D = var1d!("Y", 10.0, 90.0, 250.0, 450.0);
        let xy_var: Variable2D =  var2d!(&x_var, &y_var);

        let cov: f64 = xy_var.covariance();

        // 3 fixed decimals
        let precision: f64 = 1e-3;
        assert_eq!(abs_diff_eq!(cov, 76.000, epsilon = precision), true);
    }

    #[test]
    fn test_correlation() {
        let x_var: Variable1D = var1d!("X", 5.1, 5.2, 5.9, 6.2);
        let y_var: Variable1D = var1d!("Y", 10.0, 90.0, 250.0, 450.0);
        let xy_var: Variable2D =  var2d!(&x_var, &y_var);

        // the coefficient of correlation is annotated with `r`
        let r: f64 = xy_var.correlation();

        // 3 fixed decimals
        let precision: f64 = 1e-3;
        assert_eq!(abs_diff_eq!(r, 0.978, epsilon = precision), true);
    }

    
}
