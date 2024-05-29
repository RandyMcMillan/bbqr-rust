use sha2::{Digest, Sha256};
use std::env;
use std::str;

fn sha256_string(data: &str) -> Result<String, String> {
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    let result = hasher.finalize();

    // Convert the hash result (byte array) to a hex string
    let hex_result = hex::encode(result);

    Ok(hex_result.to_string())
}

pub fn handle_command(mut args: env::Args) -> Result<bool, Box<dyn std::error::Error>> {
    print!("\nhandle_command");
    let _ = args.next(); // program name
    let command = args.next().unwrap(); // must be there or we would not have been called

    #[cfg(debug_assertions)]
    println!("\n*** gnostr-gui is running in command mode ***");
    #[cfg(debug_assertions)]
    println!("*** COMMAND = {} ***\n", command);

    match &*command {
        //version
        "-V" => version(),
        //support help2man
        "-v" => version(),
        "--version" => version(),
        "version" => version(),
        //help
        "-h" => help(),
        "--help" => help(),
        "help" => help(),
        //other
        other => println!("Unknown command {}", other),
    }
    //do_it();
    Ok(true)
}
fn help() {
    print!("\nhelp");
    use std::process;

    let package_name = env!("CARGO_PKG_NAME");
    let crate_name = env!("CARGO_CRATE_NAME");
    let version = env!("CARGO_PKG_VERSION");
    print!(
        "\n1:{} v{}\n\n",
        package_name.replace("jj-cli", "gnostr"),
        version
    );
    print!("\n{} v{}\n\n", crate_name.replace("git_", ""), version);
    print!("3:{} get\n", crate_name.replace("git_", ""));
    print!("4:       <csv_relay_list>\n");
    print!("5:{} json\n", crate_name.replace("git_", ""));
    print!("6:       <json_relay_list>\n");
    print!("7:{} stripped\n", crate_name.replace("git_", "-"));
    print!("8:       <string_relay_list> <int_length_last>\n");
    process::exit(0);
}
fn version() {
    use std::process;
    let version = env!("CARGO_PKG_VERSION");
    let crate_name = env!("CARGO_CRATE_NAME");
    println!(
        "{} v{}",
        crate_name.replace("git_gnostr", "gnostr"),
        version
    );
    process::exit(0);
}
fn default() {
    print!("\ndefault");
    help();
}
use image::Luma;
use qrcode::QrCode;

fn render(data: &str) {
    let code = QrCode::new(&data).unwrap();
    let hash = sha256_string(&data).unwrap();
    let image = code.render::<Luma<u8>>().build();
    let location = format!("{}.png", hash);
    image.save(location).unwrap();
    let string = code
        .render::<char>()
        .quiet_zone(false)
        .module_dimensions(2, 1)
        .build();
    println!("{}", string);
}
fn main() {
    use std::env;
    let args = env::args();
    let mut getnext: [u8; 5] = [0; 5];
    if args.len() >= 2 {
        for arg in args {
            if getnext[0] > 0 {
                //print!("\narg={}", arg);
                //print!("\ngetnext={:?}", getnext);
                render(&arg);
            }
            if arg == "--data" {
                getnext[0] = 1;
            } else {
            }
        }
    }
}
