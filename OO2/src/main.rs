mod stack;

fn main() {
    let mut vector = Vec::new();
    let mut ans : String = String::from("YES");
    while ans == "YES" {
        println!("1. PUSH");
        println!("2. POP");
        println!("3. PEEK");
        println!("4. DISPLAY");
        let mut op = String::new();
        std::io::stdin().read_line(&mut op).unwrap();
        match op.trim().parse::<i32>(){
           Ok(i) => {
               if i == 1 {
                   stack::push(&mut vector);
               }
               else if i == 2 {
                   println!("POPPED ELEMENT -> {}" , stack::pop(&mut vector));
               }
               else if i == 3 {
                   println!("PEEK ELEMENT -> {}" , stack::peek(&mut vector));
               }
               else if i == 4 {
                   stack::display(&mut vector);
               }
               else {
                   println!("INVALID");
               }
              println!("CONTINUE? (YES/NO) : "); 
              let mut cont = String::new();
              std::io::stdin().read_line(&mut cont).unwrap();
              ans = cont.trim().to_string();
           },
           Err(_) => println!("NOT VALID"),
         }
    }
    println!("\n^-^")
}
