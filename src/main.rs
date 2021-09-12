use libc;
use std::env;
use std::process::exit;
use std::fs;
use std::os::unix::io::AsRawFd;
use std::io::Write;

macro_rules! die {
    ($($string:expr),+) => {
        $(
        print!("{} ",$string);
        )+
        print!("\n");
        exit(1)
    };
}

fn isatty(fd: i32) -> bool{
    let res;
    unsafe{
        res=libc::isatty(fd);
    }
    if res == 0 {
        false
    }else{
        true
    }
}
fn heuristic_tty() -> String{
    let ppid;
    let mut procstr = String::from("/proc/");
    unsafe{                
        ppid = libc::getppid();
    }
    if ppid == 1{ /* Don't try init */
        die!("No TTY connected to process!");
        /* Not reached */
    }
    procstr.push_str(&ppid.to_string());
    procstr.push_str("/fd/0");
    if let Ok(hndl) = fs::File::open(&procstr) {
        if isatty(hndl.as_raw_fd()){
                  procstr
        }else{
            die!("No TTY connected to process!");
            /* Not reached */
        }
        }else{
            die!("No TTY connected to process!");
            /* Not reached */
        }           
}

fn get_handle(file: &str) -> fs::File{
    let fd = if let Ok(handle)= fs::OpenOptions::new().append(true).open(file){
        handle
    }else{
        die!("Could'nt get file descriptor!");
        /* Not reached */
    };
    fd
}

fn resetty(tty: &str){
        let mut fd = get_handle(tty);
        if let Err(size) = fd.write(b"\x1Bc")/*.and_then(|_|{fd.write(b"\x0A")})*/{
            die!("Couldn't open tty:",size);
        }

}

fn main() -> Result<(),()> {
    let heur;
    let mut args = env::args();

    match args.len(){
        0..=1 =>{
            heur=true;
        },
        2 =>{
            heur=false;
        },
        _  => {
        die!("Usage\n",||-> String {if let Some(val) = env::args().nth(0){val}else{String::from("resetty")}}(),"[TTY]");
         /* Not reached */
        }
    }
    if heur {
    let tty = if let Ok(hndl) = fs::File::open("/proc/self/fd/0"){
        if isatty(hndl.as_raw_fd()){
                String::from("/proc/self/fd/0")
        }else{
            heuristic_tty()
        }
        }else{ 
            heuristic_tty()
       };
       resetty(&tty);

    }else{
        let tty = args.nth(1).unwrap();
        if let Ok(hndl) = fs::File::open(&tty){
            if isatty(hndl.as_raw_fd()){
                resetty(&tty);
            }else{
                die!("No TTY connected to process!");
                /* Not reached */
            }
            
        }else{
            die!("Invalid file descriptor!");
            /* Not reached */
        }
    }
    Ok(())
}
