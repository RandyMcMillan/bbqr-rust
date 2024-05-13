# bbqr-rust

Implementaion of the bbqr spec in rust:

https://github.com/coinkite/BBQr/blob/master/BBQr.md

## Usage

### Split large data up

```rust
use bbqr::{
   encode::Encoding,
   file_type::FileType,
   join::Joined,
   qr::Version,
   split::{Split, SplitOptions},
};

let data: &[u8] = b"Hello, World!, but much larger";

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
).expect("Failed to split data");

// print out each of the parts
println!("{:#?}", split.parts);

// generate the qr codes
let qr_codes = split.generate_qr_codes();
```

### Join split QR codes

```rust
// get the parts from somewhere
let parts: Vec<String> = "...";

// join the parts
let joined = Joined::try_from_parts(parts).expect("Failed to join");

/// joined.data has the raw bytes
match &joined.encoding {
  Encoding::Unicode => String::from_utf8(joined.data),
  other => {
    // do whatever
  }
}
```

### Join QR codes one by one

```rust
use bbqr::continuous_join::{ContinuousJoinResult, ContinuousJoiner};

// create new continuous joiner
let mut continuous_join = ContinuousJoiner::new();

// part from QR code scanning;
let part = "... placeholder ...";

let join_result = continuous_join.add_part(part).expect("Failed to add part");

match join_result {
    ContinuousJoinResult::NotStarted => println!("Not started, part was empty"),

    ContinuousJoinResult::InProgress { parts_left } => {
        // print out the number of parts left
        println!("Parts left: {}", parts_left);
    }

    // if the part is the last part, then the data is ready
    ContinuousJoinResult::Complete(joined) => {
        // the data is ready, do something with it
    }
}
```
