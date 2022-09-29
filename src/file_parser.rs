use std::fs;

/// Returns a vector of coordinate points based on the contents of the file.
pub fn parse_file(file_name: &str) -> Result<Vec<(f64, f64)>, &'static str> {

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
    match contents {
        Err(_) => Err("error trying to read file"),
        Ok(contents) => Ok(contents),
    }
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

    match num_points {
        Err(_) => Err("could not parse number of dimensions"),
        Ok(num_points) => Ok(num_points),
    }
}

fn get_point(line: &str) -> Result<(f64, f64), &'static str> {

    let point = line
        .split_whitespace()
        .collect::<Vec<&str>>();

    let (x, y) = (
        point.get(1)
            .ok_or_else(|| "x coordinate not provided")?
            .parse::<f64>(),
        point.get(2)
            .ok_or_else(|| "y coordinate not provided")?
            .parse::<f64>()
    );

    let x = match x {
        Err(_) =>  return Err("x coordinate was not an integer"),
        Ok(x) => x,
    };
    let y = match y {
        Err(_) => return Err("y coordinate was not an integer"),
        Ok(y) => y,
    };

    Ok((x, y))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gets_correct_num_points() {
        let line = "DIMENSIONS : 123";
        assert_eq!(123, get_num_points(Some(line)).unwrap());
    }

    #[test]
    fn gets_correct_point() {
        let line = "column1 123 321";
        assert_eq!((123.0, 321.0), get_point(line).unwrap());
    }
}
