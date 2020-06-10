fn main() {
    use iced::{Sandbox, Settings};
    let settings = Settings::default();
    task_rs::mvc::Model::run(settings);
}
