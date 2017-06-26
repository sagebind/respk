extern crate getopts;
extern crate respk;
extern crate walkdir;

use respk::Package;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io;
use std::process::exit;
use walkdir::WalkDir;


macro_rules! printerr {
    ($fmt:expr) => {
        use ::std::io::{stderr, Write};
        let _ = writeln!(stderr(), $fmt);
    };
    ($fmt:expr, $($arg:tt)*) => {
        use ::std::io::{stderr, Write};
        let _ = writeln!(stderr(), $fmt, $($arg)*);
    };
}

fn main() {
    let mut options = getopts::Options::new();

    options.optflag("h", "help", "Show this help message");
    options.optflag("v", "version", "Show the program version");

    let mut args = options.parse(env::args()).unwrap_or_else(|e| {
        printerr!("{}", e);
        exit(1);
    });

    if args.opt_present("h") {
        println!("{}", options.usage(
"Usage: respk <command> [options] <package> [files...]

Commands:
    add         Add resources to package
    delete      Delete resources from package
    list        List resources in package
    extract     Extract resources from package"
        ));
        return;
    }

    if args.free.len() < 2 {
        printerr!("No command specified");
        exit(1);
    }

    if args.free.len() < 3 {
        printerr!("No package specified");
        exit(1);
    }

    let command = args.free.remove(1);
    let package_path = args.free.remove(1);
    let paths = &args.free[1..];

    let package = Package::open(package_path).unwrap_or_else(|_| {
        printerr!("Failed to open package.");
        exit(1);
    });

    let result = match command.as_ref() {
        "add" => add(package, paths),
        "delete" => delete(package, paths),
        "list" => list(package),
        "extract" => extract(package),
        _ => {
            printerr!("Unknown command: '{}'", command);
            exit(1);
        },
    };

    if let Err(e) = result {
        printerr!("{}", e);
    }
}

fn add(package: Package, paths: &[String]) -> Result<(), Box<Error>> {
    for path in paths {
        for entry in WalkDir::new(path) {
            if let Ok(entry) = entry {
                if let Ok(full_path) = entry.path().canonicalize() {
                    println!("{}", full_path.display());

                    if let Ok(file) = File::open(full_path) {
                        if let Some(name) = entry.path().to_str() {
                            package.write(name, file).unwrap();
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

fn delete(package: Package, paths: &[String]) -> Result<(), Box<Error>> {
    for path in paths {
        if let Err(respk::Error::ResourceNotFound) = package.delete(path) {
            printerr!("Resource not found: {}", path);
        }
    }

    Ok(())
}

fn list(package: Package) -> Result<(), Box<Error>> {
    for resource in package.resources().unwrap() {
        println!(
            "{:16} {}B ({}B compressed)",
            resource.path(),
            resource.size(),
            resource.compressed_size(),
        );
    }

    Ok(())
}

fn extract(package: Package) -> Result<(), Box<Error>> {
    for resource in package.resources().unwrap() {
        let mut file = File::create(resource.path())?;
        let mut resource = package.stream(resource.path()).unwrap();

        io::copy(&mut resource, &mut file)?;
    }

    Ok(())
}
