fn main() {
    let y = {
        let x = 5;
        x+1
    };

    println!("{}", y );

    fun_ction();
    sum(10, 4);
    println!("fun seven(): {}", seven());
    println!("fun bigger than 10(): {}", is_bigger_than_10(5));
    println!("fun bigger than 10(): {}", is_bigger_than_10(10));
    println!("fun bigger than 10(): {}", is_bigger_than_10(15));
}


fn fun_ction() {
    println!("Just a dumb fun_ction");
}

fn sum(x: i32, y:i32) {
    println!("Sum is {}", x+y);
}

fn seven() -> i16{
    7
}

fn is_bigger_than_10(x: i32) -> bool {
    if x > 10  {
        return true;
    }
    false
}
