use nu_plugin::Plugin;
use nu_protocol::LabeledError;
use nu_protocol::{CompareTypes, Value};

#[derive(Debug)]
pub enum NuType {
    Any,
    Boolean,
    Integer,
    Float,
    FileSize,
    Duration,
    DateTime,
    Range,
    String,
    Nothing,
    Binary,
    Glob,
    CellPath,
    Record(Vec<(String, NuType)>),
    BareRecord,
    Table(Vec<(String, NuType)>),
    BareTable,
    List(Box<NuType>),
}

impl NuType {
    fn label(&self) -> &str {
        match self {
            NuType::Any => "any",
            NuType::Boolean => "bool",
            NuType::Integer => "int",
            NuType::Float => "float",
            NuType::FileSize => "filesize",
            NuType::Duration => "duration",
            NuType::DateTime => "date",
            NuType::Range => "range",
            NuType::String => "string",
            NuType::Nothing => "nothing",
            NuType::Binary => "binary",
            NuType::Glob => "glob",
            NuType::CellPath => "cellpath",
            NuType::List(_) => "list",
            NuType::Record(_) => "record",
            NuType::BareRecord => "record",
            NuType::Table(_) => "table",
            NuType::BareTable => "table",
        }
    }
}

fn get_record(v: &Value) -> Result<&nu_protocol::Record, LabeledError> {
    match v {
        Value::Record { val, .. } => Ok(val),
        _ => Err(LabeledError::new("expected record")),
    }
}

fn parse_fields(record: &nu_protocol::Record) -> Result<Vec<(String, NuType)>, LabeledError> {
    match record.get("columns") {
        None => Ok(vec![]),
        Some(Value::Record { val: cols, .. }) => cols
            .iter()
            .map(|(k, v)| {
                get_record(v)
                    .and_then(|r| parse_describe(r))
                    .map(|t| (k.clone(), t))
            })
            .collect(),
        _ => Err(LabeledError::new("columns has unexpected shape")),
    }
}

fn parse_describe(record: &nu_protocol::Record) -> Result<NuType, LabeledError> {
    let type_str = record
        .get("type")
        .and_then(|v| {
            if let Value::String { val, .. } = v {
                Some(val.as_str())
            } else {
                None
            }
        })
        .ok_or_else(|| LabeledError::new("missing type field"))?;

    match type_str {
        "any" => Ok(NuType::Any),
        "bool" => Ok(NuType::Boolean),
        "int" => Ok(NuType::Integer),
        "float" => Ok(NuType::Float),
        "filesize" => Ok(NuType::FileSize),
        "duration" => Ok(NuType::Duration),
        "date" => Ok(NuType::DateTime),
        "range" => Ok(NuType::Range),
        "string" => Ok(NuType::String),
        "nothing" => Ok(NuType::Nothing),
        "binary" => Ok(NuType::Binary),
        "glob" => Ok(NuType::Glob),
        "cellpath" => Ok(NuType::CellPath),

        "list" => {
            let inner = match record.get("value") {
                Some(Value::List { vals, .. }) if !vals.is_empty() => {
                    get_record(&vals[0]).and_then(|r| parse_describe(r))?
                }
                _ => NuType::Any,
            };
            Ok(NuType::List(Box::new(inner)))
        }
        "record" => parse_fields(record).map(|fields| match fields.is_empty() {
            true => NuType::BareRecord,
            false => NuType::Record(fields),
        }),
        "table" => parse_fields(record).map(|fields| match fields.is_empty() {
            true => NuType::BareTable,
            false => NuType::Table(fields),
        }),

        other => Err(LabeledError::new(format!("unknown type: {other}"))),
    }
}

pub fn render_types(
    nutype: &NuType,
    name: Option<&str>,
    prefix: &str,
    child_prefix: &str,
) -> String {
    let head = match name {
        Some(n) => format!("{prefix}{n}: {}\n", nutype.label()),
        None => format!("{prefix}{}\n", nutype.label()),
    };
    match nutype {
        NuType::Record(field) => {
            let children_string =
                field
                    .iter()
                    .enumerate()
                    .fold(String::new(), |mut acc, (i, (name, nutype))| {
                        acc.push_str(&render_types(nutype, Some(name), prefix, child_prefix));
                        acc
                    });
            head + &children_string
        }
        NuType::Table(field) => {
            let children_string =
                (field)
                    .iter()
                    .enumerate()
                    .fold(String::new(), |mut acc, (i, (name, nutype))| {
                        (&mut acc).push_str(&render_types(
                            nutype,
                            Some(name),
                            prefix,
                            child_prefix,
                        ));
                        acc
                    });
            head + &children_string
        }
        NuType::List(inner) => head + &render_types(inner.as_ref(), None, prefix, child_prefix),
        _ => head,
    };
    String::new()
}

#[test]
fn test_describe_shape() {
    let output = std::process::Command::new("nu")
        .args(["-c", "http get https://jsonplaceholder.typicode.com/users | first | describe --detailed | to nuon"])
        .output()
        .expect("failed to run nu");

    let nuon_str = String::from_utf8_lossy(&output.stdout);
    let value = nuon::from_nuon(nuon_str.trim(), None).expect("failed to parse nuon");

    let result = match &value {
        Value::Record { val, .. } => parse_describe(val).expect("failed to parse describe"),
        _ => panic!("expected record"),
    };

    println!("{result:#?}");
}
