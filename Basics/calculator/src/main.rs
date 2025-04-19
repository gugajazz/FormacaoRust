// use std::result;

/// Desafio da calculadora
/// Pode ser implementado com recurso à analise de apenas
/// uma string ou com cada elemento separado na sua propria string.
/// Podem ver exemplos de como vai ser utilizada a função nos testes disponíveis.
///
/// Devem apenas implementar uma das funções.
///
/// Podem comentar a função que não vão implementar para não haver problemas de compilação, incluindo os testes
/// para a mesma.

fn main() {
    let mut result = calculator_str("1 + 1");
    println!("{result}");
    result = calculator_str_list(&vec!["2", "*", "3"]);
    println!("{result}");
}


fn calculator_str(string: &str) -> i32 {
    let string_whitespace_iterator = string.split_whitespace();
    let parts: Vec<&str> = string_whitespace_iterator.collect();

    if parts.len() != 3 {
        panic!("Invalid input");
    }

    let first_number: i32 = parts[0].parse().expect("Invalid first number");
    let second_number: i32 = parts[2].parse().expect("Invalid second number");
    let operator: &str = parts[1];

    let result : i32;
    match operator {
        "+" => { result = first_number + second_number }
        "-" => { result = first_number - second_number }
        "*" => { result = first_number * second_number }
        "/" => { result = first_number / second_number }
        _ => panic!("Invalid operator"),
    };

    result

}

fn calculator_str_list(string: &[&str]) -> i32 {
    let first_number: i32 = string[0].parse().expect("Invalid first number");
    let second_number: i32 = string[2].parse().expect("Invalid second number");
    let operator: &str = string[1];

    let result : i32;
    match operator {
        "+" => { result = first_number + second_number }
        "-" => { result = first_number - second_number }
        "*" => { result = first_number * second_number }
        "/" => { result = first_number / second_number }
        _ => panic!("Invalid operator"),
    };

    result
}

#[cfg(test)]
pub mod calculator_test {

    #[test]
    fn test_calculator_str() {
        assert_eq!(super::calculator_str("1 + 1"), 2); // 1 + 1 = 2
        assert_eq!(super::calculator_str("2 * 2"), 4);
        assert_eq!(super::calculator_str("2 / 2"), 1);
        assert_eq!(super::calculator_str("2 - 2"), 0);
    }

    #[test]
    fn test_calculator_str_list() {
        assert_eq!(super::calculator_str_list(&vec!["2", "*", "3"]), 6);
        assert_eq!(super::calculator_str_list(&vec!["2", "+", "3"]), 5);
        assert_eq!(super::calculator_str_list(&vec!["3", "-", "2"]), 1);
        assert_eq!(super::calculator_str_list(&vec!["6", "/", "3"]), 2);
    }

}