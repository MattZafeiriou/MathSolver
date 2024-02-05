const EQUATION : &str = "2*a*{var}*{var}*3+5.2*3/10=3";

/*
 * This function takes an equation, moves everything to the left side
 * and simplifies the equation.
*/
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
    
    // // connect the parts and operators into a string
    // let mut parts_str = String::new();
    // for i in 0..parts.len() {
    //     parts_str += (operators[i].to_string().as_str().to_owned() + parts[i]).as_str();
    // }
    // println!("Parts: {:?} Operators: {:?}", parts, operators);
    // let b = handle_part(&mut parts_str, "1");
    // println!("Parts: {:?}", b);

    // Print the results
    let test = multiply_two_parts("a-b", "-3");

    answer
}

/*
 * This function takes a part of the equation and a coefficient and
 * simplifies the part.
*/
fn handle_part(part: &str, coefficient: &str) -> String {
    // if first char is + remove it
    let mut part = part;
    if let Some(remaining) = part.strip_prefix('+') {
        part = remaining;
    }

    let mut operators = split_operators(part, vec!['+', '-']);
    let parts = split_parts(part, vec!['+', '-']);

    let mut parts: Vec<String> = parts.iter().map(|s| s.to_string()).collect();

    // multiply each part by coefficient
    if coefficient != "1" {
        for i in 0..parts.len() {
            let part = &mut parts[i];
            let num = part.parse::<f32>();
            if num.is_ok() {
                let coefficient_num = coefficient.parse::<f32>();
                // check if coefficient is a number
                if coefficient_num.is_ok()  {
                    let num = num.unwrap();
                    let new_num = num * coefficient_num.unwrap();
                    *part = new_num.to_string();
                } else {
                    *part = format!("{}*{}", coefficient, part);
                }
            } else {
                *part = format!("{}*{}", coefficient, part);
            }
        }
    }
    
    let mut parts: Vec<String> = parts.iter().map(|s| s.to_string()).collect();
    if parts.len() == operators.len() + 1 {
        operators.insert(0, '+');
    }
    
    // simplify each part
    for i in 0..parts.len() {
        let part = &mut parts[i];
        let operator = &mut operators[i];

        simplify_part(part);

        parts[i] = part.to_string();
        operators[i] = *operator;
    }


    // finalise the part
    let mut final_part: String = String::new();
    for i in 0..parts.len() {
        if i < operators.len() && (operators[i] != '+' || i > 0) {
            final_part += operators[i].to_string().as_str();
        }
        final_part += parts[i].as_str();
    }

    final_part
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

fn multiply_two_parts(part1: &str, part2: &str) -> String {
    let mut parts_1 = split_parts(part1, vec!['+', '-']);
    let mut parts_2 = split_parts(part2, vec!['+', '-']);
    let mut operators_1 = split_operators(part1, vec!['+', '-']);
    let mut operators_2 = split_operators(part2, vec!['+', '-']);

    if operators_1.len() == (parts_1.len() - 1) {
        operators_1.insert(0, '+');
    }
    if operators_2.len() == (parts_2.len() - 1) {
        operators_2.insert(0, '+');
    }
    println!("Parts 1: {:?} Parts 2: {:?}", parts_1, parts_2);
    println!("Operators 1: {:?} Operators 2: {:?}", operators_1, operators_2);
    println!("Lengths: {} {}", parts_1.len(), operators_1.len());
    let mut final_s = String::new();
    for i in 0..parts_1.len() {
        let part = &mut parts_1[i];
        let operator = &mut operators_1[i];
        println!("Part: {} Operator: {}", part, operator);
        for j in 0..parts_2.len() {
            let part2 = &mut parts_2[j];
            let mut operator2 = operators_2[j];
            if operator == &operator2 {
                operator2 = '+';
            } else {
                operator2 = '-';
            }
            if part2 == &"" {
                continue;
            }
            let num = part.parse::<f64>();
            let num2 = part2.parse::<f64>();
            if num.is_ok() && num2.is_ok() {
                let num = num.unwrap();
                let num2 = num2.unwrap();
                let new_num = num * num2;
                let part2 = new_num.to_string();
                final_s += (operator2.to_string() + part2.as_str()).as_str();
            } else {
                let formatted = format!("{}*{}", part, part2);
                let part2 = &mut formatted.as_str();
                final_s += (operator2.to_string() + part2).as_str();
            }
        }
    }
    println!("Final: {}", final_s);
    final_s
}

// fn evaluate_equation(equation: &str) -> f64 {
//     0.0
// }