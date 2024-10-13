/// m4s_fix
///
/// Make m4s file to mp3 or mp4
/// You can costume your extension
mod fixer;
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
    let result = fixer::m4s_fixed(target_file, output_extension);
    if result {
        println!("[M4S_FIXER] Success!");
    } else {
        println!("[M4S_FIXER] Failed!");
    }
}


