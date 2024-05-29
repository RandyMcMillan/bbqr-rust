use bbqr::continuous_join::{ContinuousJoinResult, ContinuousJoiner};
use bbqr::{
    encode::Encoding,
    file_type::FileType,
    join::Joined,
    qr::Version,
    split::{Split, SplitOptions},
};
//use fast_qr::QRCode;
//use fast_qr::qr::QRCodeError;
use std::env;
fn do_it() -> () {
    print!("\ndo_it");
    let mut data: &[u8] = b"Hello, World!, but much larger";
    split_up(env::args());
}
pub fn split_up(mut args: env::Args) {
    print!("\nsplit_up");

    let args = env::args();
    let mut data: &[u8] = b"Hello, World!, but much larger";
    let mut getnext: &[u8] = b"false";
    if args.len() > 1 {
        print!("\nsplit_up:args.len()={}", args.len());
        //let _ = handle_command(env::args());
    } else {
        default();
    }
    if args.len() >= 2 {
        for arg in args {
            print!("\n{}", arg);
            if getnext == b"true" {
                print!("\ngetnext={:?}", getnext);
                //data = &data;
            }
            if arg == "--data" {
                getnext = b"true";
            } else {
            }
        }
    }

    // split the data using default options
    let split = Split::try_from_data(data, FileType::UnicodeText, Default::default())
        .expect("Failed to split data");

    // or split the data using zlib encoding, and custom options
    let split = Split::try_from_data(
        data,
        FileType::UnicodeText,
        SplitOptions {
            encoding: Encoding::Zlib,
            min_split_number: 2,
            max_split_number: 100,
            min_version: Version::V03,
            max_version: Version::V30,
        },
    )
    .expect("Failed to split data");

    // print out each of the parts
    println!("LINE:58:{:#?}", split.parts);

    // generate the qr codes
    let qr_codes = split.generate_qr_codes();
    Some(qr_codes);
    //Some(true);
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
        //
        "--data" => do_it(),
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
fn main() {
    let args = env::args();
    if args.len() >= 2 {
        //let _ = help();
        //let _ = handle_command(env::args());
    } else {
        //default();
    }

    let data: &[u8] = b"Hello, World!, but much larger";

    // split the data using default options
    let split = Split::try_from_data(data, FileType::UnicodeText, Default::default())
        .expect("Failed to split data");

    // print out each of the parts
    println!("{:}", split.parts[0].to_string());

    // generate the qr codes
    let qr_codes = split.generate_qr_codes();
    //print!("{:?}", qr_codes);
}
