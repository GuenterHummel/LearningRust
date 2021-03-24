fn main() {
    println!("Hello, world!");
    println!("Gue was here");
    println!("Printing some lines");

    greet_world();

    println!("Juhu!!!");
}

fn greet_world() {
    println!("Hello, world!");

    let southern_germany = "Grüß Gott!";
    let japan = "ハロー・ワールド";
    let russian = "Привет, мир";

    let regions = [southern_germany, japan, russian];

    for region in regions.iter() {
        println!("{}", &region);
    }

    let my_list = ["One", "Two", "Three"];

    for num in &my_list {
        println!("{}", &num);
    }
}
