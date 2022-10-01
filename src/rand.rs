use crate::pearson::Sponge;
use std::cell::RefCell;
use std::io::Write;

const HEX: [char; 16] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e',
    'f',
];

thread_local! {
    pub static PS: RefCell<Sponge<64>> = initialized_rand();
}

pub fn hex_digit() -> char {
    let mut b: u8 = 0;

    PS.with(|ps| {
        let mut ps = ps.borrow_mut();
        b = ps.squeeze();
    });

    b &= 0xF;

    HEX[b as usize]
}

pub fn feed_event<S: std::fmt::Debug>(event: S) {
    PS.with(|ps| {
        let mut ps = ps.borrow_mut();
        writeln!(ps, "{:?}", event).unwrap();
        writeln!(ps, "{:?}", std::time::SystemTime::now()).unwrap();
        writeln!(ps, "{:?}", std::time::Instant::now()).unwrap();
    });
}

fn initialized_rand() -> RefCell<Sponge<64>> {
    let mut ps = Sponge::<64>::new();

    // Write application-specific seed
    writeln!(ps, "ipfs API RNG seed").unwrap();

    // Process ID
    writeln!(ps, "Process ID: {:?}", std::process::id()).unwrap();

    // Thread ID and name
    writeln!(ps, "Thread ID: {:?}", std::thread::current().id()).unwrap();
    writeln!(ps, "Thread name: {:?}", std::thread::current().name()).unwrap();

    // Build-time constants for build target
    writeln!(ps, "{:?}", std::env::consts::ARCH).unwrap();
    writeln!(ps, "{:?}", std::env::consts::DLL_EXTENSION).unwrap();
    writeln!(ps, "{:?}", std::env::consts::DLL_PREFIX).unwrap();
    writeln!(ps, "{:?}", std::env::consts::DLL_SUFFIX).unwrap();
    writeln!(ps, "{:?}", std::env::consts::EXE_EXTENSION).unwrap();
    writeln!(ps, "{:?}", std::env::consts::EXE_SUFFIX).unwrap();
    writeln!(ps, "{:?}", std::env::consts::FAMILY).unwrap();
    writeln!(ps, "{:?}", std::env::consts::OS).unwrap();

    // Command line arguments
    writeln!(ps, "Command line arguments").unwrap();

    for arg in std::env::args_os() {
        writeln!(ps, "{:?}", arg).unwrap();
    }

    // Current dir
    writeln!(ps, "Current dir: {:?}", std::env::current_dir()).unwrap();

    // Current executable
    writeln!(ps, "Current exe: {:?}", std::env::current_exe()).unwrap();

    // Temp dir
    writeln!(ps, "Temp dir: {:?}", std::env::temp_dir()).unwrap();

    // Environment variables
    for (key, value) in std::env::vars_os() {
        writeln!(ps, "{:?} -> {:?}", key, value).unwrap();
    }

    // Timestamps
    writeln!(ps, "{:?}", std::time::SystemTime::now()).unwrap();
    writeln!(ps, "{:?}", std::time::Instant::now()).unwrap();

    RefCell::new(ps)
}
