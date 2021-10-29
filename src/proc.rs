use std::process::Command;
use std::str;
use sysinfo::{self, ProcessExt, SystemExt};

struct ZombieProc {
    ppids: Vec<i32>,
}

pub struct ZombiesProcs {
    procs: Vec<ZombieProc>,
}

impl ZombiesProcs {
    pub fn new() -> ZombiesProcs {
        ZombiesProcs { procs: Vec::new() }
    }

    pub fn list_zombies(&mut self) {
        let mut sys = sysinfo::System::new_all();
        sys.refresh_all();

        for (&pid, proc) in sys.processes() {
            let proc_state: String = proc.status().to_string();
            if proc_state == "Zombie" {
                let zombie = ZombieProc {
                    ppids: list_ppids(pid),
                };

                self.procs.push(zombie)
            }
        }
    }

    pub fn kill_processes(&self) {
        for proc in &self.procs {
            for ppid in &proc.ppids {
                println!("Killing PPID: {}", ppid);
                Command::new("kill")
                    .args(["-9", &ppid.to_string()])
                    .spawn()
                    .unwrap();
            }
        }
    }
}

fn list_ppids(pid: i32) -> Vec<i32> {
    let mut ppids: Vec<i32> = Vec::new();

    let pid_str_arg: String = format!("ppid={}", pid.to_string());
    let raw_ppid_output = Command::new("ps")
        .args(["-o", &pid_str_arg])
        .output()
        .expect("Failed to execute ps command.");

    let str_ppid_output: &str = match str::from_utf8(&raw_ppid_output.stdout) {
        Ok(output) => output,
        Err(_) => panic!("Couldn't convert stdout to string."),
    };

    for ppid in str_ppid_output.lines() {
        let p: i32 = ppid.trim().parse().unwrap();
        ppids.push(p);
    }

    ppids
}
