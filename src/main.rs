use std::io;
use serde_json::{Value};
use std::string::{String};
use std::process::{Command};

fn main() {
    let mut recipe_info = String::new();

    io::stdin()
        .read_line(&mut recipe_info)
        .expect("Failed to read line");

    let v: Value = serde_json::from_str(&recipe_info)
                            .expect("Failed to deserialize recipe info!");

    
    for dependencies in v.as_array().unwrap() {
        println!("{}", dependencies["reference"]);
        println!("    Binary: {}", dependencies["binary"]);
        println!("    Recipe: {}", dependencies["recipe"]);
        println!("    export_folder: {}", dependencies["export_folder"]);
        println!("    package_folder: {}", dependencies["package_folder"]);


        let export_folder = String::from(dependencies["export_folder"].as_str().unwrap());
        let package_folder = String::from(dependencies["package_folder"].as_str().unwrap());
        let ref_vec = dependencies["reference"].as_str()
                                               .expect("reference node is not a string!")
                                               .split(&['/', '@'][..])
                                               .collect::<Vec<&str>>();


        let mut dst_folder = String::from("./sdk");
        if ref_vec.len() >= 3 {
            let name = ref_vec[0];
            let user = ref_vec[2];

            dst_folder = format!("{}/{}/{}/", dst_folder, user, name);
        } else if ref_vec.len() >=1 {
            let name = ref_vec[0];

            dst_folder = format!("{}/{}/", dst_folder, name);
        } else {
            println!("Bad reference {}", dependencies["reference"]);
            continue;
        }

        Command::new("sh").arg("-c")
                          .arg(format!("mkdir -p {}", dst_folder))
                          .output()
                          .expect("Failed to create sdk directory!");

        Command::new("sh").arg("-c")
                          .arg(format!("cp -rf {}/* {}", package_folder, dst_folder))
                          .output()
                          .expect("Failed to export package!");

        Command::new("sh").arg("-c")
                          .arg(format!("cp -f {}/conanfile.py {}", export_folder, dst_folder))
                          .output()
                          .expect("Failed to export conan recipe!");
    }

}
