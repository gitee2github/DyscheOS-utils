extern crate clap;
use clap::clap_app;

fn main() {
    let _matches = clap_app!(dyschecli =>
		(version: "0.1.0")
		(author: "Guangxing Deng. <dengguangxing@huawei.com>")
		(about: "Cli tools for dysche partition management")
		(@arg help: -h ... "Show dysche-cli help infos")
		(@subcommand list =>
			(about: "Show availible partitions")
			(version: "0.1.0")
			(author: "Guangxing Deng. <dengguangxing@huawei.com>")
			(@arg verbose: -v --verbose "Print partiton information verbosely")
		)
	).get_matches();
}
