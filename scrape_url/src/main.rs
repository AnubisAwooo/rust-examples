fn main() {
    let url = std::env::args()
        .skip(1)
        .next()
        .unwrap_or("https://www.rust-lang.org".into());
    let output = "rust.md";

    println!("Fetching url: {}", url);
    let body = reqwest::blocking::get(url).unwrap().text().unwrap();

    println!("Converting html to markdown...");
    let md = html2md::parse_html(&body);

    std::fs::write(output, md.as_bytes()).unwrap();

    println!("Converted markdown has been saved in {}.", output);
}
