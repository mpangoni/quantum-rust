
mod quantum {

    // in future use complex numbers structure.
    struct Qubit {
        
        alpha_real: f32,
        alpha_imag: f32,
        
        betha_real: f32,
        betha_imag: f32
    }

    const ZERO : Qubit = { 
        alpha_real = 1.0,
        alpha_imag = 0.0,

        betha_real = 0.0,
        betha_imag = 0.0 }
    
    const ONE : Qubit = { 
        alpha_real: 0.0,
        alpha_imag: 0.0,

        betha_real: 1.0,
        betha_imag: 0.0 }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}


