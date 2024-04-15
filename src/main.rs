slint::include_modules!();
mod commands;
use crate::commands::commands::{get_disks,recover_files};
use std::fs::{self, File};
use std::io::{self, Read, Write};



fn main() -> Result<(), slint::PlatformError> {
    let disks_output = get_disks();
    let disk="sda1";
    let folder_destino = "/home/sebastianf/Downloads/recovs";
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

    ui.on_recover_jpgs({
        move ||{println!("jpg");
        // match read_disk(disk) {
        //     Ok(file_count) => {
        //         println!("Discovered {} JPEG files in unallocated space of disk {}", file_count, disk);
        //     }
        //     Err(err) => {
        //         eprintln!("Error: {}", err);
        //     }
        // }
        // match recover_files(disk, "jpg", folder_destino) {
        //     Ok(output) => println!("Output of recover_files: {:?}", output),
        //     Err(err) => eprintln!("Error recovering files: {:?}", err),
        // }
        recover_files(disk, "jpg", folder_destino);
    
    }
    });

    ui.on_recover_pngs({
        move ||{println!("png");
        // match recover_files(disk, "png", folder_destino) {
        //     Ok(output) => println!("Output of recover_files: {:?}", output),
        //     Err(err) => eprintln!("Error recovering files: {:?}", err),
        // }
        recover_files(disk, "png", folder_destino);
    }
        
    });

    ui.on_get_disks_ui({
        move ||{println!("disks");}
    });

    ui.on_set_disk({
        move |item|{println!("selected {}",item);}
    });

    ui.run()
}

fn read_disk(disk_name: &str) -> io::Result<usize> {
    // Open the disk as a file
    let mut disk_file = File::open(&("/dev/".to_string() + disk_name))?;

    // Read the entire disk into a buffer
    let mut buffer = Vec::new();
    disk_file.read_to_end(&mut buffer)?;

    // JPEG magic numbers
    let jpeg_magic_numbers: [u8; 3] = [0xFF, 0xD8, 0xFF];

    // Iterate through the buffer searching for JPEG magic numbers
    let mut file_count = 0;
    let mut start_index = 0;
    while let Some(mut index) = buffer[start_index..].windows(3).position(|window| window == jpeg_magic_numbers) {
        // Offset index to start of JPEG
        index += start_index;
        // Find the end of the JPEG
        let mut end_index = index;
        while end_index + 1 < buffer.len() && !(buffer[end_index] == 0xFF && buffer[end_index + 1] == 0xD9) {
            end_index += 1;
        }
        // Save JPEG file
        let jpeg_data = &buffer[index..=end_index];
        let mut file = File::create(format!("jpg_file_{}.jpg", file_count))?;
        file.write_all(jpeg_data)?;

        // Increment file count and move start_index to the byte following the JPEG end marker
        file_count += 1;
        start_index = end_index + 2;
    }

    Ok(file_count)
}