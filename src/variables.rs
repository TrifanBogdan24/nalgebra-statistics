#[derive(Clone)]
pub struct Variable1D {
    pub name: String,
    pub values: Vec<f64>,
}

#[derive(Clone)]
pub struct Variable2D {
    pub x: Variable1D,
    pub y: Variable1D,
}

pub trait ToString {
    fn tostring(&self);
}

impl ToString for Variable1D {
    fn tostring(&self) {
        println!("{} = {:?}", self.name, self.values);
    }
}

impl ToString for Variable2D {
    fn tostring(&self) {
        self.x.tostring();
        self.y.tostring();
    }
}


impl Variable1D {
    // copy constructor for Variable1D
    pub fn new(variable_name: String, val: &[f64]) -> Self {
        return Variable1D {
            name: variable_name,
            values: val.to_vec(),
        };
    }
}


impl Variable2D {
    // copy constructor for Variable2D
    pub fn new(var1: &Variable1D, var2: &Variable1D) -> Self {
        return Variable2D {
            x: var1.clone(),
            y: var2.clone(),
        };
    }
}

#[macro_export]
// let my_variable = var1d!("my_variable", 1.0, 2.0, 3.0);
macro_rules! var1d {
    ($name:expr, $($val:expr),*) => {
        Variable1D {
            name: $name.to_string(),
            values: vec![$($val),*],
        }
    };
}


#[macro_export]
// let my_variable_2d = var2d!(variable_x, variable_y);
macro_rules! var2d {
    ($var1:expr, $var2:expr) => {
        Variable2D {
            x: $var1.clone(),
            y: $var2.clone(),
        }
    };
}
