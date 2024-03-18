///
/// Dependencies
/// - firefox
/// - mpv
/// - btfs (btplay)
///
use std::{
    io,
    process::{Command, Stdio},
};

const BASE_URL: &str = "https://thepiratebay.org/";
const SEARCH: &str = "search.php?q=«search_str»";

fn main() {
    let film_name = get_film_name();
    show_list(&film_name);
    let magnet_link = get_magnet_link();
    play(&magnet_link);
}

fn get_film_name() -> String {
    println!("Which film do you want to watch? 🎬");
    let mut film_name = String::new();
    io::stdin()
        .read_line(&mut film_name)
        .expect("input film_name");
    film_name.replace('\n', "").replace(' ', "+")
}

fn show_list(film_name: &str) {
    println!("Opening Firefox... 🔥🦊");
    let search_string = format!("{}{}", BASE_URL, SEARCH.replace("«search_str»", film_name));
    println!("👉 firefox {}", &search_string);
    Command::new("firefox")
        .arg(&search_string)
        .stdout(Stdio::null())
        .spawn()
        .expect("firefox search");
}

fn get_magnet_link() -> String {
    println!("Please insert 🧲 link here:");
    let mut magnet_link = String::new();
    io::stdin()
        .read_line(&mut magnet_link)
        .expect("input film_name");
    magnet_link.replace('\n', "")
}

fn play(magnet_link: &str) {
    println!("Starting btfs (btplay) 📃 with mpv 📺...");
    println!("👉 btplay -p mpv «magnet_link»");
    let status = Command::new("btplay")
        .args(["-p", "mpv", magnet_link])
        .status()
        .expect("btplay mpv");
    println!("{:#?}", status);
}
