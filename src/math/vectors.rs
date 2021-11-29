#![allow(dead_code)]

pub struct MathVector {
    values: Vec<f64>,
    dimension: usize
}

impl MathVector {
    pub fn new(values: &Vec<f64>) -> MathVector{
        MathVector { values:    values.clone(),
                     dimension: values.len() }
    }

    pub fn dimension(self: &Self) -> usize {
        self.dimension
    }

    pub fn get_values(self: &Self) -> &'_ Vec<f64> {
        &self.values
    }

    pub fn set_values(self: &mut Self, vals: &Vec<f64>) -> bool {
        match vals.len() == self.dimension() {
            true  => { self.values = vals.clone(); 
                       return true; }
            false => { return false; } 
        }
    }
}

impl std::ops::Add for MathVector {
    type Output = Self;
    fn add(self, other: Self) -> MathVector {
        match self.dimension() == other.dimension() {
            true  =>  { 
                let mut vals: Vec<f64> = Vec::with_capacity(self.dimension);
                for i in 0..self.dimension() {
                    vals.push(self.values[i] + other.values[i]);
                }
                MathVector { values:    vals,
                             dimension: self.dimension }
            }
            false => { panic!("Error: Expected same dimensions.") }
        }
    }
}

impl std::ops::Sub for MathVector {
    type Output = Self;
    fn sub(self, other: Self) -> MathVector {
        match self.dimension() == other.dimension() {
            true  =>  { 
                let mut vals: Vec<f64> = Vec::with_capacity(self.dimension);
                for i in 0..self.dimension() {
                    vals.push(self.values[i] - other.values[i]);
                }
                MathVector { values:    vals,
                             dimension: self.dimension }
            }
            false => { panic!("Error: Expected same dimensions.") }
        }
    }
}

pub fn scalar_mul(scalar: f64, other: &MathVector) -> MathVector {

    let mut vals: Vec<f64> = Vec::with_capacity(other.dimension);

    for i in 0..other.dimension {
        vals.push(scalar * other.values[i]);
    }
    MathVector { values:    vals,
                 dimension: other.dimension }
}

impl std::fmt::Display for MathVector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut finstring: String = "(".to_owned();
        for i in 0..self.dimension {
            finstring =  " ".to_owned() + &finstring + &self.values[i].to_string() + ";";
        }
        finstring.pop();
        finstring = finstring + " )";
        write!(f, "{}", finstring)
    }
}