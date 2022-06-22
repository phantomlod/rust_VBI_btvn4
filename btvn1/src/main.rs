#[derive(Debug)]
struct Fibonacci {
    a: u32,
    b: u32,
}

impl Iterator for Fibonacci {
    type Item = u32 ;

    fn next(&mut self) -> Option<Self::Item> {
        if self.a >= self.b {
            self.b = self.a + self.b;
        } else {
            self.a = self.b + self.a ;
        }
        Some(self.a + self.b)
    }
}
fn fibonacci_numbers() -> Fibonacci {
    Fibonacci { a: 1 , b: 0 }
 }

fn main() {
    // let mut x = fibonacci_numbers();
    // println!("{:?}",x.next());
    for number in fibonacci_numbers() {
        println!("{}", number);
        
    }
}
