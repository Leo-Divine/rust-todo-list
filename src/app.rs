use iced::{widget::{button, Button, text, Text, text_input, TextInput, column, Column, row}, Element, Length};
use id_counter::S;
use crate::Message;

#[derive(Default)]
pub struct App {
    text_input_content: String,
    todo_list: Vec<Task>
}

mod id_counter {
    static mut NEW_ID: usize = 0;
    pub struct S;
    impl S {
        pub fn new_id(&self) -> usize {
            unsafe {
                NEW_ID += 1;
                NEW_ID
            }
        }
    }
}

struct Task {
    id: usize,
    text: String
}

impl App {
    pub fn title(&self) -> String {
		String::from("TodoList")
	}
	
    pub fn update(&mut self, message: Message) {
		match message {
            Message::Submit => {
                let content: &String = &self.text_input_content;
                self.todo_list.push(Task { id: id_counter::S::new_id(&S), text: content.to_string() });
            }
            Message::TextInputUpdate(content) => {
                self.text_input_content = content;
            }
            Message::TaskCompleted(id) => {
                let index: Option<usize> = self.todo_list.iter().position(|n| n.id == id);
                match index {
                    Some(x) => self.todo_list.remove(x),
                    None => self.todo_list.remove(0), //If the message is fired it must have at least one task in self.todo_list. If it somehow can't find the proper task, it defaults to the first task in the vector.
                };
            }
        }
	}
	
    pub fn view(&self) -> Element<Message> {
        let mut list: Column<'_, Message> = Column::new().width(Length::FillPortion(3u16));
        for row in &self.todo_list {
            let text: Text = text!("• {}", row.text).size(24);
            let checkbox = button("Complete").on_press(Message::TaskCompleted(row.id));
            let task_row = row![text, checkbox].spacing(20);
            list = list.push(task_row);
        }

        let increment: Button<'_, Message> = button("Add").on_press(Message::Submit);
        let input: TextInput<'_, Message> = text_input("Add a Task", &self.text_input_content).on_input(Message::TextInputUpdate);
        let controls  = column![input, increment].width(Length::FillPortion(2u16));
        let interface = row![list, controls];

        interface.into()
	}
}