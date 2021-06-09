extern crate clap;

fn main() {
    let yml = clap::load_yaml!("param.yml");
    let matches = clap::App::from_yaml(yml).get_matches();

    if let Some(_sc) = matches.subcommand_matches("list") {
        if _sc.is_present("verbose") {
            println!("List partition details: ");
        } else {
            println!("List partitions:");
        }
    } else if let Some(_sc) = matches.subcommand_matches("create") {
        println!("create app partition:");
    } else {
        let _ = clap::App::from_yaml(yml).print_long_help();
    }

}
