
mod complexmath;
use complexmath::Complex as Complex;

mod quantum {

    // in future use complex numbers structure.
    struct Qubit {
        
        alpha: Complex,
        betha: Complex
    }

    const ZERO : Qubit = Qubit { 
        alpha: Complex {real: 1.0, image:0.0 },
        betha: Complex {real: 0.0, image:0.0 } };
    
    const ZERO : Qubit = Qubit { 
        alpha: Complex {real: 0.0, image:0.0 },
        betha: Complex {real: 1.0, image:0.0 } };

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


