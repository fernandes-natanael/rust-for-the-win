fn main() {
    let x = 5;

    let x = x + 10;

    println!("result: {}", x);
    {
        let x = x * x;
        println!("result: {}", x);
    }

    println!("result: {}", x);
}
