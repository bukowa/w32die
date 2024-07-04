use std::{env, io};
use std::io::ErrorKind;
use std::process::{Command, Stdio};

fn main() -> Result<(), std::io::Error> {
    // Collect the command and its arguments from the command line input
    let args: Vec<String> = env::args().collect();

    // Find the position of "--" which separates the wrapper's arguments from the subcommand's arguments
    let separator_pos = args.iter().position(|arg| arg == "--").unwrap_or(1);

    // Extract the binary name from the full path
    let binary_name = std::path::Path::new(&args[0])
        .file_name()
        .unwrap()
        .to_str()
        .unwrap();

    // Command and its arguments should be after "--"
    if separator_pos >= args.len() - 1 {
        eprintln!(
            "Usage: {} [wrapper options] -- <command> [args...]",
            binary_name
        );
        std::process::exit(1);
    }

    // The command to be executed
    let command = &args[separator_pos + 1];
    // The arguments for the command
    let command_args = &args[separator_pos + 2..];

    // Paste code from win32job example ^^
    let job = win32job::Job::create().unwrap();
    let mut info = job.query_extended_limit_info().unwrap();
    info.limit_kill_on_job_close();
    job.set_extended_limit_info(&mut info).unwrap();
    job.assign_current_process().unwrap();
    job.into_handle();

    // Spawn the child process
    let mut child = Command::new(command)
        .args(command_args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .map_err(|e| io::Error::new(ErrorKind::Other, format!("win32die: {}: {}", command, e)))?;

    // Wait for the child process to exit
    let status = child.wait()?;

    // Exit with the same status as the child process
    std::process::exit(status.code().unwrap_or(1));
}