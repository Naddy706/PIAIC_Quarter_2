// fn main() {
//     let x = | | println!("hello world"); //make a closure which takes no argument and prints hello world
//     x();
// }



// fn main() {
//     let x = | a:u32 | -> u32 { a + 1}; //Make a closure which takes one u32 data type as argument and returns with adding 1 to it.
//     let  y = 2;
//     println!("The function returns: {}",x(y));
    
// }


// fn main() {
//     let mut c = 1;
//     let mut x = ||{ c = c + 1 };//Make a closure which captures value of variable "c" from environment and change the value of c with adding 1.
//     x();
//     println!("The new value of c is: {}",c); // should print 2
// }




// Write a function which accepts a closure, and in the funciton body,
// it calls the closure. The closure accepts no argument and returns nothing. 
// What should the closure be about? Be creative!



// fn main() {

//     let c =|| { print!("hello")};
//  //   c();

//     abc(c);
// }

// fn abc(x:fn()){


// let jj="world";

// println!("{:?}  {}",x(),jj);

// }



// Write a function which expects a closure as an argument and in the funciton body, 
// it calls the closure. The closure expects u32 argument and returns the u32 value.
// The closure adds 1 to the argument and returns it.

fn clous(cdk: fn(a:u32) -> u32) -> u32{

    cdk(1)

 }

fn main() {

    let dc=clous(|x:u32| x + 1);

    println!("{:?}",dc );


}

