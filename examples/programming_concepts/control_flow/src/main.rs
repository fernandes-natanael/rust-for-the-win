fn main() {

    //Expressions

    let check: i8 = 11;

    if check > 10 {
        println!("It's bigger than 10");
    
    } else if check < 10{
        println!("It's smaller than 10");
    
    } else {
        println!("It's equals to 10");
    }


    let check: bool = true;
    let num = if check { 100 } else {0}; 

    println!("Jojo is good? {}!", num);


    // Repeating
    
    // infinity
    let mut i: i8 = 0;
    loop {
        
        if i < 10 {
            println!("Oh god we will never left this loop");
        } else {
            println!("Finally! freedom");
            break;
        }
        i += 1;
    }

    // Loop label
    'shrek_loop: loop {
        let mut x = 0;
        loop {
            if x > 20 {
                println!("shrek 4 was annouce, I don't know how to write this word");
                break 'shrek_loop;
            }
            x +=1;
        }
    }

    let mut counter = 0;
    // Loop returns WTF!
    let result = loop {
        counter += 1;
        if counter > 30 {
            break counter * 42;
        }
    };

    println!("Result {}", result);

    // Good old WHILE

    counter = 10;
    while counter > 0 {
        println!("Count {}", counter);
        counter -= 2;
        
    }

    let a = [1, 10, 11, 100, 101, 110];
    let mut index = 0;

    while index < 5 {
        println!("the value {} is {}", index, a[index]);
        index += 1;
    }

    // for loop (in this case he is in reverse)

    for num in (1..10).rev() {
        println!("for {}", num);
    }

    // foreach loop

    for elem in a {
        println!("element is {}", elem);
    }
}
