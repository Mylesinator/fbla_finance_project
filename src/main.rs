slint::include_modules!();
use slint::{PhysicalSize, WindowSize, PlatformError, ComponentHandle};

fn handle_visibility<F>(action: F)
where
    F: FnOnce() -> Result<(), PlatformError>,
{
    if let Err(e) = action() {
        eprintln!("Error changing visibility: {}", e);
    }
}

fn switch_window(current_window: Box<slint::Weak<dyn ComponentHandle>>, next_window: Box<slint::Weak<dyn ComponentHandle>>) {
        let current_window = current_window.upgrade().unwrap();
        let next_window = next_window.upgrade().unwrap();
        let window_position = current_window.window().position();
        next_window.window().set_position(window_position);
        next_window.window().set_size(current_window.window().size());
        handle_visibility(|| next_window.show());
        handle_visibility(|| current_window.hide());
}

fn main() {
    let login = Login::new().unwrap();
    let signup = SignUp::new().unwrap();
    let home = Home::new().unwrap();
    login.window().set_size(WindowSize::Physical(PhysicalSize::new(800, 600)));
    signup.window().set_size(WindowSize::Physical(PhysicalSize::new(800, 600)));
    home.window().set_size(WindowSize::Physical(PhysicalSize::new(800, 600)));
    let login_weak = login.as_weak();
    let signup_weak = signup.as_weak();
    // let home_weak = home.as_weak();

    signup.on_signup_pressed({
        // let signup_weak = signup_weak.clone();
        move || {
            // let signup = signup_weak.upgrade().unwrap();
            // println!("{}", signup);
            println!("Sign up!");
        }
    });

    signup.on_login_pressed({
        let signup_weak = signup_weak.clone();
        let login_weak = login_weak.clone();
    });

    login.on_signup_pressed({
        let login_weak = login_weak.clone();
        let signup_weak = signup_weak.clone();
        move || {
            let login = login_weak.upgrade().unwrap();
            let signup = signup_weak.upgrade().unwrap();
            signup.set_username(login.get_username());
            signup.set_password(login.get_password());
            let window_position = login.window().position();
            signup.window().set_position(window_position);
            signup.window().set_size(login.window().size());
            handle_visibility(|| signup.show());
            handle_visibility(|| login.hide());
        }
    });

    login.on_login_pressed({
        let login_weak = login_weak.clone();
        move || {
            let login = login_weak.upgrade().unwrap();
            let username = login.get_username();
            let password = login.get_password();
            if username != "" || password != "" {
                println!("Login form: {} & {}", username, password);
                home.set_username(username);
                handle_visibility(|| home.show());
                handle_visibility(|| login.hide());
            }
        }
    });
    
    login.run().unwrap();
    println!("Programmed Ended!");
}