fn main() {
    use iced::{Sandbox, Settings};
    let settings = Settings::default();
    task_rs::desktop::UserInterface::run(settings);
}
