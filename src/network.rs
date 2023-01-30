use nix::sys::wait::wait;
use nix::unistd::ForkResult::{Child, Parent};
use nix::unistd::fork;
use exec;

pub fn ping() {

    let yaml = crate::utils::load_yaml("commands.yaml");
    let args = crate::utils::get_args(&yaml, "ping");

    unsafe {
        let pid = fork();

        match pid.expect("Fork failed") {
            Child => {
                let err = exec::Command::new("ping")
                    .args(&args)
                    .exec();

                println!("Error: {err}");
                std::process::exit(1)
            },
            Parent { child: _ } => {
                wait().unwrap();
                println!("Ping completed");
            }
        }
    }
}

pub fn ssh_into() {

    let yaml = crate::utils::load_yaml("commands.yaml");
    let args = crate::utils::get_args(&yaml, "ssh");

    unsafe {
        let pid = fork();

        match pid.expect("Fork failed") {
            Child => {
                let err = exec::Command::new("ssh")
                    .args(&args)
                    .exec();

                println!("Error: {err}");
                std::process::exit(1)
            },
            Parent { child: _ } => {
                wait().unwrap();
                println!("Exited SSH.");
            }
        }
    }
}