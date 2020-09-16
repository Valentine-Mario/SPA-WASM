// src/lib.rs
use wasm_bindgen::prelude::*;
use yew::prelude::*;
//use yew::services::console::ConsoleService;


struct Todo {
    link:ComponentLink<Self>,
    state:State
}

#[derive(Debug, PartialEq)]
struct List{
    id:usize,
    name:String,
    description:String,
    done:bool
}

struct State{
    todo_list:Vec<List>,
    form_value:String,
    form_desc_value:String
}

enum Msg{
    Add,
    UpdateForm(String),
    UpdateDescForm(String),
    MarkAsComplete(usize),
    DeleteItem(usize)
}

impl Component for Todo {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let todo_list:Vec<List>=vec![];
        Self {
            link,
            state:State{
                todo_list,
                form_value:"".to_string(),
                form_desc_value:"".to_string()
            }
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg{
            Msg::Add=>{
                if !self.state.form_value.is_empty() && !self.state.form_desc_value.is_empty(){
                    let id=self.state.todo_list.len()+1;

                    let list=List{
                        id:id,
                        name:self.state.form_value.to_string(),
                        description:self.state.form_desc_value.to_string(),
                        done:false
                    };

                    self.state.todo_list.push(list);
                }
                self.state.form_value="".to_string();
                self.state.form_desc_value="".to_string();
                
            }
            Msg::UpdateForm(value)=>{
                self.state.form_value=value.to_string();
            }
            Msg::UpdateDescForm(value)=>{
                self.state.form_desc_value=value.to_string();
            }
            Msg::MarkAsComplete(list_id)=>{
                let item = self.state.todo_list.iter().find(|p: && List| p.id == list_id).unwrap();
                
                //construct new list struct
                let complete=List{
                   id:item.id,
                   name: item.name.to_string(),
                   description: item.description.to_string(),
                   done:true
                };

                let index = self.state.todo_list.iter().position(|r| r == item).unwrap();
                
                //remove item from list
                self.state.todo_list.remove(index);
                
                //readd item to the previous index in vector
                self.state.todo_list.insert(index, complete)

                // ConsoleService::log(format!("{:?}", index).as_str())
            }

            Msg::DeleteItem(list_id)=>{
                let item = self.state.todo_list.iter().find(|p: && List| p.id == list_id).unwrap();
                let index = self.state.todo_list.iter().position(|r| r == item).unwrap();
                self.state.todo_list.remove(index);
            }
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let list:Vec<Html>=self.state.todo_list.iter()
        .map(|list:&List|{
            let list_id = list.id;
            html!{
                <div>
                <p>{&list.id}</p>
                <h2>{&list.name}</h2>
                <p>{&list.description}</p>
                <small>{"completed :"} {&list.done}</small>
                <button onclick=self.link.callback(move |_| Msg::MarkAsComplete(list_id))>{"Mark as complete"}</button>
                <button onclick=self.link.callback(move |_| Msg::DeleteItem(list_id))>{"Delte"}</button>
                </div>
            }
        }).collect();

        let add_fields=html!{
            <div>
                <p>{"Simple Todo application build with web assembly!!!"}</p>
                <input 
                oninput=self.link.callback(|e: InputData| Msg::UpdateForm(e.value))
                placeholder="add item to todo" 
                type="text"
                value=&self.state.form_value
                />

                <textarea 
                placeholder="todo description"
                oninput=self.link.callback(|e: InputData| Msg::UpdateDescForm(e.value))
                value=&self.state.form_desc_value/>
                
                <button onclick=self.link.callback(move |_| Msg::Add)>{"Add"}</button>
                </div>
        };


        if self.state.todo_list.len()>0{
            html! { 
                <div>
                {add_fields}
                <span>{list}</span>
                </div> 
            }
        }else{
            html!{
               <div>
               {add_fields}
                <p>{"list currently empty"}</p>
                </div>
            }
        }
       
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Todo>::new().mount_to_body();
}