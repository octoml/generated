#![doc = "# Resources and Methods\n    * [users](resources/users/struct.UsersActions.html)\n      * [data_sources](resources/users/data_sources/struct.DataSourcesActions.html)\n        * [*create*](resources/users/data_sources/struct.CreateRequestBuilder.html), [*delete*](resources/users/data_sources/struct.DeleteRequestBuilder.html), [*get*](resources/users/data_sources/struct.GetRequestBuilder.html), [*list*](resources/users/data_sources/struct.ListRequestBuilder.html), [*update*](resources/users/data_sources/struct.UpdateRequestBuilder.html)\n        * [data_point_changes](resources/users/data_sources/data_point_changes/struct.DataPointChangesActions.html)\n          * [*list*](resources/users/data_sources/data_point_changes/struct.ListRequestBuilder.html)\n        * [datasets](resources/users/data_sources/datasets/struct.DatasetsActions.html)\n          * [*delete*](resources/users/data_sources/datasets/struct.DeleteRequestBuilder.html), [*get*](resources/users/data_sources/datasets/struct.GetRequestBuilder.html), [*patch*](resources/users/data_sources/datasets/struct.PatchRequestBuilder.html)\n      * [dataset](resources/users/dataset/struct.DatasetActions.html)\n        * [*aggregate*](resources/users/dataset/struct.AggregateRequestBuilder.html)\n      * [sessions](resources/users/sessions/struct.SessionsActions.html)\n        * [*delete*](resources/users/sessions/struct.DeleteRequestBuilder.html), [*list*](resources/users/sessions/struct.ListRequestBuilder.html), [*update*](resources/users/sessions/struct.UpdateRequestBuilder.html)\n"]
pub mod scopes {
    #[doc = "Use Google Fit to see and store your physical activity data\n\n`https://www.googleapis.com/auth/fitness.activity.read`"]
    pub const FITNESS_ACTIVITY_READ: &str = "https://www.googleapis.com/auth/fitness.activity.read";
    #[doc = "See and add to your Google Fit physical activity data\n\n`https://www.googleapis.com/auth/fitness.activity.write`"]
    pub const FITNESS_ACTIVITY_WRITE: &str =
        "https://www.googleapis.com/auth/fitness.activity.write";
    #[doc = "See info about your blood glucose in Google Fit. I consent to Google sharing my blood glucose information with this app.\n\n`https://www.googleapis.com/auth/fitness.blood_glucose.read`"]
    pub const FITNESS_BLOOD_GLUCOSE_READ: &str =
        "https://www.googleapis.com/auth/fitness.blood_glucose.read";
    #[doc = "See and add info about your blood glucose to Google Fit. I consent to Google sharing my blood glucose information with this app.\n\n`https://www.googleapis.com/auth/fitness.blood_glucose.write`"]
    pub const FITNESS_BLOOD_GLUCOSE_WRITE: &str =
        "https://www.googleapis.com/auth/fitness.blood_glucose.write";
    #[doc = "See info about your blood pressure in Google Fit. I consent to Google sharing my blood pressure information with this app.\n\n`https://www.googleapis.com/auth/fitness.blood_pressure.read`"]
    pub const FITNESS_BLOOD_PRESSURE_READ: &str =
        "https://www.googleapis.com/auth/fitness.blood_pressure.read";
    #[doc = "See and add info about your blood pressure in Google Fit. I consent to Google sharing my blood pressure information with this app.\n\n`https://www.googleapis.com/auth/fitness.blood_pressure.write`"]
    pub const FITNESS_BLOOD_PRESSURE_WRITE: &str =
        "https://www.googleapis.com/auth/fitness.blood_pressure.write";
    #[doc = "See info about your body measurements and heart rate in Google Fit\n\n`https://www.googleapis.com/auth/fitness.body.read`"]
    pub const FITNESS_BODY_READ: &str = "https://www.googleapis.com/auth/fitness.body.read";
    #[doc = "See and add info about your body measurements and heart rate to Google Fit\n\n`https://www.googleapis.com/auth/fitness.body.write`"]
    pub const FITNESS_BODY_WRITE: &str = "https://www.googleapis.com/auth/fitness.body.write";
    #[doc = "See info about your body temperature in Google Fit. I consent to Google sharing my body temperature information with this app.\n\n`https://www.googleapis.com/auth/fitness.body_temperature.read`"]
    pub const FITNESS_BODY_TEMPERATURE_READ: &str =
        "https://www.googleapis.com/auth/fitness.body_temperature.read";
    #[doc = "See and add to info about your body temperature in Google Fit. I consent to Google sharing my body temperature information with this app.\n\n`https://www.googleapis.com/auth/fitness.body_temperature.write`"]
    pub const FITNESS_BODY_TEMPERATURE_WRITE: &str =
        "https://www.googleapis.com/auth/fitness.body_temperature.write";
    #[doc = "See your Google Fit speed and distance data\n\n`https://www.googleapis.com/auth/fitness.location.read`"]
    pub const FITNESS_LOCATION_READ: &str = "https://www.googleapis.com/auth/fitness.location.read";
    #[doc = "See and add to your Google Fit location data\n\n`https://www.googleapis.com/auth/fitness.location.write`"]
    pub const FITNESS_LOCATION_WRITE: &str =
        "https://www.googleapis.com/auth/fitness.location.write";
    #[doc = "See info about your nutrition in Google Fit\n\n`https://www.googleapis.com/auth/fitness.nutrition.read`"]
    pub const FITNESS_NUTRITION_READ: &str =
        "https://www.googleapis.com/auth/fitness.nutrition.read";
    #[doc = "See and add to info about your nutrition in Google Fit\n\n`https://www.googleapis.com/auth/fitness.nutrition.write`"]
    pub const FITNESS_NUTRITION_WRITE: &str =
        "https://www.googleapis.com/auth/fitness.nutrition.write";
    #[doc = "See info about your oxygen saturation in Google Fit. I consent to Google sharing my oxygen saturation information with this app.\n\n`https://www.googleapis.com/auth/fitness.oxygen_saturation.read`"]
    pub const FITNESS_OXYGEN_SATURATION_READ: &str =
        "https://www.googleapis.com/auth/fitness.oxygen_saturation.read";
    #[doc = "See and add info about your oxygen saturation in Google Fit. I consent to Google sharing my oxygen saturation information with this app.\n\n`https://www.googleapis.com/auth/fitness.oxygen_saturation.write`"]
    pub const FITNESS_OXYGEN_SATURATION_WRITE: &str =
        "https://www.googleapis.com/auth/fitness.oxygen_saturation.write";
    #[doc = "See info about your reproductive health in Google Fit. I consent to Google sharing my reporductive health information with this app.\n\n`https://www.googleapis.com/auth/fitness.reproductive_health.read`"]
    pub const FITNESS_REPRODUCTIVE_HEALTH_READ: &str =
        "https://www.googleapis.com/auth/fitness.reproductive_health.read";
    #[doc = "See and add info about your reproductive health in Google Fit. I consent to Google sharing my reporductive health information with this app.\n\n`https://www.googleapis.com/auth/fitness.reproductive_health.write`"]
    pub const FITNESS_REPRODUCTIVE_HEALTH_WRITE: &str =
        "https://www.googleapis.com/auth/fitness.reproductive_health.write";
}
pub mod schemas {
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct AggregateBucket {
        #[doc = "Available for Bucket.Type.ACTIVITY_TYPE, Bucket.Type.ACTIVITY_SEGMENT"]
        #[serde(
            rename = "activity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub activity: ::std::option::Option<i32>,
        #[doc = "There will be one dataset per AggregateBy in the request."]
        #[serde(
            rename = "dataset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dataset: ::std::option::Option<Vec<crate::schemas::Dataset>>,
        #[doc = "The end time for the aggregated data, in milliseconds since epoch,\ninclusive."]
        #[serde(
            rename = "endTimeMillis",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub end_time_millis: ::std::option::Option<i64>,
        #[doc = "The type of a bucket signifies how the data aggregation is performed in the\nbucket."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::AggregateBucketType>,
        #[doc = "Available for Bucket.Type.SESSION"]
        #[serde(
            rename = "session",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub session: ::std::option::Option<crate::schemas::Session>,
        #[doc = "The start time for the aggregated data, in milliseconds since epoch,\ninclusive."]
        #[serde(
            rename = "startTimeMillis",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub start_time_millis: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for AggregateBucket {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AggregateBucket {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum AggregateBucketType {
        #[doc = "Denotes that bucketing by individual activity segment is requested. This\nwill aggregate data by the time boundaries specified by each activity\nsegment occurring within the dataset time frame of interest."]
        ActivitySegment,
        #[doc = "Denotes that bucketing by activity type is requested. When this is\nspecified, there will be one bucket for each unique activity type that\na user participated in, during the dataset time frame of interest."]
        ActivityType,
        #[doc = "Denotes that bucketing by session is requested. When this is specified,\nonly data that occurs within sessions that begin and end within the\ndataset time frame, is included in the results."]
        Session,
        #[doc = "Denotes that bucketing by time is requested. When this is specified, the\ntimeBucketDurationMillis field is used to determine how many buckets will\nbe returned."]
        Time,
        Unknown,
    }
    impl AggregateBucketType {
        pub fn as_str(self) -> &'static str {
            match self {
                AggregateBucketType::ActivitySegment => "activitySegment",
                AggregateBucketType::ActivityType => "activityType",
                AggregateBucketType::Session => "session",
                AggregateBucketType::Time => "time",
                AggregateBucketType::Unknown => "unknown",
            }
        }
    }
    impl ::std::convert::AsRef<str> for AggregateBucketType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for AggregateBucketType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<AggregateBucketType, ()> {
            Ok(match s {
                "activitySegment" => AggregateBucketType::ActivitySegment,
                "activityType" => AggregateBucketType::ActivityType,
                "session" => AggregateBucketType::Session,
                "time" => AggregateBucketType::Time,
                "unknown" => AggregateBucketType::Unknown,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for AggregateBucketType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AggregateBucketType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AggregateBucketType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "activitySegment" => AggregateBucketType::ActivitySegment,
                "activityType" => AggregateBucketType::ActivityType,
                "session" => AggregateBucketType::Session,
                "time" => AggregateBucketType::Time,
                "unknown" => AggregateBucketType::Unknown,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for AggregateBucketType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AggregateBucketType {
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
    pub struct AggregateBy {
        #[doc = "A data source ID to aggregate. Only data from the specified data source ID\nwill be included in the aggregation. If specified, this data source must\nexist; the OAuth scopes in the supplied credentials must grant read access\nto this data type. The dataset in the response will have the same data\nsource ID. Note: Data can be aggregated by either the dataTypeName or the\ndataSourceId, not both."]
        #[serde(
            rename = "dataSourceId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub data_source_id: ::std::option::Option<String>,
        #[doc = "The data type to aggregate. All data sources providing this data type will\ncontribute data to the aggregation. The response will contain a single\ndataset for this data type name. The dataset will have a data source ID of\nderived:<output data type name>:com.google.android.gms:aggregated.\nIf the user has no data for this data type, an empty data set will be\nreturned. Note: Data can be aggregated by either the dataTypeName or the\ndataSourceId, not both."]
        #[serde(
            rename = "dataTypeName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub data_type_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for AggregateBy {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AggregateBy {
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
    pub struct AggregateRequest {
        #[doc = "The specification of data to be aggregated. At least one aggregateBy spec\nmust be provided. All data that is specified will be aggregated using the\nsame bucketing criteria. There will be one dataset in the response for\nevery aggregateBy spec."]
        #[serde(
            rename = "aggregateBy",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub aggregate_by: ::std::option::Option<Vec<crate::schemas::AggregateBy>>,
        #[doc = "Specifies that data be aggregated each activity segment recored for a user.\nSimilar to bucketByActivitySegment, but bucketing is done for each activity\nsegment rather than all segments of the same type. Mutually exclusive of\nother bucketing specifications."]
        #[serde(
            rename = "bucketByActivitySegment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bucket_by_activity_segment: ::std::option::Option<crate::schemas::BucketByActivity>,
        #[doc = "Specifies that data be aggregated by the type of activity being performed\nwhen the data was recorded. All data that was recorded during a certain\nactivity type (for the given time range) will be aggregated into the same\nbucket. Data that was recorded while the user was not active will not be\nincluded in the response. Mutually exclusive of other bucketing\nspecifications."]
        #[serde(
            rename = "bucketByActivityType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bucket_by_activity_type: ::std::option::Option<crate::schemas::BucketByActivity>,
        #[doc = "Specifies that data be aggregated by user sessions. Data that does not fall\nwithin the time range of a session will not be included in the response.\nMutually exclusive of other bucketing specifications."]
        #[serde(
            rename = "bucketBySession",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bucket_by_session: ::std::option::Option<crate::schemas::BucketBySession>,
        #[doc = "Specifies that data be aggregated by a single time interval. Mutually\nexclusive of other bucketing specifications."]
        #[serde(
            rename = "bucketByTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bucket_by_time: ::std::option::Option<crate::schemas::BucketByTime>,
        #[doc = "The end of a window of time. Data that intersects with this time\nwindow will be aggregated. The time is in milliseconds since epoch,\ninclusive."]
        #[serde(
            rename = "endTimeMillis",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub end_time_millis: ::std::option::Option<i64>,
        #[doc = "DO NOT POPULATE THIS FIELD. It is ignored."]
        #[serde(
            rename = "filteredDataQualityStandard",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub filtered_data_quality_standard: ::std::option::Option<
            Vec<crate::schemas::AggregateRequestFilteredDataQualityStandardItems>,
        >,
        #[doc = "The start of a window of time. Data that intersects with this time\nwindow will be aggregated. The time is in milliseconds since epoch,\ninclusive."]
        #[serde(
            rename = "startTimeMillis",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub start_time_millis: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for AggregateRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AggregateRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum AggregateRequestFilteredDataQualityStandardItems {
        DataQualityBloodGlucoseIso151972003,
        DataQualityBloodGlucoseIso151972013,
        DataQualityBloodPressureAami,
        DataQualityBloodPressureBhsAA,
        DataQualityBloodPressureBhsAB,
        DataQualityBloodPressureBhsBA,
        DataQualityBloodPressureBhsBB,
        DataQualityBloodPressureEsh2002,
        DataQualityBloodPressureEsh2010,
        DataQualityUnknown,
    }
    impl AggregateRequestFilteredDataQualityStandardItems {
        pub fn as_str(self) -> &'static str {
            match self { AggregateRequestFilteredDataQualityStandardItems :: DataQualityBloodGlucoseIso151972003 => "dataQualityBloodGlucoseIso151972003" , AggregateRequestFilteredDataQualityStandardItems :: DataQualityBloodGlucoseIso151972013 => "dataQualityBloodGlucoseIso151972013" , AggregateRequestFilteredDataQualityStandardItems :: DataQualityBloodPressureAami => "dataQualityBloodPressureAami" , AggregateRequestFilteredDataQualityStandardItems :: DataQualityBloodPressureBhsAA => "dataQualityBloodPressureBhsAA" , AggregateRequestFilteredDataQualityStandardItems :: DataQualityBloodPressureBhsAB => "dataQualityBloodPressureBhsAB" , AggregateRequestFilteredDataQualityStandardItems :: DataQualityBloodPressureBhsBA => "dataQualityBloodPressureBhsBA" , AggregateRequestFilteredDataQualityStandardItems :: DataQualityBloodPressureBhsBB => "dataQualityBloodPressureBhsBB" , AggregateRequestFilteredDataQualityStandardItems :: DataQualityBloodPressureEsh2002 => "dataQualityBloodPressureEsh2002" , AggregateRequestFilteredDataQualityStandardItems :: DataQualityBloodPressureEsh2010 => "dataQualityBloodPressureEsh2010" , AggregateRequestFilteredDataQualityStandardItems :: DataQualityUnknown => "dataQualityUnknown" , }
        }
    }
    impl ::std::convert::AsRef<str> for AggregateRequestFilteredDataQualityStandardItems {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for AggregateRequestFilteredDataQualityStandardItems {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<AggregateRequestFilteredDataQualityStandardItems, ()> {
            Ok ( match s { "dataQualityBloodGlucoseIso151972003" => AggregateRequestFilteredDataQualityStandardItems :: DataQualityBloodGlucoseIso151972003 , "dataQualityBloodGlucoseIso151972013" => AggregateRequestFilteredDataQualityStandardItems :: DataQualityBloodGlucoseIso151972013 , "dataQualityBloodPressureAami" => AggregateRequestFilteredDataQualityStandardItems :: DataQualityBloodPressureAami , "dataQualityBloodPressureBhsAA" => AggregateRequestFilteredDataQualityStandardItems :: DataQualityBloodPressureBhsAA , "dataQualityBloodPressureBhsAB" => AggregateRequestFilteredDataQualityStandardItems :: DataQualityBloodPressureBhsAB , "dataQualityBloodPressureBhsBA" => AggregateRequestFilteredDataQualityStandardItems :: DataQualityBloodPressureBhsBA , "dataQualityBloodPressureBhsBB" => AggregateRequestFilteredDataQualityStandardItems :: DataQualityBloodPressureBhsBB , "dataQualityBloodPressureEsh2002" => AggregateRequestFilteredDataQualityStandardItems :: DataQualityBloodPressureEsh2002 , "dataQualityBloodPressureEsh2010" => AggregateRequestFilteredDataQualityStandardItems :: DataQualityBloodPressureEsh2010 , "dataQualityUnknown" => AggregateRequestFilteredDataQualityStandardItems :: DataQualityUnknown , _ => return Err ( ( ) ) , } )
        }
    }
    impl ::std::fmt::Display for AggregateRequestFilteredDataQualityStandardItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AggregateRequestFilteredDataQualityStandardItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AggregateRequestFilteredDataQualityStandardItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok ( match value { "dataQualityBloodGlucoseIso151972003" => AggregateRequestFilteredDataQualityStandardItems :: DataQualityBloodGlucoseIso151972003 , "dataQualityBloodGlucoseIso151972013" => AggregateRequestFilteredDataQualityStandardItems :: DataQualityBloodGlucoseIso151972013 , "dataQualityBloodPressureAami" => AggregateRequestFilteredDataQualityStandardItems :: DataQualityBloodPressureAami , "dataQualityBloodPressureBhsAA" => AggregateRequestFilteredDataQualityStandardItems :: DataQualityBloodPressureBhsAA , "dataQualityBloodPressureBhsAB" => AggregateRequestFilteredDataQualityStandardItems :: DataQualityBloodPressureBhsAB , "dataQualityBloodPressureBhsBA" => AggregateRequestFilteredDataQualityStandardItems :: DataQualityBloodPressureBhsBA , "dataQualityBloodPressureBhsBB" => AggregateRequestFilteredDataQualityStandardItems :: DataQualityBloodPressureBhsBB , "dataQualityBloodPressureEsh2002" => AggregateRequestFilteredDataQualityStandardItems :: DataQualityBloodPressureEsh2002 , "dataQualityBloodPressureEsh2010" => AggregateRequestFilteredDataQualityStandardItems :: DataQualityBloodPressureEsh2010 , "dataQualityUnknown" => AggregateRequestFilteredDataQualityStandardItems :: DataQualityUnknown , _ => return Err ( :: serde :: de :: Error :: custom ( format ! ( "invalid enum for #name: {}" , value ) ) ) , } )
        }
    }
    impl ::google_field_selector::FieldSelector for AggregateRequestFilteredDataQualityStandardItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AggregateRequestFilteredDataQualityStandardItems {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct AggregateResponse {
        #[doc = "A list of buckets containing the aggregated data."]
        #[serde(
            rename = "bucket",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bucket: ::std::option::Option<Vec<crate::schemas::AggregateBucket>>,
    }
    impl ::google_field_selector::FieldSelector for AggregateResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AggregateResponse {
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
    pub struct Application {
        #[doc = "An optional URI that can be used to link back to the application."]
        #[serde(
            rename = "detailsUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub details_url: ::std::option::Option<String>,
        #[doc = "The name of this application. This is required for REST clients, but we\ndo not enforce uniqueness of this name. It is provided as a matter of\nconvenience for other developers who would like to identify which REST\ncreated an Application or Data Source."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Package name for this application. This is used as a unique\nidentifier when created by Android applications, but cannot be specified\nby REST clients. REST clients will have their developer project number\nreflected into the Data Source data stream IDs, instead of the packageName."]
        #[serde(
            rename = "packageName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub package_name: ::std::option::Option<String>,
        #[doc = "Version of the application. You should update this field whenever the\napplication changes in a way that affects the computation of the data."]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Application {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Application {
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
    pub struct BucketByActivity {
        #[doc = "The default activity stream will be used if a specific activityDataSourceId\nis not specified."]
        #[serde(
            rename = "activityDataSourceId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub activity_data_source_id: ::std::option::Option<String>,
        #[doc = "Specifies that only activity segments of duration longer than\nminDurationMillis are considered and used as a container for aggregated\ndata."]
        #[serde(
            rename = "minDurationMillis",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub min_duration_millis: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for BucketByActivity {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BucketByActivity {
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
    pub struct BucketBySession {
        #[doc = "Specifies that only sessions of duration longer than minDurationMillis are\nconsidered and used as a container for aggregated data."]
        #[serde(
            rename = "minDurationMillis",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub min_duration_millis: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for BucketBySession {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BucketBySession {
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
    pub struct BucketByTime {
        #[doc = "Specifies that result buckets aggregate data by exactly durationMillis time\nframes. Time frames that contain no data will be included in the response\nwith an empty dataset."]
        #[serde(
            rename = "durationMillis",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub duration_millis: ::std::option::Option<i64>,
        #[serde(
            rename = "period",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub period: ::std::option::Option<crate::schemas::BucketByTimePeriod>,
    }
    impl ::google_field_selector::FieldSelector for BucketByTime {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BucketByTime {
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
    pub struct BucketByTimePeriod {
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::BucketByTimePeriodType>,
        #[doc = "org.joda.timezone.DateTimeZone"]
        #[serde(
            rename = "timeZoneId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub time_zone_id: ::std::option::Option<String>,
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for BucketByTimePeriod {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BucketByTimePeriod {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum BucketByTimePeriodType {
        Day,
        Month,
        Week,
    }
    impl BucketByTimePeriodType {
        pub fn as_str(self) -> &'static str {
            match self {
                BucketByTimePeriodType::Day => "day",
                BucketByTimePeriodType::Month => "month",
                BucketByTimePeriodType::Week => "week",
            }
        }
    }
    impl ::std::convert::AsRef<str> for BucketByTimePeriodType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for BucketByTimePeriodType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<BucketByTimePeriodType, ()> {
            Ok(match s {
                "day" => BucketByTimePeriodType::Day,
                "month" => BucketByTimePeriodType::Month,
                "week" => BucketByTimePeriodType::Week,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for BucketByTimePeriodType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for BucketByTimePeriodType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for BucketByTimePeriodType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "day" => BucketByTimePeriodType::Day,
                "month" => BucketByTimePeriodType::Month,
                "week" => BucketByTimePeriodType::Week,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for BucketByTimePeriodType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BucketByTimePeriodType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct DataPoint {
        #[doc = "DO NOT USE THIS FIELD. It is ignored, and not stored."]
        #[serde(
            rename = "computationTimeMillis",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub computation_time_millis: ::std::option::Option<i64>,
        #[doc = "The data type defining the format of the values in this data point."]
        #[serde(
            rename = "dataTypeName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub data_type_name: ::std::option::Option<String>,
        #[doc = "The end time of the interval represented by this data point, in\nnanoseconds since epoch."]
        #[serde(
            rename = "endTimeNanos",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub end_time_nanos: ::std::option::Option<i64>,
        #[doc = "Indicates the last time this data point was modified. Useful only in\ncontexts where we are listing the data changes, rather than representing\nthe current state of the data."]
        #[serde(
            rename = "modifiedTimeMillis",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub modified_time_millis: ::std::option::Option<i64>,
        #[doc = "If the data point is contained in a dataset for a derived data source,\nthis field will be populated with the data source stream ID that created\nthe data point originally.\n\nWARNING: do not rely on this field for anything other than debugging. The\nvalue of this field, if it is set at all, is an implementation detail and\nis not guaranteed to remain consistent."]
        #[serde(
            rename = "originDataSourceId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub origin_data_source_id: ::std::option::Option<String>,
        #[doc = "The raw timestamp from the original SensorEvent."]
        #[serde(
            rename = "rawTimestampNanos",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub raw_timestamp_nanos: ::std::option::Option<i64>,
        #[doc = "The start time of the interval represented by this data point, in\nnanoseconds since epoch."]
        #[serde(
            rename = "startTimeNanos",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub start_time_nanos: ::std::option::Option<i64>,
        #[doc = "Values of each data type field for the data point. It is expected that each\nvalue corresponding to a data type field will occur in the same order that\nthe field is listed with in the data type specified in a data source.\n\nOnly one of integer and floating point fields will be populated, depending\non the format enum value within data source's type field."]
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value: ::std::option::Option<Vec<crate::schemas::Value>>,
    }
    impl ::google_field_selector::FieldSelector for DataPoint {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DataPoint {
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
    pub struct DataSource {
        #[doc = "Information about an application which feeds sensor data into the platform."]
        #[serde(
            rename = "application",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub application: ::std::option::Option<crate::schemas::Application>,
        #[doc = "DO NOT POPULATE THIS FIELD. It is never populated in responses from the\nplatform, and is ignored in queries. It will be removed in a future version\nentirely."]
        #[serde(
            rename = "dataQualityStandard",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub data_quality_standard:
            ::std::option::Option<Vec<crate::schemas::DataSourceDataQualityStandardItems>>,
        #[doc = "A unique identifier for the data stream produced by this data source. The\nidentifier includes:<br/><br/>\n\n<ul>\n<li>The physical device's manufacturer, model, and serial number\n(UID).</li>\n<li>The application's package name or name. Package name is used when the\ndata source was created by an Android application. The developer project\nnumber is used when the data source was created by a REST client.</li>\n<li>The data source's type.</li>\n<li>The data source's stream name.</li>\n</ul>\nNote that not all attributes of the data source are used as part of the\nstream identifier. In particular, the version of the hardware/the\napplication isn't used. This allows us to preserve the same stream through\nversion updates. This also means that two DataSource objects may represent\nthe same data stream even if they're not equal.\n\nThe exact format of the data stream ID created by an Android application\nis:\n<var>type:dataType.name<wbr/>:application.packageName<wbr/>:device.manufacturer<wbr/>:device.model<wbr/>:device.uid<wbr/>:dataStreamName</var>\n\nThe exact format of the data stream ID created by a REST client is:\n<var>type:dataType.name<wbr/>:developer project\nnumber<wbr/>:device.manufacturer<wbr/>:device.model:device.uid<wbr/>:dataStreamName</var>\n\nWhen any of the optional fields that make up the data stream ID are absent,\nthey will be omitted from the data stream ID. The minimum viable data\nstream ID would be:\ntype:dataType.name:developer project number\n\nFinally, the developer project number and device UID are obfuscated when\nread by any REST or Android client that did not create the data source.\nOnly the data source creator will see the developer project number in clear\nand normal form. This means a client will see a different set of\ndata_stream_ids than another client with different credentials."]
        #[serde(
            rename = "dataStreamId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub data_stream_id: ::std::option::Option<String>,
        #[doc = "The stream name uniquely identifies this particular data source among\nother data sources of the same type from the same underlying producer.\nSetting the stream name is optional, but should be done whenever an\napplication exposes two streams for the same data type, or when a device\nhas two equivalent sensors."]
        #[serde(
            rename = "dataStreamName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub data_stream_name: ::std::option::Option<String>,
        #[doc = "The data type defines the schema for a stream of data being collected by,\ninserted into, or queried from the Fitness API."]
        #[serde(
            rename = "dataType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub data_type: ::std::option::Option<crate::schemas::DataType>,
        #[doc = "Representation of an integrated device (such as a phone or a wearable) that\ncan hold sensors."]
        #[serde(
            rename = "device",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub device: ::std::option::Option<crate::schemas::Device>,
        #[doc = "An end-user visible name for this data source."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "A constant describing the type of this data source. Indicates whether this\ndata source produces raw or derived data."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::DataSourceType>,
    }
    impl ::google_field_selector::FieldSelector for DataSource {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DataSource {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DataSourceDataQualityStandardItems {
        DataQualityBloodGlucoseIso151972003,
        DataQualityBloodGlucoseIso151972013,
        DataQualityBloodPressureAami,
        DataQualityBloodPressureBhsAA,
        DataQualityBloodPressureBhsAB,
        DataQualityBloodPressureBhsBA,
        DataQualityBloodPressureBhsBB,
        DataQualityBloodPressureEsh2002,
        DataQualityBloodPressureEsh2010,
        DataQualityUnknown,
    }
    impl DataSourceDataQualityStandardItems {
        pub fn as_str(self) -> &'static str {
            match self {
                DataSourceDataQualityStandardItems::DataQualityBloodGlucoseIso151972003 => {
                    "dataQualityBloodGlucoseIso151972003"
                }
                DataSourceDataQualityStandardItems::DataQualityBloodGlucoseIso151972013 => {
                    "dataQualityBloodGlucoseIso151972013"
                }
                DataSourceDataQualityStandardItems::DataQualityBloodPressureAami => {
                    "dataQualityBloodPressureAami"
                }
                DataSourceDataQualityStandardItems::DataQualityBloodPressureBhsAA => {
                    "dataQualityBloodPressureBhsAA"
                }
                DataSourceDataQualityStandardItems::DataQualityBloodPressureBhsAB => {
                    "dataQualityBloodPressureBhsAB"
                }
                DataSourceDataQualityStandardItems::DataQualityBloodPressureBhsBA => {
                    "dataQualityBloodPressureBhsBA"
                }
                DataSourceDataQualityStandardItems::DataQualityBloodPressureBhsBB => {
                    "dataQualityBloodPressureBhsBB"
                }
                DataSourceDataQualityStandardItems::DataQualityBloodPressureEsh2002 => {
                    "dataQualityBloodPressureEsh2002"
                }
                DataSourceDataQualityStandardItems::DataQualityBloodPressureEsh2010 => {
                    "dataQualityBloodPressureEsh2010"
                }
                DataSourceDataQualityStandardItems::DataQualityUnknown => "dataQualityUnknown",
            }
        }
    }
    impl ::std::convert::AsRef<str> for DataSourceDataQualityStandardItems {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for DataSourceDataQualityStandardItems {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<DataSourceDataQualityStandardItems, ()> {
            Ok(match s {
                "dataQualityBloodGlucoseIso151972003" => {
                    DataSourceDataQualityStandardItems::DataQualityBloodGlucoseIso151972003
                }
                "dataQualityBloodGlucoseIso151972013" => {
                    DataSourceDataQualityStandardItems::DataQualityBloodGlucoseIso151972013
                }
                "dataQualityBloodPressureAami" => {
                    DataSourceDataQualityStandardItems::DataQualityBloodPressureAami
                }
                "dataQualityBloodPressureBhsAA" => {
                    DataSourceDataQualityStandardItems::DataQualityBloodPressureBhsAA
                }
                "dataQualityBloodPressureBhsAB" => {
                    DataSourceDataQualityStandardItems::DataQualityBloodPressureBhsAB
                }
                "dataQualityBloodPressureBhsBA" => {
                    DataSourceDataQualityStandardItems::DataQualityBloodPressureBhsBA
                }
                "dataQualityBloodPressureBhsBB" => {
                    DataSourceDataQualityStandardItems::DataQualityBloodPressureBhsBB
                }
                "dataQualityBloodPressureEsh2002" => {
                    DataSourceDataQualityStandardItems::DataQualityBloodPressureEsh2002
                }
                "dataQualityBloodPressureEsh2010" => {
                    DataSourceDataQualityStandardItems::DataQualityBloodPressureEsh2010
                }
                "dataQualityUnknown" => DataSourceDataQualityStandardItems::DataQualityUnknown,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for DataSourceDataQualityStandardItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DataSourceDataQualityStandardItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DataSourceDataQualityStandardItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "dataQualityBloodGlucoseIso151972003" => {
                    DataSourceDataQualityStandardItems::DataQualityBloodGlucoseIso151972003
                }
                "dataQualityBloodGlucoseIso151972013" => {
                    DataSourceDataQualityStandardItems::DataQualityBloodGlucoseIso151972013
                }
                "dataQualityBloodPressureAami" => {
                    DataSourceDataQualityStandardItems::DataQualityBloodPressureAami
                }
                "dataQualityBloodPressureBhsAA" => {
                    DataSourceDataQualityStandardItems::DataQualityBloodPressureBhsAA
                }
                "dataQualityBloodPressureBhsAB" => {
                    DataSourceDataQualityStandardItems::DataQualityBloodPressureBhsAB
                }
                "dataQualityBloodPressureBhsBA" => {
                    DataSourceDataQualityStandardItems::DataQualityBloodPressureBhsBA
                }
                "dataQualityBloodPressureBhsBB" => {
                    DataSourceDataQualityStandardItems::DataQualityBloodPressureBhsBB
                }
                "dataQualityBloodPressureEsh2002" => {
                    DataSourceDataQualityStandardItems::DataQualityBloodPressureEsh2002
                }
                "dataQualityBloodPressureEsh2010" => {
                    DataSourceDataQualityStandardItems::DataQualityBloodPressureEsh2010
                }
                "dataQualityUnknown" => DataSourceDataQualityStandardItems::DataQualityUnknown,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for DataSourceDataQualityStandardItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DataSourceDataQualityStandardItems {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DataSourceType {
        Derived,
        Raw,
    }
    impl DataSourceType {
        pub fn as_str(self) -> &'static str {
            match self {
                DataSourceType::Derived => "derived",
                DataSourceType::Raw => "raw",
            }
        }
    }
    impl ::std::convert::AsRef<str> for DataSourceType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for DataSourceType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<DataSourceType, ()> {
            Ok(match s {
                "derived" => DataSourceType::Derived,
                "raw" => DataSourceType::Raw,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for DataSourceType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DataSourceType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DataSourceType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "derived" => DataSourceType::Derived,
                "raw" => DataSourceType::Raw,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for DataSourceType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DataSourceType {
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
    pub struct DataType {
        #[doc = "A field represents one dimension of a data type."]
        #[serde(
            rename = "field",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub field: ::std::option::Option<Vec<crate::schemas::DataTypeField>>,
        #[doc = "Each data type has a unique, namespaced, name. All data types in the\ncom.google namespace are shared as part of the platform."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for DataType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DataType {
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
    pub struct DataTypeField {
        #[doc = "The different supported formats for each field in a data type."]
        #[serde(
            rename = "format",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub format: ::std::option::Option<crate::schemas::DataTypeFieldFormat>,
        #[doc = "Defines the name and format of data. Unlike data type names, field names\nare not namespaced, and only need to be unique within the data type."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[serde(
            rename = "optional",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub optional: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for DataTypeField {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DataTypeField {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DataTypeFieldFormat {
        Blob,
        FloatList,
        FloatPoint,
        Integer,
        IntegerList,
        Map,
        String,
    }
    impl DataTypeFieldFormat {
        pub fn as_str(self) -> &'static str {
            match self {
                DataTypeFieldFormat::Blob => "blob",
                DataTypeFieldFormat::FloatList => "floatList",
                DataTypeFieldFormat::FloatPoint => "floatPoint",
                DataTypeFieldFormat::Integer => "integer",
                DataTypeFieldFormat::IntegerList => "integerList",
                DataTypeFieldFormat::Map => "map",
                DataTypeFieldFormat::String => "string",
            }
        }
    }
    impl ::std::convert::AsRef<str> for DataTypeFieldFormat {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for DataTypeFieldFormat {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<DataTypeFieldFormat, ()> {
            Ok(match s {
                "blob" => DataTypeFieldFormat::Blob,
                "floatList" => DataTypeFieldFormat::FloatList,
                "floatPoint" => DataTypeFieldFormat::FloatPoint,
                "integer" => DataTypeFieldFormat::Integer,
                "integerList" => DataTypeFieldFormat::IntegerList,
                "map" => DataTypeFieldFormat::Map,
                "string" => DataTypeFieldFormat::String,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for DataTypeFieldFormat {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DataTypeFieldFormat {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DataTypeFieldFormat {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "blob" => DataTypeFieldFormat::Blob,
                "floatList" => DataTypeFieldFormat::FloatList,
                "floatPoint" => DataTypeFieldFormat::FloatPoint,
                "integer" => DataTypeFieldFormat::Integer,
                "integerList" => DataTypeFieldFormat::IntegerList,
                "map" => DataTypeFieldFormat::Map,
                "string" => DataTypeFieldFormat::String,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for DataTypeFieldFormat {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DataTypeFieldFormat {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Dataset {
        #[doc = "The data stream ID of the data source that created the points in this\ndataset."]
        #[serde(
            rename = "dataSourceId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub data_source_id: ::std::option::Option<String>,
        #[doc = "The largest end time of all data points in this possibly partial\nrepresentation of the dataset. Time is in nanoseconds from epoch. This\nshould also match the second part of the dataset identifier."]
        #[serde(
            rename = "maxEndTimeNs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub max_end_time_ns: ::std::option::Option<i64>,
        #[doc = "The smallest start time of all data points in this possibly partial\nrepresentation of the dataset. Time is in nanoseconds from epoch. This\nshould also match the first part of the dataset identifier."]
        #[serde(
            rename = "minStartTimeNs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub min_start_time_ns: ::std::option::Option<i64>,
        #[doc = "This token will be set when a dataset is received in response to a GET\nrequest and the dataset is too large to be included in a single response.\nProvide this value in a subsequent GET request to return the next page of\ndata points within this dataset."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "A partial list of data points contained in the dataset, ordered by largest\nendTimeNanos first. This list is considered complete when retrieving a\nsmall dataset and partial when patching a dataset or retrieving a dataset\nthat is too large to include in a single response."]
        #[serde(
            rename = "point",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub point: ::std::option::Option<Vec<crate::schemas::DataPoint>>,
    }
    impl ::google_field_selector::FieldSelector for Dataset {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Dataset {
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
    pub struct Device {
        #[doc = "Manufacturer of the product/hardware."]
        #[serde(
            rename = "manufacturer",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub manufacturer: ::std::option::Option<String>,
        #[doc = "End-user visible model name for the device."]
        #[serde(
            rename = "model",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub model: ::std::option::Option<String>,
        #[doc = "A constant representing the type of the device."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::DeviceType>,
        #[doc = "The serial number or other unique ID for the hardware. This field is\nobfuscated when read by any REST or Android client that did not create\nthe data source. Only the data source creator will see the uid field in\nclear and normal form.\n\nThe obfuscation preserves equality; that is, given two IDs, if id1 == id2,\nobfuscated(id1) == obfuscated(id2)."]
        #[serde(
            rename = "uid",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub uid: ::std::option::Option<String>,
        #[doc = "Version string for the device hardware/software."]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Device {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Device {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DeviceType {
        #[doc = "A chest strap."]
        ChestStrap,
        #[doc = "Glass or other head-mounted device."]
        HeadMounted,
        #[doc = "An Android phone."]
        Phone,
        #[doc = "A scale."]
        Scale,
        #[doc = "An Android tablet."]
        Tablet,
        #[doc = "Device type is not known."]
        Unknown,
        #[doc = "A watch or other wrist-mounted band."]
        Watch,
    }
    impl DeviceType {
        pub fn as_str(self) -> &'static str {
            match self {
                DeviceType::ChestStrap => "chestStrap",
                DeviceType::HeadMounted => "headMounted",
                DeviceType::Phone => "phone",
                DeviceType::Scale => "scale",
                DeviceType::Tablet => "tablet",
                DeviceType::Unknown => "unknown",
                DeviceType::Watch => "watch",
            }
        }
    }
    impl ::std::convert::AsRef<str> for DeviceType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for DeviceType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<DeviceType, ()> {
            Ok(match s {
                "chestStrap" => DeviceType::ChestStrap,
                "headMounted" => DeviceType::HeadMounted,
                "phone" => DeviceType::Phone,
                "scale" => DeviceType::Scale,
                "tablet" => DeviceType::Tablet,
                "unknown" => DeviceType::Unknown,
                "watch" => DeviceType::Watch,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for DeviceType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DeviceType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DeviceType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "chestStrap" => DeviceType::ChestStrap,
                "headMounted" => DeviceType::HeadMounted,
                "phone" => DeviceType::Phone,
                "scale" => DeviceType::Scale,
                "tablet" => DeviceType::Tablet,
                "unknown" => DeviceType::Unknown,
                "watch" => DeviceType::Watch,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for DeviceType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DeviceType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ListDataPointChangesResponse {
        #[doc = "The data stream ID of the data source with data point changes."]
        #[serde(
            rename = "dataSourceId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub data_source_id: ::std::option::Option<String>,
        #[doc = "Deleted data points for the user. Note, for modifications this should be\nparsed before handling insertions."]
        #[serde(
            rename = "deletedDataPoint",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub deleted_data_point: ::std::option::Option<Vec<crate::schemas::DataPoint>>,
        #[doc = "Inserted data points for the user."]
        #[serde(
            rename = "insertedDataPoint",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub inserted_data_point: ::std::option::Option<Vec<crate::schemas::DataPoint>>,
        #[doc = "The continuation token, which is used to page through large result sets.\nProvide this value in a subsequent request to return the next page of\nresults."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ListDataPointChangesResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListDataPointChangesResponse {
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
    pub struct ListDataSourcesResponse {
        #[doc = "A previously created data source."]
        #[serde(
            rename = "dataSource",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub data_source: ::std::option::Option<Vec<crate::schemas::DataSource>>,
    }
    impl ::google_field_selector::FieldSelector for ListDataSourcesResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListDataSourcesResponse {
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
    pub struct ListSessionsResponse {
        #[doc = "If <code>includeDeleted</code> is set to true in the request, and\n<var>startTime</var> and <var>endTime</var> are omitted, this will include\nsessions which were deleted since the last sync."]
        #[serde(
            rename = "deletedSession",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub deleted_session: ::std::option::Option<Vec<crate::schemas::Session>>,
        #[doc = "Flag to indicate server has more data to transfer.\nDO NOT USE THIS FIELD. It is never populated in responses from the server."]
        #[serde(
            rename = "hasMoreData",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub has_more_data: ::std::option::Option<bool>,
        #[doc = "The sync token which is used to sync further changes. This will only be\nprovided if both <var>startTime</var> and <var>endTime</var> are omitted\nfrom the request."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "Sessions with an end time that is between <var>startTime</var> and\n<var>endTime</var> of the request."]
        #[serde(
            rename = "session",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub session: ::std::option::Option<Vec<crate::schemas::Session>>,
    }
    impl ::google_field_selector::FieldSelector for ListSessionsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListSessionsResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct MapValue {
        #[doc = "Floating point value."]
        #[serde(
            rename = "fpVal",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fp_val: ::std::option::Option<f64>,
    }
    impl ::google_field_selector::FieldSelector for MapValue {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MapValue {
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
    pub struct Session {
        #[doc = "Session active time. While start_time_millis and end_time_millis define\nthe full session time, the active time can be shorter and specified by\nactive_time_millis.\nIf the inactive time during the session is known, it should also be\ninserted via a com.google.activity.segment data point with a STILL\nactivity value"]
        #[serde(
            rename = "activeTimeMillis",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub active_time_millis: ::std::option::Option<i64>,
        #[doc = "The type of activity this session represents."]
        #[serde(
            rename = "activityType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub activity_type: ::std::option::Option<i32>,
        #[doc = "The application that created the session."]
        #[serde(
            rename = "application",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub application: ::std::option::Option<crate::schemas::Application>,
        #[doc = "A description for this session."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "An end time, in milliseconds since epoch, inclusive."]
        #[serde(
            rename = "endTimeMillis",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub end_time_millis: ::std::option::Option<i64>,
        #[doc = "A client-generated identifier that is unique across all sessions owned by\nthis particular user."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "A timestamp that indicates when the session was last modified."]
        #[serde(
            rename = "modifiedTimeMillis",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub modified_time_millis: ::std::option::Option<i64>,
        #[doc = "A human readable name of the session."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "A start time, in milliseconds since epoch, inclusive."]
        #[serde(
            rename = "startTimeMillis",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub start_time_millis: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for Session {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Session {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Value {
        #[doc = "Floating point value. When this is set, other values must not be set."]
        #[serde(
            rename = "fpVal",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fp_val: ::std::option::Option<f64>,
        #[doc = "Integer value. When this is set, other values must not be set."]
        #[serde(
            rename = "intVal",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub int_val: ::std::option::Option<i32>,
        #[doc = "Map value.  The valid key space and units for the corresponding value\nof each entry should be documented as part of the data type definition.\nKeys should be kept small whenever possible. Data streams with large keys\nand high data frequency may be down sampled."]
        #[serde(
            rename = "mapVal",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub map_val: ::std::option::Option<Vec<crate::schemas::ValueMapValEntry>>,
        #[doc = "String value.  When this is set, other values must not be set.\nStrings should be kept small whenever possible.  Data streams with large\nstring values and high data frequency may be down sampled."]
        #[serde(
            rename = "stringVal",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub string_val: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Value {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Value {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ValueMapValEntry {
        #[serde(
            rename = "key",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub key: ::std::option::Option<String>,
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value: ::std::option::Option<crate::schemas::MapValue>,
    }
    impl ::google_field_selector::FieldSelector for ValueMapValEntry {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ValueMapValEntry {
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
    #[doc = "Actions that can be performed on the users resource"]
    pub fn users(&self) -> crate::resources::users::UsersActions {
        crate::resources::users::UsersActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
}
pub mod resources {
    pub mod users {
        pub mod params {}
        pub struct UsersActions<'a> {
            pub(crate) reqwest: &'a reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> UsersActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Actions that can be performed on the data_sources resource"]
            pub fn data_sources(
                &self,
            ) -> crate::resources::users::data_sources::DataSourcesActions {
                crate::resources::users::data_sources::DataSourcesActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
            #[doc = "Actions that can be performed on the dataset resource"]
            pub fn dataset(&self) -> crate::resources::users::dataset::DatasetActions {
                crate::resources::users::dataset::DatasetActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
            #[doc = "Actions that can be performed on the sessions resource"]
            pub fn sessions(&self) -> crate::resources::users::sessions::SessionsActions {
                crate::resources::users::sessions::SessionsActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
        }
        pub mod data_sources {
            pub mod params {}
            pub struct DataSourcesActions<'a> {
                pub(crate) reqwest: &'a reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> DataSourcesActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Creates a new data source that is unique across all data sources belonging\nto this user.\n\nA data source is a unique source of sensor data. Data sources can expose\nraw data coming from hardware sensors on local or companion devices. They\ncan also expose derived data, created by transforming or merging other data\nsources. Multiple data sources can exist for the same data type. Every data\npoint in every dataset inserted into or read from the Fitness API has an\nassociated data source.\n\nEach data source produces a unique stream of dataset updates, with a\nunique data source identifier. Not all changes to data source affect the\ndata stream ID, so that data collected by updated versions of the same\napplication/device can still be considered to belong to the same data\nsource.\n\nData sources are identified using a string generated by the server, based\non the contents of the source being created. The <code>dataStreamId</code>\nfield should not be set when invoking this method. It\nwill be automatically generated by the server with the correct format. If\na <code>dataStreamId</code> is set, it must match the format that the\nserver would generate. This format is a combination of some fields from the\ndata source, and has a specific order. If it doesn't match, the request\nwill fail with an error.\n\nSpecifying a DataType which is not a known type (beginning with\n\"com.google.\") will create a DataSource with a <em>custom data type</em>.\nCustom data types are only readable by the application that created them.\nCustom data types are <strong>deprecated</strong>; use standard data types\ninstead.\n\nIn addition to the data source fields included in the data source ID, the\ndeveloper project number that is authenticated when creating the data\nsource is included. This developer project number is obfuscated when read\nby any other developer reading public data types."]
                pub fn create(
                    &self,
                    request: crate::schemas::DataSource,
                    user_id: impl Into<String>,
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
                        user_id: user_id.into(),
                    }
                }
                #[doc = "Deletes the specified data source. The request will fail if the data\nsource contains any data points."]
                pub fn delete(
                    &self,
                    user_id: impl Into<String>,
                    data_source_id: impl Into<String>,
                ) -> DeleteRequestBuilder {
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
                        user_id: user_id.into(),
                        data_source_id: data_source_id.into(),
                    }
                }
                #[doc = "Returns the specified data source."]
                pub fn get(
                    &self,
                    user_id: impl Into<String>,
                    data_source_id: impl Into<String>,
                ) -> GetRequestBuilder {
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
                        user_id: user_id.into(),
                        data_source_id: data_source_id.into(),
                    }
                }
                #[doc = "Lists all data sources that are visible to the developer, using the OAuth\nscopes provided. The list is not exhaustive; the user may have private\ndata sources that are only visible to other developers, or calls using\nother scopes."]
                pub fn list(&self, user_id: impl Into<String>) -> ListRequestBuilder {
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
                        user_id: user_id.into(),
                        data_type_name: None,
                    }
                }
                #[doc = "Updates the specified data source. The <code>dataStreamId</code>,\n<code>dataType</code>, <code>type</code>, <code>dataStreamName</code>, and\n<code>device</code> properties with the exception of <code>version</code>,\ncannot be modified.\n\nData sources are identified by their <code>dataStreamId</code>."]
                pub fn update(
                    &self,
                    request: crate::schemas::DataSource,
                    user_id: impl Into<String>,
                    data_source_id: impl Into<String>,
                ) -> UpdateRequestBuilder {
                    UpdateRequestBuilder {
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
                        user_id: user_id.into(),
                        data_source_id: data_source_id.into(),
                    }
                }
                #[doc = "Actions that can be performed on the data_point_changes resource"]pub fn data_point_changes ( & self ) -> crate :: resources :: users :: data_sources :: data_point_changes :: DataPointChangesActions{
                    crate :: resources :: users :: data_sources :: data_point_changes :: DataPointChangesActions { reqwest : & self . reqwest , auth : self . auth_ref ( ) , }
                }
                #[doc = "Actions that can be performed on the datasets resource"]
                pub fn datasets(
                    &self,
                ) -> crate::resources::users::data_sources::datasets::DatasetsActions
                {
                    crate::resources::users::data_sources::datasets::DatasetsActions {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                    }
                }
            }
            #[doc = "Created via [DataSourcesActions::create()](struct.DataSourcesActions.html#method.create)"]
            #[derive(Debug, Clone)]
            pub struct CreateRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::DataSource,
                user_id: String,
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
                ) -> Result<crate::schemas::DataSource, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::DataSource, crate::Error> {
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
                    let mut output = "https://www.googleapis.com/".to_owned();
                    output.push_str("fitness/v1/users/");
                    {
                        let var_as_str = &self.user_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/dataSources");
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
            #[doc = "Created via [DataSourcesActions::delete()](struct.DataSourcesActions.html#method.delete)"]
            #[derive(Debug, Clone)]
            pub struct DeleteRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                user_id: String,
                data_source_id: String,
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
                ) -> Result<crate::schemas::DataSource, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::DataSource, crate::Error> {
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
                    let mut output = "https://www.googleapis.com/".to_owned();
                    output.push_str("fitness/v1/users/");
                    {
                        let var_as_str = &self.user_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/dataSources/");
                    {
                        let var_as_str = &self.data_source_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
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
            #[doc = "Created via [DataSourcesActions::get()](struct.DataSourcesActions.html#method.get)"]
            #[derive(Debug, Clone)]
            pub struct GetRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                user_id: String,
                data_source_id: String,
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
                ) -> Result<crate::schemas::DataSource, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::DataSource, crate::Error> {
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
                    let mut output = "https://www.googleapis.com/".to_owned();
                    output.push_str("fitness/v1/users/");
                    {
                        let var_as_str = &self.user_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/dataSources/");
                    {
                        let var_as_str = &self.data_source_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
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
            #[doc = "Created via [DataSourcesActions::list()](struct.DataSourcesActions.html#method.list)"]
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                user_id: String,
                data_type_name: Option<Vec<String>>,
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
                #[doc = "The names of data types to include in the list. If not specified, all\ndata sources will be returned."]
                pub fn data_type_name(mut self, value: impl Into<Vec<String>>) -> Self {
                    self.data_type_name = Some(value.into());
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
                ) -> Result<crate::schemas::ListDataSourcesResponse, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::ListDataSourcesResponse, crate::Error> {
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
                    let mut output = "https://www.googleapis.com/".to_owned();
                    output.push_str("fitness/v1/users/");
                    {
                        let var_as_str = &self.user_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/dataSources");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("dataTypeName", &self.data_type_name)]);
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
            #[doc = "Created via [DataSourcesActions::update()](struct.DataSourcesActions.html#method.update)"]
            #[derive(Debug, Clone)]
            pub struct UpdateRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::DataSource,
                user_id: String,
                data_source_id: String,
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
            impl<'a> UpdateRequestBuilder<'a> {
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
                ) -> Result<crate::schemas::DataSource, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::DataSource, crate::Error> {
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
                    let mut output = "https://www.googleapis.com/".to_owned();
                    output.push_str("fitness/v1/users/");
                    {
                        let var_as_str = &self.user_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/dataSources/");
                    {
                        let var_as_str = &self.data_source_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let req = self.reqwest.request(::reqwest::Method::PUT, path);
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
            pub mod data_point_changes {
                pub mod params {}
                pub struct DataPointChangesActions<'a> {
                    pub(crate) reqwest: &'a reqwest::blocking::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                }
                impl<'a> DataPointChangesActions<'a> {
                    fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                        self.auth
                    }
                    #[doc = "Queries for user's data point changes for a particular data source."]
                    pub fn list(
                        &self,
                        user_id: impl Into<String>,
                        data_source_id: impl Into<String>,
                    ) -> ListRequestBuilder {
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
                            user_id: user_id.into(),
                            data_source_id: data_source_id.into(),
                            limit: None,
                            page_token: None,
                        }
                    }
                }
                #[doc = "Created via [DataPointChangesActions::list()](struct.DataPointChangesActions.html#method.list)"]
                #[derive(Debug, Clone)]
                pub struct ListRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    user_id: String,
                    data_source_id: String,
                    limit: Option<i32>,
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
                    #[doc = "If specified, no more than this many data point changes will be included\nin the response."]
                    pub fn limit(mut self, value: i32) -> Self {
                        self.limit = Some(value);
                        self
                    }
                    #[doc = "The continuation token, which is used to page through large result sets.\nTo get the next page of results, set this parameter to the value of\n<code>nextPageToken</code> from the previous response."]
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
                    pub fn iter_deleted_data_point<T>(self) -> crate::iter::PageItemIter<Self, T>
                    where
                        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.iter_deleted_data_point_with_fields(fields)
                    }
                    #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                    #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                    #[doc = r" fields in `#items_type` will be the default fields populated by"]
                    #[doc = r" the server."]
                    pub fn iter_deleted_data_point_with_default_fields(
                        self,
                    ) -> crate::iter::PageItemIter<Self, crate::schemas::DataPoint>
                    {
                        self.iter_deleted_data_point_with_fields(None::<String>)
                    }
                    #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                    #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                    #[doc = r" fields in `#items_type` will be all fields available. This should"]
                    #[doc = r" primarily be used during developement and debugging as fetching"]
                    #[doc = r" all fields can be expensive both in bandwidth and server"]
                    #[doc = r" resources."]
                    pub fn iter_deleted_data_point_with_all_fields(
                        self,
                    ) -> crate::iter::PageItemIter<Self, crate::schemas::DataPoint>
                    {
                        self.iter_deleted_data_point_with_fields(Some("*"))
                    }
                    pub fn iter_deleted_data_point_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> crate::iter::PageItemIter<Self, T>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: AsRef<str>,
                    {
                        self.fields = Some({
                            let mut selector =
                                concat!("nextPageToken,", "deletedDataPoint").to_owned();
                            let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                            if !items_fields.is_empty() {
                                selector.push_str("(");
                                selector.push_str(items_fields);
                                selector.push_str(")");
                            }
                            selector
                        });
                        crate::iter::PageItemIter::new(self, "deletedDataPoint")
                    }
                    #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                    #[doc = r" items yielded by the iterator are chosen by the caller of this"]
                    #[doc = r" method and must implement `Deserialize` and `FieldSelector`. The"]
                    #[doc = r" populated fields in the yielded items will be determined by the"]
                    #[doc = r" `FieldSelector` implementation."]
                    pub fn iter_inserted_data_point<T>(self) -> crate::iter::PageItemIter<Self, T>
                    where
                        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.iter_inserted_data_point_with_fields(fields)
                    }
                    #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                    #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                    #[doc = r" fields in `#items_type` will be the default fields populated by"]
                    #[doc = r" the server."]
                    pub fn iter_inserted_data_point_with_default_fields(
                        self,
                    ) -> crate::iter::PageItemIter<Self, crate::schemas::DataPoint>
                    {
                        self.iter_inserted_data_point_with_fields(None::<String>)
                    }
                    #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                    #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                    #[doc = r" fields in `#items_type` will be all fields available. This should"]
                    #[doc = r" primarily be used during developement and debugging as fetching"]
                    #[doc = r" all fields can be expensive both in bandwidth and server"]
                    #[doc = r" resources."]
                    pub fn iter_inserted_data_point_with_all_fields(
                        self,
                    ) -> crate::iter::PageItemIter<Self, crate::schemas::DataPoint>
                    {
                        self.iter_inserted_data_point_with_fields(Some("*"))
                    }
                    pub fn iter_inserted_data_point_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> crate::iter::PageItemIter<Self, T>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: AsRef<str>,
                    {
                        self.fields = Some({
                            let mut selector =
                                concat!("nextPageToken,", "insertedDataPoint").to_owned();
                            let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                            if !items_fields.is_empty() {
                                selector.push_str("(");
                                selector.push_str(items_fields);
                                selector.push_str(")");
                            }
                            selector
                        });
                        crate::iter::PageItemIter::new(self, "insertedDataPoint")
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
                    ) -> crate::iter::PageIter<Self, crate::schemas::ListDataPointChangesResponse>
                    {
                        self.iter_with_fields(None::<&str>)
                    }
                    pub fn iter_with_all_fields(
                        self,
                    ) -> crate::iter::PageIter<Self, crate::schemas::ListDataPointChangesResponse>
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
                    ) -> Result<crate::schemas::ListDataPointChangesResponse, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::ListDataPointChangesResponse, crate::Error>
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
                        let mut output = "https://www.googleapis.com/".to_owned();
                        output.push_str("fitness/v1/users/");
                        {
                            let var_as_str = &self.user_id;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::SIMPLE,
                            ));
                        }
                        output.push_str("/dataSources/");
                        {
                            let var_as_str = &self.data_source_id;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::SIMPLE,
                            ));
                        }
                        output.push_str("/dataPointChanges");
                        output
                    }
                    fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error>
                    {
                        let req = self.reqwest.request(::reqwest::Method::GET, path);
                        let req = req.query(&[("limit", &self.limit)]);
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
            pub mod datasets {
                pub mod params {}
                pub struct DatasetsActions<'a> {
                    pub(crate) reqwest: &'a reqwest::blocking::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                }
                impl<'a> DatasetsActions<'a> {
                    fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                        self.auth
                    }
                    #[doc = "Performs an inclusive delete of all data points whose start and end times\nhave any overlap with the time range specified by the dataset ID. For most\ndata types, the entire data point will be deleted. For data types where the\ntime span represents a consistent value (such as\n<code>com.google.activity.segment</code>), and a data point straddles\neither end point of the dataset, only the overlapping portion of the data\npoint will be deleted."]
                    pub fn delete(
                        &self,
                        user_id: impl Into<String>,
                        data_source_id: impl Into<String>,
                        dataset_id: impl Into<String>,
                    ) -> DeleteRequestBuilder {
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
                            user_id: user_id.into(),
                            data_source_id: data_source_id.into(),
                            dataset_id: dataset_id.into(),
                            current_time_millis: None,
                            modified_time_millis: None,
                        }
                    }
                    #[doc = "Returns a dataset containing all data points whose start and end times\noverlap with the specified range of the dataset minimum start time and\nmaximum end time. Specifically, any data point whose start time is less\nthan or equal to the dataset end time and whose end time is greater than or\nequal to the dataset start time."]
                    pub fn get(
                        &self,
                        user_id: impl Into<String>,
                        data_source_id: impl Into<String>,
                        dataset_id: impl Into<String>,
                    ) -> GetRequestBuilder {
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
                            user_id: user_id.into(),
                            data_source_id: data_source_id.into(),
                            dataset_id: dataset_id.into(),
                            limit: None,
                            page_token: None,
                        }
                    }
                    #[doc = "Adds data points to a dataset. The dataset need not be previously created.\nAll points within the given dataset will be returned with subsquent calls\nto retrieve this dataset. Data points can belong to more than one dataset.\nThis method does not use patch semantics."]
                    pub fn patch(
                        &self,
                        request: crate::schemas::Dataset,
                        user_id: impl Into<String>,
                        data_source_id: impl Into<String>,
                        dataset_id: impl Into<String>,
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
                            user_id: user_id.into(),
                            data_source_id: data_source_id.into(),
                            dataset_id: dataset_id.into(),
                            current_time_millis: None,
                        }
                    }
                }
                #[doc = "Created via [DatasetsActions::delete()](struct.DatasetsActions.html#method.delete)"]
                #[derive(Debug, Clone)]
                pub struct DeleteRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    user_id: String,
                    data_source_id: String,
                    dataset_id: String,
                    current_time_millis: Option<i64>,
                    modified_time_millis: Option<i64>,
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
                    #[doc = "The client's current time in milliseconds since epoch."]
                    pub fn current_time_millis(mut self, value: i64) -> Self {
                        self.current_time_millis = Some(value);
                        self
                    }
                    #[doc = "When the operation was performed on the client."]
                    pub fn modified_time_millis(mut self, value: i64) -> Self {
                        self.modified_time_millis = Some(value);
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
                    pub fn execute(self) -> Result<(), crate::Error> {
                        let req = self._request(&self._path())?;
                        crate::error_from_response(req.send()?)?;
                        Ok(())
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://www.googleapis.com/".to_owned();
                        output.push_str("fitness/v1/users/");
                        {
                            let var_as_str = &self.user_id;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::SIMPLE,
                            ));
                        }
                        output.push_str("/dataSources/");
                        {
                            let var_as_str = &self.data_source_id;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::SIMPLE,
                            ));
                        }
                        output.push_str("/datasets/");
                        {
                            let var_as_str = &self.dataset_id;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::SIMPLE,
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
                        let req = req.query(&[("currentTimeMillis", &self.current_time_millis)]);
                        let req = req.query(&[("modifiedTimeMillis", &self.modified_time_millis)]);
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
                #[doc = "Created via [DatasetsActions::get()](struct.DatasetsActions.html#method.get)"]
                #[derive(Debug, Clone)]
                pub struct GetRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    user_id: String,
                    data_source_id: String,
                    dataset_id: String,
                    limit: Option<i32>,
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
                impl<'a> GetRequestBuilder<'a> {
                    #[doc = "If specified, no more than this many data points will be included in the\ndataset. If there are more data points in the dataset, nextPageToken\nwill be set in the dataset response."]
                    pub fn limit(mut self, value: i32) -> Self {
                        self.limit = Some(value);
                        self
                    }
                    #[doc = "The continuation token, which is used to page through large datasets.\nTo get the next page of a dataset, set this parameter to the value of\n<code>nextPageToken</code> from the previous response. Each subsequent\ncall will yield a partial dataset with data point end timestamps that are\nstrictly smaller than those in the previous partial response."]
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
                    pub fn iter_point<T>(self) -> crate::iter::PageItemIter<Self, T>
                    where
                        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.iter_point_with_fields(fields)
                    }
                    #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                    #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                    #[doc = r" fields in `#items_type` will be the default fields populated by"]
                    #[doc = r" the server."]
                    pub fn iter_point_with_default_fields(
                        self,
                    ) -> crate::iter::PageItemIter<Self, crate::schemas::DataPoint>
                    {
                        self.iter_point_with_fields(None::<String>)
                    }
                    #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                    #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                    #[doc = r" fields in `#items_type` will be all fields available. This should"]
                    #[doc = r" primarily be used during developement and debugging as fetching"]
                    #[doc = r" all fields can be expensive both in bandwidth and server"]
                    #[doc = r" resources."]
                    pub fn iter_point_with_all_fields(
                        self,
                    ) -> crate::iter::PageItemIter<Self, crate::schemas::DataPoint>
                    {
                        self.iter_point_with_fields(Some("*"))
                    }
                    pub fn iter_point_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> crate::iter::PageItemIter<Self, T>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: AsRef<str>,
                    {
                        self.fields = Some({
                            let mut selector = concat!("nextPageToken,", "point").to_owned();
                            let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                            if !items_fields.is_empty() {
                                selector.push_str("(");
                                selector.push_str(items_fields);
                                selector.push_str(")");
                            }
                            selector
                        });
                        crate::iter::PageItemIter::new(self, "point")
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
                    ) -> crate::iter::PageIter<Self, crate::schemas::Dataset> {
                        self.iter_with_fields(None::<&str>)
                    }
                    pub fn iter_with_all_fields(
                        self,
                    ) -> crate::iter::PageIter<Self, crate::schemas::Dataset> {
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
                    ) -> Result<crate::schemas::Dataset, crate::Error> {
                        self.execute_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::Dataset, crate::Error> {
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
                        let mut output = "https://www.googleapis.com/".to_owned();
                        output.push_str("fitness/v1/users/");
                        {
                            let var_as_str = &self.user_id;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::SIMPLE,
                            ));
                        }
                        output.push_str("/dataSources/");
                        {
                            let var_as_str = &self.data_source_id;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::SIMPLE,
                            ));
                        }
                        output.push_str("/datasets/");
                        {
                            let var_as_str = &self.dataset_id;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::SIMPLE,
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
                        let req = req.query(&[("limit", &self.limit)]);
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
                impl<'a> crate::iter::IterableMethod for GetRequestBuilder<'a> {
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
                #[doc = "Created via [DatasetsActions::patch()](struct.DatasetsActions.html#method.patch)"]
                #[derive(Debug, Clone)]
                pub struct PatchRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    request: crate::schemas::Dataset,
                    user_id: String,
                    data_source_id: String,
                    dataset_id: String,
                    current_time_millis: Option<i64>,
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
                    #[doc = "The client's current time in milliseconds since epoch. Note that the\n<code>minStartTimeNs</code> and <code>maxEndTimeNs</code> properties in\nthe request body are in nanoseconds instead of milliseconds."]
                    pub fn current_time_millis(mut self, value: i64) -> Self {
                        self.current_time_millis = Some(value);
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
                    ) -> Result<crate::schemas::Dataset, crate::Error> {
                        self.execute_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::Dataset, crate::Error> {
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
                        let mut output = "https://www.googleapis.com/".to_owned();
                        output.push_str("fitness/v1/users/");
                        {
                            let var_as_str = &self.user_id;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::SIMPLE,
                            ));
                        }
                        output.push_str("/dataSources/");
                        {
                            let var_as_str = &self.data_source_id;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::SIMPLE,
                            ));
                        }
                        output.push_str("/datasets/");
                        {
                            let var_as_str = &self.dataset_id;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::SIMPLE,
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
                        let req = req.query(&[("currentTimeMillis", &self.current_time_millis)]);
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
        pub mod dataset {
            pub mod params {}
            pub struct DatasetActions<'a> {
                pub(crate) reqwest: &'a reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> DatasetActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Aggregates data of a certain type or stream into buckets divided by a given\ntype of boundary. Multiple data sets of multiple types and from multiple\nsources can be aggregated into exactly one bucket type per request."]
                pub fn aggregate(
                    &self,
                    request: crate::schemas::AggregateRequest,
                    user_id: impl Into<String>,
                ) -> AggregateRequestBuilder {
                    AggregateRequestBuilder {
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
                        user_id: user_id.into(),
                    }
                }
            }
            #[doc = "Created via [DatasetActions::aggregate()](struct.DatasetActions.html#method.aggregate)"]
            #[derive(Debug, Clone)]
            pub struct AggregateRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::AggregateRequest,
                user_id: String,
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
            impl<'a> AggregateRequestBuilder<'a> {
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
                ) -> Result<crate::schemas::AggregateResponse, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::AggregateResponse, crate::Error> {
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
                    let mut output = "https://www.googleapis.com/".to_owned();
                    output.push_str("fitness/v1/users/");
                    {
                        let var_as_str = &self.user_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/dataset:aggregate");
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
        pub mod sessions {
            pub mod params {}
            pub struct SessionsActions<'a> {
                pub(crate) reqwest: &'a reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> SessionsActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Deletes a session specified by the given session ID."]
                pub fn delete(
                    &self,
                    user_id: impl Into<String>,
                    session_id: impl Into<String>,
                ) -> DeleteRequestBuilder {
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
                        user_id: user_id.into(),
                        session_id: session_id.into(),
                        current_time_millis: None,
                    }
                }
                #[doc = "Lists sessions previously created."]
                pub fn list(&self, user_id: impl Into<String>) -> ListRequestBuilder {
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
                        user_id: user_id.into(),
                        activity_type: None,
                        end_time: None,
                        include_deleted: None,
                        page_token: None,
                        start_time: None,
                    }
                }
                #[doc = "Updates or insert a given session."]
                pub fn update(
                    &self,
                    request: crate::schemas::Session,
                    user_id: impl Into<String>,
                    session_id: impl Into<String>,
                ) -> UpdateRequestBuilder {
                    UpdateRequestBuilder {
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
                        user_id: user_id.into(),
                        session_id: session_id.into(),
                        current_time_millis: None,
                    }
                }
            }
            #[doc = "Created via [SessionsActions::delete()](struct.SessionsActions.html#method.delete)"]
            #[derive(Debug, Clone)]
            pub struct DeleteRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                user_id: String,
                session_id: String,
                current_time_millis: Option<i64>,
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
                #[doc = "The client's current time in milliseconds since epoch."]
                pub fn current_time_millis(mut self, value: i64) -> Self {
                    self.current_time_millis = Some(value);
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
                pub fn execute(self) -> Result<(), crate::Error> {
                    let req = self._request(&self._path())?;
                    crate::error_from_response(req.send()?)?;
                    Ok(())
                }
                fn _path(&self) -> String {
                    let mut output = "https://www.googleapis.com/".to_owned();
                    output.push_str("fitness/v1/users/");
                    {
                        let var_as_str = &self.user_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/sessions/");
                    {
                        let var_as_str = &self.session_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let req = self.reqwest.request(::reqwest::Method::DELETE, path);
                    let req = req.query(&[("currentTimeMillis", &self.current_time_millis)]);
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
            #[doc = "Created via [SessionsActions::list()](struct.SessionsActions.html#method.list)"]
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                user_id: String,
                activity_type: Option<Vec<i32>>,
                end_time: Option<String>,
                include_deleted: Option<bool>,
                page_token: Option<String>,
                start_time: Option<String>,
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
                #[doc = "If non-empty, only sessions with these activity types should be returned."]
                pub fn activity_type(mut self, value: impl Into<Vec<i32>>) -> Self {
                    self.activity_type = Some(value.into());
                    self
                }
                #[doc = "An <a href=\"https://www.ietf.org/rfc/rfc3339.txt\">RFC3339</a> timestamp.\nOnly sessions ending between the start and end times will be included in\nthe response. If this time is omitted but <var>startTime</var> is\nspecified, all sessions from <var>startTime</var> to the end of time will\nbe returned."]
                pub fn end_time(mut self, value: impl Into<String>) -> Self {
                    self.end_time = Some(value.into());
                    self
                }
                #[doc = "If true, and if both <var>startTime</var> and <var>endTime</var> are\nomitted, session deletions will be returned."]
                pub fn include_deleted(mut self, value: bool) -> Self {
                    self.include_deleted = Some(value);
                    self
                }
                #[doc = "The continuation token, which is used for incremental syncing.\nTo get the next batch of changes, set this parameter to the value of\n<code>nextPageToken</code> from the previous response. The page token is\nignored if either start or end time is specified. If none of start time,\nend time, and the page token is specified, sessions modified in the last\n30 days are returned."]
                pub fn page_token(mut self, value: impl Into<String>) -> Self {
                    self.page_token = Some(value.into());
                    self
                }
                #[doc = "An <a href=\"https://www.ietf.org/rfc/rfc3339.txt\">RFC3339</a> timestamp.\nOnly sessions ending between the start and end times will be included in\nthe response. If this time is omitted but <var>endTime</var> is specified,\nall sessions from the start of time up to <var>endTime</var> will be\nreturned."]
                pub fn start_time(mut self, value: impl Into<String>) -> Self {
                    self.start_time = Some(value.into());
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
                pub fn iter_deleted_session<T>(self) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.iter_deleted_session_with_fields(fields)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be the default fields populated by"]
                #[doc = r" the server."]
                pub fn iter_deleted_session_with_default_fields(
                    self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::Session> {
                    self.iter_deleted_session_with_fields(None::<String>)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be all fields available. This should"]
                #[doc = r" primarily be used during developement and debugging as fetching"]
                #[doc = r" all fields can be expensive both in bandwidth and server"]
                #[doc = r" resources."]
                pub fn iter_deleted_session_with_all_fields(
                    self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::Session> {
                    self.iter_deleted_session_with_fields(Some("*"))
                }
                pub fn iter_deleted_session_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: AsRef<str>,
                {
                    self.fields = Some({
                        let mut selector = concat!("nextPageToken,", "deletedSession").to_owned();
                        let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                        if !items_fields.is_empty() {
                            selector.push_str("(");
                            selector.push_str(items_fields);
                            selector.push_str(")");
                        }
                        selector
                    });
                    crate::iter::PageItemIter::new(self, "deletedSession")
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are chosen by the caller of this"]
                #[doc = r" method and must implement `Deserialize` and `FieldSelector`. The"]
                #[doc = r" populated fields in the yielded items will be determined by the"]
                #[doc = r" `FieldSelector` implementation."]
                pub fn iter_session<T>(self) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.iter_session_with_fields(fields)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be the default fields populated by"]
                #[doc = r" the server."]
                pub fn iter_session_with_default_fields(
                    self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::Session> {
                    self.iter_session_with_fields(None::<String>)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be all fields available. This should"]
                #[doc = r" primarily be used during developement and debugging as fetching"]
                #[doc = r" all fields can be expensive both in bandwidth and server"]
                #[doc = r" resources."]
                pub fn iter_session_with_all_fields(
                    self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::Session> {
                    self.iter_session_with_fields(Some("*"))
                }
                pub fn iter_session_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: AsRef<str>,
                {
                    self.fields = Some({
                        let mut selector = concat!("nextPageToken,", "session").to_owned();
                        let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                        if !items_fields.is_empty() {
                            selector.push_str("(");
                            selector.push_str(items_fields);
                            selector.push_str(")");
                        }
                        selector
                    });
                    crate::iter::PageItemIter::new(self, "session")
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
                ) -> crate::iter::PageIter<Self, crate::schemas::ListSessionsResponse>
                {
                    self.iter_with_fields(None::<&str>)
                }
                pub fn iter_with_all_fields(
                    self,
                ) -> crate::iter::PageIter<Self, crate::schemas::ListSessionsResponse>
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
                ) -> Result<crate::schemas::ListSessionsResponse, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::ListSessionsResponse, crate::Error> {
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
                    let mut output = "https://www.googleapis.com/".to_owned();
                    output.push_str("fitness/v1/users/");
                    {
                        let var_as_str = &self.user_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/sessions");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("activityType", &self.activity_type)]);
                    let req = req.query(&[("endTime", &self.end_time)]);
                    let req = req.query(&[("includeDeleted", &self.include_deleted)]);
                    let req = req.query(&[("pageToken", &self.page_token)]);
                    let req = req.query(&[("startTime", &self.start_time)]);
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
            #[doc = "Created via [SessionsActions::update()](struct.SessionsActions.html#method.update)"]
            #[derive(Debug, Clone)]
            pub struct UpdateRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::Session,
                user_id: String,
                session_id: String,
                current_time_millis: Option<i64>,
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
            impl<'a> UpdateRequestBuilder<'a> {
                #[doc = "The client's current time in milliseconds since epoch."]
                pub fn current_time_millis(mut self, value: i64) -> Self {
                    self.current_time_millis = Some(value);
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
                ) -> Result<crate::schemas::Session, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Session, crate::Error> {
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
                    let mut output = "https://www.googleapis.com/".to_owned();
                    output.push_str("fitness/v1/users/");
                    {
                        let var_as_str = &self.user_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/sessions/");
                    {
                        let var_as_str = &self.session_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let req = self.reqwest.request(::reqwest::Method::PUT, path);
                    let req = req.query(&[("currentTimeMillis", &self.current_time_millis)]);
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
