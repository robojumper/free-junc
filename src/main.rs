use pico_args::{Arguments, Error as PicoError};
use std::ffi::{OsStr, OsString};
use walkdir::WalkDir;
use winapi::shared::winerror::ERROR_NOT_A_REPARSE_POINT;

fn report_junction(path: &OsStr, dest: &OsStr) {
    println!(
        "JUNCTION: {} -> {}",
        path.to_string_lossy(),
        dest.to_string_lossy()
    )
}

fn display_junction_info(path: &OsStr, quiet: bool, recursive: bool) -> Result<(), String> {
    fn resolve(path: &OsStr, quiet: bool) -> Option<OsString> {
        match junction::exists(path) {
            Ok(true) => match junction::get_target(path) {
                Ok(dest) => Some(dest.into_os_string()),
                Err(e) => {
                    println!(
                        "cannot access `{}`: {}",
                        path.to_string_lossy(),
                        e.to_string()
                    );
                    None
                }
            },
            Ok(false) => None,
            Err(e)
                if e.raw_os_error() == Some(ERROR_NOT_A_REPARSE_POINT as _)
                    || e.raw_os_error() == Some(32) =>
            {
                None
            }
            Err(e) => {
                if !quiet {
                    println!(
                        "Could not access path `{}` to check if junction: {}",
                        path.to_string_lossy(),
                        e.to_string()
                    );
                }
                None
            }
        }
    }
    if recursive {
        // follow_links(true) would be great but in practice, this makes it not yield a
        // recursive junction at all even if we would like to actually report it
        for entry in WalkDir::new(path).follow_links(false) {
            match entry {
                Ok(dir) => {
                    if let Some(p) = resolve(dir.path().as_os_str(), quiet) {
                        report_junction(dir.path().as_os_str(), &p);
                    }
                }
                Err(e) => {
                    if !quiet {
                        println!(
                            "Could not access path `{}` to check if junction: {}",
                            path.to_string_lossy(),
                            e.to_string()
                        );
                    }
                }
            }
        }
        Ok(())
    } else if let Some(p) = resolve(path, true) {
        report_junction(path, &p);
        Ok(())
    } else {
        Err(format!(
            "path `{}` is not a junction",
            path.to_string_lossy(),
        ))
    }
}

fn delete_junction(path: &OsStr) -> Result<(), String> {
    junction::delete(path).map_err(|ioe| {
        format!(
            "failed to remove junction `{}`: {}",
            path.to_string_lossy(),
            ioe.to_string()
        )
    })?;
    std::fs::remove_dir(path).map_err(|ioe| {
        format!(
            "failed to delete empty directory `{}` after removing junction: {}",
            path.to_string_lossy(),
            ioe.to_string()
        )
    })
}

fn create_junction(src: &OsStr, dest: &OsStr) -> Result<(), String> {
    junction::create(dest, src).map_err(|ioe| {
        format!(
            "failed to create junction `{}`: {}",
            src.to_string_lossy(),
            ioe.to_string()
        )
    })
}

fn main() -> Result<(), String> {
    let mut args = Arguments::from_env();

    // silently drop original junction args
    let _ = args.contains("-nobanner");
    let _ = args.contains("-accepteula");

    if args.contains("-d") {
        let paths = get_free_os_strs(&mut args);
        return if paths.len() != 1 {
            Err(format!(
                "expected exactly 1 path to junction to delete, got {}",
                paths.len()
            ))
        } else {
            delete_junction(&paths[0])
        };
    }

    let quiet = args.contains("-q");
    let recursive = args.contains("-s");

    let paths = get_free_os_strs(&mut args);

    match paths.len() {
        0 => {
            display_usage();
            Ok(())
        }

        1 => display_junction_info(&paths[0], quiet, recursive),
        2 => {
            if quiet || recursive {
                Err("q and s are for a different subcommand".to_owned())
            } else {
                create_junction(&paths[0], &paths[1])
            }
        }
        x => {
            if quiet || recursive {
                Err(format!(
                    "expected exactly 1 path to display junction information, got {}",
                    x
                ))
            } else {
                Err(format!(
                    "expected exactly 2 paths to create junction, got {}",
                    x
                ))
            }
        }
    }
}

fn display_usage() {
    let program = std::env::args().next().unwrap();
    println!(
        "free-junc v1.0: Create, delete and list NTFS junctions.
Usage: {invoc} [-s] [-q] <directory>
    List junction, if it exists, at the given directory.
      -s: Recursive. Print all junctions at and below the given directory.
      -q: Quiet. Do not report filesystem access errors.

Usage: {invoc} <junction directory> <target directory>
    Create a junction from <junction directory> to <target directory>.

Usage: {invoc} -d <junction directory>
    Remove the junction at <junction directory> and remove the resulting empty directory.
",
        invoc = program
    );
}

fn get_free_os_strs(args: &mut Arguments) -> Vec<OsString> {
    let mut paths = vec![];

    loop {
        let rep = args.free_from_os_str::<_, std::convert::Infallible>(|x| Ok(x.to_owned()));
        match rep {
            Ok(st) => paths.push(st),
            Err(PicoError::MissingArgument) => break,
            Err(p) => unreachable!("we take raw OsStrings, but got {:?}", p),
        }
    }
    paths
}
