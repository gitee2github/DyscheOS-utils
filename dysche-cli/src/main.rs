extern crate clap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const _DYSCHE_OP : &str = "/sys/modules/dysche/op";
const DYSCHE_STS : &str = "/sys/modules/dysche/status";

fn main() {
    let yml = clap::load_yaml!("param.yml");
    let matches = clap::App::from_yaml(yml).get_matches();
    let mut ret = 0;

    if let Some(_sc) = matches.subcommand_matches("list") {
        if _sc.is_present("verbose") {
            println!("List partition details: ");
        } else {
            println!("List partitions:");
        }
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
    }

    if ret > 0 {
        let _ = clap::App::from_yaml(yml).print_long_help();
    }

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


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
