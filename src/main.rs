slint::include_modules!();

use slint::{ComponentHandle, PhysicalSize, PlatformError, Window, WindowSize};

fn handle_visibility<F>(action: F)
where
    F: FnOnce() -> Result<(), PlatformError>,
{
    if let Err(e) = action() {
        eprintln!("Error changing visibility: {}", e);
    }
}

fn switch_window(current_window: &Window, next_window: &Window) {
    let window_position = current_window.position();
    
    // fixes the issue where it unmaximizes the window
    // i assume you're not supposed to reposition it when maximized cause it's stretched to the screen
    if !current_window.is_maximized() {
        next_window.set_position(window_position);
    }

    next_window.set_size(current_window.size());
    next_window.set_maximized(current_window.is_maximized());
    
    handle_visibility(|| next_window.show());
    handle_visibility(|| current_window.hide());
}

fn main() {
    let login = Login::new().unwrap();
    let signup = SignUp::new().unwrap();
    let home = Home::new().unwrap();

    // only the starting window needs to have this size considering it takes the previous window size when you switch
    login.window().set_size(WindowSize::Physical(PhysicalSize::new(800, 600)));

    // signup.window().set_size(WindowSize::Physical(PhysicalSize::new(800, 600)));
    // home.window().set_size(WindowSize::Physical(PhysicalSize::new(800, 600)));

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
        let signup = signup_weak.clone().upgrade().unwrap();
        let login = login_weak.clone().upgrade().unwrap();
        move || { switch_window(signup.window(), login.window()); }
    });

    login.on_signup_pressed({
        let signup = signup_weak.clone().upgrade().unwrap();
        let login = login_weak.clone().upgrade().unwrap();
        move || { switch_window(login.window(), signup.window()); }
    });

    login.on_login_pressed({
        let login = login_weak.clone().upgrade().unwrap();
        move || {
            let username = login.get_username();
            let password = login.get_password();

            if username != "" || password != "" {
                println!("Login form: {} & {}", username, password);

                switch_window(login.window(), home.window());
                home.set_username(username);
            }
        }
    });
    
    login.run().unwrap();
    println!("Window closed.");
}