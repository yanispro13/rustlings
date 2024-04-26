// This powerful wrapper provides the ability to store a positive integer value.
// Rewrite it using generics so that it supports wrapping ANY type.

// Execute `rustlings hint generics2` or use the `hint` watch subcommand for a hint.

//ajout de type generique <T> pour que Wrapper puisse stocker n'importe quel type
struct Wrapper<T> {
    value: T,
}

impl<T> Wrapper<T> {
        // Modification de la signature de new pour accepter un argument value de type T
    pub fn new(value: T) -> Self {
        Wrapper { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        // Il est pas necessaire de specifier le type générique 
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        // Spécification explicite du type générique &str
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}
