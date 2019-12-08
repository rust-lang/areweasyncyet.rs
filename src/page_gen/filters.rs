use once_cell::sync::Lazy;
use regex::{Captures, Regex};
use std::collections::HashMap;
use tera::{self, Value};

static RE_CODIFY: Lazy<Regex> = Lazy::new(|| Regex::new(r"\&#96;(.+?)\&#96;").unwrap());

pub fn codify(value: &Value, _: &HashMap<String, Value>) -> tera::Result<Value> {
    let value = match value {
        Value::String(s) => s,
        _ => return Err(format!("unsupported value for codify: {:?}", value).into()),
    };
    let result = RE_CODIFY.replace_all(&value, |captures: &Captures| {
        format!("<code>{}</code>", captures.get(1).unwrap().as_str())
    });
    Ok(result.into())
}

pub fn pr_url(value: &Value, _: &HashMap<String, Value>) -> tera::Result<Value> {
    let number = get_issue_number(value)?;
    Ok(format!("https://github.com/rust-lang/rust/pull/{}", number).into())
}

pub fn issue_url(value: &Value, _: &HashMap<String, Value>) -> tera::Result<Value> {
    let number = get_issue_number(value)?;
    Ok(format!("https://github.com/rust-lang/rust/issues/{}", number).into())
}

fn get_issue_number(value: &Value) -> tera::Result<u64> {
    let number = match value {
        Value::Number(n) => n
            .as_u64()
            .ok_or_else(|| format!("unsupport number: {:?}", n))?,
        _ => return Err(format!("unsupported value for issue number: {:?}", value).into()),
    };
    Ok(number)
}
