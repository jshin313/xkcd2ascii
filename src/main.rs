use reqwest;
use std::env;
use select::document::Document;
use select::predicate::{Attr, Name, Predicate};

// For asciify
use std::str::from_utf8;
use std::path::Path;

use std::io;
use std::fs::File;

fn main() {

        // Takes in command line arguments
        let args: Vec<String> = env::args().collect();

        let mut url = "https://xkcd.com/".to_string();

        // If the a command line argument is supplied
        if args.len() > 1
        {
            url.push_str(&args[1]);
        }
        else
        {
            url = "https://c.xkcd.com/random/comic/".to_string();
        }

        println!("Comic URL: {}", url);

        // Get the url requested by the user or a random one
        let mut res = reqwest::get(&url).unwrap();

        // Make sure the request doesn't fail
        assert!(res.status().is_success(), "URL requested is not valid!");

        let html = res.text().unwrap();

        let document = Document::from(html.as_ref());

        let mut image_url = "https:".to_string();

        let mut picture_name = "";

        // Find the image url in the html
        for element in document.find(Attr("id", "comic").descendant(Name("img"))) {
            picture_name = element.attr("src").unwrap();
            image_url.push_str(picture_name);
            println!("Image URL: {}", image_url);
        }

        let mut resp = reqwest::get(&image_url).expect("request failed");

        picture_name = picture_name.split("/").nth(4).unwrap();

        println!("Picture name: {}", picture_name);

        // Create the picture file
        let mut out = File::create(picture_name.to_string()).expect("failed to create file");
        io::copy(&mut resp, &mut out).expect("failed to copy content");

        // Convert the picture file to ascii
        image_to_ascii(picture_name.to_string());

}

/**
 * The code below is for converting the image to ascii
 * All credit goes to Charlie Edelson at https://github.com/edelsonc/asciify
 * The code below was taken from the repo linked above
 */
fn intensity_to_ascii(value: &u8) -> &str {
    // changes an intensity into an ascii character
    // this is a central step in creating the ascii art
    let ascii_chars  = [
        " ", ".", "^", ",", ":", "_", "=", "~", "+", "O", "o", "*",
        "#", "&", "%", "B", "@", "$"
    ];

    let n_chars = ascii_chars.len() as u8;
    let step = 255u8 / n_chars;
    for i in 1..(n_chars - 1) {
        let comp = &step * i;
        if value < &comp {
            let idx = (i - 1) as usize;
            return ascii_chars[idx]
        }
    }

    ascii_chars[ (n_chars - 1) as usize ]
}

fn image_to_ascii(image_name: String) {

    // open image as new dynamic image
    let img = match image::open(&Path::new(&image_name)) {
        Ok(p) => p,
        Err(_) => panic!("Not a valid image path or could no open image"),
    };

    // resize image as an option if its very large...defaults to screen width
    let dims = vec![80u32, 40u32];

    let img = img.resize_exact(dims[0], dims[1], image::FilterType::Nearest);

    // convert to LUMA and change each greyscale pixel into a character
    let imgbuf = img.to_luma();
    let ascii_art = imgbuf.pixels()
                    .map( |p| intensity_to_ascii(&p[0]) )
                    .fold( String::new(), |s, p| s + p );

    // we have one long string, but we need to chunk it by line
    let subs = ascii_art.as_bytes()
        .chunks(imgbuf.width() as usize)
        .map(from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap();
    for s in subs {
        println!("{}", s);
    }
}
