use if_empty::IfEmpty;

#[derive(IfEmpty)]
struct Example {
    string: String,
}

impl Clone for Example {
    fn clone(self: &Example) -> Self {
        let e = Example {
            string: self.string.clone(),
        };
        e
    }
}

impl Example {
    fn new() -> Self {
        Self {
            string: String::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.string.is_empty()
    }
}

fn main() {
    let e = Example::new();

    let e2 = e.if_empty(Example {
        string: "default val".to_string(),
    });

    print!("Example '{}'", e2.string);
}
