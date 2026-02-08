use std::{env, path::PathBuf, process};
use mem_rs::prelude::*;


fn main() {
    let exe_path = env::current_exe().unwrap();
    let dll_path = PathBuf::from(exe_path).parent().unwrap().join("er_havok_aabb_debug.dll");

    if !dll_path.exists() {
        println!("ERROR: Can't find DLL");
        process::exit(0);
    }

    let mut process: Process = Process::new("eldenring.exe");
    process.refresh().expect("Failed to attach to process.");

    if let Some(_module) = get_module(&mut process, "er_havok_aabb_debug.dll") {
        println!("ERROR: DLL already injected");
        process::exit(0);
    }

    process.inject_dll(dll_path.into_os_string().to_str().unwrap()).expect("ERROR: Failed to inject DLL");
    get_module(&mut process, "er_havok_aabb_debug.dll").expect("ERROR: Failed to inject DLL");
}

pub fn get_module(process: &mut Process, module_name: &str) -> Option<ProcessModule> {
    return process.get_modules().iter().find(|m| m.name == module_name).cloned();
}