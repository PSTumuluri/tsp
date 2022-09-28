use std::fs;

/// Returns a vector of coordinate points based on the contents of the file.
pub fn parse_file(file_name: &str) -> Result<Vec<(i64, i64)>, &'static str> {

    let contents = extract_contents(file_name)?;
    let mut lines = contents.lines();
    let num_points = get_num_points(lines.next())?;

    let mut points = Vec::with_capacity(num_points);

    lines.next();

    for line in lines {
        points.push(get_point(line)?);
    }

    Ok(points)
}

/// Opens the specified file and reads the contents into a string.
/// Returns an error if the file could not be read.
fn extract_contents(file_name: &str) -> Result<String, &'static str> {
    let contents = fs::read_to_string(file_name);
    if let Err(_) = contents {
        return Err("error trying to read file");
    };

    Ok(contents.unwrap())
}

fn get_num_points(line: Option<&str>) -> Result<usize, &'static str> {
    let num_points = line
        .ok_or_else(|| "file was empty")?
        .split(" : ")
        .collect::<Vec<&str>>()
        .get(1)
        .ok_or_else(|| "first line not formatted properly")?
        .trim()
        .parse::<usize>();

    if let Err(_) = num_points {
        return Err("could not parse number of dimension");
    }

    Ok(num_points.unwrap())
}

fn get_point(line: &str) -> Result<(i64, i64), &'static str> {

    let point = line
        .split_whitespace()
        .collect::<Vec<&str>>();

    let (x, y) = (
        point.get(1)
            .ok_or_else(|| "x coordinate not provided")?
            .parse::<i64>(),
        point.get(2)
            .ok_or_else(|| "y coordinate not provided")?
            .parse::<i64>()
    );

    if let Err(_) = x {
        return Err("x coordinate was not an integer");
    }
    if let Err(_) = y {
        return Err("y coordinate was not an integer");
    }

    Ok((x.unwrap(), y.unwrap()))
}
