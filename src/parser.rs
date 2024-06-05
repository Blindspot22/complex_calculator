use crate::operations;

pub fn parse_and_compute(expression: &str) -> Result<f64, &'static str> {
    let parts: Vec<&str> = expression.split_whitespace().collect();
    if parts.len() != 3 {
        return Err("Invalid expression format");
    }

    let a = parts[0].parse::<f64>().map_err(|_| "Invalid number")?;
    let b = parts[2].parse::<f64>().map_err(|_| "Invalid number")?;
    let operator = parts[1];

    match operator {
        "+" => Ok(operations::add(a, b)),
        "-" => Ok(operations::subtract(a, b)),
        "*" => Ok(operations::multiply(a, b)),
        "/" => operations::divide(a, b),
        _ => Err("Unknown operator"),
    }
}
