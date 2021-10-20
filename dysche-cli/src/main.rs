extern crate clap;
use std::fs::File;
use std::io::{self, BufRead};
use std::io::Write;
use std::fs::OpenOptions;

static DYSCHE_OP  : &str = "/sys/modules/dysche/op";
static DYSCHE_STS : &str = "/sys/modules/dysche/status";

enum DyscheErrorCode {
    DEOK,
    DENOCPU,
    DENOKERNEL,
    DENOSPDP,
    DENULL,
}

fn main() {
    let yml = clap::load_yaml!("param.yml");
    let matches = clap::App::from_yaml(yml).get_matches();
    let mut ret : Result<i32, DyscheErrorCode> = Err(DyscheErrorCode::DENULL);

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
            ret = Err(DyscheErrorCode::DENOCPU);
            println!("Core (lists) is needed.");
        }

        if kernel == "" {
            ret = Err(DyscheErrorCode::DENOKERNEL);
            println!("kernel for the newly created partition is needed.");
        }

        match ret {
            Ok(_) => {
                println!("  creating a new partition, running {} on cores {}", kernel,
                     cpus);
                let _ret = create_partition(cpus, kernel, "console=ttyS0, 115200", "acpi_devs: ");
            },
            Err(_) => {},
        }
    } else if let Some(sc) = matches.subcommand_matches("destroy") {
        let pid = sc.value_of("pid").unwrap_or("-1");
        ret = destroy_partition(pid);
    } else if let Some(sc) = matches.subcommand_matches("show") {
        let pid = sc.value_of("pid").unwrap_or("-1");
        ret = show_partition(pid);
    }  else if let Some(sc) = matches.subcommand_matches("migrate") {
        println!("migrate resources between partitions:");

        let cpus = sc.value_of("cpu").unwrap_or("");
        let sp = sc.value_of("source_partition").unwrap_or("");
        let dp = sc.value_of("dest_partition").unwrap_or("");

        if cpus == "" {
            ret = Err(DyscheErrorCode::DENOCPU);
            println!("Core (lists) is needed.");
        }

        if sp == "" || dp == "" {
            ret = Err(DyscheErrorCode::DENOSPDP);
            println!("source & dest partitions need be specified.");
        }

        match ret {
            Ok(_) => {
                println!("  migrate cpu {} from partition {} to partition {}", cpus,
                     sp, dp);
                ret = migrate_partition(sp, dp, cpus);
            },
            Err(_) => {},
        }
    }

    match ret {
        Ok(_) => println!("success."),
        Err(e) => {
            match e {
                _ => {},
            }
            let _ = clap::App::from_yaml(yml).print_help();
        },
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

fn create_partition(cpus: &str, kernel_img: &str, kernel_param: &str, dev_list: &str) -> Result<i32, DyscheErrorCode> {
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

    match ret {
        0 => Ok(ret),
        _ => Err(DyscheErrorCode::DEOK),
    }
}

fn destroy_partition(pid: &str) -> Result<i32, DyscheErrorCode> {
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

    return Ok(_ret);
}

fn show_partition(pid: &str) -> Result<i32, DyscheErrorCode> {
    println!("The details of the partition {}", pid);
    println!("place holder. need impl later.");

    return Ok(0);
}

fn migrate_partition(_sp: &str, _dp: &str, _cpus: &str) -> Result<i32, DyscheErrorCode> {
    println!("place holder. need impl later.");

    return Ok(0);
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
