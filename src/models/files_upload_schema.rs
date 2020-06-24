/*
 * Slack Web API
 *
 * One way to interact with the Slack platform is its HTTP RPC-based Web API, a collection of methods requiring OAuth 2.0-based user, bot, or workspace tokens blessed with related OAuth scopes.
 *
 * The version of the OpenAPI document: 1.5.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// FilesUploadSchema : Schema for successful response files.upload method



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FilesUploadSchema {
    #[serde(rename = "file")]
    pub file: crate::models::ObjsFile,
    #[serde(rename = "ok")]
    pub ok: crate::models::DefsOkTrue,
}

impl FilesUploadSchema {
    /// Schema for successful response files.upload method
    pub fn new(file: crate::models::ObjsFile, ok: crate::models::DefsOkTrue) -> FilesUploadSchema {
        FilesUploadSchema {
            file,
            ok,
        }
    }
}


