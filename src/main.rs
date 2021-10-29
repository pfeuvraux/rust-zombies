use argparse;
mod proc;

fn main() {

    let mut kill_processes: bool = false;

    {
        let mut argparser = argparse::ArgumentParser::new();
        argparser.set_description("List and kill zombie processes.");

        // to kill or not to kill
        argparser.refer(&mut kill_processes)
            .add_option(&["-k", "--kill"], argparse::StoreTrue,
                "Enable killing processes.");

        argparser.parse_args_or_exit();
    }

    let mut zombie_procs = proc::ZombiesProcs::new();
    zombie_procs.list_zombies();


    if kill_processes {
        zombie_procs.kill_processes();
    }
}


