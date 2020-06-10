fn main() {
    use pipe_trait::*;
    task_rs::desktop::App::main().pipe(std::process::exit)
}
