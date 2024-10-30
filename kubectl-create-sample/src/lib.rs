use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, CustomResource, Debug, Deserialize, JsonSchema, Serialize)]
#[kube(
    kind = "Sample",
    group = "sample.custom-controller",
    version = "v1alpha1",
    namespaced
)]
pub struct SampleSpec {
    pub name: String,
}
