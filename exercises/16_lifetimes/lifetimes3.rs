// lifetimes3.rs
//
// Lifetimes are also needed when structs hold references.
//
// Execute `rustlings hint lifetimes3` or use the `hint` watch subcommand for a hint.

// Ajout du paramètre de durée de vie 'a pour la struct Book
struct Book<'a> {
    author: &'a str,
    title: &'a str,
}

fn main() {
    let name = String::from("Jill Smith");
    let title = String::from("Fish Flying");
        // Création d'une instance de Book en utilisant les références aux variables name et title
    let book = Book { author: &name, title: &title };

    println!("{} by {}", book.title, book.author);
}
