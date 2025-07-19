use std::{
    fs,
    ops::Deref,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct Gpu {
    pub driver: String,
    pub model: String,
}

impl Gpu {
    pub fn new() -> Self {
        let drm_path: &Path = Path::new("/sys/class/drm");
        let (mut driver, mut model) = (String::new(), String::new());

        let cards: Vec<String> = fs::read_dir(drm_path)
            .expect("Failed to list directories in `/sys/class/drm`")
            .filter_map(Result::ok)
            .map(|card_dir| card_dir.file_name())
            .map(|card_name| card_name.to_string_lossy().into_owned())
            .filter(|card_name| card_name[4..].chars().all(|c| c.is_ascii_digit()))
            .collect();

        let uevent_files: Vec<String> = cards
            .iter()
            .map(|card_str| {
                fs::read_to_string(
                    PathBuf::from(&drm_path)
                        .join(card_str)
                        .join("device/uevent"),
                )
                .expect(&format!("Could not access {}", &card_str))
            })
            .collect();

        let mut props: u8 = 0;
        for uevent in uevent_files {
            for line in uevent.lines() {
                if props >= 2 {
                    break;
                };
                if line.starts_with("DRIVER") {
                    props += 1;
                    driver = line
                        .split('=')
                        .nth(1)
                        .map(|s| s.to_uppercase().into())
                        .unwrap();
                }
                if line.starts_with("PCI_ID") {
                    props += 1;
                    model = line
                        .split('=')
                        .nth(1)
                        .and_then(|pci_id| match driver.deref() {
                            "NVIDIA" => pci_id.split(':').nth(1),
                            _ => None,
                        })
                        .map(|s| match s.to_uppercase().trim() {
                            "1F02" => "NVIDIA GeForce RTX 2070".to_string(),
                            _ => format!("{} (Unknown)", &s),
                        })
                        .unwrap_or_default();
                } else {
                    model = driver.clone();
                }
            }
        }

        Gpu { driver, model }
    }
}
