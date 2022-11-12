use gjson;

fn parse(search: &str) -> String {
    let json = std::fs::read_to_string("codes.json").unwrap();
    let binding = gjson::get(&json, &search);

    if !binding.exists() {
        panic!("Status code or message does not exist!");
    }

    let value = binding.to_string();

    return value;
}

pub fn code(code: &str) -> String {
    let search = format!("#(message=={}).code", code);
    let status = parse(&search);

    return status;
}

pub fn message(message: &str) -> String {
    let search = format!("#(code=={}).message", message);
    let message = parse(&search);

    return message;
}
