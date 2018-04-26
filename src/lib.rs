
mod complexmath;

mod quantum {

    use complexmath::Complex;
    
    // in future use complex numbers structure.
    pub struct Qubit {
        
        alpha: Complex,
        betha: Complex
    }

    pub const ZERO : Qubit = Qubit { 
        alpha: Complex {real: 1.0, image:0.0 },
        betha: Complex {real: 0.0, image:0.0 } };
    
    pub const ONE : Qubit = Qubit { 
        alpha: Complex {real: 0.0, image:0.0 },
        betha: Complex {real: 1.0, image:0.0 } };

    // unitary gates
    pub fn h(q: Qubit) { // Hadamard
        
    }

    pub fn x(q: Qubit) { //   Pauli-X gate or NOT gate

    }
    
    // controled gates
    pub fn cnot(c: Qubit, t: Qubit) {
    
    }
    
    pub fn ccnot(c1: Qubit, c2: Qubit, t:Qubit) { // Toffoli Gate
        
    }
    
    pub fn cswap(c: Qubit, t1: Qubit, t2:Qubit) { // Fredkin Gate
    
    }

}

#[cfg(test)]
mod tests {

}


