use std::{fs, io::Write, path, process, sync::Mutex};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use rayon::iter::{ParallelIterator, IntoParallelIterator};

const TEST: &str = "#include <cstdint>\nauto main() -> int {}";

lazy_static::lazy_static! {
    static ref STDOUT: Mutex<StandardStream> = Mutex::new(StandardStream::stdout(ColorChoice::Always));
}

#[cfg(target_os = "windows")]
const LIBS: &[&str] = &[
    "ws2_32", "comctl32", "gdi32", "oleaut32", "ole32", "uuid", "shell32", "advapi32", "comdlg32",
    "winspool", "user32", "kernel32", "odbc32", "gdiplus", "opengl32", "glu32",
];

#[cfg(target_os = "macos")]
const LIBS: &[&str] = &["Carbon", "Cocoa", "ApplicationServices", "OpenGL"];

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
const LIBS: &[&str] = &[
    "pthread",
    "X11",
    "Xext",
    "Xinerama",
    "Xcursor",
    "Xrender",
    "Xfixes",
    "Xft",
    "fontconfig",
    "pango-1.0",
    "pangoxft-1.0",
    "gobject-2.0",
    "cairo",
    "pangocairo-1.0",
    "GL",
    "GLU",
];

fn good(txt: &str) {
    let mut stdout = STDOUT.lock().unwrap();
    stdout
        .set_color(ColorSpec::new().set_fg(Some(Color::Green)))
        .unwrap();
    writeln!(&mut stdout, "{}", txt).unwrap();
    stdout.reset().unwrap();
}

fn warn(txt: &str) {
    let mut stdout = STDOUT.lock().unwrap();
    stdout
        .set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))
        .unwrap();
    writeln!(&mut stdout, "{}", txt).unwrap();
    stdout.reset().unwrap();
}

fn bad(txt: &str) {
    let mut stdout = STDOUT.lock().unwrap();
    stdout
        .set_color(ColorSpec::new().set_fg(Some(Color::Red)))
        .unwrap();
    writeln!(&mut stdout, "{}", txt).unwrap();
    stdout.reset().unwrap();
}

fn main() {
    let cxx: &str = if uname() == "msvc" { "cl" } else { "c++" };

    println!("Checking whether this env can build fltk-rs..");
    if cfg!(target_os = "windows") {
        if cxx == "c++" {
            println!("This is testing a posix environment on Windows");
        } else {
            println!("This is testing an MSVC environment on Windows");
        }
    }

    let (maj, min, host) = rustc_version("rustc");
    if maj == 1 && min > 45 {
        good(&format!("Found suitable Rust version for host {}!", host));
    } else {
        bad("You need Rust version 1.46 or higher!");
    }

    if let Ok(_) = process::Command::new("git").arg("--version").output() {
        good("Found working git executable!");
    } else {
        bad("git is not installed or not in PATH");
    }

    if let Ok(_) = process::Command::new("cmake").arg("--version").output() {
        good("Found working CMake executable!");
    } else {
        bad("cmake is not installed or not in PATH");
    }

    if let Ok(_) = process::Command::new("ninja").arg("--version").output() {
        good("Found working Ninja executable!");
    } else {
        warn("Ninja is not installed or not in PATH");
    }

    let version = if cxx == "cl" { "" } else { "--version" };

    if let Ok(_) = process::Command::new(cxx).arg(version).output() {
        good("Found a C++ compiler!");
    } else {
        bad("A C++ compiler wasn't found");
    }

    let file = "fltk_check_file.cpp";
    if path::Path::new(file).exists() {
        fs::remove_file(file).unwrap();
    }

    fs::File::create(file).unwrap();
    fs::write(file, TEST).unwrap();

    if let Ok(_) = process::Command::new(cxx).arg(file).output() {
        good("Found C++ compiler supporting C++11!");
    } else {
        bad("C++ compiler doesn't support C++11!");
        if path::Path::new(file).exists() {
            fs::remove_file(file).unwrap();
        }
        return;
    }

    LIBS.into_par_iter().for_each(|lib| {
        let spec = if cxx == "cl" {
            ""
        } else {
            if cfg!(target_os = "macos") {
                "-framework"
            } else {
                "-l"
            }
        };
        let lib_name = if cxx == "cl" {
            format!("{}.lib", lib)
        } else {
            lib.to_string()
        };
        let std = if cxx == "cl" {
            ""
        } else {
            "-std=c++11"
        };
        if let Ok(c) = process::Command::new(cxx).arg(std).arg(file).arg(&spec).arg(&lib_name).output() {
            if cxx != "cl" {
                if c.stderr.is_empty() {
                    good(&format!("Found library: {}!", lib));
                } else {
                    bad(&format!("Library {} was not found!", lib));
                }
            } else {
                if c.stdout.is_empty() {
                    bad(&format!("Library {} was not found!", lib));
                } else {
                    let out = String::from_utf8_lossy(&c.stdout).to_string();
                    if out.contains("LINK : fatal error") {
                        bad(&format!("Library {} was not found!", lib));
                    } else {
                        good(&format!("Found library: {}!", lib));
                    }
                }
            }
        } else {
            bad(&format!("Library {} was not found!", lib));
        }
    });
    if path::Path::new(file).exists() {
        fs::remove_file(file).unwrap();
    }
    if path::Path::new("a.out").exists() {
        fs::remove_file("a.out").unwrap();
    }
    if path::Path::new("a.exe").exists() {
        fs::remove_file("a.exe").unwrap();
    }
    if path::Path::new("fltk_check_file.exe").exists() {
        fs::remove_file("fltk_check_file.exe").unwrap();
    }
    if path::Path::new("fltk_check_file.obj").exists() {
        fs::remove_file("fltk_check_file.obj").unwrap();
    }
}

fn rustc_version(executable: &str) -> (u8, u8, String) {
    let cmd = process::Command::new(executable)
        .args(&["--version", "-v"])
        .output()
        .unwrap();
    let version = String::from_utf8_lossy(&cmd.stdout);
    let version: Vec<&str> = version.split_whitespace().collect();
    let host = version[11];
    let version: Vec<&str> = version[1].split('.').collect();
    (version[0].parse().unwrap(), version[1].parse().unwrap(), host.to_owned())
}

fn uname() -> &'static str {
    if let Ok(_) = process::Command::new("uname").arg("-a").output() {
        "gnu"
    } else {
        "msvc"
    }
}
