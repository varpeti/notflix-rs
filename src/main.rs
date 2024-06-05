use core::time;
///
/// Dependencies
/// - firefox
/// - mpv
/// - btfs (btplay)
///
use std::{
    env, io,
    process::{Command, Stdio},
    thread,
};

const FILM_SITE: &str = "https://thepiratebay.org/search.php?q=«search_str»";
const SUBTITLE_SITE: &str = "https://www.opensubtitles.com/en/en/search-all/q-«search_str»/hearing_impaired-include/machine_translated-/trusted_sources-";

#[derive(PartialEq)]
enum State {
    GetFilmName,
    GetMagnetLink(String),
    Play(String),
}

fn main() {
    let mut state = process_arguments();
    loop {
        match state {
            State::GetFilmName => {
                state = State::GetMagnetLink(get_film_name());
            }
            State::GetMagnetLink(ref film_name) => {
                show_list(film_name);
                state = State::Play(get_magnet_link());
            }
            State::Play(ref magnet_link) => {
                play(magnet_link);
                break;
            }
        }
    }
}

fn process_arguments() -> State {
    let arg = env::args().nth(1);
    if let Some(arg) = arg {
        if arg.starts_with("magnet:") {
            return State::Play(arg);
        } else {
            return State::GetMagnetLink(arg);
        }
    }
    println!("✨ Usage: notflix-rs <optional film name or magnet link>\n");
    State::GetFilmName
}

fn get_film_name() -> String {
    println!("Which film do you want to watch? 🎬");
    let mut film_name = String::new();
    io::stdin()
        .read_line(&mut film_name)
        .expect("input film_name");
    film_name
}

fn show_list(film_name: &str) {
    println!("Opening Firefox... 🔥🦊");
    let url_film_name = film_name.replace('\n', "").replace(' ', "+");
    // FILM
    let search_string_film_site = FILM_SITE.replace("«search_str»", &url_film_name);
    println!("👉 firefox {}", &search_string_film_site);
    Command::new("firefox")
        .arg(&search_string_film_site)
        .stdout(Stdio::null())
        .spawn()
        .expect("firefox search film");
    // Sleep is reqired, because firefox try to reuse the same window sometimes,
    // and only one site will visible to the user
    thread::sleep(time::Duration::from_millis(250));
    //SUBTITLE
    let search_string_subtitle_site = SUBTITLE_SITE.replace("«search_str»", &url_film_name);
    println!("👉 firefox {}", &search_string_subtitle_site);
    Command::new("firefox")
        .arg(&search_string_subtitle_site)
        .stdout(Stdio::null())
        .spawn()
        .expect("firefox search subtitle");
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
