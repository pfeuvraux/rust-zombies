use sysinfo::{self, SystemExt, ProcessExt};
use std::process::Command;

pub struct ZombieProc {
  pid: i32,
  ppids: Vec<i32>
}

pub struct ZombiesProcs {
  procs: Vec<ZombieProc>
}

impl ZombiesProcs {
  pub fn new() -> ZombiesProcs {
    ZombiesProcs{
      procs: Vec::new()
    }
  }

  pub fn list_zombies(&mut self) {
    let mut sys = sysinfo::System::new_all();
    sys.refresh_all();

    for (&pid, proc) in sys.processes() {
        println!("{:?}", proc);
        let proc_state: String = proc.status().to_string();
        if proc_state== "Zombie" {
          let mut ppids: Vec<i32> = Vec::new();

          match proc.parent() {
            Some(i) => ppids.push(i),
            _ => (),
          }

          let zombie = ZombieProc{
            pid: pid,
            ppids: ppids,
          };

          self.procs.push(zombie)
        }
    }
  }


  pub fn kill_processes(&self) {
    for proc in &self.procs {
      println!("Going on PID: {}", proc.pid);
      println!("{:?}", proc.ppids);
      for ppid in &proc.ppids {
        println!("Killing PPID: {}", ppid);
        Command::new("kill")
          .args(["-9", &ppid.to_string()]);
      }
    }
  }

}
