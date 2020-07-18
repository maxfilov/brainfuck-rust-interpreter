use std::io::Read;

pub fn get_source_code<T: Read>(input: &mut T) -> std::result::Result<String, std::io::Error> {
    let mut result = String::new();
    input.read_to_string(&mut result)?;
    Ok(result)
}

