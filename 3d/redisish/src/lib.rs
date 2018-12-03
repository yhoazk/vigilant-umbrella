#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
#[derive(Debug, PartialEq)]
pub enum Command {
    Publish(String),
    Retrieve
}

#[derive(Debug, PartialEq)]
pub enum Error {
    Err1,
    Err2
}

pub fn parse(input: &str) -> Result<Command, Error>{
    let publish = "PUBLISH";
    let retrieve = "RETRIEVE";
    if input.starts_with(publish) && input.ends_with('\n') {
        Ok(Command::Publish(input.trim_start_matches(publish).to_string())) 
    } else if input == retrieve {
        Ok(Command::Retrieve)
    } else {
        Err(Error::Err1)
    }
}

#[test]
fn test_pub_starts_with(){
    let ver = parse("PUBLISH ssdf\n");
    match ver{
        Ok(ver) => { println!("XXXXXXXXXXXXXx {:?}", ver); assert_eq!(ver,Command::Publish(" ssdf\n".to_string()))},
        Err(err) => { println!("YYYYY {:?}", err) },
    }
}

#[test]
fn test_retrieve(){
    let ver = parse("RETRIEVE\n");
    match ver{
        Ok(ver) => { println!("XXXXXXXXXXXXXx {:?}", ver); assert_eq!(1,4)},
        Err(err) => { println!("ERROR {:?}", err) },
    }
}

#[test]
fn test_error(){
    let ver = parse("sdaasda");
    match ver{
        Ok(ver) => { println!("XXXXXXXXXXXXXx {:?}", ver); assert_eq!(1,4)},
        Err(err) => { println!("ERROR {:?}", err); assert_eq!(err, Error::Err1) },
    }
}


#[test]
fn test_empty(){
    let ver = parse("");
    match ver{
        Ok(ver) => { println!("XXXXXXXXXXXXXx {:?}", ver); assert_eq!(1,4)},
        Err(err) => { println!("ERROR {:?}", err); assert_eq!(err, Error::Err1) },
    }
}