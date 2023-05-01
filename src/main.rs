pub mod systems;

use systems::System;
use systems::macos::MacOS;
use crate::systems::detect_system;

fn main() {
    let system = detect_system(true);

    system.refresh();

    system.upgrade();
}
