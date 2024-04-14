slint::include_modules!();
mod commands;
use crate::commands::commands::get_disks;



fn main() -> Result<(), slint::PlatformError> {
    let disks_output = get_disks();
    match disks_output {
        Ok(output) => println!("Output of get_disks: {:?}", output),
        Err(err) => eprintln!("Error obtaining disks: {:?}", err),
    }
    let ui = AppWindow2::new()?;

    ui.on_request_increase_value2({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.set_counter(ui.get_counter() + 2);
        }
    });

    ui.run()
}