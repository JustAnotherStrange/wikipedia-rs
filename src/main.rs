use std::io::stdin;
use serde_json::Value;
#[tokio::main]
async fn main() {
    println!("enter a search term");
    let mut choice = String::new();
    stdin().read_line(&mut choice).expect("failed to read line");
    println!("how many search results do you want?");
    let mut amount = String::new();
    stdin().read_line(&mut amount).expect("failed to read line");
    let link = format!("https://en.wikipedia.org/w/api.php?action=query&origin=*&format=json&generator=search&gsrnamespace=0&gsrlimit={}&gsrsearch='{}'", amount, choice);
    let resp = reqwest::get(link)
        .await.unwrap()
        .text()
        .await.unwrap();
    let json: Value = serde_json::from_str(&resp).unwrap(); // json deserialize
    // println!("{}", json["query"]["pages"][0]);
    let mut index = 1;
    let mut ids: Vec<&String> = Vec::new();
    for (k, v) in json["query"]["pages"].as_object().unwrap().iter() {
        // v is the json value, k is the title (id) of the page.
        println!("{}: {}", index, v["title"]);
        index += 1;
        ids.push(k);
    }
    println!("enter a number");
    choice.clear();
    stdin().read_line(&mut choice).expect("failed to read line");
    let mut choice_num: usize = choice.trim().parse().unwrap();
    choice_num = choice_num - 1;
    let link_url = format!("https://en.wikipedia.org/w/api.php?action=query&prop=info&format=json&pageids={}&inprop=url", ids[choice_num]);
    let resp_url = reqwest::get(link_url)
        .await.unwrap()
        .text()
        .await.unwrap();
    let json_url: Value = serde_json::from_str(&resp_url).unwrap();
    println!("{}", json_url["query"]["pages"][ids[choice_num]]["fullurl"]);
    // let user pick item and link to it and/or copy link to clipboard and/or open in browser
}
