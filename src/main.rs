slint::include_modules!();

use slint::{WindowSize, PhysicalSize, PlatformError};

fn main() -> Result<(), PlatformError> {
    let home = Home::new().unwrap();

    home.window().set_size(WindowSize::Physical(PhysicalSize::new(800, 600)));

    return home.run();
}