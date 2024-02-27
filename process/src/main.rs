use fork::{fork, Fork};
use std::{
    fs::OpenOptions,
    io::{Read, Write},
    os::fd::AsRawFd,
    process::{Command, Stdio},
};

// TODO: go through the fork::* library
// read about the unix fork() syscall
#[allow(unused)]
fn process_fork() {
    let mut x = 0;
    println!("before fork says x is {}", x);

    match fork() {
        Ok(Fork::Parent(_)) => {
            x = 4;
            println!("parent says x is {}", x)
        }
        Ok(Fork::Child) => {
            println!("child says x is {}", x)
        }
        Err(_) => println!("could not fork"),
    }

    println!("after fork says x is {}", x);
}

// TODO: go through the std::fs library
// TODO: read the unix open() syscall
// NOTE: https://stackoverflow.com/questions/31192956/whats-the-de-facto-way-of-reading-and-writing-files-in-rust-1-x
#[allow(unused)]
fn process_concurrent_file_write() -> std::io::Result<()> {
    let file_path = "misc.txt";
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_path)?;

    match fork() {
        Ok(Fork::Parent(_)) => {
            file.write_all(b"Parent writing to file")?;
            println!("Long live the parent")
        }
        Ok(Fork::Child) => {
            file.write_all(b"Child writing to file")?;
            println!("Long live the child")
        }
        Err(_) => println!("could not fork"),
    }

    file.flush()?;

    let mut str_buf = String::new();
    let read_bytes = file.read_to_string(&mut str_buf)?;
    println!(
        "content of {} is >>> {} \n and len is {}",
        file_path, str_buf, read_bytes
    );

    Ok(())
}

// TODO: read the std::process library
// NOTE: https://doc.rust-lang.org/rust-by-example/std_misc/process.html
#[allow(unused)]
fn process_hello_goodbye() {
    match fork() {
        Ok(Fork::Parent(_)) => {
            Command::new("sleep")
                .arg("3")
                .output()
                .expect("sleep call failed");
            println!("goodbye")
        }
        Ok(Fork::Child) => {
            println!("Hello")
        }
        Err(_) => println!("could not fork"),
    }
}

// TODO: go through the std::process library
// TODO: go through the std::io library
#[allow(unused)]
fn process_fork_and_any_exec() -> std::io::Result<()> {
    match fork() {
        Ok(Fork::Parent(_)) => {
            println!("Hola! from Padre")
        }
        Ok(Fork::Child) => {
            let output = Command::new("ls").output()?;
            std::io::stdout().write_all(&output.stdout)?;
        }
        Err(_) => println!("could not fork"),
    }

    Ok(())
}

// TODO: go through the nix::* library
// TODO: checkout libc library and compare with nix
#[allow(unused)]
fn process_waiter(parent_wait: bool, child_self_wait: bool) -> std::io::Result<()> {
    match fork() {
        Ok(Fork::Parent(child)) => {
            println!("parent says child pid is => {}", child);
            if parent_wait {
                let wait_status = nix::sys::wait::wait()?;
                println!("parent says child status is => {:?}", wait_status)
            }
        }
        Ok(Fork::Child) => {
            println!("Child says hey");
            if child_self_wait {
                let _ = nix::sys::wait::wait()?; // OBSERVATION: Panics `Os { code: 10, kind: Uncategorized, message: "No child processes" }`
                println!("child is waiting")
            }
        }
        Err(_) => println!("could not fork"),
    }

    Ok(())
}

// TODO: check out the waitpid() syscall
// TODO: read on when waitpid would be useful
#[allow(unused)]
fn process_pid_waiter() -> std::io::Result<()> {
    match fork() {
        Ok(Fork::Parent(child)) => {
            println!("parent says child pid is => {}", child);
            let wait_status = nix::sys::wait::waitpid(nix::unistd::Pid::from_raw(child), None)?;
            println!("parent says child status is => {:?}", wait_status)
        }
        Ok(Fork::Child) => {
            println!("Child says hey");
        }
        Err(_) => println!("could not fork"),
    }

    Ok(())
}

// TODO: checkout rust std::io library
#[allow(unused)]
fn process_no_stdout() {
    match fork() {
        Ok(Fork::Parent(_)) => {
            println!("parent says hallo");
        }
        Ok(Fork::Child) => {
            println!("child says about to close stdout");
            let fd = std::io::stdout().as_raw_fd();
            nix::unistd::close(fd).unwrap();
            // close_fd().unwrap();
            // OBSERVATION: since the fd as been closed, print does not show up in terminal (or any assumed stdout)
            println!("child says where am i writing the print statement to?")
        }
        Err(_) => println!("could not fork"),
    }
}

fn process_pied_piper() -> std::io::Result<()> {
    let ls = Command::new("cat")
        .arg("Cargo.toml")
        .stdout(Stdio::piped())
        .spawn()?;

    let ls_stdio = Stdio::from(ls.stdout.unwrap());
    let wc = Command::new("wc").arg("-l").stdin(ls_stdio).output()?;

    std::io::stdout().write_all(&wc.stdout)?;
    Ok(())
}

fn main() {
    process_pied_piper().unwrap()
}
