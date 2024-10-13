use std::fs::File;
use std::io::{Read, Write};

const CONTEXT_BUFFER_SIZE: usize = 1024;
const HEADER_BUFFER_SIZE: usize = 32;

#[derive(Debug)]
pub struct M4SFile {
    file_path: String,
    output_name: String,
    file_extension: String
}

pub fn m4s_fixed_array(files: &[M4SFile]) -> bool {
    for &path in files {
        m4s_fixed(path.file_path, path.file_extension);
    }
    return true;
}
pub fn m4s_fixed(target_path: String, extension: String) -> bool {
    //get 'File' from target path
    let mut target_file = File::open(target_path).expect("failed to read the file");
    //set a header buffer array to store m4s file's header bytes
    let mut buffer:[ u8 ; HEADER_BUFFER_SIZE ] = [ 0 ; HEADER_BUFFER_SIZE ];
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
    let mut context_buffers: [u8 ; CONTEXT_BUFFER_SIZE] = [0 ; CONTEXT_BUFFER_SIZE];
    while let Ok(length) = target_file.read(&mut context_buffers){
        if length == 0 {
            break;
        } else if length != CONTEXT_BUFFER_SIZE {
            // !!!!!!!!! it is important for the program here
            // we need to create a new array to restore last bytes
            // if you use context_buffers, it will cause file's total be more
            let last_buffers = &context_buffers[0..length];
            output_file.write(&last_buffers).expect("write bytes failed");
            break;
        }
        output_file.write(&context_buffers).expect("write bytes failed");
    }

    return true;
}



