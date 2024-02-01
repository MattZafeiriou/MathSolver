pub fn handle_math_request(equation: String) -> String {
    format!("Result for equation '{}': {}", equation, evaluate_equation(&equation))
}

fn evaluate_equation(equation: &str) -> f64 {
    0.0
}