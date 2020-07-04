use yew::prelude::*;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

// * Todos
pub struct TodoList {
    input: String,
    edit_input: String,
    tasks: Vec<Task>,
    link: ComponentLink<Self>,
}

struct Task {
    description: String,
    starred: bool,
    edit: bool,
}

pub enum Message {
    Add,                // * Create new task from input
    Update(String),     // * User typing in new task
    Remove(usize),      // * Remove task at index
    Edit(usize),        // * Changes value of task at index
    UpdateEdit(String), // * Sets the editing input
    Toggle(usize),      // * Toggles which task being edited
    RemoveAll,          // * Clear all tasks
    Nothing,            // * No action
}

impl Component for TodoList {
    type Message = Message;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        TodoList {
            tasks: vec![],
            input: String::new(),
            edit_input: String::new(),
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Message::Add => {
                let item = Task {
                    description: self.input.clone(),
                    edit: false,
                    starred: false,
                };
                self.tasks.push(item);
                self.input.clear();
            }
            Message::Update(s) => {
                self.input = s;
            }
            Message::Remove(i) => {
                self.tasks.remove(i);
            }
            Message::RemoveAll => {
                self.tasks.clear();
            }
            Message::UpdateEdit(s) => {
                self.edit_input = s;
            }
            Message::Edit(i) => {
                let modified = Task {
                    description: self.edit_input.clone(),
                    edit: false,
                    starred: self.tasks.get(i).unwrap().starred,
                };
                self.tasks.remove(i);
                self.tasks.push(modified);
            }
            Message::Toggle(i) => {
                let task = self.tasks.get_mut(i).unwrap();
                task.edit = !task.edit;
            }
            Message::Nothing => {}
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
            <h1>{"Todo List"}</h1>
            <span>
            // Input field
            <input type="text",
              placeholder="New Task",
              value=&self.input,
              oninput=self.link.callback(|e: InputData| Message::Update(e.value)),
              onkeypress=self.link.callback(|e: KeyboardEvent| {
                if e.key() == "Enter" {Message::Add} else {Message::Nothing}
              })
            />
            <button onclick=self.link.callback(|_| Message::Add)>{"Add"}</button>
            </span>
            // List
            <ul>
              { for self.tasks.iter().map(|task| html! { <li> { &task.description } </li> }) }
            </ul>
            </>
        }
    }
}
