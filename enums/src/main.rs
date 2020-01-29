fn main() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call (&self) {
           // Sexy function body
        }
    }

    let message = Message::Write(String::from("Rust enums"));
    message.call();
}
