fn main() {
    println!("Hello, world!");
    println!("Gue was here");
    println!("Printing some lines");

    greet_world();

    println!("Juhu!!!");
}

fn greet_world() {
    println!("Hello, world!");

    let german = "Hallo Welt";
    let japan = "ハロー・ワールド";
    let icelandic = "Halló heimur";
    let arabic  = "مرحبا بالعالم";
    let french = "Bonjour le monde";

    let regions = [german, japan, french, arabic, icelandic];

    for region in regions.iter() {
        println!("{}", &region);
    }

    let my_list = ["One", "Two", "Three"];

    for num in &my_list {
        println!("{}", &num);
    }
}
