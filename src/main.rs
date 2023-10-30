use std::path::PathBuf;
use structopt::StructOpt;

use qli::{from_json, from_yaml};

#[derive(StructOpt)]
struct Cli {
	#[structopt(short = "v", long = "verbose")]
	verbose: bool,
	#[structopt(parse(from_os_str))]
	path: PathBuf,
	#[structopt(short = "o", long = "output", parse(from_os_str))]
	output: Option<PathBuf>,
}

fn main() -> anyhow::Result<()> {
	let args = Cli::from_args();
	let path = args.path;

	let content = std::fs::read_to_string(&path)?;

	println!("path: {:?}", &path);
	let ext = path
		.extension()
		.ok_or(anyhow::anyhow!("Failed to get file extension"))?;

	let response = if ext == "json" {
		from_json(&content)
	} else if ext == "yaml" || ext == "yml" {
		from_yaml(&content)
	} else {
		panic!("Unsupported file extension")
	};

	match response {
		Ok(response) => {
			if args.verbose {
				println!("{:#?}", response);
			}

			if let Some(output) = args.output {
				std::fs::write(output, response.bytes()?)?;
			} else {
				println!("{}", response.text()?);
			}
		}

		Err(e) => {
			println!("Error: {}", e);
		}
	};

	Ok(())
}
