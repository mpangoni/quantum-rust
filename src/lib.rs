
mod quantum {

    // in future use complex numbers structure.
    struct Qubit {
        
        alpha_real: f32,
        alpha_imag: f32,
        
        betha_real: f32,
        betha_imag: f32
    }

    const ZERO : Qubit = Qubit { 
        alpha_real: 1.0,
        alpha_imag: 0.0,

        betha_real: 0.0,
        betha_imag: 0.0 };
    
    const ONE : Qubit = Qubit { 
        alpha_real: 0.0,
        alpha_imag: 0.0,

        betha_real: 1.0,
        betha_imag: 0.0 };

    // unitary gates
    fn h(q: Qubit) { // Hadamard
        
    }

    fn x(q: Qubit) { //   Pauli-X gate or NOT gate

    }
    
    // controled gates
    fn cnot(c: Qubit, t: Qubit) {
    
    }
    
    fn ccnot(c1: Qubit, c2: Qubit, t:Qubit) { // Toffoli Gate
        
    }
    
    fn cswap(c: Qubit, t1: Quibit, t2:Qubit) { // Fredkin Gate
    
    }

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}


