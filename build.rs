use std::env;
use std::process::exit;

fn main(){
    let mut isdebug = false;
    if let Ok(key) = env::var("DEBUG"){
        if key.contains("true"){
        isdebug = true;
        }
    }else{
        println!("Could'nt parse environment variables!\
        Exiting!");
        exit(1);
    }
    if isdebug {
        println!("cargo:rustc-cfg=isdebug");
    }
}