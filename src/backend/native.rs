// Native functions

use super::value::Value;

pub fn sqrt(args: Vec<Value>) -> Value {
    if args.len() != 1 {
        return Value::Nil;
    }
    let arg = &args[0];
    match arg {
        Value::Number(x) => Value::Number(x.sqrt()),
        _ => Value::Nil,
    }
}

pub fn concat(args: Vec<Value>) -> Value {
    if args.len() != 2 {
        return Value::Nil;
    }
    let s1 = match &args[0] {
        Value::Str(s) => s,
        _ => return Value::Nil,
    };
    let s2 = match &args[1] {
        Value::Str(s) => s,
        _ => return Value::Nil,
    };

    Value::Str(s1.concat(&s2))
}