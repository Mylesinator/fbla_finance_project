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
    let home = Home::new().unwrap();
    login.window().set_size(WindowSize::Physical(PhysicalSize::new(800, 600)));
    home.window().set_size(WindowSize::Physical(PhysicalSize::new(800, 600)));
    let login_weak = login.as_weak();
    login.on_login_pressed(move || {
        let login = login_weak.upgrade().unwrap();
        let username = login.get_username();
        let password = login.get_password();
        if username != "" || password != "" {
            println!("Login form: {} & {}", username, password);
            handle_visibility(|| home.show());
            handle_visibility(|| login.hide());
        }
    });

    // let home_weak = 
    
    login.run().unwrap();
    println!("Programmed Ended!");
}