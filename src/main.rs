use memoize::memoize;

fn main() {
    let greeting = greet("Ben", true);
    println!("Greeting is {greeting}");
    println!("{}", fib(35));
}

fn greet(name: &str, is_intro: bool) -> String {
    let intro = "Hi guys";
    let outro = "Bye then";
    let prefix = if is_intro { intro } else { outro };
    return format!("{prefix}, it's {name}")
}
#[memoize]
fn fib(n: u32) -> u64 {
    match n {
        0 | 1 => 1,
        _ => fib(n - 1) + fib(n - 2)
    }
}