use std::{collections::BTreeMap, fmt::Display, str::FromStr};

use jsoncall::{ErrorCode, bail_public};
use serde::de::DeserializeOwned;
use serde_json::{Map, Value};
use uri_template_ex::Captures;

use crate::Result;

pub use uri_template_ex;

pub fn parse_prompt_arg_opt<T>(
    arguments: &BTreeMap<String, String>,
    name: &str,
) -> Result<Option<T>>
where
    T: FromStr,
    T::Err: Display,
{
    if let Some(value) = arguments.get(name) {
        match T::from_str(value) {
            Ok(v) => Ok(Some(v)),
            Err(e) => bail_public!(
                ErrorCode::INVALID_PARAMS,
                "argument `{name}` is invalid ({e})"
            ),
        }
    } else {
        Ok(None)
    }
}

pub fn parse_prompt_arg<T>(arguments: &BTreeMap<String, String>, name: &str) -> Result<T>
where
    T: FromStr,
    T::Err: Display,
{
    if let Some(value) = parse_prompt_arg_opt(arguments, name)? {
        Ok(value)
    } else {
        bail_public!(ErrorCode::INVALID_PARAMS, "argument `{name}` is required");
    }
}

pub fn parse_resource_arg_opt<T>(captures: &Captures, index: usize, name: &str) -> Result<Option<T>>
where
    T: FromStr,
    T::Err: Display,
{
    if let Some(value) = captures.get(index) {
        match T::from_str(value.value()?.as_ref()) {
            Ok(v) => Ok(Some(v)),
            Err(e) => bail_public!(
                ErrorCode::INVALID_PARAMS,
                "variable `{name}` is invalid ({e})"
            ),
        }
    } else {
        Ok(None)
    }
}

pub fn parse_resource_arg<T>(captures: &Captures, index: usize, name: &str) -> Result<T>
where
    T: FromStr,
    T::Err: Display,
{
    if let Some(value) = parse_resource_arg_opt(captures, index, name)? {
        Ok(value)
    } else {
        bail_public!(ErrorCode::INVALID_PARAMS, "variable `{name}` is required");
    }
}

pub fn parse_tool_arg_opt<T>(
    arguments: &Option<Map<String, Value>>,
    name: &str,
) -> Result<Option<T>>
where
    T: DeserializeOwned,
{
    if let Some(arguments) = arguments {
        let value = arguments.get(name);
        if let Some(value) = value {
            return Ok(Some(serde_json::from_value(value.clone())?));
        }
    }
    Ok(None)
}
pub fn parse_tool_arg<T: DeserializeOwned>(
    arguments: &Option<Map<String, Value>>,
    name: &str,
) -> Result<T> {
    if let Some(value) = parse_tool_arg_opt(arguments, name)? {
        Ok(value)
    } else {
        bail_public!(ErrorCode::INVALID_PARAMS, "argument `{name}` is required");
    }
}
