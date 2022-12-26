mod input;
mod output;

fn main() {
    let input = input::read_files::read();
    output::create_output::create(input.0, input.1);
}
