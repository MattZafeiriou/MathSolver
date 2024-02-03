const EQUATION : &str = "2*a*{var}*{var}*3+5.2*3/10=3";


pub fn handle_math_equation(equation: String) -> String {
    // format!("Result for equation '{}': {}", equation, evaluate_equation(&equation))
    let equation = EQUATION;
    let mut answer = String::new();

    let left_right_side: Vec<&str> = equation.split('=').collect();
    if left_right_side.len() != 2 {
        return "Invalid equation".to_string();
    }
    
    let left_side = left_right_side[0];
    let right_side = left_right_side[1];

    if right_side.len() == 0 || left_side.len() == 0 {
        return "Invalid equation".to_string();
    }

    // Separate the left operators into another array
    let mut operators_l = split_operators(left_side, vec!['+', '-']);
    let parts_l = split_parts(left_side, vec!['+', '-']);

    // Separate the right operators into another array
    let mut operators_r = split_operators(right_side, vec!['+', '-']);
    let parts_r = split_parts(right_side, vec!['+', '-']);

    // add '+' to the operators if the first part is positive
    if parts_l.len() == operators_l.len() + 1 {
        operators_l.insert(0, '+');
    }
    if parts_r.len() == operators_r.len() + 1 {
        operators_r.insert(0, '+');
    }

    // move the right side to the left side
    let mut parts = parts_l.clone();
    let mut operators = operators_l.clone();
    if !(parts_r.len() == 1 && parts_r[0] == "0")
    {
        for part in parts_r {
            parts.push(part);
        }
        for op in operators_r {
            if op == '+'
            {
                operators.push('-');
            } else if op == '-'
            {
                operators.push('+');
            } else {
                operators.push(op);
            }
        }
    }

    // check if lengths are the same
    if operators.len() != parts.len() {
        return "Something went wrong! Lengths are not the same.".to_string();
    }
    
    let mut parts_str = String::new();
    for i in 0..parts.len() {
        parts_str += (operators[i].to_string().as_str().to_owned() + parts[i]).as_str();
    }
    handle_part(&mut parts_str);

    let mut parts: Vec<String> = parts.iter().map(|s| s.to_string()).collect();

    // simplify each part
    for i in 0..parts.len() {
        let part = &mut parts[i];
        let operator = &mut operators[i];

        simplify_part(part);

        parts[i] = part.to_string();
        operators[i] = *operator;
    }

    // Print the results


    answer
}

fn handle_part(part: &str) -> String {
    // if first char is + remove it
    let mut part = part;
    if let Some(remaining) = part.strip_prefix('+') {
        part = remaining;
    }

    let mut operators = split_operators(part, vec!['+', '-']);
    let parts = split_parts(part, vec!['+', '-']);

    let mut parts: Vec<String> = parts.iter().map(|s| s.to_string()).collect();

    // print
    println!("Parts: {:?}", part);

    part.to_string()
}

fn split_parts(equation: &str, operators: Vec<char>) -> Vec<&str> {
    let parts: Vec<&str> = equation
        .split(|c| operators.contains(&c))
        .collect();
    parts
}

fn split_operators(equation: &str, operators: Vec<char>) -> Vec<char> {
    let operators: Vec<char> = equation
        .chars()
        .filter(|c| operators.contains(c))
        .collect();
    operators
}

fn simplify_part(part: &mut String) {
    // remove spaces
    *part = part.replace(" ", "").to_string();

    // split by * and /
    let multiply_parts = split_parts(part.as_str(), vec!['*', '/']);
    let multiply_operators = split_operators(part.as_str(), vec!['*', '/']);

    let mut multiply_parts: Vec<String> = multiply_parts.iter().map(|s| s.to_string()).collect();
    let mut product: f64 = 1f64;
    let mut product_var: String = String::new();

    if multiply_parts.len() > 1 {
        // simplify the multiply parts
        for i in 0..multiply_parts.len() {
            let m_part = &mut multiply_parts[i];
            let num = m_part.parse::<f64>();
            if num.is_ok() {
                let num = num.unwrap();
                if i == 0 {
                    product = num;
                } else {
                    if multiply_operators[i - 1] == '*' {
                        product *= num;
                    } else if multiply_operators[i - 1] == '/' {
                        product /= num;
                    }
                }
                product = (product * 1000f64).round() / 1000f64;
                multiply_parts[i] = product.to_string();
            } else {
                if i == 0 {
                    product_var = m_part.to_string();
                } else {
                    if multiply_operators[i - 1] == '*' {
                        product_var = format!("{}*{}", product_var, m_part);
                    } else if multiply_operators[i - 1] == '/' {
                        product_var = format!("{}/{}", product_var, m_part);
                    }
                }
            }
        }
    } else {
        let num = multiply_parts[0].parse::<f64>();
        if num.is_ok() {
            product = num.unwrap();
        } else {
            product_var = multiply_parts[0].to_string();
        }
    }

    let mut final_product: String = product.to_string();

    if product_var.len() > 0 {
        final_product += product_var.as_str();
    }

    *part = final_product.to_string();
}

// fn evaluate_equation(equation: &str) -> f64 {
//     0.0
// }