pub fn show_in_folder(path: &str) {
    #[cfg(target_os = "macos")]
    {
        let mut command = std::process::Command::new("open");
        command.arg(path);
        command.spawn().unwrap();
    }
}
