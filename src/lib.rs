use serde::Deserialize;
use std::collections::HashMap;


#[derive(Deserialize)]
pub enum PrimitiveType {
    Integer,
    String,
}

#[derive(Deserialize)]
pub struct PropertySpecs {
    pub required: bool,
    pub documentation: String,
    pub primitive_type: String,
    pub update_type: String
}

#[derive(Deserialize)]
pub struct PropertyTypes {
    pub property_types: HashMap<String, PropertyType>
}

#[derive(Deserialize)]
pub struct PropertyType {
    pub name: String,
    pub documentation: String,
    pub properties: HashMap<String, PropertySpecs>
}


#[macro_export]
macro_rules! property_type_from_json {
}

#[macro_export]
macro_rules! property_type_to_cf_type {
}


#[cfg(test)]
mod tests {

    #[test]
    fn test_property_type() {
        property_type!()
    }

}

/*
AWS::AppMesh::VirtualRouter.PortMapping: {
    Documentation: "http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualrouter-portmapping.html",
    Properties: {
        Port: {
            Required: true,
            Documentation: "http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualrouter-portmapping.html#cfn-appmesh-virtualrouter-portmapping-port",
            PrimitiveType: "Integer",
            UpdateType: "Mutable"
        },
        Protocol: {
            Required: true,
            Documentation: "http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualrouter-portmapping.html#cfn-appmesh-virtualrouter-portmapping-protocol",
            PrimitiveType: "String",
            UpdateType: "Mutable"
        }
    }
}
*/