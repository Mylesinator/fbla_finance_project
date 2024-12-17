slint::include_modules!();
use slint::{PhysicalSize, WindowSize};

fn main() {
    let login = Login::new().unwrap();
    let home = Home::new().unwrap();
    login.window().set_size(WindowSize::Physical(PhysicalSize::new(800, 600)));
    let login_weak = login.as_weak();
    login.on_login_pressed(move || {
        let login = login_weak.upgrade().unwrap();
        let username = login.get_username();
        let password = login.get_password();
        if username != "" || password != "" {
            println!("Login form: {} & {}", username, password);
            Window::close;
            home.window().set_size(WindowSize::Physical(PhysicalSize::new(800, 600)));
            home.run().unwrap();
        }
    });
    
    login.run().unwrap();
    println!("Programmed Ended!");
}