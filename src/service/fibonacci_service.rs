pub struct FibonacciService;

impl FibonacciService {
    fn calculate_fibonacci(n: u32) -> u64 {
        match n {
            0 => 0,
            1 => 1,
            _ => {
                let mut a = 0;
                let mut b = 1;
                for _ in 2..=n {
                    let temp = a + b;
                    a = b;
                    b = temp;
                }
                b
            }
        }
    }

    pub fn get_fibonacci(n: u32) -> u64 {
        Self::calculate_fibonacci(n)
    }
}