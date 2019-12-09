use std::io;

fn main() {

loop{
     
    println!("\n Enter the function you wish to perform. \n");
    println!(" 1) addition  \n 2) Subtraction \n 3) multiplication \n 4) division \n 5) quit \n    Your Choice:");

    let mut choice=String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read line");
    let choice:i32=choice.trim().parse().unwrap();



  if choice == 1 { 

    let mut num1=String::new();
    io::stdin().read_line(&mut num1).expect("Failed to read line");
    let num1:u32=num1.trim().parse().unwrap();

    let mut num2=String::new();
    io::stdin().read_line(&mut num2).expect("Failed to read line");
    let num2:u32=num2.trim().parse().unwrap();

      let  res=num1+num2;
  println!("value 1 is : {} value 2 is : {} \n total addition is : {}",num1,num2,res);
  println!("power of value 1 is : {}",num1.pow(num2));
  println!("would you like to calculate again ? (y/n) ");
  
     let mut c:String=String::new();
     io::stdin().read_line(&mut c).expect("Failed to read line");
     if c.trim() == "n" {
         break
     }

 
  
  }
  else if choice == 2 {

    let mut num1=String::new();
    io::stdin().read_line(&mut num1).expect("Failed to read line");
    let num1:u32=num1.trim().parse().unwrap();

    let mut num2=String::new();
    io::stdin().read_line(&mut num2).expect("Failed to read line");
    let num2:u32=num2.trim().parse().unwrap();

          let  res=num1-num2;
  println!("value 1 is : {} value 2 is : {} \n total subtraction is : {}",num1,num2,res);

  println!("power of value 1 is : {}",num1.pow(num2));

  println!("would you like to calculate again ? (y/n) ");
  
     let mut c:String=String::new();
     io::stdin().read_line(&mut c).expect("Failed to read line");
     if c.trim() == "n" {
         break
     }
  }
  else if choice == 3 {


    let mut num1=String::new();
    io::stdin().read_line(&mut num1).expect("Failed to read line");
    let num1:u32=num1.trim().parse().unwrap();

    let mut num2=String::new();
    io::stdin().read_line(&mut num2).expect("Failed to read line");
    let num2:u32=num2.trim().parse().unwrap();

      let  res=num1*num2;
  println!("value 1 is : {} value 2 is : {} \n total multiplication is : {}",num1,num2,res);
    println!("power of value 1 is : {}",num1.pow(num2));
  println!("would you like to calculate again ? (y/n) ");
  
     let mut c:String=String::new();
     io::stdin().read_line(&mut c).expect("Failed to read line");
     if c.trim() == "n" {
         break
     }
  }
  else if choice == 4 {

    let mut num1=String::new();
    io::stdin().read_line(&mut num1).expect("Failed to read line");
    let num1:u32=num1.trim().parse().unwrap();

    let mut num2=String::new();
    io::stdin().read_line(&mut num2).expect("Failed to read line");
    let num2:u32=num2.trim().parse().unwrap();
      let  res=num1/num2;
  println!("value 1 is : {} value 2 is : {} \n total division is : {}",num1,num2,res);

  println!("power of value 1 is : {}",num1.pow(num2));

  println!("would you like to calculate again ? (y/n) ");
  
     let mut c:String=String::new();
     io::stdin().read_line(&mut c).expect("Failed to read line");
     if c.trim() == "n" {
         break
     }
  }
  else if choice == 5 {
  println!("quit funtion"); 
  break;
  }
  else {
      println!("failed");
  }

}

}

