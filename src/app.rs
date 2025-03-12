use iced::{widget::{button, Button, text, Text, column}, Element};

use crate::Message;

#[derive(Default)]
pub struct App {
    count: i32
}

impl App {
    pub fn title(&self) -> String {
		String::from("TodoList")
	}
	
    pub fn update(&mut self, message: Message) {
		match message {
            Message::Increment => {
                self.count += 1;
            }
        }
	}
	
    pub fn view(&self) -> Element<Message> {
        let increment: Button<'_, Message> = button("+").on_press(Message::Increment);
		let counter: Text = text(format!("{}", self.count));
        let interface = column![increment];
        let new_interface = interface.push(counter);

        new_interface.into()
	}
}