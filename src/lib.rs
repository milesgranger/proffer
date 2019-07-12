use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) enum PrimitiveType {
    Integer,
    String,
}

#[derive(Deserialize)]
pub(crate) struct PropertySpecs {

    #[serde(alias = "Required")]
    pub required: bool,

    #[serde(alias = "Documentation")]
    pub documentation: String,

    #[serde(alias = "PrimitiveType")]
    pub primitive_type: String,

    #[serde(alias = "UpdateType")]
    pub update_type: String,
}

type PropertyTypes = HashMap<String, PropertyType>;

#[derive(Deserialize)]
pub(crate) struct PropertyType {
    #[serde(alias = "Documentation")]
    pub documentation: String,

    #[serde(alias = "Properties")]
    pub properties: HashMap<String, PropertySpecs>,
}

#[cfg(test)]
mod tests {

    use serde_json::json;
    use crate::{PropertyType, PropertyTypes};

    #[test]
    fn test_PropertyType() {
        let property_json = json!({
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

        assert!(serde_json::from_value::<PropertyTypes>(property_json.clone()).is_ok());
        assert!(serde_json::from_value::<PropertyType>(property_json["AWS::AppMesh::VirtualRouter.PortMapping"].clone()).is_ok());
    }

}
