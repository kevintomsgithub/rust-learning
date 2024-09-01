mod arrays;
mod conditional;
mod functions;
mod loops;
mod pointers;
mod print;
mod string;
mod structs;
mod tuple;
mod types;
mod vars;
mod vectors;

fn main() {
    print::run();
    vars::run();
    types::run();
    string::run();
    tuple::run();
    arrays::run();
    vectors::run();
    conditional::run();
    loops::run();
    functions::run();
    pointers::run();
    structs::run();
}
