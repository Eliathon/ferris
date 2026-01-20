pub async fn parse_math_command(vec: Vec<&str>) -> Result<String, String> {
    if vec.len() != 3 {
        return Err("Invalid expression, use format <a>space<operator>space<b>".into());
    }

    let a: i32 = vec[0].parse().map_err(|_| "Invalid number")?;
    let op = vec[1];
    let b: i32 = vec[2].parse().map_err(|_| "Invalid number")?;

    let result = match op {
        "+" => a + b,
        "-" => a - b,
        "*" => a * b,
        "/" => {
            if b == 0 {
                return Err("Division by zero".into());
            }
            a / b
        }
        _ => return Err("Unknown operator".into()),
    };

    Ok(result.to_string())
}
