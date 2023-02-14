use regex::Regex;
struct Os<'a> {
    regex_start_init_object: Regex,
    move_command: Box<[&'a str]>,
    folder_delimiter: Box<[&'a str]>,
}
