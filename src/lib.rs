use chrono::{FixedOffset, TimeDelta};
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "$type")]
pub enum Val {
    // Null {
    //     #[serde(rename = "_value")]
    //     value: Option<T>,
    // },
    Boolean {
        #[serde(rename = "_value")]
        value: bool,
    },
    Integer {
        #[serde(
            rename = "_value",
            deserialize_with = "try_i64_from_str")]
        value: i64,
    },
    Float {
        #[serde(
            rename = "_value",
            deserialize_with = "try_f64_from_str")]
        value: f64,
    },
    String {
        #[serde(rename = "_value")]
        value: String,
    },
    ByteArray {
        #[serde(rename = "_value")]
        value: Box<[u8]>,
    },
    Map {
        #[serde(rename = "_value")]
        value: HashMap<String, Box<Val>>,
    },
    List {
        #[serde(rename = "_value")]
        value: Vec<Val>,
    },
    ZoneDateTime {
        #[serde(
            rename = "_value",
            deserialize_with = "try_zdt_from_str"
        )]
        value: chrono::DateTime<FixedOffset>,
    },
    DateTime {
        #[serde(
        rename = "_value",
        deserialize_with = "try_dt_from_str",
        )]
        value: chrono::NaiveDateTime,
    },
    Time {
        #[serde(
        rename = "_value",
        deserialize_with = "try_time_from_str",
        )]
        value: chrono::NaiveTime,
    },
    Date {
        #[serde(
        rename = "_value",
        deserialize_with = "try_date_from_str",
        )]
        value: chrono::NaiveDate,
    },
    Duration {
        #[serde(
        rename = "_value",
        deserialize_with = "try_duration_from_str",
        )]
        value: chrono::TimeDelta,
    },
}

fn try_i64_from_str<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    let string: String = Deserialize::deserialize(deserializer).unwrap();
    let integer: i64 = string.parse().unwrap();
    Ok(integer)
}

fn try_f64_from_str<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let string: String = Deserialize::deserialize(deserializer).unwrap();
    let float: f64 = string.parse().unwrap();
    Ok(float)
}

fn try_zdt_from_str<'de, D>(deserializer: D) -> Result<chrono::DateTime<FixedOffset>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(chrono::DateTime::<FixedOffset>::parse_from_str(
        Deserialize::deserialize(deserializer).unwrap(),
        "%Y-%m-%dT%H:%M:%S[%:z]",
    )
    .unwrap())
}

fn try_dt_from_str<'de, D>(deserializer: D) -> Result<chrono::NaiveDateTime, D::Error>
    where
        D: Deserializer<'de>,
{
    Ok(chrono::NaiveDateTime::parse_from_str(
        Deserialize::deserialize(deserializer).unwrap(),
        "%Y-%m-%dT%H:%M:%S",
    ).unwrap())
}

fn try_time_from_str<'de, D>(deserializer: D) -> Result<chrono::NaiveTime, D::Error>
    where
        D: Deserializer<'de>,
{
    Ok(chrono::NaiveTime::parse_from_str(
        Deserialize::deserialize(deserializer).unwrap(),
        "%H:%M:%S",
    ).unwrap())
}
fn try_date_from_str<'de, D>(deserializer: D) -> Result<chrono::NaiveDate, D::Error>
    where
        D: Deserializer<'de>,
{
    Ok(chrono::NaiveDate::parse_from_str(
        Deserialize::deserialize(deserializer).unwrap(),
        "%Y-%m-%d",
    ).unwrap())
}
fn try_duration_from_str<'de, D>(deserializer: D) -> Result<chrono::TimeDelta, D::Error>
    where
        D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer).unwrap();
    Ok(chrono::TimeDelta::new(0,0).unwrap())
}

//
#[derive(Debug, Clone, Deserialize)]
struct Body {
    #[serde(flatten)]
    body: Box<Val>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Timelike;

    // #[test]
    // fn null_deserializes() {
    //     let test = "{ \"$type\":\"Null\", \"_value\": null }";
    //     let inputs = serde_json::Deserializer::from_str(test).into_iter::<Body>();
    //     for input in inputs {
    //         match *input.unwrap().body {
    //             Val::Null { value } => {
    //                 assert_eq!(None, value);
    //             }
    //             _ => panic!("test fail"),
    //         };
    //     }
    // }

    #[test]
    fn bool_deserializes() {
        let test = "{ \"$type\":\"Boolean\", \"_value\": true }";
        let inputs = serde_json::Deserializer::from_str(test).into_iter::<Body>();
        for input in inputs {
            match *input.unwrap().body {
                Val::Boolean { value } => {
                    assert_eq!(true, value);
                }
                _ => panic!("test fail"),
            };
        }
    }

    #[test]
    fn i64_deserializes() {
        let test = "{ \"$type\":\"Integer\", \"_value\": \"10\" }";
        let inputs = serde_json::Deserializer::from_str(test).into_iter::<Body>();
        for input in inputs {
            match *input.unwrap().body {
                Val::Integer { value } => {
                    assert_eq!(10i64, value);
                }
                _ => panic!("test fail"),
            };
        }
    }

    #[test]
    fn f64_deserializes() {
        let test = "{ \"$type\":\"Float\", \"_value\": \"1.0\" }";
        let inputs = serde_json::Deserializer::from_str(test).into_iter::<Body>();
        for input in inputs {
            match *input.unwrap().body {
                Val::Float { value } => {
                    assert_eq!(1.0, value);
                }
                _ => panic!("test fail"),
            };
        }
    }

