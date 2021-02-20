use pid_manager::pid::PidManager;
use rand::Rng;
use std::env;
use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

struct Process<'a> {
    pid: u16,
    rnd: usize,
    lock_provider: Arc<Mutex<PidManager>>,
    notify_sender: &'a Sender<u8>,
}

impl<'a> Process<'a> {
    pub fn new(lock: Arc<Mutex<PidManager>>, sender: &'a Sender<u8>) -> JoinHandle<()> {
        Process {
            pid: lock.lock().unwrap().allocate_pid().unwrap(),
            rnd: rand::thread_rng().gen_range(1, 10),
            lock_provider: Arc::clone(&lock),
            notify_sender: sender,
        }
        .work()
    }

    fn work(&self) -> JoinHandle<()> {
        let lock = Arc::clone(&self.lock_provider);
        let pid = self.pid;
        let rnd = self.rnd;
        let thread_sender = self.notify_sender.clone();
        thread::spawn(move || {
            thread::sleep(Duration::from_secs(rnd as u64));
            println!(
                "I am process with pid {} exiting after sleeping for {}",
                pid, rnd
            );

            let mut pid_manager = lock.lock().unwrap();
            pid_manager.release_pid(pid);
            let _ = thread_sender.send(1);
        })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let process_num = match args.get(1) {
        Some(x) => x.parse::<u8>(),
        None => panic!("Pass in the number of processes"),
    }
    .expect("Provide a valid u8 number");

    let pid_manager = PidManager::new();

    let mut processes = vec![];
    let lock = Arc::new(Mutex::new(pid_manager));
    let (sender, receiver) = channel();

    for _ in 0..process_num {
        processes.push(Process::new(lock.clone(), &sender));
    }

    let mut x = 0 as u8;

    while x != process_num {
        x += receiver
            .recv()
            .expect("Something went wrong in receiving the value");
    }
}
