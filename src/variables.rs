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

trait Validate {
    fn error_message();
    fn valid_parameter();
}