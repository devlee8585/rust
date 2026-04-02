// use std::io;

fn main() {
    //3.1
    // let mut x = 5;
    // println!("The value of x is : {x}");
    // x = 6;
    // println!("The value of x is : {x}");
//////////////////////////////////////////    
    //3.1.2
    // let x = 5;
    // let x = x + 1;
    // {
    //     let x = x * 2;
    //     println!("The value of x in the inner scope is : {x}");

    // }
    // println!("The value of x is : {x}");
//////////////////////////////////////////
    //3.2.2
    // let tup = (500, 6.4, 1);
    // let (x,y,z) = tup;
    // println!("The value of y is : {y}");
//////////////////////////////////////////
    // let x: (i32, f64,u8)  = (500,6.4,1);
    // let five_gundred = x.0;
    // let six_point_four  = x.1;
    // let one  = x.2;

    // println!("The value of six_point_four is : {six_point_four}");
//////////////////////////////////////////
    // let a = [1,2,3,4,5];

    // println!("Please enter an array index.");
    // let mut index = String::new();

    // io::stdin()
    //     .read_line(&mut index)
    //     .expect("Failed to read line");

    // let index: usize = index
    //     .trim()
    //     .parse()
    //     .expect("Index entered was not a number");

    //     let element = a[index];

    //     println!("The value of the element at index {index} is : {element}");

//////////////////////////////////////////
    //3.3
    println!("hello,world!");
    another_function(5, 'h');

}

fn another_function(x:i32, unit_label: char){
println!("Another function. {x}, {unit_label}");

}