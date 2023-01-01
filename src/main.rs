use std::{env,  error::Error, fs::File, io::{BufRead, BufReader}};
use sha1::Digest;

const SHA_HEX_STRING_LENGTH: usize = 40;

fn main() -> Result<(), Box<dyn Error>> {
 
 let arg: Vec<String> = env::args().collect();

 if arg.len() != 3 {
    println!("Usage");

   println!("sha1_cracker: <wordlist.txt> <sha1_hash>");
    return  Ok(());
 }

let hash_to_crack = arg[2].trim();

if hash_to_crack.len() != SHA_HEX_STRING_LENGTH {
    return Err("sha1 hash is not valid".into());
}

let worklist_file = File::open(&arg[1])?;
let reader = BufReader::new(&worklist_file);

for line in reader.lines() { 

    let line = line?;
let common_password = line.trim();

if hash_to_crack == &hex::encode(sha1::Sha1::digest(common_password.as_bytes())) {
    println!("{}", &common_password);
return Ok(())
}
}


println!("password not found in wordlist :(");

Ok(())

}