    #[test]
    fn string_deserializes() {
        let test = "{ \"$type\":\"String\", \"_value\": \"bert\" }";
        let inputs = serde_json::Deserializer::from_str(test).into_iter::<Body>();
        for input in inputs {
            match *input.unwrap().body {
                Val::String { value } => {
                    assert_eq!("bert", value);
                }
                _ => panic!("test fail"),
            };
        }
    }

    #[test]
    fn u8_deserializes() {
        let test = "{ \"$type\":\"ByteArray\", \"_value\": [1,2,3,4,255] }";
        let inputs = serde_json::Deserializer::from_str(test).into_iter::<Body>();
        for input in inputs {
            match *input.unwrap().body {
                Val::ByteArray { value } => {
                    let res: &[u8] = &*value;
                    assert_eq!([1u8,2u8,3u8,4u8,255u8], res);
                }
                _ => panic!("test fail"),
            };
        }
    }

    #[test]
    fn map_deserializes() {
        let test = "{ \"$type\":\"Map\", \"_value\": {\"k\": { \"$type\":\"String\", \"_value\": \"bert\" } } }";
        let inputs = serde_json::Deserializer::from_str(test).into_iter::<Body>();
        for input in inputs {
            match *input.unwrap().body {
                Val::Map { value } => {
                    let v = value.get("k").unwrap().as_ref().clone();
                    match v {
                        Val::String { value } => {
                            assert_eq!("bert", value);
                        }
                        _ => panic!("test fail"),
                    }
                }
                _ => panic!("test fail"),
            };
        }
    }

    #[test]
    fn nest_map_deserializes() {
        let test = "{ \"$type\":\"Map\", \"_value\": {\"k\": { \"$type\":\"Map\", \"_value\":  {\"m\": { \"$type\":\"String\", \"_value\": \"bert\" } } } } }";
        let inputs = serde_json::Deserializer::from_str(test).into_iter::<Body>();
        for input in inputs {
            match *input.unwrap().body {
                Val::Map { value } => {
                    let v = value.get("k").unwrap().as_ref().clone();
                    match v {
                        Val::Map { value } => {
                            let r = value.get("m").unwrap().as_ref().clone();
                            match r {
                                Val::String { value } => {
                                    assert_eq!("bert", value);
                                }
                                _ => panic!("test fail"),
                            }
                        }
                        _ => panic!("test fail"),
                    }
                }
                _ => panic!("test fail"),
            };
        }
    }

    #[test]
    fn zdt_deserializes() {
        let test = "{ \"$type\":\"ZoneDateTime\", \"_value\": \"2012-01-01T12:00:00[+02:00]\" }";
        let inputs = serde_json::Deserializer::from_str(test).into_iter::<Body>();
        for input in inputs {
            match *input.unwrap().body {
                Val::ZoneDateTime { value } => {
                    assert_eq!(12u32, value.hour());
                }
                _ => panic!("test fail"),
            };
        }
    }
    #[test]
    fn dt_deserializes() {
        let test = "{ \"$type\":\"DateTime\", \"_value\": \"2012-01-01T12:00:00\" }";
        let inputs = serde_json::Deserializer::from_str(test).into_iter::<Body>();
        for input in inputs {
            match *input.unwrap().body {
                Val::DateTime { value } => {
                    assert_eq!(12u32, value.hour());
                }
                _ => panic!("test fail"),
            };
        }
    }
    // #[test ]
    // fn duration_deserializes() {
    //     let test = "{ \"$type\":\"Duration\", \"_value\": \"P14DT16H12M\" }";
    //     let inputs = serde_json::Deserializer::from_str(test).into_iter::<Body>();
    //     for input in inputs {
    //         match *input.unwrap().body {
    //             Val::DateTime { value } => {
    //                 assert_eq!(12u32, value.hour());
    //             }
    //             _ => panic!("test fail"),
    //         };
    //     }
    // }
    #[test]
    fn list_deserializes() {
        let test = "{ \"$type\":\"List\", \"_value\": [{ \"$type\":\"Integer\", \"_value\": \"10\" }]}";
        let inputs = serde_json::Deserializer::from_str(test).into_iter::<Body>();
        for input in inputs {
            match *input.unwrap().body {
                Val::List { value } => {
                    match *value.get(0).unwrap() {
                        Val::Integer {value} => {
                            assert_eq!(value, 10);
                        }
                        _ => panic!("test fail"),
                    }
                }
                _ => panic!("test fail"),
            };
        }
    }
}

#[test]
fn nest_list_deserializes() {
    let test = "{ \"$type\":\"List\", \"_value\": [{ \"$type\":\"List\", \"_value\": [{\"$type\":\"Integer\", \"_value\": \"10\"}] }]}";
    let inputs = serde_json::Deserializer::from_str(test).into_iter::<Body>();
    for input in inputs {
        match *input.unwrap().body {
            Val::List { value } => {
                let outer = value.get(0).unwrap();
                match outer {
                    Val::List { value } => {
                        let inner = value.get(0).unwrap();
                        match inner {
                            Val::Integer {value} => {
                                assert_eq!(*value, 10);
                            }
                            _ => panic!("test fail"),
                        }
                    }
                    _ => panic!("test fail"),
                }
            }
            _ => panic!("test fail"),
        };
    }
}
