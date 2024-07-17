#[macro_export]
macro_rules! version {
    () => {
        pub mod version {
            use git_version::git_version;
            pub const RAW_VERSION: &str =
                git_version!(args = ["--tags", "--dirty"], fallback = "unknown version");
            pub fn create_version_str() -> std::io::Result<()> {
                const VERSION_LEN_MAX: usize = 32;
                let _ = std::fs::remove_file("src/version.rs");
                let version = if RAW_VERSION.len() >= VERSION_LEN_MAX{
                    RAW_VERSION.split_at(VERSION_LEN_MAX).0
                } else {
                    RAW_VERSION
                };
                std::fs::write(
                    "src/version.rs",
                    format!("///Version &str extracted by git described\npub const VERSION: &str = \"{}\";\n/// Maximum possible Length of the Version &str possible \npub const VERSION_LEN_MAX: usize = {};", version, VERSION_LEN_MAX),
                )?;
                Ok(())
            }
        }
        version::create_version_str().expect("Could not create Version file");
    };
}
