// use std::collections::{BTreeMap, HashMap};

// use serde::{Deserialize, Serialize};

// use crate::{
//     resources::{Resource, ResourceGraph, ResourceManager, ResourceManagerBackend},
//     state::AssetId,
// };

// mod resource_types {
//     pub const EXPERIENCE: &str = "experience";
//     pub const EXPERIENCE_THUMBNAIL: &str = "experience_thumbnail";
//     pub const EXPERIENCE_THUMBNAIL_ORDER: &str = "experience_thumbnail_order";
// }

// pub const SINGLETON_RESOURCE_ID: &str = "singleton";

// fn create_experience() -> Result<Resource, String> {
//     Ok(
//         Resource::new(resource_types::EXPERIENCE, SINGLETON_RESOURCE_ID)
//             .add_value_input(
//                 "configuration",
//                 &(HashMap::new() as HashMap<String, String>),
//             )?
//             .add_output("assetId", &(1234567890 as AssetId))?
//             .clone(),
//     )
// }

// fn create_thumbnail(file_path: &str) -> Result<Resource, String> {
//     Ok(
//         Resource::new(resource_types::EXPERIENCE_THUMBNAIL, file_path)
//             .add_ref_input(
//                 "experienceId",
//                 resource_types::EXPERIENCE,
//                 SINGLETON_RESOURCE_ID,
//                 "assetId",
//             )
//             .add_value_input("filePath", &file_path)?
//             .add_value_input("fileHash", &"ashdkjashkjdhaskjdhaskjdahs")?
//             .clone(),
//     )
// }

// fn create_thumbnail_order(file_paths: &Vec<&str>) -> Result<Resource, String> {
//     Ok(Resource::new(
//         resource_types::EXPERIENCE_THUMBNAIL_ORDER,
//         SINGLETON_RESOURCE_ID,
//     )
//     .add_ref_input(
//         "experienceId",
//         resource_types::EXPERIENCE,
//         SINGLETON_RESOURCE_ID,
//         "assetId",
//     )
//     .add_ref_input_list(
//         "assetIds",
//         &file_paths
//             .iter()
//             .map(|file_path| (resource_types::EXPERIENCE_THUMBNAIL, *file_path, "assetId"))
//             .collect(),
//     )
//     .clone())
// }

// #[derive(Serialize, Deserialize, Clone)]
// #[serde(rename_all = "camelCase")]
// struct ExperienceThumbnailInputs {
//     experience_id: AssetId,
//     file_path: String,
//     file_hash: String,
// }

// // #[derive(Serialize, Deserialize, Clone)]
// // #[serde(rename_all = "camelCase")]
// // struct ExperienceThumbnailOutputs {
// //     asset_id: AssetId,
// // }

// #[derive(Serialize, Deserialize, Clone)]
// #[serde(rename_all = "camelCase")]
// struct ExperienceThumbnailOrderInputs {
//     experience_id: AssetId,
//     asset_ids: Vec<AssetId>,
// }

// struct FakeResourceManager {}

// impl ResourceManagerBackend for FakeResourceManager {
//     fn create(
//         &self,
//         resource_type: &str,
//         resource_inputs: &BTreeMap<String, crate::resources::InputValue>,
//     ) -> Result<HashMap<String, crate::resources::OutputValue>, String> {
//         println!("CREATE: {} {:?}", resource_type, resource_inputs);
//         match resource_type {
//             resource_types::EXPERIENCE_THUMBNAIL => {
//                 // TODO: accept inputs in serde_yaml::Value format to simplify deserialization
//                 let inputs = serde_yaml::from_value::<ExperienceThumbnailInputs>(
//                     serde_yaml::to_value(resource_inputs)
//                         .map_err(|e| format!("Failed to serialize inputs: {}", e))?,
//                 )
//                 .map_err(|_| "Failed to deserialize inputs 1")?;

