fn fibonacci(n: u32) -> u32 {
    if n == 0{
        return 0;
    }
    else if n == 1 {
        return 1;
    }
    else{
        return fibonacci(n-1) + fibonacci(n-2);
    }

    /* 
    fibonacci(5) = fibonacci(4) + fibonacci(3)
    fibonacci(4) = fibonacci(3) + fibonacci(2)
    fibonacci(3) = fibonacci(2) + fibonacci(1)
    fibonacci(2) = fibonacci(1) + fibonacci(0)

    fibonacci(5) = 3 + 2
    fibonacci(4) = 2 + 1
    fibonacci(3) = 1 + 1
    fibonacci(2) = 1 + 0
    
    */
}

#[cfg(test)]
mod fibonacci_test {

    #[test]
    fn test_fibonacci() {
        assert_eq!(super::fibonacci(0), 0);
        assert_eq!(super::fibonacci(1), 1);
        assert_eq!(super::fibonacci(2), 1);
        assert_eq!(super::fibonacci(3), 2);
        assert_eq!(super::fibonacci(4), 3);
        assert_eq!(super::fibonacci(5), 5);
        assert_eq!(super::fibonacci(6), 8);
        assert_eq!(super::fibonacci(7), 13);
        assert_eq!(super::fibonacci(8), 21);
        assert_eq!(super::fibonacci(9), 34);
        assert_eq!(super::fibonacci(10), 55);
    }

}

fn main() {
    println!("Fibonacci of 10 is {}", fibonacci(10));

    let deleteme = fibonacci(10);
    println!("Fibonacci of 10 is {deleteme}");
}