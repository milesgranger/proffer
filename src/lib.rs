use std::collections::HashMap;

use codegen::{Scope, Module};
use serde::{Deserialize, Serialize};
use serde_json::Value;

type ResourceTypes = HashMap<String, ResourceSpec>;
type Properties = HashMap<String, PropertySpec>;

#[derive(Serialize, Deserialize)]
pub(crate) enum PrimitiveType {
    Integer,
    String,
}

impl PrimitiveType {
    pub fn as_rust_type_str(&self) -> &str {
        match self {
            PrimitiveType::String => "&str",
            PrimitiveType::Integer => "i32",
        }
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) enum Type {
    List,
}

#[derive(Serialize, Deserialize)]
pub(crate) enum UpdateType {
    Mutable,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct PropertySpec {
    #[serde(alias = "Required")]
    pub required: bool,

    #[serde(alias = "Documentation")]
    pub documentation: String,

    #[serde(alias = "PrimitiveType")]
    pub primitive_type: PrimitiveType,

    #[serde(alias = "UpdateType")]
    pub update_type: UpdateType,

    #[serde(alias = "Type")]
    pub type_: Option<Type>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct ResourceSpec {
    #[serde(alias = "Documentation")]
    pub documentation: String,

    #[serde(alias = "Properties")]
    pub properties: HashMap<String, PropertySpec>,
}


pub(crate) fn generate_resource(scope: &mut Scope, resource_name: &str, resource_spec: &ResourceSpec) {



    let pieces = resource_name.split("::").collect::<Vec<&str>>();
    let modules = &pieces[..pieces.len() - 1];
    let struct_name = &pieces[pieces.len() - 1].replace('.', "_");

    println!("{:?}", modules);

    // drill down to the module that this struct belongs in; creating them along the way.
    let aws_module = scope.get_or_new_module("CloudFormation");
    let module = modules.iter().fold(aws_module, |current_module, new_module| current_module.get_or_new_module(new_module));

    let mut tmp_scope = codegen::Scope::new();
    let s = tmp_scope.new_struct(&struct_name);
    s.doc(&format!("Documentation: {}", &resource_spec.documentation));
    resource_spec.properties.iter()
        .for_each(|(property_name, property_spec)| {
            s.field(property_name, property_spec.primitive_type.as_rust_type_str());
        });
    module.push_struct(s.clone());

}


pub(crate) fn generate(json_spec: Value) -> String {
    let resources = serde_json::from_value::<ResourceTypes>(json_spec).unwrap();

    let mut scope = codegen::Scope::new();
    resources.iter()
        .for_each(|(resource_name, resouce_spec)| {
            generate_resource(&mut scope, resource_name, resouce_spec)
        });
    scope.to_string()
}


#[cfg(test)]
mod tests {

    use crate::{ResourceSpec, ResourceTypes, generate_resource, generate};
    use serde_json::{json, Value};

    #[test]
    fn test_deserialization() {
        let resource = json!({
            "AWS::AppMesh::VirtualRouter.PortMapping": {
                "Documentation": "http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualrouter-portmapping.html",
                "Properties": {
                    "Port": {
                        "Required": true,
                        "Documentation": "http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualrouter-portmapping.html#cfn-appmesh-virtualrouter-portmapping-port",
                        "PrimitiveType": "Integer",
                        "UpdateType": "Mutable"
                    },
                    "Protocol": {
                        "Required": true,
                        "Documentation": "http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualrouter-portmapping.html#cfn-appmesh-virtualrouter-portmapping-protocol",
                        "PrimitiveType": "String",
                        "UpdateType": "Mutable"
                    }
                }
            }
        });
        assert!(serde_json::from_value::<ResourceTypes>(resource.clone()).is_ok());
        assert!(serde_json::from_value::<ResourceSpec>(
            resource["AWS::AppMesh::VirtualRouter.PortMapping"].clone()
        )
        .is_ok());
    }

    #[test]
    fn test_generation() {
        let resources = json!({
            "AWS::AppMesh::VirtualRouter.PortMapping": {
                "Documentation": "http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualrouter-portmapping.html",
                "Properties": {
                    "Port": {
                        "Required": true,
                        "Documentation": "http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualrouter-portmapping.html#cfn-appmesh-virtualrouter-portmapping-port",
                        "PrimitiveType": "Integer",
                        "UpdateType": "Mutable"
                    },
                    "Protocol": {
                        "Required": true,
                        "Documentation": "http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualrouter-portmapping.html#cfn-appmesh-virtualrouter-portmapping-protocol",
                        "PrimitiveType": "String",
                        "UpdateType": "Mutable"
                    }
                }
            }
        });
        let source_code = generate(resources);
        println!("{}", source_code);
    }

}
