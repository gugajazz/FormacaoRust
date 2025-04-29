/*
Using traits and generics implement two sorting algorithms (bubble, select) that are able to sort any Vec, as long as T: Ord.
*/

trait SortingAlgorithm<T: Ord> {
    fn sort(&self, vec: &mut Vec<T>);
}

struct BubbleSort;
struct SelectSort;

impl<T: Ord> SortingAlgorithm<T> for BubbleSort {
    fn sort(&self, vec: &mut Vec<T>) {
        let len = vec.len();

        // Return early if vector is empty or has only one element
        if len <= 1 {
            return;
        }

        for i in 0..len {
            // Flag to exit early if no swaps were made
            let mut swapped = false;

            // Last i elements are already sorted
            for j in 0..len - i - 1 {
                if vec[j] > vec[j + 1] {
                    // Swap elements
                    vec.swap(j, j + 1);
                    swapped = true;
                }
            }

            // If no swaps were made in this pass, array is sorted
            if !swapped {
                break;
            }
        }
    }
}

impl<T: Ord> SortingAlgorithm<T> for SelectSort {
    fn sort(&self, vec: &mut Vec<T>) {
        let len = vec.len();

        // Return early if vector is empty or has only one element
        if len <= 1 {
            return;
        }

        // Selection sort implementation
        for i in 0..len - 1 {
            // Find the minimum element in unsorted portion
            let mut min_idx = i;

            for j in i + 1..len {
                if vec[j] < vec[min_idx] {
                    min_idx = j;
                }
            }

            // Swap the found minimum element with the first element of unsorted portion
            if min_idx != i {
                vec.swap(i, min_idx);
            }
        }
    }
}

// Example usage
fn main() {
    // Test with integers
    let mut numbers = vec![5, 2, 9, 1, 5, 6];
    let bubble_sorter = BubbleSort;
    bubble_sorter.sort(&mut numbers);
    println!("Bubble sorted: {:?}", numbers);

    let mut numbers = vec![5, 2, 9, 1, 5, 6];
    let select_sorter = SelectSort;
    select_sorter.sort(&mut numbers);
    println!("Selection sorted: {:?}", numbers);

    // Test with strings
    let mut words = vec!["banana", "apple", "cherry", "date"];
    bubble_sorter.sort(&mut words);
    println!("Bubble sorted words: {:?}", words);

    let mut words = vec!["banana", "apple", "cherry", "date"];
    select_sorter.sort(&mut words);
    println!("Selection sorted words: {:?}", words);
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to test sorting algorithms with various inputs
    fn test_sorting_algorithm<T: Ord + Clone + std::fmt::Debug>(
        algorithm: &impl SortingAlgorithm<T>,
        input: &[T],
        expected: &[T],
    ) {
        let mut vec = input.to_vec();
        algorithm.sort(&mut vec);
        assert_eq!(vec, expected, "Sorting failed for input: {:?}", input);
    }

    #[test]
    fn test_bubble_sort_integers() {
        let sorter = BubbleSort;

        // Regular case
        test_sorting_algorithm(&sorter, &[5, 2, 9, 1, 5, 6], &[1, 2, 5, 5, 6, 9]);

        // Already sorted
        test_sorting_algorithm(&sorter, &[1, 2, 3, 4, 5], &[1, 2, 3, 4, 5]);

        // Reverse sorted
        test_sorting_algorithm(&sorter, &[5, 4, 3, 2, 1], &[1, 2, 3, 4, 5]);

        // Empty vector
        test_sorting_algorithm::<i32>(&sorter, &[], &[]);

        // Single element
        test_sorting_algorithm(&sorter, &[42], &[42]);

        // Duplicate elements
        test_sorting_algorithm(&sorter, &[3, 3, 3, 1, 1, 2], &[1, 1, 2, 3, 3, 3]);

        // Negative numbers
        test_sorting_algorithm(&sorter, &[-5, 12, -10, 0, 8, -3], &[-10, -5, -3, 0, 8, 12]);
    }

    #[test]
    fn test_selection_sort_integers() {
        let mut sorter = SelectSort;

        // Regular case
        test_sorting_algorithm(&mut sorter, &[5, 2, 9, 1, 5, 6], &[1, 2, 5, 5, 6, 9]);

        // Already sorted
        test_sorting_algorithm(&mut sorter, &[1, 2, 3, 4, 5], &[1, 2, 3, 4, 5]);

        // Reverse sorted
        test_sorting_algorithm(&mut sorter, &[5, 4, 3, 2, 1], &[1, 2, 3, 4, 5]);

        // Empty vector
        test_sorting_algorithm::<i32>(&mut sorter, &[], &[]);

        // Single element
        test_sorting_algorithm(&mut sorter, &[42], &[42]);

        // Duplicate elements
        test_sorting_algorithm(&mut sorter, &[3, 3, 3, 1, 1, 2], &[1, 1, 2, 3, 3, 3]);

        // Negative numbers
        test_sorting_algorithm(
            &mut sorter,
            &[-5, 12, -10, 0, 8, -3],
            &[-10, -5, -3, 0, 8, 12],
        );
    }

    #[test]
    fn test_sorting_strings() {
        let mut bubble_sorter = BubbleSort;
        let mut select_sorter = SelectSort;

        // Test strings with bubble sort
        test_sorting_algorithm(
            &mut bubble_sorter,
            &["banana", "apple", "cherry", "date"],
            &["apple", "banana", "cherry", "date"],
        );

        // Test strings with selection sort
        test_sorting_algorithm(
            &mut select_sorter,
            &["banana", "apple", "cherry", "date"],
            &["apple", "banana", "cherry", "date"],
        );

        // Empty strings and mixed lengths
        test_sorting_algorithm(
            &mut bubble_sorter,
            &["", "abc", "a", "ab"],
            &["", "a", "ab", "abc"],
        );

        test_sorting_algorithm(
            &mut select_sorter,
            &["", "abc", "a", "ab"],
            &["", "a", "ab", "abc"],
        );
    }

    #[test]
    fn test_sorting_custom_types() {
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
        struct Person {
            age: u32,
            name: String,
        }

        let people = vec![
            Person {
                age: 30,
                name: "Alice".to_string(),
            },
            Person {
                age: 25,
                name: "Bob".to_string(),
            },
            Person {
                age: 40,
                name: "Charlie".to_string(),
            },
            Person {
                age: 20,
                name: "David".to_string(),
            },
        ];

        let expected = vec![
            Person {
                age: 20,
                name: "David".to_string(),
            },
            Person {
                age: 25,
                name: "Bob".to_string(),
            },
            Person {
                age: 30,
                name: "Alice".to_string(),
            },
            Person {
                age: 40,
                name: "Charlie".to_string(),
            },
        ];

        // Test custom type with bubble sort
        test_sorting_algorithm(&mut BubbleSort, &people, &expected);

        // Test custom type with selection sort
        test_sorting_algorithm(&mut SelectSort, &people, &expected);
    }

    #[test]
    fn test_large_vectors() {
        // Test with larger vectors to ensure algorithms handle them correctly
        let large_vec: Vec<i32> = (0..1000).rev().collect();
        let expected: Vec<i32> = (0..1000).collect();

        let bubble_sorter = BubbleSort;
        let mut bubble_vec = large_vec.clone();
        bubble_sorter.sort(&mut bubble_vec);
        assert_eq!(bubble_vec, expected);

        let select_sorter = SelectSort;
        let mut select_vec = large_vec.clone();
        select_sorter.sort(&mut select_vec);
        assert_eq!(select_vec, expected);
    }
}
