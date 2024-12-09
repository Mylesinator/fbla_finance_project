slint::slint! {
    import { HorizontalBox, VerticalBox, LineEdit, Button } from "std-widgets.slint";

    export component App inherits Window {
        in-out property <int> test: 0;
        callback login-pressed <=> login.clicked;
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
                text: "Login " + root.test;
            }
        }
    }
}

fn main() {
    let app = App::new().unwrap();
    let app_weak = app.as_weak();
    app.on_login_pressed(move || {
        let app = app_weak.upgrade().unwrap();
        let mut value = app.get_test();
        value += 1;
        app.set_test(value);
        println!("{value}");
    });
    
    app.run().unwrap();
    println!("Programmed Ended!");
}