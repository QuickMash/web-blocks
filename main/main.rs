extern crate web_view;

use std::fs;
use web_view::*;

fn main() {
    let html_content = fs::read_to_string("/assets/html/content.html").expect("Failed to read content.html");

    web_view::builder()
        .title("Rust HTML Layout")
        .content(Content::Html(html_content))
        .size(800, 600)
        .maximized(true) // Set the window to be maximized on start
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .run()
        .unwrap();
}
