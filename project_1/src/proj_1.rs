pub mod triangle {
    use crate::helpers::cli_reader::*;

    enum MathError {
        ValueLessOrEqualToZero,
    }

    struct Triangle {
        a: f32,
        b: f32,
        c: f32,
    }
    impl Triangle {
        fn check(&self) -> Result<bool, MathError> {
            if self.a < 0_f32 || self.b < 0_f32 || self.c < 0_f32 {
                return Err(MathError::ValueLessOrEqualToZero);
            }
            if self.a.powf(2.0) + self.b.powf(2.0) < self.c.powf(2.0) {
                return Ok(false);
            }
            if self.b.powf(2.0) + self.c.powf(2.0) < self.a.powf(2.0) {
                return Ok(false);
            }
            if self.a.powf(2.0) + self.c.powf(2.0) < self.b.powf(2.0) {
                return Ok(false);
            }
            return Ok(true);
        }
    }
    fn float_parse(input: String) -> f32 {
        let a = input.parse::<f32>();
        match a {
            Ok(rez) => {
                return rez;
            }
            Err(e) => panic!(e),
        }
    }

    pub fn check_triangle() {
        println!("Input a");
        let input: String = UserInput::read_line("failed to read");
        let a = float_parse(input);

        println!("Input b");
        let input: String = UserInput::read_line("failed to read");
        let b = float_parse(input);

        println!("Input c");
        let input: String = UserInput::read_line("failed to read");
        let c = float_parse(input);

        let triangle = Triangle { a: a, b: b, c: c };

        println!(
            "is the triangle possible : {}",
            triangle.check().unwrap_or_default()
        );
    }
}
