slint::slint!{
    import { Button, VerticalBox } from "std-widgets.slint";
    export component App inherits Window {
        property <int> counter: 1;
        GridLayout {
            Text { 
                text: "Hello World! " + counter;
                color: green;
            }
            Row {
                Button { 
                    text: "Click";
                    clicked => {counter += 1;}
                }
            }
        }
    }
}

fn main() {
    let app = App::new().unwrap();
    app.run().unwrap();

    println!("Hello, world!");
}
