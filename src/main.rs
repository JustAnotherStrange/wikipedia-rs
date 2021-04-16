use std::io::stdin;
use serde_json::Value;
#[tokio::main]
async fn main() {
    println!("enter a search term");
    let mut searchterm = String::new();
    stdin().read_line(&mut searchterm).expect("failed to read line");
    let link = format!("https://en.wikipedia.org/w/api.php?action=query&origin=*&format=json&generator=search&gsrnamespace=0&gsrlimit=5&gsrsearch='{}'", searchterm);
    let resp = reqwest::get(link)
        .await.unwrap()
        .text()
        .await.unwrap();
    let json: Value = serde_json::from_str(&resp).unwrap(); // json deserialize
    // println!("{}", json["query"]["pages"][0]);
    let mut index = 1;
    for (k, v) in json["query"]["pages"].as_object().unwrap().iter() {
        // v is the json value, k is the title (id) of the page.
        println!("{}: {}", index, v["title"]);
        index += 1;
    }
    // let user pick item and link to it and/or copy link to clipboard and/or open in browser
}
