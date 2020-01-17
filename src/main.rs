use reqwest;
use std::env;
use select::document::Document;
use select::predicate::{Attr, Name, Predicate};

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

        println!("{}", url);

        let mut res = reqwest::get(&url).unwrap();

        // Make sure the request doesn't fail
        assert!(res.status().is_success(), "URL requested is not valid!");

        let html = res.text().unwrap();

        let document = Document::from(html.as_ref());

        let mut image_url = "https://xkcd.com".to_string();
        // Find the image url in the html
        for element in document.find(Attr("id", "comic").descendant(Name("img"))) {
            image_url.push_str(element.attr("src").unwrap());
            println!("{}", image_url);
        }
}
