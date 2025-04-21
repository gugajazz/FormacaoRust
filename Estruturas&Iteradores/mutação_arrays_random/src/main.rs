/*
Dada uma operação e um número (semelhante à calculadora),
aplicar esta operação a todos os membros de um array definido por si.
Resolver com recurso a passagem de ownership e com referências.
*/

fn mut_array_iterator(array: &mut [u32], operation: char, other_member: u32) {
    array.iter_mut().for_each(|x| {
        *x = match operation {
            '+' => *x + other_member,
            '-' => *x - other_member,
            '*' => *x * other_member,
            '/' => *x / other_member,
            _ => panic!("Invalid operation"),
        }
    });
}

fn main() {
    mut_array_iterator(&mut [1, 2, 3, 4, 5], '+', 1);
}

#[cfg(test)]
mod mutable_array_test {
    const OWNERSHIP_TEST_ARRAY: [u32; 5] = [1, 2, 3, 4, 5];

    #[test]
    fn test_mut_ref_mutation() {
        let mut array = OWNERSHIP_TEST_ARRAY.clone();

        super::mut_array_iterator(&mut array, '+', 1);

        assert_eq!(array, [2, 3, 4, 5, 6]);

        let mut array = OWNERSHIP_TEST_ARRAY.clone();

        super::mut_array_iterator(&mut array, '-', 1);

        assert_eq!(array, [0, 1, 2, 3, 4]);

        let mut array = OWNERSHIP_TEST_ARRAY.clone();

        super::mut_array_iterator(&mut array, '*', 2);

        assert_eq!(array, [2, 4, 6, 8, 10]);

        let mut array = OWNERSHIP_TEST_ARRAY.clone();

        super::mut_array_iterator(&mut array, '/', 2);

        assert_eq!(array, [0, 1, 1, 2, 2]);
    }
}
