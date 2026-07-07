use std::path::PathBuf;

pub struct FileAssociations;

impl FileAssociations {
    #[cfg(target_os = "linux")]
    pub fn install() -> Result<(), Box<dyn std::error::Error>> {
        let desktop_dir = dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("applications");

        std::fs::create_dir_all(&desktop_dir)?;

        let desktop_file = desktop_dir.join("freatepad.desktop");

        let content = r#"[Desktop Entry]
Name=FreatePad
Comment=A lightweight Markdown editor
Exec=freatepad %f
Icon=text-editor
Terminal=false
Type=Application
MimeType=text/markdown;text/x-markdown;
Categories=TextEditor;Utility;
        "#;

        std::fs::write(&desktop_file, content)?;

        // Update MIME database
        let _ = std::process::Command::new("update-desktop-database")
            .arg(&desktop_dir)
            .status();

        Ok(())
    }

    #[cfg(target_os = "windows")]
    pub fn install() -> Result<(), Box<dyn std::error::Error>> {
        use std::process::Command;

        let exe_path = std::env::current_exe()?;

        // Create file association via registry
        let _ = Command::new("reg")
            .args(&[
                "add",
                "HKCR\\.md",
                "/ve",
                "/t",
                "REG_SZ",
                "/d",
                "FreatePad.Markdown",
                "/f",
            ])
            .status();

        let _ = Command::new("reg")
            .args(&[
                "add",
                "HKCR\\FreatePad.Markdown\\shell\\open\\command",
                "/ve",
                "/t",
                "REG_SZ",
                "/d",
                &format!("\"{}\" \"%1\"", exe_path.display()),
                "/f",
            ])
            .status();

        Ok(())
    }

    #[cfg(not(target_os = "linux"))]
    #[cfg(not(target_os = "windows"))]
    pub fn install() -> Result<(), Box<dyn std::error::Error>> {
        log::warn!("File association not supported on this platform");
        Ok(())
    }

    #[allow(dead_code)]
    pub fn is_installed() -> bool {
        #[cfg(target_os = "linux")]
        {
            let desktop_file = dirs::data_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join("applications")
                .join("freatepad.desktop");

            desktop_file.exists()
        }

        #[cfg(not(target_os = "linux"))]
        {
            false
        }
    }
}
