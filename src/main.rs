use std::{env, fs, io, path::PathBuf};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let path = if args.len() > 1 {
        PathBuf::from(&args[1])
    } else {
        env::current_dir().unwrap()
    };

    fs::create_dir_all(path.join("main"))?;
    fs::create_dir_all(path.join(".vscode"))?;

    let vs_code_settings = r#"{
    "clangd.arguments": [
        "--compile-commands-dir=build",
        "--query-driver=/**/*xtensa*"
    ]
}
"#;

    let root_cmakelists = r#"cmake_minimum_required(VERSION 3.5)
include($ENV{IDF_PATH}/tools/cmake/project.cmake)
project(my_project)
"#;
    let main_cmakelists = r#"idf_component_register(SRCS "main.c"
                       INCLUDE_DIRS ".")
"#;

    let main_c = r#"#include "freertos/FreeRTOS.h"
#include "freertos/task.h"
#include "driver/gpio.h"

void app_main(void) {
    
} 
    "#;

    let clangd = r#"CompileFlags:
  Add:
    - "-D__XTENSA__=1"
  Remove:
    - "-mlongcalls"
    - "-fstrict-volatile-bitfields""#;

    fs::write(path.join(".vscode/settings.json"), vs_code_settings)?;
    fs::write(path.join("main/CMakeLists.txt"), main_cmakelists)?;
    fs::write(path.join("main/main.c"), main_c)?;
    fs::write(path.join("CMakeLists.txt"), root_cmakelists)?;
    fs::write(path.join(".clangd"), clangd)?;

    let project_path = fs::canonicalize(path)?;

    println!("Success.\nType 'cd {}' to get into your new project.", project_path.display());

    Ok(())
}
