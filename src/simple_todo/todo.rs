
#[derive(Debug)]
struct Todo {
    title : String,
    description : String,
    status: Status
}


#[derive(Debug,Clone)]
enum Status {
    COMPLETED,
    PENDING,
    PROGRESS,
    NOTSTARTED
}

impl Todo {
      fn add_new(&self, TodoList : &mut Vec<Todo>) {
             let new_todo = Todo {
                title : self.title.clone(),
                description : self.description.clone(),
                status: self.status.clone()
             };
             TodoList.push(new_todo);
      }

      fn display_todos(TodoList : &mut Vec<Todo>) {
        println!("{:?}",TodoList);
      }
      fn remove_todo(index : usize,TodoList : &mut Vec<Todo>){
            TodoList.remove(index);
      }
}

fn main() {
    let mut TodoList:Vec<Todo> = Vec::new();
    println!("We have nothing in the todo,{:?}",TodoList);
    let new_todo = Todo {
        title : "go to the gym".to_string(),
        description: "I have to go to the gym every morning at 7:00 AM".to_string(),
        status: Status::PROGRESS
    };
    new_todo.add_new(&mut TodoList);
    println!("We have a Todo in the list i think : {:?}",TodoList);
    
       
}