use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
const ARROW: char = '\u{21B3}';

pub fn read_command(rl: &mut DefaultEditor, current_folder: String) -> Result<String, ReadlineError> {
    let mut buffer = String::new();
    let root_indent = current_folder.len() / 2;
    let mut indent_stack: Vec<usize> = vec![root_indent];

    println!("{}: ", current_folder);

    loop {
        let indent = *indent_stack.last().unwrap();
        let prompt = format!("{}{} ", " ".repeat(indent), ARROW);
        let line = rl.readline(&prompt)?;
        let trimmed = line.trim();

        if !buffer.is_empty() {
            buffer.push(' ');
        }
        buffer.push_str(trimmed);

        if trimmed.ends_with(';') {
            if indent_stack.len() == 1 {
                return Ok(buffer);
            }
            indent_stack.pop();
        } else if trimmed.ends_with(':') {
            let name = trimmed.trim_end_matches(':');
indent_stack.push(indent + (name.len() + 1) / 2);
        }
    }
}
