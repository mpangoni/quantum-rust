use std::ops;

pub struct Complex {
  
  pub real: f32,
  pub image: f32
  
}

// + operator for complex number
impl ops::Add for Complex {
  type Output = Complex;

  fn add(self, _rhs: Complex) -> Complex {
    Complex {
      real: self.real + _rhs.real,
      image: self.image + _rhs.image
    }
  }
}

// * operator for complex number
impl ops::Mul for Complex {
  type Output = Complex;

  fn mul(self, _rhs: Complex) -> Complex {
    Complex {
      real: self.real * _rhs.real,
      image: self.image * _rhs.image
    }
  }
} 


#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn test_addition() {

    let a = Complex { real: 1.0, image: 0.0};
    let b = Complex { real: 0.5, image: 0.34};

    let result =  a + b;
    assert_eq!( result.real, 1.5);
    assert_eq!( result.image, 0.34);
  }

  #[test]
  fn test_multiplication() {

    let a = Complex { real: 2.0, image: 4.0};
    let b = Complex { real: 0.5, image: 0.5};

    let result =  a * b;
    assert_eq!( result.real, 1.0);
    assert_eq!( result.image, 2.0);
    
  }

}
