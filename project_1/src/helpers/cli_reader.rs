pub struct UserInput {}

impl UserInput {
    pub fn read_line(error_msg: &str) -> String {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).expect(error_msg);
        return line.trim().to_string();
    }
}
