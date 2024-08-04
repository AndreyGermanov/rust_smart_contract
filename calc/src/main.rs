enum Operation {
    Add {x:f64,y:f64},
    Subtract {x:f64,y:f64},
    Multiply {x:f64,y:f64},
    Divide {x:f64,y:f64}
}

impl Operation {
    fn calculate(&self) -> f64 {
        match self {
            Operation::Add{x, y} => x+y,
            Operation::Subtract{x, y} => x-y,
            Operation::Multiply{x, y} => x*y,
            Operation::Divide{x, y} => x/y,
        }
    }
}

fn main() {
    let stdin = std::io::stdin();
    let (mut sfirst, mut sop, mut ssecond) = (String::new(),String::new(),String::new());
    let (mut x, mut y, mut op) = (0.0,0.0, 0);
    println!("Enter a first number");
    if stdin.read_line(&mut sfirst).is_ok() {
        if let Ok(v) = sfirst.trim().parse::<f64>() {
            x = v;
        } else {
            panic!("Incorrect value");
        }
    }
    println!("Select an operation: 1 - Add, 2 - Subtract, 3 - Multiply 4 - Divide");
    if stdin.read_line(&mut sop).is_ok() {
        if let Ok(v) = sop.trim().parse::<usize>() {            
            if v < 1 || v > 4 { 
                panic!("Incorrect value");
            }
            op = v;
        } else {
            panic!("Incorrect value");
        }
    }
    println!("Enter a second number");
    if stdin.read_line(&mut ssecond).is_ok() {
        if let Ok(v) = ssecond.trim().parse::<f64>() {
            y = v;
        }
    }
    let operation = match op {
        1 => Operation::Add { x,y },
        2 => Operation::Subtract { x,y },
        3 => Operation::Multiply { x,y },
        4 => Operation::Divide { x,y },
        _ => panic!("Incorrect operation")
    };
    println!("{}",operation.calculate());
}
