pub mod commands {
    use std::process::{Command, Output, Stdio};

    pub fn get_disks() -> Result<String, std::io::Error> {
        let lsblk_output = Command::new("lsblk")
                                .arg("-o")
                                .arg("NAME")
                                .arg("-n")
                                .arg("-d")
                                .output()?;
        
        if !lsblk_output.status.success() {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "lsblk command failed"));
        }
    
        Ok(String::from_utf8_lossy(&lsblk_output.stdout).into_owned())
    }
    
    pub fn recover_files(disk: &str, filetype: &str, folder_destino: &str) {
        let mut command = Command::new("photorec");
    
        // Add common options
        command.arg("/log").arg("/d").arg(folder_destino).arg("/cmd");
    
        // Add disk-specific option
        let disk_option = format!("/dev/{}", disk);
        command.arg(disk_option);
    
        // Add file types
        let option = format!("fileopt,everything,disable,{},enable,search", filetype);
        command.arg(option);
    
        // Spawn the command in the background
        command.stdin(Stdio::null())
               .stdout(Stdio::null())
               .stderr(Stdio::null())
               .spawn()
               .expect("Failed to execute command");
    }
    
    fn list_files(path: &str) -> Result<Output, std::io::Error> {
        Command::new("ls")
                .arg(path)
                .output()
    }
    
    fn mount_disk(disk: &str) -> Result<Output, std::io::Error> {
        Command::new("mount")
                .arg(disk)
                .output()
    }
    
    fn create_file(path: &str, name: &str, content: &str) -> Result<Output, std::io::Error> {
        Command::new("sh")
                .arg("-c")
                .arg(format!("echo '{}' > {}/{}", content, path, name))
                .output()
    }
    
    fn read_file(path: &str) -> Result<Output, std::io::Error> {
        Command::new("cat")
                .arg(path)
                .output()
    }
    
    fn copy_file(source: &str, destination: &str) -> Result<Output, std::io::Error> {
        Command::new("cp")
                .arg("-r")
                .arg(source)
                .arg(destination)
                .output()
    }
    
    fn get_wifi_networks() -> Result<Output, std::io::Error> {
        Command::new("nmcli")
                .arg("dev")
                .arg("wifi")
                .arg("list")
                .output()
    }
    
    fn connect_to_wifi(ssid: &str, password: &str) -> Result<Output, std::io::Error> {
        Command::new("nmcli")
                .arg("dev")
                .arg("wifi")
                .arg("connect")
                .arg(ssid)
                .arg("password")
                .arg(password)
                .output()
    }
    
    fn stop_processes(process: &str) -> Result<Output, std::io::Error> {
        Command::new("pkill")
                .arg("-9")
                .arg("-f")
                .arg(process)
                .output()
    }
}
