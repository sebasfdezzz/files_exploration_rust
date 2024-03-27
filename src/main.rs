// fn main() {
//     MainWindow::new().unwrap().run().unwrap();
// }

// slint::slint! {
//     export component MainWindow inherits Window {
//         Text {
//             text: "hello world";
//             color: green;
//         }
//     }
// }

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    ui.on_request_increase_value({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.set_counter(ui.get_counter() + 1);
        }
    });

    ui.on_request_increase_value2({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.set_counter(ui.get_counter() + 2);
        }
    });

    ui.run()
}