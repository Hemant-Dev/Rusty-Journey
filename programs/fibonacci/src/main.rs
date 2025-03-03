use std::io;

fn main() {
    println!("Enter the Nth Fibonacci Number to generate");
    let mut n_th_fib = String::new();
    io::stdin()
        .read_line(&mut n_th_fib)
        .expect("failed to get the input variable");
    let n_th_fib = match n_th_fib.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid value!!");
            return;
        },
    };
    println!("{}", n_th_fibonacci(n_th_fib));

}

fn n_th_fibonacci(n: u32) -> u32 {
    let mut a = 0;
    let mut b = 1;
    if n == 0 || n == 1
        {return n;}

    for _iter in 1..n+1 {
        let temp = a;
        a = a + b;
        b = temp;
    }
    return a;
}
