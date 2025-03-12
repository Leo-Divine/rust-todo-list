mod app;

use app::App;

fn main() -> iced::Result {
	iced::run(App::title, App::update, App::view)
}

#[derive(Debug, Clone)]
enum Message {
    Increment,
}