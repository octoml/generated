#![doc = "# Resources and Methods\n    * [projects](resources/projects/struct.ProjectsActions.html)\n      * [*explain*](resources/projects/struct.ExplainRequestBuilder.html), [*getConfig*](resources/projects/struct.GetConfigRequestBuilder.html), [*predict*](resources/projects/struct.PredictRequestBuilder.html)\n      * [jobs](resources/projects/jobs/struct.JobsActions.html)\n        * [*cancel*](resources/projects/jobs/struct.CancelRequestBuilder.html), [*create*](resources/projects/jobs/struct.CreateRequestBuilder.html), [*get*](resources/projects/jobs/struct.GetRequestBuilder.html), [*getIamPolicy*](resources/projects/jobs/struct.GetIamPolicyRequestBuilder.html), [*list*](resources/projects/jobs/struct.ListRequestBuilder.html), [*patch*](resources/projects/jobs/struct.PatchRequestBuilder.html), [*setIamPolicy*](resources/projects/jobs/struct.SetIamPolicyRequestBuilder.html), [*testIamPermissions*](resources/projects/jobs/struct.TestIamPermissionsRequestBuilder.html)\n      * [locations](resources/projects/locations/struct.LocationsActions.html)\n        * [*get*](resources/projects/locations/struct.GetRequestBuilder.html), [*list*](resources/projects/locations/struct.ListRequestBuilder.html)\n        * [operations](resources/projects/locations/operations/struct.OperationsActions.html)\n          * [*cancel*](resources/projects/locations/operations/struct.CancelRequestBuilder.html), [*get*](resources/projects/locations/operations/struct.GetRequestBuilder.html)\n        * [studies](resources/projects/locations/studies/struct.StudiesActions.html)\n          * [*create*](resources/projects/locations/studies/struct.CreateRequestBuilder.html), [*delete*](resources/projects/locations/studies/struct.DeleteRequestBuilder.html), [*get*](resources/projects/locations/studies/struct.GetRequestBuilder.html), [*list*](resources/projects/locations/studies/struct.ListRequestBuilder.html)\n          * [trials](resources/projects/locations/studies/trials/struct.TrialsActions.html)\n            * [*addMeasurement*](resources/projects/locations/studies/trials/struct.AddMeasurementRequestBuilder.html), [*checkEarlyStoppingState*](resources/projects/locations/studies/trials/struct.CheckEarlyStoppingStateRequestBuilder.html), [*complete*](resources/projects/locations/studies/trials/struct.CompleteRequestBuilder.html), [*create*](resources/projects/locations/studies/trials/struct.CreateRequestBuilder.html), [*delete*](resources/projects/locations/studies/trials/struct.DeleteRequestBuilder.html), [*get*](resources/projects/locations/studies/trials/struct.GetRequestBuilder.html), [*list*](resources/projects/locations/studies/trials/struct.ListRequestBuilder.html), [*stop*](resources/projects/locations/studies/trials/struct.StopRequestBuilder.html), [*suggest*](resources/projects/locations/studies/trials/struct.SuggestRequestBuilder.html)\n      * [models](resources/projects/models/struct.ModelsActions.html)\n        * [*create*](resources/projects/models/struct.CreateRequestBuilder.html), [*delete*](resources/projects/models/struct.DeleteRequestBuilder.html), [*get*](resources/projects/models/struct.GetRequestBuilder.html), [*getIamPolicy*](resources/projects/models/struct.GetIamPolicyRequestBuilder.html), [*list*](resources/projects/models/struct.ListRequestBuilder.html), [*patch*](resources/projects/models/struct.PatchRequestBuilder.html), [*setIamPolicy*](resources/projects/models/struct.SetIamPolicyRequestBuilder.html), [*testIamPermissions*](resources/projects/models/struct.TestIamPermissionsRequestBuilder.html)\n        * [versions](resources/projects/models/versions/struct.VersionsActions.html)\n          * [*create*](resources/projects/models/versions/struct.CreateRequestBuilder.html), [*delete*](resources/projects/models/versions/struct.DeleteRequestBuilder.html), [*get*](resources/projects/models/versions/struct.GetRequestBuilder.html), [*list*](resources/projects/models/versions/struct.ListRequestBuilder.html), [*patch*](resources/projects/models/versions/struct.PatchRequestBuilder.html), [*setDefault*](resources/projects/models/versions/struct.SetDefaultRequestBuilder.html)\n      * [operations](resources/projects/operations/struct.OperationsActions.html)\n        * [*cancel*](resources/projects/operations/struct.CancelRequestBuilder.html), [*get*](resources/projects/operations/struct.GetRequestBuilder.html), [*list*](resources/projects/operations/struct.ListRequestBuilder.html)\n"]
pub mod scopes {
    #[doc = "View and manage your data across Google Cloud Platform services\n\n`https://www.googleapis.com/auth/cloud-platform`"]
    pub const CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";
    #[doc = "View your data across Google Cloud Platform services\n\n`https://www.googleapis.com/auth/cloud-platform.read-only`"]
    pub const CLOUD_PLATFORM_READ_ONLY: &str =
        "https://www.googleapis.com/auth/cloud-platform.read-only";
}
pub mod schemas {
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleApiHttpBody {
        #[doc = "The HTTP Content-Type header value specifying the content type of the body."]
        #[serde(
            rename = "contentType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub content_type: ::std::option::Option<String>,
        #[doc = "The HTTP request/response body as raw binary."]
        #[serde(
            rename = "data",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub data: ::std::option::Option<::google_api_bytes::Bytes>,
        #[doc = "Application specific response metadata. Must be set in the first response\nfor streaming APIs."]
        #[serde(
            rename = "extensions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub extensions:
            ::std::option::Option<Vec<::std::collections::BTreeMap<String, ::serde_json::Value>>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleApiHttpBody {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleApiHttpBody {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudMlV1AcceleratorConfig {
        #[doc = "The number of accelerators to attach to each machine running the job."]
        #[serde(
            rename = "count",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub count: ::std::option::Option<i64>,
        #[doc = "The type of accelerator to use."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::GoogleCloudMlV1AcceleratorConfigType>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1AcceleratorConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1AcceleratorConfig {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudMlV1AcceleratorConfigType {
        #[doc = "Unspecified accelerator type. Default to no GPU."]
        AcceleratorTypeUnspecified,
        #[doc = "Nvidia Tesla K80 GPU."]
        NvidiaTeslaK80,
        #[doc = "Nvidia Tesla P100 GPU."]
        NvidiaTeslaP100,
        #[doc = "Nvidia Tesla P4 GPU."]
        NvidiaTeslaP4,
        #[doc = "Nvidia Tesla T4 GPU."]
        NvidiaTeslaT4,
        #[doc = "Nvidia Tesla V100 GPU."]
        NvidiaTeslaV100,
        #[doc = "TPU v2."]
        TpuV2,
        #[doc = "TPU v3."]
        TpuV3,
    }
    impl GoogleCloudMlV1AcceleratorConfigType {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudMlV1AcceleratorConfigType::AcceleratorTypeUnspecified => {
                    "ACCELERATOR_TYPE_UNSPECIFIED"
                }
                GoogleCloudMlV1AcceleratorConfigType::NvidiaTeslaK80 => "NVIDIA_TESLA_K80",
                GoogleCloudMlV1AcceleratorConfigType::NvidiaTeslaP100 => "NVIDIA_TESLA_P100",
                GoogleCloudMlV1AcceleratorConfigType::NvidiaTeslaP4 => "NVIDIA_TESLA_P4",
                GoogleCloudMlV1AcceleratorConfigType::NvidiaTeslaT4 => "NVIDIA_TESLA_T4",
                GoogleCloudMlV1AcceleratorConfigType::NvidiaTeslaV100 => "NVIDIA_TESLA_V100",
                GoogleCloudMlV1AcceleratorConfigType::TpuV2 => "TPU_V2",
                GoogleCloudMlV1AcceleratorConfigType::TpuV3 => "TPU_V3",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudMlV1AcceleratorConfigType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudMlV1AcceleratorConfigType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<GoogleCloudMlV1AcceleratorConfigType, ()> {
            Ok(match s {
                "ACCELERATOR_TYPE_UNSPECIFIED" => {
                    GoogleCloudMlV1AcceleratorConfigType::AcceleratorTypeUnspecified
                }
                "NVIDIA_TESLA_K80" => GoogleCloudMlV1AcceleratorConfigType::NvidiaTeslaK80,
                "NVIDIA_TESLA_P100" => GoogleCloudMlV1AcceleratorConfigType::NvidiaTeslaP100,
                "NVIDIA_TESLA_P4" => GoogleCloudMlV1AcceleratorConfigType::NvidiaTeslaP4,
                "NVIDIA_TESLA_T4" => GoogleCloudMlV1AcceleratorConfigType::NvidiaTeslaT4,
                "NVIDIA_TESLA_V100" => GoogleCloudMlV1AcceleratorConfigType::NvidiaTeslaV100,
                "TPU_V2" => GoogleCloudMlV1AcceleratorConfigType::TpuV2,
                "TPU_V3" => GoogleCloudMlV1AcceleratorConfigType::TpuV3,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleCloudMlV1AcceleratorConfigType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudMlV1AcceleratorConfigType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudMlV1AcceleratorConfigType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ACCELERATOR_TYPE_UNSPECIFIED" => {
                    GoogleCloudMlV1AcceleratorConfigType::AcceleratorTypeUnspecified
                }
                "NVIDIA_TESLA_K80" => GoogleCloudMlV1AcceleratorConfigType::NvidiaTeslaK80,
                "NVIDIA_TESLA_P100" => GoogleCloudMlV1AcceleratorConfigType::NvidiaTeslaP100,
                "NVIDIA_TESLA_P4" => GoogleCloudMlV1AcceleratorConfigType::NvidiaTeslaP4,
                "NVIDIA_TESLA_T4" => GoogleCloudMlV1AcceleratorConfigType::NvidiaTeslaT4,
                "NVIDIA_TESLA_V100" => GoogleCloudMlV1AcceleratorConfigType::NvidiaTeslaV100,
                "TPU_V2" => GoogleCloudMlV1AcceleratorConfigType::TpuV2,
                "TPU_V3" => GoogleCloudMlV1AcceleratorConfigType::TpuV3,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1AcceleratorConfigType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1AcceleratorConfigType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudMlV1AddTrialMeasurementRequest {
        #[doc = "Required. The measurement to be added to a trial."]
        #[serde(
            rename = "measurement",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub measurement: ::std::option::Option<crate::schemas::GoogleCloudMlV1Measurement>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1AddTrialMeasurementRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1AddTrialMeasurementRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudMlV1AutoScaling {
        #[doc = "Optional. The minimum number of nodes to allocate for this model. These\nnodes are always up, starting from the time the model is deployed.\nTherefore, the cost of operating this model will be at least\n`rate` * `min_nodes` * number of hours since last billing cycle,\nwhere `rate` is the cost per node-hour as documented in the\n[pricing guide](/ml-engine/docs/pricing),\neven if no predictions are performed. There is additional cost for each\nprediction performed.\n\nUnlike manual scaling, if the load gets too heavy for the nodes\nthat are up, the service will automatically add nodes to handle the\nincreased load as well as scale back as traffic drops, always maintaining\nat least `min_nodes`. You will be charged for the time in which additional\nnodes are used.\n\nIf `min_nodes` is not specified and AutoScaling is used with a [legacy\n(MLS1) machine type](/ml-engine/docs/machine-types-online-prediction),\n`min_nodes` defaults to 0, in which case, when traffic to a model stops\n(and after a cool-down period), nodes will be shut down and no charges will\nbe incurred until traffic to the model resumes.\n\nIf `min_nodes` is not specified and AutoScaling is used with a [Compute\nEngine (N1) machine type](/ml-engine/docs/machine-types-online-prediction),\n`min_nodes` defaults to 1. `min_nodes` must be at least 1 for use with a\nCompute Engine machine type.\n\nNote that you cannot use AutoScaling if your version uses\n[GPUs](#Version.FIELDS.accelerator_config). Instead, you must use\nManualScaling.\n\nYou can set `min_nodes` when creating the model version, and you can also\nupdate `min_nodes` for an existing version:\n\n<pre>\nupdate_body.json:\n{\n  'autoScaling': {\n    'minNodes': 5\n  }\n}\n</pre>\n\nHTTP request:\n\n<pre style=\"max-width: 626px;\">\nPATCH\nhttps://ml.googleapis.com/v1/{name=projects/*/models/*/versions/*}?update_mask=autoScaling.minNodes\n-d @./update_body.json\n</pre>"]
        #[serde(
            rename = "minNodes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub min_nodes: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1AutoScaling {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1AutoScaling {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudMlV1AutomatedStoppingConfig {
        #[serde(
            rename = "decayCurveStoppingConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub decay_curve_stopping_config: ::std::option::Option<
            crate::schemas::GoogleCloudMlV1AutomatedStoppingConfigDecayCurveAutomatedStoppingConfig,
        >,
        #[serde(
            rename = "medianAutomatedStoppingConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub median_automated_stopping_config: ::std::option::Option<
            crate::schemas::GoogleCloudMlV1AutomatedStoppingConfigMedianAutomatedStoppingConfig,
        >,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1AutomatedStoppingConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1AutomatedStoppingConfig {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudMlV1AutomatedStoppingConfigDecayCurveAutomatedStoppingConfig {
        #[doc = "If true, measurement.elapsed_time is used as the x-axis of each\nTrials Decay Curve. Otherwise, Measurement.steps will be used as the\nx-axis."]
        #[serde(
            rename = "useElapsedTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub use_elapsed_time: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudMlV1AutomatedStoppingConfigDecayCurveAutomatedStoppingConfig
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudMlV1AutomatedStoppingConfigDecayCurveAutomatedStoppingConfig
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudMlV1AutomatedStoppingConfigMedianAutomatedStoppingConfig {
        #[doc = "If true, the median automated stopping rule applies to\nmeasurement.use_elapsed_time, which means the elapsed_time field of\nthe current trial's\nlatest measurement is used to compute the median objective\nvalue for each completed trial."]
        #[serde(
            rename = "useElapsedTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub use_elapsed_time: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudMlV1AutomatedStoppingConfigMedianAutomatedStoppingConfig
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudMlV1AutomatedStoppingConfigMedianAutomatedStoppingConfig
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudMlV1BuiltInAlgorithmOutput {
        #[doc = "Framework on which the built-in algorithm was trained."]
        #[serde(
            rename = "framework",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub framework: ::std::option::Option<String>,
        #[doc = "The Cloud Storage path to the `model/` directory where the training job\nsaves the trained model. Only set for successful jobs that don't use\nhyperparameter tuning."]
        #[serde(
            rename = "modelPath",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub model_path: ::std::option::Option<String>,
        #[doc = "Python version on which the built-in algorithm was trained."]
        #[serde(
            rename = "pythonVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub python_version: ::std::option::Option<String>,
        #[doc = "AI Platform runtime version on which the built-in algorithm was\ntrained."]
        #[serde(
            rename = "runtimeVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub runtime_version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1BuiltInAlgorithmOutput {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1BuiltInAlgorithmOutput {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudMlV1CancelJobRequest {}
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1CancelJobRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1CancelJobRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudMlV1Capability {
        #[doc = "Available accelerators for the capability."]
        #[serde(
            rename = "availableAccelerators",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub available_accelerators: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudMlV1CapabilityAvailableAcceleratorsItems>,
        >,
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::GoogleCloudMlV1CapabilityType>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1Capability {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1Capability {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudMlV1CapabilityAvailableAcceleratorsItems {
        AcceleratorTypeUnspecified,
        NvidiaTeslaK80,
        NvidiaTeslaP100,
        NvidiaTeslaP4,
        NvidiaTeslaT4,
        NvidiaTeslaV100,
        TpuV2,
        TpuV3,
    }
    impl GoogleCloudMlV1CapabilityAvailableAcceleratorsItems {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudMlV1CapabilityAvailableAcceleratorsItems::AcceleratorTypeUnspecified => {
                    "ACCELERATOR_TYPE_UNSPECIFIED"
                }
                GoogleCloudMlV1CapabilityAvailableAcceleratorsItems::NvidiaTeslaK80 => {
                    "NVIDIA_TESLA_K80"
                }
                GoogleCloudMlV1CapabilityAvailableAcceleratorsItems::NvidiaTeslaP100 => {
                    "NVIDIA_TESLA_P100"
                }
                GoogleCloudMlV1CapabilityAvailableAcceleratorsItems::NvidiaTeslaP4 => {
                    "NVIDIA_TESLA_P4"
                }
                GoogleCloudMlV1CapabilityAvailableAcceleratorsItems::NvidiaTeslaT4 => {
                    "NVIDIA_TESLA_T4"
                }
                GoogleCloudMlV1CapabilityAvailableAcceleratorsItems::NvidiaTeslaV100 => {
                    "NVIDIA_TESLA_V100"
                }
                GoogleCloudMlV1CapabilityAvailableAcceleratorsItems::TpuV2 => "TPU_V2",
                GoogleCloudMlV1CapabilityAvailableAcceleratorsItems::TpuV3 => "TPU_V3",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudMlV1CapabilityAvailableAcceleratorsItems {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudMlV1CapabilityAvailableAcceleratorsItems {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleCloudMlV1CapabilityAvailableAcceleratorsItems, ()>
        {
            Ok(match s {
                "ACCELERATOR_TYPE_UNSPECIFIED" => {
                    GoogleCloudMlV1CapabilityAvailableAcceleratorsItems::AcceleratorTypeUnspecified
                }
                "NVIDIA_TESLA_K80" => {
                    GoogleCloudMlV1CapabilityAvailableAcceleratorsItems::NvidiaTeslaK80
                }
                "NVIDIA_TESLA_P100" => {
                    GoogleCloudMlV1CapabilityAvailableAcceleratorsItems::NvidiaTeslaP100
                }
                "NVIDIA_TESLA_P4" => {
                    GoogleCloudMlV1CapabilityAvailableAcceleratorsItems::NvidiaTeslaP4
                }
                "NVIDIA_TESLA_T4" => {
                    GoogleCloudMlV1CapabilityAvailableAcceleratorsItems::NvidiaTeslaT4
                }
                "NVIDIA_TESLA_V100" => {
                    GoogleCloudMlV1CapabilityAvailableAcceleratorsItems::NvidiaTeslaV100
                }
                "TPU_V2" => GoogleCloudMlV1CapabilityAvailableAcceleratorsItems::TpuV2,
                "TPU_V3" => GoogleCloudMlV1CapabilityAvailableAcceleratorsItems::TpuV3,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleCloudMlV1CapabilityAvailableAcceleratorsItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudMlV1CapabilityAvailableAcceleratorsItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudMlV1CapabilityAvailableAcceleratorsItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ACCELERATOR_TYPE_UNSPECIFIED" => {
                    GoogleCloudMlV1CapabilityAvailableAcceleratorsItems::AcceleratorTypeUnspecified
                }
                "NVIDIA_TESLA_K80" => {
                    GoogleCloudMlV1CapabilityAvailableAcceleratorsItems::NvidiaTeslaK80
                }
                "NVIDIA_TESLA_P100" => {
                    GoogleCloudMlV1CapabilityAvailableAcceleratorsItems::NvidiaTeslaP100
                }
                "NVIDIA_TESLA_P4" => {
                    GoogleCloudMlV1CapabilityAvailableAcceleratorsItems::NvidiaTeslaP4
                }
                "NVIDIA_TESLA_T4" => {
                    GoogleCloudMlV1CapabilityAvailableAcceleratorsItems::NvidiaTeslaT4
                }
                "NVIDIA_TESLA_V100" => {
                    GoogleCloudMlV1CapabilityAvailableAcceleratorsItems::NvidiaTeslaV100
                }
                "TPU_V2" => GoogleCloudMlV1CapabilityAvailableAcceleratorsItems::TpuV2,
                "TPU_V3" => GoogleCloudMlV1CapabilityAvailableAcceleratorsItems::TpuV3,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudMlV1CapabilityAvailableAcceleratorsItems
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1CapabilityAvailableAcceleratorsItems {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudMlV1CapabilityType {
        BatchPrediction,
        OnlinePrediction,
        Training,
        TypeUnspecified,
    }
    impl GoogleCloudMlV1CapabilityType {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudMlV1CapabilityType::BatchPrediction => "BATCH_PREDICTION",
                GoogleCloudMlV1CapabilityType::OnlinePrediction => "ONLINE_PREDICTION",
                GoogleCloudMlV1CapabilityType::Training => "TRAINING",
                GoogleCloudMlV1CapabilityType::TypeUnspecified => "TYPE_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudMlV1CapabilityType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudMlV1CapabilityType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<GoogleCloudMlV1CapabilityType, ()> {
            Ok(match s {
                "BATCH_PREDICTION" => GoogleCloudMlV1CapabilityType::BatchPrediction,
                "ONLINE_PREDICTION" => GoogleCloudMlV1CapabilityType::OnlinePrediction,
                "TRAINING" => GoogleCloudMlV1CapabilityType::Training,
                "TYPE_UNSPECIFIED" => GoogleCloudMlV1CapabilityType::TypeUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleCloudMlV1CapabilityType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudMlV1CapabilityType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudMlV1CapabilityType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BATCH_PREDICTION" => GoogleCloudMlV1CapabilityType::BatchPrediction,
                "ONLINE_PREDICTION" => GoogleCloudMlV1CapabilityType::OnlinePrediction,
                "TRAINING" => GoogleCloudMlV1CapabilityType::Training,
                "TYPE_UNSPECIFIED" => GoogleCloudMlV1CapabilityType::TypeUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1CapabilityType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1CapabilityType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudMlV1CheckTrialEarlyStoppingStateMetatdata {
        #[doc = "The time at which the operation was submitted."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "The name of the study that the trial belongs to."]
        #[serde(
            rename = "study",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub study: ::std::option::Option<String>,
        #[doc = "The trial name."]
        #[serde(
            rename = "trial",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub trial: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudMlV1CheckTrialEarlyStoppingStateMetatdata
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1CheckTrialEarlyStoppingStateMetatdata {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudMlV1CheckTrialEarlyStoppingStateRequest {}
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1CheckTrialEarlyStoppingStateRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1CheckTrialEarlyStoppingStateRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudMlV1CheckTrialEarlyStoppingStateResponse {
        #[doc = "The time at which operation processing completed."]
        #[serde(
            rename = "endTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end_time: ::std::option::Option<String>,
        #[doc = "True if the Trial should stop."]
        #[serde(
            rename = "shouldStop",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub should_stop: ::std::option::Option<bool>,
        #[doc = "The time at which the operation was started."]
        #[serde(
            rename = "startTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_time: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudMlV1CheckTrialEarlyStoppingStateResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1CheckTrialEarlyStoppingStateResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudMlV1CompleteTrialRequest {
        #[doc = "Optional. If provided, it will be used as the completed trial's\nfinal_measurement; Otherwise, the service will auto-select a\npreviously reported measurement as the final-measurement"]
        #[serde(
            rename = "finalMeasurement",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub final_measurement: ::std::option::Option<crate::schemas::GoogleCloudMlV1Measurement>,
        #[doc = "Optional. A human readable reason why the trial was infeasible. This should\nonly be provided if `trial_infeasible` is true."]
        #[serde(
            rename = "infeasibleReason",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub infeasible_reason: ::std::option::Option<String>,
        #[doc = "Optional. True if the trial cannot be run with the given Parameter, and\nfinal_measurement will be ignored."]
        #[serde(
            rename = "trialInfeasible",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub trial_infeasible: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1CompleteTrialRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1CompleteTrialRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudMlV1Config {
        #[doc = "The service account Cloud ML uses to run on TPU node."]
        #[serde(
            rename = "tpuServiceAccount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub tpu_service_account: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1Config {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1Config {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudMlV1EncryptionConfig {
        #[doc = "The Cloud KMS resource identifier of the customer-managed encryption key\nused to protect a resource, such as a training job. It has the following\nformat:\n`projects/{PROJECT_ID}/locations/{REGION}/keyRings/{KEY_RING_NAME}/cryptoKeys/{KEY_NAME}`"]
        #[serde(
            rename = "kmsKeyName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kms_key_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1EncryptionConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1EncryptionConfig {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudMlV1ExplainRequest {
        #[doc = "Required.\nThe explanation request body."]
        #[serde(
            rename = "httpBody",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub http_body: ::std::option::Option<crate::schemas::GoogleApiHttpBody>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1ExplainRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1ExplainRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudMlV1ExplanationConfig {
        #[doc = "Attributes credit by computing the Aumann-Shapley value taking advantage\nof the model's fully differentiable structure. Refer to this paper for\nmore details: http://proceedings.mlr.press/v70/sundararajan17a.html"]
        #[serde(
            rename = "integratedGradientsAttribution",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub integrated_gradients_attribution:
            ::std::option::Option<crate::schemas::GoogleCloudMlV1IntegratedGradientsAttribution>,
        #[doc = "An attribution method that approximates Shapley values for features that\ncontribute to the label being predicted. A sampling strategy is used to\napproximate the value rather than considering all subsets of features."]
        #[serde(
            rename = "sampledShapleyAttribution",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sampled_shapley_attribution:
            ::std::option::Option<crate::schemas::GoogleCloudMlV1SampledShapleyAttribution>,
        #[doc = "Attributes credit by computing the XRAI taking advantage\nof the model's fully differentiable structure. Refer to this paper for\nmore details: https://arxiv.org/abs/1906.02825\nCurrently only implemented for models with natural image inputs."]
        #[serde(
            rename = "xraiAttribution",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub xrai_attribution: ::std::option::Option<crate::schemas::GoogleCloudMlV1XraiAttribution>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1ExplanationConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1ExplanationConfig {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudMlV1GetConfigResponse {
        #[serde(
            rename = "config",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub config: ::std::option::Option<crate::schemas::GoogleCloudMlV1Config>,
        #[doc = "The service account Cloud ML uses to access resources in the project."]
        #[serde(
            rename = "serviceAccount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub service_account: ::std::option::Option<String>,
        #[doc = "The project number for `service_account`."]
        #[serde(
            rename = "serviceAccountProject",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub service_account_project: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1GetConfigResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1GetConfigResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudMlV1HyperparameterOutput {
        #[doc = "All recorded object metrics for this trial. This field is not currently\npopulated."]
        #[serde(
            rename = "allMetrics",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub all_metrics: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudMlV1HyperparameterOutputHyperparameterMetric>,
        >,
        #[doc = "Details related to built-in algorithms jobs.\nOnly set for trials of built-in algorithms jobs that have succeeded."]
        #[serde(
            rename = "builtInAlgorithmOutput",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub built_in_algorithm_output:
            ::std::option::Option<crate::schemas::GoogleCloudMlV1BuiltInAlgorithmOutput>,
        #[doc = "Output only. End time for the trial."]
        #[serde(
            rename = "endTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end_time: ::std::option::Option<String>,
        #[doc = "The final objective metric seen for this trial."]
        #[serde(
            rename = "finalMetric",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub final_metric: ::std::option::Option<
            crate::schemas::GoogleCloudMlV1HyperparameterOutputHyperparameterMetric,
        >,
        #[doc = "The hyperparameters given to this trial."]
        #[serde(
            rename = "hyperparameters",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub hyperparameters: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "True if the trial is stopped early."]
        #[serde(
            rename = "isTrialStoppedEarly",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub is_trial_stopped_early: ::std::option::Option<bool>,
        #[doc = "Output only. Start time for the trial."]
        #[serde(
            rename = "startTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_time: ::std::option::Option<String>,
        #[doc = "Output only. The detailed state of the trial."]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state: ::std::option::Option<crate::schemas::GoogleCloudMlV1HyperparameterOutputState>,
        #[doc = "The trial id for these results."]
        #[serde(
            rename = "trialId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub trial_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1HyperparameterOutput {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1HyperparameterOutput {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudMlV1HyperparameterOutputState {
        #[doc = "The job has been cancelled.\n`error_message` should describe the reason for the cancellation."]
        Cancelled,
        #[doc = "The job is being cancelled.\n`error_message` should describe the reason for the cancellation."]
        Cancelling,
        #[doc = "The job failed.\n`error_message` should contain the details of the failure."]
        Failed,
        #[doc = "The service is preparing to run the job."]
        Preparing,
        #[doc = "The job has been just created and processing has not yet begun."]
        Queued,
        #[doc = "The job is in progress."]
        Running,
        #[doc = "The job state is unspecified."]
        StateUnspecified,
        #[doc = "The job completed successfully."]
        Succeeded,
    }
    impl GoogleCloudMlV1HyperparameterOutputState {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudMlV1HyperparameterOutputState::Cancelled => "CANCELLED",
                GoogleCloudMlV1HyperparameterOutputState::Cancelling => "CANCELLING",
                GoogleCloudMlV1HyperparameterOutputState::Failed => "FAILED",
                GoogleCloudMlV1HyperparameterOutputState::Preparing => "PREPARING",
                GoogleCloudMlV1HyperparameterOutputState::Queued => "QUEUED",
                GoogleCloudMlV1HyperparameterOutputState::Running => "RUNNING",
                GoogleCloudMlV1HyperparameterOutputState::StateUnspecified => "STATE_UNSPECIFIED",
                GoogleCloudMlV1HyperparameterOutputState::Succeeded => "SUCCEEDED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudMlV1HyperparameterOutputState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudMlV1HyperparameterOutputState {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleCloudMlV1HyperparameterOutputState, ()> {
            Ok(match s {
                "CANCELLED" => GoogleCloudMlV1HyperparameterOutputState::Cancelled,
                "CANCELLING" => GoogleCloudMlV1HyperparameterOutputState::Cancelling,
                "FAILED" => GoogleCloudMlV1HyperparameterOutputState::Failed,
                "PREPARING" => GoogleCloudMlV1HyperparameterOutputState::Preparing,
                "QUEUED" => GoogleCloudMlV1HyperparameterOutputState::Queued,
                "RUNNING" => GoogleCloudMlV1HyperparameterOutputState::Running,
                "STATE_UNSPECIFIED" => GoogleCloudMlV1HyperparameterOutputState::StateUnspecified,
                "SUCCEEDED" => GoogleCloudMlV1HyperparameterOutputState::Succeeded,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleCloudMlV1HyperparameterOutputState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudMlV1HyperparameterOutputState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudMlV1HyperparameterOutputState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CANCELLED" => GoogleCloudMlV1HyperparameterOutputState::Cancelled,
                "CANCELLING" => GoogleCloudMlV1HyperparameterOutputState::Cancelling,
                "FAILED" => GoogleCloudMlV1HyperparameterOutputState::Failed,
                "PREPARING" => GoogleCloudMlV1HyperparameterOutputState::Preparing,
                "QUEUED" => GoogleCloudMlV1HyperparameterOutputState::Queued,
                "RUNNING" => GoogleCloudMlV1HyperparameterOutputState::Running,
                "STATE_UNSPECIFIED" => GoogleCloudMlV1HyperparameterOutputState::StateUnspecified,
                "SUCCEEDED" => GoogleCloudMlV1HyperparameterOutputState::Succeeded,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1HyperparameterOutputState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1HyperparameterOutputState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudMlV1HyperparameterOutputHyperparameterMetric {
        #[doc = "The objective value at this training step."]
        #[serde(
            rename = "objectiveValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub objective_value: ::std::option::Option<f64>,
        #[doc = "The global training step for this metric."]
        #[serde(
            rename = "trainingStep",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub training_step: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudMlV1HyperparameterOutputHyperparameterMetric
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudMlV1HyperparameterOutputHyperparameterMetric
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudMlV1HyperparameterSpec {
        #[doc = "Optional. The search algorithm specified for the hyperparameter\ntuning job.\nUses the default AI Platform hyperparameter tuning\nalgorithm if unspecified."]
        #[serde(
            rename = "algorithm",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub algorithm:
            ::std::option::Option<crate::schemas::GoogleCloudMlV1HyperparameterSpecAlgorithm>,
        #[doc = "Optional. Indicates if the hyperparameter tuning job enables auto trial\nearly stopping."]
        #[serde(
            rename = "enableTrialEarlyStopping",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub enable_trial_early_stopping: ::std::option::Option<bool>,
        #[doc = "Required. The type of goal to use for tuning. Available types are\n`MAXIMIZE` and `MINIMIZE`.\n\nDefaults to `MAXIMIZE`."]
        #[serde(
            rename = "goal",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub goal: ::std::option::Option<crate::schemas::GoogleCloudMlV1HyperparameterSpecGoal>,
        #[doc = "Optional. The TensorFlow summary tag name to use for optimizing trials. For\ncurrent versions of TensorFlow, this tag name should exactly match what is\nshown in TensorBoard, including all scopes.  For versions of TensorFlow\nprior to 0.12, this should be only the tag passed to tf.Summary.\nBy default, \"training/hptuning/metric\" will be used."]
        #[serde(
            rename = "hyperparameterMetricTag",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub hyperparameter_metric_tag: ::std::option::Option<String>,
        #[doc = "Optional. The number of failed trials that need to be seen before failing\nthe hyperparameter tuning job. You can specify this field to override the\ndefault failing criteria for AI Platform hyperparameter tuning jobs.\n\nDefaults to zero, which means the service decides when a hyperparameter\njob should fail."]
        #[serde(
            rename = "maxFailedTrials",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub max_failed_trials: ::std::option::Option<i32>,
        #[doc = "Optional. The number of training trials to run concurrently.\nYou can reduce the time it takes to perform hyperparameter tuning by adding\ntrials in parallel. However, each trail only benefits from the information\ngained in completed trials. That means that a trial does not get access to\nthe results of trials running at the same time, which could reduce the\nquality of the overall optimization.\n\nEach trial will use the same scale tier and machine types.\n\nDefaults to one."]
        #[serde(
            rename = "maxParallelTrials",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub max_parallel_trials: ::std::option::Option<i32>,
        #[doc = "Optional. How many training trials should be attempted to optimize\nthe specified hyperparameters.\n\nDefaults to one."]
        #[serde(
            rename = "maxTrials",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub max_trials: ::std::option::Option<i32>,
        #[doc = "Required. The set of parameters to tune."]
        #[serde(
            rename = "params",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub params: ::std::option::Option<Vec<crate::schemas::GoogleCloudMlV1ParameterSpec>>,
        #[doc = "Optional. The prior hyperparameter tuning job id that users hope to\ncontinue with. The job id will be used to find the corresponding vizier\nstudy guid and resume the study."]
        #[serde(
            rename = "resumePreviousJobId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resume_previous_job_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1HyperparameterSpec {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1HyperparameterSpec {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudMlV1HyperparameterSpecAlgorithm {
        #[doc = "The default algorithm used by the hyperparameter tuning service. This is\na Bayesian optimization algorithm."]
        AlgorithmUnspecified,
        #[doc = "Simple grid search within the feasible space. To use grid search,\nall parameters must be `INTEGER`, `CATEGORICAL`, or `DISCRETE`."]
        GridSearch,
        #[doc = "Simple random search within the feasible space."]
        RandomSearch,
    }
    impl GoogleCloudMlV1HyperparameterSpecAlgorithm {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudMlV1HyperparameterSpecAlgorithm::AlgorithmUnspecified => {
                    "ALGORITHM_UNSPECIFIED"
                }
                GoogleCloudMlV1HyperparameterSpecAlgorithm::GridSearch => "GRID_SEARCH",
                GoogleCloudMlV1HyperparameterSpecAlgorithm::RandomSearch => "RANDOM_SEARCH",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudMlV1HyperparameterSpecAlgorithm {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudMlV1HyperparameterSpecAlgorithm {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleCloudMlV1HyperparameterSpecAlgorithm, ()> {
            Ok(match s {
                "ALGORITHM_UNSPECIFIED" => {
                    GoogleCloudMlV1HyperparameterSpecAlgorithm::AlgorithmUnspecified
                }
                "GRID_SEARCH" => GoogleCloudMlV1HyperparameterSpecAlgorithm::GridSearch,
                "RANDOM_SEARCH" => GoogleCloudMlV1HyperparameterSpecAlgorithm::RandomSearch,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleCloudMlV1HyperparameterSpecAlgorithm {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudMlV1HyperparameterSpecAlgorithm {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudMlV1HyperparameterSpecAlgorithm {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ALGORITHM_UNSPECIFIED" => {
                    GoogleCloudMlV1HyperparameterSpecAlgorithm::AlgorithmUnspecified
                }
                "GRID_SEARCH" => GoogleCloudMlV1HyperparameterSpecAlgorithm::GridSearch,
                "RANDOM_SEARCH" => GoogleCloudMlV1HyperparameterSpecAlgorithm::RandomSearch,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1HyperparameterSpecAlgorithm {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1HyperparameterSpecAlgorithm {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudMlV1HyperparameterSpecGoal {
        #[doc = "Goal Type will default to maximize."]
        GoalTypeUnspecified,
        #[doc = "Maximize the goal metric."]
        Maximize,
        #[doc = "Minimize the goal metric."]
        Minimize,
    }
    impl GoogleCloudMlV1HyperparameterSpecGoal {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudMlV1HyperparameterSpecGoal::GoalTypeUnspecified => {
                    "GOAL_TYPE_UNSPECIFIED"
                }
                GoogleCloudMlV1HyperparameterSpecGoal::Maximize => "MAXIMIZE",
                GoogleCloudMlV1HyperparameterSpecGoal::Minimize => "MINIMIZE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudMlV1HyperparameterSpecGoal {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudMlV1HyperparameterSpecGoal {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<GoogleCloudMlV1HyperparameterSpecGoal, ()> {
            Ok(match s {
                "GOAL_TYPE_UNSPECIFIED" => {
                    GoogleCloudMlV1HyperparameterSpecGoal::GoalTypeUnspecified
                }
                "MAXIMIZE" => GoogleCloudMlV1HyperparameterSpecGoal::Maximize,
                "MINIMIZE" => GoogleCloudMlV1HyperparameterSpecGoal::Minimize,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleCloudMlV1HyperparameterSpecGoal {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudMlV1HyperparameterSpecGoal {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudMlV1HyperparameterSpecGoal {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "GOAL_TYPE_UNSPECIFIED" => {
                    GoogleCloudMlV1HyperparameterSpecGoal::GoalTypeUnspecified
                }
                "MAXIMIZE" => GoogleCloudMlV1HyperparameterSpecGoal::Maximize,
                "MINIMIZE" => GoogleCloudMlV1HyperparameterSpecGoal::Minimize,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1HyperparameterSpecGoal {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1HyperparameterSpecGoal {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudMlV1IntegratedGradientsAttribution {
        #[doc = "Number of steps for approximating the path integral.\nA good value to start is 50 and gradually increase until the\nsum to diff property is met within the desired error range."]
        #[serde(
            rename = "numIntegralSteps",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub num_integral_steps: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1IntegratedGradientsAttribution {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1IntegratedGradientsAttribution {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudMlV1Job {
        #[doc = "Output only. When the job was created."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "Output only. When the job processing was completed."]
        #[serde(
            rename = "endTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end_time: ::std::option::Option<String>,
        #[doc = "Output only. The details of a failure or a cancellation."]
        #[serde(
            rename = "errorMessage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub error_message: ::std::option::Option<String>,
        #[doc = "`etag` is used for optimistic concurrency control as a way to help\nprevent simultaneous updates of a job from overwriting each other.\nIt is strongly suggested that systems make use of the `etag` in the\nread-modify-write cycle to perform job updates in order to avoid race\nconditions: An `etag` is returned in the response to `GetJob`, and\nsystems are expected to put that etag in the request to `UpdateJob` to\nensure that their change will be applied to the same version of the job."]
        #[serde(
            rename = "etag",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub etag: ::std::option::Option<::google_api_bytes::Bytes>,
        #[doc = "Required. The user-specified id of the job."]
        #[serde(
            rename = "jobId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub job_id: ::std::option::Option<String>,
        #[doc = "Optional. One or more labels that you can add, to organize your jobs.\nEach label is a key-value pair, where both the key and the value are\narbitrary strings that you supply.\nFor more information, see the documentation on\n<a href=\"/ml-engine/docs/tensorflow/resource-labels\">using labels</a>."]
        #[serde(
            rename = "labels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub labels: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "Input parameters to create a prediction job."]
        #[serde(
            rename = "predictionInput",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub prediction_input: ::std::option::Option<crate::schemas::GoogleCloudMlV1PredictionInput>,
        #[doc = "The current prediction job result."]
        #[serde(
            rename = "predictionOutput",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub prediction_output:
            ::std::option::Option<crate::schemas::GoogleCloudMlV1PredictionOutput>,
        #[doc = "Output only. When the job processing was started."]
        #[serde(
            rename = "startTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_time: ::std::option::Option<String>,
        #[doc = "Output only. The detailed state of a job."]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state: ::std::option::Option<crate::schemas::GoogleCloudMlV1JobState>,
        #[doc = "Input parameters to create a training job."]
        #[serde(
            rename = "trainingInput",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub training_input: ::std::option::Option<crate::schemas::GoogleCloudMlV1TrainingInput>,
        #[doc = "The current training job result."]
        #[serde(
            rename = "trainingOutput",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub training_output: ::std::option::Option<crate::schemas::GoogleCloudMlV1TrainingOutput>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1Job {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1Job {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudMlV1JobState {
        #[doc = "The job has been cancelled.\n`error_message` should describe the reason for the cancellation."]
        Cancelled,
        #[doc = "The job is being cancelled.\n`error_message` should describe the reason for the cancellation."]
        Cancelling,
        #[doc = "The job failed.\n`error_message` should contain the details of the failure."]
        Failed,
        #[doc = "The service is preparing to run the job."]
        Preparing,
        #[doc = "The job has been just created and processing has not yet begun."]
        Queued,
        #[doc = "The job is in progress."]
        Running,
        #[doc = "The job state is unspecified."]
        StateUnspecified,
        #[doc = "The job completed successfully."]
        Succeeded,
    }
    impl GoogleCloudMlV1JobState {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudMlV1JobState::Cancelled => "CANCELLED",
                GoogleCloudMlV1JobState::Cancelling => "CANCELLING",
                GoogleCloudMlV1JobState::Failed => "FAILED",
                GoogleCloudMlV1JobState::Preparing => "PREPARING",
                GoogleCloudMlV1JobState::Queued => "QUEUED",
                GoogleCloudMlV1JobState::Running => "RUNNING",
                GoogleCloudMlV1JobState::StateUnspecified => "STATE_UNSPECIFIED",
                GoogleCloudMlV1JobState::Succeeded => "SUCCEEDED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudMlV1JobState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudMlV1JobState {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<GoogleCloudMlV1JobState, ()> {
            Ok(match s {
                "CANCELLED" => GoogleCloudMlV1JobState::Cancelled,
                "CANCELLING" => GoogleCloudMlV1JobState::Cancelling,
                "FAILED" => GoogleCloudMlV1JobState::Failed,
                "PREPARING" => GoogleCloudMlV1JobState::Preparing,
                "QUEUED" => GoogleCloudMlV1JobState::Queued,
                "RUNNING" => GoogleCloudMlV1JobState::Running,
                "STATE_UNSPECIFIED" => GoogleCloudMlV1JobState::StateUnspecified,
                "SUCCEEDED" => GoogleCloudMlV1JobState::Succeeded,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleCloudMlV1JobState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudMlV1JobState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudMlV1JobState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CANCELLED" => GoogleCloudMlV1JobState::Cancelled,
                "CANCELLING" => GoogleCloudMlV1JobState::Cancelling,
                "FAILED" => GoogleCloudMlV1JobState::Failed,
                "PREPARING" => GoogleCloudMlV1JobState::Preparing,
                "QUEUED" => GoogleCloudMlV1JobState::Queued,
                "RUNNING" => GoogleCloudMlV1JobState::Running,
                "STATE_UNSPECIFIED" => GoogleCloudMlV1JobState::StateUnspecified,
                "SUCCEEDED" => GoogleCloudMlV1JobState::Succeeded,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1JobState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1JobState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudMlV1ListJobsResponse {
        #[doc = "The list of jobs."]
        #[serde(
            rename = "jobs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub jobs: ::std::option::Option<Vec<crate::schemas::GoogleCloudMlV1Job>>,
        #[doc = "Optional. Pass this token as the `page_token` field of the request for a\nsubsequent call."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1ListJobsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1ListJobsResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudMlV1ListLocationsResponse {
        #[doc = "Locations where at least one type of CMLE capability is available."]
        #[serde(
            rename = "locations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub locations: ::std::option::Option<Vec<crate::schemas::GoogleCloudMlV1Location>>,
        #[doc = "Optional. Pass this token as the `page_token` field of the request for a\nsubsequent call."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1ListLocationsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1ListLocationsResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudMlV1ListModelsResponse {
        #[doc = "The list of models."]
        #[serde(
            rename = "models",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub models: ::std::option::Option<Vec<crate::schemas::GoogleCloudMlV1Model>>,
        #[doc = "Optional. Pass this token as the `page_token` field of the request for a\nsubsequent call."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1ListModelsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1ListModelsResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudMlV1ListStudiesResponse {
        #[doc = "The studies associated with the project."]
        #[serde(
            rename = "studies",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub studies: ::std::option::Option<Vec<crate::schemas::GoogleCloudMlV1Study>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1ListStudiesResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1ListStudiesResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudMlV1ListTrialsResponse {
        #[doc = "The trials associated with the study."]
        #[serde(
            rename = "trials",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub trials: ::std::option::Option<Vec<crate::schemas::GoogleCloudMlV1Trial>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1ListTrialsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1ListTrialsResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudMlV1ListVersionsResponse {
        #[doc = "Optional. Pass this token as the `page_token` field of the request for a\nsubsequent call."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "The list of versions."]
        #[serde(
            rename = "versions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub versions: ::std::option::Option<Vec<crate::schemas::GoogleCloudMlV1Version>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1ListVersionsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1ListVersionsResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudMlV1Location {
        #[doc = "Capabilities available in the location."]
        #[serde(
            rename = "capabilities",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub capabilities: ::std::option::Option<Vec<crate::schemas::GoogleCloudMlV1Capability>>,
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1Location {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1Location {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudMlV1ManualScaling {
        #[doc = "The number of nodes to allocate for this model. These nodes are always up,\nstarting from the time the model is deployed, so the cost of operating\nthis model will be proportional to `nodes` * number of hours since\nlast billing cycle plus the cost for each prediction performed."]
        #[serde(
            rename = "nodes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub nodes: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1ManualScaling {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1ManualScaling {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudMlV1Measurement {
        #[doc = "Output only. Time that the trial has been running at the point of\nthis measurement."]
        #[serde(
            rename = "elapsedTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub elapsed_time: ::std::option::Option<String>,
        #[doc = "Provides a list of metrics that act as inputs into the objective\nfunction."]
        #[serde(
            rename = "metrics",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metrics: ::std::option::Option<Vec<crate::schemas::GoogleCloudMlV1MeasurementMetric>>,
        #[doc = "The number of steps a machine learning model has been trained for.\nMust be non-negative."]
        #[serde(
            rename = "stepCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub step_count: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1Measurement {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1Measurement {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudMlV1MeasurementMetric {
        #[doc = "Required. Metric name."]
        #[serde(
            rename = "metric",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metric: ::std::option::Option<String>,
        #[doc = "Required. The value for this metric."]
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value: ::std::option::Option<f64>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1MeasurementMetric {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1MeasurementMetric {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudMlV1Model {
        #[doc = "Output only. The default version of the model. This version will be used to\nhandle prediction requests that do not specify a version.\n\nYou can change the default version by calling\nprojects.models.versions.setDefault."]
        #[serde(
            rename = "defaultVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub default_version: ::std::option::Option<crate::schemas::GoogleCloudMlV1Version>,
        #[doc = "Optional. The description specified for the model when it was created."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "`etag` is used for optimistic concurrency control as a way to help\nprevent simultaneous updates of a model from overwriting each other.\nIt is strongly suggested that systems make use of the `etag` in the\nread-modify-write cycle to perform model updates in order to avoid race\nconditions: An `etag` is returned in the response to `GetModel`, and\nsystems are expected to put that etag in the request to `UpdateModel` to\nensure that their change will be applied to the model as intended."]
        #[serde(
            rename = "etag",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub etag: ::std::option::Option<::google_api_bytes::Bytes>,
        #[doc = "Optional. One or more labels that you can add, to organize your models.\nEach label is a key-value pair, where both the key and the value are\narbitrary strings that you supply.\nFor more information, see the documentation on\n<a href=\"/ml-engine/docs/tensorflow/resource-labels\">using labels</a>."]
        #[serde(
            rename = "labels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub labels: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "Required. The name specified for the model when it was created.\n\nThe model name must be unique within the project it is created in."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Optional. If true, online prediction nodes send `stderr` and `stdout`\nstreams to Stackdriver Logging. These can be more verbose than the standard\naccess logs (see `onlinePredictionLogging`) and can incur higher cost.\nHowever, they are helpful for debugging. Note that\n[Stackdriver logs may incur a cost](/stackdriver/pricing), especially if\nyour project receives prediction requests at a high QPS. Estimate your\ncosts before enabling this option.\n\nDefault is false."]
        #[serde(
            rename = "onlinePredictionConsoleLogging",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub online_prediction_console_logging: ::std::option::Option<bool>,
        #[doc = "Optional. If true, online prediction access logs are sent to StackDriver\nLogging. These logs are like standard server access logs, containing\ninformation like timestamp and latency for each request. Note that\n[Stackdriver logs may incur a cost](/stackdriver/pricing), especially if\nyour project receives prediction requests at a high queries per second rate\n(QPS). Estimate your costs before enabling this option.\n\nDefault is false."]
        #[serde(
            rename = "onlinePredictionLogging",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub online_prediction_logging: ::std::option::Option<bool>,
        #[doc = "Optional. The list of regions where the model is going to be deployed.\nOnly one region per model is supported.\nDefaults to 'us-central1' if nothing is set.\nSee the <a href=\"/ml-engine/docs/tensorflow/regions\">available regions</a>\nfor AI Platform services.\nNote:\n\n* No matter where a model is deployed, it can always be accessed by\n  users from anywhere, both for online and batch prediction.\n* The region for a batch prediction job is set by the region field when\n  submitting the batch prediction job and does not take its value from\n  this field."]
        #[serde(
            rename = "regions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub regions: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1Model {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1Model {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudMlV1OperationMetadata {
        #[doc = "The time the operation was submitted."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "The time operation processing completed."]
        #[serde(
            rename = "endTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end_time: ::std::option::Option<String>,
        #[doc = "Indicates whether a request to cancel this operation has been made."]
        #[serde(
            rename = "isCancellationRequested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub is_cancellation_requested: ::std::option::Option<bool>,
        #[doc = "The user labels, inherited from the model or the model version being\noperated on."]
        #[serde(
            rename = "labels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub labels: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "Contains the name of the model associated with the operation."]
        #[serde(
            rename = "modelName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub model_name: ::std::option::Option<String>,
        #[doc = "The operation type."]
        #[serde(
            rename = "operationType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub operation_type:
            ::std::option::Option<crate::schemas::GoogleCloudMlV1OperationMetadataOperationType>,
        #[doc = "Contains the project number associated with the operation."]
        #[serde(
            rename = "projectNumber",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub project_number: ::std::option::Option<i64>,
        #[doc = "The time operation processing started."]
        #[serde(
            rename = "startTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_time: ::std::option::Option<String>,
        #[doc = "Contains the version associated with the operation."]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<crate::schemas::GoogleCloudMlV1Version>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1OperationMetadata {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1OperationMetadata {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudMlV1OperationMetadataOperationType {
        #[doc = "An operation to create a new version."]
        CreateVersion,
        #[doc = "An operation to delete an existing model."]
        DeleteModel,
        #[doc = "An operation to delete an existing version."]
        DeleteVersion,
        #[doc = "Unspecified operation type."]
        OperationTypeUnspecified,
        #[doc = "An operation to update project configuration."]
        UpdateConfig,
        #[doc = "An operation to update an existing model."]
        UpdateModel,
        #[doc = "An operation to update an existing version."]
        UpdateVersion,
    }
    impl GoogleCloudMlV1OperationMetadataOperationType {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudMlV1OperationMetadataOperationType::CreateVersion => "CREATE_VERSION",
                GoogleCloudMlV1OperationMetadataOperationType::DeleteModel => "DELETE_MODEL",
                GoogleCloudMlV1OperationMetadataOperationType::DeleteVersion => "DELETE_VERSION",
                GoogleCloudMlV1OperationMetadataOperationType::OperationTypeUnspecified => {
                    "OPERATION_TYPE_UNSPECIFIED"
                }
                GoogleCloudMlV1OperationMetadataOperationType::UpdateConfig => "UPDATE_CONFIG",
                GoogleCloudMlV1OperationMetadataOperationType::UpdateModel => "UPDATE_MODEL",
                GoogleCloudMlV1OperationMetadataOperationType::UpdateVersion => "UPDATE_VERSION",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudMlV1OperationMetadataOperationType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudMlV1OperationMetadataOperationType {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleCloudMlV1OperationMetadataOperationType, ()> {
            Ok(match s {
                "CREATE_VERSION" => GoogleCloudMlV1OperationMetadataOperationType::CreateVersion,
                "DELETE_MODEL" => GoogleCloudMlV1OperationMetadataOperationType::DeleteModel,
                "DELETE_VERSION" => GoogleCloudMlV1OperationMetadataOperationType::DeleteVersion,
                "OPERATION_TYPE_UNSPECIFIED" => {
                    GoogleCloudMlV1OperationMetadataOperationType::OperationTypeUnspecified
                }
                "UPDATE_CONFIG" => GoogleCloudMlV1OperationMetadataOperationType::UpdateConfig,
                "UPDATE_MODEL" => GoogleCloudMlV1OperationMetadataOperationType::UpdateModel,
                "UPDATE_VERSION" => GoogleCloudMlV1OperationMetadataOperationType::UpdateVersion,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleCloudMlV1OperationMetadataOperationType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudMlV1OperationMetadataOperationType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudMlV1OperationMetadataOperationType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CREATE_VERSION" => GoogleCloudMlV1OperationMetadataOperationType::CreateVersion,
                "DELETE_MODEL" => GoogleCloudMlV1OperationMetadataOperationType::DeleteModel,
                "DELETE_VERSION" => GoogleCloudMlV1OperationMetadataOperationType::DeleteVersion,
                "OPERATION_TYPE_UNSPECIFIED" => {
                    GoogleCloudMlV1OperationMetadataOperationType::OperationTypeUnspecified
                }
                "UPDATE_CONFIG" => GoogleCloudMlV1OperationMetadataOperationType::UpdateConfig,
                "UPDATE_MODEL" => GoogleCloudMlV1OperationMetadataOperationType::UpdateModel,
                "UPDATE_VERSION" => GoogleCloudMlV1OperationMetadataOperationType::UpdateVersion,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1OperationMetadataOperationType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1OperationMetadataOperationType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudMlV1ParameterSpec {
        #[doc = "Required if type is `CATEGORICAL`. The list of possible categories."]
        #[serde(
            rename = "categoricalValues",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub categorical_values: ::std::option::Option<Vec<String>>,
        #[doc = "Required if type is `DISCRETE`.\nA list of feasible points.\nThe list should be in strictly increasing order. For instance, this\nparameter might have possible settings of 1.5, 2.5, and 4.0. This list\nshould not contain more than 1,000 values."]
        #[serde(
            rename = "discreteValues",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub discrete_values: ::std::option::Option<Vec<f64>>,
        #[doc = "Required if type is `DOUBLE` or `INTEGER`. This field\nshould be unset if type is `CATEGORICAL`. This value should be integers if\ntype is `INTEGER`."]
        #[serde(
            rename = "maxValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub max_value: ::std::option::Option<f64>,
        #[doc = "Required if type is `DOUBLE` or `INTEGER`. This field\nshould be unset if type is `CATEGORICAL`. This value should be integers if\ntype is INTEGER."]
        #[serde(
            rename = "minValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub min_value: ::std::option::Option<f64>,
        #[doc = "Required. The parameter name must be unique amongst all ParameterConfigs in\na HyperparameterSpec message. E.g., \"learning_rate\"."]
        #[serde(
            rename = "parameterName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub parameter_name: ::std::option::Option<String>,
        #[doc = "Required. The type of the parameter."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::GoogleCloudMlV1ParameterSpecType>,
        #[doc = "Optional. How the parameter should be scaled to the hypercube.\nLeave unset for categorical parameters.\nSome kind of scaling is strongly recommended for real or integral\nparameters (e.g., `UNIT_LINEAR_SCALE`)."]
        #[serde(
            rename = "scaleType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub scale_type:
            ::std::option::Option<crate::schemas::GoogleCloudMlV1ParameterSpecScaleType>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1ParameterSpec {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1ParameterSpec {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudMlV1ParameterSpecType {
        #[doc = "The parameter is categorical, with a value chosen from the categories\nfield."]
        Categorical,
        #[doc = "The parameter is real valued, with a fixed set of feasible points. If\n`type==DISCRETE`, feasible_points must be provided, and\n{`min_value`, `max_value`} will be ignored."]
        Discrete,
        #[doc = "Type for real-valued parameters."]
        Double,
        #[doc = "Type for integral parameters."]
        Integer,
        #[doc = "You must specify a valid type. Using this unspecified type will result in\nan error."]
        ParameterTypeUnspecified,
    }
    impl GoogleCloudMlV1ParameterSpecType {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudMlV1ParameterSpecType::Categorical => "CATEGORICAL",
                GoogleCloudMlV1ParameterSpecType::Discrete => "DISCRETE",
                GoogleCloudMlV1ParameterSpecType::Double => "DOUBLE",
                GoogleCloudMlV1ParameterSpecType::Integer => "INTEGER",
                GoogleCloudMlV1ParameterSpecType::ParameterTypeUnspecified => {
                    "PARAMETER_TYPE_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudMlV1ParameterSpecType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudMlV1ParameterSpecType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<GoogleCloudMlV1ParameterSpecType, ()> {
            Ok(match s {
                "CATEGORICAL" => GoogleCloudMlV1ParameterSpecType::Categorical,
                "DISCRETE" => GoogleCloudMlV1ParameterSpecType::Discrete,
                "DOUBLE" => GoogleCloudMlV1ParameterSpecType::Double,
                "INTEGER" => GoogleCloudMlV1ParameterSpecType::Integer,
                "PARAMETER_TYPE_UNSPECIFIED" => {
                    GoogleCloudMlV1ParameterSpecType::ParameterTypeUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleCloudMlV1ParameterSpecType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudMlV1ParameterSpecType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudMlV1ParameterSpecType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CATEGORICAL" => GoogleCloudMlV1ParameterSpecType::Categorical,
                "DISCRETE" => GoogleCloudMlV1ParameterSpecType::Discrete,
                "DOUBLE" => GoogleCloudMlV1ParameterSpecType::Double,
                "INTEGER" => GoogleCloudMlV1ParameterSpecType::Integer,
                "PARAMETER_TYPE_UNSPECIFIED" => {
                    GoogleCloudMlV1ParameterSpecType::ParameterTypeUnspecified
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1ParameterSpecType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1ParameterSpecType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudMlV1ParameterSpecScaleType {
        #[doc = "By default, no scaling is applied."]
        None,
        #[doc = "Scales the feasible space to (0, 1) linearly."]
        UnitLinearScale,
        #[doc = "Scales the feasible space logarithmically to (0, 1). The entire feasible\nspace must be strictly positive."]
        UnitLogScale,
        #[doc = "Scales the feasible space \"reverse\" logarithmically to (0, 1). The result\nis that values close to the top of the feasible space are spread out more\nthan points near the bottom. The entire feasible space must be strictly\npositive."]
        UnitReverseLogScale,
    }
    impl GoogleCloudMlV1ParameterSpecScaleType {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudMlV1ParameterSpecScaleType::None => "NONE",
                GoogleCloudMlV1ParameterSpecScaleType::UnitLinearScale => "UNIT_LINEAR_SCALE",
                GoogleCloudMlV1ParameterSpecScaleType::UnitLogScale => "UNIT_LOG_SCALE",
                GoogleCloudMlV1ParameterSpecScaleType::UnitReverseLogScale => {
                    "UNIT_REVERSE_LOG_SCALE"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudMlV1ParameterSpecScaleType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudMlV1ParameterSpecScaleType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<GoogleCloudMlV1ParameterSpecScaleType, ()> {
            Ok(match s {
                "NONE" => GoogleCloudMlV1ParameterSpecScaleType::None,
                "UNIT_LINEAR_SCALE" => GoogleCloudMlV1ParameterSpecScaleType::UnitLinearScale,
                "UNIT_LOG_SCALE" => GoogleCloudMlV1ParameterSpecScaleType::UnitLogScale,
                "UNIT_REVERSE_LOG_SCALE" => {
                    GoogleCloudMlV1ParameterSpecScaleType::UnitReverseLogScale
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleCloudMlV1ParameterSpecScaleType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudMlV1ParameterSpecScaleType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudMlV1ParameterSpecScaleType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "NONE" => GoogleCloudMlV1ParameterSpecScaleType::None,
                "UNIT_LINEAR_SCALE" => GoogleCloudMlV1ParameterSpecScaleType::UnitLinearScale,
                "UNIT_LOG_SCALE" => GoogleCloudMlV1ParameterSpecScaleType::UnitLogScale,
                "UNIT_REVERSE_LOG_SCALE" => {
                    GoogleCloudMlV1ParameterSpecScaleType::UnitReverseLogScale
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1ParameterSpecScaleType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1ParameterSpecScaleType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudMlV1PredictRequest {
        #[doc = "Required. The prediction request body. Refer to the [request body details\nsection](#request-body-details) for more information on how to structure\nyour request."]
        #[serde(
            rename = "httpBody",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub http_body: ::std::option::Option<crate::schemas::GoogleApiHttpBody>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1PredictRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1PredictRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudMlV1PredictionInput {
        #[doc = "Optional. Number of records per batch, defaults to 64.\nThe service will buffer batch_size number of records in memory before\ninvoking one Tensorflow prediction call internally. So take the record\nsize and memory available into consideration when setting this parameter."]
        #[serde(
            rename = "batchSize",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub batch_size: ::std::option::Option<i64>,
        #[doc = "Required. The format of the input data files."]
        #[serde(
            rename = "dataFormat",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub data_format:
            ::std::option::Option<crate::schemas::GoogleCloudMlV1PredictionInputDataFormat>,
        #[doc = "Required. The Cloud Storage location of the input data files. May contain\n<a href=\"/storage/docs/gsutil/addlhelp/WildcardNames\">wildcards</a>."]
        #[serde(
            rename = "inputPaths",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub input_paths: ::std::option::Option<Vec<String>>,
        #[doc = "Optional. The maximum number of workers to be used for parallel processing.\nDefaults to 10 if not specified."]
        #[serde(
            rename = "maxWorkerCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub max_worker_count: ::std::option::Option<i64>,
        #[doc = "Use this field if you want to use the default version for the specified\nmodel. The string must use the following format:\n\n`\"projects/YOUR_PROJECT/models/YOUR_MODEL\"`"]
        #[serde(
            rename = "modelName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub model_name: ::std::option::Option<String>,
        #[doc = "Optional. Format of the output data files, defaults to JSON."]
        #[serde(
            rename = "outputDataFormat",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub output_data_format:
            ::std::option::Option<crate::schemas::GoogleCloudMlV1PredictionInputOutputDataFormat>,
        #[doc = "Required. The output Google Cloud Storage location."]
        #[serde(
            rename = "outputPath",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub output_path: ::std::option::Option<String>,
        #[doc = "Required. The Google Compute Engine region to run the prediction job in.\nSee the <a href=\"/ml-engine/docs/tensorflow/regions\">available regions</a>\nfor AI Platform services."]
        #[serde(
            rename = "region",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub region: ::std::option::Option<String>,
        #[doc = "Optional. The AI Platform runtime version to use for this batch\nprediction. If not set, AI Platform will pick the runtime version used\nduring the CreateVersion request for this model version, or choose the\nlatest stable version when model version information is not available\nsuch as when the model is specified by uri."]
        #[serde(
            rename = "runtimeVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub runtime_version: ::std::option::Option<String>,
        #[doc = "Optional. The name of the signature defined in the SavedModel to use for\nthis job. Please refer to\n[SavedModel](https://tensorflow.github.io/serving/serving_basic.html)\nfor information about how to use signatures.\n\nDefaults to\n[DEFAULT_SERVING_SIGNATURE_DEF_KEY](https://www.tensorflow.org/api_docs/python/tf/saved_model/signature_constants)\n, which is \"serving_default\"."]
        #[serde(
            rename = "signatureName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub signature_name: ::std::option::Option<String>,
        #[doc = "Use this field if you want to specify a Google Cloud Storage path for\nthe model to use."]
        #[serde(
            rename = "uri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub uri: ::std::option::Option<String>,
        #[doc = "Use this field if you want to specify a version of the model to use. The\nstring is formatted the same way as `model_version`, with the addition\nof the version information:\n\n`\"projects/YOUR_PROJECT/models/YOUR_MODEL/versions/YOUR_VERSION\"`"]
        #[serde(
            rename = "versionName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1PredictionInput {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1PredictionInput {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudMlV1PredictionInputDataFormat {
        #[doc = "Values are comma-separated rows, with keys in a separate file.\nCurrently available only for output data."]
        Csv,
        #[doc = "Unspecified format."]
        DataFormatUnspecified,
        #[doc = "Each line of the file is a JSON dictionary representing one record."]
        Json,
        #[doc = "Deprecated. Use JSON instead."]
        Text,
        #[doc = "The source file is a TFRecord file.\nCurrently available only for input data."]
        TfRecord,
        #[doc = "The source file is a GZIP-compressed TFRecord file.\nCurrently available only for input data."]
        TfRecordGzip,
    }
    impl GoogleCloudMlV1PredictionInputDataFormat {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudMlV1PredictionInputDataFormat::Csv => "CSV",
                GoogleCloudMlV1PredictionInputDataFormat::DataFormatUnspecified => {
                    "DATA_FORMAT_UNSPECIFIED"
                }
                GoogleCloudMlV1PredictionInputDataFormat::Json => "JSON",
                GoogleCloudMlV1PredictionInputDataFormat::Text => "TEXT",
                GoogleCloudMlV1PredictionInputDataFormat::TfRecord => "TF_RECORD",
                GoogleCloudMlV1PredictionInputDataFormat::TfRecordGzip => "TF_RECORD_GZIP",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudMlV1PredictionInputDataFormat {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudMlV1PredictionInputDataFormat {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleCloudMlV1PredictionInputDataFormat, ()> {
            Ok(match s {
                "CSV" => GoogleCloudMlV1PredictionInputDataFormat::Csv,
                "DATA_FORMAT_UNSPECIFIED" => {
                    GoogleCloudMlV1PredictionInputDataFormat::DataFormatUnspecified
                }
                "JSON" => GoogleCloudMlV1PredictionInputDataFormat::Json,
                "TEXT" => GoogleCloudMlV1PredictionInputDataFormat::Text,
                "TF_RECORD" => GoogleCloudMlV1PredictionInputDataFormat::TfRecord,
                "TF_RECORD_GZIP" => GoogleCloudMlV1PredictionInputDataFormat::TfRecordGzip,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleCloudMlV1PredictionInputDataFormat {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudMlV1PredictionInputDataFormat {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudMlV1PredictionInputDataFormat {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CSV" => GoogleCloudMlV1PredictionInputDataFormat::Csv,
                "DATA_FORMAT_UNSPECIFIED" => {
                    GoogleCloudMlV1PredictionInputDataFormat::DataFormatUnspecified
                }
                "JSON" => GoogleCloudMlV1PredictionInputDataFormat::Json,
                "TEXT" => GoogleCloudMlV1PredictionInputDataFormat::Text,
                "TF_RECORD" => GoogleCloudMlV1PredictionInputDataFormat::TfRecord,
                "TF_RECORD_GZIP" => GoogleCloudMlV1PredictionInputDataFormat::TfRecordGzip,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1PredictionInputDataFormat {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1PredictionInputDataFormat {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudMlV1PredictionInputOutputDataFormat {
        #[doc = "Values are comma-separated rows, with keys in a separate file.\nCurrently available only for output data."]
        Csv,
        #[doc = "Unspecified format."]
        DataFormatUnspecified,
        #[doc = "Each line of the file is a JSON dictionary representing one record."]
        Json,
        #[doc = "Deprecated. Use JSON instead."]
        Text,
        #[doc = "The source file is a TFRecord file.\nCurrently available only for input data."]
        TfRecord,
        #[doc = "The source file is a GZIP-compressed TFRecord file.\nCurrently available only for input data."]
        TfRecordGzip,
    }
    impl GoogleCloudMlV1PredictionInputOutputDataFormat {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudMlV1PredictionInputOutputDataFormat::Csv => "CSV",
                GoogleCloudMlV1PredictionInputOutputDataFormat::DataFormatUnspecified => {
                    "DATA_FORMAT_UNSPECIFIED"
                }
                GoogleCloudMlV1PredictionInputOutputDataFormat::Json => "JSON",
                GoogleCloudMlV1PredictionInputOutputDataFormat::Text => "TEXT",
                GoogleCloudMlV1PredictionInputOutputDataFormat::TfRecord => "TF_RECORD",
                GoogleCloudMlV1PredictionInputOutputDataFormat::TfRecordGzip => "TF_RECORD_GZIP",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudMlV1PredictionInputOutputDataFormat {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudMlV1PredictionInputOutputDataFormat {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleCloudMlV1PredictionInputOutputDataFormat, ()> {
            Ok(match s {
                "CSV" => GoogleCloudMlV1PredictionInputOutputDataFormat::Csv,
                "DATA_FORMAT_UNSPECIFIED" => {
                    GoogleCloudMlV1PredictionInputOutputDataFormat::DataFormatUnspecified
                }
                "JSON" => GoogleCloudMlV1PredictionInputOutputDataFormat::Json,
                "TEXT" => GoogleCloudMlV1PredictionInputOutputDataFormat::Text,
                "TF_RECORD" => GoogleCloudMlV1PredictionInputOutputDataFormat::TfRecord,
                "TF_RECORD_GZIP" => GoogleCloudMlV1PredictionInputOutputDataFormat::TfRecordGzip,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleCloudMlV1PredictionInputOutputDataFormat {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudMlV1PredictionInputOutputDataFormat {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudMlV1PredictionInputOutputDataFormat {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CSV" => GoogleCloudMlV1PredictionInputOutputDataFormat::Csv,
                "DATA_FORMAT_UNSPECIFIED" => {
                    GoogleCloudMlV1PredictionInputOutputDataFormat::DataFormatUnspecified
                }
                "JSON" => GoogleCloudMlV1PredictionInputOutputDataFormat::Json,
                "TEXT" => GoogleCloudMlV1PredictionInputOutputDataFormat::Text,
                "TF_RECORD" => GoogleCloudMlV1PredictionInputOutputDataFormat::TfRecord,
                "TF_RECORD_GZIP" => GoogleCloudMlV1PredictionInputOutputDataFormat::TfRecordGzip,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1PredictionInputOutputDataFormat {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1PredictionInputOutputDataFormat {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudMlV1PredictionOutput {
        #[doc = "The number of data instances which resulted in errors."]
        #[serde(
            rename = "errorCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub error_count: ::std::option::Option<i64>,
        #[doc = "Node hours used by the batch prediction job."]
        #[serde(
            rename = "nodeHours",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub node_hours: ::std::option::Option<f64>,
        #[doc = "The output Google Cloud Storage location provided at the job creation time."]
        #[serde(
            rename = "outputPath",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub output_path: ::std::option::Option<String>,
        #[doc = "The number of generated predictions."]
        #[serde(
            rename = "predictionCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub prediction_count: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1PredictionOutput {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1PredictionOutput {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudMlV1ReplicaConfig {
        #[doc = "Represents the type and number of accelerators used by the replica.\n[Learn about restrictions on accelerator configurations for\ntraining.](/ai-platform/training/docs/using-gpus#compute-engine-machine-types-with-gpu)"]
        #[serde(
            rename = "acceleratorConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub accelerator_config:
            ::std::option::Option<crate::schemas::GoogleCloudMlV1AcceleratorConfig>,
        #[doc = "Arguments to the entrypoint command.\nThe following rules apply for container_command and container_args:\n\n* If you do not supply command or args:\n  The defaults defined in the Docker image are used.\n* If you supply a command but no args:\n  The default EntryPoint and the default Cmd defined in the Docker image\n  are ignored. Your command is run without any arguments.\n* If you supply only args:\n  The default Entrypoint defined in the Docker image is run with the args\n  that you supplied.\n* If you supply a command and args:\n  The default Entrypoint and the default Cmd defined in the Docker image\n  are ignored. Your command is run with your args.\n  It cannot be set if custom container image is\n  not provided.\n  Note that this field and [TrainingInput.args] are mutually exclusive, i.e.,\n  both cannot be set at the same time."]
        #[serde(
            rename = "containerArgs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub container_args: ::std::option::Option<Vec<String>>,
        #[doc = "The command with which the replica's custom container is run.\nIf provided, it will override default ENTRYPOINT of the docker image.\nIf not provided, the docker image's ENTRYPOINT is used.\nIt cannot be set if custom container image is\nnot provided.\nNote that this field and [TrainingInput.args] are mutually exclusive, i.e.,\nboth cannot be set at the same time."]
        #[serde(
            rename = "containerCommand",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub container_command: ::std::option::Option<Vec<String>>,
        #[doc = "The Docker image to run on the replica. This image must be in Container\nRegistry. Learn more about [configuring custom\ncontainers](/ai-platform/training/docs/distributed-training-containers)."]
        #[serde(
            rename = "imageUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub image_uri: ::std::option::Option<String>,
        #[doc = "The AI Platform runtime version that includes a TensorFlow version matching\nthe one used in the custom container. This field is required if the replica\nis a TPU worker that uses a custom container. Otherwise, do not specify\nthis field. This must be a [runtime version that currently supports\ntraining with\nTPUs](/ml-engine/docs/tensorflow/runtime-version-list#tpu-support).\n\nNote that the version of TensorFlow included in a runtime version may\ndiffer from the numbering of the runtime version itself, because it may\nhave a different [patch\nversion](https://www.tensorflow.org/guide/version_compat#semantic_versioning_20).\nIn this field, you must specify the runtime version (TensorFlow minor\nversion). For example, if your custom container runs TensorFlow `1.x.y`,\nspecify `1.x`."]
        #[serde(
            rename = "tpuTfVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub tpu_tf_version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1ReplicaConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1ReplicaConfig {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudMlV1RequestLoggingConfig {
        #[doc = "Required. Fully qualified BigQuery table name in the following format:\n\"<var>project_id</var>.<var>dataset_name</var>.<var>table_name</var>\"\n\nThe specified table must already exist, and the \"Cloud ML Service Agent\"\nfor your project must have permission to write to it. The table must have\nthe following [schema](/bigquery/docs/schemas):\n\n<table>\n  <tr><th>Field name</th><th style=\"display: table-cell\">Type</th>\n    <th style=\"display: table-cell\">Mode</th></tr>\n  <tr><td>model</td><td>STRING</td><td>REQUIRED</td></tr>\n  <tr><td>model_version</td><td>STRING</td><td>REQUIRED</td></tr>\n  <tr><td>time</td><td>TIMESTAMP</td><td>REQUIRED</td></tr>\n  <tr><td>raw_data</td><td>STRING</td><td>REQUIRED</td></tr>\n  <tr><td>raw_prediction</td><td>STRING</td><td>NULLABLE</td></tr>\n  <tr><td>groundtruth</td><td>STRING</td><td>NULLABLE</td></tr>\n</table>"]
        #[serde(
            rename = "bigqueryTableName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bigquery_table_name: ::std::option::Option<String>,
        #[doc = "Percentage of requests to be logged, expressed as a fraction from 0 to 1.\nFor example, if you want to log 10% of requests, enter `0.1`. The sampling\nwindow is the lifetime of the model version. Defaults to 0."]
        #[serde(
            rename = "samplingPercentage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sampling_percentage: ::std::option::Option<f64>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1RequestLoggingConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1RequestLoggingConfig {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudMlV1SampledShapleyAttribution {
        #[doc = "The number of feature permutations to consider when approximating the\nShapley values."]
        #[serde(
            rename = "numPaths",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub num_paths: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1SampledShapleyAttribution {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1SampledShapleyAttribution {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudMlV1Scheduling {
        #[doc = "Optional. The maximum job running time, expressed in seconds. The field can\ncontain up to nine fractional digits, terminated by `s`. If not specified,\nthis field defaults to `604800s` (seven days).\n\nIf the training job is still running after this duration, AI Platform\nTraining cancels it.\n\nFor example, if you want to ensure your job runs for no more than 2 hours,\nset this field to `7200s` (2 hours * 60 minutes / hour * 60 seconds /\nminute).\n\nIf you submit your training job using the `gcloud` tool, you can [provide\nthis field in a `config.yaml`\nfile](/ai-platform/training/docs/training-jobs#formatting_your_configuration_parameters).\nFor example:\n\n````textyaml\ntrainingInput:\n  ...\n  scheduling:\n    maxRunningTime: 7200s\n  ...\n````"]
        #[serde(
            rename = "maxRunningTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub max_running_time: ::std::option::Option<String>,
        #[serde(
            rename = "maxWaitTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub max_wait_time: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1Scheduling {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1Scheduling {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudMlV1SetDefaultVersionRequest {}
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1SetDefaultVersionRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1SetDefaultVersionRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudMlV1StopTrialRequest {}
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1StopTrialRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1StopTrialRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudMlV1Study {
        #[doc = "Output only. Time at which the study was created."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "Output only. A human readable reason why the Study is inactive.\nThis should be empty if a study is ACTIVE or COMPLETED."]
        #[serde(
            rename = "inactiveReason",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub inactive_reason: ::std::option::Option<String>,
        #[doc = "Output only. The name of a study."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Output only. The detailed state of a study."]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state: ::std::option::Option<crate::schemas::GoogleCloudMlV1StudyState>,
        #[doc = "Required. Configuration of the study."]
        #[serde(
            rename = "studyConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub study_config: ::std::option::Option<crate::schemas::GoogleCloudMlV1StudyConfig>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1Study {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1Study {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudMlV1StudyState {
        #[doc = "The study is active."]
        Active,
        #[doc = "The study is done when the service exhausts the parameter search space\nor max_trial_count is reached."]
        Completed,
        #[doc = "The study is stopped due to an internal error."]
        Inactive,
        #[doc = "The study state is unspecified."]
        StateUnspecified,
    }
    impl GoogleCloudMlV1StudyState {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudMlV1StudyState::Active => "ACTIVE",
                GoogleCloudMlV1StudyState::Completed => "COMPLETED",
                GoogleCloudMlV1StudyState::Inactive => "INACTIVE",
                GoogleCloudMlV1StudyState::StateUnspecified => "STATE_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudMlV1StudyState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudMlV1StudyState {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<GoogleCloudMlV1StudyState, ()> {
            Ok(match s {
                "ACTIVE" => GoogleCloudMlV1StudyState::Active,
                "COMPLETED" => GoogleCloudMlV1StudyState::Completed,
                "INACTIVE" => GoogleCloudMlV1StudyState::Inactive,
                "STATE_UNSPECIFIED" => GoogleCloudMlV1StudyState::StateUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleCloudMlV1StudyState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudMlV1StudyState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudMlV1StudyState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ACTIVE" => GoogleCloudMlV1StudyState::Active,
                "COMPLETED" => GoogleCloudMlV1StudyState::Completed,
                "INACTIVE" => GoogleCloudMlV1StudyState::Inactive,
                "STATE_UNSPECIFIED" => GoogleCloudMlV1StudyState::StateUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1StudyState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1StudyState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudMlV1StudyConfig {
        #[doc = "The search algorithm specified for the study."]
        #[serde(
            rename = "algorithm",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub algorithm: ::std::option::Option<crate::schemas::GoogleCloudMlV1StudyConfigAlgorithm>,
        #[doc = "Configuration for automated stopping of unpromising Trials."]
        #[serde(
            rename = "automatedStoppingConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub automated_stopping_config:
            ::std::option::Option<crate::schemas::GoogleCloudMlV1AutomatedStoppingConfig>,
        #[doc = "Metric specs for the study."]
        #[serde(
            rename = "metrics",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metrics:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudMlV1StudyConfigMetricSpec>>,
        #[doc = "Required. The set of parameters to tune."]
        #[serde(
            rename = "parameters",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub parameters:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudMlV1StudyConfigParameterSpec>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1StudyConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1StudyConfig {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudMlV1StudyConfigAlgorithm {
        #[doc = "The default algorithm used by the Cloud AI Platform Optimization service."]
        AlgorithmUnspecified,
        #[doc = "Gaussian Process Bandit."]
        GaussianProcessBandit,
        #[doc = "Simple grid search within the feasible space. To use grid search,\nall parameters must be `INTEGER`, `CATEGORICAL`, or `DISCRETE`."]
        GridSearch,
        #[doc = "Simple random search within the feasible space."]
        RandomSearch,
    }
    impl GoogleCloudMlV1StudyConfigAlgorithm {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudMlV1StudyConfigAlgorithm::AlgorithmUnspecified => {
                    "ALGORITHM_UNSPECIFIED"
                }
                GoogleCloudMlV1StudyConfigAlgorithm::GaussianProcessBandit => {
                    "GAUSSIAN_PROCESS_BANDIT"
                }
                GoogleCloudMlV1StudyConfigAlgorithm::GridSearch => "GRID_SEARCH",
                GoogleCloudMlV1StudyConfigAlgorithm::RandomSearch => "RANDOM_SEARCH",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudMlV1StudyConfigAlgorithm {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudMlV1StudyConfigAlgorithm {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<GoogleCloudMlV1StudyConfigAlgorithm, ()> {
            Ok(match s {
                "ALGORITHM_UNSPECIFIED" => {
                    GoogleCloudMlV1StudyConfigAlgorithm::AlgorithmUnspecified
                }
                "GAUSSIAN_PROCESS_BANDIT" => {
                    GoogleCloudMlV1StudyConfigAlgorithm::GaussianProcessBandit
                }
                "GRID_SEARCH" => GoogleCloudMlV1StudyConfigAlgorithm::GridSearch,
                "RANDOM_SEARCH" => GoogleCloudMlV1StudyConfigAlgorithm::RandomSearch,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleCloudMlV1StudyConfigAlgorithm {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudMlV1StudyConfigAlgorithm {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudMlV1StudyConfigAlgorithm {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ALGORITHM_UNSPECIFIED" => {
                    GoogleCloudMlV1StudyConfigAlgorithm::AlgorithmUnspecified
                }
                "GAUSSIAN_PROCESS_BANDIT" => {
                    GoogleCloudMlV1StudyConfigAlgorithm::GaussianProcessBandit
                }
                "GRID_SEARCH" => GoogleCloudMlV1StudyConfigAlgorithm::GridSearch,
                "RANDOM_SEARCH" => GoogleCloudMlV1StudyConfigAlgorithm::RandomSearch,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1StudyConfigAlgorithm {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1StudyConfigAlgorithm {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudMlV1StudyConfigMetricSpec {
        #[doc = "Required. The optimization goal of the metric."]
        #[serde(
            rename = "goal",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub goal: ::std::option::Option<crate::schemas::GoogleCloudMlV1StudyConfigMetricSpecGoal>,
        #[doc = "Required. The name of the metric."]
        #[serde(
            rename = "metric",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metric: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1StudyConfigMetricSpec {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1StudyConfigMetricSpec {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudMlV1StudyConfigMetricSpecGoal {
        #[doc = "Goal Type will default to maximize."]
        GoalTypeUnspecified,
        #[doc = "Maximize the goal metric."]
        Maximize,
        #[doc = "Minimize the goal metric."]
        Minimize,
    }
    impl GoogleCloudMlV1StudyConfigMetricSpecGoal {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudMlV1StudyConfigMetricSpecGoal::GoalTypeUnspecified => {
                    "GOAL_TYPE_UNSPECIFIED"
                }
                GoogleCloudMlV1StudyConfigMetricSpecGoal::Maximize => "MAXIMIZE",
                GoogleCloudMlV1StudyConfigMetricSpecGoal::Minimize => "MINIMIZE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudMlV1StudyConfigMetricSpecGoal {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudMlV1StudyConfigMetricSpecGoal {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleCloudMlV1StudyConfigMetricSpecGoal, ()> {
            Ok(match s {
                "GOAL_TYPE_UNSPECIFIED" => {
                    GoogleCloudMlV1StudyConfigMetricSpecGoal::GoalTypeUnspecified
                }
                "MAXIMIZE" => GoogleCloudMlV1StudyConfigMetricSpecGoal::Maximize,
                "MINIMIZE" => GoogleCloudMlV1StudyConfigMetricSpecGoal::Minimize,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleCloudMlV1StudyConfigMetricSpecGoal {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudMlV1StudyConfigMetricSpecGoal {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudMlV1StudyConfigMetricSpecGoal {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "GOAL_TYPE_UNSPECIFIED" => {
                    GoogleCloudMlV1StudyConfigMetricSpecGoal::GoalTypeUnspecified
                }
                "MAXIMIZE" => GoogleCloudMlV1StudyConfigMetricSpecGoal::Maximize,
                "MINIMIZE" => GoogleCloudMlV1StudyConfigMetricSpecGoal::Minimize,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1StudyConfigMetricSpecGoal {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1StudyConfigMetricSpecGoal {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudMlV1StudyConfigParameterSpec { # [ doc = "The value spec for a 'CATEGORICAL' parameter." ] # [ serde ( rename = "categoricalValueSpec" , default , skip_serializing_if = "std::option::Option::is_none" ) ] pub categorical_value_spec : :: std :: option :: Option < crate :: schemas :: GoogleCloudMlV1StudyConfigParameterSpecCategoricalValueSpec > , # [ doc = "A child node is active if the parameter's value matches the child node's\nmatching_parent_values.\n\nIf two items in child_parameter_specs have the same name, they must have\ndisjoint matching_parent_values." ] # [ serde ( rename = "childParameterSpecs" , default , skip_serializing_if = "std::option::Option::is_none" ) ] pub child_parameter_specs : :: std :: option :: Option < Vec < crate :: schemas :: GoogleCloudMlV1StudyConfigParameterSpec > > , # [ doc = "The value spec for a 'DISCRETE' parameter." ] # [ serde ( rename = "discreteValueSpec" , default , skip_serializing_if = "std::option::Option::is_none" ) ] pub discrete_value_spec : :: std :: option :: Option < crate :: schemas :: GoogleCloudMlV1StudyConfigParameterSpecDiscreteValueSpec > , # [ doc = "The value spec for a 'DOUBLE' parameter." ] # [ serde ( rename = "doubleValueSpec" , default , skip_serializing_if = "std::option::Option::is_none" ) ] pub double_value_spec : :: std :: option :: Option < crate :: schemas :: GoogleCloudMlV1StudyConfigParameterSpecDoubleValueSpec > , # [ doc = "The value spec for an 'INTEGER' parameter." ] # [ serde ( rename = "integerValueSpec" , default , skip_serializing_if = "std::option::Option::is_none" ) ] pub integer_value_spec : :: std :: option :: Option < crate :: schemas :: GoogleCloudMlV1StudyConfigParameterSpecIntegerValueSpec > , # [ doc = "Required. The parameter name must be unique amongst all ParameterSpecs." ] # [ serde ( rename = "parameter" , default , skip_serializing_if = "std::option::Option::is_none" ) ] pub parameter : :: std :: option :: Option < String > , # [ serde ( rename = "parentCategoricalValues" , default , skip_serializing_if = "std::option::Option::is_none" ) ] pub parent_categorical_values : :: std :: option :: Option < crate :: schemas :: GoogleCloudMlV1StudyConfigParameterSpecMatchingParentCategoricalValueSpec > , # [ serde ( rename = "parentDiscreteValues" , default , skip_serializing_if = "std::option::Option::is_none" ) ] pub parent_discrete_values : :: std :: option :: Option < crate :: schemas :: GoogleCloudMlV1StudyConfigParameterSpecMatchingParentDiscreteValueSpec > , # [ serde ( rename = "parentIntValues" , default , skip_serializing_if = "std::option::Option::is_none" ) ] pub parent_int_values : :: std :: option :: Option < crate :: schemas :: GoogleCloudMlV1StudyConfigParameterSpecMatchingParentIntValueSpec > , # [ doc = "Required. The type of the parameter." ] # [ serde ( rename = "type" , default , skip_serializing_if = "std::option::Option::is_none" ) ] pub r#type : :: std :: option :: Option < crate :: schemas :: GoogleCloudMlV1StudyConfigParameterSpecType > , # [ doc = "How the parameter should be scaled.\nLeave unset for categorical parameters." ] # [ serde ( rename = "scaleType" , default , skip_serializing_if = "std::option::Option::is_none" ) ] pub scale_type : :: std :: option :: Option < crate :: schemas :: GoogleCloudMlV1StudyConfigParameterSpecScaleType > , }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1StudyConfigParameterSpec {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1StudyConfigParameterSpec {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudMlV1StudyConfigParameterSpecType {
        #[doc = "The parameter is categorical, with a value chosen from the categories\nfield."]
        Categorical,
        #[doc = "The parameter is real valued, with a fixed set of feasible points. If\n`type==DISCRETE`, feasible_points must be provided, and\n{`min_value`, `max_value`} will be ignored."]
        Discrete,
        #[doc = "Type for real-valued parameters."]
        Double,
        #[doc = "Type for integral parameters."]
        Integer,
        #[doc = "You must specify a valid type. Using this unspecified type will result\nin an error."]
        ParameterTypeUnspecified,
    }
    impl GoogleCloudMlV1StudyConfigParameterSpecType {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudMlV1StudyConfigParameterSpecType::Categorical => "CATEGORICAL",
                GoogleCloudMlV1StudyConfigParameterSpecType::Discrete => "DISCRETE",
                GoogleCloudMlV1StudyConfigParameterSpecType::Double => "DOUBLE",
                GoogleCloudMlV1StudyConfigParameterSpecType::Integer => "INTEGER",
                GoogleCloudMlV1StudyConfigParameterSpecType::ParameterTypeUnspecified => {
                    "PARAMETER_TYPE_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudMlV1StudyConfigParameterSpecType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudMlV1StudyConfigParameterSpecType {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleCloudMlV1StudyConfigParameterSpecType, ()> {
            Ok(match s {
                "CATEGORICAL" => GoogleCloudMlV1StudyConfigParameterSpecType::Categorical,
                "DISCRETE" => GoogleCloudMlV1StudyConfigParameterSpecType::Discrete,
                "DOUBLE" => GoogleCloudMlV1StudyConfigParameterSpecType::Double,
                "INTEGER" => GoogleCloudMlV1StudyConfigParameterSpecType::Integer,
                "PARAMETER_TYPE_UNSPECIFIED" => {
                    GoogleCloudMlV1StudyConfigParameterSpecType::ParameterTypeUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleCloudMlV1StudyConfigParameterSpecType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudMlV1StudyConfigParameterSpecType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudMlV1StudyConfigParameterSpecType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CATEGORICAL" => GoogleCloudMlV1StudyConfigParameterSpecType::Categorical,
                "DISCRETE" => GoogleCloudMlV1StudyConfigParameterSpecType::Discrete,
                "DOUBLE" => GoogleCloudMlV1StudyConfigParameterSpecType::Double,
                "INTEGER" => GoogleCloudMlV1StudyConfigParameterSpecType::Integer,
                "PARAMETER_TYPE_UNSPECIFIED" => {
                    GoogleCloudMlV1StudyConfigParameterSpecType::ParameterTypeUnspecified
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1StudyConfigParameterSpecType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1StudyConfigParameterSpecType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudMlV1StudyConfigParameterSpecScaleType {
        #[doc = "By default, no scaling is applied."]
        ScaleTypeUnspecified,
        #[doc = "Scales the feasible space to (0, 1) linearly."]
        UnitLinearScale,
        #[doc = "Scales the feasible space logarithmically to (0, 1). The entire\nfeasible space must be strictly positive."]
        UnitLogScale,
        #[doc = "Scales the feasible space \"reverse\" logarithmically to (0, 1). The\nresult is that values close to the top of the feasible space are spread\nout more than points near the bottom. The entire feasible space must be\nstrictly positive."]
        UnitReverseLogScale,
    }
    impl GoogleCloudMlV1StudyConfigParameterSpecScaleType {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudMlV1StudyConfigParameterSpecScaleType::ScaleTypeUnspecified => {
                    "SCALE_TYPE_UNSPECIFIED"
                }
                GoogleCloudMlV1StudyConfigParameterSpecScaleType::UnitLinearScale => {
                    "UNIT_LINEAR_SCALE"
                }
                GoogleCloudMlV1StudyConfigParameterSpecScaleType::UnitLogScale => "UNIT_LOG_SCALE",
                GoogleCloudMlV1StudyConfigParameterSpecScaleType::UnitReverseLogScale => {
                    "UNIT_REVERSE_LOG_SCALE"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudMlV1StudyConfigParameterSpecScaleType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudMlV1StudyConfigParameterSpecScaleType {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleCloudMlV1StudyConfigParameterSpecScaleType, ()> {
            Ok(match s {
                "SCALE_TYPE_UNSPECIFIED" => {
                    GoogleCloudMlV1StudyConfigParameterSpecScaleType::ScaleTypeUnspecified
                }
                "UNIT_LINEAR_SCALE" => {
                    GoogleCloudMlV1StudyConfigParameterSpecScaleType::UnitLinearScale
                }
                "UNIT_LOG_SCALE" => GoogleCloudMlV1StudyConfigParameterSpecScaleType::UnitLogScale,
                "UNIT_REVERSE_LOG_SCALE" => {
                    GoogleCloudMlV1StudyConfigParameterSpecScaleType::UnitReverseLogScale
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleCloudMlV1StudyConfigParameterSpecScaleType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudMlV1StudyConfigParameterSpecScaleType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudMlV1StudyConfigParameterSpecScaleType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "SCALE_TYPE_UNSPECIFIED" => {
                    GoogleCloudMlV1StudyConfigParameterSpecScaleType::ScaleTypeUnspecified
                }
                "UNIT_LINEAR_SCALE" => {
                    GoogleCloudMlV1StudyConfigParameterSpecScaleType::UnitLinearScale
                }
                "UNIT_LOG_SCALE" => GoogleCloudMlV1StudyConfigParameterSpecScaleType::UnitLogScale,
                "UNIT_REVERSE_LOG_SCALE" => {
                    GoogleCloudMlV1StudyConfigParameterSpecScaleType::UnitReverseLogScale
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1StudyConfigParameterSpecScaleType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1StudyConfigParameterSpecScaleType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudMlV1StudyConfigParameterSpecCategoricalValueSpec {
        #[doc = "Must be specified if type is `CATEGORICAL`.\nThe list of possible categories."]
        #[serde(
            rename = "values",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub values: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudMlV1StudyConfigParameterSpecCategoricalValueSpec
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudMlV1StudyConfigParameterSpecCategoricalValueSpec
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudMlV1StudyConfigParameterSpecDiscreteValueSpec {
        #[doc = "Must be specified if type is `DISCRETE`.\nA list of feasible points.\nThe list should be in strictly increasing order. For instance, this\nparameter might have possible settings of 1.5, 2.5, and 4.0. This list\nshould not contain more than 1,000 values."]
        #[serde(
            rename = "values",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub values: ::std::option::Option<Vec<f64>>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudMlV1StudyConfigParameterSpecDiscreteValueSpec
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudMlV1StudyConfigParameterSpecDiscreteValueSpec
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudMlV1StudyConfigParameterSpecDoubleValueSpec {
        #[doc = "Must be specified if type is `DOUBLE`. Maximum value of the parameter."]
        #[serde(
            rename = "maxValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub max_value: ::std::option::Option<f64>,
        #[doc = "Must be specified if type is `DOUBLE`. Minimum value of the parameter."]
        #[serde(
            rename = "minValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub min_value: ::std::option::Option<f64>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudMlV1StudyConfigParameterSpecDoubleValueSpec
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudMlV1StudyConfigParameterSpecDoubleValueSpec
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudMlV1StudyConfigParameterSpecIntegerValueSpec {
        #[doc = "Must be specified if type is `INTEGER`. Maximum value of the parameter."]
        #[serde(
            rename = "maxValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub max_value: ::std::option::Option<i64>,
        #[doc = "Must be specified if type is `INTEGER`. Minimum value of the parameter."]
        #[serde(
            rename = "minValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub min_value: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudMlV1StudyConfigParameterSpecIntegerValueSpec
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudMlV1StudyConfigParameterSpecIntegerValueSpec
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudMlV1StudyConfigParameterSpecMatchingParentCategoricalValueSpec {
        #[doc = "Matches values of the parent parameter with type 'CATEGORICAL'.\nAll values must exist in `categorical_value_spec` of parent parameter."]
        #[serde(
            rename = "values",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub values: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudMlV1StudyConfigParameterSpecMatchingParentCategoricalValueSpec
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudMlV1StudyConfigParameterSpecMatchingParentCategoricalValueSpec
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudMlV1StudyConfigParameterSpecMatchingParentDiscreteValueSpec {
        #[doc = "Matches values of the parent parameter with type 'DISCRETE'.\nAll values must exist in `discrete_value_spec` of parent parameter."]
        #[serde(
            rename = "values",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub values: ::std::option::Option<Vec<f64>>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudMlV1StudyConfigParameterSpecMatchingParentDiscreteValueSpec
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudMlV1StudyConfigParameterSpecMatchingParentDiscreteValueSpec
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudMlV1StudyConfigParameterSpecMatchingParentIntValueSpec {
        #[doc = "Matches values of the parent parameter with type 'INTEGER'.\nAll values must lie in `integer_value_spec` of parent parameter."]
        #[serde(
            rename = "values",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub values: ::std::option::Option<Vec<i64>>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudMlV1StudyConfigParameterSpecMatchingParentIntValueSpec
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudMlV1StudyConfigParameterSpecMatchingParentIntValueSpec
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudMlV1SuggestTrialsMetadata {
        #[doc = "The identifier of the client that is requesting the suggestion."]
        #[serde(
            rename = "clientId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub client_id: ::std::option::Option<String>,
        #[doc = "The time operation was submitted."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "The name of the study that the trial belongs to."]
        #[serde(
            rename = "study",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub study: ::std::option::Option<String>,
        #[doc = "The number of suggestions requested."]
        #[serde(
            rename = "suggestionCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub suggestion_count: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1SuggestTrialsMetadata {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1SuggestTrialsMetadata {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudMlV1SuggestTrialsRequest {
        #[doc = "Required. The identifier of the client that is requesting the suggestion.\n\nIf multiple SuggestTrialsRequests have the same `client_id`,\nthe service will return the identical suggested trial if the trial is\npending, and provide a new trial if the last suggested trial was completed."]
        #[serde(
            rename = "clientId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub client_id: ::std::option::Option<String>,
        #[doc = "Required. The number of suggestions requested."]
        #[serde(
            rename = "suggestionCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub suggestion_count: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1SuggestTrialsRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1SuggestTrialsRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudMlV1SuggestTrialsResponse {
        #[doc = "The time at which operation processing completed."]
        #[serde(
            rename = "endTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end_time: ::std::option::Option<String>,
        #[doc = "The time at which the operation was started."]
        #[serde(
            rename = "startTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_time: ::std::option::Option<String>,
        #[doc = "The state of the study."]
        #[serde(
            rename = "studyState",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub study_state:
            ::std::option::Option<crate::schemas::GoogleCloudMlV1SuggestTrialsResponseStudyState>,
        #[doc = "A list of trials."]
        #[serde(
            rename = "trials",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub trials: ::std::option::Option<Vec<crate::schemas::GoogleCloudMlV1Trial>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1SuggestTrialsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1SuggestTrialsResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudMlV1SuggestTrialsResponseStudyState {
        #[doc = "The study is active."]
        Active,
        #[doc = "The study is done when the service exhausts the parameter search space\nor max_trial_count is reached."]
        Completed,
        #[doc = "The study is stopped due to an internal error."]
        Inactive,
        #[doc = "The study state is unspecified."]
        StateUnspecified,
    }
    impl GoogleCloudMlV1SuggestTrialsResponseStudyState {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudMlV1SuggestTrialsResponseStudyState::Active => "ACTIVE",
                GoogleCloudMlV1SuggestTrialsResponseStudyState::Completed => "COMPLETED",
                GoogleCloudMlV1SuggestTrialsResponseStudyState::Inactive => "INACTIVE",
                GoogleCloudMlV1SuggestTrialsResponseStudyState::StateUnspecified => {
                    "STATE_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudMlV1SuggestTrialsResponseStudyState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudMlV1SuggestTrialsResponseStudyState {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleCloudMlV1SuggestTrialsResponseStudyState, ()> {
            Ok(match s {
                "ACTIVE" => GoogleCloudMlV1SuggestTrialsResponseStudyState::Active,
                "COMPLETED" => GoogleCloudMlV1SuggestTrialsResponseStudyState::Completed,
                "INACTIVE" => GoogleCloudMlV1SuggestTrialsResponseStudyState::Inactive,
                "STATE_UNSPECIFIED" => {
                    GoogleCloudMlV1SuggestTrialsResponseStudyState::StateUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleCloudMlV1SuggestTrialsResponseStudyState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudMlV1SuggestTrialsResponseStudyState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudMlV1SuggestTrialsResponseStudyState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ACTIVE" => GoogleCloudMlV1SuggestTrialsResponseStudyState::Active,
                "COMPLETED" => GoogleCloudMlV1SuggestTrialsResponseStudyState::Completed,
                "INACTIVE" => GoogleCloudMlV1SuggestTrialsResponseStudyState::Inactive,
                "STATE_UNSPECIFIED" => {
                    GoogleCloudMlV1SuggestTrialsResponseStudyState::StateUnspecified
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1SuggestTrialsResponseStudyState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1SuggestTrialsResponseStudyState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudMlV1TrainingInput {
        #[doc = "Optional. Command-line arguments passed to the training application when it\nstarts. If your job uses a custom container, then the arguments are passed\nto the container's <a class=\"external\" target=\"_blank\"\nhref=\"https://docs.docker.com/engine/reference/builder/#entrypoint\">\n`ENTRYPOINT`</a> command."]
        #[serde(
            rename = "args",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub args: ::std::option::Option<Vec<String>>,
        #[doc = "Optional. Options for using customer-managed encryption keys (CMEK) to\nprotect resources created by a training job, instead of using Google's\ndefault encryption. If this is set, then all resources created by the\ntraining job will be encrypted with the customer-managed encryption key\nthat you specify.\n\n[Learn how and when to use CMEK with AI Platform\nTraining](/ai-platform/training/docs/cmek)."]
        #[serde(
            rename = "encryptionConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub encryption_config:
            ::std::option::Option<crate::schemas::GoogleCloudMlV1EncryptionConfig>,
        #[doc = "Optional. The configuration for evaluators.\n\nYou should only set `evaluatorConfig.acceleratorConfig` if\n`evaluatorType` is set to a Compute Engine machine type. [Learn\nabout restrictions on accelerator configurations for\ntraining.](/ai-platform/training/docs/using-gpus#compute-engine-machine-types-with-gpu)\n\nSet `evaluatorConfig.imageUri` only if you build a custom image for\nyour evaluator. If `evaluatorConfig.imageUri` has not been\nset, AI Platform uses the value of `masterConfig.imageUri`. Learn more about [configuring custom\ncontainers](/ai-platform/training/docs/distributed-training-containers)."]
        #[serde(
            rename = "evaluatorConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub evaluator_config: ::std::option::Option<crate::schemas::GoogleCloudMlV1ReplicaConfig>,
        #[doc = "Optional. The number of evaluator replicas to use for the training job.\nEach replica in the cluster will be of the type specified in\n`evaluator_type`.\n\nThis value can only be used when `scale_tier` is set to `CUSTOM`. If you\nset this value, you must also set `evaluator_type`.\n\nThe default value is zero."]
        #[serde(
            rename = "evaluatorCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub evaluator_count: ::std::option::Option<i64>,
        #[doc = "Optional. Specifies the type of virtual machine to use for your training\njob's evaluator nodes.\n\nThe supported values are the same as those described in the entry for\n`masterType`.\n\nThis value must be consistent with the category of machine type that\n`masterType` uses. In other words, both must be Compute Engine machine\ntypes or both must be legacy machine types.\n\nThis value must be present when `scaleTier` is set to `CUSTOM` and\n`evaluatorCount` is greater than zero."]
        #[serde(
            rename = "evaluatorType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub evaluator_type: ::std::option::Option<String>,
        #[doc = "Optional. The set of Hyperparameters to tune."]
        #[serde(
            rename = "hyperparameters",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub hyperparameters:
            ::std::option::Option<crate::schemas::GoogleCloudMlV1HyperparameterSpec>,
        #[doc = "Optional. A Google Cloud Storage path in which to store training outputs\nand other data needed for training. This path is passed to your TensorFlow\nprogram as the '--job-dir' command-line argument. The benefit of specifying\nthis field is that Cloud ML validates the path for use in training."]
        #[serde(
            rename = "jobDir",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub job_dir: ::std::option::Option<String>,
        #[doc = "Optional. The configuration for your master worker.\n\nYou should only set `masterConfig.acceleratorConfig` if `masterType` is set\nto a Compute Engine machine type. Learn about [restrictions on accelerator\nconfigurations for\ntraining.](/ai-platform/training/docs/using-gpus#compute-engine-machine-types-with-gpu)\n\nSet `masterConfig.imageUri` only if you build a custom image. Only one of\n`masterConfig.imageUri` and `runtimeVersion` should be set. Learn more\nabout [configuring custom\ncontainers](/ai-platform/training/docs/distributed-training-containers)."]
        #[serde(
            rename = "masterConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub master_config: ::std::option::Option<crate::schemas::GoogleCloudMlV1ReplicaConfig>,
        #[doc = "Optional. Specifies the type of virtual machine to use for your training\njob's master worker. You must specify this field when `scaleTier` is set to\n`CUSTOM`.\n\nYou can use certain Compute Engine machine types directly in this field.\nThe following types are supported:\n\n* `n1-standard-4`\n* `n1-standard-8`\n* `n1-standard-16`\n* `n1-standard-32`\n* `n1-standard-64`\n* `n1-standard-96`\n* `n1-highmem-2`\n* `n1-highmem-4`\n* `n1-highmem-8`\n* `n1-highmem-16`\n* `n1-highmem-32`\n* `n1-highmem-64`\n* `n1-highmem-96`\n* `n1-highcpu-16`\n* `n1-highcpu-32`\n* `n1-highcpu-64`\n* `n1-highcpu-96`\n\nLearn more about [using Compute Engine machine\ntypes](/ml-engine/docs/machine-types#compute-engine-machine-types).\n\nAlternatively, you can use the following legacy machine types:\n\n* `standard`\n* `large_model`\n* `complex_model_s`\n* `complex_model_m`\n* `complex_model_l`\n* `standard_gpu`\n* `complex_model_m_gpu`\n* `complex_model_l_gpu`\n* `standard_p100`\n* `complex_model_m_p100`\n* `standard_v100`\n* `large_model_v100`\n* `complex_model_m_v100`\n* `complex_model_l_v100`\n\nLearn more about [using legacy machine\ntypes](/ml-engine/docs/machine-types#legacy-machine-types).\n\nFinally, if you want to use a TPU for training, specify `cloud_tpu` in this\nfield. Learn more about the [special configuration options for training\nwith\nTPUs](/ml-engine/docs/tensorflow/using-tpus#configuring_a_custom_tpu_machine)."]
        #[serde(
            rename = "masterType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub master_type: ::std::option::Option<String>,
        #[doc = "Optional. The full name of the Google Compute Engine\n[network](/compute/docs/networks-and-firewalls#networks) to which the Job\nis peered. For example, projects/12345/global/networks/myVPC. Format is of\nthe form projects/{project}/global/networks/{network}. Where {project} is a\nproject number, as in '12345', and {network} is network name.\".\n\nPrivate services access must already be configured for the network. If left\nunspecified, the Job is not peered with any network. Learn more -\nConnecting Job to user network over private\nIP."]
        #[serde(
            rename = "network",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub network: ::std::option::Option<String>,
        #[doc = "Required. The Google Cloud Storage location of the packages with\nthe training program and any additional dependencies.\nThe maximum number of package URIs is 100."]
        #[serde(
            rename = "packageUris",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub package_uris: ::std::option::Option<Vec<String>>,
        #[doc = "Optional. The configuration for parameter servers.\n\nYou should only set `parameterServerConfig.acceleratorConfig` if\n`parameterServerType` is set to a Compute Engine machine type. [Learn\nabout restrictions on accelerator configurations for\ntraining.](/ai-platform/training/docs/using-gpus#compute-engine-machine-types-with-gpu)\n\nSet `parameterServerConfig.imageUri` only if you build a custom image for\nyour parameter server. If `parameterServerConfig.imageUri` has not been\nset, AI Platform uses the value of `masterConfig.imageUri`. Learn more about [configuring custom\ncontainers](/ai-platform/training/docs/distributed-training-containers)."]
        #[serde(
            rename = "parameterServerConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub parameter_server_config:
            ::std::option::Option<crate::schemas::GoogleCloudMlV1ReplicaConfig>,
        #[doc = "Optional. The number of parameter server replicas to use for the training\njob. Each replica in the cluster will be of the type specified in\n`parameter_server_type`.\n\nThis value can only be used when `scale_tier` is set to `CUSTOM`. If you\nset this value, you must also set `parameter_server_type`.\n\nThe default value is zero."]
        #[serde(
            rename = "parameterServerCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub parameter_server_count: ::std::option::Option<i64>,
        #[doc = "Optional. Specifies the type of virtual machine to use for your training\njob's parameter server.\n\nThe supported values are the same as those described in the entry for\n`master_type`.\n\nThis value must be consistent with the category of machine type that\n`masterType` uses. In other words, both must be Compute Engine machine\ntypes or both must be legacy machine types.\n\nThis value must be present when `scaleTier` is set to `CUSTOM` and\n`parameter_server_count` is greater than zero."]
        #[serde(
            rename = "parameterServerType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub parameter_server_type: ::std::option::Option<String>,
        #[doc = "Required. The Python module name to run after installing the packages."]
        #[serde(
            rename = "pythonModule",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub python_module: ::std::option::Option<String>,
        #[doc = "Optional. The version of Python used in training. You must either specify\nthis field or specify `masterConfig.imageUri`.\n\nThe following Python versions are available:\n\n* Python '3.7' is available when `runtime_version` is set to '1.15' or\n  later.\n* Python '3.5' is available when `runtime_version` is set to a version\n  from '1.4' to '1.14'.\n* Python '2.7' is available when `runtime_version` is set to '1.15' or\n  earlier.\n\nRead more about the Python versions available for [each runtime\nversion](/ml-engine/docs/runtime-version-list)."]
        #[serde(
            rename = "pythonVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub python_version: ::std::option::Option<String>,
        #[doc = "Required. The region to run the training job in. See the [available\nregions](/ai-platform/training/docs/regions) for AI Platform Training."]
        #[serde(
            rename = "region",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub region: ::std::option::Option<String>,
        #[doc = "Optional. The AI Platform runtime version to use for training. You must\neither specify this field or specify `masterConfig.imageUri`.\n\nFor more information, see the [runtime version\nlist](/ai-platform/training/docs/runtime-version-list) and learn [how to\nmanage runtime versions](/ai-platform/training/docs/versioning)."]
        #[serde(
            rename = "runtimeVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub runtime_version: ::std::option::Option<String>,
        #[doc = "Required. Specifies the machine types, the number of replicas for workers\nand parameter servers."]
        #[serde(
            rename = "scaleTier",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub scale_tier:
            ::std::option::Option<crate::schemas::GoogleCloudMlV1TrainingInputScaleTier>,
        #[doc = "Optional. Scheduling options for a training job."]
        #[serde(
            rename = "scheduling",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub scheduling: ::std::option::Option<crate::schemas::GoogleCloudMlV1Scheduling>,
        #[doc = "Optional. Use `chief` instead of `master` in the `TF_CONFIG` environment\nvariable when training with a custom container. Defaults to `false`. [Learn\nmore about this\nfield.](/ai-platform/training/docs/distributed-training-details#chief-versus-master)\n\nThis field has no effect for training jobs that don't use a custom\ncontainer."]
        #[serde(
            rename = "useChiefInTfConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub use_chief_in_tf_config: ::std::option::Option<bool>,
        #[doc = "Optional. The configuration for workers.\n\nYou should only set `workerConfig.acceleratorConfig` if `workerType` is set\nto a Compute Engine machine type. [Learn about restrictions on accelerator\nconfigurations for\ntraining.](/ai-platform/training/docs/using-gpus#compute-engine-machine-types-with-gpu)\n\nSet `workerConfig.imageUri` only if you build a custom image for your\nworker. If `workerConfig.imageUri` has not been set, AI Platform uses\nthe value of `masterConfig.imageUri`. Learn more about [configuring custom\ncontainers](/ai-platform/training/docs/distributed-training-containers)."]
        #[serde(
            rename = "workerConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub worker_config: ::std::option::Option<crate::schemas::GoogleCloudMlV1ReplicaConfig>,
        #[doc = "Optional. The number of worker replicas to use for the training job. Each\nreplica in the cluster will be of the type specified in `worker_type`.\n\nThis value can only be used when `scale_tier` is set to `CUSTOM`. If you\nset this value, you must also set `worker_type`.\n\nThe default value is zero."]
        #[serde(
            rename = "workerCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub worker_count: ::std::option::Option<i64>,
        #[doc = "Optional. Specifies the type of virtual machine to use for your training\njob's worker nodes.\n\nThe supported values are the same as those described in the entry for\n`masterType`.\n\nThis value must be consistent with the category of machine type that\n`masterType` uses. In other words, both must be Compute Engine machine\ntypes or both must be legacy machine types.\n\nIf you use `cloud_tpu` for this value, see special instructions for\n[configuring a custom TPU\nmachine](/ml-engine/docs/tensorflow/using-tpus#configuring_a_custom_tpu_machine).\n\nThis value must be present when `scaleTier` is set to `CUSTOM` and\n`workerCount` is greater than zero."]
        #[serde(
            rename = "workerType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub worker_type: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1TrainingInput {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1TrainingInput {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudMlV1TrainingInputScaleTier {
        #[doc = "A single worker instance. This tier is suitable for learning how to use\nCloud ML, and for experimenting with new models using small datasets."]
        Basic,
        #[doc = "A single worker instance [with a\nGPU](/ai-platform/training/docs/using-gpus)."]
        BasicGpu,
        #[doc = "A single worker instance with a\n[Cloud TPU](/ml-engine/docs/tensorflow/using-tpus)."]
        BasicTpu,
        #[doc = "The CUSTOM tier is not a set tier, but rather enables you to use your\nown cluster specification. When you use this tier, set values to\nconfigure your processing cluster according to these guidelines:\n\n* You *must* set `TrainingInput.masterType` to specify the type\n  of machine to use for your master node. This is the only required\n  setting.\n\n* You *may* set `TrainingInput.workerCount` to specify the number of\n  workers to use. If you specify one or more workers, you *must* also\n  set `TrainingInput.workerType` to specify the type of machine to use\n  for your worker nodes.\n\n* You *may* set `TrainingInput.parameterServerCount` to specify the\n  number of parameter servers to use. If you specify one or more\n  parameter servers, you *must* also set\n  `TrainingInput.parameterServerType` to specify the type of machine to\n  use for your parameter servers.\n\nNote that all of your workers must use the same machine type, which can\nbe different from your parameter server type and master type. Your\nparameter servers must likewise use the same machine type, which can be\ndifferent from your worker type and master type."]
        Custom,
        #[doc = "A large number of workers with many parameter servers."]
        Premium1,
        #[doc = "Many workers and a few parameter servers."]
        Standard1,
    }
    impl GoogleCloudMlV1TrainingInputScaleTier {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudMlV1TrainingInputScaleTier::Basic => "BASIC",
                GoogleCloudMlV1TrainingInputScaleTier::BasicGpu => "BASIC_GPU",
                GoogleCloudMlV1TrainingInputScaleTier::BasicTpu => "BASIC_TPU",
                GoogleCloudMlV1TrainingInputScaleTier::Custom => "CUSTOM",
                GoogleCloudMlV1TrainingInputScaleTier::Premium1 => "PREMIUM_1",
                GoogleCloudMlV1TrainingInputScaleTier::Standard1 => "STANDARD_1",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudMlV1TrainingInputScaleTier {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudMlV1TrainingInputScaleTier {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<GoogleCloudMlV1TrainingInputScaleTier, ()> {
            Ok(match s {
                "BASIC" => GoogleCloudMlV1TrainingInputScaleTier::Basic,
                "BASIC_GPU" => GoogleCloudMlV1TrainingInputScaleTier::BasicGpu,
                "BASIC_TPU" => GoogleCloudMlV1TrainingInputScaleTier::BasicTpu,
                "CUSTOM" => GoogleCloudMlV1TrainingInputScaleTier::Custom,
                "PREMIUM_1" => GoogleCloudMlV1TrainingInputScaleTier::Premium1,
                "STANDARD_1" => GoogleCloudMlV1TrainingInputScaleTier::Standard1,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleCloudMlV1TrainingInputScaleTier {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudMlV1TrainingInputScaleTier {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudMlV1TrainingInputScaleTier {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BASIC" => GoogleCloudMlV1TrainingInputScaleTier::Basic,
                "BASIC_GPU" => GoogleCloudMlV1TrainingInputScaleTier::BasicGpu,
                "BASIC_TPU" => GoogleCloudMlV1TrainingInputScaleTier::BasicTpu,
                "CUSTOM" => GoogleCloudMlV1TrainingInputScaleTier::Custom,
                "PREMIUM_1" => GoogleCloudMlV1TrainingInputScaleTier::Premium1,
                "STANDARD_1" => GoogleCloudMlV1TrainingInputScaleTier::Standard1,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1TrainingInputScaleTier {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1TrainingInputScaleTier {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudMlV1TrainingOutput {
        #[doc = "Details related to built-in algorithms jobs.\nOnly set for built-in algorithms jobs."]
        #[serde(
            rename = "builtInAlgorithmOutput",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub built_in_algorithm_output:
            ::std::option::Option<crate::schemas::GoogleCloudMlV1BuiltInAlgorithmOutput>,
        #[doc = "The number of hyperparameter tuning trials that completed successfully.\nOnly set for hyperparameter tuning jobs."]
        #[serde(
            rename = "completedTrialCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub completed_trial_count: ::std::option::Option<i64>,
        #[doc = "The amount of ML units consumed by the job."]
        #[serde(
            rename = "consumedMLUnits",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub consumed_ml_units: ::std::option::Option<f64>,
        #[doc = "The TensorFlow summary tag name used for optimizing hyperparameter tuning\ntrials. See\n[`HyperparameterSpec.hyperparameterMetricTag`](#HyperparameterSpec.FIELDS.hyperparameter_metric_tag)\nfor more information. Only set for hyperparameter tuning jobs."]
        #[serde(
            rename = "hyperparameterMetricTag",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub hyperparameter_metric_tag: ::std::option::Option<String>,
        #[doc = "Whether this job is a built-in Algorithm job."]
        #[serde(
            rename = "isBuiltInAlgorithmJob",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub is_built_in_algorithm_job: ::std::option::Option<bool>,
        #[doc = "Whether this job is a hyperparameter tuning job."]
        #[serde(
            rename = "isHyperparameterTuningJob",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub is_hyperparameter_tuning_job: ::std::option::Option<bool>,
        #[doc = "Results for individual Hyperparameter trials.\nOnly set for hyperparameter tuning jobs."]
        #[serde(
            rename = "trials",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub trials: ::std::option::Option<Vec<crate::schemas::GoogleCloudMlV1HyperparameterOutput>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1TrainingOutput {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1TrainingOutput {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudMlV1Trial {
        #[doc = "Output only. The identifier of the client that originally requested this trial."]
        #[serde(
            rename = "clientId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub client_id: ::std::option::Option<String>,
        #[doc = "Output only. Time at which the trial's status changed to COMPLETED."]
        #[serde(
            rename = "endTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end_time: ::std::option::Option<String>,
        #[doc = "The final measurement containing the objective value."]
        #[serde(
            rename = "finalMeasurement",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub final_measurement: ::std::option::Option<crate::schemas::GoogleCloudMlV1Measurement>,
        #[doc = "Output only. A human readable string describing why the trial is\ninfeasible. This should only be set if trial_infeasible is true."]
        #[serde(
            rename = "infeasibleReason",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub infeasible_reason: ::std::option::Option<String>,
        #[doc = "A list of measurements that are strictly lexicographically\nordered by their induced tuples (steps, elapsed_time).\nThese are used for early stopping computations."]
        #[serde(
            rename = "measurements",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub measurements: ::std::option::Option<Vec<crate::schemas::GoogleCloudMlV1Measurement>>,
        #[doc = "Output only. Name of the trial assigned by the service."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The parameters of the trial."]
        #[serde(
            rename = "parameters",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub parameters: ::std::option::Option<Vec<crate::schemas::GoogleCloudMlV1TrialParameter>>,
        #[doc = "Output only. Time at which the trial was started."]
        #[serde(
            rename = "startTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_time: ::std::option::Option<String>,
        #[doc = "The detailed state of a trial."]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state: ::std::option::Option<crate::schemas::GoogleCloudMlV1TrialState>,
        #[doc = "Output only. If true, the parameters in this trial are not attempted again."]
        #[serde(
            rename = "trialInfeasible",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub trial_infeasible: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1Trial {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1Trial {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudMlV1TrialState {
        #[doc = "Indicates that the trial has been suggested."]
        Active,
        #[doc = "Indicates that the trial is done, and either has a final_measurement\nset, or is marked as trial_infeasible."]
        Completed,
        #[doc = "Indicates that a specific trial has been requested, but it has not yet\nbeen suggested by the service."]
        Requested,
        #[doc = "The trial state is unspecified."]
        StateUnspecified,
        #[doc = "Indicates that the trial should stop according to the service."]
        Stopping,
    }
    impl GoogleCloudMlV1TrialState {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudMlV1TrialState::Active => "ACTIVE",
                GoogleCloudMlV1TrialState::Completed => "COMPLETED",
                GoogleCloudMlV1TrialState::Requested => "REQUESTED",
                GoogleCloudMlV1TrialState::StateUnspecified => "STATE_UNSPECIFIED",
                GoogleCloudMlV1TrialState::Stopping => "STOPPING",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudMlV1TrialState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudMlV1TrialState {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<GoogleCloudMlV1TrialState, ()> {
            Ok(match s {
                "ACTIVE" => GoogleCloudMlV1TrialState::Active,
                "COMPLETED" => GoogleCloudMlV1TrialState::Completed,
                "REQUESTED" => GoogleCloudMlV1TrialState::Requested,
                "STATE_UNSPECIFIED" => GoogleCloudMlV1TrialState::StateUnspecified,
                "STOPPING" => GoogleCloudMlV1TrialState::Stopping,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleCloudMlV1TrialState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudMlV1TrialState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudMlV1TrialState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ACTIVE" => GoogleCloudMlV1TrialState::Active,
                "COMPLETED" => GoogleCloudMlV1TrialState::Completed,
                "REQUESTED" => GoogleCloudMlV1TrialState::Requested,
                "STATE_UNSPECIFIED" => GoogleCloudMlV1TrialState::StateUnspecified,
                "STOPPING" => GoogleCloudMlV1TrialState::Stopping,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1TrialState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1TrialState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudMlV1TrialParameter {
        #[doc = "Must be set if ParameterType is DOUBLE or DISCRETE."]
        #[serde(
            rename = "floatValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub float_value: ::std::option::Option<f64>,
        #[doc = "Must be set if ParameterType is INTEGER"]
        #[serde(
            rename = "intValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub int_value: ::std::option::Option<i64>,
        #[doc = "The name of the parameter."]
        #[serde(
            rename = "parameter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub parameter: ::std::option::Option<String>,
        #[doc = "Must be set if ParameterTypeis CATEGORICAL"]
        #[serde(
            rename = "stringValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub string_value: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1TrialParameter {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1TrialParameter {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudMlV1Version {
        #[doc = "Optional. Accelerator config for using GPUs for online prediction (beta).\nOnly specify this field if you have specified a Compute Engine (N1) machine\ntype in the `machineType` field. Learn more about [using GPUs for online\nprediction](/ml-engine/docs/machine-types-online-prediction#gpus)."]
        #[serde(
            rename = "acceleratorConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub accelerator_config:
            ::std::option::Option<crate::schemas::GoogleCloudMlV1AcceleratorConfig>,
        #[doc = "Automatically scale the number of nodes used to serve the model in\nresponse to increases and decreases in traffic. Care should be\ntaken to ramp up traffic according to the model's ability to scale\nor you will start seeing increases in latency and 429 response codes.\n\nNote that you cannot use AutoScaling if your version uses\n[GPUs](#Version.FIELDS.accelerator_config). Instead, you must use specify\n`manual_scaling`."]
        #[serde(
            rename = "autoScaling",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub auto_scaling: ::std::option::Option<crate::schemas::GoogleCloudMlV1AutoScaling>,
        #[doc = "Output only. The time the version was created."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "Required. The Cloud Storage location of the trained model used to\ncreate the version. See the\n[guide to model\ndeployment](/ml-engine/docs/tensorflow/deploying-models) for more\ninformation.\n\nWhen passing Version to\nprojects.models.versions.create\nthe model service uses the specified location as the source of the model.\nOnce deployed, the model version is hosted by the prediction service, so\nthis location is useful only as a historical record.\nThe total number of model files can't exceed 1000."]
        #[serde(
            rename = "deploymentUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub deployment_uri: ::std::option::Option<String>,
        #[doc = "Optional. The description specified for the version when it was created."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "Output only. The details of a failure or a cancellation."]
        #[serde(
            rename = "errorMessage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub error_message: ::std::option::Option<String>,
        #[doc = "`etag` is used for optimistic concurrency control as a way to help\nprevent simultaneous updates of a model from overwriting each other.\nIt is strongly suggested that systems make use of the `etag` in the\nread-modify-write cycle to perform model updates in order to avoid race\nconditions: An `etag` is returned in the response to `GetVersion`, and\nsystems are expected to put that etag in the request to `UpdateVersion` to\nensure that their change will be applied to the model as intended."]
        #[serde(
            rename = "etag",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub etag: ::std::option::Option<::google_api_bytes::Bytes>,
        #[doc = "Optional. Configures explainability features on the model's version.\nSome explanation features require additional metadata to be loaded\nas part of the model payload."]
        #[serde(
            rename = "explanationConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub explanation_config:
            ::std::option::Option<crate::schemas::GoogleCloudMlV1ExplanationConfig>,
        #[doc = "Optional. The machine learning framework AI Platform uses to train\nthis version of the model. Valid values are `TENSORFLOW`, `SCIKIT_LEARN`,\n`XGBOOST`. If you do not specify a framework, AI Platform\nwill analyze files in the deployment_uri to determine a framework. If you\nchoose `SCIKIT_LEARN` or `XGBOOST`, you must also set the runtime version\nof the model to 1.4 or greater.\n\nDo **not** specify a framework if you're deploying a [custom\nprediction routine](/ml-engine/docs/tensorflow/custom-prediction-routines).\n\nIf you specify a [Compute Engine (N1) machine\ntype](/ml-engine/docs/machine-types-online-prediction) in the\n`machineType` field, you must specify `TENSORFLOW`\nfor the framework."]
        #[serde(
            rename = "framework",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub framework: ::std::option::Option<crate::schemas::GoogleCloudMlV1VersionFramework>,
        #[doc = "Output only. If true, this version will be used to handle prediction\nrequests that do not specify a version.\n\nYou can change the default version by calling\nprojects.methods.versions.setDefault."]
        #[serde(
            rename = "isDefault",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub is_default: ::std::option::Option<bool>,
        #[doc = "Optional. One or more labels that you can add, to organize your model\nversions. Each label is a key-value pair, where both the key and the value\nare arbitrary strings that you supply.\nFor more information, see the documentation on\n<a href=\"/ml-engine/docs/tensorflow/resource-labels\">using labels</a>."]
        #[serde(
            rename = "labels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub labels: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "Output only. The time the version was last used for prediction."]
        #[serde(
            rename = "lastUseTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub last_use_time: ::std::option::Option<String>,
        #[doc = "Optional. The type of machine on which to serve the model. Currently only\napplies to online prediction service. If this field is not specified, it\ndefaults to `mls1-c1-m2`.\n\nOnline prediction supports the following machine types:\n\n* `mls1-c1-m2`\n* `mls1-c4-m2`\n* `n1-standard-2`\n* `n1-standard-4`\n* `n1-standard-8`\n* `n1-standard-16`\n* `n1-standard-32`\n* `n1-highmem-2`\n* `n1-highmem-4`\n* `n1-highmem-8`\n* `n1-highmem-16`\n* `n1-highmem-32`\n* `n1-highcpu-2`\n* `n1-highcpu-4`\n* `n1-highcpu-8`\n* `n1-highcpu-16`\n* `n1-highcpu-32`\n\n`mls1-c1-m2` is generally available. All other machine types are available\nin beta. Learn more about the [differences between machine\ntypes](/ml-engine/docs/machine-types-online-prediction)."]
        #[serde(
            rename = "machineType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub machine_type: ::std::option::Option<String>,
        #[doc = "Manually select the number of nodes to use for serving the\nmodel. You should generally use `auto_scaling` with an appropriate\n`min_nodes` instead, but this option is available if you want more\npredictable billing. Beware that latency and error rates will increase\nif the traffic exceeds that capability of the system to serve it based\non the selected number of nodes."]
        #[serde(
            rename = "manualScaling",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub manual_scaling: ::std::option::Option<crate::schemas::GoogleCloudMlV1ManualScaling>,
        #[doc = "Required. The name specified for the version when it was created.\n\nThe version name must be unique within the model it is created in."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Optional. Cloud Storage paths (`gs://…`) of packages for [custom\nprediction routines](/ml-engine/docs/tensorflow/custom-prediction-routines)\nor [scikit-learn pipelines with custom\ncode](/ml-engine/docs/scikit/exporting-for-prediction#custom-pipeline-code).\n\nFor a custom prediction routine, one of these packages must contain your\nPredictor class (see\n[`predictionClass`](#Version.FIELDS.prediction_class)). Additionally,\ninclude any dependencies used by your Predictor or scikit-learn pipeline\nuses that are not already included in your selected [runtime\nversion](/ml-engine/docs/tensorflow/runtime-version-list).\n\nIf you specify this field, you must also set\n[`runtimeVersion`](#Version.FIELDS.runtime_version) to 1.4 or greater."]
        #[serde(
            rename = "packageUris",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub package_uris: ::std::option::Option<Vec<String>>,
        #[doc = "Optional. The fully qualified name\n(<var>module_name</var>.<var>class_name</var>) of a class that implements\nthe Predictor interface described in this reference field. The module\ncontaining this class should be included in a package provided to the\n[`packageUris` field](#Version.FIELDS.package_uris).\n\nSpecify this field if and only if you are deploying a [custom prediction\nroutine (beta)](/ml-engine/docs/tensorflow/custom-prediction-routines).\nIf you specify this field, you must set\n[`runtimeVersion`](#Version.FIELDS.runtime_version) to 1.4 or greater and\nyou must set `machineType` to a [legacy (MLS1)\nmachine type](/ml-engine/docs/machine-types-online-prediction).\n\nThe following code sample provides the Predictor interface:\n\n<pre style=\"max-width: 626px;\">\nclass Predictor(object):\n\"\"\"Interface for constructing custom predictors.\"\"\"\n\ndef predict(self, instances, **kwargs):\n    \"\"\"Performs custom prediction.\n\n    Instances are the decoded values from the request. They have already\n    been deserialized from JSON.\n\n    Args:\n        instances: A list of prediction input instances.\n        **kwargs: A dictionary of keyword args provided as additional\n            fields on the predict request body.\n\n    Returns:\n        A list of outputs containing the prediction results. This list must\n        be JSON serializable.\n    \"\"\"\n    raise NotImplementedError()\n\n@classmethod\ndef from_path(cls, model_dir):\n    \"\"\"Creates an instance of Predictor using the given path.\n\n    Loading of the predictor should be done in this method.\n\n    Args:\n        model_dir: The local directory that contains the exported model\n            file along with any additional files uploaded when creating the\n            version resource.\n\n    Returns:\n        An instance implementing this Predictor class.\n    \"\"\"\n    raise NotImplementedError()\n</pre>\n\nLearn more about [the Predictor interface and custom prediction\nroutines](/ml-engine/docs/tensorflow/custom-prediction-routines)."]
        #[serde(
            rename = "predictionClass",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub prediction_class: ::std::option::Option<String>,
        #[doc = "Required. The version of Python used in prediction.\n\nThe following Python versions are available:\n\n* Python '3.7' is available when `runtime_version` is set to '1.15' or\n  later.\n* Python '3.5' is available when `runtime_version` is set to a version\n  from '1.4' to '1.14'.\n* Python '2.7' is available when `runtime_version` is set to '1.15' or\n  earlier.\n\nRead more about the Python versions available for [each runtime\nversion](/ml-engine/docs/runtime-version-list)."]
        #[serde(
            rename = "pythonVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub python_version: ::std::option::Option<String>,
        #[doc = "Optional. *Only* specify this field in a\nprojects.models.versions.patch\nrequest. Specifying it in a\nprojects.models.versions.create\nrequest has no effect.\n\nConfigures the request-response pair logging on predictions from this\nVersion."]
        #[serde(
            rename = "requestLoggingConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub request_logging_config:
            ::std::option::Option<crate::schemas::GoogleCloudMlV1RequestLoggingConfig>,
        #[doc = "Required. The AI Platform runtime version to use for this deployment.\n\nFor more information, see the\n[runtime version list](/ml-engine/docs/runtime-version-list) and\n[how to manage runtime versions](/ml-engine/docs/versioning)."]
        #[serde(
            rename = "runtimeVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub runtime_version: ::std::option::Option<String>,
        #[doc = "Optional. Specifies the service account for resource access control."]
        #[serde(
            rename = "serviceAccount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub service_account: ::std::option::Option<String>,
        #[doc = "Output only. The state of a version."]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state: ::std::option::Option<crate::schemas::GoogleCloudMlV1VersionState>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1Version {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1Version {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudMlV1VersionFramework {
        #[doc = "Unspecified framework. Assigns a value based on the file suffix."]
        FrameworkUnspecified,
        #[doc = "Scikit-learn framework."]
        ScikitLearn,
        #[doc = "Tensorflow framework."]
        Tensorflow,
        #[doc = "XGBoost framework."]
        Xgboost,
    }
    impl GoogleCloudMlV1VersionFramework {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudMlV1VersionFramework::FrameworkUnspecified => "FRAMEWORK_UNSPECIFIED",
                GoogleCloudMlV1VersionFramework::ScikitLearn => "SCIKIT_LEARN",
                GoogleCloudMlV1VersionFramework::Tensorflow => "TENSORFLOW",
                GoogleCloudMlV1VersionFramework::Xgboost => "XGBOOST",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudMlV1VersionFramework {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudMlV1VersionFramework {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<GoogleCloudMlV1VersionFramework, ()> {
            Ok(match s {
                "FRAMEWORK_UNSPECIFIED" => GoogleCloudMlV1VersionFramework::FrameworkUnspecified,
                "SCIKIT_LEARN" => GoogleCloudMlV1VersionFramework::ScikitLearn,
                "TENSORFLOW" => GoogleCloudMlV1VersionFramework::Tensorflow,
                "XGBOOST" => GoogleCloudMlV1VersionFramework::Xgboost,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleCloudMlV1VersionFramework {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudMlV1VersionFramework {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudMlV1VersionFramework {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "FRAMEWORK_UNSPECIFIED" => GoogleCloudMlV1VersionFramework::FrameworkUnspecified,
                "SCIKIT_LEARN" => GoogleCloudMlV1VersionFramework::ScikitLearn,
                "TENSORFLOW" => GoogleCloudMlV1VersionFramework::Tensorflow,
                "XGBOOST" => GoogleCloudMlV1VersionFramework::Xgboost,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1VersionFramework {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1VersionFramework {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudMlV1VersionState {
        #[doc = "The version is being created. New UpdateVersion and DeleteVersion\nrequests will fail if a version is in the CREATING state."]
        Creating,
        #[doc = "The version is being deleted. New UpdateVersion and DeleteVersion\nrequests will fail if a version is in the DELETING state."]
        Deleting,
        #[doc = "The version failed to be created, possibly cancelled.\n`error_message` should contain the details of the failure."]
        Failed,
        #[doc = "The version is ready for prediction."]
        Ready,
        #[doc = "The version state is unspecified."]
        Unknown,
        #[doc = "The version is being updated. New UpdateVersion and DeleteVersion\nrequests will fail if a version is in the UPDATING state."]
        Updating,
    }
    impl GoogleCloudMlV1VersionState {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudMlV1VersionState::Creating => "CREATING",
                GoogleCloudMlV1VersionState::Deleting => "DELETING",
                GoogleCloudMlV1VersionState::Failed => "FAILED",
                GoogleCloudMlV1VersionState::Ready => "READY",
                GoogleCloudMlV1VersionState::Unknown => "UNKNOWN",
                GoogleCloudMlV1VersionState::Updating => "UPDATING",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudMlV1VersionState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudMlV1VersionState {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<GoogleCloudMlV1VersionState, ()> {
            Ok(match s {
                "CREATING" => GoogleCloudMlV1VersionState::Creating,
                "DELETING" => GoogleCloudMlV1VersionState::Deleting,
                "FAILED" => GoogleCloudMlV1VersionState::Failed,
                "READY" => GoogleCloudMlV1VersionState::Ready,
                "UNKNOWN" => GoogleCloudMlV1VersionState::Unknown,
                "UPDATING" => GoogleCloudMlV1VersionState::Updating,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleCloudMlV1VersionState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudMlV1VersionState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudMlV1VersionState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CREATING" => GoogleCloudMlV1VersionState::Creating,
                "DELETING" => GoogleCloudMlV1VersionState::Deleting,
                "FAILED" => GoogleCloudMlV1VersionState::Failed,
                "READY" => GoogleCloudMlV1VersionState::Ready,
                "UNKNOWN" => GoogleCloudMlV1VersionState::Unknown,
                "UPDATING" => GoogleCloudMlV1VersionState::Updating,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1VersionState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1VersionState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudMlV1XraiAttribution {
        #[doc = "Number of steps for approximating the path integral.\nA good value to start is 50 and gradually increase until the\nsum to diff property is met within the desired error range."]
        #[serde(
            rename = "numIntegralSteps",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub num_integral_steps: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1XraiAttribution {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1XraiAttribution {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleIamV1AuditConfig {
        #[doc = "The configuration for logging of each type of permission."]
        #[serde(
            rename = "auditLogConfigs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub audit_log_configs:
            ::std::option::Option<Vec<crate::schemas::GoogleIamV1AuditLogConfig>>,
        #[doc = "Specifies a service that will be enabled for audit logging.\nFor example, `storage.googleapis.com`, `cloudsql.googleapis.com`.\n`allServices` is a special value that covers all services."]
        #[serde(
            rename = "service",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub service: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleIamV1AuditConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleIamV1AuditConfig {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleIamV1AuditLogConfig {
        #[doc = "Specifies the identities that do not cause logging for this type of\npermission.\nFollows the same format of Binding.members."]
        #[serde(
            rename = "exemptedMembers",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub exempted_members: ::std::option::Option<Vec<String>>,
        #[doc = "The log type that this config enables."]
        #[serde(
            rename = "logType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub log_type: ::std::option::Option<crate::schemas::GoogleIamV1AuditLogConfigLogType>,
    }
    impl ::google_field_selector::FieldSelector for GoogleIamV1AuditLogConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleIamV1AuditLogConfig {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleIamV1AuditLogConfigLogType {
        #[doc = "Admin reads. Example: CloudIAM getIamPolicy"]
        AdminRead,
        #[doc = "Data reads. Example: CloudSQL Users list"]
        DataRead,
        #[doc = "Data writes. Example: CloudSQL Users create"]
        DataWrite,
        #[doc = "Default case. Should never be this."]
        LogTypeUnspecified,
    }
    impl GoogleIamV1AuditLogConfigLogType {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleIamV1AuditLogConfigLogType::AdminRead => "ADMIN_READ",
                GoogleIamV1AuditLogConfigLogType::DataRead => "DATA_READ",
                GoogleIamV1AuditLogConfigLogType::DataWrite => "DATA_WRITE",
                GoogleIamV1AuditLogConfigLogType::LogTypeUnspecified => "LOG_TYPE_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleIamV1AuditLogConfigLogType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleIamV1AuditLogConfigLogType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<GoogleIamV1AuditLogConfigLogType, ()> {
            Ok(match s {
                "ADMIN_READ" => GoogleIamV1AuditLogConfigLogType::AdminRead,
                "DATA_READ" => GoogleIamV1AuditLogConfigLogType::DataRead,
                "DATA_WRITE" => GoogleIamV1AuditLogConfigLogType::DataWrite,
                "LOG_TYPE_UNSPECIFIED" => GoogleIamV1AuditLogConfigLogType::LogTypeUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleIamV1AuditLogConfigLogType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleIamV1AuditLogConfigLogType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleIamV1AuditLogConfigLogType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ADMIN_READ" => GoogleIamV1AuditLogConfigLogType::AdminRead,
                "DATA_READ" => GoogleIamV1AuditLogConfigLogType::DataRead,
                "DATA_WRITE" => GoogleIamV1AuditLogConfigLogType::DataWrite,
                "LOG_TYPE_UNSPECIFIED" => GoogleIamV1AuditLogConfigLogType::LogTypeUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleIamV1AuditLogConfigLogType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleIamV1AuditLogConfigLogType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleIamV1Binding {
        #[doc = "The condition that is associated with this binding.\n\nIf the condition evaluates to `true`, then this binding applies to the\ncurrent request.\n\nIf the condition evaluates to `false`, then this binding does not apply to\nthe current request. However, a different role binding might grant the same\nrole to one or more of the members in this binding.\n\nTo learn which resources support conditions in their IAM policies, see the\n[IAM\ndocumentation](https://cloud.google.com/iam/help/conditions/resource-policies)."]
        #[serde(
            rename = "condition",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub condition: ::std::option::Option<crate::schemas::GoogleTypeExpr>,
        #[doc = "Specifies the identities requesting access for a Cloud Platform resource.\n`members` can have the following values:\n\n* `allUsers`: A special identifier that represents anyone who is\n  on the internet; with or without a Google account.\n\n* `allAuthenticatedUsers`: A special identifier that represents anyone\n  who is authenticated with a Google account or a service account.\n\n* `user:{emailid}`: An email address that represents a specific Google\n  account. For example, `alice@example.com` .\n\n* `serviceAccount:{emailid}`: An email address that represents a service\n  account. For example, `my-other-app@appspot.gserviceaccount.com`.\n\n* `group:{emailid}`: An email address that represents a Google group.\n  For example, `admins@example.com`.\n\n* `deleted:user:{emailid}?uid={uniqueid}`: An email address (plus unique\n  identifier) representing a user that has been recently deleted. For\n  example, `alice@example.com?uid=123456789012345678901`. If the user is\n  recovered, this value reverts to `user:{emailid}` and the recovered user\n  retains the role in the binding.\n\n* `deleted:serviceAccount:{emailid}?uid={uniqueid}`: An email address (plus\n  unique identifier) representing a service account that has been recently\n  deleted. For example,\n  `my-other-app@appspot.gserviceaccount.com?uid=123456789012345678901`.\n  If the service account is undeleted, this value reverts to\n  `serviceAccount:{emailid}` and the undeleted service account retains the\n  role in the binding.\n\n* `deleted:group:{emailid}?uid={uniqueid}`: An email address (plus unique\n  identifier) representing a Google group that has been recently\n  deleted. For example, `admins@example.com?uid=123456789012345678901`. If\n  the group is recovered, this value reverts to `group:{emailid}` and the\n  recovered group retains the role in the binding.\n\n* `domain:{domain}`: The G Suite domain (primary) that represents all the\n  users of that domain. For example, `google.com` or `example.com`."]
        #[serde(
            rename = "members",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub members: ::std::option::Option<Vec<String>>,
        #[doc = "Role that is assigned to `members`.\nFor example, `roles/viewer`, `roles/editor`, or `roles/owner`."]
        #[serde(
            rename = "role",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub role: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleIamV1Binding {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleIamV1Binding {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleIamV1Policy {
        #[doc = "Specifies cloud audit logging configuration for this policy."]
        #[serde(
            rename = "auditConfigs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub audit_configs: ::std::option::Option<Vec<crate::schemas::GoogleIamV1AuditConfig>>,
        #[doc = "Associates a list of `members` to a `role`. Optionally, may specify a\n`condition` that determines how and when the `bindings` are applied. Each\nof the `bindings` must contain at least one member."]
        #[serde(
            rename = "bindings",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bindings: ::std::option::Option<Vec<crate::schemas::GoogleIamV1Binding>>,
        #[doc = "`etag` is used for optimistic concurrency control as a way to help\nprevent simultaneous updates of a policy from overwriting each other.\nIt is strongly suggested that systems make use of the `etag` in the\nread-modify-write cycle to perform policy updates in order to avoid race\nconditions: An `etag` is returned in the response to `getIamPolicy`, and\nsystems are expected to put that etag in the request to `setIamPolicy` to\nensure that their change will be applied to the same version of the policy.\n\n**Important:** If you use IAM Conditions, you must include the `etag` field\nwhenever you call `setIamPolicy`. If you omit this field, then IAM allows\nyou to overwrite a version `3` policy with a version `1` policy, and all of\nthe conditions in the version `3` policy are lost."]
        #[serde(
            rename = "etag",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub etag: ::std::option::Option<::google_api_bytes::Bytes>,
        #[doc = "Specifies the format of the policy.\n\nValid values are `0`, `1`, and `3`. Requests that specify an invalid value\nare rejected.\n\nAny operation that affects conditional role bindings must specify version\n`3`. This requirement applies to the following operations:\n\n* Getting a policy that includes a conditional role binding\n* Adding a conditional role binding to a policy\n* Changing a conditional role binding in a policy\n* Removing any role binding, with or without a condition, from a policy\n  that includes conditions\n\n**Important:** If you use IAM Conditions, you must include the `etag` field\nwhenever you call `setIamPolicy`. If you omit this field, then IAM allows\nyou to overwrite a version `3` policy with a version `1` policy, and all of\nthe conditions in the version `3` policy are lost.\n\nIf a policy does not include any conditions, operations on that policy may\nspecify any valid version or leave the field unset.\n\nTo learn which resources support conditions in their IAM policies, see the\n[IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies)."]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for GoogleIamV1Policy {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleIamV1Policy {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleIamV1SetIamPolicyRequest {
        #[doc = "REQUIRED: The complete policy to be applied to the `resource`. The size of\nthe policy is limited to a few 10s of KB. An empty policy is a\nvalid policy but certain Cloud Platform services (such as Projects)\nmight reject them."]
        #[serde(
            rename = "policy",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub policy: ::std::option::Option<crate::schemas::GoogleIamV1Policy>,
        #[doc = "OPTIONAL: A FieldMask specifying which fields of the policy to modify. Only\nthe fields in the mask will be modified. If no mask is provided, the\nfollowing default mask is used:\n\n`paths: \"bindings, etag\"`"]
        #[serde(
            rename = "updateMask",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_mask: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleIamV1SetIamPolicyRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleIamV1SetIamPolicyRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleIamV1TestIamPermissionsRequest {
        #[doc = "The set of permissions to check for the `resource`. Permissions with\nwildcards (such as '*' or 'storage.*') are not allowed. For more\ninformation see\n[IAM Overview](https://cloud.google.com/iam/docs/overview#permissions)."]
        #[serde(
            rename = "permissions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub permissions: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleIamV1TestIamPermissionsRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleIamV1TestIamPermissionsRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleIamV1TestIamPermissionsResponse {
        #[doc = "A subset of `TestPermissionsRequest.permissions` that the caller is\nallowed."]
        #[serde(
            rename = "permissions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub permissions: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleIamV1TestIamPermissionsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleIamV1TestIamPermissionsResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleLongrunningListOperationsResponse {
        #[doc = "The standard List next-page token."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "A list of operations that matches the specified filter in the request."]
        #[serde(
            rename = "operations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub operations: ::std::option::Option<Vec<crate::schemas::GoogleLongrunningOperation>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleLongrunningListOperationsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleLongrunningListOperationsResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleLongrunningOperation {
        #[doc = "If the value is `false`, it means the operation is still in progress.\nIf `true`, the operation is completed, and either `error` or `response` is\navailable."]
        #[serde(
            rename = "done",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub done: ::std::option::Option<bool>,
        #[doc = "The error result of the operation in case of failure or cancellation."]
        #[serde(
            rename = "error",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub error: ::std::option::Option<crate::schemas::GoogleRpcStatus>,
        #[doc = "Service-specific metadata associated with the operation.  It typically\ncontains progress information and common metadata such as create time.\nSome services might not provide such metadata.  Any method that returns a\nlong-running operation should document the metadata type, if any."]
        #[serde(
            rename = "metadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metadata:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "The server-assigned name, which is only unique within the same service that\noriginally returns it. If you use the default HTTP mapping, the\n`name` should be a resource name ending with `operations/{unique_id}`."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The normal response of the operation in case of success.  If the original\nmethod returns no data on success, such as `Delete`, the response is\n`google.protobuf.Empty`.  If the original method is standard\n`Get`/`Create`/`Update`, the response should be the resource.  For other\nmethods, the response should have the type `XxxResponse`, where `Xxx`\nis the original method name.  For example, if the original method name\nis `TakeSnapshot()`, the inferred response type is\n`TakeSnapshotResponse`."]
        #[serde(
            rename = "response",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub response:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleLongrunningOperation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleLongrunningOperation {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleProtobufEmpty {}
    impl ::google_field_selector::FieldSelector for GoogleProtobufEmpty {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleProtobufEmpty {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleRpcStatus {
        #[doc = "The status code, which should be an enum value of google.rpc.Code."]
        #[serde(
            rename = "code",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub code: ::std::option::Option<i32>,
        #[doc = "A list of messages that carry the error details.  There is a common set of\nmessage types for APIs to use."]
        #[serde(
            rename = "details",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub details:
            ::std::option::Option<Vec<::std::collections::BTreeMap<String, ::serde_json::Value>>>,
        #[doc = "A developer-facing error message, which should be in English. Any\nuser-facing error message should be localized and sent in the\ngoogle.rpc.Status.details field, or localized by the client."]
        #[serde(
            rename = "message",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub message: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleRpcStatus {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleRpcStatus {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleTypeExpr {
        #[doc = "Optional. Description of the expression. This is a longer text which\ndescribes the expression, e.g. when hovered over it in a UI."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "Textual representation of an expression in Common Expression Language\nsyntax."]
        #[serde(
            rename = "expression",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub expression: ::std::option::Option<String>,
        #[doc = "Optional. String indicating the location of the expression for error\nreporting, e.g. a file name and a position in the file."]
        #[serde(
            rename = "location",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub location: ::std::option::Option<String>,
        #[doc = "Optional. Title for the expression, i.e. a short string describing\nits purpose. This can be used e.g. in UIs which allow to enter the\nexpression."]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleTypeExpr {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleTypeExpr {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
}
pub mod params {
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum Alt {
        #[doc = "Responses with Content-Type of application/json"]
        Json,
        #[doc = "Media download with context-dependent Content-Type"]
        Media,
        #[doc = "Responses with Content-Type of application/x-protobuf"]
        Proto,
    }
    impl Alt {
        pub fn as_str(self) -> &'static str {
            match self {
                Alt::Json => "json",
                Alt::Media => "media",
                Alt::Proto => "proto",
            }
        }
    }
    impl ::std::convert::AsRef<str> for Alt {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for Alt {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<Alt, ()> {
            Ok(match s {
                "json" => Alt::Json,
                "media" => Alt::Media,
                "proto" => Alt::Proto,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for Alt {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for Alt {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for Alt {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "json" => Alt::Json,
                "media" => Alt::Media,
                "proto" => Alt::Proto,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for Alt {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Alt {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum Xgafv {
        #[doc = "v1 error format"]
        _1,
        #[doc = "v2 error format"]
        _2,
    }
    impl Xgafv {
        pub fn as_str(self) -> &'static str {
            match self {
                Xgafv::_1 => "1",
                Xgafv::_2 => "2",
            }
        }
    }
    impl ::std::convert::AsRef<str> for Xgafv {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for Xgafv {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<Xgafv, ()> {
            Ok(match s {
                "1" => Xgafv::_1,
                "2" => Xgafv::_2,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for Xgafv {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for Xgafv {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for Xgafv {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "1" => Xgafv::_1,
                "2" => Xgafv::_2,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for Xgafv {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Xgafv {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
}
pub struct Client {
    reqwest: ::reqwest::blocking::Client,
    auth: Box<dyn ::google_api_auth::GetAccessToken>,
}
impl Client {
    pub fn new<A>(auth: A) -> Self
    where
        A: Into<Box<dyn ::google_api_auth::GetAccessToken>>,
    {
        Client::with_reqwest_client(
            auth,
            ::reqwest::blocking::Client::builder()
                .timeout(None)
                .build()
                .unwrap(),
        )
    }
    pub fn with_reqwest_client<A>(auth: A, reqwest: ::reqwest::blocking::Client) -> Self
    where
        A: Into<Box<dyn ::google_api_auth::GetAccessToken>>,
    {
        Client {
            reqwest,
            auth: auth.into(),
        }
    }
    fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
        self.auth.as_ref()
    }
    #[doc = "Actions that can be performed on the projects resource"]
    pub fn projects(&self) -> crate::resources::projects::ProjectsActions {
        crate::resources::projects::ProjectsActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
}
pub mod resources {
    pub mod projects {
        pub mod params {}
        pub struct ProjectsActions<'a> {
            pub(crate) reqwest: &'a reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> ProjectsActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Performs explanation on the data in the request.\n\n<div>{% dynamic include \"/ai-platform/includes/___explain-request\" %}</div>"]
            pub fn explain(
                &self,
                request: crate::schemas::GoogleCloudMlV1ExplainRequest,
                name: impl Into<String>,
            ) -> ExplainRequestBuilder {
                ExplainRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    request,
                    access_token: None,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                    name: name.into(),
                }
            }
            #[doc = "Get the service account information associated with your project. You need\nthis information in order to grant the service account permissions for\nthe Google Cloud Storage location where you put your model training code\nfor training the model with Google Cloud Machine Learning."]
            pub fn get_config(&self, name: impl Into<String>) -> GetConfigRequestBuilder {
                GetConfigRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    access_token: None,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                    name: name.into(),
                }
            }
            #[doc = "Performs online prediction on the data in the request.\n\n<div>{% dynamic include \"/ai-platform/includes/___predict-request\" %}</div>"]
            pub fn predict(
                &self,
                request: crate::schemas::GoogleCloudMlV1PredictRequest,
                name: impl Into<String>,
            ) -> PredictRequestBuilder {
                PredictRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    request,
                    access_token: None,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                    name: name.into(),
                }
            }
            #[doc = "Actions that can be performed on the jobs resource"]
            pub fn jobs(&self) -> crate::resources::projects::jobs::JobsActions {
                crate::resources::projects::jobs::JobsActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
            #[doc = "Actions that can be performed on the locations resource"]
            pub fn locations(&self) -> crate::resources::projects::locations::LocationsActions {
                crate::resources::projects::locations::LocationsActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
            #[doc = "Actions that can be performed on the models resource"]
            pub fn models(&self) -> crate::resources::projects::models::ModelsActions {
                crate::resources::projects::models::ModelsActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
            #[doc = "Actions that can be performed on the operations resource"]
            pub fn operations(&self) -> crate::resources::projects::operations::OperationsActions {
                crate::resources::projects::operations::OperationsActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
        }
        #[doc = "Created via [ProjectsActions::explain()](struct.ProjectsActions.html#method.explain)"]
        #[derive(Debug, Clone)]
        pub struct ExplainRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::GoogleCloudMlV1ExplainRequest,
            name: String,
            access_token: Option<String>,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a> ExplainRequestBuilder<'a> {
            #[doc = "OAuth access token."]
            pub fn access_token(mut self, value: impl Into<String>) -> Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "JSONP"]
            pub fn callback(mut self, value: impl Into<String>) -> Self {
                self.callback = Some(value.into());
                self
            }
            #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
            pub fn key(mut self, value: impl Into<String>) -> Self {
                self.key = Some(value.into());
                self
            }
            #[doc = "OAuth 2.0 token for the current user."]
            pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                self.oauth_token = Some(value.into());
                self
            }
            #[doc = "Returns response with indentations and line breaks."]
            pub fn pretty_print(mut self, value: bool) -> Self {
                self.pretty_print = Some(value);
                self
            }
            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
            pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                self.upload_protocol = Some(value.into());
                self
            }
            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
            pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                self.upload_type = Some(value.into());
                self
            }
            #[doc = "V1 error format."]
            pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                self.xgafv = Some(value);
                self
            }
            #[doc = r" Execute the given operation. The fields requested are"]
            #[doc = r" determined by the FieldSelector attribute of the return type."]
            #[doc = r" This allows for flexible and ergonomic partial responses. See"]
            #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
            #[doc = r" are not generic over the return type and deserialize the"]
            #[doc = r" response into an auto-generated struct will all possible"]
            #[doc = r" fields."]
            pub fn execute<T>(self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_with_fields(fields)
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub fn execute_with_default_fields(
                self,
            ) -> Result<crate::schemas::GoogleApiHttpBody, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::GoogleApiHttpBody, crate::Error> {
                self.execute_with_fields(Some("*"))
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub fn execute_with_fields<T, F>(mut self, fields: Option<F>) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute()
            }
            fn _execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path())?;
                let req = req.json(&self.request);
                Ok(crate::error_from_response(req.send()?)?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://ml.googleapis.com/".to_owned();
                output.push_str("v1/");
                {
                    let var_as_str = &self.name;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str(":explain");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("access_token", &self.access_token)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("callback", &self.callback)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                let req = req.query(&[("uploadType", &self.upload_type)]);
                let req = req.query(&[("$.xgafv", &self.xgafv)]);
                let req = req.bearer_auth(
                    self.auth
                        .access_token()
                        .map_err(|err| crate::Error::OAuth2(err))?,
                );
                Ok(req)
            }
        }
        #[doc = "Created via [ProjectsActions::get_config()](struct.ProjectsActions.html#method.get_config)"]
        #[derive(Debug, Clone)]
        pub struct GetConfigRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            name: String,
            access_token: Option<String>,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a> GetConfigRequestBuilder<'a> {
            #[doc = "OAuth access token."]
            pub fn access_token(mut self, value: impl Into<String>) -> Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "JSONP"]
            pub fn callback(mut self, value: impl Into<String>) -> Self {
                self.callback = Some(value.into());
                self
            }
            #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
            pub fn key(mut self, value: impl Into<String>) -> Self {
                self.key = Some(value.into());
                self
            }
            #[doc = "OAuth 2.0 token for the current user."]
            pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                self.oauth_token = Some(value.into());
                self
            }
            #[doc = "Returns response with indentations and line breaks."]
            pub fn pretty_print(mut self, value: bool) -> Self {
                self.pretty_print = Some(value);
                self
            }
            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
            pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                self.upload_protocol = Some(value.into());
                self
            }
            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
            pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                self.upload_type = Some(value.into());
                self
            }
            #[doc = "V1 error format."]
            pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                self.xgafv = Some(value);
                self
            }
            #[doc = r" Execute the given operation. The fields requested are"]
            #[doc = r" determined by the FieldSelector attribute of the return type."]
            #[doc = r" This allows for flexible and ergonomic partial responses. See"]
            #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
            #[doc = r" are not generic over the return type and deserialize the"]
            #[doc = r" response into an auto-generated struct will all possible"]
            #[doc = r" fields."]
            pub fn execute<T>(self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_with_fields(fields)
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub fn execute_with_default_fields(
                self,
            ) -> Result<crate::schemas::GoogleCloudMlV1GetConfigResponse, crate::Error>
            {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::GoogleCloudMlV1GetConfigResponse, crate::Error>
            {
                self.execute_with_fields(Some("*"))
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub fn execute_with_fields<T, F>(mut self, fields: Option<F>) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute()
            }
            fn _execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path())?;
                Ok(crate::error_from_response(req.send()?)?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://ml.googleapis.com/".to_owned();
                output.push_str("v1/");
                {
                    let var_as_str = &self.name;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str(":getConfig");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("access_token", &self.access_token)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("callback", &self.callback)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                let req = req.query(&[("uploadType", &self.upload_type)]);
                let req = req.query(&[("$.xgafv", &self.xgafv)]);
                let req = req.bearer_auth(
                    self.auth
                        .access_token()
                        .map_err(|err| crate::Error::OAuth2(err))?,
                );
                Ok(req)
            }
        }
        #[doc = "Created via [ProjectsActions::predict()](struct.ProjectsActions.html#method.predict)"]
        #[derive(Debug, Clone)]
        pub struct PredictRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::GoogleCloudMlV1PredictRequest,
            name: String,
            access_token: Option<String>,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a> PredictRequestBuilder<'a> {
            #[doc = "OAuth access token."]
            pub fn access_token(mut self, value: impl Into<String>) -> Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "JSONP"]
            pub fn callback(mut self, value: impl Into<String>) -> Self {
                self.callback = Some(value.into());
                self
            }
            #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
            pub fn key(mut self, value: impl Into<String>) -> Self {
                self.key = Some(value.into());
                self
            }
            #[doc = "OAuth 2.0 token for the current user."]
            pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                self.oauth_token = Some(value.into());
                self
            }
            #[doc = "Returns response with indentations and line breaks."]
            pub fn pretty_print(mut self, value: bool) -> Self {
                self.pretty_print = Some(value);
                self
            }
            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
            pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                self.upload_protocol = Some(value.into());
                self
            }
            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
            pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                self.upload_type = Some(value.into());
                self
            }
            #[doc = "V1 error format."]
            pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                self.xgafv = Some(value);
                self
            }
            #[doc = r" Execute the given operation. The fields requested are"]
            #[doc = r" determined by the FieldSelector attribute of the return type."]
            #[doc = r" This allows for flexible and ergonomic partial responses. See"]
            #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
            #[doc = r" are not generic over the return type and deserialize the"]
            #[doc = r" response into an auto-generated struct will all possible"]
            #[doc = r" fields."]
            pub fn execute<T>(self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_with_fields(fields)
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub fn execute_with_default_fields(
                self,
            ) -> Result<crate::schemas::GoogleApiHttpBody, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::GoogleApiHttpBody, crate::Error> {
                self.execute_with_fields(Some("*"))
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub fn execute_with_fields<T, F>(mut self, fields: Option<F>) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute()
            }
            fn _execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path())?;
                let req = req.json(&self.request);
                Ok(crate::error_from_response(req.send()?)?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://ml.googleapis.com/".to_owned();
                output.push_str("v1/");
                {
                    let var_as_str = &self.name;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str(":predict");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("access_token", &self.access_token)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("callback", &self.callback)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                let req = req.query(&[("uploadType", &self.upload_type)]);
                let req = req.query(&[("$.xgafv", &self.xgafv)]);
                let req = req.bearer_auth(
                    self.auth
                        .access_token()
                        .map_err(|err| crate::Error::OAuth2(err))?,
                );
                Ok(req)
            }
        }
        pub mod jobs {
            pub mod params {}
            pub struct JobsActions<'a> {
                pub(crate) reqwest: &'a reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> JobsActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Cancels a running job."]
                pub fn cancel(
                    &self,
                    request: crate::schemas::GoogleCloudMlV1CancelJobRequest,
                    name: impl Into<String>,
                ) -> CancelRequestBuilder {
                    CancelRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                        request,
                        access_token: None,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        name: name.into(),
                    }
                }
                #[doc = "Creates a training or a batch prediction job."]
                pub fn create(
                    &self,
                    request: crate::schemas::GoogleCloudMlV1Job,
                    parent: impl Into<String>,
                ) -> CreateRequestBuilder {
                    CreateRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                        request,
                        access_token: None,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        parent: parent.into(),
                    }
                }
                #[doc = "Describes a job."]
                pub fn get(&self, name: impl Into<String>) -> GetRequestBuilder {
                    GetRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                        access_token: None,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        name: name.into(),
                    }
                }
                #[doc = "Gets the access control policy for a resource.\nReturns an empty policy if the resource exists and does not have a policy\nset."]
                pub fn get_iam_policy(
                    &self,
                    resource: impl Into<String>,
                ) -> GetIamPolicyRequestBuilder {
                    GetIamPolicyRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                        access_token: None,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        resource: resource.into(),
                        options_requested_policy_version: None,
                    }
                }
                #[doc = "Lists the jobs in the project.\n\nIf there are no jobs that match the request parameters, the list\nrequest returns an empty response body: {}."]
                pub fn list(&self, parent: impl Into<String>) -> ListRequestBuilder {
                    ListRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                        access_token: None,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        parent: parent.into(),
                        filter: None,
                        page_size: None,
                        page_token: None,
                    }
                }
                #[doc = "Updates a specific job resource.\n\nCurrently the only supported fields to update are `labels`."]
                pub fn patch(
                    &self,
                    request: crate::schemas::GoogleCloudMlV1Job,
                    name: impl Into<String>,
                ) -> PatchRequestBuilder {
                    PatchRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                        request,
                        access_token: None,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        name: name.into(),
                        update_mask: None,
                    }
                }
                #[doc = "Sets the access control policy on the specified resource. Replaces any\nexisting policy.\n\nCan return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors."]
                pub fn set_iam_policy(
                    &self,
                    request: crate::schemas::GoogleIamV1SetIamPolicyRequest,
                    resource: impl Into<String>,
                ) -> SetIamPolicyRequestBuilder {
                    SetIamPolicyRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                        request,
                        access_token: None,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        resource: resource.into(),
                    }
                }
                #[doc = "Returns permissions that a caller has on the specified resource.\nIf the resource does not exist, this will return an empty set of\npermissions, not a `NOT_FOUND` error.\n\nNote: This operation is designed to be used for building permission-aware\nUIs and command-line tools, not for authorization checking. This operation\nmay \"fail open\" without warning."]
                pub fn test_iam_permissions(
                    &self,
                    request: crate::schemas::GoogleIamV1TestIamPermissionsRequest,
                    resource: impl Into<String>,
                ) -> TestIamPermissionsRequestBuilder {
                    TestIamPermissionsRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                        request,
                        access_token: None,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        resource: resource.into(),
                    }
                }
            }
            #[doc = "Created via [JobsActions::cancel()](struct.JobsActions.html#method.cancel)"]
            #[derive(Debug, Clone)]
            pub struct CancelRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::GoogleCloudMlV1CancelJobRequest,
                name: String,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a> CancelRequestBuilder<'a> {
                #[doc = "OAuth access token."]
                pub fn access_token(mut self, value: impl Into<String>) -> Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(mut self, value: impl Into<String>) -> Self {
                    self.callback = Some(value.into());
                    self
                }
                #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                pub fn key(mut self, value: impl Into<String>) -> Self {
                    self.key = Some(value.into());
                    self
                }
                #[doc = "OAuth 2.0 token for the current user."]
                pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                    self.oauth_token = Some(value.into());
                    self
                }
                #[doc = "Returns response with indentations and line breaks."]
                pub fn pretty_print(mut self, value: bool) -> Self {
                    self.pretty_print = Some(value);
                    self
                }
                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                    self.xgafv = Some(value);
                    self
                }
                #[doc = r" Execute the given operation. The fields requested are"]
                #[doc = r" determined by the FieldSelector attribute of the return type."]
                #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                #[doc = r" are not generic over the return type and deserialize the"]
                #[doc = r" response into an auto-generated struct will all possible"]
                #[doc = r" fields."]
                pub fn execute<T>(self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_with_fields(fields)
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub fn execute_with_default_fields(
                    self,
                ) -> Result<crate::schemas::GoogleProtobufEmpty, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GoogleProtobufEmpty, crate::Error> {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute()
                }
                fn _execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path())?;
                    let req = req.json(&self.request);
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://ml.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str(":cancel");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let req = self.reqwest.request(::reqwest::Method::POST, path);
                    let req = req.query(&[("access_token", &self.access_token)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("callback", &self.callback)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    let req = req.query(&[("uploadType", &self.upload_type)]);
                    let req = req.query(&[("$.xgafv", &self.xgafv)]);
                    let req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
                    Ok(req)
                }
            }
            #[doc = "Created via [JobsActions::create()](struct.JobsActions.html#method.create)"]
            #[derive(Debug, Clone)]
            pub struct CreateRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::GoogleCloudMlV1Job,
                parent: String,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a> CreateRequestBuilder<'a> {
                #[doc = "OAuth access token."]
                pub fn access_token(mut self, value: impl Into<String>) -> Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(mut self, value: impl Into<String>) -> Self {
                    self.callback = Some(value.into());
                    self
                }
                #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                pub fn key(mut self, value: impl Into<String>) -> Self {
                    self.key = Some(value.into());
                    self
                }
                #[doc = "OAuth 2.0 token for the current user."]
                pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                    self.oauth_token = Some(value.into());
                    self
                }
                #[doc = "Returns response with indentations and line breaks."]
                pub fn pretty_print(mut self, value: bool) -> Self {
                    self.pretty_print = Some(value);
                    self
                }
                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                    self.xgafv = Some(value);
                    self
                }
                #[doc = r" Execute the given operation. The fields requested are"]
                #[doc = r" determined by the FieldSelector attribute of the return type."]
                #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                #[doc = r" are not generic over the return type and deserialize the"]
                #[doc = r" response into an auto-generated struct will all possible"]
                #[doc = r" fields."]
                pub fn execute<T>(self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_with_fields(fields)
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub fn execute_with_default_fields(
                    self,
                ) -> Result<crate::schemas::GoogleCloudMlV1Job, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GoogleCloudMlV1Job, crate::Error> {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute()
                }
                fn _execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path())?;
                    let req = req.json(&self.request);
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://ml.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/jobs");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let req = self.reqwest.request(::reqwest::Method::POST, path);
                    let req = req.query(&[("access_token", &self.access_token)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("callback", &self.callback)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    let req = req.query(&[("uploadType", &self.upload_type)]);
                    let req = req.query(&[("$.xgafv", &self.xgafv)]);
                    let req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
                    Ok(req)
                }
            }
            #[doc = "Created via [JobsActions::get()](struct.JobsActions.html#method.get)"]
            #[derive(Debug, Clone)]
            pub struct GetRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                name: String,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a> GetRequestBuilder<'a> {
                #[doc = "OAuth access token."]
                pub fn access_token(mut self, value: impl Into<String>) -> Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(mut self, value: impl Into<String>) -> Self {
                    self.callback = Some(value.into());
                    self
                }
                #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                pub fn key(mut self, value: impl Into<String>) -> Self {
                    self.key = Some(value.into());
                    self
                }
                #[doc = "OAuth 2.0 token for the current user."]
                pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                    self.oauth_token = Some(value.into());
                    self
                }
                #[doc = "Returns response with indentations and line breaks."]
                pub fn pretty_print(mut self, value: bool) -> Self {
                    self.pretty_print = Some(value);
                    self
                }
                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                    self.xgafv = Some(value);
                    self
                }
                #[doc = r" Execute the given operation. The fields requested are"]
                #[doc = r" determined by the FieldSelector attribute of the return type."]
                #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                #[doc = r" are not generic over the return type and deserialize the"]
                #[doc = r" response into an auto-generated struct will all possible"]
                #[doc = r" fields."]
                pub fn execute<T>(self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_with_fields(fields)
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub fn execute_with_default_fields(
                    self,
                ) -> Result<crate::schemas::GoogleCloudMlV1Job, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GoogleCloudMlV1Job, crate::Error> {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute()
                }
                fn _execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path())?;
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://ml.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("access_token", &self.access_token)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("callback", &self.callback)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    let req = req.query(&[("uploadType", &self.upload_type)]);
                    let req = req.query(&[("$.xgafv", &self.xgafv)]);
                    let req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
                    Ok(req)
                }
            }
            #[doc = "Created via [JobsActions::get_iam_policy()](struct.JobsActions.html#method.get_iam_policy)"]
            #[derive(Debug, Clone)]
            pub struct GetIamPolicyRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                resource: String,
                options_requested_policy_version: Option<i32>,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a> GetIamPolicyRequestBuilder<'a> {
                #[doc = "Optional. The policy format version to be returned.\n\nValid values are 0, 1, and 3. Requests specifying an invalid value will be\nrejected.\n\nRequests for policies with any conditional bindings must specify version 3.\nPolicies without any conditional bindings may specify any valid value or\nleave the field unset.\n\nTo learn which resources support conditions in their IAM policies, see the\n[IAM\ndocumentation](https://cloud.google.com/iam/help/conditions/resource-policies)."]
                pub fn options_requested_policy_version(mut self, value: i32) -> Self {
                    self.options_requested_policy_version = Some(value);
                    self
                }
                #[doc = "OAuth access token."]
                pub fn access_token(mut self, value: impl Into<String>) -> Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(mut self, value: impl Into<String>) -> Self {
                    self.callback = Some(value.into());
                    self
                }
                #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                pub fn key(mut self, value: impl Into<String>) -> Self {
                    self.key = Some(value.into());
                    self
                }
                #[doc = "OAuth 2.0 token for the current user."]
                pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                    self.oauth_token = Some(value.into());
                    self
                }
                #[doc = "Returns response with indentations and line breaks."]
                pub fn pretty_print(mut self, value: bool) -> Self {
                    self.pretty_print = Some(value);
                    self
                }
                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                    self.xgafv = Some(value);
                    self
                }
                #[doc = r" Execute the given operation. The fields requested are"]
                #[doc = r" determined by the FieldSelector attribute of the return type."]
                #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                #[doc = r" are not generic over the return type and deserialize the"]
                #[doc = r" response into an auto-generated struct will all possible"]
                #[doc = r" fields."]
                pub fn execute<T>(self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_with_fields(fields)
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub fn execute_with_default_fields(
                    self,
                ) -> Result<crate::schemas::GoogleIamV1Policy, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GoogleIamV1Policy, crate::Error> {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute()
                }
                fn _execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path())?;
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://ml.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.resource;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str(":getIamPolicy");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[(
                        "options.requestedPolicyVersion",
                        &self.options_requested_policy_version,
                    )]);
                    let req = req.query(&[("access_token", &self.access_token)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("callback", &self.callback)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    let req = req.query(&[("uploadType", &self.upload_type)]);
                    let req = req.query(&[("$.xgafv", &self.xgafv)]);
                    let req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
                    Ok(req)
                }
            }
            #[doc = "Created via [JobsActions::list()](struct.JobsActions.html#method.list)"]
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                parent: String,
                filter: Option<String>,
                page_size: Option<i32>,
                page_token: Option<String>,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a> ListRequestBuilder<'a> {
                #[doc = "Optional. Specifies the subset of jobs to retrieve.\nYou can filter on the value of one or more attributes of the job object.\nFor example, retrieve jobs with a job identifier that starts with 'census':\n\n<p><code>gcloud ai-platform jobs list --filter='jobId:census*'</code>\n<p>List all failed jobs with names that start with 'rnn':\n<p><code>gcloud ai-platform jobs list --filter='jobId:rnn*\nAND state:FAILED'</code>\n<p>For more examples, see the guide to\n<a href=\"/ml-engine/docs/tensorflow/monitor-training\">monitoring jobs</a>."]
                pub fn filter(mut self, value: impl Into<String>) -> Self {
                    self.filter = Some(value.into());
                    self
                }
                #[doc = "Optional. The number of jobs to retrieve per \"page\" of results. If there\nare more remaining results than this number, the response message will\ncontain a valid value in the `next_page_token` field.\n\nThe default value is 20, and the maximum page size is 100."]
                pub fn page_size(mut self, value: i32) -> Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "Optional. A page token to request the next page of results.\n\nYou get the token from the `next_page_token` field of the response from\nthe previous call."]
                pub fn page_token(mut self, value: impl Into<String>) -> Self {
                    self.page_token = Some(value.into());
                    self
                }
                #[doc = "OAuth access token."]
                pub fn access_token(mut self, value: impl Into<String>) -> Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(mut self, value: impl Into<String>) -> Self {
                    self.callback = Some(value.into());
                    self
                }
                #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                pub fn key(mut self, value: impl Into<String>) -> Self {
                    self.key = Some(value.into());
                    self
                }
                #[doc = "OAuth 2.0 token for the current user."]
                pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                    self.oauth_token = Some(value.into());
                    self
                }
                #[doc = "Returns response with indentations and line breaks."]
                pub fn pretty_print(mut self, value: bool) -> Self {
                    self.pretty_print = Some(value);
                    self
                }
                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                    self.xgafv = Some(value);
                    self
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are chosen by the caller of this"]
                #[doc = r" method and must implement `Deserialize` and `FieldSelector`. The"]
                #[doc = r" populated fields in the yielded items will be determined by the"]
                #[doc = r" `FieldSelector` implementation."]
                pub fn iter_jobs<T>(self) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.iter_jobs_with_fields(fields)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be the default fields populated by"]
                #[doc = r" the server."]
                pub fn iter_jobs_with_default_fields(
                    self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::GoogleCloudMlV1Job>
                {
                    self.iter_jobs_with_fields(None::<String>)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be all fields available. This should"]
                #[doc = r" primarily be used during developement and debugging as fetching"]
                #[doc = r" all fields can be expensive both in bandwidth and server"]
                #[doc = r" resources."]
                pub fn iter_jobs_with_all_fields(
                    self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::GoogleCloudMlV1Job>
                {
                    self.iter_jobs_with_fields(Some("*"))
                }
                pub fn iter_jobs_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: AsRef<str>,
                {
                    self.fields = Some({
                        let mut selector = concat!("nextPageToken,", "jobs").to_owned();
                        let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                        if !items_fields.is_empty() {
                            selector.push_str("(");
                            selector.push_str(items_fields);
                            selector.push_str(")");
                        }
                        selector
                    });
                    crate::iter::PageItemIter::new(self, "jobs")
                }
                pub fn iter<T>(self) -> crate::iter::PageIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.iter_with_fields(fields)
                }
                pub fn iter_with_default_fields(
                    self,
                ) -> crate::iter::PageIter<Self, crate::schemas::GoogleCloudMlV1ListJobsResponse>
                {
                    self.iter_with_fields(None::<&str>)
                }
                pub fn iter_with_all_fields(
                    self,
                ) -> crate::iter::PageIter<Self, crate::schemas::GoogleCloudMlV1ListJobsResponse>
                {
                    self.iter_with_fields(Some("*"))
                }
                pub fn iter_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> crate::iter::PageIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: AsRef<str>,
                {
                    let mut fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("").to_owned();
                    if !fields.is_empty() {
                        match fields.chars().rev().nth(0) {
                            Some(',') | None => {}
                            _ => fields.push_str(","),
                        }
                        fields.push_str("nextPageToken");
                        self.fields = Some(fields);
                    }
                    crate::iter::PageIter::new(self)
                }
                #[doc = r" Execute the given operation. The fields requested are"]
                #[doc = r" determined by the FieldSelector attribute of the return type."]
                #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                #[doc = r" are not generic over the return type and deserialize the"]
                #[doc = r" response into an auto-generated struct will all possible"]
                #[doc = r" fields."]
                pub fn execute<T>(self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_with_fields(fields)
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub fn execute_with_default_fields(
                    self,
                ) -> Result<crate::schemas::GoogleCloudMlV1ListJobsResponse, crate::Error>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GoogleCloudMlV1ListJobsResponse, crate::Error>
                {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute()
                }
                fn _execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path())?;
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://ml.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/jobs");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("filter", &self.filter)]);
                    let req = req.query(&[("pageSize", &self.page_size)]);
                    let req = req.query(&[("pageToken", &self.page_token)]);
                    let req = req.query(&[("access_token", &self.access_token)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("callback", &self.callback)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    let req = req.query(&[("uploadType", &self.upload_type)]);
                    let req = req.query(&[("$.xgafv", &self.xgafv)]);
                    let req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
                    Ok(req)
                }
            }
            impl<'a> crate::iter::IterableMethod for ListRequestBuilder<'a> {
                fn set_page_token(&mut self, value: String) {
                    self.page_token = value.into();
                }
                fn execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    self._execute()
                }
            }
            #[doc = "Created via [JobsActions::patch()](struct.JobsActions.html#method.patch)"]
            #[derive(Debug, Clone)]
            pub struct PatchRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::GoogleCloudMlV1Job,
                name: String,
                update_mask: Option<String>,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a> PatchRequestBuilder<'a> {
                #[doc = "Required. Specifies the path, relative to `Job`, of the field to update.\nTo adopt etag mechanism, include `etag` field in the mask, and include the\n`etag` value in your job resource.\n\nFor example, to change the labels of a job, the `update_mask` parameter\nwould be specified as `labels`, `etag`, and the\n`PATCH` request body would specify the new value, as follows:\n{\n\"labels\": {\n\"owner\": \"Google\",\n\"color\": \"Blue\"\n}\n\"etag\": \"33a64df551425fcc55e4d42a148795d9f25f89d4\"\n}\nIf `etag` matches the one on the server, the labels of the job will be\nreplaced with the given ones, and the server end `etag` will be\nrecalculated.\n\nCurrently the only supported update masks are `labels` and `etag`."]
                pub fn update_mask(mut self, value: impl Into<String>) -> Self {
                    self.update_mask = Some(value.into());
                    self
                }
                #[doc = "OAuth access token."]
                pub fn access_token(mut self, value: impl Into<String>) -> Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(mut self, value: impl Into<String>) -> Self {
                    self.callback = Some(value.into());
                    self
                }
                #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                pub fn key(mut self, value: impl Into<String>) -> Self {
                    self.key = Some(value.into());
                    self
                }
                #[doc = "OAuth 2.0 token for the current user."]
                pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                    self.oauth_token = Some(value.into());
                    self
                }
                #[doc = "Returns response with indentations and line breaks."]
                pub fn pretty_print(mut self, value: bool) -> Self {
                    self.pretty_print = Some(value);
                    self
                }
                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                    self.xgafv = Some(value);
                    self
                }
                #[doc = r" Execute the given operation. The fields requested are"]
                #[doc = r" determined by the FieldSelector attribute of the return type."]
                #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                #[doc = r" are not generic over the return type and deserialize the"]
                #[doc = r" response into an auto-generated struct will all possible"]
                #[doc = r" fields."]
                pub fn execute<T>(self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_with_fields(fields)
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub fn execute_with_default_fields(
                    self,
                ) -> Result<crate::schemas::GoogleCloudMlV1Job, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GoogleCloudMlV1Job, crate::Error> {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute()
                }
                fn _execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path())?;
                    let req = req.json(&self.request);
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://ml.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let req = self.reqwest.request(::reqwest::Method::PATCH, path);
                    let req = req.query(&[("updateMask", &self.update_mask)]);
                    let req = req.query(&[("access_token", &self.access_token)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("callback", &self.callback)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    let req = req.query(&[("uploadType", &self.upload_type)]);
                    let req = req.query(&[("$.xgafv", &self.xgafv)]);
                    let req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
                    Ok(req)
                }
            }
            #[doc = "Created via [JobsActions::set_iam_policy()](struct.JobsActions.html#method.set_iam_policy)"]
            #[derive(Debug, Clone)]
            pub struct SetIamPolicyRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::GoogleIamV1SetIamPolicyRequest,
                resource: String,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a> SetIamPolicyRequestBuilder<'a> {
                #[doc = "OAuth access token."]
                pub fn access_token(mut self, value: impl Into<String>) -> Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(mut self, value: impl Into<String>) -> Self {
                    self.callback = Some(value.into());
                    self
                }
                #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                pub fn key(mut self, value: impl Into<String>) -> Self {
                    self.key = Some(value.into());
                    self
                }
                #[doc = "OAuth 2.0 token for the current user."]
                pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                    self.oauth_token = Some(value.into());
                    self
                }
                #[doc = "Returns response with indentations and line breaks."]
                pub fn pretty_print(mut self, value: bool) -> Self {
                    self.pretty_print = Some(value);
                    self
                }
                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                    self.xgafv = Some(value);
                    self
                }
                #[doc = r" Execute the given operation. The fields requested are"]
                #[doc = r" determined by the FieldSelector attribute of the return type."]
                #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                #[doc = r" are not generic over the return type and deserialize the"]
                #[doc = r" response into an auto-generated struct will all possible"]
                #[doc = r" fields."]
                pub fn execute<T>(self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_with_fields(fields)
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub fn execute_with_default_fields(
                    self,
                ) -> Result<crate::schemas::GoogleIamV1Policy, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GoogleIamV1Policy, crate::Error> {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute()
                }
                fn _execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path())?;
                    let req = req.json(&self.request);
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://ml.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.resource;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str(":setIamPolicy");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let req = self.reqwest.request(::reqwest::Method::POST, path);
                    let req = req.query(&[("access_token", &self.access_token)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("callback", &self.callback)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    let req = req.query(&[("uploadType", &self.upload_type)]);
                    let req = req.query(&[("$.xgafv", &self.xgafv)]);
                    let req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
                    Ok(req)
                }
            }
            #[doc = "Created via [JobsActions::test_iam_permissions()](struct.JobsActions.html#method.test_iam_permissions)"]
            #[derive(Debug, Clone)]
            pub struct TestIamPermissionsRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::GoogleIamV1TestIamPermissionsRequest,
                resource: String,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a> TestIamPermissionsRequestBuilder<'a> {
                #[doc = "OAuth access token."]
                pub fn access_token(mut self, value: impl Into<String>) -> Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(mut self, value: impl Into<String>) -> Self {
                    self.callback = Some(value.into());
                    self
                }
                #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                pub fn key(mut self, value: impl Into<String>) -> Self {
                    self.key = Some(value.into());
                    self
                }
                #[doc = "OAuth 2.0 token for the current user."]
                pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                    self.oauth_token = Some(value.into());
                    self
                }
                #[doc = "Returns response with indentations and line breaks."]
                pub fn pretty_print(mut self, value: bool) -> Self {
                    self.pretty_print = Some(value);
                    self
                }
                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                    self.xgafv = Some(value);
                    self
                }
                #[doc = r" Execute the given operation. The fields requested are"]
                #[doc = r" determined by the FieldSelector attribute of the return type."]
                #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                #[doc = r" are not generic over the return type and deserialize the"]
                #[doc = r" response into an auto-generated struct will all possible"]
                #[doc = r" fields."]
                pub fn execute<T>(self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_with_fields(fields)
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub fn execute_with_default_fields(
                    self,
                ) -> Result<crate::schemas::GoogleIamV1TestIamPermissionsResponse, crate::Error>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GoogleIamV1TestIamPermissionsResponse, crate::Error>
                {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute()
                }
                fn _execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path())?;
                    let req = req.json(&self.request);
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://ml.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.resource;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str(":testIamPermissions");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let req = self.reqwest.request(::reqwest::Method::POST, path);
                    let req = req.query(&[("access_token", &self.access_token)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("callback", &self.callback)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    let req = req.query(&[("uploadType", &self.upload_type)]);
                    let req = req.query(&[("$.xgafv", &self.xgafv)]);
                    let req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
                    Ok(req)
                }
            }
        }
        pub mod locations {
            pub mod params {}
            pub struct LocationsActions<'a> {
                pub(crate) reqwest: &'a reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> LocationsActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Get the complete list of CMLE capabilities in a location, along with their\nlocation-specific properties."]
                pub fn get(&self, name: impl Into<String>) -> GetRequestBuilder {
                    GetRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                        access_token: None,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        name: name.into(),
                    }
                }
                #[doc = "List all locations that provides at least one type of CMLE capability."]
                pub fn list(&self, parent: impl Into<String>) -> ListRequestBuilder {
                    ListRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                        access_token: None,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        parent: parent.into(),
                        page_size: None,
                        page_token: None,
                    }
                }
                #[doc = "Actions that can be performed on the operations resource"]
                pub fn operations(
                    &self,
                ) -> crate::resources::projects::locations::operations::OperationsActions
                {
                    crate::resources::projects::locations::operations::OperationsActions {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                    }
                }
                #[doc = "Actions that can be performed on the studies resource"]
                pub fn studies(
                    &self,
                ) -> crate::resources::projects::locations::studies::StudiesActions
                {
                    crate::resources::projects::locations::studies::StudiesActions {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                    }
                }
            }
            #[doc = "Created via [LocationsActions::get()](struct.LocationsActions.html#method.get)"]
            #[derive(Debug, Clone)]
            pub struct GetRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                name: String,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a> GetRequestBuilder<'a> {
                #[doc = "OAuth access token."]
                pub fn access_token(mut self, value: impl Into<String>) -> Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(mut self, value: impl Into<String>) -> Self {
                    self.callback = Some(value.into());
                    self
                }
                #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                pub fn key(mut self, value: impl Into<String>) -> Self {
                    self.key = Some(value.into());
                    self
                }
                #[doc = "OAuth 2.0 token for the current user."]
                pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                    self.oauth_token = Some(value.into());
                    self
                }
                #[doc = "Returns response with indentations and line breaks."]
                pub fn pretty_print(mut self, value: bool) -> Self {
                    self.pretty_print = Some(value);
                    self
                }
                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                    self.xgafv = Some(value);
                    self
                }
                #[doc = r" Execute the given operation. The fields requested are"]
                #[doc = r" determined by the FieldSelector attribute of the return type."]
                #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                #[doc = r" are not generic over the return type and deserialize the"]
                #[doc = r" response into an auto-generated struct will all possible"]
                #[doc = r" fields."]
                pub fn execute<T>(self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_with_fields(fields)
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub fn execute_with_default_fields(
                    self,
                ) -> Result<crate::schemas::GoogleCloudMlV1Location, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GoogleCloudMlV1Location, crate::Error> {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute()
                }
                fn _execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path())?;
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://ml.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("access_token", &self.access_token)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("callback", &self.callback)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    let req = req.query(&[("uploadType", &self.upload_type)]);
                    let req = req.query(&[("$.xgafv", &self.xgafv)]);
                    let req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
                    Ok(req)
                }
            }
            #[doc = "Created via [LocationsActions::list()](struct.LocationsActions.html#method.list)"]
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                parent: String,
                page_size: Option<i32>,
                page_token: Option<String>,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a> ListRequestBuilder<'a> {
                #[doc = "Optional. The number of locations to retrieve per \"page\" of results. If\nthere are more remaining results than this number, the response message\nwill contain a valid value in the `next_page_token` field.\n\nThe default value is 20, and the maximum page size is 100."]
                pub fn page_size(mut self, value: i32) -> Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "Optional. A page token to request the next page of results.\n\nYou get the token from the `next_page_token` field of the response from\nthe previous call."]
                pub fn page_token(mut self, value: impl Into<String>) -> Self {
                    self.page_token = Some(value.into());
                    self
                }
                #[doc = "OAuth access token."]
                pub fn access_token(mut self, value: impl Into<String>) -> Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(mut self, value: impl Into<String>) -> Self {
                    self.callback = Some(value.into());
                    self
                }
                #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                pub fn key(mut self, value: impl Into<String>) -> Self {
                    self.key = Some(value.into());
                    self
                }
                #[doc = "OAuth 2.0 token for the current user."]
                pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                    self.oauth_token = Some(value.into());
                    self
                }
                #[doc = "Returns response with indentations and line breaks."]
                pub fn pretty_print(mut self, value: bool) -> Self {
                    self.pretty_print = Some(value);
                    self
                }
                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                    self.xgafv = Some(value);
                    self
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are chosen by the caller of this"]
                #[doc = r" method and must implement `Deserialize` and `FieldSelector`. The"]
                #[doc = r" populated fields in the yielded items will be determined by the"]
                #[doc = r" `FieldSelector` implementation."]
                pub fn iter_locations<T>(self) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.iter_locations_with_fields(fields)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be the default fields populated by"]
                #[doc = r" the server."]
                pub fn iter_locations_with_default_fields(
                    self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::GoogleCloudMlV1Location>
                {
                    self.iter_locations_with_fields(None::<String>)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be all fields available. This should"]
                #[doc = r" primarily be used during developement and debugging as fetching"]
                #[doc = r" all fields can be expensive both in bandwidth and server"]
                #[doc = r" resources."]
                pub fn iter_locations_with_all_fields(
                    self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::GoogleCloudMlV1Location>
                {
                    self.iter_locations_with_fields(Some("*"))
                }
                pub fn iter_locations_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: AsRef<str>,
                {
                    self.fields = Some({
                        let mut selector = concat!("nextPageToken,", "locations").to_owned();
                        let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                        if !items_fields.is_empty() {
                            selector.push_str("(");
                            selector.push_str(items_fields);
                            selector.push_str(")");
                        }
                        selector
                    });
                    crate::iter::PageItemIter::new(self, "locations")
                }
                pub fn iter<T>(self) -> crate::iter::PageIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.iter_with_fields(fields)
                }
                pub fn iter_with_default_fields(
                    self,
                ) -> crate::iter::PageIter<Self, crate::schemas::GoogleCloudMlV1ListLocationsResponse>
                {
                    self.iter_with_fields(None::<&str>)
                }
                pub fn iter_with_all_fields(
                    self,
                ) -> crate::iter::PageIter<Self, crate::schemas::GoogleCloudMlV1ListLocationsResponse>
                {
                    self.iter_with_fields(Some("*"))
                }
                pub fn iter_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> crate::iter::PageIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: AsRef<str>,
                {
                    let mut fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("").to_owned();
                    if !fields.is_empty() {
                        match fields.chars().rev().nth(0) {
                            Some(',') | None => {}
                            _ => fields.push_str(","),
                        }
                        fields.push_str("nextPageToken");
                        self.fields = Some(fields);
                    }
                    crate::iter::PageIter::new(self)
                }
                #[doc = r" Execute the given operation. The fields requested are"]
                #[doc = r" determined by the FieldSelector attribute of the return type."]
                #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                #[doc = r" are not generic over the return type and deserialize the"]
                #[doc = r" response into an auto-generated struct will all possible"]
                #[doc = r" fields."]
                pub fn execute<T>(self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_with_fields(fields)
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub fn execute_with_default_fields(
                    self,
                ) -> Result<crate::schemas::GoogleCloudMlV1ListLocationsResponse, crate::Error>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GoogleCloudMlV1ListLocationsResponse, crate::Error>
                {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute()
                }
                fn _execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path())?;
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://ml.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/locations");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("pageSize", &self.page_size)]);
                    let req = req.query(&[("pageToken", &self.page_token)]);
                    let req = req.query(&[("access_token", &self.access_token)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("callback", &self.callback)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    let req = req.query(&[("uploadType", &self.upload_type)]);
                    let req = req.query(&[("$.xgafv", &self.xgafv)]);
                    let req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
                    Ok(req)
                }
            }
            impl<'a> crate::iter::IterableMethod for ListRequestBuilder<'a> {
                fn set_page_token(&mut self, value: String) {
                    self.page_token = value.into();
                }
                fn execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    self._execute()
                }
            }
            pub mod operations {
                pub mod params {}
                pub struct OperationsActions<'a> {
                    pub(crate) reqwest: &'a reqwest::blocking::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                }
                impl<'a> OperationsActions<'a> {
                    fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                        self.auth
                    }
                    #[doc = "Starts asynchronous cancellation on a long-running operation.  The server\nmakes a best effort to cancel the operation, but success is not\nguaranteed.  If the server doesn't support this method, it returns\n`google.rpc.Code.UNIMPLEMENTED`.  Clients can use\nOperations.GetOperation or\nother methods to check whether the cancellation succeeded or whether the\noperation completed despite cancellation. On successful cancellation,\nthe operation is not deleted; instead, it becomes an operation with\nan Operation.error value with a google.rpc.Status.code of 1,\ncorresponding to `Code.CANCELLED`."]
                    pub fn cancel(&self, name: impl Into<String>) -> CancelRequestBuilder {
                        CancelRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: self.auth_ref(),
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            name: name.into(),
                        }
                    }
                    #[doc = "Gets the latest state of a long-running operation.  Clients can use this\nmethod to poll the operation result at intervals as recommended by the API\nservice."]
                    pub fn get(&self, name: impl Into<String>) -> GetRequestBuilder {
                        GetRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: self.auth_ref(),
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            name: name.into(),
                        }
                    }
                }
                #[doc = "Created via [OperationsActions::cancel()](struct.OperationsActions.html#method.cancel)"]
                #[derive(Debug, Clone)]
                pub struct CancelRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    name: String,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a> CancelRequestBuilder<'a> {
                    #[doc = "OAuth access token."]
                    pub fn access_token(mut self, value: impl Into<String>) -> Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(mut self, value: impl Into<String>) -> Self {
                        self.callback = Some(value.into());
                        self
                    }
                    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                    pub fn key(mut self, value: impl Into<String>) -> Self {
                        self.key = Some(value.into());
                        self
                    }
                    #[doc = "OAuth 2.0 token for the current user."]
                    pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                        self.oauth_token = Some(value.into());
                        self
                    }
                    #[doc = "Returns response with indentations and line breaks."]
                    pub fn pretty_print(mut self, value: bool) -> Self {
                        self.pretty_print = Some(value);
                        self
                    }
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                        self.xgafv = Some(value);
                        self
                    }
                    #[doc = r" Execute the given operation. The fields requested are"]
                    #[doc = r" determined by the FieldSelector attribute of the return type."]
                    #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                    #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                    #[doc = r" are not generic over the return type and deserialize the"]
                    #[doc = r" response into an auto-generated struct will all possible"]
                    #[doc = r" fields."]
                    pub fn execute<T>(self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.execute_with_fields(fields)
                    }
                    #[doc = r" Execute the given operation. This will not provide any"]
                    #[doc = r" `fields` selector indicating that the server will determine"]
                    #[doc = r" the fields returned. This typically includes the most common"]
                    #[doc = r" fields, but it will not include every possible attribute of"]
                    #[doc = r" the response resource."]
                    pub fn execute_with_default_fields(
                        self,
                    ) -> Result<crate::schemas::GoogleProtobufEmpty, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::GoogleProtobufEmpty, crate::Error>
                    {
                        self.execute_with_fields(Some("*"))
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub fn execute_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: Into<String>,
                    {
                        self.fields = fields.map(Into::into);
                        self._execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                    {
                        let req = self._request(&self._path())?;
                        Ok(crate::error_from_response(req.send()?)?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://ml.googleapis.com/".to_owned();
                        output.push_str("v1/");
                        {
                            let var_as_str = &self.name;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str(":cancel");
                        output
                    }
                    fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error>
                    {
                        let req = self.reqwest.request(::reqwest::Method::POST, path);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let req = req.bearer_auth(
                            self.auth
                                .access_token()
                                .map_err(|err| crate::Error::OAuth2(err))?,
                        );
                        Ok(req)
                    }
                }
                #[doc = "Created via [OperationsActions::get()](struct.OperationsActions.html#method.get)"]
                #[derive(Debug, Clone)]
                pub struct GetRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    name: String,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a> GetRequestBuilder<'a> {
                    #[doc = "OAuth access token."]
                    pub fn access_token(mut self, value: impl Into<String>) -> Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(mut self, value: impl Into<String>) -> Self {
                        self.callback = Some(value.into());
                        self
                    }
                    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                    pub fn key(mut self, value: impl Into<String>) -> Self {
                        self.key = Some(value.into());
                        self
                    }
                    #[doc = "OAuth 2.0 token for the current user."]
                    pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                        self.oauth_token = Some(value.into());
                        self
                    }
                    #[doc = "Returns response with indentations and line breaks."]
                    pub fn pretty_print(mut self, value: bool) -> Self {
                        self.pretty_print = Some(value);
                        self
                    }
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                        self.xgafv = Some(value);
                        self
                    }
                    #[doc = r" Execute the given operation. The fields requested are"]
                    #[doc = r" determined by the FieldSelector attribute of the return type."]
                    #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                    #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                    #[doc = r" are not generic over the return type and deserialize the"]
                    #[doc = r" response into an auto-generated struct will all possible"]
                    #[doc = r" fields."]
                    pub fn execute<T>(self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.execute_with_fields(fields)
                    }
                    #[doc = r" Execute the given operation. This will not provide any"]
                    #[doc = r" `fields` selector indicating that the server will determine"]
                    #[doc = r" the fields returned. This typically includes the most common"]
                    #[doc = r" fields, but it will not include every possible attribute of"]
                    #[doc = r" the response resource."]
                    pub fn execute_with_default_fields(
                        self,
                    ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error>
                    {
                        self.execute_with_fields(Some("*"))
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub fn execute_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: Into<String>,
                    {
                        self.fields = fields.map(Into::into);
                        self._execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                    {
                        let req = self._request(&self._path())?;
                        Ok(crate::error_from_response(req.send()?)?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://ml.googleapis.com/".to_owned();
                        output.push_str("v1/");
                        {
                            let var_as_str = &self.name;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output
                    }
                    fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error>
                    {
                        let req = self.reqwest.request(::reqwest::Method::GET, path);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let req = req.bearer_auth(
                            self.auth
                                .access_token()
                                .map_err(|err| crate::Error::OAuth2(err))?,
                        );
                        Ok(req)
                    }
                }
            }
            pub mod studies {
                pub mod params {}
                pub struct StudiesActions<'a> {
                    pub(crate) reqwest: &'a reqwest::blocking::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                }
                impl<'a> StudiesActions<'a> {
                    fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                        self.auth
                    }
                    #[doc = "Creates a study."]
                    pub fn create(
                        &self,
                        request: crate::schemas::GoogleCloudMlV1Study,
                        parent: impl Into<String>,
                    ) -> CreateRequestBuilder {
                        CreateRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: self.auth_ref(),
                            request,
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            parent: parent.into(),
                            study_id: None,
                        }
                    }
                    #[doc = "Deletes a study."]
                    pub fn delete(&self, name: impl Into<String>) -> DeleteRequestBuilder {
                        DeleteRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: self.auth_ref(),
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            name: name.into(),
                        }
                    }
                    #[doc = "Gets a study."]
                    pub fn get(&self, name: impl Into<String>) -> GetRequestBuilder {
                        GetRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: self.auth_ref(),
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            name: name.into(),
                        }
                    }
                    #[doc = "Lists all the studies in a region for an associated project."]
                    pub fn list(&self, parent: impl Into<String>) -> ListRequestBuilder {
                        ListRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: self.auth_ref(),
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            parent: parent.into(),
                        }
                    }
                    #[doc = "Actions that can be performed on the trials resource"]
                    pub fn trials(
                        &self,
                    ) -> crate::resources::projects::locations::studies::trials::TrialsActions
                    {
                        crate::resources::projects::locations::studies::trials::TrialsActions {
                            reqwest: &self.reqwest,
                            auth: self.auth_ref(),
                        }
                    }
                }
                #[doc = "Created via [StudiesActions::create()](struct.StudiesActions.html#method.create)"]
                #[derive(Debug, Clone)]
                pub struct CreateRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    request: crate::schemas::GoogleCloudMlV1Study,
                    parent: String,
                    study_id: Option<String>,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a> CreateRequestBuilder<'a> {
                    #[doc = "Required. The ID to use for the study, which will become the final component of\nthe study's resource name."]
                    pub fn study_id(mut self, value: impl Into<String>) -> Self {
                        self.study_id = Some(value.into());
                        self
                    }
                    #[doc = "OAuth access token."]
                    pub fn access_token(mut self, value: impl Into<String>) -> Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(mut self, value: impl Into<String>) -> Self {
                        self.callback = Some(value.into());
                        self
                    }
                    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                    pub fn key(mut self, value: impl Into<String>) -> Self {
                        self.key = Some(value.into());
                        self
                    }
                    #[doc = "OAuth 2.0 token for the current user."]
                    pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                        self.oauth_token = Some(value.into());
                        self
                    }
                    #[doc = "Returns response with indentations and line breaks."]
                    pub fn pretty_print(mut self, value: bool) -> Self {
                        self.pretty_print = Some(value);
                        self
                    }
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                        self.xgafv = Some(value);
                        self
                    }
                    #[doc = r" Execute the given operation. The fields requested are"]
                    #[doc = r" determined by the FieldSelector attribute of the return type."]
                    #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                    #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                    #[doc = r" are not generic over the return type and deserialize the"]
                    #[doc = r" response into an auto-generated struct will all possible"]
                    #[doc = r" fields."]
                    pub fn execute<T>(self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.execute_with_fields(fields)
                    }
                    #[doc = r" Execute the given operation. This will not provide any"]
                    #[doc = r" `fields` selector indicating that the server will determine"]
                    #[doc = r" the fields returned. This typically includes the most common"]
                    #[doc = r" fields, but it will not include every possible attribute of"]
                    #[doc = r" the response resource."]
                    pub fn execute_with_default_fields(
                        self,
                    ) -> Result<crate::schemas::GoogleCloudMlV1Study, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::GoogleCloudMlV1Study, crate::Error>
                    {
                        self.execute_with_fields(Some("*"))
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub fn execute_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: Into<String>,
                    {
                        self.fields = fields.map(Into::into);
                        self._execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                    {
                        let req = self._request(&self._path())?;
                        let req = req.json(&self.request);
                        Ok(crate::error_from_response(req.send()?)?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://ml.googleapis.com/".to_owned();
                        output.push_str("v1/");
                        {
                            let var_as_str = &self.parent;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/studies");
                        output
                    }
                    fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error>
                    {
                        let req = self.reqwest.request(::reqwest::Method::POST, path);
                        let req = req.query(&[("studyId", &self.study_id)]);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let req = req.bearer_auth(
                            self.auth
                                .access_token()
                                .map_err(|err| crate::Error::OAuth2(err))?,
                        );
                        Ok(req)
                    }
                }
                #[doc = "Created via [StudiesActions::delete()](struct.StudiesActions.html#method.delete)"]
                #[derive(Debug, Clone)]
                pub struct DeleteRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    name: String,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a> DeleteRequestBuilder<'a> {
                    #[doc = "OAuth access token."]
                    pub fn access_token(mut self, value: impl Into<String>) -> Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(mut self, value: impl Into<String>) -> Self {
                        self.callback = Some(value.into());
                        self
                    }
                    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                    pub fn key(mut self, value: impl Into<String>) -> Self {
                        self.key = Some(value.into());
                        self
                    }
                    #[doc = "OAuth 2.0 token for the current user."]
                    pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                        self.oauth_token = Some(value.into());
                        self
                    }
                    #[doc = "Returns response with indentations and line breaks."]
                    pub fn pretty_print(mut self, value: bool) -> Self {
                        self.pretty_print = Some(value);
                        self
                    }
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                        self.xgafv = Some(value);
                        self
                    }
                    #[doc = r" Execute the given operation. The fields requested are"]
                    #[doc = r" determined by the FieldSelector attribute of the return type."]
                    #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                    #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                    #[doc = r" are not generic over the return type and deserialize the"]
                    #[doc = r" response into an auto-generated struct will all possible"]
                    #[doc = r" fields."]
                    pub fn execute<T>(self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.execute_with_fields(fields)
                    }
                    #[doc = r" Execute the given operation. This will not provide any"]
                    #[doc = r" `fields` selector indicating that the server will determine"]
                    #[doc = r" the fields returned. This typically includes the most common"]
                    #[doc = r" fields, but it will not include every possible attribute of"]
                    #[doc = r" the response resource."]
                    pub fn execute_with_default_fields(
                        self,
                    ) -> Result<crate::schemas::GoogleProtobufEmpty, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::GoogleProtobufEmpty, crate::Error>
                    {
                        self.execute_with_fields(Some("*"))
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub fn execute_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: Into<String>,
                    {
                        self.fields = fields.map(Into::into);
                        self._execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                    {
                        let req = self._request(&self._path())?;
                        Ok(crate::error_from_response(req.send()?)?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://ml.googleapis.com/".to_owned();
                        output.push_str("v1/");
                        {
                            let var_as_str = &self.name;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output
                    }
                    fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error>
                    {
                        let req = self.reqwest.request(::reqwest::Method::DELETE, path);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let req = req.bearer_auth(
                            self.auth
                                .access_token()
                                .map_err(|err| crate::Error::OAuth2(err))?,
                        );
                        Ok(req)
                    }
                }
                #[doc = "Created via [StudiesActions::get()](struct.StudiesActions.html#method.get)"]
                #[derive(Debug, Clone)]
                pub struct GetRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    name: String,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a> GetRequestBuilder<'a> {
                    #[doc = "OAuth access token."]
                    pub fn access_token(mut self, value: impl Into<String>) -> Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(mut self, value: impl Into<String>) -> Self {
                        self.callback = Some(value.into());
                        self
                    }
                    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                    pub fn key(mut self, value: impl Into<String>) -> Self {
                        self.key = Some(value.into());
                        self
                    }
                    #[doc = "OAuth 2.0 token for the current user."]
                    pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                        self.oauth_token = Some(value.into());
                        self
                    }
                    #[doc = "Returns response with indentations and line breaks."]
                    pub fn pretty_print(mut self, value: bool) -> Self {
                        self.pretty_print = Some(value);
                        self
                    }
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                        self.xgafv = Some(value);
                        self
                    }
                    #[doc = r" Execute the given operation. The fields requested are"]
                    #[doc = r" determined by the FieldSelector attribute of the return type."]
                    #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                    #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                    #[doc = r" are not generic over the return type and deserialize the"]
                    #[doc = r" response into an auto-generated struct will all possible"]
                    #[doc = r" fields."]
                    pub fn execute<T>(self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.execute_with_fields(fields)
                    }
                    #[doc = r" Execute the given operation. This will not provide any"]
                    #[doc = r" `fields` selector indicating that the server will determine"]
                    #[doc = r" the fields returned. This typically includes the most common"]
                    #[doc = r" fields, but it will not include every possible attribute of"]
                    #[doc = r" the response resource."]
                    pub fn execute_with_default_fields(
                        self,
                    ) -> Result<crate::schemas::GoogleCloudMlV1Study, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::GoogleCloudMlV1Study, crate::Error>
                    {
                        self.execute_with_fields(Some("*"))
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub fn execute_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: Into<String>,
                    {
                        self.fields = fields.map(Into::into);
                        self._execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                    {
                        let req = self._request(&self._path())?;
                        Ok(crate::error_from_response(req.send()?)?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://ml.googleapis.com/".to_owned();
                        output.push_str("v1/");
                        {
                            let var_as_str = &self.name;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output
                    }
                    fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error>
                    {
                        let req = self.reqwest.request(::reqwest::Method::GET, path);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let req = req.bearer_auth(
                            self.auth
                                .access_token()
                                .map_err(|err| crate::Error::OAuth2(err))?,
                        );
                        Ok(req)
                    }
                }
                #[doc = "Created via [StudiesActions::list()](struct.StudiesActions.html#method.list)"]
                #[derive(Debug, Clone)]
                pub struct ListRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    parent: String,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a> ListRequestBuilder<'a> {
                    #[doc = "OAuth access token."]
                    pub fn access_token(mut self, value: impl Into<String>) -> Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(mut self, value: impl Into<String>) -> Self {
                        self.callback = Some(value.into());
                        self
                    }
                    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                    pub fn key(mut self, value: impl Into<String>) -> Self {
                        self.key = Some(value.into());
                        self
                    }
                    #[doc = "OAuth 2.0 token for the current user."]
                    pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                        self.oauth_token = Some(value.into());
                        self
                    }
                    #[doc = "Returns response with indentations and line breaks."]
                    pub fn pretty_print(mut self, value: bool) -> Self {
                        self.pretty_print = Some(value);
                        self
                    }
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                        self.xgafv = Some(value);
                        self
                    }
                    #[doc = r" Execute the given operation. The fields requested are"]
                    #[doc = r" determined by the FieldSelector attribute of the return type."]
                    #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                    #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                    #[doc = r" are not generic over the return type and deserialize the"]
                    #[doc = r" response into an auto-generated struct will all possible"]
                    #[doc = r" fields."]
                    pub fn execute<T>(self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.execute_with_fields(fields)
                    }
                    #[doc = r" Execute the given operation. This will not provide any"]
                    #[doc = r" `fields` selector indicating that the server will determine"]
                    #[doc = r" the fields returned. This typically includes the most common"]
                    #[doc = r" fields, but it will not include every possible attribute of"]
                    #[doc = r" the response resource."]
                    pub fn execute_with_default_fields(
                        self,
                    ) -> Result<crate::schemas::GoogleCloudMlV1ListStudiesResponse, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::GoogleCloudMlV1ListStudiesResponse, crate::Error>
                    {
                        self.execute_with_fields(Some("*"))
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub fn execute_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: Into<String>,
                    {
                        self.fields = fields.map(Into::into);
                        self._execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                    {
                        let req = self._request(&self._path())?;
                        Ok(crate::error_from_response(req.send()?)?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://ml.googleapis.com/".to_owned();
                        output.push_str("v1/");
                        {
                            let var_as_str = &self.parent;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/studies");
                        output
                    }
                    fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error>
                    {
                        let req = self.reqwest.request(::reqwest::Method::GET, path);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let req = req.bearer_auth(
                            self.auth
                                .access_token()
                                .map_err(|err| crate::Error::OAuth2(err))?,
                        );
                        Ok(req)
                    }
                }
                pub mod trials {
                    pub mod params {}
                    pub struct TrialsActions<'a> {
                        pub(crate) reqwest: &'a reqwest::blocking::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    }
                    impl<'a> TrialsActions<'a> {
                        fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                            self.auth
                        }
                        #[doc = "Adds a measurement of the objective metrics to a trial. This measurement\nis assumed to have been taken before the trial is complete."]
                        pub fn add_measurement(
                            &self,
                            request: crate::schemas::GoogleCloudMlV1AddTrialMeasurementRequest,
                            name: impl Into<String>,
                        ) -> AddMeasurementRequestBuilder {
                            AddMeasurementRequestBuilder {
                                reqwest: &self.reqwest,
                                auth: self.auth_ref(),
                                request,
                                access_token: None,
                                alt: None,
                                callback: None,
                                fields: None,
                                key: None,
                                oauth_token: None,
                                pretty_print: None,
                                quota_user: None,
                                upload_protocol: None,
                                upload_type: None,
                                xgafv: None,
                                name: name.into(),
                            }
                        }
                        #[doc = "Checks  whether a trial should stop or not. Returns a\nlong-running operation. When the operation is successful,\nit will contain a\nCheckTrialEarlyStoppingStateResponse."]
                        pub fn check_early_stopping_state(
                            &self,
                            request : crate :: schemas :: GoogleCloudMlV1CheckTrialEarlyStoppingStateRequest,
                            name: impl Into<String>,
                        ) -> CheckEarlyStoppingStateRequestBuilder {
                            CheckEarlyStoppingStateRequestBuilder {
                                reqwest: &self.reqwest,
                                auth: self.auth_ref(),
                                request,
                                access_token: None,
                                alt: None,
                                callback: None,
                                fields: None,
                                key: None,
                                oauth_token: None,
                                pretty_print: None,
                                quota_user: None,
                                upload_protocol: None,
                                upload_type: None,
                                xgafv: None,
                                name: name.into(),
                            }
                        }
                        #[doc = "Marks a trial as complete."]
                        pub fn complete(
                            &self,
                            request: crate::schemas::GoogleCloudMlV1CompleteTrialRequest,
                            name: impl Into<String>,
                        ) -> CompleteRequestBuilder {
                            CompleteRequestBuilder {
                                reqwest: &self.reqwest,
                                auth: self.auth_ref(),
                                request,
                                access_token: None,
                                alt: None,
                                callback: None,
                                fields: None,
                                key: None,
                                oauth_token: None,
                                pretty_print: None,
                                quota_user: None,
                                upload_protocol: None,
                                upload_type: None,
                                xgafv: None,
                                name: name.into(),
                            }
                        }
                        #[doc = "Adds a user provided trial to a study."]
                        pub fn create(
                            &self,
                            request: crate::schemas::GoogleCloudMlV1Trial,
                            parent: impl Into<String>,
                        ) -> CreateRequestBuilder {
                            CreateRequestBuilder {
                                reqwest: &self.reqwest,
                                auth: self.auth_ref(),
                                request,
                                access_token: None,
                                alt: None,
                                callback: None,
                                fields: None,
                                key: None,
                                oauth_token: None,
                                pretty_print: None,
                                quota_user: None,
                                upload_protocol: None,
                                upload_type: None,
                                xgafv: None,
                                parent: parent.into(),
                            }
                        }
                        #[doc = "Deletes a trial."]
                        pub fn delete(&self, name: impl Into<String>) -> DeleteRequestBuilder {
                            DeleteRequestBuilder {
                                reqwest: &self.reqwest,
                                auth: self.auth_ref(),
                                access_token: None,
                                alt: None,
                                callback: None,
                                fields: None,
                                key: None,
                                oauth_token: None,
                                pretty_print: None,
                                quota_user: None,
                                upload_protocol: None,
                                upload_type: None,
                                xgafv: None,
                                name: name.into(),
                            }
                        }
                        #[doc = "Gets a trial."]
                        pub fn get(&self, name: impl Into<String>) -> GetRequestBuilder {
                            GetRequestBuilder {
                                reqwest: &self.reqwest,
                                auth: self.auth_ref(),
                                access_token: None,
                                alt: None,
                                callback: None,
                                fields: None,
                                key: None,
                                oauth_token: None,
                                pretty_print: None,
                                quota_user: None,
                                upload_protocol: None,
                                upload_type: None,
                                xgafv: None,
                                name: name.into(),
                            }
                        }
                        #[doc = "Lists the trials associated with a study."]
                        pub fn list(&self, parent: impl Into<String>) -> ListRequestBuilder {
                            ListRequestBuilder {
                                reqwest: &self.reqwest,
                                auth: self.auth_ref(),
                                access_token: None,
                                alt: None,
                                callback: None,
                                fields: None,
                                key: None,
                                oauth_token: None,
                                pretty_print: None,
                                quota_user: None,
                                upload_protocol: None,
                                upload_type: None,
                                xgafv: None,
                                parent: parent.into(),
                            }
                        }
                        #[doc = "Stops a trial."]
                        pub fn stop(
                            &self,
                            request: crate::schemas::GoogleCloudMlV1StopTrialRequest,
                            name: impl Into<String>,
                        ) -> StopRequestBuilder {
                            StopRequestBuilder {
                                reqwest: &self.reqwest,
                                auth: self.auth_ref(),
                                request,
                                access_token: None,
                                alt: None,
                                callback: None,
                                fields: None,
                                key: None,
                                oauth_token: None,
                                pretty_print: None,
                                quota_user: None,
                                upload_protocol: None,
                                upload_type: None,
                                xgafv: None,
                                name: name.into(),
                            }
                        }
                        #[doc = "Adds one or more trials to a study, with parameter values\nsuggested by AI Platform Optimizer. Returns a long-running\noperation associated with the generation of trial suggestions.\nWhen this long-running operation succeeds, it will contain\na SuggestTrialsResponse."]
                        pub fn suggest(
                            &self,
                            request: crate::schemas::GoogleCloudMlV1SuggestTrialsRequest,
                            parent: impl Into<String>,
                        ) -> SuggestRequestBuilder {
                            SuggestRequestBuilder {
                                reqwest: &self.reqwest,
                                auth: self.auth_ref(),
                                request,
                                access_token: None,
                                alt: None,
                                callback: None,
                                fields: None,
                                key: None,
                                oauth_token: None,
                                pretty_print: None,
                                quota_user: None,
                                upload_protocol: None,
                                upload_type: None,
                                xgafv: None,
                                parent: parent.into(),
                            }
                        }
                    }
                    #[doc = "Created via [TrialsActions::add_measurement()](struct.TrialsActions.html#method.add_measurement)"]
                    #[derive(Debug, Clone)]
                    pub struct AddMeasurementRequestBuilder<'a> {
                        pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                        request: crate::schemas::GoogleCloudMlV1AddTrialMeasurementRequest,
                        name: String,
                        access_token: Option<String>,
                        alt: Option<crate::params::Alt>,
                        callback: Option<String>,
                        fields: Option<String>,
                        key: Option<String>,
                        oauth_token: Option<String>,
                        pretty_print: Option<bool>,
                        quota_user: Option<String>,
                        upload_protocol: Option<String>,
                        upload_type: Option<String>,
                        xgafv: Option<crate::params::Xgafv>,
                    }
                    impl<'a> AddMeasurementRequestBuilder<'a> {
                        #[doc = "OAuth access token."]
                        pub fn access_token(mut self, value: impl Into<String>) -> Self {
                            self.access_token = Some(value.into());
                            self
                        }
                        #[doc = "JSONP"]
                        pub fn callback(mut self, value: impl Into<String>) -> Self {
                            self.callback = Some(value.into());
                            self
                        }
                        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                        pub fn key(mut self, value: impl Into<String>) -> Self {
                            self.key = Some(value.into());
                            self
                        }
                        #[doc = "OAuth 2.0 token for the current user."]
                        pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                            self.oauth_token = Some(value.into());
                            self
                        }
                        #[doc = "Returns response with indentations and line breaks."]
                        pub fn pretty_print(mut self, value: bool) -> Self {
                            self.pretty_print = Some(value);
                            self
                        }
                        #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                        pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                            self.quota_user = Some(value.into());
                            self
                        }
                        #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                        pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                            self.upload_protocol = Some(value.into());
                            self
                        }
                        #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                        pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                            self.upload_type = Some(value.into());
                            self
                        }
                        #[doc = "V1 error format."]
                        pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                            self.xgafv = Some(value);
                            self
                        }
                        #[doc = r" Execute the given operation. The fields requested are"]
                        #[doc = r" determined by the FieldSelector attribute of the return type."]
                        #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                        #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                        #[doc = r" are not generic over the return type and deserialize the"]
                        #[doc = r" response into an auto-generated struct will all possible"]
                        #[doc = r" fields."]
                        pub fn execute<T>(self) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned
                                + ::google_field_selector::FieldSelector,
                        {
                            let fields = ::google_field_selector::to_string::<T>();
                            let fields: Option<String> = if fields.is_empty() {
                                None
                            } else {
                                Some(fields)
                            };
                            self.execute_with_fields(fields)
                        }
                        #[doc = r" Execute the given operation. This will not provide any"]
                        #[doc = r" `fields` selector indicating that the server will determine"]
                        #[doc = r" the fields returned. This typically includes the most common"]
                        #[doc = r" fields, but it will not include every possible attribute of"]
                        #[doc = r" the response resource."]
                        pub fn execute_with_default_fields(
                            self,
                        ) -> Result<crate::schemas::GoogleCloudMlV1Trial, crate::Error>
                        {
                            self.execute_with_fields(None::<&str>)
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]
                        pub fn execute_with_all_fields(
                            self,
                        ) -> Result<crate::schemas::GoogleCloudMlV1Trial, crate::Error>
                        {
                            self.execute_with_fields(Some("*"))
                        }
                        #[doc = r" Execute the given operation. This will use the `fields`"]
                        #[doc = r" selector provided and will deserialize the response into"]
                        #[doc = r" whatever return value is provided."]
                        pub fn execute_with_fields<T, F>(
                            mut self,
                            fields: Option<F>,
                        ) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned,
                            F: Into<String>,
                        {
                            self.fields = fields.map(Into::into);
                            self._execute()
                        }
                        fn _execute<T>(&mut self) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned,
                        {
                            let req = self._request(&self._path())?;
                            let req = req.json(&self.request);
                            Ok(crate::error_from_response(req.send()?)?.json()?)
                        }
                        fn _path(&self) -> String {
                            let mut output = "https://ml.googleapis.com/".to_owned();
                            output.push_str("v1/");
                            {
                                let var_as_str = &self.name;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::RESERVED,
                                ));
                            }
                            output.push_str(":addMeasurement");
                            output
                        }
                        fn _request(
                            &self,
                            path: &str,
                        ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error>
                        {
                            let req = self.reqwest.request(::reqwest::Method::POST, path);
                            let req = req.query(&[("access_token", &self.access_token)]);
                            let req = req.query(&[("alt", &self.alt)]);
                            let req = req.query(&[("callback", &self.callback)]);
                            let req = req.query(&[("fields", &self.fields)]);
                            let req = req.query(&[("key", &self.key)]);
                            let req = req.query(&[("oauth_token", &self.oauth_token)]);
                            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                            let req = req.query(&[("quotaUser", &self.quota_user)]);
                            let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                            let req = req.query(&[("uploadType", &self.upload_type)]);
                            let req = req.query(&[("$.xgafv", &self.xgafv)]);
                            let req = req.bearer_auth(
                                self.auth
                                    .access_token()
                                    .map_err(|err| crate::Error::OAuth2(err))?,
                            );
                            Ok(req)
                        }
                    }
                    #[doc = "Created via [TrialsActions::check_early_stopping_state()](struct.TrialsActions.html#method.check_early_stopping_state)"]
                    #[derive(Debug, Clone)]
                    pub struct CheckEarlyStoppingStateRequestBuilder<'a> {
                        pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                        request: crate::schemas::GoogleCloudMlV1CheckTrialEarlyStoppingStateRequest,
                        name: String,
                        access_token: Option<String>,
                        alt: Option<crate::params::Alt>,
                        callback: Option<String>,
                        fields: Option<String>,
                        key: Option<String>,
                        oauth_token: Option<String>,
                        pretty_print: Option<bool>,
                        quota_user: Option<String>,
                        upload_protocol: Option<String>,
                        upload_type: Option<String>,
                        xgafv: Option<crate::params::Xgafv>,
                    }
                    impl<'a> CheckEarlyStoppingStateRequestBuilder<'a> {
                        #[doc = "OAuth access token."]
                        pub fn access_token(mut self, value: impl Into<String>) -> Self {
                            self.access_token = Some(value.into());
                            self
                        }
                        #[doc = "JSONP"]
                        pub fn callback(mut self, value: impl Into<String>) -> Self {
                            self.callback = Some(value.into());
                            self
                        }
                        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                        pub fn key(mut self, value: impl Into<String>) -> Self {
                            self.key = Some(value.into());
                            self
                        }
                        #[doc = "OAuth 2.0 token for the current user."]
                        pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                            self.oauth_token = Some(value.into());
                            self
                        }
                        #[doc = "Returns response with indentations and line breaks."]
                        pub fn pretty_print(mut self, value: bool) -> Self {
                            self.pretty_print = Some(value);
                            self
                        }
                        #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                        pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                            self.quota_user = Some(value.into());
                            self
                        }
                        #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                        pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                            self.upload_protocol = Some(value.into());
                            self
                        }
                        #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                        pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                            self.upload_type = Some(value.into());
                            self
                        }
                        #[doc = "V1 error format."]
                        pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                            self.xgafv = Some(value);
                            self
                        }
                        #[doc = r" Execute the given operation. The fields requested are"]
                        #[doc = r" determined by the FieldSelector attribute of the return type."]
                        #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                        #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                        #[doc = r" are not generic over the return type and deserialize the"]
                        #[doc = r" response into an auto-generated struct will all possible"]
                        #[doc = r" fields."]
                        pub fn execute<T>(self) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned
                                + ::google_field_selector::FieldSelector,
                        {
                            let fields = ::google_field_selector::to_string::<T>();
                            let fields: Option<String> = if fields.is_empty() {
                                None
                            } else {
                                Some(fields)
                            };
                            self.execute_with_fields(fields)
                        }
                        #[doc = r" Execute the given operation. This will not provide any"]
                        #[doc = r" `fields` selector indicating that the server will determine"]
                        #[doc = r" the fields returned. This typically includes the most common"]
                        #[doc = r" fields, but it will not include every possible attribute of"]
                        #[doc = r" the response resource."]
                        pub fn execute_with_default_fields(
                            self,
                        ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error>
                        {
                            self.execute_with_fields(None::<&str>)
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]
                        pub fn execute_with_all_fields(
                            self,
                        ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error>
                        {
                            self.execute_with_fields(Some("*"))
                        }
                        #[doc = r" Execute the given operation. This will use the `fields`"]
                        #[doc = r" selector provided and will deserialize the response into"]
                        #[doc = r" whatever return value is provided."]
                        pub fn execute_with_fields<T, F>(
                            mut self,
                            fields: Option<F>,
                        ) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned,
                            F: Into<String>,
                        {
                            self.fields = fields.map(Into::into);
                            self._execute()
                        }
                        fn _execute<T>(&mut self) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned,
                        {
                            let req = self._request(&self._path())?;
                            let req = req.json(&self.request);
                            Ok(crate::error_from_response(req.send()?)?.json()?)
                        }
                        fn _path(&self) -> String {
                            let mut output = "https://ml.googleapis.com/".to_owned();
                            output.push_str("v1/");
                            {
                                let var_as_str = &self.name;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::RESERVED,
                                ));
                            }
                            output.push_str(":checkEarlyStoppingState");
                            output
                        }
                        fn _request(
                            &self,
                            path: &str,
                        ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error>
                        {
                            let req = self.reqwest.request(::reqwest::Method::POST, path);
                            let req = req.query(&[("access_token", &self.access_token)]);
                            let req = req.query(&[("alt", &self.alt)]);
                            let req = req.query(&[("callback", &self.callback)]);
                            let req = req.query(&[("fields", &self.fields)]);
                            let req = req.query(&[("key", &self.key)]);
                            let req = req.query(&[("oauth_token", &self.oauth_token)]);
                            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                            let req = req.query(&[("quotaUser", &self.quota_user)]);
                            let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                            let req = req.query(&[("uploadType", &self.upload_type)]);
                            let req = req.query(&[("$.xgafv", &self.xgafv)]);
                            let req = req.bearer_auth(
                                self.auth
                                    .access_token()
                                    .map_err(|err| crate::Error::OAuth2(err))?,
                            );
                            Ok(req)
                        }
                    }
                    #[doc = "Created via [TrialsActions::complete()](struct.TrialsActions.html#method.complete)"]
                    #[derive(Debug, Clone)]
                    pub struct CompleteRequestBuilder<'a> {
                        pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                        request: crate::schemas::GoogleCloudMlV1CompleteTrialRequest,
                        name: String,
                        access_token: Option<String>,
                        alt: Option<crate::params::Alt>,
                        callback: Option<String>,
                        fields: Option<String>,
                        key: Option<String>,
                        oauth_token: Option<String>,
                        pretty_print: Option<bool>,
                        quota_user: Option<String>,
                        upload_protocol: Option<String>,
                        upload_type: Option<String>,
                        xgafv: Option<crate::params::Xgafv>,
                    }
                    impl<'a> CompleteRequestBuilder<'a> {
                        #[doc = "OAuth access token."]
                        pub fn access_token(mut self, value: impl Into<String>) -> Self {
                            self.access_token = Some(value.into());
                            self
                        }
                        #[doc = "JSONP"]
                        pub fn callback(mut self, value: impl Into<String>) -> Self {
                            self.callback = Some(value.into());
                            self
                        }
                        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                        pub fn key(mut self, value: impl Into<String>) -> Self {
                            self.key = Some(value.into());
                            self
                        }
                        #[doc = "OAuth 2.0 token for the current user."]
                        pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                            self.oauth_token = Some(value.into());
                            self
                        }
                        #[doc = "Returns response with indentations and line breaks."]
                        pub fn pretty_print(mut self, value: bool) -> Self {
                            self.pretty_print = Some(value);
                            self
                        }
                        #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                        pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                            self.quota_user = Some(value.into());
                            self
                        }
                        #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                        pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                            self.upload_protocol = Some(value.into());
                            self
                        }
                        #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                        pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                            self.upload_type = Some(value.into());
                            self
                        }
                        #[doc = "V1 error format."]
                        pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                            self.xgafv = Some(value);
                            self
                        }
                        #[doc = r" Execute the given operation. The fields requested are"]
                        #[doc = r" determined by the FieldSelector attribute of the return type."]
                        #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                        #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                        #[doc = r" are not generic over the return type and deserialize the"]
                        #[doc = r" response into an auto-generated struct will all possible"]
                        #[doc = r" fields."]
                        pub fn execute<T>(self) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned
                                + ::google_field_selector::FieldSelector,
                        {
                            let fields = ::google_field_selector::to_string::<T>();
                            let fields: Option<String> = if fields.is_empty() {
                                None
                            } else {
                                Some(fields)
                            };
                            self.execute_with_fields(fields)
                        }
                        #[doc = r" Execute the given operation. This will not provide any"]
                        #[doc = r" `fields` selector indicating that the server will determine"]
                        #[doc = r" the fields returned. This typically includes the most common"]
                        #[doc = r" fields, but it will not include every possible attribute of"]
                        #[doc = r" the response resource."]
                        pub fn execute_with_default_fields(
                            self,
                        ) -> Result<crate::schemas::GoogleCloudMlV1Trial, crate::Error>
                        {
                            self.execute_with_fields(None::<&str>)
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]
                        pub fn execute_with_all_fields(
                            self,
                        ) -> Result<crate::schemas::GoogleCloudMlV1Trial, crate::Error>
                        {
                            self.execute_with_fields(Some("*"))
                        }
                        #[doc = r" Execute the given operation. This will use the `fields`"]
                        #[doc = r" selector provided and will deserialize the response into"]
                        #[doc = r" whatever return value is provided."]
                        pub fn execute_with_fields<T, F>(
                            mut self,
                            fields: Option<F>,
                        ) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned,
                            F: Into<String>,
                        {
                            self.fields = fields.map(Into::into);
                            self._execute()
                        }
                        fn _execute<T>(&mut self) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned,
                        {
                            let req = self._request(&self._path())?;
                            let req = req.json(&self.request);
                            Ok(crate::error_from_response(req.send()?)?.json()?)
                        }
                        fn _path(&self) -> String {
                            let mut output = "https://ml.googleapis.com/".to_owned();
                            output.push_str("v1/");
                            {
                                let var_as_str = &self.name;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::RESERVED,
                                ));
                            }
                            output.push_str(":complete");
                            output
                        }
                        fn _request(
                            &self,
                            path: &str,
                        ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error>
                        {
                            let req = self.reqwest.request(::reqwest::Method::POST, path);
                            let req = req.query(&[("access_token", &self.access_token)]);
                            let req = req.query(&[("alt", &self.alt)]);
                            let req = req.query(&[("callback", &self.callback)]);
                            let req = req.query(&[("fields", &self.fields)]);
                            let req = req.query(&[("key", &self.key)]);
                            let req = req.query(&[("oauth_token", &self.oauth_token)]);
                            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                            let req = req.query(&[("quotaUser", &self.quota_user)]);
                            let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                            let req = req.query(&[("uploadType", &self.upload_type)]);
                            let req = req.query(&[("$.xgafv", &self.xgafv)]);
                            let req = req.bearer_auth(
                                self.auth
                                    .access_token()
                                    .map_err(|err| crate::Error::OAuth2(err))?,
                            );
                            Ok(req)
                        }
                    }
                    #[doc = "Created via [TrialsActions::create()](struct.TrialsActions.html#method.create)"]
                    #[derive(Debug, Clone)]
                    pub struct CreateRequestBuilder<'a> {
                        pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                        request: crate::schemas::GoogleCloudMlV1Trial,
                        parent: String,
                        access_token: Option<String>,
                        alt: Option<crate::params::Alt>,
                        callback: Option<String>,
                        fields: Option<String>,
                        key: Option<String>,
                        oauth_token: Option<String>,
                        pretty_print: Option<bool>,
                        quota_user: Option<String>,
                        upload_protocol: Option<String>,
                        upload_type: Option<String>,
                        xgafv: Option<crate::params::Xgafv>,
                    }
                    impl<'a> CreateRequestBuilder<'a> {
                        #[doc = "OAuth access token."]
                        pub fn access_token(mut self, value: impl Into<String>) -> Self {
                            self.access_token = Some(value.into());
                            self
                        }
                        #[doc = "JSONP"]
                        pub fn callback(mut self, value: impl Into<String>) -> Self {
                            self.callback = Some(value.into());
                            self
                        }
                        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                        pub fn key(mut self, value: impl Into<String>) -> Self {
                            self.key = Some(value.into());
                            self
                        }
                        #[doc = "OAuth 2.0 token for the current user."]
                        pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                            self.oauth_token = Some(value.into());
                            self
                        }
                        #[doc = "Returns response with indentations and line breaks."]
                        pub fn pretty_print(mut self, value: bool) -> Self {
                            self.pretty_print = Some(value);
                            self
                        }
                        #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                        pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                            self.quota_user = Some(value.into());
                            self
                        }
                        #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                        pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                            self.upload_protocol = Some(value.into());
                            self
                        }
                        #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                        pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                            self.upload_type = Some(value.into());
                            self
                        }
                        #[doc = "V1 error format."]
                        pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                            self.xgafv = Some(value);
                            self
                        }
                        #[doc = r" Execute the given operation. The fields requested are"]
                        #[doc = r" determined by the FieldSelector attribute of the return type."]
                        #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                        #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                        #[doc = r" are not generic over the return type and deserialize the"]
                        #[doc = r" response into an auto-generated struct will all possible"]
                        #[doc = r" fields."]
                        pub fn execute<T>(self) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned
                                + ::google_field_selector::FieldSelector,
                        {
                            let fields = ::google_field_selector::to_string::<T>();
                            let fields: Option<String> = if fields.is_empty() {
                                None
                            } else {
                                Some(fields)
                            };
                            self.execute_with_fields(fields)
                        }
                        #[doc = r" Execute the given operation. This will not provide any"]
                        #[doc = r" `fields` selector indicating that the server will determine"]
                        #[doc = r" the fields returned. This typically includes the most common"]
                        #[doc = r" fields, but it will not include every possible attribute of"]
                        #[doc = r" the response resource."]
                        pub fn execute_with_default_fields(
                            self,
                        ) -> Result<crate::schemas::GoogleCloudMlV1Trial, crate::Error>
                        {
                            self.execute_with_fields(None::<&str>)
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]
                        pub fn execute_with_all_fields(
                            self,
                        ) -> Result<crate::schemas::GoogleCloudMlV1Trial, crate::Error>
                        {
                            self.execute_with_fields(Some("*"))
                        }
                        #[doc = r" Execute the given operation. This will use the `fields`"]
                        #[doc = r" selector provided and will deserialize the response into"]
                        #[doc = r" whatever return value is provided."]
                        pub fn execute_with_fields<T, F>(
                            mut self,
                            fields: Option<F>,
                        ) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned,
                            F: Into<String>,
                        {
                            self.fields = fields.map(Into::into);
                            self._execute()
                        }
                        fn _execute<T>(&mut self) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned,
                        {
                            let req = self._request(&self._path())?;
                            let req = req.json(&self.request);
                            Ok(crate::error_from_response(req.send()?)?.json()?)
                        }
                        fn _path(&self) -> String {
                            let mut output = "https://ml.googleapis.com/".to_owned();
                            output.push_str("v1/");
                            {
                                let var_as_str = &self.parent;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::RESERVED,
                                ));
                            }
                            output.push_str("/trials");
                            output
                        }
                        fn _request(
                            &self,
                            path: &str,
                        ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error>
                        {
                            let req = self.reqwest.request(::reqwest::Method::POST, path);
                            let req = req.query(&[("access_token", &self.access_token)]);
                            let req = req.query(&[("alt", &self.alt)]);
                            let req = req.query(&[("callback", &self.callback)]);
                            let req = req.query(&[("fields", &self.fields)]);
                            let req = req.query(&[("key", &self.key)]);
                            let req = req.query(&[("oauth_token", &self.oauth_token)]);
                            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                            let req = req.query(&[("quotaUser", &self.quota_user)]);
                            let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                            let req = req.query(&[("uploadType", &self.upload_type)]);
                            let req = req.query(&[("$.xgafv", &self.xgafv)]);
                            let req = req.bearer_auth(
                                self.auth
                                    .access_token()
                                    .map_err(|err| crate::Error::OAuth2(err))?,
                            );
                            Ok(req)
                        }
                    }
                    #[doc = "Created via [TrialsActions::delete()](struct.TrialsActions.html#method.delete)"]
                    #[derive(Debug, Clone)]
                    pub struct DeleteRequestBuilder<'a> {
                        pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                        name: String,
                        access_token: Option<String>,
                        alt: Option<crate::params::Alt>,
                        callback: Option<String>,
                        fields: Option<String>,
                        key: Option<String>,
                        oauth_token: Option<String>,
                        pretty_print: Option<bool>,
                        quota_user: Option<String>,
                        upload_protocol: Option<String>,
                        upload_type: Option<String>,
                        xgafv: Option<crate::params::Xgafv>,
                    }
                    impl<'a> DeleteRequestBuilder<'a> {
                        #[doc = "OAuth access token."]
                        pub fn access_token(mut self, value: impl Into<String>) -> Self {
                            self.access_token = Some(value.into());
                            self
                        }
                        #[doc = "JSONP"]
                        pub fn callback(mut self, value: impl Into<String>) -> Self {
                            self.callback = Some(value.into());
                            self
                        }
                        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                        pub fn key(mut self, value: impl Into<String>) -> Self {
                            self.key = Some(value.into());
                            self
                        }
                        #[doc = "OAuth 2.0 token for the current user."]
                        pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                            self.oauth_token = Some(value.into());
                            self
                        }
                        #[doc = "Returns response with indentations and line breaks."]
                        pub fn pretty_print(mut self, value: bool) -> Self {
                            self.pretty_print = Some(value);
                            self
                        }
                        #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                        pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                            self.quota_user = Some(value.into());
                            self
                        }
                        #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                        pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                            self.upload_protocol = Some(value.into());
                            self
                        }
                        #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                        pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                            self.upload_type = Some(value.into());
                            self
                        }
                        #[doc = "V1 error format."]
                        pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                            self.xgafv = Some(value);
                            self
                        }
                        #[doc = r" Execute the given operation. The fields requested are"]
                        #[doc = r" determined by the FieldSelector attribute of the return type."]
                        #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                        #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                        #[doc = r" are not generic over the return type and deserialize the"]
                        #[doc = r" response into an auto-generated struct will all possible"]
                        #[doc = r" fields."]
                        pub fn execute<T>(self) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned
                                + ::google_field_selector::FieldSelector,
                        {
                            let fields = ::google_field_selector::to_string::<T>();
                            let fields: Option<String> = if fields.is_empty() {
                                None
                            } else {
                                Some(fields)
                            };
                            self.execute_with_fields(fields)
                        }
                        #[doc = r" Execute the given operation. This will not provide any"]
                        #[doc = r" `fields` selector indicating that the server will determine"]
                        #[doc = r" the fields returned. This typically includes the most common"]
                        #[doc = r" fields, but it will not include every possible attribute of"]
                        #[doc = r" the response resource."]
                        pub fn execute_with_default_fields(
                            self,
                        ) -> Result<crate::schemas::GoogleProtobufEmpty, crate::Error>
                        {
                            self.execute_with_fields(None::<&str>)
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]
                        pub fn execute_with_all_fields(
                            self,
                        ) -> Result<crate::schemas::GoogleProtobufEmpty, crate::Error>
                        {
                            self.execute_with_fields(Some("*"))
                        }
                        #[doc = r" Execute the given operation. This will use the `fields`"]
                        #[doc = r" selector provided and will deserialize the response into"]
                        #[doc = r" whatever return value is provided."]
                        pub fn execute_with_fields<T, F>(
                            mut self,
                            fields: Option<F>,
                        ) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned,
                            F: Into<String>,
                        {
                            self.fields = fields.map(Into::into);
                            self._execute()
                        }
                        fn _execute<T>(&mut self) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned,
                        {
                            let req = self._request(&self._path())?;
                            Ok(crate::error_from_response(req.send()?)?.json()?)
                        }
                        fn _path(&self) -> String {
                            let mut output = "https://ml.googleapis.com/".to_owned();
                            output.push_str("v1/");
                            {
                                let var_as_str = &self.name;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::RESERVED,
                                ));
                            }
                            output
                        }
                        fn _request(
                            &self,
                            path: &str,
                        ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error>
                        {
                            let req = self.reqwest.request(::reqwest::Method::DELETE, path);
                            let req = req.query(&[("access_token", &self.access_token)]);
                            let req = req.query(&[("alt", &self.alt)]);
                            let req = req.query(&[("callback", &self.callback)]);
                            let req = req.query(&[("fields", &self.fields)]);
                            let req = req.query(&[("key", &self.key)]);
                            let req = req.query(&[("oauth_token", &self.oauth_token)]);
                            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                            let req = req.query(&[("quotaUser", &self.quota_user)]);
                            let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                            let req = req.query(&[("uploadType", &self.upload_type)]);
                            let req = req.query(&[("$.xgafv", &self.xgafv)]);
                            let req = req.bearer_auth(
                                self.auth
                                    .access_token()
                                    .map_err(|err| crate::Error::OAuth2(err))?,
                            );
                            Ok(req)
                        }
                    }
                    #[doc = "Created via [TrialsActions::get()](struct.TrialsActions.html#method.get)"]
                    #[derive(Debug, Clone)]
                    pub struct GetRequestBuilder<'a> {
                        pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                        name: String,
                        access_token: Option<String>,
                        alt: Option<crate::params::Alt>,
                        callback: Option<String>,
                        fields: Option<String>,
                        key: Option<String>,
                        oauth_token: Option<String>,
                        pretty_print: Option<bool>,
                        quota_user: Option<String>,
                        upload_protocol: Option<String>,
                        upload_type: Option<String>,
                        xgafv: Option<crate::params::Xgafv>,
                    }
                    impl<'a> GetRequestBuilder<'a> {
                        #[doc = "OAuth access token."]
                        pub fn access_token(mut self, value: impl Into<String>) -> Self {
                            self.access_token = Some(value.into());
                            self
                        }
                        #[doc = "JSONP"]
                        pub fn callback(mut self, value: impl Into<String>) -> Self {
                            self.callback = Some(value.into());
                            self
                        }
                        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                        pub fn key(mut self, value: impl Into<String>) -> Self {
                            self.key = Some(value.into());
                            self
                        }
                        #[doc = "OAuth 2.0 token for the current user."]
                        pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                            self.oauth_token = Some(value.into());
                            self
                        }
                        #[doc = "Returns response with indentations and line breaks."]
                        pub fn pretty_print(mut self, value: bool) -> Self {
                            self.pretty_print = Some(value);
                            self
                        }
                        #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                        pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                            self.quota_user = Some(value.into());
                            self
                        }
                        #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                        pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                            self.upload_protocol = Some(value.into());
                            self
                        }
                        #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                        pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                            self.upload_type = Some(value.into());
                            self
                        }
                        #[doc = "V1 error format."]
                        pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                            self.xgafv = Some(value);
                            self
                        }
                        #[doc = r" Execute the given operation. The fields requested are"]
                        #[doc = r" determined by the FieldSelector attribute of the return type."]
                        #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                        #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                        #[doc = r" are not generic over the return type and deserialize the"]
                        #[doc = r" response into an auto-generated struct will all possible"]
                        #[doc = r" fields."]
                        pub fn execute<T>(self) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned
                                + ::google_field_selector::FieldSelector,
                        {
                            let fields = ::google_field_selector::to_string::<T>();
                            let fields: Option<String> = if fields.is_empty() {
                                None
                            } else {
                                Some(fields)
                            };
                            self.execute_with_fields(fields)
                        }
                        #[doc = r" Execute the given operation. This will not provide any"]
                        #[doc = r" `fields` selector indicating that the server will determine"]
                        #[doc = r" the fields returned. This typically includes the most common"]
                        #[doc = r" fields, but it will not include every possible attribute of"]
                        #[doc = r" the response resource."]
                        pub fn execute_with_default_fields(
                            self,
                        ) -> Result<crate::schemas::GoogleCloudMlV1Trial, crate::Error>
                        {
                            self.execute_with_fields(None::<&str>)
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]
                        pub fn execute_with_all_fields(
                            self,
                        ) -> Result<crate::schemas::GoogleCloudMlV1Trial, crate::Error>
                        {
                            self.execute_with_fields(Some("*"))
                        }
                        #[doc = r" Execute the given operation. This will use the `fields`"]
                        #[doc = r" selector provided and will deserialize the response into"]
                        #[doc = r" whatever return value is provided."]
                        pub fn execute_with_fields<T, F>(
                            mut self,
                            fields: Option<F>,
                        ) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned,
                            F: Into<String>,
                        {
                            self.fields = fields.map(Into::into);
                            self._execute()
                        }
                        fn _execute<T>(&mut self) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned,
                        {
                            let req = self._request(&self._path())?;
                            Ok(crate::error_from_response(req.send()?)?.json()?)
                        }
                        fn _path(&self) -> String {
                            let mut output = "https://ml.googleapis.com/".to_owned();
                            output.push_str("v1/");
                            {
                                let var_as_str = &self.name;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::RESERVED,
                                ));
                            }
                            output
                        }
                        fn _request(
                            &self,
                            path: &str,
                        ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error>
                        {
                            let req = self.reqwest.request(::reqwest::Method::GET, path);
                            let req = req.query(&[("access_token", &self.access_token)]);
                            let req = req.query(&[("alt", &self.alt)]);
                            let req = req.query(&[("callback", &self.callback)]);
                            let req = req.query(&[("fields", &self.fields)]);
                            let req = req.query(&[("key", &self.key)]);
                            let req = req.query(&[("oauth_token", &self.oauth_token)]);
                            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                            let req = req.query(&[("quotaUser", &self.quota_user)]);
                            let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                            let req = req.query(&[("uploadType", &self.upload_type)]);
                            let req = req.query(&[("$.xgafv", &self.xgafv)]);
                            let req = req.bearer_auth(
                                self.auth
                                    .access_token()
                                    .map_err(|err| crate::Error::OAuth2(err))?,
                            );
                            Ok(req)
                        }
                    }
                    #[doc = "Created via [TrialsActions::list()](struct.TrialsActions.html#method.list)"]
                    #[derive(Debug, Clone)]
                    pub struct ListRequestBuilder<'a> {
                        pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                        parent: String,
                        access_token: Option<String>,
                        alt: Option<crate::params::Alt>,
                        callback: Option<String>,
                        fields: Option<String>,
                        key: Option<String>,
                        oauth_token: Option<String>,
                        pretty_print: Option<bool>,
                        quota_user: Option<String>,
                        upload_protocol: Option<String>,
                        upload_type: Option<String>,
                        xgafv: Option<crate::params::Xgafv>,
                    }
                    impl<'a> ListRequestBuilder<'a> {
                        #[doc = "OAuth access token."]
                        pub fn access_token(mut self, value: impl Into<String>) -> Self {
                            self.access_token = Some(value.into());
                            self
                        }
                        #[doc = "JSONP"]
                        pub fn callback(mut self, value: impl Into<String>) -> Self {
                            self.callback = Some(value.into());
                            self
                        }
                        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                        pub fn key(mut self, value: impl Into<String>) -> Self {
                            self.key = Some(value.into());
                            self
                        }
                        #[doc = "OAuth 2.0 token for the current user."]
                        pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                            self.oauth_token = Some(value.into());
                            self
                        }
                        #[doc = "Returns response with indentations and line breaks."]
                        pub fn pretty_print(mut self, value: bool) -> Self {
                            self.pretty_print = Some(value);
                            self
                        }
                        #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                        pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                            self.quota_user = Some(value.into());
                            self
                        }
                        #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                        pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                            self.upload_protocol = Some(value.into());
                            self
                        }
                        #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                        pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                            self.upload_type = Some(value.into());
                            self
                        }
                        #[doc = "V1 error format."]
                        pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                            self.xgafv = Some(value);
                            self
                        }
                        #[doc = r" Execute the given operation. The fields requested are"]
                        #[doc = r" determined by the FieldSelector attribute of the return type."]
                        #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                        #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                        #[doc = r" are not generic over the return type and deserialize the"]
                        #[doc = r" response into an auto-generated struct will all possible"]
                        #[doc = r" fields."]
                        pub fn execute<T>(self) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned
                                + ::google_field_selector::FieldSelector,
                        {
                            let fields = ::google_field_selector::to_string::<T>();
                            let fields: Option<String> = if fields.is_empty() {
                                None
                            } else {
                                Some(fields)
                            };
                            self.execute_with_fields(fields)
                        }
                        #[doc = r" Execute the given operation. This will not provide any"]
                        #[doc = r" `fields` selector indicating that the server will determine"]
                        #[doc = r" the fields returned. This typically includes the most common"]
                        #[doc = r" fields, but it will not include every possible attribute of"]
                        #[doc = r" the response resource."]
                        pub fn execute_with_default_fields(
                            self,
                        ) -> Result<crate::schemas::GoogleCloudMlV1ListTrialsResponse, crate::Error>
                        {
                            self.execute_with_fields(None::<&str>)
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]
                        pub fn execute_with_all_fields(
                            self,
                        ) -> Result<crate::schemas::GoogleCloudMlV1ListTrialsResponse, crate::Error>
                        {
                            self.execute_with_fields(Some("*"))
                        }
                        #[doc = r" Execute the given operation. This will use the `fields`"]
                        #[doc = r" selector provided and will deserialize the response into"]
                        #[doc = r" whatever return value is provided."]
                        pub fn execute_with_fields<T, F>(
                            mut self,
                            fields: Option<F>,
                        ) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned,
                            F: Into<String>,
                        {
                            self.fields = fields.map(Into::into);
                            self._execute()
                        }
                        fn _execute<T>(&mut self) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned,
                        {
                            let req = self._request(&self._path())?;
                            Ok(crate::error_from_response(req.send()?)?.json()?)
                        }
                        fn _path(&self) -> String {
                            let mut output = "https://ml.googleapis.com/".to_owned();
                            output.push_str("v1/");
                            {
                                let var_as_str = &self.parent;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::RESERVED,
                                ));
                            }
                            output.push_str("/trials");
                            output
                        }
                        fn _request(
                            &self,
                            path: &str,
                        ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error>
                        {
                            let req = self.reqwest.request(::reqwest::Method::GET, path);
                            let req = req.query(&[("access_token", &self.access_token)]);
                            let req = req.query(&[("alt", &self.alt)]);
                            let req = req.query(&[("callback", &self.callback)]);
                            let req = req.query(&[("fields", &self.fields)]);
                            let req = req.query(&[("key", &self.key)]);
                            let req = req.query(&[("oauth_token", &self.oauth_token)]);
                            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                            let req = req.query(&[("quotaUser", &self.quota_user)]);
                            let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                            let req = req.query(&[("uploadType", &self.upload_type)]);
                            let req = req.query(&[("$.xgafv", &self.xgafv)]);
                            let req = req.bearer_auth(
                                self.auth
                                    .access_token()
                                    .map_err(|err| crate::Error::OAuth2(err))?,
                            );
                            Ok(req)
                        }
                    }
                    #[doc = "Created via [TrialsActions::stop()](struct.TrialsActions.html#method.stop)"]
                    #[derive(Debug, Clone)]
                    pub struct StopRequestBuilder<'a> {
                        pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                        request: crate::schemas::GoogleCloudMlV1StopTrialRequest,
                        name: String,
                        access_token: Option<String>,
                        alt: Option<crate::params::Alt>,
                        callback: Option<String>,
                        fields: Option<String>,
                        key: Option<String>,
                        oauth_token: Option<String>,
                        pretty_print: Option<bool>,
                        quota_user: Option<String>,
                        upload_protocol: Option<String>,
                        upload_type: Option<String>,
                        xgafv: Option<crate::params::Xgafv>,
                    }
                    impl<'a> StopRequestBuilder<'a> {
                        #[doc = "OAuth access token."]
                        pub fn access_token(mut self, value: impl Into<String>) -> Self {
                            self.access_token = Some(value.into());
                            self
                        }
                        #[doc = "JSONP"]
                        pub fn callback(mut self, value: impl Into<String>) -> Self {
                            self.callback = Some(value.into());
                            self
                        }
                        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                        pub fn key(mut self, value: impl Into<String>) -> Self {
                            self.key = Some(value.into());
                            self
                        }
                        #[doc = "OAuth 2.0 token for the current user."]
                        pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                            self.oauth_token = Some(value.into());
                            self
                        }
                        #[doc = "Returns response with indentations and line breaks."]
                        pub fn pretty_print(mut self, value: bool) -> Self {
                            self.pretty_print = Some(value);
                            self
                        }
                        #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                        pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                            self.quota_user = Some(value.into());
                            self
                        }
                        #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                        pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                            self.upload_protocol = Some(value.into());
                            self
                        }
                        #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                        pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                            self.upload_type = Some(value.into());
                            self
                        }
                        #[doc = "V1 error format."]
                        pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                            self.xgafv = Some(value);
                            self
                        }
                        #[doc = r" Execute the given operation. The fields requested are"]
                        #[doc = r" determined by the FieldSelector attribute of the return type."]
                        #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                        #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                        #[doc = r" are not generic over the return type and deserialize the"]
                        #[doc = r" response into an auto-generated struct will all possible"]
                        #[doc = r" fields."]
                        pub fn execute<T>(self) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned
                                + ::google_field_selector::FieldSelector,
                        {
                            let fields = ::google_field_selector::to_string::<T>();
                            let fields: Option<String> = if fields.is_empty() {
                                None
                            } else {
                                Some(fields)
                            };
                            self.execute_with_fields(fields)
                        }
                        #[doc = r" Execute the given operation. This will not provide any"]
                        #[doc = r" `fields` selector indicating that the server will determine"]
                        #[doc = r" the fields returned. This typically includes the most common"]
                        #[doc = r" fields, but it will not include every possible attribute of"]
                        #[doc = r" the response resource."]
                        pub fn execute_with_default_fields(
                            self,
                        ) -> Result<crate::schemas::GoogleCloudMlV1Trial, crate::Error>
                        {
                            self.execute_with_fields(None::<&str>)
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]
                        pub fn execute_with_all_fields(
                            self,
                        ) -> Result<crate::schemas::GoogleCloudMlV1Trial, crate::Error>
                        {
                            self.execute_with_fields(Some("*"))
                        }
                        #[doc = r" Execute the given operation. This will use the `fields`"]
                        #[doc = r" selector provided and will deserialize the response into"]
                        #[doc = r" whatever return value is provided."]
                        pub fn execute_with_fields<T, F>(
                            mut self,
                            fields: Option<F>,
                        ) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned,
                            F: Into<String>,
                        {
                            self.fields = fields.map(Into::into);
                            self._execute()
                        }
                        fn _execute<T>(&mut self) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned,
                        {
                            let req = self._request(&self._path())?;
                            let req = req.json(&self.request);
                            Ok(crate::error_from_response(req.send()?)?.json()?)
                        }
                        fn _path(&self) -> String {
                            let mut output = "https://ml.googleapis.com/".to_owned();
                            output.push_str("v1/");
                            {
                                let var_as_str = &self.name;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::RESERVED,
                                ));
                            }
                            output.push_str(":stop");
                            output
                        }
                        fn _request(
                            &self,
                            path: &str,
                        ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error>
                        {
                            let req = self.reqwest.request(::reqwest::Method::POST, path);
                            let req = req.query(&[("access_token", &self.access_token)]);
                            let req = req.query(&[("alt", &self.alt)]);
                            let req = req.query(&[("callback", &self.callback)]);
                            let req = req.query(&[("fields", &self.fields)]);
                            let req = req.query(&[("key", &self.key)]);
                            let req = req.query(&[("oauth_token", &self.oauth_token)]);
                            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                            let req = req.query(&[("quotaUser", &self.quota_user)]);
                            let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                            let req = req.query(&[("uploadType", &self.upload_type)]);
                            let req = req.query(&[("$.xgafv", &self.xgafv)]);
                            let req = req.bearer_auth(
                                self.auth
                                    .access_token()
                                    .map_err(|err| crate::Error::OAuth2(err))?,
                            );
                            Ok(req)
                        }
                    }
                    #[doc = "Created via [TrialsActions::suggest()](struct.TrialsActions.html#method.suggest)"]
                    #[derive(Debug, Clone)]
                    pub struct SuggestRequestBuilder<'a> {
                        pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                        request: crate::schemas::GoogleCloudMlV1SuggestTrialsRequest,
                        parent: String,
                        access_token: Option<String>,
                        alt: Option<crate::params::Alt>,
                        callback: Option<String>,
                        fields: Option<String>,
                        key: Option<String>,
                        oauth_token: Option<String>,
                        pretty_print: Option<bool>,
                        quota_user: Option<String>,
                        upload_protocol: Option<String>,
                        upload_type: Option<String>,
                        xgafv: Option<crate::params::Xgafv>,
                    }
                    impl<'a> SuggestRequestBuilder<'a> {
                        #[doc = "OAuth access token."]
                        pub fn access_token(mut self, value: impl Into<String>) -> Self {
                            self.access_token = Some(value.into());
                            self
                        }
                        #[doc = "JSONP"]
                        pub fn callback(mut self, value: impl Into<String>) -> Self {
                            self.callback = Some(value.into());
                            self
                        }
                        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                        pub fn key(mut self, value: impl Into<String>) -> Self {
                            self.key = Some(value.into());
                            self
                        }
                        #[doc = "OAuth 2.0 token for the current user."]
                        pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                            self.oauth_token = Some(value.into());
                            self
                        }
                        #[doc = "Returns response with indentations and line breaks."]
                        pub fn pretty_print(mut self, value: bool) -> Self {
                            self.pretty_print = Some(value);
                            self
                        }
                        #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                        pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                            self.quota_user = Some(value.into());
                            self
                        }
                        #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                        pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                            self.upload_protocol = Some(value.into());
                            self
                        }
                        #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                        pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                            self.upload_type = Some(value.into());
                            self
                        }
                        #[doc = "V1 error format."]
                        pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                            self.xgafv = Some(value);
                            self
                        }
                        #[doc = r" Execute the given operation. The fields requested are"]
                        #[doc = r" determined by the FieldSelector attribute of the return type."]
                        #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                        #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                        #[doc = r" are not generic over the return type and deserialize the"]
                        #[doc = r" response into an auto-generated struct will all possible"]
                        #[doc = r" fields."]
                        pub fn execute<T>(self) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned
                                + ::google_field_selector::FieldSelector,
                        {
                            let fields = ::google_field_selector::to_string::<T>();
                            let fields: Option<String> = if fields.is_empty() {
                                None
                            } else {
                                Some(fields)
                            };
                            self.execute_with_fields(fields)
                        }
                        #[doc = r" Execute the given operation. This will not provide any"]
                        #[doc = r" `fields` selector indicating that the server will determine"]
                        #[doc = r" the fields returned. This typically includes the most common"]
                        #[doc = r" fields, but it will not include every possible attribute of"]
                        #[doc = r" the response resource."]
                        pub fn execute_with_default_fields(
                            self,
                        ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error>
                        {
                            self.execute_with_fields(None::<&str>)
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]
                        pub fn execute_with_all_fields(
                            self,
                        ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error>
                        {
                            self.execute_with_fields(Some("*"))
                        }
                        #[doc = r" Execute the given operation. This will use the `fields`"]
                        #[doc = r" selector provided and will deserialize the response into"]
                        #[doc = r" whatever return value is provided."]
                        pub fn execute_with_fields<T, F>(
                            mut self,
                            fields: Option<F>,
                        ) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned,
                            F: Into<String>,
                        {
                            self.fields = fields.map(Into::into);
                            self._execute()
                        }
                        fn _execute<T>(&mut self) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned,
                        {
                            let req = self._request(&self._path())?;
                            let req = req.json(&self.request);
                            Ok(crate::error_from_response(req.send()?)?.json()?)
                        }
                        fn _path(&self) -> String {
                            let mut output = "https://ml.googleapis.com/".to_owned();
                            output.push_str("v1/");
                            {
                                let var_as_str = &self.parent;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::RESERVED,
                                ));
                            }
                            output.push_str("/trials:suggest");
                            output
                        }
                        fn _request(
                            &self,
                            path: &str,
                        ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error>
                        {
                            let req = self.reqwest.request(::reqwest::Method::POST, path);
                            let req = req.query(&[("access_token", &self.access_token)]);
                            let req = req.query(&[("alt", &self.alt)]);
                            let req = req.query(&[("callback", &self.callback)]);
                            let req = req.query(&[("fields", &self.fields)]);
                            let req = req.query(&[("key", &self.key)]);
                            let req = req.query(&[("oauth_token", &self.oauth_token)]);
                            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                            let req = req.query(&[("quotaUser", &self.quota_user)]);
                            let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                            let req = req.query(&[("uploadType", &self.upload_type)]);
                            let req = req.query(&[("$.xgafv", &self.xgafv)]);
                            let req = req.bearer_auth(
                                self.auth
                                    .access_token()
                                    .map_err(|err| crate::Error::OAuth2(err))?,
                            );
                            Ok(req)
                        }
                    }
                }
            }
        }
        pub mod models {
            pub mod params {}
            pub struct ModelsActions<'a> {
                pub(crate) reqwest: &'a reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> ModelsActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Creates a model which will later contain one or more versions.\n\nYou must add at least one version before you can request predictions from\nthe model. Add versions by calling\nprojects.models.versions.create."]
                pub fn create(
                    &self,
                    request: crate::schemas::GoogleCloudMlV1Model,
                    parent: impl Into<String>,
                ) -> CreateRequestBuilder {
                    CreateRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                        request,
                        access_token: None,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        parent: parent.into(),
                    }
                }
                #[doc = "Deletes a model.\n\nYou can only delete a model if there are no versions in it. You can delete\nversions by calling\nprojects.models.versions.delete."]
                pub fn delete(&self, name: impl Into<String>) -> DeleteRequestBuilder {
                    DeleteRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                        access_token: None,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        name: name.into(),
                    }
                }
                #[doc = "Gets information about a model, including its name, the description (if\nset), and the default version (if at least one version of the model has\nbeen deployed)."]
                pub fn get(&self, name: impl Into<String>) -> GetRequestBuilder {
                    GetRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                        access_token: None,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        name: name.into(),
                    }
                }
                #[doc = "Gets the access control policy for a resource.\nReturns an empty policy if the resource exists and does not have a policy\nset."]
                pub fn get_iam_policy(
                    &self,
                    resource: impl Into<String>,
                ) -> GetIamPolicyRequestBuilder {
                    GetIamPolicyRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                        access_token: None,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        resource: resource.into(),
                        options_requested_policy_version: None,
                    }
                }
                #[doc = "Lists the models in a project.\n\nEach project can contain multiple models, and each model can have multiple\nversions.\n\nIf there are no models that match the request parameters, the list request\nreturns an empty response body: {}."]
                pub fn list(&self, parent: impl Into<String>) -> ListRequestBuilder {
                    ListRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                        access_token: None,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        parent: parent.into(),
                        filter: None,
                        page_size: None,
                        page_token: None,
                    }
                }
                #[doc = "Updates a specific model resource.\n\nCurrently the only supported fields to update are `description` and\n`default_version.name`."]
                pub fn patch(
                    &self,
                    request: crate::schemas::GoogleCloudMlV1Model,
                    name: impl Into<String>,
                ) -> PatchRequestBuilder {
                    PatchRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                        request,
                        access_token: None,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        name: name.into(),
                        update_mask: None,
                    }
                }
                #[doc = "Sets the access control policy on the specified resource. Replaces any\nexisting policy.\n\nCan return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors."]
                pub fn set_iam_policy(
                    &self,
                    request: crate::schemas::GoogleIamV1SetIamPolicyRequest,
                    resource: impl Into<String>,
                ) -> SetIamPolicyRequestBuilder {
                    SetIamPolicyRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                        request,
                        access_token: None,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        resource: resource.into(),
                    }
                }
                #[doc = "Returns permissions that a caller has on the specified resource.\nIf the resource does not exist, this will return an empty set of\npermissions, not a `NOT_FOUND` error.\n\nNote: This operation is designed to be used for building permission-aware\nUIs and command-line tools, not for authorization checking. This operation\nmay \"fail open\" without warning."]
                pub fn test_iam_permissions(
                    &self,
                    request: crate::schemas::GoogleIamV1TestIamPermissionsRequest,
                    resource: impl Into<String>,
                ) -> TestIamPermissionsRequestBuilder {
                    TestIamPermissionsRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                        request,
                        access_token: None,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        resource: resource.into(),
                    }
                }
                #[doc = "Actions that can be performed on the versions resource"]
                pub fn versions(
                    &self,
                ) -> crate::resources::projects::models::versions::VersionsActions {
                    crate::resources::projects::models::versions::VersionsActions {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                    }
                }
            }
            #[doc = "Created via [ModelsActions::create()](struct.ModelsActions.html#method.create)"]
            #[derive(Debug, Clone)]
            pub struct CreateRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::GoogleCloudMlV1Model,
                parent: String,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a> CreateRequestBuilder<'a> {
                #[doc = "OAuth access token."]
                pub fn access_token(mut self, value: impl Into<String>) -> Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(mut self, value: impl Into<String>) -> Self {
                    self.callback = Some(value.into());
                    self
                }
                #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                pub fn key(mut self, value: impl Into<String>) -> Self {
                    self.key = Some(value.into());
                    self
                }
                #[doc = "OAuth 2.0 token for the current user."]
                pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                    self.oauth_token = Some(value.into());
                    self
                }
                #[doc = "Returns response with indentations and line breaks."]
                pub fn pretty_print(mut self, value: bool) -> Self {
                    self.pretty_print = Some(value);
                    self
                }
                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                    self.xgafv = Some(value);
                    self
                }
                #[doc = r" Execute the given operation. The fields requested are"]
                #[doc = r" determined by the FieldSelector attribute of the return type."]
                #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                #[doc = r" are not generic over the return type and deserialize the"]
                #[doc = r" response into an auto-generated struct will all possible"]
                #[doc = r" fields."]
                pub fn execute<T>(self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_with_fields(fields)
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub fn execute_with_default_fields(
                    self,
                ) -> Result<crate::schemas::GoogleCloudMlV1Model, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GoogleCloudMlV1Model, crate::Error> {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute()
                }
                fn _execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path())?;
                    let req = req.json(&self.request);
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://ml.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/models");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let req = self.reqwest.request(::reqwest::Method::POST, path);
                    let req = req.query(&[("access_token", &self.access_token)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("callback", &self.callback)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    let req = req.query(&[("uploadType", &self.upload_type)]);
                    let req = req.query(&[("$.xgafv", &self.xgafv)]);
                    let req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
                    Ok(req)
                }
            }
            #[doc = "Created via [ModelsActions::delete()](struct.ModelsActions.html#method.delete)"]
            #[derive(Debug, Clone)]
            pub struct DeleteRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                name: String,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a> DeleteRequestBuilder<'a> {
                #[doc = "OAuth access token."]
                pub fn access_token(mut self, value: impl Into<String>) -> Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(mut self, value: impl Into<String>) -> Self {
                    self.callback = Some(value.into());
                    self
                }
                #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                pub fn key(mut self, value: impl Into<String>) -> Self {
                    self.key = Some(value.into());
                    self
                }
                #[doc = "OAuth 2.0 token for the current user."]
                pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                    self.oauth_token = Some(value.into());
                    self
                }
                #[doc = "Returns response with indentations and line breaks."]
                pub fn pretty_print(mut self, value: bool) -> Self {
                    self.pretty_print = Some(value);
                    self
                }
                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                    self.xgafv = Some(value);
                    self
                }
                #[doc = r" Execute the given operation. The fields requested are"]
                #[doc = r" determined by the FieldSelector attribute of the return type."]
                #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                #[doc = r" are not generic over the return type and deserialize the"]
                #[doc = r" response into an auto-generated struct will all possible"]
                #[doc = r" fields."]
                pub fn execute<T>(self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_with_fields(fields)
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub fn execute_with_default_fields(
                    self,
                ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error>
                {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute()
                }
                fn _execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path())?;
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://ml.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let req = self.reqwest.request(::reqwest::Method::DELETE, path);
                    let req = req.query(&[("access_token", &self.access_token)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("callback", &self.callback)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    let req = req.query(&[("uploadType", &self.upload_type)]);
                    let req = req.query(&[("$.xgafv", &self.xgafv)]);
                    let req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
                    Ok(req)
                }
            }
            #[doc = "Created via [ModelsActions::get()](struct.ModelsActions.html#method.get)"]
            #[derive(Debug, Clone)]
            pub struct GetRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                name: String,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a> GetRequestBuilder<'a> {
                #[doc = "OAuth access token."]
                pub fn access_token(mut self, value: impl Into<String>) -> Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(mut self, value: impl Into<String>) -> Self {
                    self.callback = Some(value.into());
                    self
                }
                #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                pub fn key(mut self, value: impl Into<String>) -> Self {
                    self.key = Some(value.into());
                    self
                }
                #[doc = "OAuth 2.0 token for the current user."]
                pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                    self.oauth_token = Some(value.into());
                    self
                }
                #[doc = "Returns response with indentations and line breaks."]
                pub fn pretty_print(mut self, value: bool) -> Self {
                    self.pretty_print = Some(value);
                    self
                }
                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                    self.xgafv = Some(value);
                    self
                }
                #[doc = r" Execute the given operation. The fields requested are"]
                #[doc = r" determined by the FieldSelector attribute of the return type."]
                #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                #[doc = r" are not generic over the return type and deserialize the"]
                #[doc = r" response into an auto-generated struct will all possible"]
                #[doc = r" fields."]
                pub fn execute<T>(self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_with_fields(fields)
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub fn execute_with_default_fields(
                    self,
                ) -> Result<crate::schemas::GoogleCloudMlV1Model, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GoogleCloudMlV1Model, crate::Error> {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute()
                }
                fn _execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path())?;
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://ml.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("access_token", &self.access_token)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("callback", &self.callback)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    let req = req.query(&[("uploadType", &self.upload_type)]);
                    let req = req.query(&[("$.xgafv", &self.xgafv)]);
                    let req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
                    Ok(req)
                }
            }
            #[doc = "Created via [ModelsActions::get_iam_policy()](struct.ModelsActions.html#method.get_iam_policy)"]
            #[derive(Debug, Clone)]
            pub struct GetIamPolicyRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                resource: String,
                options_requested_policy_version: Option<i32>,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a> GetIamPolicyRequestBuilder<'a> {
                #[doc = "Optional. The policy format version to be returned.\n\nValid values are 0, 1, and 3. Requests specifying an invalid value will be\nrejected.\n\nRequests for policies with any conditional bindings must specify version 3.\nPolicies without any conditional bindings may specify any valid value or\nleave the field unset.\n\nTo learn which resources support conditions in their IAM policies, see the\n[IAM\ndocumentation](https://cloud.google.com/iam/help/conditions/resource-policies)."]
                pub fn options_requested_policy_version(mut self, value: i32) -> Self {
                    self.options_requested_policy_version = Some(value);
                    self
                }
                #[doc = "OAuth access token."]
                pub fn access_token(mut self, value: impl Into<String>) -> Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(mut self, value: impl Into<String>) -> Self {
                    self.callback = Some(value.into());
                    self
                }
                #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                pub fn key(mut self, value: impl Into<String>) -> Self {
                    self.key = Some(value.into());
                    self
                }
                #[doc = "OAuth 2.0 token for the current user."]
                pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                    self.oauth_token = Some(value.into());
                    self
                }
                #[doc = "Returns response with indentations and line breaks."]
                pub fn pretty_print(mut self, value: bool) -> Self {
                    self.pretty_print = Some(value);
                    self
                }
                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                    self.xgafv = Some(value);
                    self
                }
                #[doc = r" Execute the given operation. The fields requested are"]
                #[doc = r" determined by the FieldSelector attribute of the return type."]
                #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                #[doc = r" are not generic over the return type and deserialize the"]
                #[doc = r" response into an auto-generated struct will all possible"]
                #[doc = r" fields."]
                pub fn execute<T>(self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_with_fields(fields)
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub fn execute_with_default_fields(
                    self,
                ) -> Result<crate::schemas::GoogleIamV1Policy, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GoogleIamV1Policy, crate::Error> {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute()
                }
                fn _execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path())?;
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://ml.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.resource;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str(":getIamPolicy");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[(
                        "options.requestedPolicyVersion",
                        &self.options_requested_policy_version,
                    )]);
                    let req = req.query(&[("access_token", &self.access_token)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("callback", &self.callback)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    let req = req.query(&[("uploadType", &self.upload_type)]);
                    let req = req.query(&[("$.xgafv", &self.xgafv)]);
                    let req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
                    Ok(req)
                }
            }
            #[doc = "Created via [ModelsActions::list()](struct.ModelsActions.html#method.list)"]
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                parent: String,
                filter: Option<String>,
                page_size: Option<i32>,
                page_token: Option<String>,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a> ListRequestBuilder<'a> {
                #[doc = "Optional. Specifies the subset of models to retrieve."]
                pub fn filter(mut self, value: impl Into<String>) -> Self {
                    self.filter = Some(value.into());
                    self
                }
                #[doc = "Optional. The number of models to retrieve per \"page\" of results. If there\nare more remaining results than this number, the response message will\ncontain a valid value in the `next_page_token` field.\n\nThe default value is 20, and the maximum page size is 100."]
                pub fn page_size(mut self, value: i32) -> Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "Optional. A page token to request the next page of results.\n\nYou get the token from the `next_page_token` field of the response from\nthe previous call."]
                pub fn page_token(mut self, value: impl Into<String>) -> Self {
                    self.page_token = Some(value.into());
                    self
                }
                #[doc = "OAuth access token."]
                pub fn access_token(mut self, value: impl Into<String>) -> Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(mut self, value: impl Into<String>) -> Self {
                    self.callback = Some(value.into());
                    self
                }
                #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                pub fn key(mut self, value: impl Into<String>) -> Self {
                    self.key = Some(value.into());
                    self
                }
                #[doc = "OAuth 2.0 token for the current user."]
                pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                    self.oauth_token = Some(value.into());
                    self
                }
                #[doc = "Returns response with indentations and line breaks."]
                pub fn pretty_print(mut self, value: bool) -> Self {
                    self.pretty_print = Some(value);
                    self
                }
                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                    self.xgafv = Some(value);
                    self
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are chosen by the caller of this"]
                #[doc = r" method and must implement `Deserialize` and `FieldSelector`. The"]
                #[doc = r" populated fields in the yielded items will be determined by the"]
                #[doc = r" `FieldSelector` implementation."]
                pub fn iter_models<T>(self) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.iter_models_with_fields(fields)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be the default fields populated by"]
                #[doc = r" the server."]
                pub fn iter_models_with_default_fields(
                    self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::GoogleCloudMlV1Model>
                {
                    self.iter_models_with_fields(None::<String>)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be all fields available. This should"]
                #[doc = r" primarily be used during developement and debugging as fetching"]
                #[doc = r" all fields can be expensive both in bandwidth and server"]
                #[doc = r" resources."]
                pub fn iter_models_with_all_fields(
                    self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::GoogleCloudMlV1Model>
                {
                    self.iter_models_with_fields(Some("*"))
                }
                pub fn iter_models_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: AsRef<str>,
                {
                    self.fields = Some({
                        let mut selector = concat!("nextPageToken,", "models").to_owned();
                        let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                        if !items_fields.is_empty() {
                            selector.push_str("(");
                            selector.push_str(items_fields);
                            selector.push_str(")");
                        }
                        selector
                    });
                    crate::iter::PageItemIter::new(self, "models")
                }
                pub fn iter<T>(self) -> crate::iter::PageIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.iter_with_fields(fields)
                }
                pub fn iter_with_default_fields(
                    self,
                ) -> crate::iter::PageIter<Self, crate::schemas::GoogleCloudMlV1ListModelsResponse>
                {
                    self.iter_with_fields(None::<&str>)
                }
                pub fn iter_with_all_fields(
                    self,
                ) -> crate::iter::PageIter<Self, crate::schemas::GoogleCloudMlV1ListModelsResponse>
                {
                    self.iter_with_fields(Some("*"))
                }
                pub fn iter_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> crate::iter::PageIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: AsRef<str>,
                {
                    let mut fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("").to_owned();
                    if !fields.is_empty() {
                        match fields.chars().rev().nth(0) {
                            Some(',') | None => {}
                            _ => fields.push_str(","),
                        }
                        fields.push_str("nextPageToken");
                        self.fields = Some(fields);
                    }
                    crate::iter::PageIter::new(self)
                }
                #[doc = r" Execute the given operation. The fields requested are"]
                #[doc = r" determined by the FieldSelector attribute of the return type."]
                #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                #[doc = r" are not generic over the return type and deserialize the"]
                #[doc = r" response into an auto-generated struct will all possible"]
                #[doc = r" fields."]
                pub fn execute<T>(self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_with_fields(fields)
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub fn execute_with_default_fields(
                    self,
                ) -> Result<crate::schemas::GoogleCloudMlV1ListModelsResponse, crate::Error>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GoogleCloudMlV1ListModelsResponse, crate::Error>
                {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute()
                }
                fn _execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path())?;
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://ml.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/models");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("filter", &self.filter)]);
                    let req = req.query(&[("pageSize", &self.page_size)]);
                    let req = req.query(&[("pageToken", &self.page_token)]);
                    let req = req.query(&[("access_token", &self.access_token)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("callback", &self.callback)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    let req = req.query(&[("uploadType", &self.upload_type)]);
                    let req = req.query(&[("$.xgafv", &self.xgafv)]);
                    let req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
                    Ok(req)
                }
            }
            impl<'a> crate::iter::IterableMethod for ListRequestBuilder<'a> {
                fn set_page_token(&mut self, value: String) {
                    self.page_token = value.into();
                }
                fn execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    self._execute()
                }
            }
            #[doc = "Created via [ModelsActions::patch()](struct.ModelsActions.html#method.patch)"]
            #[derive(Debug, Clone)]
            pub struct PatchRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::GoogleCloudMlV1Model,
                name: String,
                update_mask: Option<String>,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a> PatchRequestBuilder<'a> {
                #[doc = "Required. Specifies the path, relative to `Model`, of the field to update.\n\nFor example, to change the description of a model to \"foo\" and set its\ndefault version to \"version_1\", the `update_mask` parameter would be\nspecified as `description`, `default_version.name`, and the `PATCH`\nrequest body would specify the new value, as follows:\n{\n\"description\": \"foo\",\n\"defaultVersion\": {\n\"name\":\"version_1\"\n}\n}\n\nCurrently the supported update masks are `description` and\n`default_version.name`."]
                pub fn update_mask(mut self, value: impl Into<String>) -> Self {
                    self.update_mask = Some(value.into());
                    self
                }
                #[doc = "OAuth access token."]
                pub fn access_token(mut self, value: impl Into<String>) -> Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(mut self, value: impl Into<String>) -> Self {
                    self.callback = Some(value.into());
                    self
                }
                #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                pub fn key(mut self, value: impl Into<String>) -> Self {
                    self.key = Some(value.into());
                    self
                }
                #[doc = "OAuth 2.0 token for the current user."]
                pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                    self.oauth_token = Some(value.into());
                    self
                }
                #[doc = "Returns response with indentations and line breaks."]
                pub fn pretty_print(mut self, value: bool) -> Self {
                    self.pretty_print = Some(value);
                    self
                }
                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                    self.xgafv = Some(value);
                    self
                }
                #[doc = r" Execute the given operation. The fields requested are"]
                #[doc = r" determined by the FieldSelector attribute of the return type."]
                #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                #[doc = r" are not generic over the return type and deserialize the"]
                #[doc = r" response into an auto-generated struct will all possible"]
                #[doc = r" fields."]
                pub fn execute<T>(self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_with_fields(fields)
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub fn execute_with_default_fields(
                    self,
                ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error>
                {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute()
                }
                fn _execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path())?;
                    let req = req.json(&self.request);
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://ml.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let req = self.reqwest.request(::reqwest::Method::PATCH, path);
                    let req = req.query(&[("updateMask", &self.update_mask)]);
                    let req = req.query(&[("access_token", &self.access_token)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("callback", &self.callback)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    let req = req.query(&[("uploadType", &self.upload_type)]);
                    let req = req.query(&[("$.xgafv", &self.xgafv)]);
                    let req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
                    Ok(req)
                }
            }
            #[doc = "Created via [ModelsActions::set_iam_policy()](struct.ModelsActions.html#method.set_iam_policy)"]
            #[derive(Debug, Clone)]
            pub struct SetIamPolicyRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::GoogleIamV1SetIamPolicyRequest,
                resource: String,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a> SetIamPolicyRequestBuilder<'a> {
                #[doc = "OAuth access token."]
                pub fn access_token(mut self, value: impl Into<String>) -> Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(mut self, value: impl Into<String>) -> Self {
                    self.callback = Some(value.into());
                    self
                }
                #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                pub fn key(mut self, value: impl Into<String>) -> Self {
                    self.key = Some(value.into());
                    self
                }
                #[doc = "OAuth 2.0 token for the current user."]
                pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                    self.oauth_token = Some(value.into());
                    self
                }
                #[doc = "Returns response with indentations and line breaks."]
                pub fn pretty_print(mut self, value: bool) -> Self {
                    self.pretty_print = Some(value);
                    self
                }
                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                    self.xgafv = Some(value);
                    self
                }
                #[doc = r" Execute the given operation. The fields requested are"]
                #[doc = r" determined by the FieldSelector attribute of the return type."]
                #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                #[doc = r" are not generic over the return type and deserialize the"]
                #[doc = r" response into an auto-generated struct will all possible"]
                #[doc = r" fields."]
                pub fn execute<T>(self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_with_fields(fields)
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub fn execute_with_default_fields(
                    self,
                ) -> Result<crate::schemas::GoogleIamV1Policy, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GoogleIamV1Policy, crate::Error> {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute()
                }
                fn _execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path())?;
                    let req = req.json(&self.request);
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://ml.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.resource;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str(":setIamPolicy");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let req = self.reqwest.request(::reqwest::Method::POST, path);
                    let req = req.query(&[("access_token", &self.access_token)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("callback", &self.callback)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    let req = req.query(&[("uploadType", &self.upload_type)]);
                    let req = req.query(&[("$.xgafv", &self.xgafv)]);
                    let req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
                    Ok(req)
                }
            }
            #[doc = "Created via [ModelsActions::test_iam_permissions()](struct.ModelsActions.html#method.test_iam_permissions)"]
            #[derive(Debug, Clone)]
            pub struct TestIamPermissionsRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::GoogleIamV1TestIamPermissionsRequest,
                resource: String,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a> TestIamPermissionsRequestBuilder<'a> {
                #[doc = "OAuth access token."]
                pub fn access_token(mut self, value: impl Into<String>) -> Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(mut self, value: impl Into<String>) -> Self {
                    self.callback = Some(value.into());
                    self
                }
                #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                pub fn key(mut self, value: impl Into<String>) -> Self {
                    self.key = Some(value.into());
                    self
                }
                #[doc = "OAuth 2.0 token for the current user."]
                pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                    self.oauth_token = Some(value.into());
                    self
                }
                #[doc = "Returns response with indentations and line breaks."]
                pub fn pretty_print(mut self, value: bool) -> Self {
                    self.pretty_print = Some(value);
                    self
                }
                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                    self.xgafv = Some(value);
                    self
                }
                #[doc = r" Execute the given operation. The fields requested are"]
                #[doc = r" determined by the FieldSelector attribute of the return type."]
                #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                #[doc = r" are not generic over the return type and deserialize the"]
                #[doc = r" response into an auto-generated struct will all possible"]
                #[doc = r" fields."]
                pub fn execute<T>(self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_with_fields(fields)
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub fn execute_with_default_fields(
                    self,
                ) -> Result<crate::schemas::GoogleIamV1TestIamPermissionsResponse, crate::Error>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GoogleIamV1TestIamPermissionsResponse, crate::Error>
                {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute()
                }
                fn _execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path())?;
                    let req = req.json(&self.request);
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://ml.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.resource;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str(":testIamPermissions");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let req = self.reqwest.request(::reqwest::Method::POST, path);
                    let req = req.query(&[("access_token", &self.access_token)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("callback", &self.callback)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    let req = req.query(&[("uploadType", &self.upload_type)]);
                    let req = req.query(&[("$.xgafv", &self.xgafv)]);
                    let req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
                    Ok(req)
                }
            }
            pub mod versions {
                pub mod params {}
                pub struct VersionsActions<'a> {
                    pub(crate) reqwest: &'a reqwest::blocking::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                }
                impl<'a> VersionsActions<'a> {
                    fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                        self.auth
                    }
                    #[doc = "Creates a new version of a model from a trained TensorFlow model.\n\nIf the version created in the cloud by this call is the first deployed\nversion of the specified model, it will be made the default version of the\nmodel. When you add a version to a model that already has one or more\nversions, the default version does not automatically change. If you want a\nnew version to be the default, you must call\nprojects.models.versions.setDefault."]
                    pub fn create(
                        &self,
                        request: crate::schemas::GoogleCloudMlV1Version,
                        parent: impl Into<String>,
                    ) -> CreateRequestBuilder {
                        CreateRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: self.auth_ref(),
                            request,
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            parent: parent.into(),
                        }
                    }
                    #[doc = "Deletes a model version.\n\nEach model can have multiple versions deployed and in use at any given\ntime. Use this method to remove a single version.\n\nNote: You cannot delete the version that is set as the default version\nof the model unless it is the only remaining version."]
                    pub fn delete(&self, name: impl Into<String>) -> DeleteRequestBuilder {
                        DeleteRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: self.auth_ref(),
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            name: name.into(),
                        }
                    }
                    #[doc = "Gets information about a model version.\n\nModels can have multiple versions. You can call\nprojects.models.versions.list\nto get the same information that this method returns for all of the\nversions of a model."]
                    pub fn get(&self, name: impl Into<String>) -> GetRequestBuilder {
                        GetRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: self.auth_ref(),
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            name: name.into(),
                        }
                    }
                    #[doc = "Gets basic information about all the versions of a model.\n\nIf you expect that a model has many versions, or if you need to handle\nonly a limited number of results at a time, you can request that the list\nbe retrieved in batches (called pages).\n\nIf there are no versions that match the request parameters, the list\nrequest returns an empty response body: {}."]
                    pub fn list(&self, parent: impl Into<String>) -> ListRequestBuilder {
                        ListRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: self.auth_ref(),
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            parent: parent.into(),
                            filter: None,
                            page_size: None,
                            page_token: None,
                        }
                    }
                    #[doc = "Updates the specified Version resource.\n\nCurrently the only update-able fields are `description`,\n`requestLoggingConfig`, `autoScaling.minNodes`, and `manualScaling.nodes`."]
                    pub fn patch(
                        &self,
                        request: crate::schemas::GoogleCloudMlV1Version,
                        name: impl Into<String>,
                    ) -> PatchRequestBuilder {
                        PatchRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: self.auth_ref(),
                            request,
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            name: name.into(),
                            update_mask: None,
                        }
                    }
                    #[doc = "Designates a version to be the default for the model.\n\nThe default version is used for prediction requests made against the model\nthat don't specify a version.\n\nThe first version to be created for a model is automatically set as the\ndefault. You must make any subsequent changes to the default version\nsetting manually using this method."]
                    pub fn set_default(
                        &self,
                        request: crate::schemas::GoogleCloudMlV1SetDefaultVersionRequest,
                        name: impl Into<String>,
                    ) -> SetDefaultRequestBuilder {
                        SetDefaultRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: self.auth_ref(),
                            request,
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            name: name.into(),
                        }
                    }
                }
                #[doc = "Created via [VersionsActions::create()](struct.VersionsActions.html#method.create)"]
                #[derive(Debug, Clone)]
                pub struct CreateRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    request: crate::schemas::GoogleCloudMlV1Version,
                    parent: String,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a> CreateRequestBuilder<'a> {
                    #[doc = "OAuth access token."]
                    pub fn access_token(mut self, value: impl Into<String>) -> Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(mut self, value: impl Into<String>) -> Self {
                        self.callback = Some(value.into());
                        self
                    }
                    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                    pub fn key(mut self, value: impl Into<String>) -> Self {
                        self.key = Some(value.into());
                        self
                    }
                    #[doc = "OAuth 2.0 token for the current user."]
                    pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                        self.oauth_token = Some(value.into());
                        self
                    }
                    #[doc = "Returns response with indentations and line breaks."]
                    pub fn pretty_print(mut self, value: bool) -> Self {
                        self.pretty_print = Some(value);
                        self
                    }
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                        self.xgafv = Some(value);
                        self
                    }
                    #[doc = r" Execute the given operation. The fields requested are"]
                    #[doc = r" determined by the FieldSelector attribute of the return type."]
                    #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                    #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                    #[doc = r" are not generic over the return type and deserialize the"]
                    #[doc = r" response into an auto-generated struct will all possible"]
                    #[doc = r" fields."]
                    pub fn execute<T>(self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.execute_with_fields(fields)
                    }
                    #[doc = r" Execute the given operation. This will not provide any"]
                    #[doc = r" `fields` selector indicating that the server will determine"]
                    #[doc = r" the fields returned. This typically includes the most common"]
                    #[doc = r" fields, but it will not include every possible attribute of"]
                    #[doc = r" the response resource."]
                    pub fn execute_with_default_fields(
                        self,
                    ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error>
                    {
                        self.execute_with_fields(Some("*"))
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub fn execute_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: Into<String>,
                    {
                        self.fields = fields.map(Into::into);
                        self._execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                    {
                        let req = self._request(&self._path())?;
                        let req = req.json(&self.request);
                        Ok(crate::error_from_response(req.send()?)?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://ml.googleapis.com/".to_owned();
                        output.push_str("v1/");
                        {
                            let var_as_str = &self.parent;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/versions");
                        output
                    }
                    fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error>
                    {
                        let req = self.reqwest.request(::reqwest::Method::POST, path);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let req = req.bearer_auth(
                            self.auth
                                .access_token()
                                .map_err(|err| crate::Error::OAuth2(err))?,
                        );
                        Ok(req)
                    }
                }
                #[doc = "Created via [VersionsActions::delete()](struct.VersionsActions.html#method.delete)"]
                #[derive(Debug, Clone)]
                pub struct DeleteRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    name: String,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a> DeleteRequestBuilder<'a> {
                    #[doc = "OAuth access token."]
                    pub fn access_token(mut self, value: impl Into<String>) -> Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(mut self, value: impl Into<String>) -> Self {
                        self.callback = Some(value.into());
                        self
                    }
                    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                    pub fn key(mut self, value: impl Into<String>) -> Self {
                        self.key = Some(value.into());
                        self
                    }
                    #[doc = "OAuth 2.0 token for the current user."]
                    pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                        self.oauth_token = Some(value.into());
                        self
                    }
                    #[doc = "Returns response with indentations and line breaks."]
                    pub fn pretty_print(mut self, value: bool) -> Self {
                        self.pretty_print = Some(value);
                        self
                    }
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                        self.xgafv = Some(value);
                        self
                    }
                    #[doc = r" Execute the given operation. The fields requested are"]
                    #[doc = r" determined by the FieldSelector attribute of the return type."]
                    #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                    #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                    #[doc = r" are not generic over the return type and deserialize the"]
                    #[doc = r" response into an auto-generated struct will all possible"]
                    #[doc = r" fields."]
                    pub fn execute<T>(self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.execute_with_fields(fields)
                    }
                    #[doc = r" Execute the given operation. This will not provide any"]
                    #[doc = r" `fields` selector indicating that the server will determine"]
                    #[doc = r" the fields returned. This typically includes the most common"]
                    #[doc = r" fields, but it will not include every possible attribute of"]
                    #[doc = r" the response resource."]
                    pub fn execute_with_default_fields(
                        self,
                    ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error>
                    {
                        self.execute_with_fields(Some("*"))
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub fn execute_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: Into<String>,
                    {
                        self.fields = fields.map(Into::into);
                        self._execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                    {
                        let req = self._request(&self._path())?;
                        Ok(crate::error_from_response(req.send()?)?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://ml.googleapis.com/".to_owned();
                        output.push_str("v1/");
                        {
                            let var_as_str = &self.name;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output
                    }
                    fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error>
                    {
                        let req = self.reqwest.request(::reqwest::Method::DELETE, path);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let req = req.bearer_auth(
                            self.auth
                                .access_token()
                                .map_err(|err| crate::Error::OAuth2(err))?,
                        );
                        Ok(req)
                    }
                }
                #[doc = "Created via [VersionsActions::get()](struct.VersionsActions.html#method.get)"]
                #[derive(Debug, Clone)]
                pub struct GetRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    name: String,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a> GetRequestBuilder<'a> {
                    #[doc = "OAuth access token."]
                    pub fn access_token(mut self, value: impl Into<String>) -> Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(mut self, value: impl Into<String>) -> Self {
                        self.callback = Some(value.into());
                        self
                    }
                    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                    pub fn key(mut self, value: impl Into<String>) -> Self {
                        self.key = Some(value.into());
                        self
                    }
                    #[doc = "OAuth 2.0 token for the current user."]
                    pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                        self.oauth_token = Some(value.into());
                        self
                    }
                    #[doc = "Returns response with indentations and line breaks."]
                    pub fn pretty_print(mut self, value: bool) -> Self {
                        self.pretty_print = Some(value);
                        self
                    }
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                        self.xgafv = Some(value);
                        self
                    }
                    #[doc = r" Execute the given operation. The fields requested are"]
                    #[doc = r" determined by the FieldSelector attribute of the return type."]
                    #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                    #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                    #[doc = r" are not generic over the return type and deserialize the"]
                    #[doc = r" response into an auto-generated struct will all possible"]
                    #[doc = r" fields."]
                    pub fn execute<T>(self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.execute_with_fields(fields)
                    }
                    #[doc = r" Execute the given operation. This will not provide any"]
                    #[doc = r" `fields` selector indicating that the server will determine"]
                    #[doc = r" the fields returned. This typically includes the most common"]
                    #[doc = r" fields, but it will not include every possible attribute of"]
                    #[doc = r" the response resource."]
                    pub fn execute_with_default_fields(
                        self,
                    ) -> Result<crate::schemas::GoogleCloudMlV1Version, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::GoogleCloudMlV1Version, crate::Error>
                    {
                        self.execute_with_fields(Some("*"))
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub fn execute_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: Into<String>,
                    {
                        self.fields = fields.map(Into::into);
                        self._execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                    {
                        let req = self._request(&self._path())?;
                        Ok(crate::error_from_response(req.send()?)?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://ml.googleapis.com/".to_owned();
                        output.push_str("v1/");
                        {
                            let var_as_str = &self.name;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output
                    }
                    fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error>
                    {
                        let req = self.reqwest.request(::reqwest::Method::GET, path);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let req = req.bearer_auth(
                            self.auth
                                .access_token()
                                .map_err(|err| crate::Error::OAuth2(err))?,
                        );
                        Ok(req)
                    }
                }
                #[doc = "Created via [VersionsActions::list()](struct.VersionsActions.html#method.list)"]
                #[derive(Debug, Clone)]
                pub struct ListRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    parent: String,
                    filter: Option<String>,
                    page_size: Option<i32>,
                    page_token: Option<String>,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a> ListRequestBuilder<'a> {
                    #[doc = "Optional. Specifies the subset of versions to retrieve."]
                    pub fn filter(mut self, value: impl Into<String>) -> Self {
                        self.filter = Some(value.into());
                        self
                    }
                    #[doc = "Optional. The number of versions to retrieve per \"page\" of results. If\nthere are more remaining results than this number, the response message\nwill contain a valid value in the `next_page_token` field.\n\nThe default value is 20, and the maximum page size is 100."]
                    pub fn page_size(mut self, value: i32) -> Self {
                        self.page_size = Some(value);
                        self
                    }
                    #[doc = "Optional. A page token to request the next page of results.\n\nYou get the token from the `next_page_token` field of the response from\nthe previous call."]
                    pub fn page_token(mut self, value: impl Into<String>) -> Self {
                        self.page_token = Some(value.into());
                        self
                    }
                    #[doc = "OAuth access token."]
                    pub fn access_token(mut self, value: impl Into<String>) -> Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(mut self, value: impl Into<String>) -> Self {
                        self.callback = Some(value.into());
                        self
                    }
                    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                    pub fn key(mut self, value: impl Into<String>) -> Self {
                        self.key = Some(value.into());
                        self
                    }
                    #[doc = "OAuth 2.0 token for the current user."]
                    pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                        self.oauth_token = Some(value.into());
                        self
                    }
                    #[doc = "Returns response with indentations and line breaks."]
                    pub fn pretty_print(mut self, value: bool) -> Self {
                        self.pretty_print = Some(value);
                        self
                    }
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                        self.xgafv = Some(value);
                        self
                    }
                    #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                    #[doc = r" items yielded by the iterator are chosen by the caller of this"]
                    #[doc = r" method and must implement `Deserialize` and `FieldSelector`. The"]
                    #[doc = r" populated fields in the yielded items will be determined by the"]
                    #[doc = r" `FieldSelector` implementation."]
                    pub fn iter_versions<T>(self) -> crate::iter::PageItemIter<Self, T>
                    where
                        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.iter_versions_with_fields(fields)
                    }
                    #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                    #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                    #[doc = r" fields in `#items_type` will be the default fields populated by"]
                    #[doc = r" the server."]
                    pub fn iter_versions_with_default_fields(
                        self,
                    ) -> crate::iter::PageItemIter<Self, crate::schemas::GoogleCloudMlV1Version>
                    {
                        self.iter_versions_with_fields(None::<String>)
                    }
                    #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                    #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                    #[doc = r" fields in `#items_type` will be all fields available. This should"]
                    #[doc = r" primarily be used during developement and debugging as fetching"]
                    #[doc = r" all fields can be expensive both in bandwidth and server"]
                    #[doc = r" resources."]
                    pub fn iter_versions_with_all_fields(
                        self,
                    ) -> crate::iter::PageItemIter<Self, crate::schemas::GoogleCloudMlV1Version>
                    {
                        self.iter_versions_with_fields(Some("*"))
                    }
                    pub fn iter_versions_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> crate::iter::PageItemIter<Self, T>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: AsRef<str>,
                    {
                        self.fields = Some({
                            let mut selector = concat!("nextPageToken,", "versions").to_owned();
                            let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                            if !items_fields.is_empty() {
                                selector.push_str("(");
                                selector.push_str(items_fields);
                                selector.push_str(")");
                            }
                            selector
                        });
                        crate::iter::PageItemIter::new(self, "versions")
                    }
                    pub fn iter<T>(self) -> crate::iter::PageIter<Self, T>
                    where
                        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.iter_with_fields(fields)
                    }
                    pub fn iter_with_default_fields(
                        self,
                    ) -> crate::iter::PageIter<
                        Self,
                        crate::schemas::GoogleCloudMlV1ListVersionsResponse,
                    > {
                        self.iter_with_fields(None::<&str>)
                    }
                    pub fn iter_with_all_fields(
                        self,
                    ) -> crate::iter::PageIter<
                        Self,
                        crate::schemas::GoogleCloudMlV1ListVersionsResponse,
                    > {
                        self.iter_with_fields(Some("*"))
                    }
                    pub fn iter_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> crate::iter::PageIter<Self, T>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: AsRef<str>,
                    {
                        let mut fields =
                            fields.as_ref().map(|x| x.as_ref()).unwrap_or("").to_owned();
                        if !fields.is_empty() {
                            match fields.chars().rev().nth(0) {
                                Some(',') | None => {}
                                _ => fields.push_str(","),
                            }
                            fields.push_str("nextPageToken");
                            self.fields = Some(fields);
                        }
                        crate::iter::PageIter::new(self)
                    }
                    #[doc = r" Execute the given operation. The fields requested are"]
                    #[doc = r" determined by the FieldSelector attribute of the return type."]
                    #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                    #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                    #[doc = r" are not generic over the return type and deserialize the"]
                    #[doc = r" response into an auto-generated struct will all possible"]
                    #[doc = r" fields."]
                    pub fn execute<T>(self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.execute_with_fields(fields)
                    }
                    #[doc = r" Execute the given operation. This will not provide any"]
                    #[doc = r" `fields` selector indicating that the server will determine"]
                    #[doc = r" the fields returned. This typically includes the most common"]
                    #[doc = r" fields, but it will not include every possible attribute of"]
                    #[doc = r" the response resource."]
                    pub fn execute_with_default_fields(
                        self,
                    ) -> Result<crate::schemas::GoogleCloudMlV1ListVersionsResponse, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::GoogleCloudMlV1ListVersionsResponse, crate::Error>
                    {
                        self.execute_with_fields(Some("*"))
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub fn execute_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: Into<String>,
                    {
                        self.fields = fields.map(Into::into);
                        self._execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                    {
                        let req = self._request(&self._path())?;
                        Ok(crate::error_from_response(req.send()?)?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://ml.googleapis.com/".to_owned();
                        output.push_str("v1/");
                        {
                            let var_as_str = &self.parent;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/versions");
                        output
                    }
                    fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error>
                    {
                        let req = self.reqwest.request(::reqwest::Method::GET, path);
                        let req = req.query(&[("filter", &self.filter)]);
                        let req = req.query(&[("pageSize", &self.page_size)]);
                        let req = req.query(&[("pageToken", &self.page_token)]);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let req = req.bearer_auth(
                            self.auth
                                .access_token()
                                .map_err(|err| crate::Error::OAuth2(err))?,
                        );
                        Ok(req)
                    }
                }
                impl<'a> crate::iter::IterableMethod for ListRequestBuilder<'a> {
                    fn set_page_token(&mut self, value: String) {
                        self.page_token = value.into();
                    }
                    fn execute<T>(&mut self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                    {
                        self._execute()
                    }
                }
                #[doc = "Created via [VersionsActions::patch()](struct.VersionsActions.html#method.patch)"]
                #[derive(Debug, Clone)]
                pub struct PatchRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    request: crate::schemas::GoogleCloudMlV1Version,
                    name: String,
                    update_mask: Option<String>,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a> PatchRequestBuilder<'a> {
                    #[doc = "Required. Specifies the path, relative to `Version`, of the field to\nupdate. Must be present and non-empty.\n\nFor example, to change the description of a version to \"foo\", the\n`update_mask` parameter would be specified as `description`, and the\n`PATCH` request body would specify the new value, as follows:\n\n````text\n{\n  \"description\": \"foo\"\n}\n````\n\nCurrently the only supported update mask fields are `description`,\n`requestLoggingConfig`, `autoScaling.minNodes`, and `manualScaling.nodes`.\nHowever, you can only update `manualScaling.nodes` if the version uses a\n[Compute Engine (N1)\nmachine type](/ml-engine/docs/machine-types-online-prediction)."]
                    pub fn update_mask(mut self, value: impl Into<String>) -> Self {
                        self.update_mask = Some(value.into());
                        self
                    }
                    #[doc = "OAuth access token."]
                    pub fn access_token(mut self, value: impl Into<String>) -> Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(mut self, value: impl Into<String>) -> Self {
                        self.callback = Some(value.into());
                        self
                    }
                    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                    pub fn key(mut self, value: impl Into<String>) -> Self {
                        self.key = Some(value.into());
                        self
                    }
                    #[doc = "OAuth 2.0 token for the current user."]
                    pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                        self.oauth_token = Some(value.into());
                        self
                    }
                    #[doc = "Returns response with indentations and line breaks."]
                    pub fn pretty_print(mut self, value: bool) -> Self {
                        self.pretty_print = Some(value);
                        self
                    }
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                        self.xgafv = Some(value);
                        self
                    }
                    #[doc = r" Execute the given operation. The fields requested are"]
                    #[doc = r" determined by the FieldSelector attribute of the return type."]
                    #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                    #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                    #[doc = r" are not generic over the return type and deserialize the"]
                    #[doc = r" response into an auto-generated struct will all possible"]
                    #[doc = r" fields."]
                    pub fn execute<T>(self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.execute_with_fields(fields)
                    }
                    #[doc = r" Execute the given operation. This will not provide any"]
                    #[doc = r" `fields` selector indicating that the server will determine"]
                    #[doc = r" the fields returned. This typically includes the most common"]
                    #[doc = r" fields, but it will not include every possible attribute of"]
                    #[doc = r" the response resource."]
                    pub fn execute_with_default_fields(
                        self,
                    ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error>
                    {
                        self.execute_with_fields(Some("*"))
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub fn execute_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: Into<String>,
                    {
                        self.fields = fields.map(Into::into);
                        self._execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                    {
                        let req = self._request(&self._path())?;
                        let req = req.json(&self.request);
                        Ok(crate::error_from_response(req.send()?)?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://ml.googleapis.com/".to_owned();
                        output.push_str("v1/");
                        {
                            let var_as_str = &self.name;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output
                    }
                    fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error>
                    {
                        let req = self.reqwest.request(::reqwest::Method::PATCH, path);
                        let req = req.query(&[("updateMask", &self.update_mask)]);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let req = req.bearer_auth(
                            self.auth
                                .access_token()
                                .map_err(|err| crate::Error::OAuth2(err))?,
                        );
                        Ok(req)
                    }
                }
                #[doc = "Created via [VersionsActions::set_default()](struct.VersionsActions.html#method.set_default)"]
                #[derive(Debug, Clone)]
                pub struct SetDefaultRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    request: crate::schemas::GoogleCloudMlV1SetDefaultVersionRequest,
                    name: String,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a> SetDefaultRequestBuilder<'a> {
                    #[doc = "OAuth access token."]
                    pub fn access_token(mut self, value: impl Into<String>) -> Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(mut self, value: impl Into<String>) -> Self {
                        self.callback = Some(value.into());
                        self
                    }
                    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                    pub fn key(mut self, value: impl Into<String>) -> Self {
                        self.key = Some(value.into());
                        self
                    }
                    #[doc = "OAuth 2.0 token for the current user."]
                    pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                        self.oauth_token = Some(value.into());
                        self
                    }
                    #[doc = "Returns response with indentations and line breaks."]
                    pub fn pretty_print(mut self, value: bool) -> Self {
                        self.pretty_print = Some(value);
                        self
                    }
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                        self.xgafv = Some(value);
                        self
                    }
                    #[doc = r" Execute the given operation. The fields requested are"]
                    #[doc = r" determined by the FieldSelector attribute of the return type."]
                    #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                    #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                    #[doc = r" are not generic over the return type and deserialize the"]
                    #[doc = r" response into an auto-generated struct will all possible"]
                    #[doc = r" fields."]
                    pub fn execute<T>(self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.execute_with_fields(fields)
                    }
                    #[doc = r" Execute the given operation. This will not provide any"]
                    #[doc = r" `fields` selector indicating that the server will determine"]
                    #[doc = r" the fields returned. This typically includes the most common"]
                    #[doc = r" fields, but it will not include every possible attribute of"]
                    #[doc = r" the response resource."]
                    pub fn execute_with_default_fields(
                        self,
                    ) -> Result<crate::schemas::GoogleCloudMlV1Version, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::GoogleCloudMlV1Version, crate::Error>
                    {
                        self.execute_with_fields(Some("*"))
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub fn execute_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: Into<String>,
                    {
                        self.fields = fields.map(Into::into);
                        self._execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                    {
                        let req = self._request(&self._path())?;
                        let req = req.json(&self.request);
                        Ok(crate::error_from_response(req.send()?)?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://ml.googleapis.com/".to_owned();
                        output.push_str("v1/");
                        {
                            let var_as_str = &self.name;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str(":setDefault");
                        output
                    }
                    fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error>
                    {
                        let req = self.reqwest.request(::reqwest::Method::POST, path);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let req = req.bearer_auth(
                            self.auth
                                .access_token()
                                .map_err(|err| crate::Error::OAuth2(err))?,
                        );
                        Ok(req)
                    }
                }
            }
        }
        pub mod operations {
            pub mod params {}
            pub struct OperationsActions<'a> {
                pub(crate) reqwest: &'a reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> OperationsActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Starts asynchronous cancellation on a long-running operation.  The server\nmakes a best effort to cancel the operation, but success is not\nguaranteed.  If the server doesn't support this method, it returns\n`google.rpc.Code.UNIMPLEMENTED`.  Clients can use\nOperations.GetOperation or\nother methods to check whether the cancellation succeeded or whether the\noperation completed despite cancellation. On successful cancellation,\nthe operation is not deleted; instead, it becomes an operation with\nan Operation.error value with a google.rpc.Status.code of 1,\ncorresponding to `Code.CANCELLED`."]
                pub fn cancel(&self, name: impl Into<String>) -> CancelRequestBuilder {
                    CancelRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                        access_token: None,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        name: name.into(),
                    }
                }
                #[doc = "Gets the latest state of a long-running operation.  Clients can use this\nmethod to poll the operation result at intervals as recommended by the API\nservice."]
                pub fn get(&self, name: impl Into<String>) -> GetRequestBuilder {
                    GetRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                        access_token: None,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        name: name.into(),
                    }
                }
                #[doc = "Lists operations that match the specified filter in the request. If the\nserver doesn't support this method, it returns `UNIMPLEMENTED`.\n\nNOTE: the `name` binding allows API services to override the binding\nto use different resource name schemes, such as `users/*/operations`. To\noverride the binding, API services can add a binding such as\n`\"/v1/{name=users/*}/operations\"` to their service configuration.\nFor backwards compatibility, the default name includes the operations\ncollection id, however overriding users must ensure the name binding\nis the parent resource, without the operations collection id."]
                pub fn list(&self, name: impl Into<String>) -> ListRequestBuilder {
                    ListRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                        access_token: None,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        name: name.into(),
                        filter: None,
                        page_size: None,
                        page_token: None,
                    }
                }
            }
            #[doc = "Created via [OperationsActions::cancel()](struct.OperationsActions.html#method.cancel)"]
            #[derive(Debug, Clone)]
            pub struct CancelRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                name: String,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a> CancelRequestBuilder<'a> {
                #[doc = "OAuth access token."]
                pub fn access_token(mut self, value: impl Into<String>) -> Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(mut self, value: impl Into<String>) -> Self {
                    self.callback = Some(value.into());
                    self
                }
                #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                pub fn key(mut self, value: impl Into<String>) -> Self {
                    self.key = Some(value.into());
                    self
                }
                #[doc = "OAuth 2.0 token for the current user."]
                pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                    self.oauth_token = Some(value.into());
                    self
                }
                #[doc = "Returns response with indentations and line breaks."]
                pub fn pretty_print(mut self, value: bool) -> Self {
                    self.pretty_print = Some(value);
                    self
                }
                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                    self.xgafv = Some(value);
                    self
                }
                #[doc = r" Execute the given operation. The fields requested are"]
                #[doc = r" determined by the FieldSelector attribute of the return type."]
                #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                #[doc = r" are not generic over the return type and deserialize the"]
                #[doc = r" response into an auto-generated struct will all possible"]
                #[doc = r" fields."]
                pub fn execute<T>(self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_with_fields(fields)
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub fn execute_with_default_fields(
                    self,
                ) -> Result<crate::schemas::GoogleProtobufEmpty, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GoogleProtobufEmpty, crate::Error> {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute()
                }
                fn _execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path())?;
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://ml.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str(":cancel");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let req = self.reqwest.request(::reqwest::Method::POST, path);
                    let req = req.query(&[("access_token", &self.access_token)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("callback", &self.callback)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    let req = req.query(&[("uploadType", &self.upload_type)]);
                    let req = req.query(&[("$.xgafv", &self.xgafv)]);
                    let req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
                    Ok(req)
                }
            }
            #[doc = "Created via [OperationsActions::get()](struct.OperationsActions.html#method.get)"]
            #[derive(Debug, Clone)]
            pub struct GetRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                name: String,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a> GetRequestBuilder<'a> {
                #[doc = "OAuth access token."]
                pub fn access_token(mut self, value: impl Into<String>) -> Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(mut self, value: impl Into<String>) -> Self {
                    self.callback = Some(value.into());
                    self
                }
                #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                pub fn key(mut self, value: impl Into<String>) -> Self {
                    self.key = Some(value.into());
                    self
                }
                #[doc = "OAuth 2.0 token for the current user."]
                pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                    self.oauth_token = Some(value.into());
                    self
                }
                #[doc = "Returns response with indentations and line breaks."]
                pub fn pretty_print(mut self, value: bool) -> Self {
                    self.pretty_print = Some(value);
                    self
                }
                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                    self.xgafv = Some(value);
                    self
                }
                #[doc = r" Execute the given operation. The fields requested are"]
                #[doc = r" determined by the FieldSelector attribute of the return type."]
                #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                #[doc = r" are not generic over the return type and deserialize the"]
                #[doc = r" response into an auto-generated struct will all possible"]
                #[doc = r" fields."]
                pub fn execute<T>(self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_with_fields(fields)
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub fn execute_with_default_fields(
                    self,
                ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error>
                {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute()
                }
                fn _execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path())?;
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://ml.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("access_token", &self.access_token)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("callback", &self.callback)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    let req = req.query(&[("uploadType", &self.upload_type)]);
                    let req = req.query(&[("$.xgafv", &self.xgafv)]);
                    let req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
                    Ok(req)
                }
            }
            #[doc = "Created via [OperationsActions::list()](struct.OperationsActions.html#method.list)"]
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                name: String,
                filter: Option<String>,
                page_size: Option<i32>,
                page_token: Option<String>,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a> ListRequestBuilder<'a> {
                #[doc = "The standard list filter."]
                pub fn filter(mut self, value: impl Into<String>) -> Self {
                    self.filter = Some(value.into());
                    self
                }
                #[doc = "The standard list page size."]
                pub fn page_size(mut self, value: i32) -> Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "The standard list page token."]
                pub fn page_token(mut self, value: impl Into<String>) -> Self {
                    self.page_token = Some(value.into());
                    self
                }
                #[doc = "OAuth access token."]
                pub fn access_token(mut self, value: impl Into<String>) -> Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(mut self, value: impl Into<String>) -> Self {
                    self.callback = Some(value.into());
                    self
                }
                #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                pub fn key(mut self, value: impl Into<String>) -> Self {
                    self.key = Some(value.into());
                    self
                }
                #[doc = "OAuth 2.0 token for the current user."]
                pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                    self.oauth_token = Some(value.into());
                    self
                }
                #[doc = "Returns response with indentations and line breaks."]
                pub fn pretty_print(mut self, value: bool) -> Self {
                    self.pretty_print = Some(value);
                    self
                }
                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                    self.xgafv = Some(value);
                    self
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are chosen by the caller of this"]
                #[doc = r" method and must implement `Deserialize` and `FieldSelector`. The"]
                #[doc = r" populated fields in the yielded items will be determined by the"]
                #[doc = r" `FieldSelector` implementation."]
                pub fn iter_operations<T>(self) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.iter_operations_with_fields(fields)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be the default fields populated by"]
                #[doc = r" the server."]
                pub fn iter_operations_with_default_fields(
                    self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::GoogleLongrunningOperation>
                {
                    self.iter_operations_with_fields(None::<String>)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be all fields available. This should"]
                #[doc = r" primarily be used during developement and debugging as fetching"]
                #[doc = r" all fields can be expensive both in bandwidth and server"]
                #[doc = r" resources."]
                pub fn iter_operations_with_all_fields(
                    self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::GoogleLongrunningOperation>
                {
                    self.iter_operations_with_fields(Some("*"))
                }
                pub fn iter_operations_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: AsRef<str>,
                {
                    self.fields = Some({
                        let mut selector = concat!("nextPageToken,", "operations").to_owned();
                        let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                        if !items_fields.is_empty() {
                            selector.push_str("(");
                            selector.push_str(items_fields);
                            selector.push_str(")");
                        }
                        selector
                    });
                    crate::iter::PageItemIter::new(self, "operations")
                }
                pub fn iter<T>(self) -> crate::iter::PageIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.iter_with_fields(fields)
                }
                pub fn iter_with_default_fields(
                    self,
                ) -> crate::iter::PageIter<
                    Self,
                    crate::schemas::GoogleLongrunningListOperationsResponse,
                > {
                    self.iter_with_fields(None::<&str>)
                }
                pub fn iter_with_all_fields(
                    self,
                ) -> crate::iter::PageIter<
                    Self,
                    crate::schemas::GoogleLongrunningListOperationsResponse,
                > {
                    self.iter_with_fields(Some("*"))
                }
                pub fn iter_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> crate::iter::PageIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: AsRef<str>,
                {
                    let mut fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("").to_owned();
                    if !fields.is_empty() {
                        match fields.chars().rev().nth(0) {
                            Some(',') | None => {}
                            _ => fields.push_str(","),
                        }
                        fields.push_str("nextPageToken");
                        self.fields = Some(fields);
                    }
                    crate::iter::PageIter::new(self)
                }
                #[doc = r" Execute the given operation. The fields requested are"]
                #[doc = r" determined by the FieldSelector attribute of the return type."]
                #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                #[doc = r" are not generic over the return type and deserialize the"]
                #[doc = r" response into an auto-generated struct will all possible"]
                #[doc = r" fields."]
                pub fn execute<T>(self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_with_fields(fields)
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub fn execute_with_default_fields(
                    self,
                ) -> Result<crate::schemas::GoogleLongrunningListOperationsResponse, crate::Error>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GoogleLongrunningListOperationsResponse, crate::Error>
                {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute()
                }
                fn _execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path())?;
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://ml.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/operations");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("filter", &self.filter)]);
                    let req = req.query(&[("pageSize", &self.page_size)]);
                    let req = req.query(&[("pageToken", &self.page_token)]);
                    let req = req.query(&[("access_token", &self.access_token)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("callback", &self.callback)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    let req = req.query(&[("uploadType", &self.upload_type)]);
                    let req = req.query(&[("$.xgafv", &self.xgafv)]);
                    let req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
                    Ok(req)
                }
            }
            impl<'a> crate::iter::IterableMethod for ListRequestBuilder<'a> {
                fn set_page_token(&mut self, value: String) {
                    self.page_token = value.into();
                }
                fn execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    self._execute()
                }
            }
        }
    }
}
#[derive(Debug)]
pub enum Error {
    OAuth2(Box<dyn ::std::error::Error + Send + Sync>),
    JSON(::serde_json::Error),
    Reqwest {
        reqwest_err: ::reqwest::Error,
        body: Option<String>,
    },
    Other(Box<dyn ::std::error::Error + Send + Sync>),
}

impl Error {
    pub fn json_error(&self) -> Option<&::serde_json::Error> {
        match self {
            Error::OAuth2(_) => None,
            Error::JSON(err) => Some(err),
            Error::Reqwest { .. } => None,
            Error::Other(_) => None,
        }
    }
}

impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            Error::OAuth2(err) => write!(f, "OAuth2 Error: {}", err),
            Error::JSON(err) => write!(f, "JSON Error: {}", err),
            Error::Reqwest { reqwest_err, body } => {
                write!(f, "Reqwest Error: {}", reqwest_err)?;
                if let Some(body) = body {
                    write!(f, ": {}", body)?;
                }
                Ok(())
            }
            Error::Other(err) => write!(f, "Uknown Error: {}", err),
        }
    }
}

impl ::std::error::Error for Error {}

impl From<::serde_json::Error> for Error {
    fn from(err: ::serde_json::Error) -> Error {
        Error::JSON(err)
    }
}

impl From<::reqwest::Error> for Error {
    fn from(reqwest_err: ::reqwest::Error) -> Error {
        Error::Reqwest {
            reqwest_err,
            body: None,
        }
    }
}

/// Check the response to see if the status code represents an error. If so
/// convert it into the Reqwest variant of Error.
fn error_from_response(
    response: ::reqwest::blocking::Response,
) -> Result<::reqwest::blocking::Response, Error> {
    match response.error_for_status_ref() {
        Err(reqwest_err) => {
            let body = response.text().ok();
            Err(Error::Reqwest { reqwest_err, body })
        }
        Ok(_) => Ok(response),
    }
}
#[allow(dead_code)]
const SIMPLE: &::percent_encoding::AsciiSet = &::percent_encoding::NON_ALPHANUMERIC
    .remove(b'-')
    .remove(b'.')
    .remove(b'_')
    .remove(b'~');

#[allow(dead_code)]
const RESERVED: &::percent_encoding::AsciiSet = &SIMPLE
    .remove(b'%')
    .remove(b':')
    .remove(b'/')
    .remove(b'?')
    .remove(b'#')
    .remove(b'[')
    .remove(b']')
    .remove(b'@')
    .remove(b'!')
    .remove(b'$')
    .remove(b'&')
    .remove(b'\'')
    .remove(b'(')
    .remove(b')')
    .remove(b'*')
    .remove(b'+')
    .remove(b',')
    .remove(b';')
    .remove(b'=');
#[allow(dead_code)]
mod multipart {
    pub(crate) struct RelatedMultiPart {
        parts: Vec<Part>,
        boundary: String,
    }

    impl RelatedMultiPart {
        pub(crate) fn new() -> Self {
            RelatedMultiPart {
                parts: Vec::new(),
                boundary: ::textnonce::TextNonce::sized(68).unwrap().0,
            }
        }

        pub(crate) fn new_part(&mut self, part: Part) {
            self.parts.push(part);
        }

        pub(crate) fn boundary(&self) -> &str {
            &self.boundary
        }

        pub(crate) fn into_reader(self) -> RelatedMultiPartReader {
            let boundary_marker = boundary_marker(&self.boundary);
            RelatedMultiPartReader {
                state: RelatedMultiPartReaderState::WriteBoundary {
                    start: 0,
                    boundary: format!("{}\r\n", &boundary_marker),
                },
                boundary: boundary_marker,
                next_body: None,
                parts: self.parts.into_iter(),
            }
        }
    }

    pub(crate) struct Part {
        content_type: ::mime::Mime,
        body: Box<dyn ::std::io::Read + Send>,
    }

    impl Part {
        pub(crate) fn new(
            content_type: ::mime::Mime,
            body: Box<dyn ::std::io::Read + Send>,
        ) -> Part {
            Part { content_type, body }
        }
    }

    pub(crate) struct RelatedMultiPartReader {
        state: RelatedMultiPartReaderState,
        boundary: String,
        next_body: Option<Box<dyn ::std::io::Read + Send>>,
        parts: std::vec::IntoIter<Part>,
    }

    enum RelatedMultiPartReaderState {
        WriteBoundary {
            start: usize,
            boundary: String,
        },
        WriteContentType {
            start: usize,
            content_type: Vec<u8>,
        },
        WriteBody {
            body: Box<dyn ::std::io::Read + Send>,
        },
    }

    impl ::std::io::Read for RelatedMultiPartReader {
        fn read(&mut self, buf: &mut [u8]) -> ::std::io::Result<usize> {
            use RelatedMultiPartReaderState::*;
            let mut bytes_written: usize = 0;
            loop {
                let rem_buf = &mut buf[bytes_written..];
                match &mut self.state {
                    WriteBoundary { start, boundary } => {
                        let bytes_to_copy = std::cmp::min(boundary.len() - *start, rem_buf.len());
                        rem_buf[..bytes_to_copy]
                            .copy_from_slice(&boundary.as_bytes()[*start..*start + bytes_to_copy]);
                        *start += bytes_to_copy;
                        bytes_written += bytes_to_copy;
                        if *start == boundary.len() {
                            let next_part = match self.parts.next() {
                                None => break,
                                Some(part) => part,
                            };
                            self.next_body = Some(next_part.body);
                            self.state = WriteContentType {
                                start: 0,
                                content_type: format!(
                                    "Content-Type: {}\r\n\r\n",
                                    next_part.content_type
                                )
                                .into_bytes(),
                            };
                        } else {
                            break;
                        }
                    }
                    WriteContentType {
                        start,
                        content_type,
                    } => {
                        let bytes_to_copy =
                            std::cmp::min(content_type.len() - *start, rem_buf.len());
                        rem_buf[..bytes_to_copy]
                            .copy_from_slice(&content_type[*start..*start + bytes_to_copy]);
                        *start += bytes_to_copy;
                        bytes_written += bytes_to_copy;
                        if *start == content_type.len() {
                            self.state = WriteBody {
                                body: self.next_body.take().unwrap(),
                            };
                        } else {
                            break;
                        }
                    }
                    WriteBody { body } => {
                        let written = body.read(rem_buf)?;
                        bytes_written += written;
                        if written == 0 {
                            self.state = WriteBoundary {
                                start: 0,
                                boundary: format!("\r\n{}\r\n", &self.boundary),
                            };
                        } else {
                            break;
                        }
                    }
                }
            }
            Ok(bytes_written)
        }
    }

    fn boundary_marker(boundary: &str) -> String {
        let mut marker = String::with_capacity(boundary.len() + 2);
        marker.push_str("--");
        marker.push_str(boundary);
        marker
    }
}
// A serde helper module that can be used with the `with` attribute
// to deserialize any string to a FromStr type and serialize any
// Display type to a String. Google API's encode i64, u64 values as
// strings.
#[allow(dead_code)]
mod parsed_string {
    pub fn serialize<T, S>(
        value: &Option<T>,
        serializer: S,
    ) -> ::std::result::Result<S::Ok, S::Error>
    where
        T: ::std::fmt::Display,
        S: ::serde::Serializer,
    {
        use ::serde::Serialize;
        value.as_ref().map(|x| x.to_string()).serialize(serializer)
    }

    pub fn deserialize<'de, T, D>(deserializer: D) -> ::std::result::Result<Option<T>, D::Error>
    where
        T: ::std::str::FromStr,
        T::Err: ::std::fmt::Display,
        D: ::serde::de::Deserializer<'de>,
    {
        use ::serde::Deserialize;
        match Option::<String>::deserialize(deserializer)? {
            Some(x) => Ok(Some(x.parse().map_err(::serde::de::Error::custom)?)),
            None => Ok(None),
        }
    }
}
pub mod iter {
    pub trait IterableMethod {
        fn set_page_token(&mut self, value: String);
        fn execute<T>(&mut self) -> Result<T, crate::Error>
        where
            T: ::serde::de::DeserializeOwned;
    }

    pub struct PageIter<M, T> {
        pub method: M,
        pub finished: bool,
        pub _phantom: ::std::marker::PhantomData<T>,
    }

    impl<M, T> PageIter<M, T>
    where
        M: IterableMethod,
        T: ::serde::de::DeserializeOwned,
    {
        pub(crate) fn new(method: M) -> Self {
            PageIter {
                method,
                finished: false,
                _phantom: ::std::marker::PhantomData,
            }
        }
    }

    impl<M, T> Iterator for PageIter<M, T>
    where
        M: IterableMethod,
        T: ::serde::de::DeserializeOwned,
    {
        type Item = Result<T, crate::Error>;

        fn next(&mut self) -> Option<Result<T, crate::Error>> {
            if self.finished {
                return None;
            }
            let paginated_result: ::serde_json::Map<String, ::serde_json::Value> =
                match self.method.execute() {
                    Ok(r) => r,
                    Err(err) => return Some(Err(err)),
                };
            if let Some(next_page_token) = paginated_result
                .get("nextPageToken")
                .and_then(|t| t.as_str())
            {
                self.method.set_page_token(next_page_token.to_owned());
            } else {
                self.finished = true;
            }

            Some(
                match ::serde_json::from_value(::serde_json::Value::Object(paginated_result)) {
                    Ok(resp) => Ok(resp),
                    Err(err) => Err(err.into()),
                },
            )
        }
    }

    pub struct PageItemIter<M, T> {
        items_field: &'static str,
        page_iter: PageIter<M, ::serde_json::Map<String, ::serde_json::Value>>,
        items: ::std::vec::IntoIter<T>,
    }

    impl<M, T> PageItemIter<M, T>
    where
        M: IterableMethod,
        T: ::serde::de::DeserializeOwned,
    {
        pub(crate) fn new(method: M, items_field: &'static str) -> Self {
            PageItemIter {
                items_field,
                page_iter: PageIter::new(method),
                items: Vec::new().into_iter(),
            }
        }
    }

    impl<M, T> Iterator for PageItemIter<M, T>
    where
        M: IterableMethod,
        T: ::serde::de::DeserializeOwned,
    {
        type Item = Result<T, crate::Error>;

        fn next(&mut self) -> Option<Result<T, crate::Error>> {
            loop {
                if let Some(v) = self.items.next() {
                    return Some(Ok(v));
                }

                let next_page = self.page_iter.next();
                match next_page {
                    None => return None,
                    Some(Err(err)) => return Some(Err(err)),
                    Some(Ok(next_page)) => {
                        let mut next_page: ::serde_json::Map<String, ::serde_json::Value> =
                            next_page;
                        let items_array = match next_page.remove(self.items_field) {
                            Some(items) => items,
                            None => {
                                return Some(Err(crate::Error::Other(
                                    format!("no {} field found in iter response", self.items_field)
                                        .into(),
                                )))
                            }
                        };
                        let items_vec: Result<Vec<T>, _> = ::serde_json::from_value(items_array);
                        match items_vec {
                            Ok(items) => self.items = items.into_iter(),
                            Err(err) => return Some(Err(err.into())),
                        }
                    }
                }
            }
        }
    }
}
