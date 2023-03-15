struct Value {
    idx : usize,
    task: String,
    done: bool
}

struct Todo {
    data: Vec<Value>
}

impl Todo {
    fn new() -> Todo {
        Todo {
            data : vec![]
        }
    }

    fn display(&self) {
        for i in 0..self.data.len() {
            let value: &Value = &self.data[i];
            println!("{} : {} --- {}", value.idx, value.task, if value.done {"DONE"} else {"PENDING"})
        }
    }

    fn add(&mut self, task: String) {
       self.data.push( Value {
          idx: self.data.len()+1,
          task,
          done:false
       }) 
    }

    fn toggle(&mut self, idx: usize) {
        self.data[idx - 1].done = !self.data[idx].done;
    }

    fn delete(&mut self, idx: usize) {
        self.data.remove(idx);
    }
}

pub fn run(name: String) {
   let mut todos = Todo::new();
    println!(">>> START YOUR TODO, {}", name);
    println!("1.>>> DISPLAY");
    println!("2.>>> ADD");
    println!("3.>>> TOGGLE");
    println!("4.>>> DELETE");

    let mut ans = String::from("YES");
    while ans == "YES" {
        println!(">>> YOUR OPTION? : ");
        let mut option = String::from("");
        std::io::stdin().read_line(&mut option).expect("NOT A VALID INPUT");
        let value = option.trim().parse::<usize>().unwrap_or(1);
        match &value {
            1 => todos.display(),
            2 => { 
                    println!(">>> ENTER TASK :");
                    let mut task = String::from("");
                    std::io::stdin().read_line(&mut task).expect("NOT A VALID INPUT");
                    todos.add(task);
            },
            3 =>{
                    println!(">>> ENTER ID :");
                    let mut i = String::from("");
                    std::io::stdin().read_line(&mut i).expect("NOT A VALID INPUT");
                    let idx = option.parse::<usize>().unwrap_or(1);
                    todos.toggle(idx);
            }
            4 => {
                    println!(">>> ENTER ID :");
                    let mut i = String::from("");
                    std::io::stdin().read_line(&mut i).expect("NOT A VALID INPUT");
                    let idx = option.parse::<usize>().unwrap_or(1);
                    todos.delete(idx);
            },
            _ => println!("NOT A VALID OPTION"),
        }
        println!("CONTINUE? (YES/NO) : "); 
        let mut cont = String::new();
        std::io::stdin().read_line(&mut cont).unwrap();
        ans = cont.trim().to_string();
    }
    println!("^-^")
}
