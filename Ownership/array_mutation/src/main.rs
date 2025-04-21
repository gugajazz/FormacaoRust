

/*
Dada uma operação e um número (semelhante à calculadora), 
aplicar esta operação a todos os membros de um array definido por si. 
Resolver com recurso a passagem de ownership e com referências.
*/


fn array_mut_ownership(array: [u32; 5], op: char, val: u32) -> [u32; 5] {
    let mut result = [0; 5];

    for (i, &x) in array.iter().enumerate() {
        result[i] = match op {
            '+' => x + val,
            '-' => x - val,
            '*' => x * val,
            '/' => x / val,
            _ => panic!("Invalid operation"),
        };
    }

    result
}



fn array_mut_mut(array: &mut [u32], operation: char, other_member: u32) {
    for i in 0..array.len() {
        array[i] = match operation {
            '+' => array[i] + other_member,
            '-' => array[i] - (other_member),
            '*' => array[i] * other_member,
            '/' => array[i] / other_member,
            _ => panic!("Invalid operation"),
        };
    }
}

fn main(){
    let array = [1, 2, 3, 4, 5];
    let result = array_mut_ownership(array, '+', 1);
    println!("Original array: {:?}", array); // I can only access this because ths array is a copy type.
    println!("Result with ownership: {:?}", result);

    let mut array = [1, 2, 3, 4, 5];
    array_mut_mut(&mut array, '+', 1);
    println!("Result with mutable reference: {:?}", array);
}

#[cfg(test)]
mod array_mutation_test {

    const OWNERSHIP_TEST_ARRAY: [u32; 5] = [1, 2, 3, 4, 5];

    #[test]
    fn test_ownership_mutation() {
        assert_eq!(super::array_mut_ownership(OWNERSHIP_TEST_ARRAY, '+', 1), [2, 3, 4, 5, 6]);
        assert_eq!(super::array_mut_ownership(OWNERSHIP_TEST_ARRAY, '-', 1), [0, 1, 2, 3, 4]);
        assert_eq!(super::array_mut_ownership(OWNERSHIP_TEST_ARRAY, '*', 2), [2, 4, 6, 8, 10]);
        assert_eq!(super::array_mut_ownership(OWNERSHIP_TEST_ARRAY, '/', 2), [0, 1, 1, 2, 2]);
    }

    #[test]
    fn test_mut_ref_mutation() {
        let mut array = OWNERSHIP_TEST_ARRAY.clone();

        super::array_mut_mut(&mut array, '+', 1);

        assert_eq!(array, [2, 3, 4, 5, 6]);

        let mut array = OWNERSHIP_TEST_ARRAY.clone();

        super::array_mut_mut(&mut array, '-', 1);

        assert_eq!(array, [0, 1, 2, 3, 4]);

        let mut array = OWNERSHIP_TEST_ARRAY.clone();

        super::array_mut_mut(&mut array, '*', 2);

        assert_eq!(array, [2, 4, 6, 8, 10]);

        let mut array = OWNERSHIP_TEST_ARRAY.clone();

        super::array_mut_mut(&mut array, '/', 2);

        assert_eq!(array, [0, 1, 1, 2, 2]);
    }

}
