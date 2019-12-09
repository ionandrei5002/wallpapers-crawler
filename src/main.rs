use serde_json;
use serde_json::Value;
use rand::Rng;

use std::{io, thread};
use std::fs::File;
use std::time::Duration;
use std::io::Write;

use std::process::Command;

fn save_image(path: &String) {
    let res = reqwest::blocking::get("https://unsplash.com/napi/search/photos?query=universe&xp=&per_page=1&page=1").unwrap();

    println!("Status: {:?}", res.status());
    if res.status() == 200 {
        let body = res.text().unwrap();

        let val: Value = serde_json::from_str(body.as_str()).unwrap();
        println!("images {}", val["total_pages"]);

        let nb_images = val["total_pages"].as_u64().unwrap();
        let page = rand::thread_rng().gen_range(1, nb_images);
        println!("image {}", page);

        let get_image = reqwest::blocking::get((String::from("https://unsplash.com/napi/search/photos?query=universe&xp=&per_page=1&page=") + page.to_string().as_str()).as_str()).unwrap();
        if get_image.status() == 200 {
            let body_image = get_image.text().unwrap();
            let val_image: Value = serde_json::from_str(body_image.as_str()).unwrap();

            let images = val_image["results"].as_array().unwrap();
            let img = &images[0];
            let urls = &img["urls"];

            let str = urls["raw"].to_string().replace("\"", "");
            let parts = str.split("?").collect::<Vec<&str>>();
            let image_link = String::from(parts[0]);

            println!("{}", image_link);

            let mut response = reqwest::blocking::get(image_link.as_str()).unwrap();

            let mut out = File::create(path.as_str()).unwrap();
            match io::copy(&mut response, &mut out) {
                Ok(r) => {
                    println!("bytes wrote {}", r);
                },
                Err(e) => {
                    println!("err {}", e.to_string());
                }
            }
            match out.flush() {
                Ok(()) => {},
                _ => {}
            }
        }
    }
}

fn create_nitrogen_bg_saved(path: &String, image: &String) {
    let mut out = File::create(path.as_str()).unwrap();

    out.write("[xin_0]\n".as_bytes());
    out.write((String::from("file=") + image + "\n").as_bytes());
    out.write("mode=0\nbgcolor=#000000\n\n".as_bytes());

    out.write("[xin_1]\n".as_bytes());
    out.write((String::from("file=") + image + "\n").as_bytes());
    out.write("mode=0\nbgcolor=#000000\n\n".as_bytes());
    out.flush();
}

fn main() -> () {
    let path = String::from("/tmp/bg-wallpaper.jpg");
    let config = String::from("/home/andrei/.config/nitrogen/bg-saved.cfg");
    loop {
        save_image(&path);
        create_nitrogen_bg_saved(&config, &path);

        let comm = Command::new("nitrogen")
            .arg("--force-setter=xinerama")
            .arg("--restore")
            .arg("&").output().unwrap();
        println!("{}", String::from_utf8(comm.stdout).unwrap());

        thread::sleep(Duration::from_secs(300));
    }
}
