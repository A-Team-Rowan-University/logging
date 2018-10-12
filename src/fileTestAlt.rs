 #[test]
fn log (s: &str, erlog: LogLevel){
    use std::fs;

    //println!("{} {}", erlog.to_string(), s); 

    fs::write( "errorLog.txt",erlog.to_string());
    fs::write( "errorLog.txt",s);
}