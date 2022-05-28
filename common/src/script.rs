use std::collections::HashMap;
use serde_json::Value;

use slab::Slab;
#[derive(Clone)]
pub struct ScriptNode {
    pub component: String,
    pub label: Option<String>,
    pub component_args: HashMap<String, Value>,
    pub env: HashMap<String, Value>,
    pub subnodes: Slab<ScriptNode>
}

impl ScriptNode {
    pub fn new(content: String, env: HashMap<String, Value>) -> ScriptNode{
        let v: Value = serde_json::from_str(&content).unwrap();
        let mut node = ScriptNode {
            component: String::from("default"),
            label: None,
            component_args: HashMap::new(),
            env,
            subnodes: Slab::new(),
        };
        _ = v.as_object().unwrap().clone()
            .into_iter()
            .for_each(|(key, val)| {
                // println!("--- > {}: {}",&key,val.as_str().unwrap_or("default"));
                if key.eq_ignore_ascii_case("component") {
                    
                    node.component = val.as_str().unwrap_or("default").to_string();
                } else if key.eq_ignore_ascii_case("env") {
                    if let Value::Object(map) = val {
                        node.env.extend(map.clone().into_iter());
                    }
                } else if key.eq_ignore_ascii_case("args") {
                    if let Value::Object(map) = val {
                        node.component_args.extend(map.clone().into_iter());
                    }
                } else if key.eq_ignore_ascii_case("label") {
                    node.label = Some(val.as_str().unwrap().to_string());
                } else if let Value::Object(obj) = val  {
                    node.subnodes.insert(
                        ScriptNode::new(
                            serde_json::to_string(&obj).unwrap(), 
                            node.env.clone(),
                        ),
                    );
                }
            });

        return  node;
    }

}

#[cfg(test)]
mod script_tests {

    use std::collections::HashMap;

    use serde_json::Value;

    use crate::script::ScriptNode;

    #[test]
    fn load_script() {
        let s = r#"
        {
            "component": "Plan",
            "env": {
                "name": "liudao",
                "token": "demo token"
            },
            "thread_group": {
                "component": "ThreadGroup",
                "label": "my thread",
                "env": {
                    "c": 18
                },
                "args": {
                    "num": 10
                }
            }
        }
        "#;
        let node = ScriptNode::new(s.to_string(), HashMap::<String, Value>::new());
        assert_eq!(node.component, "Plan");
        assert_eq!(node.env.get("name"), Some(&serde_json::json!("liudao")));
        assert_eq!(node.subnodes.get(0).unwrap().component, "ThreadGroup");
        assert_eq!(node.subnodes.get(0).unwrap().env.get("c"), Some(&serde_json::json!(18)));
        assert_eq!(node.subnodes.get(0).unwrap().env.get("token"), Some(&serde_json::json!("demo token")));
    }
}