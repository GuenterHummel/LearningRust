fn main() {
    println!("Hello, world!");
    println!("Gue was here");
    println!("Printing some lines");

    greet_world();

    println!("Juhu!!!");
}

fn greet_world() {
    println!("Hello, world!");

    let southern_germany = "Grüße, Welt";
    let japan = "ハロー・ワールド";
    let icelandic = "Halló heimur";
    let arabic: &str = "مرحبا بالعالم";

    let regions = [southern_germany, japan, icelandic, arabic];

    for region in regions.iter() {
        println!("{}", &region);
    }

    let my_list = ["One", "Two", "Three"];

    for num in &my_list {
        println!("{}", &num);
    }
}
