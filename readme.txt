> cargo install cargo-watch
> cargo watch -w src -x run

Json:
http://technosophos.com/2018/06/12/from-go-to-rust-json-and-yaml.html

#[serde(rename_all = "camelCase")]  // struct level

#[serde(skip_serializing_if="Option::is_none")] // attr level
#[serde(skip_serializing_if="Vec::is_empty")]   // arrt level

// reading from a file
    use std::fs::File;
    let f = File::open("person.json").unwrap();
    let person: Person = serde_json::from_reader(f).unwrap();
