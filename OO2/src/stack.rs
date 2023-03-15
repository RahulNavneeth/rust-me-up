pub fn push(vector : &mut Vec<i32>){
    let mut i : i32 = 5;
    while i > 0 {
        println!("ENETR THE NUMBER TO BE PUSHED : ");
        let mut value = String::new();
        std::io::stdin().read_line(&mut value).unwrap();
        match value.trim().parse::<i32>(){
            Ok(i) => vector.push(i),
            Err(_) => println!("NOT VALID"),
        }
      i=i-1;
    }
}

pub fn pop(vector : &mut Vec<i32>) -> i32 {
    let value : i32 = vector[vector.len() - 1];
    vector.pop();
    value
}

pub fn peek(vector : &mut Vec<i32>) -> i32 {
    vector[vector.len() - 1]
}

pub fn display(vector : &mut Vec<i32>){
    println!("STACK -> {:?}" , vector);
}
