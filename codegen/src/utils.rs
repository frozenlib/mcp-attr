use anyhow::{bail, Result};
use proc_macro2::TokenStream;
use std::{
    io::Write,
    mem::take,
    process::{Command, Stdio},
};

pub fn push_derive(type_name: &str, derives: &[&str], ts: &mut TokenStream) -> Result<()> {
    let derive = format!("#[derive({})]", derives.join(","));
    let struct_def = format!("pub struct {type_name}");
    let enum_def = format!("pub enum {type_name}");
    ts_replace(&struct_def, &format!("{derive} {struct_def}"), ts)?;
    ts_replace(&enum_def, &format!("{derive} {enum_def}"), ts)?;
    Ok(())
}

pub fn ts_replace(from: &str, to: &str, ts: &mut TokenStream) -> Result<()> {
    let from = from.parse()?;
    let to = to.parse()?;
    *ts = macro_rules_rt::Rule::new(from, to)?.replace_all_tokens(take(ts));
    Ok(())
}

pub fn save(ts: TokenStream, path: &str) -> Result<()> {
    let s = rustfmt(&ts.to_string());
    std::fs::write(path, s)?;
    Ok(())
}
fn rustfmt(s: &str) -> String {
    match rustfmt_raw(s) {
        Ok(s) => s,
        Err(_) => s.replace("}", "}\n"),
    }
}
fn rustfmt_raw(s: &str) -> Result<String> {
    let child = Command::new("rustfmt")
        .args(["--config", "normalize_doc_attributes=true"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    child.stdin.as_ref().unwrap().write_all(s.as_bytes())?;
    let o = child.wait_with_output()?;
    if o.status.success() {
        Ok(std::str::from_utf8(&o.stdout)?.to_string())
    } else {
        bail!("{}", std::str::from_utf8(&o.stderr)?);
    }
}
