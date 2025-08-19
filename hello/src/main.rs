fn main() {
    println!("Hello, world!");
    println!("Gue was here");
    println!("Printing some lines");

    greet_world();

    println!("Juhu!!!");
}

fn greet_world() {
    println!("Hello, world!");

    let german: &'static str = "Hallo Welt";
    let japanese: &'static str = "ハロー・ワールド";
    let icelandic: &'static str = "Halló heimur";
    let arabic: &'static str = "مرحبا بالعالم";
    let french: &'static str = "Bonjour le monde";

    let regions: [&'static str; 5] = [german, japanese, french, arabic, icelandic];

    for region in regions.iter() {
        println!("{}", &region);
    }

    let my_list: [&'static str; 3] = ["One", "Two", "Three"];

    for num in &my_list {
        println!("{}", &num);
    }
}
