use std::env;
use std::path;
use std::process;

fn main() {
    println!("cargo:rerun-if-changed=src/CMakeLists.txt");
    println!("cargo:rerun-if-changed=src/main.cpp");

    let out_dir = path::PathBuf::from(env::var("OUT_DIR").unwrap());
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let rustc = env::var("RUSTC").unwrap();

    let (maj, min) = rustc_version(&rustc);
    assert!(maj == 1 && min > 45);

    process::Command::new("git")
        .arg("--version")
        .spawn()
        .expect("git is not installed or not in PATH");
    process::Command::new("cmake")
        .arg("--version")
        .spawn()
        .expect("cmake is not installed or not in PATH");

    cc::Build::new()
        .file("src/main.cpp")
        .cpp(true)
        .define("main", "libmain")
        .compile("check");

    cmake::Config::new("src").build();

    process::Command::new(out_dir.join("check"))
        .spawn()
        .expect("Couldn't run cmake artifact!");

    match target_os.as_str() {
        "macos" => {
            println!("cargo:rustc-link-lib=framework=Carbon");
            println!("cargo:rustc-link-lib=framework=Cocoa");
            println!("cargo:rustc-link-lib=framework=ApplicationServices");
            println!("cargo:rustc-link-lib=framework=OpenGL");
        }
        "windows" => {
            println!("cargo:rustc-link-lib=dylib=ws2_32");
            println!("cargo:rustc-link-lib=dylib=comctl32");
            println!("cargo:rustc-link-lib=dylib=gdi32");
            println!("cargo:rustc-link-lib=dylib=oleaut32");
            println!("cargo:rustc-link-lib=dylib=ole32");
            println!("cargo:rustc-link-lib=dylib=uuid");
            println!("cargo:rustc-link-lib=dylib=shell32");
            println!("cargo:rustc-link-lib=dylib=advapi32");
            println!("cargo:rustc-link-lib=dylib=comdlg32");
            println!("cargo:rustc-link-lib=dylib=winspool");
            println!("cargo:rustc-link-lib=dylib=user32");
            println!("cargo:rustc-link-lib=dylib=kernel32");
            println!("cargo:rustc-link-lib=dylib=odbc32");
            println!("cargo:rustc-link-lib=dylib=gdiplus");
            println!("cargo:rustc-link-lib=dylib=opengl32");
            println!("cargo:rustc-link-lib=dylib=glu32");
        }
        _ => {
            println!("cargo:rustc-link-lib=dylib=pthread");
            println!("cargo:rustc-link-lib=dylib=X11");
            println!("cargo:rustc-link-lib=dylib=Xext");
            println!("cargo:rustc-link-lib=dylib=Xinerama");
            println!("cargo:rustc-link-lib=dylib=Xcursor");
            println!("cargo:rustc-link-lib=dylib=Xrender");
            println!("cargo:rustc-link-lib=dylib=Xfixes");
            println!("cargo:rustc-link-lib=dylib=Xft");
            println!("cargo:rustc-link-lib=dylib=fontconfig");
            println!("cargo:rustc-link-lib=dylib=pango-1.0");
            println!("cargo:rustc-link-lib=dylib=pangoxft-1.0");
            println!("cargo:rustc-link-lib=dylib=gobject-2.0");
            println!("cargo:rustc-link-lib=dylib=cairo");
            println!("cargo:rustc-link-lib=dylib=pangocairo-1.0");
            println!("cargo:rustc-link-lib=dylib=GL");
            println!("cargo:rustc-link-lib=dylib=GLU");
        }
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
