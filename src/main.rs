use std::{fs, io::Write, path, process};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

const TEST: &str = "#include <cstdint>\nauto main() -> int {}";

#[cfg(all(target_os = "windows", target_family = "msvc"))]
const CXX: &str = "cl";

#[cfg(not(all(target_os = "windows", target_family = "msvc")))]
const CXX: &str = "c++";

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
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout
        .set_color(ColorSpec::new().set_fg(Some(Color::Green)))
        .unwrap();
    writeln!(&mut stdout, "{}", txt).unwrap();
}

fn bad(txt: &str) {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout
        .set_color(ColorSpec::new().set_fg(Some(Color::Red)))
        .unwrap();
    writeln!(&mut stdout, "{}", txt).unwrap();
}

fn main() {
    println!("Checking whether this env can build fltk-rs...");

    let (maj, min) = rustc_version("rustc");
    if maj == 1 && min > 45 {
        good("Found suitable Rust version!");
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
        bad("Ninja is not installed or not in PATH");
    }

    if let Ok(_) = process::Command::new(CXX).arg("--version").output() {
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

    if let Ok(_) = process::Command::new(CXX).arg(file).output() {
        good("Found C++ compiler supporting C++11!");
    } else {
        bad("C++ compiler doesn't support C++11!");
        if path::Path::new(file).exists() {
            fs::remove_file(file).unwrap();
        }
        return;
    }

    for lib in LIBS {
        let lib_arg = if CXX == "cl" {
            format!("{}.lib", lib)
        } else {
            #[cfg(not(target_os = "macos"))]
            {
                format!("-l{}", lib)
            }

            #[cfg(target_os = "macos")]
            {
                format!("-framework {}", lib)
            }
        };
        if let Ok(c) = process::Command::new(CXX).arg(file).arg(&lib_arg).output() {
            if c.stderr.is_empty() {
                good(&format!("Found library: {}!", lib));
            } else {
                bad(&format!("Library {} was not found!", lib));
            }
        } else {
            bad(&format!("Library {} was not found!", lib));
        }
    }
    if path::Path::new(file).exists() {
        fs::remove_file(file).unwrap();
    }
    if path::Path::new("a.out").exists() {
        fs::remove_file("a.out").unwrap();
    }
    if path::Path::new("fltk_check_file.exe").exists() {
        fs::remove_file("fltk_check_file.exe").unwrap();
    }
}

fn rustc_version(executable: &str) -> (u8, u8) {
    let cmd = process::Command::new(executable)
        .arg("--version")
        .output()
        .unwrap();
    let version = String::from_utf8_lossy(&cmd.stdout);
    let version: Vec<&str> = version.split_whitespace().collect();
    let version: Vec<&str> = version[1].split('.').collect();
    (version[0].parse().unwrap(), version[1].parse().unwrap())
}
