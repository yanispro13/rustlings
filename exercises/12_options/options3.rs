// options3.rs
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a hint.

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });
    // Utilisation de "ref" dans le bras Some du match pour emprunter une référence à p au lieu de consommer l'option
    match y {
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        // Remplacement de panic! par println! dans le bras _ du match pour éviter de planter le programme
        _ => println!("no match"),
    }
    //utilisation de la var y apres le match pour montrer qu'elle est toujours accessible
    y; // Fix without deleting this line.
}
