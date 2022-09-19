// Native functions

use super::value::Value;

pub fn sqrt(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("'sqrt' expects one argument.".to_string());
    }
    let arg = &args[0];
    match arg {
        Value::Number(x) => Ok(Value::Number(x.sqrt())),
        _ => Err("Expect number as argument for 'sqrt'".to_string()),
    }
}

pub fn concat(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("'concat' expects two arguments.".to_string());
    }
    let s1 = match &args[0] {
        Value::Str(s) => s,
        _ => return Err("'concat' expects string arguments.".to_string()),
    };
    let s2 = match &args[1] {
        Value::Str(s) => s,
        _ => return Err("'concat' expects string arguments.".to_string()),
    };

    Ok(Value::Str(s1.concat(&s2)))
}