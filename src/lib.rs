mod file_parser;

/// Attempts to run the traveling salesperson problem, using the file named
/// by the argument to populate the map.
/// Returns an Error if the string does not represent a file, or if the file
/// is not correctly formatted.
pub fn run(file_name: &str) -> Result<(), &'static str> { 

    let point_vector = file_parser::parse_file(file_name)?;

    for i in point_vector {
        println!("{:?}", i);
    }

    Ok(())
}
