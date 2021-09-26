use std::env;
use std::process::exit;

fn main(){
    let isdebug;
    if let Ok(_) = env::var("DEBUG"){
        isdebug = true;
    }else{
        println!("Could'nt parse environment variables!\
        Exiting!");
        exit(1);
    }
    if isdebug {
        println!("cargo:rustc-cfg=isdebug");
    }
}