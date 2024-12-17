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

    signup.on_login_pressed(move || {
        // let signup = signup_weak.upgrade().unwrap();
        println!("Sign up!");
    });

    login.on_signup_pressed(move || {
        let login = login_weak.upgrade().unwrap();
        signup.set_username(login.get_username());
        signup.set_password(login.get_password());
        handle_visibility(|| signup.show());
        handle_visibility(|| login.hide());
    });

    login.on_login_pressed(move || {
        let login = login_weak.upgrade().unwrap();
        let username = login.get_username();
        let password = login.get_password();
        if username != "" || password != "" {
            println!("Login form: {} & {}", username, password);
            home.set_username(username);
            handle_visibility(|| home.show());
            handle_visibility(|| login.hide());
        }
    });
    
    login.run().unwrap();
    println!("Programmed Ended!");
}