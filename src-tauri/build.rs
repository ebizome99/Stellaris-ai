fn main() {
    tauri_build::build();
    
    #[cfg(windows)]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("icons/icon.ico");
        res.set("ProductName", "Stellaris AI");
        res.set("FileDescription", "Enterprise AI Image Generation System");
        res.set("LegalCopyright", "Copyright © 2024 Stellaris AI Team");
        res.compile().unwrap();
    }
}
