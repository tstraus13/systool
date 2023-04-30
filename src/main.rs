pub mod systems;

use systems::System;
use systems::macos::MacOS;

fn main() {
    let mac = MacOS::new(false);

    mac.refresh();
    //mac.upgrade();
}
