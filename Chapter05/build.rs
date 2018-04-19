use std::process::Command;
use std::env;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    Command::new("gcc").args(&["src/motor1.c", "-c", "-fPIC", "-o"])
                       .arg(&format!("{}/motor1.o", out_dir))
                       .status().unwrap();
    Command::new("ar").args(&["crus", "libmotor1.a", "motor1.o"])
                      .current_dir(&Path::new(&out_dir))
                      .status().unwrap();
    Command::new("gcc").args(&["src/motor2.c", "-c", "-fPIC", "-o"])
                       .arg(&format!("{}/motor2.o", out_dir))
                       .status().unwrap();
    Command::new("ar").args(&["crus", "libmotor2.a", "motor2.o"])
                      .current_dir(&Path::new(&out_dir))
                      .status().unwrap();
    Command::new("gcc").args(&["src/motor3.c", "-c", "-fPIC", "-o"])
                       .arg(&format!("{}/motor3.o", out_dir))
                       .status().unwrap();
    Command::new("ar").args(&["crus", "libmotor3.a", "motor3.o"])
                      .current_dir(&Path::new(&out_dir))
                      .status().unwrap();

    Command::new("gcc").args(&["src/elevator1.c", "-c", "-fPIC", "-o"])
                       .arg(&format!("{}/elevator1.o", out_dir))
                       .status().unwrap();
    Command::new("ar").args(&["crus", "libelevator1.a", "elevator1.o"])
                      .current_dir(&Path::new(&out_dir))
                      .status().unwrap();
    Command::new("gcc").args(&["src/elevator2.c", "-c", "-fPIC", "-o"])
                       .arg(&format!("{}/elevator2.o", out_dir))
                       .status().unwrap();
    Command::new("ar").args(&["crus", "libelevator2.a", "elevator2.o"])
                      .current_dir(&Path::new(&out_dir))
                      .status().unwrap();
    Command::new("gcc").args(&["src/elevator3.c", "-c", "-fPIC", "-o"])
                       .arg(&format!("{}/elevator3.o", out_dir))
                       .status().unwrap();
    Command::new("ar").args(&["crus", "libelevator3.a", "elevator3.o"])
                      .current_dir(&Path::new(&out_dir))
                      .status().unwrap();

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=motor1");
    println!("cargo:rustc-link-lib=static=motor2");
    println!("cargo:rustc-link-lib=static=motor3");
    println!("cargo:rustc-link-lib=static=elevator1");
    println!("cargo:rustc-link-lib=static=elevator2");
    println!("cargo:rustc-link-lib=static=elevator3");
}
