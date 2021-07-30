# IfEmpty derive macro

This crate provides a derive macro implementing the [if_empty](https://crates.io/crates/if_empty) functionality if the type has an `is_empty` function.

```rust
#[derive(IfEmpty)]
struct Example {
    value: String,
}

impl Example {
    fn is_empty(&self) -> bool {
        self.value.is_empty()
    }
}

fn main() {
    let example = Example {
        value: String::new(),
    };
    print!("Example '{}'", example.value);
    // Example ''

    let not_empty_example = e.if_empty(Example {
        value: "default val".to_string(),
    });

    print!("Example '{}'", not_empty_example.value);
    // Example 'default val'
}
```
