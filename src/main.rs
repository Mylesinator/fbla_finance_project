slint::slint!{
    import { Button } from "std-widgets.slint";
    export component App inherits Window {
        Text { text: "Hello World!"; }
        Button { text: "Click"; }
    }
}

fn main() {
    App::new().unwrap().run().unwrap();
    println!("Hello, world!");
}
