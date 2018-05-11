
mod complex;

mod quantum {

    use complex::Complex;
    
    // in future use complex numbers structure.
    pub struct Qubit {
        alpha: Complex,
        betha: Complex
    }

    pub const ZERO : Qubit = Qubit { 
        alpha: Complex::new(1.0, 0.0),
        betha: Complex::new(0.0, 0.0);
    }

    pub const ONE : Qubit = Qubit { 
        alpha: Complex::new(0.0, 0.0),
        betha: Complex::new(1.0, 0.0);
    }

    // unitary gates
    pub fn u(q: &mut Qubit, m: (f32, f32, f32, f32)){

    }

    pub fn h(q: &mut Qubit) { // Hadamard
        let matrix = (0, 1, 1, 0);
    }

    pub fn x(q: &mut Qubit) { // Pauli-X gate or NOT gate

    }
    
    // controled gates
    pub fn cnot(c: &Qubit, t: &mut Qubit) {
    
    }
    
    pub fn ccnot(c1: &Qubit, c2: &Qubit, t: &mut Qubit) { // Toffoli Gate
        
    }
    
    pub fn cswap(c: &Qubit, t1: &mut Qubit, t2: &mut Qubit) { // Fredkin Gate
    
    }

}

#[cfg(test)]
mod tests {

}