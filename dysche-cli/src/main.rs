extern crate clap;
use std::fs::File;
use std::io::{self, BufRead};
use std::io::Write;
use std::fs::OpenOptions;

const DYSCHE_OP  : &str = "/sys/modules/dysche/op";
const DYSCHE_STS : &str = "/sys/modules/dysche/status";

fn main() {
    let yml = clap::load_yaml!("param.yml");
    let matches = clap::App::from_yaml(yml).get_matches();
    let mut ret = 0;

    if let Some(_sc) = matches.subcommand_matches("list") {
        let mut verb = false;
        if _sc.is_present("verbose") {
            println!("List partition details: ");
            verb = true;
        } else {
            println!("List partitions:");
        }
        list_partitions(verb);
    } else if let Some(sc) = matches.subcommand_matches("create") {
        println!("create app partition:");

        let cpus = sc.value_of("cpu").unwrap_or("");
        let kernel = sc.value_of("kernel").unwrap_or("");

        if cpus == "" {
            ret = 1;
            println!("Core (lists) is needed.");
        }

        if kernel == "" {
            ret = 1;
            println!("kernel for the newly created partition is needed.");
        }

        if ret <= 0 {
            println!("  creating a new partition, running {} on cores {}", kernel,
                     cpus);
            ret = create_partition(cpus, kernel, "console=ttyS0, 115200", "acpi_devs: ");
        }
    } else if let Some(sc) = matches.subcommand_matches("destroy") {
        let pid = sc.value_of("pid").unwrap_or("-1");
        ret = destroy_partition(pid);
    }  else if let Some(sc) = matches.subcommand_matches("migrate") {
        println!("migrate resources between partitions:");

        let cpus = sc.value_of("cpu").unwrap_or("");
        let sp = sc.value_of("source_partition").unwrap_or("");
        let dp = sc.value_of("dest_partition").unwrap_or("");

        if cpus == "" {
            ret = 1;
            println!("Core (lists) is needed.");
        }

        if sp == "" || dp == "" {
            ret = 1;
            println!("source & dest partitions need be specified.");
        }

        if ret <= 0 {
            println!("  migrate cpu {} from partition {} to partition {}", cpus,
                     sp, dp);
            ret = migrate_partition(sp, dp, cpus);
        }
    }

    if ret > 0 {
        let _ = clap::App::from_yaml(yml).print_long_help();
    }

}

fn list_partitions(_verbose: bool) -> i32 {
    let mut ret = 0;
    println!("Read partition information form : {}", DYSCHE_STS);

    if let Ok(lines) = read_lines(DYSCHE_STS) {
        for line in lines {
            if let Ok(l) = line {
                println!("{}", l);
            }
        }
    } else {
        println!("{} is not present.", DYSCHE_STS);
        println!("  check if the kernel module is enabled or not.");
        println!();
        ret = 1;
    }

    return ret;
}

fn create_partition(cpus: &str, kernel_img: &str, kernel_param: &str, dev_list: &str) -> i32 {
    let mut ret = 0;
    println!("Info: cpu: {}, kernel: {}, kernel_param: {}, dev_list: {}",
             cpus, kernel_img, kernel_param, dev_list);

    if let Ok(lines) = read_lines(DYSCHE_STS) {
        for line in lines {
            if let Ok(l) = line {
                println!("{}", l);
            }
        }
    } else {
        println!("{} is not present.", DYSCHE_STS);
        println!("  check if the kernel module is enabled or not.");
        println!();
        ret = 1;
    }

    return ret;
}

fn destroy_partition(pid: &str) -> i32 {
    let mut _ret = 0;

    println!("Will force destroy partition {}", pid);

    let cmd = format!("destory {}", pid);
    _ret = write_line(DYSCHE_OP, &cmd);

    if _ret == 0 {
        println!("partition {} is destoried.", pid);
    } else {
        println!("{} is not present.", DYSCHE_OP);
        println!("  check if the kernel module is enabled or not.");
        println!();
        _ret = 1;
    }

    return _ret;
}

fn migrate_partition(_sp: &str, _dp: &str, _cpus: &str) -> i32 {
    println!("place holder. need impl later.");

    return 0;
}

fn write_line(filename: &str, line: &str) -> i32 {
    let mut ret = 0;

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(filename)
        .unwrap();

    match file.write_all(line.as_bytes()) {
        Err(_e) => {
            println!("Write error");
            ret = -1;
        }
        Ok(_) => {
            println!("Write success");
            //file.sync_all();
        }
    }

    return ret;
}

fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = match File::open(filename) {
        Err(why) => panic!("couldn't open {}: {}", &filename, why),
        Ok(file) => file,
    };
    Ok(io::BufReader::new(file).lines())
}
