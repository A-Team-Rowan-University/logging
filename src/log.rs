use log_level::LogLevel;
use std::io::Write;
use std::fs::OpenOptions;


pub fn log (s: &str, erlog: LogLevel){

    //println!("{} {}", erlog.to_string(), s); 

    //fs::write( "errorLog.txt",erlog.to_string());
    //fs::write( "errorLog.txt",s);

    let input = format!("{} {}",s,erlog.to_string());

    let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open("Nickslist").unwrap();

        file.write(&input.into_bytes());
}