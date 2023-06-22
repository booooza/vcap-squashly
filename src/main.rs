use serde::Deserialize;
use std::collections::HashMap;
use std::env;

use serde_json::Value;

fn main() {
    let mut env_vars: Vec<(String, String)> = Vec::new();

    for service in get_services() {
        parse_json_to_env_vars(service.name.as_str(), &service.credentials, &mut env_vars);
    }

    for (key, value) in &env_vars {
        println!("{}", format_var(clean_var(key), value));
    }
}

pub fn get_services() -> Vec<Service> {
    let vcap_services = env::var("VCAP_SERVICES").unwrap_or_default();
    let services =
        serde_json::from_str::<HashMap<String, Vec<Service>>>(&vcap_services).unwrap_or_default();
    services.values().flatten().cloned().collect()
}

fn parse_json_to_env_vars(prefix: &str, value: &Value, env_vars: &mut Vec<(String, String)>) {
    match value {
        Value::Object(obj) => {
            for (key, value) in obj {
                let new_prefix = if prefix.is_empty() {
                    key.to_string()
                } else {
                    format!("{}_{}", prefix, key)
                };
                parse_json_to_env_vars(&new_prefix, value, env_vars);
            }
        }
        Value::Array(arr) => {
            for (index, value) in arr.iter().enumerate() {
                let new_prefix = if prefix.is_empty() {
                    "".to_string()
                } else {
                    format!("{}_{}", prefix, index)
                };
                parse_json_to_env_vars(&new_prefix, value, env_vars)
            }
        }
        Value::String(s) => {
            env_vars.push((prefix.to_string(), s.to_string()));
        }
        Value::Number(n) => {
            env_vars.push((prefix.to_string(), n.to_string()));
        }
        Value::Bool(b) => {
            env_vars.push((prefix.to_string(), b.to_string()));
        }
        _ => {}
    }
}

fn clean_var(name: &str) -> String {
    match regex::Regex::new(r#"[-.+~`!@#$%^&*(){}\\:;"',?<>/]"#) {
        Ok(re) => re.replace_all(name, "_").to_ascii_uppercase(),
        Err(_) => name.to_string(),
    }
}

fn format_var(key: String, value: &String) -> String {
    format!("export {}=\"{}\"", key, value)
}

#[derive(Deserialize, PartialEq, Debug, Clone)]
pub struct Service<Credentials = Value> {
    pub name: String,
    pub credentials: Credentials,
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    #[test]
    fn get_services_test() {
        let vcap_services: &str = "{\
            \"database\":[
                {
                    \"name\": \"db\",
                    \"credentials\": {}
                }
            ],
            \"mailserver\":[
                {
                    \"name\": \"mymailserver\",
                    \"credentials\": {}
                }
            ]
        }";

        std::env::set_var("VCAP_SERVICES", vcap_services);
        let services = crate::get_services();
        assert_eq!(2, services.len());
    }

    #[test]
    fn test_clean_var() {
        assert_eq!(crate::clean_var("username"), "USERNAME");
        assert_eq!(crate::clean_var("first-name"), "FIRST_NAME");
        assert_eq!(crate::clean_var("api.key"), "API_KEY");
        assert_eq!(crate::clean_var("$special!chars#"), "_SPECIAL_CHARS_");
    }

    #[test]
    fn test_format_var() {
        assert_eq!(
            crate::format_var("USERNAME".to_string(), &"user".to_string()),
            "export USERNAME=\"user\""
        );
    }

    #[test]
    fn test_parse_json_to_env_vars() {
        let credentials = json!({
            "uri": "db://exampleuser:examplepass@babar.database.com:5432/exampleuser",
            "nested": {
                "key": "value",
                "number": 0,
                "bool": true,
                "array": [
                    true,
                    3,
                    "string",
                    {
                        "key": "value"
                    }
                ],
                "object": {
                    "key": "value"
                }
            }
        });

        let mut env_vars: Vec<(String, String)> = Vec::new();

        crate::parse_json_to_env_vars("database", &credentials, &mut env_vars);
        assert_eq!(env_vars.len(), 9);
        assert_eq!(
            env_vars[0],
            ("database_nested_array_0".to_string(), "true".to_string())
        );
        assert_eq!(
            env_vars[1],
            ("database_nested_array_1".to_string(), "3".to_string())
        );
        assert_eq!(
            env_vars[2],
            ("database_nested_array_2".to_string(), "string".to_string())
        );
        assert_eq!(
            env_vars[3],
            (
                "database_nested_array_3_key".to_string(),
                "value".to_string()
            )
        );
        assert_eq!(
            env_vars[4],
            ("database_nested_bool".to_string(), "true".to_string())
        );
        assert_eq!(
            env_vars[5],
            ("database_nested_key".to_string(), "value".to_string())
        );
        assert_eq!(
            env_vars[6],
            ("database_nested_number".to_string(), "0".to_string())
        );
        assert_eq!(
            env_vars[7],
            (
                "database_nested_object_key".to_string(),
                "value".to_string()
            )
        );
        assert_eq!(
            env_vars[8],
            (
                "database_uri".to_string(),
                "db://exampleuser:examplepass@babar.database.com:5432/exampleuser".to_string()
            )
        );
    }
}
