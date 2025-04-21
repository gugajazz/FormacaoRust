/*
Utilizando iteradores, FOS e estruturas de dados, faça um método que aceite Vec<Vec<u32>>
e retorne apenas um Vec<u32> sem elementos repetidos e com apenas múltiplos de 2 e 3.
*/

use std::collections::HashSet;

fn achatar_deduplicar_filtrar(v: Vec<Vec<u32>>) -> Vec<u32> {
    // Flatten the Vec<Vec<u32>> into a single iterator, removing nested structure
    v.into_iter()
        .flat_map(|x| x.into_iter()) // Flatten the inner vectors
        .filter(|&x| x % 2 == 0 || x % 3 == 0) // Keep only multiples of 2 or 3
        .collect::<HashSet<_>>() // Deduplicate by collecting into a HashSet
        .into_iter() // Convert HashSet back to an iterator
        .collect() // Collect into a Vec<u32>
}

fn main() {}

#[cfg(test)]
mod achatar_deduplicar_filtrar_test {
    use std::collections::HashSet;

    #[test]
    fn test_func() {
        let vec = vec![vec![1, 2, 3], vec![3, 4, 5], vec![5, 6, 7]];

        let result = super::achatar_deduplicar_filtrar(vec);

        assert!(result.iter().all(|x| x % 2 == 0 || x % 3 == 0));

        let mut seen = HashSet::new();

        assert!(result.iter().all(|x| seen.insert(x)));
    }
}
