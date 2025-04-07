use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Immettere posizione da calcolare.");
        return;
    }

    let take_at;
    match args[1].parse::<usize>() {
        Ok(r) => take_at = r,
        Err(_e) => {
            println!("Immettere un valore valido");
            return;
        }
    }

    let mut fib: Vec<u128> = Vec::from([0, 1]);
    calc_fib(take_at, &mut fib);

    println!(
        "Fibonacci alla posizione {} è: {}",
        take_at,
        fib.last().unwrap()
    )
}

fn calc_fib(target: usize, fib: &mut Vec<u128>) {
    while fib.len() <= target {
        let next = fib.last().unwrap() + fib.get(fib.len() - 2).unwrap();
        fib.push(next);
    }
}