//                 // TODO: return outputs in serde_yaml::Value format to simplify serialization
//                 // let outputs = ExperienceThumbnailOutputs {
//                 //     asset_id: 1234567890,
//                 // };
//                 let mut outputs = HashMap::new();
//                 outputs.insert(
//                     "assetId".to_owned(),
//                     serde_yaml::to_value(&(1234567890 as AssetId))
//                         .map_err(|e| format!("Failed to serialize asset id\n\t{}", e))?,
//                 );
//                 Ok(outputs)
//             }
//             resource_types::EXPERIENCE_THUMBNAIL_ORDER => {
//                 // TODO: accept inputs in serde_yaml::Value format to simplify deserialization
//                 let inputs = serde_yaml::from_value::<ExperienceThumbnailOrderInputs>(
//                     serde_yaml::to_value(resource_inputs)
//                         .map_err(|e| format!("Failed to serialize inputs: {}", e))?,
//                 )
//                 .map_err(|_| "Failed to deserialize inputs")?;
//                 Ok(HashMap::new())
//             }
//             _ => panic!("Create not implemented for {}", resource_type),
//         }
//     }

//     fn update(
//         &self,
//         resource_type: &str,
//         resource_inputs: &BTreeMap<String, crate::resources::InputValue>,
//     ) -> Result<HashMap<String, crate::resources::OutputValue>, String> {
//         println!("UPDATE: {} {:?}", resource_type, resource_inputs);
//         match resource_type {
//             resource_types::EXPERIENCE_THUMBNAIL_ORDER => {
//                 // TODO: accept inputs in serde_yaml::Value format to simplify deserialization
//                 let inputs = serde_yaml::from_value::<ExperienceThumbnailOrderInputs>(
//                     serde_yaml::to_value(resource_inputs)
//                         .map_err(|e| format!("Failed to serialize inputs: {}", e))?,
//                 )
//                 .map_err(|_| "Failed to deserialize inputs")?;
//                 Ok(HashMap::new())
//             }
//             _ => panic!("Update not implemented for {}", resource_type),
//         }
//     }

//     fn delete(
//         &self,
//         resource_type: &str,
//         resource_outputs: &HashMap<String, crate::resources::OutputValue>,
//     ) -> Result<(), String> {
//         println!("DELETE: {} {:?}", resource_type, resource_outputs);
//         Ok(())
//     }
// }

pub fn run() -> Result<(), String> {
    // let resource_manager = ResourceManager::new(Box::new(FakeResourceManager {}));

    // let experience = create_experience()?;
    // let thumbnail_1 = create_thumbnail("thumbnail-1.png")?;
    // let thumbnail_2 = create_thumbnail("thumbnail-2.png")?;
    // let thumbnail_3 = create_thumbnail("thumbnail-3.png")?;
    // let thumbnail_order = create_thumbnail_order(&vec![
    //     "thumbnail-1.png",
    //     "thumbnail-2.png",
    //     "thumbnail-3.png",
    // ])?;

    // let mut desired_graph = ResourceGraph::new()
    //     .add_resource(&experience)
    //     .add_resource(&thumbnail_1)
    //     .add_resource(&thumbnail_2)
    //     .add_resource(&thumbnail_3)
    //     .add_resource(&thumbnail_order);

    // let previous_graph = ResourceGraph::new()
    //     .add_resource(&experience.clone())
    //     .add_resource(&create_thumbnail("thumbnail-4.png")?);
    // // .add_resource(
    // //     &thumbnail_2
    // //         .clone()
    // //         .add_output("assetId", &(2222 as AssetId))?
    // //         .clone(),
    // // )
    // // .add_resource(
    // //     &thumbnail_3
    // //         .clone()
    // //         .add_output("assetId", &(3333 as AssetId))?
    // //         .clone(),
    // // )
    // // .add_resource(&create_thumbnail_order(&vec![
    // //     "thumbnail-1.png",
    // //     "thumbnail-2.png",
    // //     "thumbnail-3.png",
    // // ])?);

    // println!("BEFORE\n{}", serde_yaml::to_string(&desired_graph).unwrap());
    // desired_graph.resolve(&resource_manager, &previous_graph)?;
    // println!(
    //     "\n\nAFTER\n{}",
    //     serde_yaml::to_string(&desired_graph).unwrap()
    // );

    Ok(())
}
