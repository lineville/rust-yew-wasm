use yew::prelude::*;
use yew::services::storage::{Area, StorageService};
use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct Model {
    link: ComponentLink<Self>,
    storage: StorageService,
    state: TodoList,
}

pub struct TodoList {
    input: String,
    edit_input: String,
    tasks: Vec<Task>,
}

pub struct Task {
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

impl Component for Model {
    type Message = Message;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let storage = StorageService::new(Area::Local).expect("storage was disabled by the user");
        let tasks = TodoList {
            tasks: vec![],
            input: String::new(),
            edit_input: String::new(),
        };
        Model {
            link,
            storage,
            state: tasks,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Message::Add => {
                let item = Task {
                    description: self.state.input.clone(),
                    edit: false,
                    starred: false,
                };
                self.state.tasks.push(item);
                self.state.input.clear();
            }
            Message::Update(s) => {
                self.state.input = s;
            }
            Message::Remove(i) => {
                self.state.tasks.remove(i);
            }
            Message::RemoveAll => {
                self.state.tasks.clear();
            }
            Message::UpdateEdit(s) => {
                self.state.edit_input = s;
            }
            Message::Edit(i) => {
                let modified = Task {
                    description: self.state.edit_input.clone(),
                    edit: false,
                    starred: self.state.tasks.get(i).unwrap().starred,
                };
                self.state.tasks.remove(i);
                self.state.tasks.push(modified);
            }
            Message::Toggle(i) => {
                let task = self.state.tasks.get_mut(i).unwrap();
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
              value=&self.state.input,
              oninput=self.link.callback(|e: InputData| Message::Update(e.value)),
              onkeypress=self.link.callback(|e: KeyboardEvent| {
                if e.key() == "Enter" {Message::Add} else {Message::Nothing}
              })
            />
            <button onclick=self.link.callback(|_| Message::Add)>{"Add"}</button>
            </span>
            // List
            <ul>
              { for self.state.tasks.iter().map(|task| self.view_list_item(&task)) }
            </ul>
            </>
        }
    }
}

impl Model {
    fn view_list_item(&self, task: &Task) -> Html {
        html! { <li> { &task.description } </li> }
    }
}
