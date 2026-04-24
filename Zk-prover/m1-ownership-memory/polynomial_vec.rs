

struct PolynomialVec(Vec<u64>);

impl PolynomialVec{
    fn new(data:Vec<u64>)->Self{
        Self(data)
    }
    fn scale(& mut self,factor:u64){
    for i in 0..self.len(){
        self[i] = self[i] * factor;
    }
    
}
    
fn add_poly(&self,other:PolynomialVec) -> PolynomialVec{
    let mut result = self.clone();
    // i nedd clone here coz i want to return a new vector
    for i in 0..self.len(){
        result[i] = self[i] + other[i];
    }
    result
}
}


impl Deref for PolynomialVec{
    type Target = Vec<u64>;
    
    fn deref(&self)->&Self::Target{
        &self.0;
    }
}

impl DerefMut for PolynomialVec{
    fn deref_mut(&mut self)->& mut Self::Target{
        &mut self.0;
    }
}


