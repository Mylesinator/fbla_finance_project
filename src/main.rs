slint::slint!{
    import { HorizontalBox, VerticalBox, LineEdit, Button } from "std-widgets.slint";

    export component Recipe inherits Window {
        VerticalBox {
            padding: 10rem;
            username := LineEdit {
                placeholder-text: "username";
            }
            password := LineEdit {
                input-type: InputType.password;
                placeholder-text: "password";
            }
            login := Button {
                text: "Login";
            }
        }
    }
}

fn main() {
    let app = Recipe::new().unwrap();
    app.run().unwrap();

    println!("End Program");
}