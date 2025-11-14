//Print nums
fn rec1(n: i32) {
    if n < 1 {
        return;
    };
    rec1(n - 1);
    print!("{} ", n);
}

// fibonacci
fn fibo(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }
    fibo(n - 1) + fibo(n - 2)
}
use std::collections::HashMap;

fn fib(n: i32, mp: &mut HashMap<i32, i32>) -> i32 {
    if let Some(result) = mp.get(&n) {
        return result.clone();
    }
    if n <= 1 {
        return n;
    }
    let result = fib(n - 1, mp) + fib(n - 2, mp);
    mp.insert(n, result);
    result
}
fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let n: i32 = buf.trim().parse().unwrap();
    rec1(n);
    println!("Recursive fib - {}", fibo(n as i32));
    let mut mp: HashMap<i32, i32> = HashMap::new();
    println!("Memoized Recursive fib - {}", fib(6, &mut mp));
    println!("{:#?}", mp);
}
