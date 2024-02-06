const EQUATION : &str = "a*(b+c*(d+e))=0";

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
    
    // connect the parts and operators into a string
    let mut parts_str = String::new();
    for i in 0..parts.len() {
        parts_str += (operators[i].to_string().as_str().to_owned() + parts[i].as_str()).as_str();
    }
    let b = handle_part(&mut parts_str, "1");

    println!("Final: {}", b);
    // Print the results
    

    answer
}

/*
 * This function takes a part of the equation and a coefficient
 * removes unnecessary parenthesis and spaces
 * multiplies the part by the coefficient
 * splits the function into parts and operators of + and -
 * and simplifies each part
*/
fn handle_part(part: &str, coefficient: &str) -> String {
    // if first char is + remove it
    let mut part = part;
    if let Some(remaining) = part.strip_prefix('+') {
        part = remaining;
    }

    // if part is in parenthesis remove them
    if part.chars().nth(0).unwrap() == '(' && part.chars().nth(part.len() - 1).unwrap() == ')' {
        part = &part[1..part.len() - 1];
    }

    let mut part = part.to_string();
    // multiply each part by coefficient
    if coefficient != "1" {
        part = multiply_two_parts(part.as_str(), coefficient);
    }
    let part = part.as_str();

    let mut operators = split_operators(part, vec!['+', '-']);
    let mut parts = split_parts(part, vec!['+', '-']);
    
    if parts.len() == operators.len() + 1 {
        operators.insert(0, '+');
    }
    
    // simplify each part
    for i in 0..parts.len() {
        let part = &mut parts[i];
        let operator = &mut operators[i];

        println!("Part: {} Operator: {}", part, operator);
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

fn split_parts(equation: &str, operators: Vec<char>) -> Vec<String> {
    let mut parts: Vec<String> = Vec::new();
    let mut part = String::new();
    let mut parenthesis: i8 = 0;
    for i in 0..equation.len() {
        let c = equation.chars().nth(i).unwrap();
        if c == '(' {
            parenthesis += 1;
        } else if c == ')' {
            parenthesis -= 1;
        }
        if operators.contains(&c) && parenthesis == 0 {
            if i > 0 {
                parts.push(part.clone());
                part = String::new();
            }
        } else {
            part += c.to_string().as_str();
        }
    }
    parts.push(part.clone());
    parts
}

fn split_operators(equation: &str, operators: Vec<char>) -> Vec<char> {
    let mut operators_: Vec<char> = Vec::new();
    let mut parenthesis: i8 = 0;
    for i in 0..equation.len() {
        let c = equation.chars().nth(i).unwrap();
        if c == '(' {
            parenthesis += 1;
        } else if c == ')' {
            parenthesis -= 1;
        }
        if operators.contains(&c) && parenthesis == 0 {
            operators_.push(c);
        }
    }
    operators_
}

fn simplify_part(part: &mut String) {
    // remove spaces
    *part = part.replace(" ", "").to_string();

    // split by * and /
    let mut multiply_parts = split_parts(part.as_str(), vec!['*', '/']);
    let multiply_operators = split_operators(part.as_str(), vec!['*', '/']);

    for i in 0..multiply_parts.len() {
        let part = &mut multiply_parts[i];
        if part.contains(['(', ')'].as_ref()) {
            *part = handle_part(part, "1");
        }
    }

    for i in 0..(multiply_parts.len() - 1) {
        let idk = multiply_two_parts(multiply_parts[i].as_str(), multiply_parts[i + 1].as_str());
        println!("{:?}", idk);
        *part = idk;
    }

    println!("Multiply parts: {:?} Start: {:?}", multiply_parts, part);
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
    let mut final_s = String::new();
    for i in 0..parts_1.len() {
        let part = &mut parts_1[i];
        let operator = &mut operators_1[i];
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
    final_s
}

// fn evaluate_equation(equation: &str) -> f64 {
//     0.0
// }