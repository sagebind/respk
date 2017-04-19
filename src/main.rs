extern crate getopts;
extern crate respk;
extern crate walkdir;

use respk::Package;
use std::env;
use std::fs::{self, File};
use std::io::Read;
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

    options.optflag("", "ignore-errors", "Keep going when encountering I/O errors");
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

    match command.as_ref() {
        "add" => add(package, paths),
        "delete" => delete(package, paths),
        "list" => list(package),
        "extract" => extract(package, paths),
        _ => {
            printerr!("Unknown command: '{}'", command);
            exit(1);
        },
    }
}

fn add(package: Package, paths: &[String]) {
    for path in paths {
        let metadata = fs::metadata(path).unwrap();

        if metadata.is_dir() {
            for entry in WalkDir::new(path) {}
        } else {
            println!("{}", path);

            let file = File::open(path).unwrap();
            package.add(path, file).unwrap();
        }
    }
}

fn delete(package: Package, paths: &[String]) {}

fn list(package: Package) {
    for resource in package.resources().unwrap() {
        println!("{}    {} bytes", resource.path(), resource.size());
    }
}

fn extract(package: Package, paths: &[String]) {}