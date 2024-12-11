use slint::{PhysicalSize, WindowSize};

slint::slint! {
    import { HorizontalBox, VerticalBox, LineEdit, Button } from "std-widgets.slint";

    export component Login inherits Window {
        title: "FBLA Finance Login";
        always-on-top: false;
        background: black;
        out property <string> username;
        in-out property <string> password;
        callback login-pressed <=> login.clicked;
        VerticalBox {
            padding: 10rem;
            LineEdit {
                input-type: text;
                placeholder-text: "username";
                padding-left: 2rem;
                font-size: 3rem;
                edited(text) => {username = text}
            }
            LineEdit {
                input-type: InputType.password;
                placeholder-text: "password";
                font-size: 3rem;
                edited(text) => {password = text}
            }
            login := Button {
                text: "Login";
            }
            Button {
                accessible-role: AccessibleRole.none;
                primary: true;
                text: "Button";
            }
        }
    }
}

fn main() {
    let login = Login::new().unwrap();
    login.window().set_size(WindowSize::Physical(PhysicalSize::new(800, 600)));
    let login_weak = login.as_weak();
    login.on_login_pressed(move || {
        let login = login_weak.upgrade().unwrap();
        let username = login.get_username();
        let password = login.get_password();
        if username != "" || password != "" {
            println!("Login form: {} & {}", username, password);
        }
    });
    
    login.run().unwrap();
    println!("Programmed Ended!");
}