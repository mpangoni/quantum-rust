use std::ops;

mod complexmath{

  pub struct Complex {
    
    real: f32,
    image: f32
    
  }

  // + operator for complex number
  impl ops::Add for Complex {
    type Output = Complex;

    fn add(self, _rhs: Complex) -> Complex {
      Complex {
        real: self.real + _rhs.real;
        image: self.image + _rhs.image;
      }
    }
  }

  // * operator for complex number
  impl ops::Mul for Complex {
    type Output = Complex;

    fn mul(self, _rhs: Complex) -> Complex {
      Complex {
        real: self.real * _rhs.real;
        image: self.image * _rhs.image;
      }
    }
  }

  
}
