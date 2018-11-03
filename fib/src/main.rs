use std::io;

fn print_fib(n: usize) {
    let mut fib = [1, 1, 2];
    if n == 0 {
        println!("not a positive interger");
    } else if n <= 3 {
        println!("fib[{}] = {}", n - 1, fib[n - 1]);
    } else {
        let n = n - 3;
        for i in 0..n {
            fib[0] = fib[1];
            fib[1] = fib[2];
            fib[2] = fib[0] + fib[1];
        }
        println!("fib[{}] = {}", n - 1, fib[2]);
    }
}

fn main() {
    loop {
        println!("输入中...");
        let mut n = String::new();
        io::stdin().read_line(&mut n)
            .expect("Fail to read");
        let n : usize = n.trim().parse()
            .expect("not a positive interger");
        print_fib(n);
    }
}
