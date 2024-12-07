slint::slint!{
    export component App inherits Window {
        Text { text: "Hello World!"; }
    }
}

fn main() {
    println!("Hello, world!");
}
