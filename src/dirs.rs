use directories::ProjectDirs;
use std::sync::OnceLock;

static DIRS: OnceLock<ProjectDirs> = OnceLock::new();

pub fn init() {
    DIRS.set(ProjectDirs::from("", "", "silvus").unwrap())
        .unwrap()
}

pub fn get() -> &'static ProjectDirs {
    DIRS.get().unwrap()
}
