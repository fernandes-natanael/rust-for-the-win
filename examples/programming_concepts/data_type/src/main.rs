use std::io;

/**

// Integers

// 8 bit
let var: i8 = 8; // or u8 for unsigned

/* this structure repeats for 16, 32 , 64 and 128 bits
 *
 * i16 or u16
 * i32 or u32
 * i64 or u64
 * i128 or u128
 * arch: isize or usize => depends os the architeture of the computer
 */

/*
 *
 * Number literals	Example
 * Decimal	        98_22
 * Hex	            0xff
 * Octal	        0o77
 * Binary	        0b1111_0000
 * Byte (u8 only)	b'A'
 *
 * */

// Decimal
let decimal: f32 = 3.0;

/* By default is used f64
 * f64 or f32
 * */

// Boolean

let valid: bool = true; // or false

// Tuple

let tuple: (bool, u16, f32) = (false, 160, 4.20);

let (x, y, z) = tuple; // you can save the tuple value in variables

let checked = tuple.0 // access index in the tupla

// Arrays

    let array = [5,4,3,2,1];

    let array[i64: 5] = [1,2,3,4,5];

    let array = [11; 3]; // let array = [11, 11, 11];

    let first = a[0];
// Operations

let sum = 5 + 2;

let diff = 95.5 - 4.3;

let mult = 10 * 2;

let quotient = 56.0 / 4.0;

let floored = 3 / 2;

let remainder = 45 % 7;

*/

fn main() {
        let a = [1, 2, 3, 4, 5];

        println!("Please enter an array index.");

        let mut index = String::new();

        io::stdin()
            .read_line(&mut index)
            .expect("Failed to read line");

        let index: usize = index
            .trim()
            .parse()
            .expect("Index entered was not a number");

        let element = a[index];

        println!(
            "The value of the element at index {} is: {}",
            index, element
        );

}
