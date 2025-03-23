use std::{
    env::{self, args},
    ffi::{OsString, c_void},
    fs::File,
    io::Write,
    os::windows::process::CommandExt,
    path::PathBuf,
    process::Command,
    ptr::{null, null_mut},
    thread::sleep,
    time::{self, Duration, Instant},
};

use service_manager::{
    ServiceInstallCtx, ServiceLabel, ServiceLevel, ServiceManager, ServiceStartCtx, ServiceStopCtx,
    ServiceUninstallCtx,
};
use windows::{
    core::{PCSTR, PCWSTR}, Win32::{
        self, Foundation::HWND, System::Console::AllocConsole, UI::{Shell::{ShellExecuteA, ShellExecuteW}, WindowsAndMessaging::SW_SHOW}
    }
};

fn main() {
    // Create a label for our service
    let exe = env::current_exe().unwrap();
    let label: ServiceLabel = "com.example.whatawhat.hehe".parse().unwrap();

    #[cfg(windows)]
    {
        let exe: String = std::env::current_exe()
            .unwrap()
            .as_os_str()
            .to_os_string()
            .into_string()
            .expect("Couldn't unwrap the string");
        let exe = format!("{} --force", exe);
        println!("{}", exe);
        // let b = exe.encode_utf16().collect::<Vec<_>>();
        let b = exe.as_ptr();

        if args().all(|v| v != "--force") {
            let v = unsafe {
                ShellExecuteA(
                    std::mem::zeroed(),
                    // PCSTR::from_raw("runas\0".encode_utf16().collect::<Vec<_>>().as_ptr()),
                    // PCSTR::from_raw(b.as_ptr()),
                    PCSTR::from_raw(b"runas\0".as_ptr()),
                    PCSTR::from_raw(b),
                    PCSTR::null(),
                    PCSTR::null(),
                    SW_SHOW,
                )
            };
            println!("Invalid {}", v.is_invalid());
            return;
        }
    }

    if args().any(|v| v == "--some-arg") {
        #[cfg(windows)] {
          unsafe { AllocConsole() }.unwrap()
        }
        let mut f = File::options()
            .create(true)
            .truncate(true)
            .write(true)
            .read(true)
            .open("/home/anoromi/code/rust/service-manager-test/test.txt")
            .unwrap();
        loop {
            f.write_all(env::current_dir().unwrap().as_os_str().as_encoded_bytes())
                .unwrap();
            // f.write_all(format!("{}", time::SystemTime::now()).as_bytes())
            //     .unwrap();
            f.write_all(b"\n").unwrap();
            sleep(Duration::from_secs(2));
        }
    }

    // Get generic service by detecting what is available on the platform
    let mut manager = <dyn ServiceManager>::native().expect("Failed to detect management platform");

    // println!("{:?}", manager);

    match manager.set_level(ServiceLevel::User) {
        Ok(_) => println!("Se"),
        Err(_) => {
            eprintln!("Hm")
        }
    }

    // Install our service using the underlying service management platform
    manager
        .install(ServiceInstallCtx {
            label: label.clone(),
            program: exe,
            args: vec![OsString::from("--some-arg --force")],
            contents: None, // Optional String for system-specific service content.
            username: None, // Optional String for alternative user to run service.
            working_directory: None, // Optional String for the working directory for the service process.
            environment: None, // Optional list of environment variables to supply the service process.
            autostart: true, // Specify whether the service should automatically start upon OS reboot.
            disable_restart_on_failure: false, // Services restart on crash by default.
        })
        .expect("Failed to install");

    // Start our service using the underlying service management platform
    // manager
    //     .start(ServiceStartCtx {
    //         label: label.clone(),
    //     })
    //     .expect("Failed to start");

    // Stop our service using the underlying service management platform
    // manager
    //     .stop(ServiceStopCtx {
    //         label: label.clone(),
    //     })
    //     .expect("Failed to stop");

    // Uninstall our service using the underlying service management platform
    manager
        .uninstall(ServiceUninstallCtx {
            label: label.clone(),
        })
        .expect("Failed to stop");
}
