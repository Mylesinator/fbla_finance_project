slint::include_modules!();

use slint::{ComponentHandle, PhysicalSize, PlatformError, Window, WindowSize};

fn switch_window(current_window: &Window, next_window: &Window) -> Result<(), PlatformError> {
    let window_position = current_window.position();
    
    if !current_window.is_maximized() {
        next_window.set_position(window_position);
    }

    next_window.set_size(current_window.size());
    next_window.set_maximized(current_window.is_maximized());
    next_window.set_fullscreen(current_window.is_fullscreen());

    next_window.show()?;
    current_window.hide()?;

    return Ok(());
}

fn main() {
    let login = Login::new().unwrap();
    let signup = SignUp::new().unwrap();
    let home = Home::new().unwrap();

    // only the starting window needs to have this size considering it takes the previous window size when you switch
    login.window().set_size(WindowSize::Physical(PhysicalSize::new(800, 600)));

    let login_weak = login.as_weak();
    let signup_weak = signup.as_weak();

    // let home_weak = home.as_weak();

    signup.on_signup_pressed({
        let signup_weak = signup_weak.clone();
        let login_weak = login_weak.clone();
        move || {
            let signup = signup_weak.upgrade().unwrap();
            let login = login_weak.upgrade().unwrap();
            let username = signup.get_username();
            let password = signup.get_password();
            println!("attempted to Signed up as {username} w/ {password}");
            if username != "" && password != "" {
                signup.set_username("".into());
                signup.set_password("".into());
                let _ = switch_window(signup.window(), login.window());
            }
        }
    });

    signup.on_login_pressed({
        let signup = signup_weak.clone().upgrade().unwrap();
        let login = login_weak.clone().upgrade().unwrap();
        move || {
            login.set_username(signup.get_username());
            login.set_password("".into());
            let _ = switch_window(signup.window(), login.window());
        }
    });

    login.on_signup_pressed({
        let signup = signup_weak.clone().upgrade().unwrap();
        let login = login_weak.clone().upgrade().unwrap();
        move || {
            signup.set_username(login.get_username());
            signup.set_password(login.get_password());
            let _ = switch_window(login.window(), signup.window());
        }
    });

    login.on_login_pressed({
        let login = login_weak.clone().upgrade().unwrap();
        move || {
            let username = login.get_username();
            let password = login.get_password();

            if username != "" && password != "" {
                println!("Login form: {} & {}", username, password);

                let _ = switch_window(login.window(), home.window());
                home.set_username(username);
            }
        }
    });
    
    login.run().unwrap();
    println!("Window closed.");
}