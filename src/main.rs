use std::fs::File;
use std::io::{Read, Write};

/// m4s_fix
///
/// Make m4s file to mp3 or mp4
/// You can costume your extension
const BUFFER_SIZE: usize = 1024;
fn main() {
    println!("[M4S_FIXER] your target file:");
    let mut target_file = String::new();
    std::io::stdin().read_line(&mut target_file)
        .expect("Failed to read line");

    println!("[M4S_FIXER] your output file extension:");
    let mut output_extension = String::new();
    std::io::stdin().read_line(&mut output_extension)
        .expect("Failed to read line");

    println!("[M4S_FIXER] Fixing...");
    let result = m4s_fixed(target_file, output_extension);
    if result {
        println!("[M4S_FIXER] Success!");
    } else {
        println!("[M4S_FIXER] Failed!");
    }
}
///
///
///
fn m4s_fixed(target_path: String, extension: String) -> bool {
    //get 'File' from target path
    let mut target_file = File::open(target_path).expect("failed to read the file");
    //set a header buffer array to store m4s file's header bytes
    let mut buffer:[ u8 ; 32 ] = [ 0 ; 32 ];
    //read 32 bytes from target file
    target_file.read(&mut buffer).unwrap();
    //shift array to String to replace error bytes
    let header = String::from_utf8(Vec::from(buffer)).unwrap();
    //replace '000000000' '$' 'avc1'
    let mut next_header = header.replace("000000000", "");
    next_header = next_header.replace("$", " ");
    next_header = next_header.replace("avc1", "");
    //create a file to output new bytes
    let mut output_file = File::create(String::from("output.") + &*extension).expect("create failed");
    //write header bytes
    output_file.write(next_header.as_bytes()).expect("write failed");
    //context bytes capacity: BUFFER_SIZE ( 1024 bytes )
    let mut context_buffers: [u8 ; BUFFER_SIZE] = [0 ; BUFFER_SIZE];
    while let Ok(length) = target_file.read(&mut context_buffers){
        if length == 0 {
            break;
        } else if length != BUFFER_SIZE {
            // !!!!!!!!! it is important for the program here
            // we need to create a new array to restore last bytes
            // if you use context_buffers, it will cause file's total be more
            let last_buffers = &context_buffers[0..length];
            output_file.write(&last_buffers).expect("TODO: panic message");
            break;
        }
        output_file.write(&context_buffers).expect("TODO: panic message");
    }

    return true;
}
