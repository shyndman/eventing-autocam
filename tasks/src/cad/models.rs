use std::collections::HashMap;

use anyhow::Result;
use reqwest::Url;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Document {
    pub id: String,
    pub name: String,
    #[serde(rename = "defaultWorkspace")]
    pub default_workspace: Workspace,
}

#[derive(Debug, Deserialize)]
pub struct Workspace {
    pub id: String,
    pub name: String,
    pub href: Url,
}

#[derive(Debug, Deserialize)]
pub struct DocumentElement {
    pub id: String,
    pub name: String,
    #[serde(rename = "filename")]
    pub file_name: Option<String>,
    #[serde(rename = "elementType")]
    pub element_type: TabElementType,
}

#[derive(Debug, Deserialize)]
pub struct AssemblyDefinition {
    pub parts: Vec<Part>,

    #[serde(rename = "rootAssembly")]
    pub root_assembly: Assembly,

    #[serde(rename = "subAssemblies")]
    pub sub_assemblies: Vec<SubAssembly>,
}

impl AssemblyDefinition {
    fn get_part_map(&self) -> Result<HashMap<(String, String), &Part>> {
        Ok(self
            .parts
            .iter()
            .map(|p| ((p.element_id.clone(), p.part_id.clone()), p))
            .collect())
    }

    pub fn all_part_instances(&self) -> Vec<(&Instance, &Part)> {
        let part_map = self.get_part_map().expect("Failed to build part map");
        self.root_assembly
            .instances
            .iter()
            .chain(
                self.sub_assemblies
                    .iter()
                    .flat_map(|sa| sa.instances.iter()),
            )
            .filter(|inst| inst.instance_type == InstanceType::Part)
            .map(|inst| {
                (inst, {
                    let element_id = inst.element_id.clone();
                    let part_id = inst.part_id.clone().expect("msg");
                    let part = part_map
                        .get(&(element_id, part_id))
                        .expect("Part not found");
                    *part
                })
            })
            .collect()
    }
}

#[derive(Debug, Deserialize)]
pub struct Assembly {
    #[serde(rename = "fullConfiguration")]
    pub full_configuration: String,
    pub instances: Vec<Instance>,
}

#[derive(Debug, Deserialize)]
pub struct SubAssembly {
    pub configuration: String,
    #[serde(rename = "fullConfiguration")]
    pub full_configuration: String,
    pub instances: Vec<Instance>,
}

/// A part or assembly
#[derive(Debug, Deserialize)]
pub struct Instance {
    pub id: String,
    pub name: String,

    #[serde(rename = "type")]
    pub instance_type: InstanceType,
    #[serde(rename = "isStandardContent", default)]
    pub is_standard_content: bool,
    #[serde(rename = "suppressed")]
    pub is_suppressed: bool,
    #[serde(rename = "elementId")]
    pub element_id: String,
    #[serde(rename = "partId")]
    pub part_id: Option<String>,
    #[serde(rename = "fullConfiguration")]
    pub full_configuration: String,
    pub configuration: String,
}

#[derive(Debug, Deserialize)]

pub struct Part {
    #[serde(rename = "bodyType")]
    pub body_type: String,
    #[serde(rename = "documentId")]
    pub document_id: String,
    #[serde(rename = "documentMicroversion")]
    pub document_microversion: String,
    #[serde(rename = "elementId")]
    pub element_id: String,
    #[serde(rename = "partId")]
    pub part_id: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub enum TabElementType {
    #[serde(rename = "APPLICATION")]
    Application,
    #[serde(rename = "ASSEMBLY")]
    Assembly,
    #[serde(rename = "BILLOFMATERIALS")]
    BillOfMaterials,
    #[serde(rename = "BLOB")]
    Blob,
    #[serde(rename = "DRAWING")]
    Drawing,
    #[serde(rename = "FEATURESTUDIO")]
    FeatureStudio,
    #[serde(rename = "PARTSTUDIO")]
    PartStudio,
    #[serde(rename = "PUBLICATIONITEM")]
    PublicationItem,
    #[serde(rename = "TABLE")]
    Table,
    #[serde(rename = "VARIABLESTUDIO")]
    VariableStudio,
    #[serde(rename = "UNKNOWN")]
    Unknown,
}

#[derive(Debug, Deserialize, PartialEq)]
pub enum InstanceType {
    Assembly,
    Feature,
    Part,
    Unknown,
}
