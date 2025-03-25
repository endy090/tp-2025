use nix::unistd::{fork, ForkResult};
use nix::sys::wait::waitpid;
use std::process;

fn main() {
    match unsafe { fork() } {
        Ok(ForkResult::Parent { child }) => {
            println!("Parent process: PID = {}", process::id());
            println!("Child created with PID = {}", child);
            
            // Attendre que l'enfant se termine
            waitpid(child, None).unwrap();
            println!("Child process terminé.");
        }
        Ok(ForkResult::Child) => {
            println!("Child process: Je suis le processus enfant !");
            process::exit(0); // Terminer le processus enfant proprement
        }
        Err(_) => {
            eprintln!("Échec du fork !");
        }
    }
}
