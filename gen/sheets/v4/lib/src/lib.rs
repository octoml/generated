#![doc = "# Resources and Methods\n    * [spreadsheets](resources/spreadsheets/struct.SpreadsheetsActions.html)\n      * [*batchUpdate*](resources/spreadsheets/struct.BatchUpdateRequestBuilder.html), [*create*](resources/spreadsheets/struct.CreateRequestBuilder.html), [*get*](resources/spreadsheets/struct.GetRequestBuilder.html), [*getByDataFilter*](resources/spreadsheets/struct.GetByDataFilterRequestBuilder.html)\n      * [developer_metadata](resources/spreadsheets/developer_metadata/struct.DeveloperMetadataActions.html)\n        * [*get*](resources/spreadsheets/developer_metadata/struct.GetRequestBuilder.html), [*search*](resources/spreadsheets/developer_metadata/struct.SearchRequestBuilder.html)\n      * [sheets](resources/spreadsheets/sheets/struct.SheetsActions.html)\n        * [*copyTo*](resources/spreadsheets/sheets/struct.CopyToRequestBuilder.html)\n      * [values](resources/spreadsheets/values/struct.ValuesActions.html)\n        * [*append*](resources/spreadsheets/values/struct.AppendRequestBuilder.html), [*batchClear*](resources/spreadsheets/values/struct.BatchClearRequestBuilder.html), [*batchClearByDataFilter*](resources/spreadsheets/values/struct.BatchClearByDataFilterRequestBuilder.html), [*batchGet*](resources/spreadsheets/values/struct.BatchGetRequestBuilder.html), [*batchGetByDataFilter*](resources/spreadsheets/values/struct.BatchGetByDataFilterRequestBuilder.html), [*batchUpdate*](resources/spreadsheets/values/struct.BatchUpdateRequestBuilder.html), [*batchUpdateByDataFilter*](resources/spreadsheets/values/struct.BatchUpdateByDataFilterRequestBuilder.html), [*clear*](resources/spreadsheets/values/struct.ClearRequestBuilder.html), [*get*](resources/spreadsheets/values/struct.GetRequestBuilder.html), [*update*](resources/spreadsheets/values/struct.UpdateRequestBuilder.html)\n"]
pub mod scopes {
    #[doc = "See, edit, create, and delete all of your Google Drive files\n\n`https://www.googleapis.com/auth/drive`"]
    pub const DRIVE: &str = "https://www.googleapis.com/auth/drive";
    #[doc = "View and manage Google Drive files and folders that you have opened or created with this app\n\n`https://www.googleapis.com/auth/drive.file`"]
    pub const DRIVE_FILE: &str = "https://www.googleapis.com/auth/drive.file";
    #[doc = "See and download all your Google Drive files\n\n`https://www.googleapis.com/auth/drive.readonly`"]
    pub const DRIVE_READONLY: &str = "https://www.googleapis.com/auth/drive.readonly";
    #[doc = "See, edit, create, and delete your spreadsheets in Google Drive\n\n`https://www.googleapis.com/auth/spreadsheets`"]
    pub const SPREADSHEETS: &str = "https://www.googleapis.com/auth/spreadsheets";
    #[doc = "View your Google Spreadsheets\n\n`https://www.googleapis.com/auth/spreadsheets.readonly`"]
    pub const SPREADSHEETS_READONLY: &str = "https://www.googleapis.com/auth/spreadsheets.readonly";
}
pub mod schemas {
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct AddBandingRequest {
        #[doc = "The banded range to add. The bandedRangeId\nfield is optional; if one is not set, an id will be randomly generated. (It\nis an error to specify the ID of a range that already exists.)"]
        #[serde(
            rename = "bandedRange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub banded_range: ::std::option::Option<crate::schemas::BandedRange>,
    }
    impl ::google_field_selector::FieldSelector for AddBandingRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AddBandingRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct AddBandingResponse {
        #[doc = "The banded range that was added."]
        #[serde(
            rename = "bandedRange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub banded_range: ::std::option::Option<crate::schemas::BandedRange>,
    }
    impl ::google_field_selector::FieldSelector for AddBandingResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AddBandingResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct AddChartRequest {
        #[doc = "The chart that should be added to the spreadsheet, including the position\nwhere it should be placed. The chartId\nfield is optional; if one is not set, an id will be randomly generated. (It\nis an error to specify the ID of an embedded object that already exists.)"]
        #[serde(
            rename = "chart",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub chart: ::std::option::Option<crate::schemas::EmbeddedChart>,
    }
    impl ::google_field_selector::FieldSelector for AddChartRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AddChartRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct AddChartResponse {
        #[doc = "The newly added chart."]
        #[serde(
            rename = "chart",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub chart: ::std::option::Option<crate::schemas::EmbeddedChart>,
    }
    impl ::google_field_selector::FieldSelector for AddChartResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AddChartResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct AddConditionalFormatRuleRequest {
        #[doc = "The zero-based index where the rule should be inserted."]
        #[serde(
            rename = "index",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub index: ::std::option::Option<i32>,
        #[doc = "The rule to add."]
        #[serde(
            rename = "rule",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rule: ::std::option::Option<crate::schemas::ConditionalFormatRule>,
    }
    impl ::google_field_selector::FieldSelector for AddConditionalFormatRuleRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AddConditionalFormatRuleRequest {
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
    pub struct AddDimensionGroupRequest {
        #[doc = "The range over which to create a group."]
        #[serde(
            rename = "range",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub range: ::std::option::Option<crate::schemas::DimensionRange>,
    }
    impl ::google_field_selector::FieldSelector for AddDimensionGroupRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AddDimensionGroupRequest {
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
    pub struct AddDimensionGroupResponse {
        #[doc = "All groups of a dimension after adding a group to that dimension."]
        #[serde(
            rename = "dimensionGroups",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dimension_groups: ::std::option::Option<Vec<crate::schemas::DimensionGroup>>,
    }
    impl ::google_field_selector::FieldSelector for AddDimensionGroupResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AddDimensionGroupResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct AddFilterViewRequest {
        #[doc = "The filter to add. The filterViewId\nfield is optional; if one is not set, an id will be randomly generated. (It\nis an error to specify the ID of a filter that already exists.)"]
        #[serde(
            rename = "filter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub filter: ::std::option::Option<crate::schemas::FilterView>,
    }
    impl ::google_field_selector::FieldSelector for AddFilterViewRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AddFilterViewRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct AddFilterViewResponse {
        #[doc = "The newly added filter view."]
        #[serde(
            rename = "filter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub filter: ::std::option::Option<crate::schemas::FilterView>,
    }
    impl ::google_field_selector::FieldSelector for AddFilterViewResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AddFilterViewResponse {
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
    pub struct AddNamedRangeRequest {
        #[doc = "The named range to add. The namedRangeId\nfield is optional; if one is not set, an id will be randomly generated. (It\nis an error to specify the ID of a range that already exists.)"]
        #[serde(
            rename = "namedRange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub named_range: ::std::option::Option<crate::schemas::NamedRange>,
    }
    impl ::google_field_selector::FieldSelector for AddNamedRangeRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AddNamedRangeRequest {
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
    pub struct AddNamedRangeResponse {
        #[doc = "The named range to add."]
        #[serde(
            rename = "namedRange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub named_range: ::std::option::Option<crate::schemas::NamedRange>,
    }
    impl ::google_field_selector::FieldSelector for AddNamedRangeResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AddNamedRangeResponse {
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
    pub struct AddProtectedRangeRequest {
        #[doc = "The protected range to be added. The\nprotectedRangeId field is optional; if\none is not set, an id will be randomly generated. (It is an error to\nspecify the ID of a range that already exists.)"]
        #[serde(
            rename = "protectedRange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub protected_range: ::std::option::Option<crate::schemas::ProtectedRange>,
    }
    impl ::google_field_selector::FieldSelector for AddProtectedRangeRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AddProtectedRangeRequest {
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
    pub struct AddProtectedRangeResponse {
        #[doc = "The newly added protected range."]
        #[serde(
            rename = "protectedRange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub protected_range: ::std::option::Option<crate::schemas::ProtectedRange>,
    }
    impl ::google_field_selector::FieldSelector for AddProtectedRangeResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AddProtectedRangeResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct AddSheetRequest {
        #[doc = "The properties the new sheet should have.\nAll properties are optional.\nThe sheetId field is optional; if one is not\nset, an id will be randomly generated. (It is an error to specify the ID\nof a sheet that already exists.)"]
        #[serde(
            rename = "properties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub properties: ::std::option::Option<crate::schemas::SheetProperties>,
    }
    impl ::google_field_selector::FieldSelector for AddSheetRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AddSheetRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct AddSheetResponse {
        #[doc = "The properties of the newly added sheet."]
        #[serde(
            rename = "properties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub properties: ::std::option::Option<crate::schemas::SheetProperties>,
    }
    impl ::google_field_selector::FieldSelector for AddSheetResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AddSheetResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct AddSlicerRequest {
        #[doc = "The slicer that should be added to the spreadsheet, including\nthe position where it should be placed. The slicerId field is optional; if one is not set, an id\nwill be randomly generated. (It is an error to specify the ID\nof a slicer that already exists.)"]
        #[serde(
            rename = "slicer",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub slicer: ::std::option::Option<crate::schemas::Slicer>,
    }
    impl ::google_field_selector::FieldSelector for AddSlicerRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AddSlicerRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct AddSlicerResponse {
        #[doc = "The newly added slicer."]
        #[serde(
            rename = "slicer",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub slicer: ::std::option::Option<crate::schemas::Slicer>,
    }
    impl ::google_field_selector::FieldSelector for AddSlicerResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AddSlicerResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct AppendCellsRequest {
        #[doc = "The fields of CellData that should be updated.\nAt least one field must be specified.\nThe root is the CellData; 'row.values.' should not be specified.\nA single `\"*\"` can be used as short-hand for listing every field."]
        #[serde(
            rename = "fields",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fields: ::std::option::Option<String>,
        #[doc = "The data to append."]
        #[serde(
            rename = "rows",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rows: ::std::option::Option<Vec<crate::schemas::RowData>>,
        #[doc = "The sheet ID to append the data to."]
        #[serde(
            rename = "sheetId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sheet_id: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for AppendCellsRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AppendCellsRequest {
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
    pub struct AppendDimensionRequest {
        #[doc = "Whether rows or columns should be appended."]
        #[serde(
            rename = "dimension",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dimension: ::std::option::Option<crate::schemas::AppendDimensionRequestDimension>,
        #[doc = "The number of rows or columns to append."]
        #[serde(
            rename = "length",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub length: ::std::option::Option<i32>,
        #[doc = "The sheet to append rows or columns to."]
        #[serde(
            rename = "sheetId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sheet_id: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for AppendDimensionRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AppendDimensionRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum AppendDimensionRequestDimension {
        #[doc = "Operates on the columns of a sheet."]
        Columns,
        #[doc = "The default value, do not use."]
        DimensionUnspecified,
        #[doc = "Operates on the rows of a sheet."]
        Rows,
    }
    impl AppendDimensionRequestDimension {
        pub fn as_str(self) -> &'static str {
            match self {
                AppendDimensionRequestDimension::Columns => "COLUMNS",
                AppendDimensionRequestDimension::DimensionUnspecified => "DIMENSION_UNSPECIFIED",
                AppendDimensionRequestDimension::Rows => "ROWS",
            }
        }
    }
    impl ::std::convert::AsRef<str> for AppendDimensionRequestDimension {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for AppendDimensionRequestDimension {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<AppendDimensionRequestDimension, ()> {
            Ok(match s {
                "COLUMNS" => AppendDimensionRequestDimension::Columns,
                "DIMENSION_UNSPECIFIED" => AppendDimensionRequestDimension::DimensionUnspecified,
                "ROWS" => AppendDimensionRequestDimension::Rows,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for AppendDimensionRequestDimension {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AppendDimensionRequestDimension {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AppendDimensionRequestDimension {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "COLUMNS" => AppendDimensionRequestDimension::Columns,
                "DIMENSION_UNSPECIFIED" => AppendDimensionRequestDimension::DimensionUnspecified,
                "ROWS" => AppendDimensionRequestDimension::Rows,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for AppendDimensionRequestDimension {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AppendDimensionRequestDimension {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct AppendValuesResponse {
        #[doc = "The spreadsheet the updates were applied to."]
        #[serde(
            rename = "spreadsheetId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub spreadsheet_id: ::std::option::Option<String>,
        #[doc = "The range (in A1 notation) of the table that values are being appended to\n(before the values were appended).\nEmpty if no table was found."]
        #[serde(
            rename = "tableRange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub table_range: ::std::option::Option<String>,
        #[doc = "Information about the updates that were applied."]
        #[serde(
            rename = "updates",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub updates: ::std::option::Option<crate::schemas::UpdateValuesResponse>,
    }
    impl ::google_field_selector::FieldSelector for AppendValuesResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AppendValuesResponse {
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
    pub struct AutoFillRequest {
        #[doc = "The range to autofill. This will examine the range and detect\nthe location that has data and automatically fill that data\nin to the rest of the range."]
        #[serde(
            rename = "range",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub range: ::std::option::Option<crate::schemas::GridRange>,
        #[doc = "The source and destination areas to autofill.\nThis explicitly lists the source of the autofill and where to\nextend that data."]
        #[serde(
            rename = "sourceAndDestination",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source_and_destination: ::std::option::Option<crate::schemas::SourceAndDestination>,
        #[doc = "True if we should generate data with the \"alternate\" series.\nThis differs based on the type and amount of source data."]
        #[serde(
            rename = "useAlternateSeries",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub use_alternate_series: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for AutoFillRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AutoFillRequest {
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
    pub struct AutoResizeDimensionsRequest {
        #[doc = "The dimensions to automatically resize."]
        #[serde(
            rename = "dimensions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dimensions: ::std::option::Option<crate::schemas::DimensionRange>,
    }
    impl ::google_field_selector::FieldSelector for AutoResizeDimensionsRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AutoResizeDimensionsRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct BandedRange {
        #[doc = "The id of the banded range."]
        #[serde(
            rename = "bandedRangeId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub banded_range_id: ::std::option::Option<i32>,
        #[doc = "Properties for column bands. These properties are applied on a column-\nby-column basis throughout all the columns in the range. At least one of\nrow_properties or column_properties must be specified."]
        #[serde(
            rename = "columnProperties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub column_properties: ::std::option::Option<crate::schemas::BandingProperties>,
        #[doc = "The range over which these properties are applied."]
        #[serde(
            rename = "range",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub range: ::std::option::Option<crate::schemas::GridRange>,
        #[doc = "Properties for row bands. These properties are applied on a row-by-row\nbasis throughout all the rows in the range. At least one of\nrow_properties or column_properties must be specified."]
        #[serde(
            rename = "rowProperties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub row_properties: ::std::option::Option<crate::schemas::BandingProperties>,
    }
    impl ::google_field_selector::FieldSelector for BandedRange {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BandedRange {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct BandingProperties {
        #[doc = "The first color that is alternating. (Required)"]
        #[serde(
            rename = "firstBandColor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub first_band_color: ::std::option::Option<crate::schemas::Color>,
        #[doc = "The first color that is alternating. (Required)\nIf first_band_color is also set, this field takes precedence."]
        #[serde(
            rename = "firstBandColorStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub first_band_color_style: ::std::option::Option<crate::schemas::ColorStyle>,
        #[doc = "The color of the last row or column. If this field is not set, the last\nrow or column is filled with either first_band_color or\nsecond_band_color, depending on the color of the previous row or\ncolumn."]
        #[serde(
            rename = "footerColor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub footer_color: ::std::option::Option<crate::schemas::Color>,
        #[doc = "The color of the last row or column. If this field is not set, the last\nrow or column is filled with either first_band_color or\nsecond_band_color, depending on the color of the previous row or\ncolumn.\nIf footer_color is also set, this field takes precedence."]
        #[serde(
            rename = "footerColorStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub footer_color_style: ::std::option::Option<crate::schemas::ColorStyle>,
        #[doc = "The color of the first row or column. If this field is set, the first row\nor column is filled with this color and the colors alternate between\nfirst_band_color and second_band_color starting from the second\nrow or column. Otherwise, the first row or column is filled with\nfirst_band_color and the colors proceed to alternate as they normally\nwould."]
        #[serde(
            rename = "headerColor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub header_color: ::std::option::Option<crate::schemas::Color>,
        #[doc = "The color of the first row or column. If this field is set, the first row\nor column is filled with this color and the colors alternate between\nfirst_band_color and second_band_color starting from the second\nrow or column. Otherwise, the first row or column is filled with\nfirst_band_color and the colors proceed to alternate as they normally\nwould. If header_color is also set, this field takes precedence."]
        #[serde(
            rename = "headerColorStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub header_color_style: ::std::option::Option<crate::schemas::ColorStyle>,
        #[doc = "The second color that is alternating. (Required)"]
        #[serde(
            rename = "secondBandColor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub second_band_color: ::std::option::Option<crate::schemas::Color>,
        #[doc = "The second color that is alternating. (Required)\nIf second_band_color is also set, this field takes precedence."]
        #[serde(
            rename = "secondBandColorStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub second_band_color_style: ::std::option::Option<crate::schemas::ColorStyle>,
    }
    impl ::google_field_selector::FieldSelector for BandingProperties {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BandingProperties {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct BaselineValueFormat {
        #[doc = "The comparison type of key value with baseline value."]
        #[serde(
            rename = "comparisonType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub comparison_type:
            ::std::option::Option<crate::schemas::BaselineValueFormatComparisonType>,
        #[doc = "Description which is appended after the baseline value.\nThis field is optional."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "Color to be used, in case baseline value represents a negative change for\nkey value. This field is optional."]
        #[serde(
            rename = "negativeColor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub negative_color: ::std::option::Option<crate::schemas::Color>,
        #[doc = "Color to be used, in case baseline value represents a negative change for\nkey value. This field is optional.\nIf negative_color is also set, this field takes precedence."]
        #[serde(
            rename = "negativeColorStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub negative_color_style: ::std::option::Option<crate::schemas::ColorStyle>,
        #[doc = "Specifies the horizontal text positioning of baseline value.\nThis field is optional. If not specified, default positioning is used."]
        #[serde(
            rename = "position",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub position: ::std::option::Option<crate::schemas::TextPosition>,
        #[doc = "Color to be used, in case baseline value represents a positive change for\nkey value. This field is optional."]
        #[serde(
            rename = "positiveColor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub positive_color: ::std::option::Option<crate::schemas::Color>,
        #[doc = "Color to be used, in case baseline value represents a positive change for\nkey value. This field is optional.\nIf positive_color is also set, this field takes precedence."]
        #[serde(
            rename = "positiveColorStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub positive_color_style: ::std::option::Option<crate::schemas::ColorStyle>,
        #[doc = "Text formatting options for baseline value."]
        #[serde(
            rename = "textFormat",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text_format: ::std::option::Option<crate::schemas::TextFormat>,
    }
    impl ::google_field_selector::FieldSelector for BaselineValueFormat {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BaselineValueFormat {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum BaselineValueFormatComparisonType {
        #[doc = "Use absolute difference between key and baseline value."]
        AbsoluteDifference,
        #[doc = "Default value, do not use."]
        ComparisonTypeUndefined,
        #[doc = "Use percentage difference between key and baseline value."]
        PercentageDifference,
    }
    impl BaselineValueFormatComparisonType {
        pub fn as_str(self) -> &'static str {
            match self {
                BaselineValueFormatComparisonType::AbsoluteDifference => "ABSOLUTE_DIFFERENCE",
                BaselineValueFormatComparisonType::ComparisonTypeUndefined => {
                    "COMPARISON_TYPE_UNDEFINED"
                }
                BaselineValueFormatComparisonType::PercentageDifference => "PERCENTAGE_DIFFERENCE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for BaselineValueFormatComparisonType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for BaselineValueFormatComparisonType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<BaselineValueFormatComparisonType, ()> {
            Ok(match s {
                "ABSOLUTE_DIFFERENCE" => BaselineValueFormatComparisonType::AbsoluteDifference,
                "COMPARISON_TYPE_UNDEFINED" => {
                    BaselineValueFormatComparisonType::ComparisonTypeUndefined
                }
                "PERCENTAGE_DIFFERENCE" => BaselineValueFormatComparisonType::PercentageDifference,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for BaselineValueFormatComparisonType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for BaselineValueFormatComparisonType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for BaselineValueFormatComparisonType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ABSOLUTE_DIFFERENCE" => BaselineValueFormatComparisonType::AbsoluteDifference,
                "COMPARISON_TYPE_UNDEFINED" => {
                    BaselineValueFormatComparisonType::ComparisonTypeUndefined
                }
                "PERCENTAGE_DIFFERENCE" => BaselineValueFormatComparisonType::PercentageDifference,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for BaselineValueFormatComparisonType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BaselineValueFormatComparisonType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct BasicChartAxis {
        #[doc = "The format of the title.\nOnly valid if the axis is not associated with the domain."]
        #[serde(
            rename = "format",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub format: ::std::option::Option<crate::schemas::TextFormat>,
        #[doc = "The position of this axis."]
        #[serde(
            rename = "position",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub position: ::std::option::Option<crate::schemas::BasicChartAxisPosition>,
        #[doc = "The title of this axis. If set, this overrides any title inferred\nfrom headers of the data."]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
        #[doc = "The axis title text position."]
        #[serde(
            rename = "titleTextPosition",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title_text_position: ::std::option::Option<crate::schemas::TextPosition>,
        #[doc = "The view window options for this axis."]
        #[serde(
            rename = "viewWindowOptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub view_window_options: ::std::option::Option<crate::schemas::ChartAxisViewWindowOptions>,
    }
    impl ::google_field_selector::FieldSelector for BasicChartAxis {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BasicChartAxis {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum BasicChartAxisPosition {
        #[doc = "Default value, do not use."]
        BasicChartAxisPositionUnspecified,
        #[doc = "The axis rendered at the bottom of a chart.\nFor most charts, this is the standard major axis.\nFor bar charts, this is a minor axis."]
        BottomAxis,
        #[doc = "The axis rendered at the left of a chart.\nFor most charts, this is a minor axis.\nFor bar charts, this is the standard major axis."]
        LeftAxis,
        #[doc = "The axis rendered at the right of a chart.\nFor most charts, this is a minor axis.\nFor bar charts, this is an unusual major axis."]
        RightAxis,
    }
    impl BasicChartAxisPosition {
        pub fn as_str(self) -> &'static str {
            match self {
                BasicChartAxisPosition::BasicChartAxisPositionUnspecified => {
                    "BASIC_CHART_AXIS_POSITION_UNSPECIFIED"
                }
                BasicChartAxisPosition::BottomAxis => "BOTTOM_AXIS",
                BasicChartAxisPosition::LeftAxis => "LEFT_AXIS",
                BasicChartAxisPosition::RightAxis => "RIGHT_AXIS",
            }
        }
    }
    impl ::std::convert::AsRef<str> for BasicChartAxisPosition {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for BasicChartAxisPosition {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<BasicChartAxisPosition, ()> {
            Ok(match s {
                "BASIC_CHART_AXIS_POSITION_UNSPECIFIED" => {
                    BasicChartAxisPosition::BasicChartAxisPositionUnspecified
                }
                "BOTTOM_AXIS" => BasicChartAxisPosition::BottomAxis,
                "LEFT_AXIS" => BasicChartAxisPosition::LeftAxis,
                "RIGHT_AXIS" => BasicChartAxisPosition::RightAxis,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for BasicChartAxisPosition {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for BasicChartAxisPosition {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for BasicChartAxisPosition {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BASIC_CHART_AXIS_POSITION_UNSPECIFIED" => {
                    BasicChartAxisPosition::BasicChartAxisPositionUnspecified
                }
                "BOTTOM_AXIS" => BasicChartAxisPosition::BottomAxis,
                "LEFT_AXIS" => BasicChartAxisPosition::LeftAxis,
                "RIGHT_AXIS" => BasicChartAxisPosition::RightAxis,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for BasicChartAxisPosition {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BasicChartAxisPosition {
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
    pub struct BasicChartDomain {
        #[doc = "The data of the domain. For example, if charting stock prices over time,\nthis is the data representing the dates."]
        #[serde(
            rename = "domain",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub domain: ::std::option::Option<crate::schemas::ChartData>,
        #[doc = "True to reverse the order of the domain values (horizontal axis)."]
        #[serde(
            rename = "reversed",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub reversed: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for BasicChartDomain {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BasicChartDomain {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct BasicChartSeries {
        #[doc = "The color for elements (such as bars, lines, and points) associated with\nthis series.  If empty, a default color is used."]
        #[serde(
            rename = "color",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub color: ::std::option::Option<crate::schemas::Color>,
        #[doc = "The color for elements (such as bars, lines, and points) associated with\nthis series.  If empty, a default color is used.\nIf color is also set, this field takes precedence."]
        #[serde(
            rename = "colorStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub color_style: ::std::option::Option<crate::schemas::ColorStyle>,
        #[doc = "The line style of this series. Valid only if the\nchartType is AREA,\nLINE, or SCATTER.\nCOMBO charts are also supported if the\nseries chart type is\nAREA or LINE."]
        #[serde(
            rename = "lineStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub line_style: ::std::option::Option<crate::schemas::LineStyle>,
        #[doc = "The type of this series. Valid only if the\nchartType is\nCOMBO.\nDifferent types will change the way the series is visualized.\nOnly LINE, AREA,\nand COLUMN are supported."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::BasicChartSeriesType>,
        #[doc = "The data being visualized in this chart series."]
        #[serde(
            rename = "series",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub series: ::std::option::Option<crate::schemas::ChartData>,
        #[doc = "The minor axis that will specify the range of values for this series.\nFor example, if charting stocks over time, the \"Volume\" series\nmay want to be pinned to the right with the prices pinned to the left,\nbecause the scale of trading volume is different than the scale of\nprices.\nIt is an error to specify an axis that isn't a valid minor axis\nfor the chart's type."]
        #[serde(
            rename = "targetAxis",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub target_axis: ::std::option::Option<crate::schemas::BasicChartSeriesTargetAxis>,
    }
    impl ::google_field_selector::FieldSelector for BasicChartSeries {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BasicChartSeries {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum BasicChartSeriesType {
        #[doc = "An <a href=\"/chart/interactive/docs/gallery/areachart\">area chart</a>."]
        Area,
        #[doc = "A <a href=\"/chart/interactive/docs/gallery/barchart\">bar chart</a>."]
        Bar,
        #[doc = "Default value, do not use."]
        BasicChartTypeUnspecified,
        #[doc = "A <a href=\"/chart/interactive/docs/gallery/columnchart\">column chart</a>."]
        Column,
        #[doc = "A <a href=\"/chart/interactive/docs/gallery/combochart\">combo chart</a>."]
        Combo,
        #[doc = "A <a href=\"/chart/interactive/docs/gallery/linechart\">line chart</a>."]
        Line,
        #[doc = "A <a href=\"/chart/interactive/docs/gallery/scatterchart\">scatter\nchart</a>."]
        Scatter,
        #[doc = "A <a href=\"/chart/interactive/docs/gallery/steppedareachart\">stepped area\nchart</a>."]
        SteppedArea,
    }
    impl BasicChartSeriesType {
        pub fn as_str(self) -> &'static str {
            match self {
                BasicChartSeriesType::Area => "AREA",
                BasicChartSeriesType::Bar => "BAR",
                BasicChartSeriesType::BasicChartTypeUnspecified => "BASIC_CHART_TYPE_UNSPECIFIED",
                BasicChartSeriesType::Column => "COLUMN",
                BasicChartSeriesType::Combo => "COMBO",
                BasicChartSeriesType::Line => "LINE",
                BasicChartSeriesType::Scatter => "SCATTER",
                BasicChartSeriesType::SteppedArea => "STEPPED_AREA",
            }
        }
    }
    impl ::std::convert::AsRef<str> for BasicChartSeriesType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for BasicChartSeriesType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<BasicChartSeriesType, ()> {
            Ok(match s {
                "AREA" => BasicChartSeriesType::Area,
                "BAR" => BasicChartSeriesType::Bar,
                "BASIC_CHART_TYPE_UNSPECIFIED" => BasicChartSeriesType::BasicChartTypeUnspecified,
                "COLUMN" => BasicChartSeriesType::Column,
                "COMBO" => BasicChartSeriesType::Combo,
                "LINE" => BasicChartSeriesType::Line,
                "SCATTER" => BasicChartSeriesType::Scatter,
                "STEPPED_AREA" => BasicChartSeriesType::SteppedArea,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for BasicChartSeriesType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for BasicChartSeriesType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for BasicChartSeriesType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "AREA" => BasicChartSeriesType::Area,
                "BAR" => BasicChartSeriesType::Bar,
                "BASIC_CHART_TYPE_UNSPECIFIED" => BasicChartSeriesType::BasicChartTypeUnspecified,
                "COLUMN" => BasicChartSeriesType::Column,
                "COMBO" => BasicChartSeriesType::Combo,
                "LINE" => BasicChartSeriesType::Line,
                "SCATTER" => BasicChartSeriesType::Scatter,
                "STEPPED_AREA" => BasicChartSeriesType::SteppedArea,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for BasicChartSeriesType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BasicChartSeriesType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum BasicChartSeriesTargetAxis {
        #[doc = "Default value, do not use."]
        BasicChartAxisPositionUnspecified,
        #[doc = "The axis rendered at the bottom of a chart.\nFor most charts, this is the standard major axis.\nFor bar charts, this is a minor axis."]
        BottomAxis,
        #[doc = "The axis rendered at the left of a chart.\nFor most charts, this is a minor axis.\nFor bar charts, this is the standard major axis."]
        LeftAxis,
        #[doc = "The axis rendered at the right of a chart.\nFor most charts, this is a minor axis.\nFor bar charts, this is an unusual major axis."]
        RightAxis,
    }
    impl BasicChartSeriesTargetAxis {
        pub fn as_str(self) -> &'static str {
            match self {
                BasicChartSeriesTargetAxis::BasicChartAxisPositionUnspecified => {
                    "BASIC_CHART_AXIS_POSITION_UNSPECIFIED"
                }
                BasicChartSeriesTargetAxis::BottomAxis => "BOTTOM_AXIS",
                BasicChartSeriesTargetAxis::LeftAxis => "LEFT_AXIS",
                BasicChartSeriesTargetAxis::RightAxis => "RIGHT_AXIS",
            }
        }
    }
    impl ::std::convert::AsRef<str> for BasicChartSeriesTargetAxis {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for BasicChartSeriesTargetAxis {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<BasicChartSeriesTargetAxis, ()> {
            Ok(match s {
                "BASIC_CHART_AXIS_POSITION_UNSPECIFIED" => {
                    BasicChartSeriesTargetAxis::BasicChartAxisPositionUnspecified
                }
                "BOTTOM_AXIS" => BasicChartSeriesTargetAxis::BottomAxis,
                "LEFT_AXIS" => BasicChartSeriesTargetAxis::LeftAxis,
                "RIGHT_AXIS" => BasicChartSeriesTargetAxis::RightAxis,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for BasicChartSeriesTargetAxis {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for BasicChartSeriesTargetAxis {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for BasicChartSeriesTargetAxis {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BASIC_CHART_AXIS_POSITION_UNSPECIFIED" => {
                    BasicChartSeriesTargetAxis::BasicChartAxisPositionUnspecified
                }
                "BOTTOM_AXIS" => BasicChartSeriesTargetAxis::BottomAxis,
                "LEFT_AXIS" => BasicChartSeriesTargetAxis::LeftAxis,
                "RIGHT_AXIS" => BasicChartSeriesTargetAxis::RightAxis,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for BasicChartSeriesTargetAxis {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BasicChartSeriesTargetAxis {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct BasicChartSpec {
        #[doc = "The axis on the chart."]
        #[serde(
            rename = "axis",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub axis: ::std::option::Option<Vec<crate::schemas::BasicChartAxis>>,
        #[doc = "The type of the chart."]
        #[serde(
            rename = "chartType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub chart_type: ::std::option::Option<crate::schemas::BasicChartSpecChartType>,
        #[doc = "The behavior of tooltips and data highlighting when hovering on data and\nchart area."]
        #[serde(
            rename = "compareMode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub compare_mode: ::std::option::Option<crate::schemas::BasicChartSpecCompareMode>,
        #[doc = "The domain of data this is charting.\nOnly a single domain is supported."]
        #[serde(
            rename = "domains",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub domains: ::std::option::Option<Vec<crate::schemas::BasicChartDomain>>,
        #[doc = "The number of rows or columns in the data that are \"headers\".\nIf not set, Google Sheets will guess how many rows are headers based\non the data.\n\n(Note that BasicChartAxis.title may override the axis title\ninferred from the header values.)"]
        #[serde(
            rename = "headerCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub header_count: ::std::option::Option<i32>,
        #[doc = "If some values in a series are missing, gaps may appear in the chart (e.g,\nsegments of lines in a line chart will be missing).  To eliminate these\ngaps set this to true.\nApplies to Line, Area, and Combo charts."]
        #[serde(
            rename = "interpolateNulls",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub interpolate_nulls: ::std::option::Option<bool>,
        #[doc = "The position of the chart legend."]
        #[serde(
            rename = "legendPosition",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub legend_position: ::std::option::Option<crate::schemas::BasicChartSpecLegendPosition>,
        #[doc = "Gets whether all lines should be rendered smooth or straight by default.\nApplies to Line charts."]
        #[serde(
            rename = "lineSmoothing",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub line_smoothing: ::std::option::Option<bool>,
        #[doc = "The data this chart is visualizing."]
        #[serde(
            rename = "series",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub series: ::std::option::Option<Vec<crate::schemas::BasicChartSeries>>,
        #[doc = "The stacked type for charts that support vertical stacking.\nApplies to Area, Bar, Column, Combo, and Stepped Area charts."]
        #[serde(
            rename = "stackedType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub stacked_type: ::std::option::Option<crate::schemas::BasicChartSpecStackedType>,
        #[doc = "True to make the chart 3D.\nApplies to Bar and Column charts."]
        #[serde(
            rename = "threeDimensional",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub three_dimensional: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for BasicChartSpec {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BasicChartSpec {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum BasicChartSpecChartType {
        #[doc = "An <a href=\"/chart/interactive/docs/gallery/areachart\">area chart</a>."]
        Area,
        #[doc = "A <a href=\"/chart/interactive/docs/gallery/barchart\">bar chart</a>."]
        Bar,
        #[doc = "Default value, do not use."]
        BasicChartTypeUnspecified,
        #[doc = "A <a href=\"/chart/interactive/docs/gallery/columnchart\">column chart</a>."]
        Column,
        #[doc = "A <a href=\"/chart/interactive/docs/gallery/combochart\">combo chart</a>."]
        Combo,
        #[doc = "A <a href=\"/chart/interactive/docs/gallery/linechart\">line chart</a>."]
        Line,
        #[doc = "A <a href=\"/chart/interactive/docs/gallery/scatterchart\">scatter\nchart</a>."]
        Scatter,
        #[doc = "A <a href=\"/chart/interactive/docs/gallery/steppedareachart\">stepped area\nchart</a>."]
        SteppedArea,
    }
    impl BasicChartSpecChartType {
        pub fn as_str(self) -> &'static str {
            match self {
                BasicChartSpecChartType::Area => "AREA",
                BasicChartSpecChartType::Bar => "BAR",
                BasicChartSpecChartType::BasicChartTypeUnspecified => {
                    "BASIC_CHART_TYPE_UNSPECIFIED"
                }
                BasicChartSpecChartType::Column => "COLUMN",
                BasicChartSpecChartType::Combo => "COMBO",
                BasicChartSpecChartType::Line => "LINE",
                BasicChartSpecChartType::Scatter => "SCATTER",
                BasicChartSpecChartType::SteppedArea => "STEPPED_AREA",
            }
        }
    }
    impl ::std::convert::AsRef<str> for BasicChartSpecChartType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for BasicChartSpecChartType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<BasicChartSpecChartType, ()> {
            Ok(match s {
                "AREA" => BasicChartSpecChartType::Area,
                "BAR" => BasicChartSpecChartType::Bar,
                "BASIC_CHART_TYPE_UNSPECIFIED" => {
                    BasicChartSpecChartType::BasicChartTypeUnspecified
                }
                "COLUMN" => BasicChartSpecChartType::Column,
                "COMBO" => BasicChartSpecChartType::Combo,
                "LINE" => BasicChartSpecChartType::Line,
                "SCATTER" => BasicChartSpecChartType::Scatter,
                "STEPPED_AREA" => BasicChartSpecChartType::SteppedArea,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for BasicChartSpecChartType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for BasicChartSpecChartType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for BasicChartSpecChartType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "AREA" => BasicChartSpecChartType::Area,
                "BAR" => BasicChartSpecChartType::Bar,
                "BASIC_CHART_TYPE_UNSPECIFIED" => {
                    BasicChartSpecChartType::BasicChartTypeUnspecified
                }
                "COLUMN" => BasicChartSpecChartType::Column,
                "COMBO" => BasicChartSpecChartType::Combo,
                "LINE" => BasicChartSpecChartType::Line,
                "SCATTER" => BasicChartSpecChartType::Scatter,
                "STEPPED_AREA" => BasicChartSpecChartType::SteppedArea,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for BasicChartSpecChartType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BasicChartSpecChartType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum BasicChartSpecCompareMode {
        #[doc = "Default value, do not use."]
        BasicChartCompareModeUnspecified,
        #[doc = "All data elements with the same category (e.g., domain value) are\nhighlighted and shown in the tooltip."]
        Category,
        #[doc = "Only the focused data element is highlighted and shown in the tooltip."]
        Datum,
    }
    impl BasicChartSpecCompareMode {
        pub fn as_str(self) -> &'static str {
            match self {
                BasicChartSpecCompareMode::BasicChartCompareModeUnspecified => {
                    "BASIC_CHART_COMPARE_MODE_UNSPECIFIED"
                }
                BasicChartSpecCompareMode::Category => "CATEGORY",
                BasicChartSpecCompareMode::Datum => "DATUM",
            }
        }
    }
    impl ::std::convert::AsRef<str> for BasicChartSpecCompareMode {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for BasicChartSpecCompareMode {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<BasicChartSpecCompareMode, ()> {
            Ok(match s {
                "BASIC_CHART_COMPARE_MODE_UNSPECIFIED" => {
                    BasicChartSpecCompareMode::BasicChartCompareModeUnspecified
                }
                "CATEGORY" => BasicChartSpecCompareMode::Category,
                "DATUM" => BasicChartSpecCompareMode::Datum,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for BasicChartSpecCompareMode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for BasicChartSpecCompareMode {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for BasicChartSpecCompareMode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BASIC_CHART_COMPARE_MODE_UNSPECIFIED" => {
                    BasicChartSpecCompareMode::BasicChartCompareModeUnspecified
                }
                "CATEGORY" => BasicChartSpecCompareMode::Category,
                "DATUM" => BasicChartSpecCompareMode::Datum,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for BasicChartSpecCompareMode {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BasicChartSpecCompareMode {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum BasicChartSpecLegendPosition {
        #[doc = "Default value, do not use."]
        BasicChartLegendPositionUnspecified,
        #[doc = "The legend is rendered on the bottom of the chart."]
        BottomLegend,
        #[doc = "The legend is rendered on the left of the chart."]
        LeftLegend,
        #[doc = "No legend is rendered."]
        NoLegend,
        #[doc = "The legend is rendered on the right of the chart."]
        RightLegend,
        #[doc = "The legend is rendered on the top of the chart."]
        TopLegend,
    }
    impl BasicChartSpecLegendPosition {
        pub fn as_str(self) -> &'static str {
            match self {
                BasicChartSpecLegendPosition::BasicChartLegendPositionUnspecified => {
                    "BASIC_CHART_LEGEND_POSITION_UNSPECIFIED"
                }
                BasicChartSpecLegendPosition::BottomLegend => "BOTTOM_LEGEND",
                BasicChartSpecLegendPosition::LeftLegend => "LEFT_LEGEND",
                BasicChartSpecLegendPosition::NoLegend => "NO_LEGEND",
                BasicChartSpecLegendPosition::RightLegend => "RIGHT_LEGEND",
                BasicChartSpecLegendPosition::TopLegend => "TOP_LEGEND",
            }
        }
    }
    impl ::std::convert::AsRef<str> for BasicChartSpecLegendPosition {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for BasicChartSpecLegendPosition {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<BasicChartSpecLegendPosition, ()> {
            Ok(match s {
                "BASIC_CHART_LEGEND_POSITION_UNSPECIFIED" => {
                    BasicChartSpecLegendPosition::BasicChartLegendPositionUnspecified
                }
                "BOTTOM_LEGEND" => BasicChartSpecLegendPosition::BottomLegend,
                "LEFT_LEGEND" => BasicChartSpecLegendPosition::LeftLegend,
                "NO_LEGEND" => BasicChartSpecLegendPosition::NoLegend,
                "RIGHT_LEGEND" => BasicChartSpecLegendPosition::RightLegend,
                "TOP_LEGEND" => BasicChartSpecLegendPosition::TopLegend,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for BasicChartSpecLegendPosition {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for BasicChartSpecLegendPosition {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for BasicChartSpecLegendPosition {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BASIC_CHART_LEGEND_POSITION_UNSPECIFIED" => {
                    BasicChartSpecLegendPosition::BasicChartLegendPositionUnspecified
                }
                "BOTTOM_LEGEND" => BasicChartSpecLegendPosition::BottomLegend,
                "LEFT_LEGEND" => BasicChartSpecLegendPosition::LeftLegend,
                "NO_LEGEND" => BasicChartSpecLegendPosition::NoLegend,
                "RIGHT_LEGEND" => BasicChartSpecLegendPosition::RightLegend,
                "TOP_LEGEND" => BasicChartSpecLegendPosition::TopLegend,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for BasicChartSpecLegendPosition {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BasicChartSpecLegendPosition {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum BasicChartSpecStackedType {
        #[doc = "Default value, do not use."]
        BasicChartStackedTypeUnspecified,
        #[doc = "Series are not stacked."]
        NotStacked,
        #[doc = "Vertical stacks are stretched to reach the top of the chart, with\nvalues laid out as percentages of each other."]
        PercentStacked,
        #[doc = "Series values are stacked, each value is rendered vertically beginning\nfrom the top of the value below it."]
        Stacked,
    }
    impl BasicChartSpecStackedType {
        pub fn as_str(self) -> &'static str {
            match self {
                BasicChartSpecStackedType::BasicChartStackedTypeUnspecified => {
                    "BASIC_CHART_STACKED_TYPE_UNSPECIFIED"
                }
                BasicChartSpecStackedType::NotStacked => "NOT_STACKED",
                BasicChartSpecStackedType::PercentStacked => "PERCENT_STACKED",
                BasicChartSpecStackedType::Stacked => "STACKED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for BasicChartSpecStackedType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for BasicChartSpecStackedType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<BasicChartSpecStackedType, ()> {
            Ok(match s {
                "BASIC_CHART_STACKED_TYPE_UNSPECIFIED" => {
                    BasicChartSpecStackedType::BasicChartStackedTypeUnspecified
                }
                "NOT_STACKED" => BasicChartSpecStackedType::NotStacked,
                "PERCENT_STACKED" => BasicChartSpecStackedType::PercentStacked,
                "STACKED" => BasicChartSpecStackedType::Stacked,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for BasicChartSpecStackedType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for BasicChartSpecStackedType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for BasicChartSpecStackedType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BASIC_CHART_STACKED_TYPE_UNSPECIFIED" => {
                    BasicChartSpecStackedType::BasicChartStackedTypeUnspecified
                }
                "NOT_STACKED" => BasicChartSpecStackedType::NotStacked,
                "PERCENT_STACKED" => BasicChartSpecStackedType::PercentStacked,
                "STACKED" => BasicChartSpecStackedType::Stacked,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for BasicChartSpecStackedType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BasicChartSpecStackedType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct BasicFilter {
        #[doc = "The criteria for showing/hiding values per column.\nThe map's key is the column index, and the value is the criteria for\nthat column."]
        #[serde(
            rename = "criteria",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub criteria: ::std::option::Option<
            ::std::collections::BTreeMap<String, crate::schemas::FilterCriteria>,
        >,
        #[doc = "The range the filter covers."]
        #[serde(
            rename = "range",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub range: ::std::option::Option<crate::schemas::GridRange>,
        #[doc = "The sort order per column. Later specifications are used when values\nare equal in the earlier specifications."]
        #[serde(
            rename = "sortSpecs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sort_specs: ::std::option::Option<Vec<crate::schemas::SortSpec>>,
    }
    impl ::google_field_selector::FieldSelector for BasicFilter {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BasicFilter {
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
    pub struct BatchClearValuesByDataFilterRequest {
        #[doc = "The DataFilters used to determine which ranges to clear."]
        #[serde(
            rename = "dataFilters",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub data_filters: ::std::option::Option<Vec<crate::schemas::DataFilter>>,
    }
    impl ::google_field_selector::FieldSelector for BatchClearValuesByDataFilterRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BatchClearValuesByDataFilterRequest {
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
    pub struct BatchClearValuesByDataFilterResponse {
        #[doc = "The ranges that were cleared, in A1 notation. If the requests are for an\nunbounded range or a ranger larger than the bounds of the sheet, this is\nthe actual ranges that were cleared, bounded to the sheet's limits."]
        #[serde(
            rename = "clearedRanges",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cleared_ranges: ::std::option::Option<Vec<String>>,
        #[doc = "The spreadsheet the updates were applied to."]
        #[serde(
            rename = "spreadsheetId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub spreadsheet_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for BatchClearValuesByDataFilterResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BatchClearValuesByDataFilterResponse {
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
    pub struct BatchClearValuesRequest {
        #[doc = "The ranges to clear, in A1 notation."]
        #[serde(
            rename = "ranges",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ranges: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for BatchClearValuesRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BatchClearValuesRequest {
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
    pub struct BatchClearValuesResponse {
        #[doc = "The ranges that were cleared, in A1 notation. If the requests are for an\nunbounded range or a ranger larger than the bounds of the sheet, this is\nthe actual ranges that were cleared, bounded to the sheet's limits."]
        #[serde(
            rename = "clearedRanges",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cleared_ranges: ::std::option::Option<Vec<String>>,
        #[doc = "The spreadsheet the updates were applied to."]
        #[serde(
            rename = "spreadsheetId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub spreadsheet_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for BatchClearValuesResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BatchClearValuesResponse {
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
    pub struct BatchGetValuesByDataFilterRequest {
        #[doc = "The data filters used to match the ranges of values to retrieve. Ranges\nthat match any of the specified data filters are included in the response."]
        #[serde(
            rename = "dataFilters",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub data_filters: ::std::option::Option<Vec<crate::schemas::DataFilter>>,
        #[doc = "How dates, times, and durations should be represented in the output.\nThis is ignored if value_render_option is\nFORMATTED_VALUE.\nThe default dateTime render option is [DateTimeRenderOption.SERIAL_NUMBER]."]
        #[serde(
            rename = "dateTimeRenderOption",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub date_time_render_option: ::std::option::Option<
            crate::schemas::BatchGetValuesByDataFilterRequestDateTimeRenderOption,
        >,
        #[doc = "The major dimension that results should use.\n\nFor example, if the spreadsheet data is: `A1=1,B1=2,A2=3,B2=4`,\nthen a request that selects that range and sets `majorDimension=ROWS`\nreturns `[[1,2],[3,4]]`, whereas a request that sets\n`majorDimension=COLUMNS` returns `[[1,3],[2,4]]`."]
        #[serde(
            rename = "majorDimension",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub major_dimension:
            ::std::option::Option<crate::schemas::BatchGetValuesByDataFilterRequestMajorDimension>,
        #[doc = "How values should be represented in the output.\nThe default render option is ValueRenderOption.FORMATTED_VALUE."]
        #[serde(
            rename = "valueRenderOption",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value_render_option: ::std::option::Option<
            crate::schemas::BatchGetValuesByDataFilterRequestValueRenderOption,
        >,
    }
    impl ::google_field_selector::FieldSelector for BatchGetValuesByDataFilterRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BatchGetValuesByDataFilterRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum BatchGetValuesByDataFilterRequestDateTimeRenderOption {
        #[doc = "Instructs date, time, datetime, and duration fields to be output\nas strings in their given number format (which is dependent\non the spreadsheet locale)."]
        FormattedString,
        #[doc = "Instructs date, time, datetime, and duration fields to be output\nas doubles in \"serial number\" format, as popularized by Lotus 1-2-3.\nThe whole number portion of the value (left of the decimal) counts\nthe days since December 30th 1899. The fractional portion (right of\nthe decimal) counts the time as a fraction of the day. For example,\nJanuary 1st 1900 at noon would be 2.5, 2 because it's 2 days after\nDecember 30st 1899, and .5 because noon is half a day.  February 1st\n1900 at 3pm would be 33.625. This correctly treats the year 1900 as\nnot a leap year."]
        SerialNumber,
    }
    impl BatchGetValuesByDataFilterRequestDateTimeRenderOption {
        pub fn as_str(self) -> &'static str {
            match self {
                BatchGetValuesByDataFilterRequestDateTimeRenderOption::FormattedString => {
                    "FORMATTED_STRING"
                }
                BatchGetValuesByDataFilterRequestDateTimeRenderOption::SerialNumber => {
                    "SERIAL_NUMBER"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for BatchGetValuesByDataFilterRequestDateTimeRenderOption {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for BatchGetValuesByDataFilterRequestDateTimeRenderOption {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<BatchGetValuesByDataFilterRequestDateTimeRenderOption, ()>
        {
            Ok(match s {
                "FORMATTED_STRING" => {
                    BatchGetValuesByDataFilterRequestDateTimeRenderOption::FormattedString
                }
                "SERIAL_NUMBER" => {
                    BatchGetValuesByDataFilterRequestDateTimeRenderOption::SerialNumber
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for BatchGetValuesByDataFilterRequestDateTimeRenderOption {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for BatchGetValuesByDataFilterRequestDateTimeRenderOption {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for BatchGetValuesByDataFilterRequestDateTimeRenderOption {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "FORMATTED_STRING" => {
                    BatchGetValuesByDataFilterRequestDateTimeRenderOption::FormattedString
                }
                "SERIAL_NUMBER" => {
                    BatchGetValuesByDataFilterRequestDateTimeRenderOption::SerialNumber
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
    impl ::google_field_selector::FieldSelector
        for BatchGetValuesByDataFilterRequestDateTimeRenderOption
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for BatchGetValuesByDataFilterRequestDateTimeRenderOption
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum BatchGetValuesByDataFilterRequestMajorDimension {
        #[doc = "Operates on the columns of a sheet."]
        Columns,
        #[doc = "The default value, do not use."]
        DimensionUnspecified,
        #[doc = "Operates on the rows of a sheet."]
        Rows,
    }
    impl BatchGetValuesByDataFilterRequestMajorDimension {
        pub fn as_str(self) -> &'static str {
            match self {
                BatchGetValuesByDataFilterRequestMajorDimension::Columns => "COLUMNS",
                BatchGetValuesByDataFilterRequestMajorDimension::DimensionUnspecified => {
                    "DIMENSION_UNSPECIFIED"
                }
                BatchGetValuesByDataFilterRequestMajorDimension::Rows => "ROWS",
            }
        }
    }
    impl ::std::convert::AsRef<str> for BatchGetValuesByDataFilterRequestMajorDimension {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for BatchGetValuesByDataFilterRequestMajorDimension {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<BatchGetValuesByDataFilterRequestMajorDimension, ()> {
            Ok(match s {
                "COLUMNS" => BatchGetValuesByDataFilterRequestMajorDimension::Columns,
                "DIMENSION_UNSPECIFIED" => {
                    BatchGetValuesByDataFilterRequestMajorDimension::DimensionUnspecified
                }
                "ROWS" => BatchGetValuesByDataFilterRequestMajorDimension::Rows,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for BatchGetValuesByDataFilterRequestMajorDimension {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for BatchGetValuesByDataFilterRequestMajorDimension {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for BatchGetValuesByDataFilterRequestMajorDimension {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "COLUMNS" => BatchGetValuesByDataFilterRequestMajorDimension::Columns,
                "DIMENSION_UNSPECIFIED" => {
                    BatchGetValuesByDataFilterRequestMajorDimension::DimensionUnspecified
                }
                "ROWS" => BatchGetValuesByDataFilterRequestMajorDimension::Rows,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for BatchGetValuesByDataFilterRequestMajorDimension {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BatchGetValuesByDataFilterRequestMajorDimension {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum BatchGetValuesByDataFilterRequestValueRenderOption {
        #[doc = "Values will be calculated & formatted in the reply according to the\ncell's formatting.  Formatting is based on the spreadsheet's locale,\nnot the requesting user's locale.\nFor example, if `A1` is `1.23` and `A2` is `=A1` and formatted as currency,\nthen `A2` would return `\"$1.23\"`."]
        FormattedValue,
        #[doc = "Values will not be calculated.  The reply will include the formulas.\nFor example, if `A1` is `1.23` and `A2` is `=A1` and formatted as currency,\nthen A2 would return `\"=A1\"`."]
        Formula,
        #[doc = "Values will be calculated, but not formatted in the reply.\nFor example, if `A1` is `1.23` and `A2` is `=A1` and formatted as currency,\nthen `A2` would return the number `1.23`."]
        UnformattedValue,
    }
    impl BatchGetValuesByDataFilterRequestValueRenderOption {
        pub fn as_str(self) -> &'static str {
            match self {
                BatchGetValuesByDataFilterRequestValueRenderOption::FormattedValue => {
                    "FORMATTED_VALUE"
                }
                BatchGetValuesByDataFilterRequestValueRenderOption::Formula => "FORMULA",
                BatchGetValuesByDataFilterRequestValueRenderOption::UnformattedValue => {
                    "UNFORMATTED_VALUE"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for BatchGetValuesByDataFilterRequestValueRenderOption {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for BatchGetValuesByDataFilterRequestValueRenderOption {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<BatchGetValuesByDataFilterRequestValueRenderOption, ()> {
            Ok(match s {
                "FORMATTED_VALUE" => {
                    BatchGetValuesByDataFilterRequestValueRenderOption::FormattedValue
                }
                "FORMULA" => BatchGetValuesByDataFilterRequestValueRenderOption::Formula,
                "UNFORMATTED_VALUE" => {
                    BatchGetValuesByDataFilterRequestValueRenderOption::UnformattedValue
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for BatchGetValuesByDataFilterRequestValueRenderOption {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for BatchGetValuesByDataFilterRequestValueRenderOption {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for BatchGetValuesByDataFilterRequestValueRenderOption {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "FORMATTED_VALUE" => {
                    BatchGetValuesByDataFilterRequestValueRenderOption::FormattedValue
                }
                "FORMULA" => BatchGetValuesByDataFilterRequestValueRenderOption::Formula,
                "UNFORMATTED_VALUE" => {
                    BatchGetValuesByDataFilterRequestValueRenderOption::UnformattedValue
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
    impl ::google_field_selector::FieldSelector for BatchGetValuesByDataFilterRequestValueRenderOption {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BatchGetValuesByDataFilterRequestValueRenderOption {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct BatchGetValuesByDataFilterResponse {
        #[doc = "The ID of the spreadsheet the data was retrieved from."]
        #[serde(
            rename = "spreadsheetId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub spreadsheet_id: ::std::option::Option<String>,
        #[doc = "The requested values with the list of data filters that matched them."]
        #[serde(
            rename = "valueRanges",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value_ranges: ::std::option::Option<Vec<crate::schemas::MatchedValueRange>>,
    }
    impl ::google_field_selector::FieldSelector for BatchGetValuesByDataFilterResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BatchGetValuesByDataFilterResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct BatchGetValuesResponse {
        #[doc = "The ID of the spreadsheet the data was retrieved from."]
        #[serde(
            rename = "spreadsheetId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub spreadsheet_id: ::std::option::Option<String>,
        #[doc = "The requested values. The order of the ValueRanges is the same as the\norder of the requested ranges."]
        #[serde(
            rename = "valueRanges",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value_ranges: ::std::option::Option<Vec<crate::schemas::ValueRange>>,
    }
    impl ::google_field_selector::FieldSelector for BatchGetValuesResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BatchGetValuesResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct BatchUpdateSpreadsheetRequest {
        #[doc = "Determines if the update response should include the spreadsheet\nresource."]
        #[serde(
            rename = "includeSpreadsheetInResponse",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub include_spreadsheet_in_response: ::std::option::Option<bool>,
        #[doc = "A list of updates to apply to the spreadsheet.\nRequests will be applied in the order they are specified.\nIf any request is not valid, no requests will be applied."]
        #[serde(
            rename = "requests",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub requests: ::std::option::Option<Vec<crate::schemas::Request>>,
        #[doc = "True if grid data should be returned. Meaningful only if\ninclude_spreadsheet_in_response is 'true'.\nThis parameter is ignored if a field mask was set in the request."]
        #[serde(
            rename = "responseIncludeGridData",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub response_include_grid_data: ::std::option::Option<bool>,
        #[doc = "Limits the ranges included in the response spreadsheet.\nMeaningful only if include_spreadsheet_in_response is 'true'."]
        #[serde(
            rename = "responseRanges",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub response_ranges: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for BatchUpdateSpreadsheetRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BatchUpdateSpreadsheetRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct BatchUpdateSpreadsheetResponse {
        #[doc = "The reply of the updates.  This maps 1:1 with the updates, although\nreplies to some requests may be empty."]
        #[serde(
            rename = "replies",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub replies: ::std::option::Option<Vec<crate::schemas::Response>>,
        #[doc = "The spreadsheet the updates were applied to."]
        #[serde(
            rename = "spreadsheetId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub spreadsheet_id: ::std::option::Option<String>,
        #[doc = "The spreadsheet after updates were applied. This is only set if\n[BatchUpdateSpreadsheetRequest.include_spreadsheet_in_response] is `true`."]
        #[serde(
            rename = "updatedSpreadsheet",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub updated_spreadsheet: ::std::option::Option<crate::schemas::Spreadsheet>,
    }
    impl ::google_field_selector::FieldSelector for BatchUpdateSpreadsheetResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BatchUpdateSpreadsheetResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct BatchUpdateValuesByDataFilterRequest {
        #[doc = "The new values to apply to the spreadsheet.  If more than one range is\nmatched by the specified DataFilter the specified values are applied to\nall of those ranges."]
        #[serde(
            rename = "data",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub data: ::std::option::Option<Vec<crate::schemas::DataFilterValueRange>>,
        #[doc = "Determines if the update response should include the values\nof the cells that were updated. By default, responses\ndo not include the updated values. The `updatedData` field within\neach of the BatchUpdateValuesResponse.responses contains the updated\nvalues. If the range to write was larger than the range actually written,\nthe response includes all values in the requested range (excluding trailing\nempty rows and columns)."]
        #[serde(
            rename = "includeValuesInResponse",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub include_values_in_response: ::std::option::Option<bool>,
        #[doc = "Determines how dates, times, and durations in the response should be\nrendered. This is ignored if response_value_render_option is\nFORMATTED_VALUE.\nThe default dateTime render option is\nDateTimeRenderOption.SERIAL_NUMBER."]
        #[serde(
            rename = "responseDateTimeRenderOption",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub response_date_time_render_option: ::std::option::Option<
            crate::schemas::BatchUpdateValuesByDataFilterRequestResponseDateTimeRenderOption,
        >,
        #[doc = "Determines how values in the response should be rendered.\nThe default render option is ValueRenderOption.FORMATTED_VALUE."]
        #[serde(
            rename = "responseValueRenderOption",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub response_value_render_option: ::std::option::Option<
            crate::schemas::BatchUpdateValuesByDataFilterRequestResponseValueRenderOption,
        >,
        #[doc = "How the input data should be interpreted."]
        #[serde(
            rename = "valueInputOption",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value_input_option: ::std::option::Option<
            crate::schemas::BatchUpdateValuesByDataFilterRequestValueInputOption,
        >,
    }
    impl ::google_field_selector::FieldSelector for BatchUpdateValuesByDataFilterRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BatchUpdateValuesByDataFilterRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum BatchUpdateValuesByDataFilterRequestResponseDateTimeRenderOption {
        #[doc = "Instructs date, time, datetime, and duration fields to be output\nas strings in their given number format (which is dependent\non the spreadsheet locale)."]
        FormattedString,
        #[doc = "Instructs date, time, datetime, and duration fields to be output\nas doubles in \"serial number\" format, as popularized by Lotus 1-2-3.\nThe whole number portion of the value (left of the decimal) counts\nthe days since December 30th 1899. The fractional portion (right of\nthe decimal) counts the time as a fraction of the day. For example,\nJanuary 1st 1900 at noon would be 2.5, 2 because it's 2 days after\nDecember 30st 1899, and .5 because noon is half a day.  February 1st\n1900 at 3pm would be 33.625. This correctly treats the year 1900 as\nnot a leap year."]
        SerialNumber,
    }
    impl BatchUpdateValuesByDataFilterRequestResponseDateTimeRenderOption {
        pub fn as_str(self) -> &'static str {
            match self { BatchUpdateValuesByDataFilterRequestResponseDateTimeRenderOption :: FormattedString => "FORMATTED_STRING" , BatchUpdateValuesByDataFilterRequestResponseDateTimeRenderOption :: SerialNumber => "SERIAL_NUMBER" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for BatchUpdateValuesByDataFilterRequestResponseDateTimeRenderOption
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for BatchUpdateValuesByDataFilterRequestResponseDateTimeRenderOption {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            BatchUpdateValuesByDataFilterRequestResponseDateTimeRenderOption,
            (),
        > {
            Ok ( match s { "FORMATTED_STRING" => BatchUpdateValuesByDataFilterRequestResponseDateTimeRenderOption :: FormattedString , "SERIAL_NUMBER" => BatchUpdateValuesByDataFilterRequestResponseDateTimeRenderOption :: SerialNumber , _ => return Err ( ( ) ) , } )
        }
    }
    impl ::std::fmt::Display for BatchUpdateValuesByDataFilterRequestResponseDateTimeRenderOption {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for BatchUpdateValuesByDataFilterRequestResponseDateTimeRenderOption {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for BatchUpdateValuesByDataFilterRequestResponseDateTimeRenderOption
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok ( match value { "FORMATTED_STRING" => BatchUpdateValuesByDataFilterRequestResponseDateTimeRenderOption :: FormattedString , "SERIAL_NUMBER" => BatchUpdateValuesByDataFilterRequestResponseDateTimeRenderOption :: SerialNumber , _ => return Err ( :: serde :: de :: Error :: custom ( format ! ( "invalid enum for #name: {}" , value ) ) ) , } )
        }
    }
    impl ::google_field_selector::FieldSelector
        for BatchUpdateValuesByDataFilterRequestResponseDateTimeRenderOption
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for BatchUpdateValuesByDataFilterRequestResponseDateTimeRenderOption
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum BatchUpdateValuesByDataFilterRequestResponseValueRenderOption {
        #[doc = "Values will be calculated & formatted in the reply according to the\ncell's formatting.  Formatting is based on the spreadsheet's locale,\nnot the requesting user's locale.\nFor example, if `A1` is `1.23` and `A2` is `=A1` and formatted as currency,\nthen `A2` would return `\"$1.23\"`."]
        FormattedValue,
        #[doc = "Values will not be calculated.  The reply will include the formulas.\nFor example, if `A1` is `1.23` and `A2` is `=A1` and formatted as currency,\nthen A2 would return `\"=A1\"`."]
        Formula,
        #[doc = "Values will be calculated, but not formatted in the reply.\nFor example, if `A1` is `1.23` and `A2` is `=A1` and formatted as currency,\nthen `A2` would return the number `1.23`."]
        UnformattedValue,
    }
    impl BatchUpdateValuesByDataFilterRequestResponseValueRenderOption {
        pub fn as_str(self) -> &'static str {
            match self {
                BatchUpdateValuesByDataFilterRequestResponseValueRenderOption::FormattedValue => {
                    "FORMATTED_VALUE"
                }
                BatchUpdateValuesByDataFilterRequestResponseValueRenderOption::Formula => "FORMULA",
                BatchUpdateValuesByDataFilterRequestResponseValueRenderOption::UnformattedValue => {
                    "UNFORMATTED_VALUE"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for BatchUpdateValuesByDataFilterRequestResponseValueRenderOption {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for BatchUpdateValuesByDataFilterRequestResponseValueRenderOption {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<BatchUpdateValuesByDataFilterRequestResponseValueRenderOption, ()>
        {
            Ok(match s {
                "FORMATTED_VALUE" => {
                    BatchUpdateValuesByDataFilterRequestResponseValueRenderOption::FormattedValue
                }
                "FORMULA" => BatchUpdateValuesByDataFilterRequestResponseValueRenderOption::Formula,
                "UNFORMATTED_VALUE" => {
                    BatchUpdateValuesByDataFilterRequestResponseValueRenderOption::UnformattedValue
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for BatchUpdateValuesByDataFilterRequestResponseValueRenderOption {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for BatchUpdateValuesByDataFilterRequestResponseValueRenderOption {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for BatchUpdateValuesByDataFilterRequestResponseValueRenderOption
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "FORMATTED_VALUE" => {
                    BatchUpdateValuesByDataFilterRequestResponseValueRenderOption::FormattedValue
                }
                "FORMULA" => BatchUpdateValuesByDataFilterRequestResponseValueRenderOption::Formula,
                "UNFORMATTED_VALUE" => {
                    BatchUpdateValuesByDataFilterRequestResponseValueRenderOption::UnformattedValue
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
    impl ::google_field_selector::FieldSelector
        for BatchUpdateValuesByDataFilterRequestResponseValueRenderOption
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for BatchUpdateValuesByDataFilterRequestResponseValueRenderOption
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum BatchUpdateValuesByDataFilterRequestValueInputOption {
        #[doc = "Default input value. This value must not be used."]
        InputValueOptionUnspecified,
        #[doc = "The values the user has entered will not be parsed and will be stored\nas-is."]
        Raw,
        #[doc = "The values will be parsed as if the user typed them into the UI.\nNumbers will stay as numbers, but strings may be converted to numbers,\ndates, etc. following the same rules that are applied when entering\ntext into a cell via the Google Sheets UI."]
        UserEntered,
    }
    impl BatchUpdateValuesByDataFilterRequestValueInputOption {
        pub fn as_str(self) -> &'static str {
            match self { BatchUpdateValuesByDataFilterRequestValueInputOption :: InputValueOptionUnspecified => "INPUT_VALUE_OPTION_UNSPECIFIED" , BatchUpdateValuesByDataFilterRequestValueInputOption :: Raw => "RAW" , BatchUpdateValuesByDataFilterRequestValueInputOption :: UserEntered => "USER_ENTERED" , }
        }
    }
    impl ::std::convert::AsRef<str> for BatchUpdateValuesByDataFilterRequestValueInputOption {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for BatchUpdateValuesByDataFilterRequestValueInputOption {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<BatchUpdateValuesByDataFilterRequestValueInputOption, ()>
        {
            Ok ( match s { "INPUT_VALUE_OPTION_UNSPECIFIED" => BatchUpdateValuesByDataFilterRequestValueInputOption :: InputValueOptionUnspecified , "RAW" => BatchUpdateValuesByDataFilterRequestValueInputOption :: Raw , "USER_ENTERED" => BatchUpdateValuesByDataFilterRequestValueInputOption :: UserEntered , _ => return Err ( ( ) ) , } )
        }
    }
    impl ::std::fmt::Display for BatchUpdateValuesByDataFilterRequestValueInputOption {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for BatchUpdateValuesByDataFilterRequestValueInputOption {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for BatchUpdateValuesByDataFilterRequestValueInputOption {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok ( match value { "INPUT_VALUE_OPTION_UNSPECIFIED" => BatchUpdateValuesByDataFilterRequestValueInputOption :: InputValueOptionUnspecified , "RAW" => BatchUpdateValuesByDataFilterRequestValueInputOption :: Raw , "USER_ENTERED" => BatchUpdateValuesByDataFilterRequestValueInputOption :: UserEntered , _ => return Err ( :: serde :: de :: Error :: custom ( format ! ( "invalid enum for #name: {}" , value ) ) ) , } )
        }
    }
    impl ::google_field_selector::FieldSelector
        for BatchUpdateValuesByDataFilterRequestValueInputOption
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BatchUpdateValuesByDataFilterRequestValueInputOption {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct BatchUpdateValuesByDataFilterResponse {
        #[doc = "The response for each range updated."]
        #[serde(
            rename = "responses",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub responses: ::std::option::Option<Vec<crate::schemas::UpdateValuesByDataFilterResponse>>,
        #[doc = "The spreadsheet the updates were applied to."]
        #[serde(
            rename = "spreadsheetId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub spreadsheet_id: ::std::option::Option<String>,
        #[doc = "The total number of cells updated."]
        #[serde(
            rename = "totalUpdatedCells",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub total_updated_cells: ::std::option::Option<i32>,
        #[doc = "The total number of columns where at least one cell in the column was\nupdated."]
        #[serde(
            rename = "totalUpdatedColumns",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub total_updated_columns: ::std::option::Option<i32>,
        #[doc = "The total number of rows where at least one cell in the row was updated."]
        #[serde(
            rename = "totalUpdatedRows",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub total_updated_rows: ::std::option::Option<i32>,
        #[doc = "The total number of sheets where at least one cell in the sheet was\nupdated."]
        #[serde(
            rename = "totalUpdatedSheets",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub total_updated_sheets: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for BatchUpdateValuesByDataFilterResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BatchUpdateValuesByDataFilterResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct BatchUpdateValuesRequest {
        #[doc = "The new values to apply to the spreadsheet."]
        #[serde(
            rename = "data",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub data: ::std::option::Option<Vec<crate::schemas::ValueRange>>,
        #[doc = "Determines if the update response should include the values\nof the cells that were updated. By default, responses\ndo not include the updated values. The `updatedData` field within\neach of the BatchUpdateValuesResponse.responses contains the updated\nvalues. If the range to write was larger than the range actually written,\nthe response includes all values in the requested range (excluding trailing\nempty rows and columns)."]
        #[serde(
            rename = "includeValuesInResponse",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub include_values_in_response: ::std::option::Option<bool>,
        #[doc = "Determines how dates, times, and durations in the response should be\nrendered. This is ignored if response_value_render_option is\nFORMATTED_VALUE.\nThe default dateTime render option is\nDateTimeRenderOption.SERIAL_NUMBER."]
        #[serde(
            rename = "responseDateTimeRenderOption",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub response_date_time_render_option: ::std::option::Option<
            crate::schemas::BatchUpdateValuesRequestResponseDateTimeRenderOption,
        >,
        #[doc = "Determines how values in the response should be rendered.\nThe default render option is ValueRenderOption.FORMATTED_VALUE."]
        #[serde(
            rename = "responseValueRenderOption",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub response_value_render_option: ::std::option::Option<
            crate::schemas::BatchUpdateValuesRequestResponseValueRenderOption,
        >,
        #[doc = "How the input data should be interpreted."]
        #[serde(
            rename = "valueInputOption",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value_input_option:
            ::std::option::Option<crate::schemas::BatchUpdateValuesRequestValueInputOption>,
    }
    impl ::google_field_selector::FieldSelector for BatchUpdateValuesRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BatchUpdateValuesRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum BatchUpdateValuesRequestResponseDateTimeRenderOption {
        #[doc = "Instructs date, time, datetime, and duration fields to be output\nas strings in their given number format (which is dependent\non the spreadsheet locale)."]
        FormattedString,
        #[doc = "Instructs date, time, datetime, and duration fields to be output\nas doubles in \"serial number\" format, as popularized by Lotus 1-2-3.\nThe whole number portion of the value (left of the decimal) counts\nthe days since December 30th 1899. The fractional portion (right of\nthe decimal) counts the time as a fraction of the day. For example,\nJanuary 1st 1900 at noon would be 2.5, 2 because it's 2 days after\nDecember 30st 1899, and .5 because noon is half a day.  February 1st\n1900 at 3pm would be 33.625. This correctly treats the year 1900 as\nnot a leap year."]
        SerialNumber,
    }
    impl BatchUpdateValuesRequestResponseDateTimeRenderOption {
        pub fn as_str(self) -> &'static str {
            match self {
                BatchUpdateValuesRequestResponseDateTimeRenderOption::FormattedString => {
                    "FORMATTED_STRING"
                }
                BatchUpdateValuesRequestResponseDateTimeRenderOption::SerialNumber => {
                    "SERIAL_NUMBER"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for BatchUpdateValuesRequestResponseDateTimeRenderOption {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for BatchUpdateValuesRequestResponseDateTimeRenderOption {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<BatchUpdateValuesRequestResponseDateTimeRenderOption, ()>
        {
            Ok(match s {
                "FORMATTED_STRING" => {
                    BatchUpdateValuesRequestResponseDateTimeRenderOption::FormattedString
                }
                "SERIAL_NUMBER" => {
                    BatchUpdateValuesRequestResponseDateTimeRenderOption::SerialNumber
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for BatchUpdateValuesRequestResponseDateTimeRenderOption {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for BatchUpdateValuesRequestResponseDateTimeRenderOption {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for BatchUpdateValuesRequestResponseDateTimeRenderOption {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "FORMATTED_STRING" => {
                    BatchUpdateValuesRequestResponseDateTimeRenderOption::FormattedString
                }
                "SERIAL_NUMBER" => {
                    BatchUpdateValuesRequestResponseDateTimeRenderOption::SerialNumber
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
    impl ::google_field_selector::FieldSelector
        for BatchUpdateValuesRequestResponseDateTimeRenderOption
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BatchUpdateValuesRequestResponseDateTimeRenderOption {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum BatchUpdateValuesRequestResponseValueRenderOption {
        #[doc = "Values will be calculated & formatted in the reply according to the\ncell's formatting.  Formatting is based on the spreadsheet's locale,\nnot the requesting user's locale.\nFor example, if `A1` is `1.23` and `A2` is `=A1` and formatted as currency,\nthen `A2` would return `\"$1.23\"`."]
        FormattedValue,
        #[doc = "Values will not be calculated.  The reply will include the formulas.\nFor example, if `A1` is `1.23` and `A2` is `=A1` and formatted as currency,\nthen A2 would return `\"=A1\"`."]
        Formula,
        #[doc = "Values will be calculated, but not formatted in the reply.\nFor example, if `A1` is `1.23` and `A2` is `=A1` and formatted as currency,\nthen `A2` would return the number `1.23`."]
        UnformattedValue,
    }
    impl BatchUpdateValuesRequestResponseValueRenderOption {
        pub fn as_str(self) -> &'static str {
            match self {
                BatchUpdateValuesRequestResponseValueRenderOption::FormattedValue => {
                    "FORMATTED_VALUE"
                }
                BatchUpdateValuesRequestResponseValueRenderOption::Formula => "FORMULA",
                BatchUpdateValuesRequestResponseValueRenderOption::UnformattedValue => {
                    "UNFORMATTED_VALUE"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for BatchUpdateValuesRequestResponseValueRenderOption {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for BatchUpdateValuesRequestResponseValueRenderOption {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<BatchUpdateValuesRequestResponseValueRenderOption, ()> {
            Ok(match s {
                "FORMATTED_VALUE" => {
                    BatchUpdateValuesRequestResponseValueRenderOption::FormattedValue
                }
                "FORMULA" => BatchUpdateValuesRequestResponseValueRenderOption::Formula,
                "UNFORMATTED_VALUE" => {
                    BatchUpdateValuesRequestResponseValueRenderOption::UnformattedValue
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for BatchUpdateValuesRequestResponseValueRenderOption {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for BatchUpdateValuesRequestResponseValueRenderOption {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for BatchUpdateValuesRequestResponseValueRenderOption {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "FORMATTED_VALUE" => {
                    BatchUpdateValuesRequestResponseValueRenderOption::FormattedValue
                }
                "FORMULA" => BatchUpdateValuesRequestResponseValueRenderOption::Formula,
                "UNFORMATTED_VALUE" => {
                    BatchUpdateValuesRequestResponseValueRenderOption::UnformattedValue
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
    impl ::google_field_selector::FieldSelector for BatchUpdateValuesRequestResponseValueRenderOption {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BatchUpdateValuesRequestResponseValueRenderOption {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum BatchUpdateValuesRequestValueInputOption {
        #[doc = "Default input value. This value must not be used."]
        InputValueOptionUnspecified,
        #[doc = "The values the user has entered will not be parsed and will be stored\nas-is."]
        Raw,
        #[doc = "The values will be parsed as if the user typed them into the UI.\nNumbers will stay as numbers, but strings may be converted to numbers,\ndates, etc. following the same rules that are applied when entering\ntext into a cell via the Google Sheets UI."]
        UserEntered,
    }
    impl BatchUpdateValuesRequestValueInputOption {
        pub fn as_str(self) -> &'static str {
            match self {
                BatchUpdateValuesRequestValueInputOption::InputValueOptionUnspecified => {
                    "INPUT_VALUE_OPTION_UNSPECIFIED"
                }
                BatchUpdateValuesRequestValueInputOption::Raw => "RAW",
                BatchUpdateValuesRequestValueInputOption::UserEntered => "USER_ENTERED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for BatchUpdateValuesRequestValueInputOption {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for BatchUpdateValuesRequestValueInputOption {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<BatchUpdateValuesRequestValueInputOption, ()> {
            Ok(match s {
                "INPUT_VALUE_OPTION_UNSPECIFIED" => {
                    BatchUpdateValuesRequestValueInputOption::InputValueOptionUnspecified
                }
                "RAW" => BatchUpdateValuesRequestValueInputOption::Raw,
                "USER_ENTERED" => BatchUpdateValuesRequestValueInputOption::UserEntered,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for BatchUpdateValuesRequestValueInputOption {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for BatchUpdateValuesRequestValueInputOption {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for BatchUpdateValuesRequestValueInputOption {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "INPUT_VALUE_OPTION_UNSPECIFIED" => {
                    BatchUpdateValuesRequestValueInputOption::InputValueOptionUnspecified
                }
                "RAW" => BatchUpdateValuesRequestValueInputOption::Raw,
                "USER_ENTERED" => BatchUpdateValuesRequestValueInputOption::UserEntered,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for BatchUpdateValuesRequestValueInputOption {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BatchUpdateValuesRequestValueInputOption {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct BatchUpdateValuesResponse {
        #[doc = "One UpdateValuesResponse per requested range, in the same order as\nthe requests appeared."]
        #[serde(
            rename = "responses",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub responses: ::std::option::Option<Vec<crate::schemas::UpdateValuesResponse>>,
        #[doc = "The spreadsheet the updates were applied to."]
        #[serde(
            rename = "spreadsheetId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub spreadsheet_id: ::std::option::Option<String>,
        #[doc = "The total number of cells updated."]
        #[serde(
            rename = "totalUpdatedCells",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub total_updated_cells: ::std::option::Option<i32>,
        #[doc = "The total number of columns where at least one cell in the column was\nupdated."]
        #[serde(
            rename = "totalUpdatedColumns",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub total_updated_columns: ::std::option::Option<i32>,
        #[doc = "The total number of rows where at least one cell in the row was updated."]
        #[serde(
            rename = "totalUpdatedRows",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub total_updated_rows: ::std::option::Option<i32>,
        #[doc = "The total number of sheets where at least one cell in the sheet was\nupdated."]
        #[serde(
            rename = "totalUpdatedSheets",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub total_updated_sheets: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for BatchUpdateValuesResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BatchUpdateValuesResponse {
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
    pub struct BooleanCondition {
        #[doc = "The type of condition."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::BooleanConditionType>,
        #[doc = "The values of the condition. The number of supported values depends\non the condition type.  Some support zero values,\nothers one or two values,\nand ConditionType.ONE_OF_LIST supports an arbitrary number of values."]
        #[serde(
            rename = "values",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub values: ::std::option::Option<Vec<crate::schemas::ConditionValue>>,
    }
    impl ::google_field_selector::FieldSelector for BooleanCondition {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BooleanCondition {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum BooleanConditionType {
        #[doc = "The cell's value must be empty.\nSupported by conditional formatting and filters.\nRequires no ConditionValues."]
        Blank,
        #[doc = "The cell's value must be TRUE/FALSE or in the list of condition values.\nSupported by data validation.\nRenders as a cell checkbox.\nSupports zero, one or two ConditionValues.  No\nvalues indicates the cell must be TRUE or FALSE, where TRUE renders as\nchecked and FALSE renders as unchecked.  One value indicates the cell\nwill render as checked when it contains that value and unchecked when it\nis blank.  Two values indicate that the cell will render as checked when\nit contains the first value and unchecked when it contains the second\nvalue.  For example, [\"Yes\",\"No\"] indicates that the cell will render a\nchecked box when it has the value \"Yes\" and an unchecked box when it has\nthe value \"No\"."]
        Boolean,
        #[doc = "The default value, do not use."]
        ConditionTypeUnspecified,
        #[doc = "The condition's formula must evaluate to true.\nSupported by data validation, conditional formatting and filters.\nRequires a single ConditionValue."]
        CustomFormula,
        #[doc = "The cell's value must be after the date of the condition's value.\nSupported by data validation, conditional formatting and filters.\nRequires a single ConditionValue\nthat may be a relative date."]
        DateAfter,
        #[doc = "The cell's value must be before the date of the condition's value.\nSupported by data validation, conditional formatting and filters.\nRequires a single ConditionValue\nthat may be a relative date."]
        DateBefore,
        #[doc = "The cell's value must be between the dates of the two condition values.\nSupported by data validation.\nRequires exactly two ConditionValues."]
        DateBetween,
        #[doc = "The cell's value must be the same date as the condition's value.\nSupported by data validation, conditional formatting and filters.\nRequires a single ConditionValue."]
        DateEq,
        #[doc = "The cell's value must be a date.\nSupported by data validation.\nRequires no ConditionValues."]
        DateIsValid,
        #[doc = "The cell's value must be outside the dates of the two condition values.\nSupported by data validation.\nRequires exactly two ConditionValues."]
        DateNotBetween,
        #[doc = "The cell's value must be on or after the date of the condition's value.\nSupported by data validation.\nRequires a single ConditionValue\nthat may be a relative date."]
        DateOnOrAfter,
        #[doc = "The cell's value must be on or before the date of the condition's value.\nSupported by data validation.\nRequires a single ConditionValue\nthat may be a relative date."]
        DateOnOrBefore,
        #[doc = "The cell's value must not be empty.\nSupported by conditional formatting and filters.\nRequires no ConditionValues."]
        NotBlank,
        #[doc = "The cell's value must be between the two condition values.\nSupported by data validation, conditional formatting and filters.\nRequires exactly two ConditionValues."]
        NumberBetween,
        #[doc = "The cell's value must be equal to the condition's value.\nSupported by data validation, conditional formatting and filters.\nRequires a single ConditionValue."]
        NumberEq,
        #[doc = "The cell's value must be greater than the condition's value.\nSupported by data validation, conditional formatting and filters.\nRequires a single ConditionValue."]
        NumberGreater,
        #[doc = "The cell's value must be greater than or equal to the condition's value.\nSupported by data validation, conditional formatting and filters.\nRequires a single ConditionValue."]
        NumberGreaterThanEq,
        #[doc = "The cell's value must be less than the condition's value.\nSupported by data validation, conditional formatting and filters.\nRequires a single ConditionValue."]
        NumberLess,
        #[doc = "The cell's value must be less than or equal to the condition's value.\nSupported by data validation, conditional formatting and filters.\nRequires a single ConditionValue."]
        NumberLessThanEq,
        #[doc = "The cell's value must not be between the two condition values.\nSupported by data validation, conditional formatting and filters.\nRequires exactly two ConditionValues."]
        NumberNotBetween,
        #[doc = "The cell's value must be not equal to the condition's value.\nSupported by data validation, conditional formatting and filters.\nRequires a single ConditionValue."]
        NumberNotEq,
        #[doc = "The cell's value must be in the list of condition values.\nSupported by data validation.\nSupports any number of condition values,\none per item in the list.\nFormulas are not supported in the values."]
        OneOfList,
        #[doc = "The cell's value must be listed in the grid in condition value's range.\nSupported by data validation.\nRequires a single ConditionValue,\nand the value must be a valid range in A1 notation."]
        OneOfRange,
        #[doc = "The cell's value must contain the condition's value.\nSupported by data validation, conditional formatting and filters.\nRequires a single ConditionValue."]
        TextContains,
        #[doc = "The cell's value must end with the condition's value.\nSupported by conditional formatting and filters.\nRequires a single ConditionValue."]
        TextEndsWith,
        #[doc = "The cell's value must be exactly the condition's value.\nSupported by data validation, conditional formatting and filters.\nRequires a single ConditionValue."]
        TextEq,
        #[doc = "The cell's value must be a valid email address.\nSupported by data validation.\nRequires no ConditionValues."]
        TextIsEmail,
        #[doc = "The cell's value must be a valid URL.\nSupported by data validation.\nRequires no ConditionValues."]
        TextIsUrl,
        #[doc = "The cell's value must not contain the condition's value.\nSupported by data validation, conditional formatting and filters.\nRequires a single ConditionValue."]
        TextNotContains,
        #[doc = "The cell's value must start with the condition's value.\nSupported by conditional formatting and filters.\nRequires a single ConditionValue."]
        TextStartsWith,
    }
    impl BooleanConditionType {
        pub fn as_str(self) -> &'static str {
            match self {
                BooleanConditionType::Blank => "BLANK",
                BooleanConditionType::Boolean => "BOOLEAN",
                BooleanConditionType::ConditionTypeUnspecified => "CONDITION_TYPE_UNSPECIFIED",
                BooleanConditionType::CustomFormula => "CUSTOM_FORMULA",
                BooleanConditionType::DateAfter => "DATE_AFTER",
                BooleanConditionType::DateBefore => "DATE_BEFORE",
                BooleanConditionType::DateBetween => "DATE_BETWEEN",
                BooleanConditionType::DateEq => "DATE_EQ",
                BooleanConditionType::DateIsValid => "DATE_IS_VALID",
                BooleanConditionType::DateNotBetween => "DATE_NOT_BETWEEN",
                BooleanConditionType::DateOnOrAfter => "DATE_ON_OR_AFTER",
                BooleanConditionType::DateOnOrBefore => "DATE_ON_OR_BEFORE",
                BooleanConditionType::NotBlank => "NOT_BLANK",
                BooleanConditionType::NumberBetween => "NUMBER_BETWEEN",
                BooleanConditionType::NumberEq => "NUMBER_EQ",
                BooleanConditionType::NumberGreater => "NUMBER_GREATER",
                BooleanConditionType::NumberGreaterThanEq => "NUMBER_GREATER_THAN_EQ",
                BooleanConditionType::NumberLess => "NUMBER_LESS",
                BooleanConditionType::NumberLessThanEq => "NUMBER_LESS_THAN_EQ",
                BooleanConditionType::NumberNotBetween => "NUMBER_NOT_BETWEEN",
                BooleanConditionType::NumberNotEq => "NUMBER_NOT_EQ",
                BooleanConditionType::OneOfList => "ONE_OF_LIST",
                BooleanConditionType::OneOfRange => "ONE_OF_RANGE",
                BooleanConditionType::TextContains => "TEXT_CONTAINS",
                BooleanConditionType::TextEndsWith => "TEXT_ENDS_WITH",
                BooleanConditionType::TextEq => "TEXT_EQ",
                BooleanConditionType::TextIsEmail => "TEXT_IS_EMAIL",
                BooleanConditionType::TextIsUrl => "TEXT_IS_URL",
                BooleanConditionType::TextNotContains => "TEXT_NOT_CONTAINS",
                BooleanConditionType::TextStartsWith => "TEXT_STARTS_WITH",
            }
        }
    }
    impl ::std::convert::AsRef<str> for BooleanConditionType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for BooleanConditionType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<BooleanConditionType, ()> {
            Ok(match s {
                "BLANK" => BooleanConditionType::Blank,
                "BOOLEAN" => BooleanConditionType::Boolean,
                "CONDITION_TYPE_UNSPECIFIED" => BooleanConditionType::ConditionTypeUnspecified,
                "CUSTOM_FORMULA" => BooleanConditionType::CustomFormula,
                "DATE_AFTER" => BooleanConditionType::DateAfter,
                "DATE_BEFORE" => BooleanConditionType::DateBefore,
                "DATE_BETWEEN" => BooleanConditionType::DateBetween,
                "DATE_EQ" => BooleanConditionType::DateEq,
                "DATE_IS_VALID" => BooleanConditionType::DateIsValid,
                "DATE_NOT_BETWEEN" => BooleanConditionType::DateNotBetween,
                "DATE_ON_OR_AFTER" => BooleanConditionType::DateOnOrAfter,
                "DATE_ON_OR_BEFORE" => BooleanConditionType::DateOnOrBefore,
                "NOT_BLANK" => BooleanConditionType::NotBlank,
                "NUMBER_BETWEEN" => BooleanConditionType::NumberBetween,
                "NUMBER_EQ" => BooleanConditionType::NumberEq,
                "NUMBER_GREATER" => BooleanConditionType::NumberGreater,
                "NUMBER_GREATER_THAN_EQ" => BooleanConditionType::NumberGreaterThanEq,
                "NUMBER_LESS" => BooleanConditionType::NumberLess,
                "NUMBER_LESS_THAN_EQ" => BooleanConditionType::NumberLessThanEq,
                "NUMBER_NOT_BETWEEN" => BooleanConditionType::NumberNotBetween,
                "NUMBER_NOT_EQ" => BooleanConditionType::NumberNotEq,
                "ONE_OF_LIST" => BooleanConditionType::OneOfList,
                "ONE_OF_RANGE" => BooleanConditionType::OneOfRange,
                "TEXT_CONTAINS" => BooleanConditionType::TextContains,
                "TEXT_ENDS_WITH" => BooleanConditionType::TextEndsWith,
                "TEXT_EQ" => BooleanConditionType::TextEq,
                "TEXT_IS_EMAIL" => BooleanConditionType::TextIsEmail,
                "TEXT_IS_URL" => BooleanConditionType::TextIsUrl,
                "TEXT_NOT_CONTAINS" => BooleanConditionType::TextNotContains,
                "TEXT_STARTS_WITH" => BooleanConditionType::TextStartsWith,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for BooleanConditionType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for BooleanConditionType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for BooleanConditionType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BLANK" => BooleanConditionType::Blank,
                "BOOLEAN" => BooleanConditionType::Boolean,
                "CONDITION_TYPE_UNSPECIFIED" => BooleanConditionType::ConditionTypeUnspecified,
                "CUSTOM_FORMULA" => BooleanConditionType::CustomFormula,
                "DATE_AFTER" => BooleanConditionType::DateAfter,
                "DATE_BEFORE" => BooleanConditionType::DateBefore,
                "DATE_BETWEEN" => BooleanConditionType::DateBetween,
                "DATE_EQ" => BooleanConditionType::DateEq,
                "DATE_IS_VALID" => BooleanConditionType::DateIsValid,
                "DATE_NOT_BETWEEN" => BooleanConditionType::DateNotBetween,
                "DATE_ON_OR_AFTER" => BooleanConditionType::DateOnOrAfter,
                "DATE_ON_OR_BEFORE" => BooleanConditionType::DateOnOrBefore,
                "NOT_BLANK" => BooleanConditionType::NotBlank,
                "NUMBER_BETWEEN" => BooleanConditionType::NumberBetween,
                "NUMBER_EQ" => BooleanConditionType::NumberEq,
                "NUMBER_GREATER" => BooleanConditionType::NumberGreater,
                "NUMBER_GREATER_THAN_EQ" => BooleanConditionType::NumberGreaterThanEq,
                "NUMBER_LESS" => BooleanConditionType::NumberLess,
                "NUMBER_LESS_THAN_EQ" => BooleanConditionType::NumberLessThanEq,
                "NUMBER_NOT_BETWEEN" => BooleanConditionType::NumberNotBetween,
                "NUMBER_NOT_EQ" => BooleanConditionType::NumberNotEq,
                "ONE_OF_LIST" => BooleanConditionType::OneOfList,
                "ONE_OF_RANGE" => BooleanConditionType::OneOfRange,
                "TEXT_CONTAINS" => BooleanConditionType::TextContains,
                "TEXT_ENDS_WITH" => BooleanConditionType::TextEndsWith,
                "TEXT_EQ" => BooleanConditionType::TextEq,
                "TEXT_IS_EMAIL" => BooleanConditionType::TextIsEmail,
                "TEXT_IS_URL" => BooleanConditionType::TextIsUrl,
                "TEXT_NOT_CONTAINS" => BooleanConditionType::TextNotContains,
                "TEXT_STARTS_WITH" => BooleanConditionType::TextStartsWith,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for BooleanConditionType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BooleanConditionType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct BooleanRule {
        #[doc = "The condition of the rule. If the condition evaluates to true,\nthe format is applied."]
        #[serde(
            rename = "condition",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub condition: ::std::option::Option<crate::schemas::BooleanCondition>,
        #[doc = "The format to apply.\nConditional formatting can only apply a subset of formatting:\nbold, italic,\nstrikethrough,\nforeground color &\nbackground color."]
        #[serde(
            rename = "format",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub format: ::std::option::Option<crate::schemas::CellFormat>,
    }
    impl ::google_field_selector::FieldSelector for BooleanRule {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BooleanRule {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Border {
        #[doc = "The color of the border."]
        #[serde(
            rename = "color",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub color: ::std::option::Option<crate::schemas::Color>,
        #[doc = "The color of the border.\nIf color is also set, this field takes precedence."]
        #[serde(
            rename = "colorStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub color_style: ::std::option::Option<crate::schemas::ColorStyle>,
        #[doc = "The style of the border."]
        #[serde(
            rename = "style",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub style: ::std::option::Option<crate::schemas::BorderStyle>,
        #[doc = "The width of the border, in pixels.\nDeprecated; the width is determined by the \"style\" field."]
        #[serde(
            rename = "width",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub width: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for Border {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Border {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum BorderStyle {
        #[doc = "The border is dashed."]
        Dashed,
        #[doc = "The border is dotted."]
        Dotted,
        #[doc = "The border is two solid lines."]
        Double,
        #[doc = "No border.\nUsed only when updating a border in order to erase it."]
        None,
        #[doc = "The border is a thin solid line."]
        Solid,
        #[doc = "The border is a medium solid line."]
        SolidMedium,
        #[doc = "The border is a thick solid line."]
        SolidThick,
        #[doc = "The style is not specified. Do not use this."]
        StyleUnspecified,
    }
    impl BorderStyle {
        pub fn as_str(self) -> &'static str {
            match self {
                BorderStyle::Dashed => "DASHED",
                BorderStyle::Dotted => "DOTTED",
                BorderStyle::Double => "DOUBLE",
                BorderStyle::None => "NONE",
                BorderStyle::Solid => "SOLID",
                BorderStyle::SolidMedium => "SOLID_MEDIUM",
                BorderStyle::SolidThick => "SOLID_THICK",
                BorderStyle::StyleUnspecified => "STYLE_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for BorderStyle {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for BorderStyle {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<BorderStyle, ()> {
            Ok(match s {
                "DASHED" => BorderStyle::Dashed,
                "DOTTED" => BorderStyle::Dotted,
                "DOUBLE" => BorderStyle::Double,
                "NONE" => BorderStyle::None,
                "SOLID" => BorderStyle::Solid,
                "SOLID_MEDIUM" => BorderStyle::SolidMedium,
                "SOLID_THICK" => BorderStyle::SolidThick,
                "STYLE_UNSPECIFIED" => BorderStyle::StyleUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for BorderStyle {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for BorderStyle {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for BorderStyle {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DASHED" => BorderStyle::Dashed,
                "DOTTED" => BorderStyle::Dotted,
                "DOUBLE" => BorderStyle::Double,
                "NONE" => BorderStyle::None,
                "SOLID" => BorderStyle::Solid,
                "SOLID_MEDIUM" => BorderStyle::SolidMedium,
                "SOLID_THICK" => BorderStyle::SolidThick,
                "STYLE_UNSPECIFIED" => BorderStyle::StyleUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for BorderStyle {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BorderStyle {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Borders {
        #[doc = "The bottom border of the cell."]
        #[serde(
            rename = "bottom",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bottom: ::std::option::Option<crate::schemas::Border>,
        #[doc = "The left border of the cell."]
        #[serde(
            rename = "left",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub left: ::std::option::Option<crate::schemas::Border>,
        #[doc = "The right border of the cell."]
        #[serde(
            rename = "right",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub right: ::std::option::Option<crate::schemas::Border>,
        #[doc = "The top border of the cell."]
        #[serde(
            rename = "top",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub top: ::std::option::Option<crate::schemas::Border>,
    }
    impl ::google_field_selector::FieldSelector for Borders {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Borders {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct BubbleChartSpec {
        #[doc = "The bubble border color."]
        #[serde(
            rename = "bubbleBorderColor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bubble_border_color: ::std::option::Option<crate::schemas::Color>,
        #[doc = "The bubble border color.\nIf bubble_border_color is also set, this field takes precedence."]
        #[serde(
            rename = "bubbleBorderColorStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bubble_border_color_style: ::std::option::Option<crate::schemas::ColorStyle>,
        #[doc = "The data containing the bubble labels.  These do not need to be unique."]
        #[serde(
            rename = "bubbleLabels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bubble_labels: ::std::option::Option<crate::schemas::ChartData>,
        #[doc = "The max radius size of the bubbles, in pixels.\nIf specified, the field must be a positive value."]
        #[serde(
            rename = "bubbleMaxRadiusSize",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bubble_max_radius_size: ::std::option::Option<i32>,
        #[doc = "The minimum radius size of the bubbles, in pixels.\nIf specific, the field must be a positive value."]
        #[serde(
            rename = "bubbleMinRadiusSize",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bubble_min_radius_size: ::std::option::Option<i32>,
        #[doc = "The opacity of the bubbles between 0 and 1.0.\n0 is fully transparent and 1 is fully opaque."]
        #[serde(
            rename = "bubbleOpacity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bubble_opacity: ::std::option::Option<f32>,
        #[doc = "The data contianing the bubble sizes.  Bubble sizes are used to draw\nthe bubbles at different sizes relative to each other.\nIf specified, group_ids must also be specified.  This field is\noptional."]
        #[serde(
            rename = "bubbleSizes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bubble_sizes: ::std::option::Option<crate::schemas::ChartData>,
        #[doc = "The format of the text inside the bubbles.\nUnderline and Strikethrough are not supported."]
        #[serde(
            rename = "bubbleTextStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bubble_text_style: ::std::option::Option<crate::schemas::TextFormat>,
        #[doc = "The data containing the bubble x-values.  These values locate the bubbles\nin the chart horizontally."]
        #[serde(
            rename = "domain",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub domain: ::std::option::Option<crate::schemas::ChartData>,
        #[doc = "The data containing the bubble group IDs. All bubbles with the same group\nID are drawn in the same color. If bubble_sizes is specified then\nthis field must also be specified but may contain blank values.\nThis field is optional."]
        #[serde(
            rename = "groupIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub group_ids: ::std::option::Option<crate::schemas::ChartData>,
        #[doc = "Where the legend of the chart should be drawn."]
        #[serde(
            rename = "legendPosition",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub legend_position: ::std::option::Option<crate::schemas::BubbleChartSpecLegendPosition>,
        #[doc = "The data contianing the bubble y-values.  These values locate the bubbles\nin the chart vertically."]
        #[serde(
            rename = "series",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub series: ::std::option::Option<crate::schemas::ChartData>,
    }
    impl ::google_field_selector::FieldSelector for BubbleChartSpec {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BubbleChartSpec {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum BubbleChartSpecLegendPosition {
        #[doc = "The legend is rendered on the bottom of the chart."]
        BottomLegend,
        #[doc = "Default value, do not use."]
        BubbleChartLegendPositionUnspecified,
        #[doc = "The legend is rendered inside the chart area."]
        InsideLegend,
        #[doc = "The legend is rendered on the left of the chart."]
        LeftLegend,
        #[doc = "No legend is rendered."]
        NoLegend,
        #[doc = "The legend is rendered on the right of the chart."]
        RightLegend,
        #[doc = "The legend is rendered on the top of the chart."]
        TopLegend,
    }
    impl BubbleChartSpecLegendPosition {
        pub fn as_str(self) -> &'static str {
            match self {
                BubbleChartSpecLegendPosition::BottomLegend => "BOTTOM_LEGEND",
                BubbleChartSpecLegendPosition::BubbleChartLegendPositionUnspecified => {
                    "BUBBLE_CHART_LEGEND_POSITION_UNSPECIFIED"
                }
                BubbleChartSpecLegendPosition::InsideLegend => "INSIDE_LEGEND",
                BubbleChartSpecLegendPosition::LeftLegend => "LEFT_LEGEND",
                BubbleChartSpecLegendPosition::NoLegend => "NO_LEGEND",
                BubbleChartSpecLegendPosition::RightLegend => "RIGHT_LEGEND",
                BubbleChartSpecLegendPosition::TopLegend => "TOP_LEGEND",
            }
        }
    }
    impl ::std::convert::AsRef<str> for BubbleChartSpecLegendPosition {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for BubbleChartSpecLegendPosition {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<BubbleChartSpecLegendPosition, ()> {
            Ok(match s {
                "BOTTOM_LEGEND" => BubbleChartSpecLegendPosition::BottomLegend,
                "BUBBLE_CHART_LEGEND_POSITION_UNSPECIFIED" => {
                    BubbleChartSpecLegendPosition::BubbleChartLegendPositionUnspecified
                }
                "INSIDE_LEGEND" => BubbleChartSpecLegendPosition::InsideLegend,
                "LEFT_LEGEND" => BubbleChartSpecLegendPosition::LeftLegend,
                "NO_LEGEND" => BubbleChartSpecLegendPosition::NoLegend,
                "RIGHT_LEGEND" => BubbleChartSpecLegendPosition::RightLegend,
                "TOP_LEGEND" => BubbleChartSpecLegendPosition::TopLegend,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for BubbleChartSpecLegendPosition {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for BubbleChartSpecLegendPosition {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for BubbleChartSpecLegendPosition {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BOTTOM_LEGEND" => BubbleChartSpecLegendPosition::BottomLegend,
                "BUBBLE_CHART_LEGEND_POSITION_UNSPECIFIED" => {
                    BubbleChartSpecLegendPosition::BubbleChartLegendPositionUnspecified
                }
                "INSIDE_LEGEND" => BubbleChartSpecLegendPosition::InsideLegend,
                "LEFT_LEGEND" => BubbleChartSpecLegendPosition::LeftLegend,
                "NO_LEGEND" => BubbleChartSpecLegendPosition::NoLegend,
                "RIGHT_LEGEND" => BubbleChartSpecLegendPosition::RightLegend,
                "TOP_LEGEND" => BubbleChartSpecLegendPosition::TopLegend,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for BubbleChartSpecLegendPosition {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BubbleChartSpecLegendPosition {
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
    pub struct CandlestickChartSpec {
        #[doc = "The Candlestick chart data.\nOnly one CandlestickData is supported."]
        #[serde(
            rename = "data",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub data: ::std::option::Option<Vec<crate::schemas::CandlestickData>>,
        #[doc = "The domain data (horizontal axis) for the candlestick chart.  String data\nwill be treated as discrete labels, other data will be treated as\ncontinuous values."]
        #[serde(
            rename = "domain",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub domain: ::std::option::Option<crate::schemas::CandlestickDomain>,
    }
    impl ::google_field_selector::FieldSelector for CandlestickChartSpec {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CandlestickChartSpec {
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
    pub struct CandlestickData {
        #[doc = "The range data (vertical axis) for the close/final value for each candle.\nThis is the top of the candle body.  If greater than the open value the\ncandle will be filled.  Otherwise the candle will be hollow."]
        #[serde(
            rename = "closeSeries",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub close_series: ::std::option::Option<crate::schemas::CandlestickSeries>,
        #[doc = "The range data (vertical axis) for the high/maximum value for each\ncandle. This is the top of the candle's center line."]
        #[serde(
            rename = "highSeries",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub high_series: ::std::option::Option<crate::schemas::CandlestickSeries>,
        #[doc = "The range data (vertical axis) for the low/minimum value for each candle.\nThis is the bottom of the candle's center line."]
        #[serde(
            rename = "lowSeries",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub low_series: ::std::option::Option<crate::schemas::CandlestickSeries>,
        #[doc = "The range data (vertical axis) for the open/initial value for each\ncandle. This is the bottom of the candle body.  If less than the close\nvalue the candle will be filled.  Otherwise the candle will be hollow."]
        #[serde(
            rename = "openSeries",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub open_series: ::std::option::Option<crate::schemas::CandlestickSeries>,
    }
    impl ::google_field_selector::FieldSelector for CandlestickData {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CandlestickData {
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
    pub struct CandlestickDomain {
        #[doc = "The data of the CandlestickDomain."]
        #[serde(
            rename = "data",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub data: ::std::option::Option<crate::schemas::ChartData>,
        #[doc = "True to reverse the order of the domain values (horizontal axis)."]
        #[serde(
            rename = "reversed",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub reversed: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for CandlestickDomain {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CandlestickDomain {
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
    pub struct CandlestickSeries {
        #[doc = "The data of the CandlestickSeries."]
        #[serde(
            rename = "data",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub data: ::std::option::Option<crate::schemas::ChartData>,
    }
    impl ::google_field_selector::FieldSelector for CandlestickSeries {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CandlestickSeries {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct CellData {
        #[doc = "A data validation rule on the cell, if any.\n\nWhen writing, the new data validation rule will overwrite any prior rule."]
        #[serde(
            rename = "dataValidation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub data_validation: ::std::option::Option<crate::schemas::DataValidationRule>,
        #[doc = "The effective format being used by the cell.\nThis includes the results of applying any conditional formatting and,\nif the cell contains a formula, the computed number format.\nIf the effective format is the default format, effective format will\nnot be written.\nThis field is read-only."]
        #[serde(
            rename = "effectiveFormat",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub effective_format: ::std::option::Option<crate::schemas::CellFormat>,
        #[doc = "The effective value of the cell. For cells with formulas, this is\nthe calculated value.  For cells with literals, this is\nthe same as the user_entered_value.\nThis field is read-only."]
        #[serde(
            rename = "effectiveValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub effective_value: ::std::option::Option<crate::schemas::ExtendedValue>,
        #[doc = "The formatted value of the cell.\nThis is the value as it's shown to the user.\nThis field is read-only."]
        #[serde(
            rename = "formattedValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub formatted_value: ::std::option::Option<String>,
        #[doc = "A hyperlink this cell points to, if any.\nThis field is read-only.  (To set it, use a `=HYPERLINK` formula\nin the userEnteredValue.formulaValue\nfield.)"]
        #[serde(
            rename = "hyperlink",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub hyperlink: ::std::option::Option<String>,
        #[doc = "Any note on the cell."]
        #[serde(
            rename = "note",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub note: ::std::option::Option<String>,
        #[doc = "A pivot table anchored at this cell. The size of pivot table itself\nis computed dynamically based on its data, grouping, filters, values,\netc. Only the top-left cell of the pivot table contains the pivot table\ndefinition. The other cells will contain the calculated values of the\nresults of the pivot in their effective_value fields."]
        #[serde(
            rename = "pivotTable",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub pivot_table: ::std::option::Option<crate::schemas::PivotTable>,
        #[doc = "Runs of rich text applied to subsections of the cell.  Runs are only valid\non user entered strings, not formulas, bools, or numbers.\nRuns start at specific indexes in the text and continue until the next\nrun. Properties of a run will continue unless explicitly changed\nin a subsequent run (and properties of the first run will continue\nthe properties of the cell unless explicitly changed).\n\nWhen writing, the new runs will overwrite any prior runs.  When writing a\nnew user_entered_value, previous runs are erased."]
        #[serde(
            rename = "textFormatRuns",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text_format_runs: ::std::option::Option<Vec<crate::schemas::TextFormatRun>>,
        #[doc = "The format the user entered for the cell.\n\nWhen writing, the new format will be merged with the existing format."]
        #[serde(
            rename = "userEnteredFormat",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub user_entered_format: ::std::option::Option<crate::schemas::CellFormat>,
        #[doc = "The value the user entered in the cell. e.g, `1234`, `'Hello'`, or `=NOW()`\nNote: Dates, Times and DateTimes are represented as doubles in\nserial number format."]
        #[serde(
            rename = "userEnteredValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub user_entered_value: ::std::option::Option<crate::schemas::ExtendedValue>,
    }
    impl ::google_field_selector::FieldSelector for CellData {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CellData {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct CellFormat {
        #[doc = "The background color of the cell."]
        #[serde(
            rename = "backgroundColor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub background_color: ::std::option::Option<crate::schemas::Color>,
        #[doc = "The background color of the cell.\nIf background_color is also set, this field takes precedence."]
        #[serde(
            rename = "backgroundColorStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub background_color_style: ::std::option::Option<crate::schemas::ColorStyle>,
        #[doc = "The borders of the cell."]
        #[serde(
            rename = "borders",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub borders: ::std::option::Option<crate::schemas::Borders>,
        #[doc = "The horizontal alignment of the value in the cell."]
        #[serde(
            rename = "horizontalAlignment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub horizontal_alignment:
            ::std::option::Option<crate::schemas::CellFormatHorizontalAlignment>,
        #[doc = "How a hyperlink, if it exists, should be displayed in the cell."]
        #[serde(
            rename = "hyperlinkDisplayType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub hyperlink_display_type:
            ::std::option::Option<crate::schemas::CellFormatHyperlinkDisplayType>,
        #[doc = "A format describing how number values should be represented to the user."]
        #[serde(
            rename = "numberFormat",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub number_format: ::std::option::Option<crate::schemas::NumberFormat>,
        #[doc = "The padding of the cell."]
        #[serde(
            rename = "padding",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub padding: ::std::option::Option<crate::schemas::Padding>,
        #[doc = "The direction of the text in the cell."]
        #[serde(
            rename = "textDirection",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text_direction: ::std::option::Option<crate::schemas::CellFormatTextDirection>,
        #[doc = "The format of the text in the cell (unless overridden by a format run)."]
        #[serde(
            rename = "textFormat",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text_format: ::std::option::Option<crate::schemas::TextFormat>,
        #[doc = "The rotation applied to text in a cell"]
        #[serde(
            rename = "textRotation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text_rotation: ::std::option::Option<crate::schemas::TextRotation>,
        #[doc = "The vertical alignment of the value in the cell."]
        #[serde(
            rename = "verticalAlignment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub vertical_alignment: ::std::option::Option<crate::schemas::CellFormatVerticalAlignment>,
        #[doc = "The wrap strategy for the value in the cell."]
        #[serde(
            rename = "wrapStrategy",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub wrap_strategy: ::std::option::Option<crate::schemas::CellFormatWrapStrategy>,
    }
    impl ::google_field_selector::FieldSelector for CellFormat {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CellFormat {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CellFormatHorizontalAlignment {
        #[doc = "The text is explicitly aligned to the center of the cell."]
        Center,
        #[doc = "The horizontal alignment is not specified. Do not use this."]
        HorizontalAlignUnspecified,
        #[doc = "The text is explicitly aligned to the left of the cell."]
        Left,
        #[doc = "The text is explicitly aligned to the right of the cell."]
        Right,
    }
    impl CellFormatHorizontalAlignment {
        pub fn as_str(self) -> &'static str {
            match self {
                CellFormatHorizontalAlignment::Center => "CENTER",
                CellFormatHorizontalAlignment::HorizontalAlignUnspecified => {
                    "HORIZONTAL_ALIGN_UNSPECIFIED"
                }
                CellFormatHorizontalAlignment::Left => "LEFT",
                CellFormatHorizontalAlignment::Right => "RIGHT",
            }
        }
    }
    impl ::std::convert::AsRef<str> for CellFormatHorizontalAlignment {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CellFormatHorizontalAlignment {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<CellFormatHorizontalAlignment, ()> {
            Ok(match s {
                "CENTER" => CellFormatHorizontalAlignment::Center,
                "HORIZONTAL_ALIGN_UNSPECIFIED" => {
                    CellFormatHorizontalAlignment::HorizontalAlignUnspecified
                }
                "LEFT" => CellFormatHorizontalAlignment::Left,
                "RIGHT" => CellFormatHorizontalAlignment::Right,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for CellFormatHorizontalAlignment {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CellFormatHorizontalAlignment {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CellFormatHorizontalAlignment {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CENTER" => CellFormatHorizontalAlignment::Center,
                "HORIZONTAL_ALIGN_UNSPECIFIED" => {
                    CellFormatHorizontalAlignment::HorizontalAlignUnspecified
                }
                "LEFT" => CellFormatHorizontalAlignment::Left,
                "RIGHT" => CellFormatHorizontalAlignment::Right,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for CellFormatHorizontalAlignment {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CellFormatHorizontalAlignment {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CellFormatHyperlinkDisplayType {
        #[doc = "The default value: the hyperlink is rendered. Do not use this."]
        HyperlinkDisplayTypeUnspecified,
        #[doc = "A hyperlink should be explicitly rendered."]
        Linked,
        #[doc = "A hyperlink should not be rendered."]
        PlainText,
    }
    impl CellFormatHyperlinkDisplayType {
        pub fn as_str(self) -> &'static str {
            match self {
                CellFormatHyperlinkDisplayType::HyperlinkDisplayTypeUnspecified => {
                    "HYPERLINK_DISPLAY_TYPE_UNSPECIFIED"
                }
                CellFormatHyperlinkDisplayType::Linked => "LINKED",
                CellFormatHyperlinkDisplayType::PlainText => "PLAIN_TEXT",
            }
        }
    }
    impl ::std::convert::AsRef<str> for CellFormatHyperlinkDisplayType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CellFormatHyperlinkDisplayType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<CellFormatHyperlinkDisplayType, ()> {
            Ok(match s {
                "HYPERLINK_DISPLAY_TYPE_UNSPECIFIED" => {
                    CellFormatHyperlinkDisplayType::HyperlinkDisplayTypeUnspecified
                }
                "LINKED" => CellFormatHyperlinkDisplayType::Linked,
                "PLAIN_TEXT" => CellFormatHyperlinkDisplayType::PlainText,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for CellFormatHyperlinkDisplayType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CellFormatHyperlinkDisplayType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CellFormatHyperlinkDisplayType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "HYPERLINK_DISPLAY_TYPE_UNSPECIFIED" => {
                    CellFormatHyperlinkDisplayType::HyperlinkDisplayTypeUnspecified
                }
                "LINKED" => CellFormatHyperlinkDisplayType::Linked,
                "PLAIN_TEXT" => CellFormatHyperlinkDisplayType::PlainText,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for CellFormatHyperlinkDisplayType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CellFormatHyperlinkDisplayType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CellFormatTextDirection {
        #[doc = "The text direction of left-to-right was set by the user."]
        LeftToRight,
        #[doc = "The text direction of right-to-left was set by the user."]
        RightToLeft,
        #[doc = "The text direction is not specified. Do not use this."]
        TextDirectionUnspecified,
    }
    impl CellFormatTextDirection {
        pub fn as_str(self) -> &'static str {
            match self {
                CellFormatTextDirection::LeftToRight => "LEFT_TO_RIGHT",
                CellFormatTextDirection::RightToLeft => "RIGHT_TO_LEFT",
                CellFormatTextDirection::TextDirectionUnspecified => "TEXT_DIRECTION_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for CellFormatTextDirection {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CellFormatTextDirection {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<CellFormatTextDirection, ()> {
            Ok(match s {
                "LEFT_TO_RIGHT" => CellFormatTextDirection::LeftToRight,
                "RIGHT_TO_LEFT" => CellFormatTextDirection::RightToLeft,
                "TEXT_DIRECTION_UNSPECIFIED" => CellFormatTextDirection::TextDirectionUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for CellFormatTextDirection {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CellFormatTextDirection {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CellFormatTextDirection {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LEFT_TO_RIGHT" => CellFormatTextDirection::LeftToRight,
                "RIGHT_TO_LEFT" => CellFormatTextDirection::RightToLeft,
                "TEXT_DIRECTION_UNSPECIFIED" => CellFormatTextDirection::TextDirectionUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for CellFormatTextDirection {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CellFormatTextDirection {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CellFormatVerticalAlignment {
        #[doc = "The text is explicitly aligned to the bottom of the cell."]
        Bottom,
        #[doc = "The text is explicitly aligned to the middle of the cell."]
        Middle,
        #[doc = "The text is explicitly aligned to the top of the cell."]
        Top,
        #[doc = "The vertical alignment is not specified.  Do not use this."]
        VerticalAlignUnspecified,
    }
    impl CellFormatVerticalAlignment {
        pub fn as_str(self) -> &'static str {
            match self {
                CellFormatVerticalAlignment::Bottom => "BOTTOM",
                CellFormatVerticalAlignment::Middle => "MIDDLE",
                CellFormatVerticalAlignment::Top => "TOP",
                CellFormatVerticalAlignment::VerticalAlignUnspecified => {
                    "VERTICAL_ALIGN_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for CellFormatVerticalAlignment {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CellFormatVerticalAlignment {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<CellFormatVerticalAlignment, ()> {
            Ok(match s {
                "BOTTOM" => CellFormatVerticalAlignment::Bottom,
                "MIDDLE" => CellFormatVerticalAlignment::Middle,
                "TOP" => CellFormatVerticalAlignment::Top,
                "VERTICAL_ALIGN_UNSPECIFIED" => {
                    CellFormatVerticalAlignment::VerticalAlignUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for CellFormatVerticalAlignment {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CellFormatVerticalAlignment {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CellFormatVerticalAlignment {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BOTTOM" => CellFormatVerticalAlignment::Bottom,
                "MIDDLE" => CellFormatVerticalAlignment::Middle,
                "TOP" => CellFormatVerticalAlignment::Top,
                "VERTICAL_ALIGN_UNSPECIFIED" => {
                    CellFormatVerticalAlignment::VerticalAlignUnspecified
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
    impl ::google_field_selector::FieldSelector for CellFormatVerticalAlignment {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CellFormatVerticalAlignment {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CellFormatWrapStrategy {
        #[doc = "Lines that are longer than the cell width will be clipped.\nThe text will never wrap to the next line unless the user manually\ninserts a new line.\nExample:\n\n````text\n| First sentence. |\n| Manual newline t| <- Text is clipped\n| Next newline.   |````"]
        Clip,
        #[doc = "This wrap strategy represents the old Google Sheets wrap strategy where\nwords that are longer than a line are clipped rather than broken. This\nstrategy is not supported on all platforms and is being phased out.\nExample:\n\n````text\n| Cell has a |\n| loooooooooo| <- Word is clipped.\n| word.      |````"]
        LegacyWrap,
        #[doc = "Lines that are longer than the cell width will be written in the next\ncell over, so long as that cell is empty. If the next cell over is\nnon-empty, this behaves the same as CLIP. The text will never wrap\nto the next line unless the user manually inserts a new line.\nExample:\n\n````text\n| First sentence. |\n| Manual newline that is very long. <- Text continues into next cell\n| Next newline.   |````"]
        OverflowCell,
        #[doc = "Words that are longer than a line are wrapped at the character level\nrather than clipped.\nExample:\n\n````text\n| Cell has a |\n| loooooooooo| <- Word is broken.\n| ong word.  |````"]
        Wrap,
        #[doc = "The default value, do not use."]
        WrapStrategyUnspecified,
    }
    impl CellFormatWrapStrategy {
        pub fn as_str(self) -> &'static str {
            match self {
                CellFormatWrapStrategy::Clip => "CLIP",
                CellFormatWrapStrategy::LegacyWrap => "LEGACY_WRAP",
                CellFormatWrapStrategy::OverflowCell => "OVERFLOW_CELL",
                CellFormatWrapStrategy::Wrap => "WRAP",
                CellFormatWrapStrategy::WrapStrategyUnspecified => "WRAP_STRATEGY_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for CellFormatWrapStrategy {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CellFormatWrapStrategy {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<CellFormatWrapStrategy, ()> {
            Ok(match s {
                "CLIP" => CellFormatWrapStrategy::Clip,
                "LEGACY_WRAP" => CellFormatWrapStrategy::LegacyWrap,
                "OVERFLOW_CELL" => CellFormatWrapStrategy::OverflowCell,
                "WRAP" => CellFormatWrapStrategy::Wrap,
                "WRAP_STRATEGY_UNSPECIFIED" => CellFormatWrapStrategy::WrapStrategyUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for CellFormatWrapStrategy {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CellFormatWrapStrategy {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CellFormatWrapStrategy {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CLIP" => CellFormatWrapStrategy::Clip,
                "LEGACY_WRAP" => CellFormatWrapStrategy::LegacyWrap,
                "OVERFLOW_CELL" => CellFormatWrapStrategy::OverflowCell,
                "WRAP" => CellFormatWrapStrategy::Wrap,
                "WRAP_STRATEGY_UNSPECIFIED" => CellFormatWrapStrategy::WrapStrategyUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for CellFormatWrapStrategy {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CellFormatWrapStrategy {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ChartAxisViewWindowOptions {
        #[doc = "The maximum numeric value to be shown in this view window. If unset, will\nautomatically determine a maximum value that looks good for the data."]
        #[serde(
            rename = "viewWindowMax",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub view_window_max: ::std::option::Option<f64>,
        #[doc = "The minimum numeric value to be shown in this view window. If unset, will\nautomatically determine a minimum value that looks good for the data."]
        #[serde(
            rename = "viewWindowMin",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub view_window_min: ::std::option::Option<f64>,
        #[doc = "The view window's mode."]
        #[serde(
            rename = "viewWindowMode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub view_window_mode:
            ::std::option::Option<crate::schemas::ChartAxisViewWindowOptionsViewWindowMode>,
    }
    impl ::google_field_selector::FieldSelector for ChartAxisViewWindowOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ChartAxisViewWindowOptions {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ChartAxisViewWindowOptionsViewWindowMode {
        #[doc = "The default view window mode used in the Sheets editor for this chart\ntype. In most cases, if set, the default mode is equivalent to\n`PRETTY`."]
        DefaultViewWindowMode,
        #[doc = "Follows the min and max exactly if specified. If a value is unspecified,\nit will fall back to the `PRETTY` value."]
        Explicit,
        #[doc = "Chooses a min and max that make the chart look good. Both min and max are\nignored in this mode."]
        Pretty,
        #[doc = "Do not use. Represents that the currently set mode is not supported by\nthe API."]
        ViewWindowModeUnsupported,
    }
    impl ChartAxisViewWindowOptionsViewWindowMode {
        pub fn as_str(self) -> &'static str {
            match self {
                ChartAxisViewWindowOptionsViewWindowMode::DefaultViewWindowMode => {
                    "DEFAULT_VIEW_WINDOW_MODE"
                }
                ChartAxisViewWindowOptionsViewWindowMode::Explicit => "EXPLICIT",
                ChartAxisViewWindowOptionsViewWindowMode::Pretty => "PRETTY",
                ChartAxisViewWindowOptionsViewWindowMode::ViewWindowModeUnsupported => {
                    "VIEW_WINDOW_MODE_UNSUPPORTED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for ChartAxisViewWindowOptionsViewWindowMode {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ChartAxisViewWindowOptionsViewWindowMode {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<ChartAxisViewWindowOptionsViewWindowMode, ()> {
            Ok(match s {
                "DEFAULT_VIEW_WINDOW_MODE" => {
                    ChartAxisViewWindowOptionsViewWindowMode::DefaultViewWindowMode
                }
                "EXPLICIT" => ChartAxisViewWindowOptionsViewWindowMode::Explicit,
                "PRETTY" => ChartAxisViewWindowOptionsViewWindowMode::Pretty,
                "VIEW_WINDOW_MODE_UNSUPPORTED" => {
                    ChartAxisViewWindowOptionsViewWindowMode::ViewWindowModeUnsupported
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ChartAxisViewWindowOptionsViewWindowMode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ChartAxisViewWindowOptionsViewWindowMode {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ChartAxisViewWindowOptionsViewWindowMode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DEFAULT_VIEW_WINDOW_MODE" => {
                    ChartAxisViewWindowOptionsViewWindowMode::DefaultViewWindowMode
                }
                "EXPLICIT" => ChartAxisViewWindowOptionsViewWindowMode::Explicit,
                "PRETTY" => ChartAxisViewWindowOptionsViewWindowMode::Pretty,
                "VIEW_WINDOW_MODE_UNSUPPORTED" => {
                    ChartAxisViewWindowOptionsViewWindowMode::ViewWindowModeUnsupported
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
    impl ::google_field_selector::FieldSelector for ChartAxisViewWindowOptionsViewWindowMode {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ChartAxisViewWindowOptionsViewWindowMode {
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
    pub struct ChartCustomNumberFormatOptions {
        #[doc = "Custom prefix to be prepended to the chart attribute.\nThis field is optional."]
        #[serde(
            rename = "prefix",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub prefix: ::std::option::Option<String>,
        #[doc = "Custom suffix to be appended to the chart attribute.\nThis field is optional."]
        #[serde(
            rename = "suffix",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub suffix: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ChartCustomNumberFormatOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ChartCustomNumberFormatOptions {
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
    pub struct ChartData {
        #[doc = "The source ranges of the data."]
        #[serde(
            rename = "sourceRange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source_range: ::std::option::Option<crate::schemas::ChartSourceRange>,
    }
    impl ::google_field_selector::FieldSelector for ChartData {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ChartData {
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
    pub struct ChartSourceRange {
        #[doc = "The ranges of data for a series or domain.\nExactly one dimension must have a length of 1,\nand all sources in the list must have the same dimension\nwith length 1.\nThe domain (if it exists) & all series must have the same number\nof source ranges. If using more than one source range, then the source\nrange at a given offset must be in order and contiguous across the domain\nand series.\n\nFor example, these are valid configurations:\n\n````text\ndomain sources: A1:A5\nseries1 sources: B1:B5\nseries2 sources: D6:D10\n\ndomain sources: A1:A5, C10:C12\nseries1 sources: B1:B5, D10:D12\nseries2 sources: C1:C5, E10:E12````"]
        #[serde(
            rename = "sources",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sources: ::std::option::Option<Vec<crate::schemas::GridRange>>,
    }
    impl ::google_field_selector::FieldSelector for ChartSourceRange {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ChartSourceRange {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ChartSpec {
        #[doc = "The alternative text that describes the chart.  This is often used\nfor accessibility."]
        #[serde(
            rename = "altText",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub alt_text: ::std::option::Option<String>,
        #[doc = "The background color of the entire chart.\nNot applicable to Org charts."]
        #[serde(
            rename = "backgroundColor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub background_color: ::std::option::Option<crate::schemas::Color>,
        #[doc = "The background color of the entire chart.\nNot applicable to Org charts.\nIf background_color is also set, this field takes precedence."]
        #[serde(
            rename = "backgroundColorStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub background_color_style: ::std::option::Option<crate::schemas::ColorStyle>,
        #[doc = "A basic chart specification, can be one of many kinds of charts.\nSee BasicChartType for the list of all\ncharts this supports."]
        #[serde(
            rename = "basicChart",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub basic_chart: ::std::option::Option<crate::schemas::BasicChartSpec>,
        #[doc = "A bubble chart specification."]
        #[serde(
            rename = "bubbleChart",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bubble_chart: ::std::option::Option<crate::schemas::BubbleChartSpec>,
        #[doc = "A candlestick chart specification."]
        #[serde(
            rename = "candlestickChart",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub candlestick_chart: ::std::option::Option<crate::schemas::CandlestickChartSpec>,
        #[doc = "The name of the font to use by default for all chart text (e.g. title,\naxis labels, legend).  If a font is specified for a specific part of the\nchart it will override this font name."]
        #[serde(
            rename = "fontName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub font_name: ::std::option::Option<String>,
        #[doc = "Determines how the charts will use hidden rows or columns."]
        #[serde(
            rename = "hiddenDimensionStrategy",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub hidden_dimension_strategy:
            ::std::option::Option<crate::schemas::ChartSpecHiddenDimensionStrategy>,
        #[doc = "A histogram chart specification."]
        #[serde(
            rename = "histogramChart",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub histogram_chart: ::std::option::Option<crate::schemas::HistogramChartSpec>,
        #[doc = "True to make a chart fill the entire space in which it's rendered with\nminimum padding.  False to use the default padding.\n(Not applicable to Geo and Org charts.)"]
        #[serde(
            rename = "maximized",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub maximized: ::std::option::Option<bool>,
        #[doc = "An org chart specification."]
        #[serde(
            rename = "orgChart",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub org_chart: ::std::option::Option<crate::schemas::OrgChartSpec>,
        #[doc = "A pie chart specification."]
        #[serde(
            rename = "pieChart",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub pie_chart: ::std::option::Option<crate::schemas::PieChartSpec>,
        #[doc = "A scorecard chart specification."]
        #[serde(
            rename = "scorecardChart",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub scorecard_chart: ::std::option::Option<crate::schemas::ScorecardChartSpec>,
        #[doc = "The subtitle of the chart."]
        #[serde(
            rename = "subtitle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub subtitle: ::std::option::Option<String>,
        #[doc = "The subtitle text format.\nStrikethrough and underline are not supported."]
        #[serde(
            rename = "subtitleTextFormat",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub subtitle_text_format: ::std::option::Option<crate::schemas::TextFormat>,
        #[doc = "The subtitle text position.\nThis field is optional."]
        #[serde(
            rename = "subtitleTextPosition",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub subtitle_text_position: ::std::option::Option<crate::schemas::TextPosition>,
        #[doc = "The title of the chart."]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
        #[doc = "The title text format.\nStrikethrough and underline are not supported."]
        #[serde(
            rename = "titleTextFormat",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title_text_format: ::std::option::Option<crate::schemas::TextFormat>,
        #[doc = "The title text position.\nThis field is optional."]
        #[serde(
            rename = "titleTextPosition",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title_text_position: ::std::option::Option<crate::schemas::TextPosition>,
        #[doc = "A treemap chart specification."]
        #[serde(
            rename = "treemapChart",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub treemap_chart: ::std::option::Option<crate::schemas::TreemapChartSpec>,
        #[doc = "A waterfall chart specification."]
        #[serde(
            rename = "waterfallChart",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub waterfall_chart: ::std::option::Option<crate::schemas::WaterfallChartSpec>,
    }
    impl ::google_field_selector::FieldSelector for ChartSpec {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ChartSpec {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ChartSpecHiddenDimensionStrategy {
        #[doc = "Default value, do not use."]
        ChartHiddenDimensionStrategyUnspecified,
        #[doc = "Charts will not skip any hidden rows or columns."]
        ShowAll,
        #[doc = "Charts will skip hidden columns only."]
        SkipHiddenColumns,
        #[doc = "Charts will skip hidden rows only."]
        SkipHiddenRows,
        #[doc = "Charts will skip hidden rows and columns."]
        SkipHiddenRowsAndColumns,
    }
    impl ChartSpecHiddenDimensionStrategy {
        pub fn as_str(self) -> &'static str {
            match self {
                ChartSpecHiddenDimensionStrategy::ChartHiddenDimensionStrategyUnspecified => {
                    "CHART_HIDDEN_DIMENSION_STRATEGY_UNSPECIFIED"
                }
                ChartSpecHiddenDimensionStrategy::ShowAll => "SHOW_ALL",
                ChartSpecHiddenDimensionStrategy::SkipHiddenColumns => "SKIP_HIDDEN_COLUMNS",
                ChartSpecHiddenDimensionStrategy::SkipHiddenRows => "SKIP_HIDDEN_ROWS",
                ChartSpecHiddenDimensionStrategy::SkipHiddenRowsAndColumns => {
                    "SKIP_HIDDEN_ROWS_AND_COLUMNS"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for ChartSpecHiddenDimensionStrategy {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ChartSpecHiddenDimensionStrategy {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ChartSpecHiddenDimensionStrategy, ()> {
            Ok(match s {
                "CHART_HIDDEN_DIMENSION_STRATEGY_UNSPECIFIED" => {
                    ChartSpecHiddenDimensionStrategy::ChartHiddenDimensionStrategyUnspecified
                }
                "SHOW_ALL" => ChartSpecHiddenDimensionStrategy::ShowAll,
                "SKIP_HIDDEN_COLUMNS" => ChartSpecHiddenDimensionStrategy::SkipHiddenColumns,
                "SKIP_HIDDEN_ROWS" => ChartSpecHiddenDimensionStrategy::SkipHiddenRows,
                "SKIP_HIDDEN_ROWS_AND_COLUMNS" => {
                    ChartSpecHiddenDimensionStrategy::SkipHiddenRowsAndColumns
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ChartSpecHiddenDimensionStrategy {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ChartSpecHiddenDimensionStrategy {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ChartSpecHiddenDimensionStrategy {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CHART_HIDDEN_DIMENSION_STRATEGY_UNSPECIFIED" => {
                    ChartSpecHiddenDimensionStrategy::ChartHiddenDimensionStrategyUnspecified
                }
                "SHOW_ALL" => ChartSpecHiddenDimensionStrategy::ShowAll,
                "SKIP_HIDDEN_COLUMNS" => ChartSpecHiddenDimensionStrategy::SkipHiddenColumns,
                "SKIP_HIDDEN_ROWS" => ChartSpecHiddenDimensionStrategy::SkipHiddenRows,
                "SKIP_HIDDEN_ROWS_AND_COLUMNS" => {
                    ChartSpecHiddenDimensionStrategy::SkipHiddenRowsAndColumns
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
    impl ::google_field_selector::FieldSelector for ChartSpecHiddenDimensionStrategy {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ChartSpecHiddenDimensionStrategy {
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
    pub struct ClearBasicFilterRequest {
        #[doc = "The sheet ID on which the basic filter should be cleared."]
        #[serde(
            rename = "sheetId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sheet_id: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for ClearBasicFilterRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ClearBasicFilterRequest {
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
    pub struct ClearValuesRequest {}
    impl ::google_field_selector::FieldSelector for ClearValuesRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ClearValuesRequest {
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
    pub struct ClearValuesResponse {
        #[doc = "The range (in A1 notation) that was cleared.\n(If the request was for an unbounded range or a ranger larger\nthan the bounds of the sheet, this will be the actual range\nthat was cleared, bounded to the sheet's limits.)"]
        #[serde(
            rename = "clearedRange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cleared_range: ::std::option::Option<String>,
        #[doc = "The spreadsheet the updates were applied to."]
        #[serde(
            rename = "spreadsheetId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub spreadsheet_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ClearValuesResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ClearValuesResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Color {
        #[doc = "The fraction of this color that should be applied to the pixel. That is,\nthe final pixel color is defined by the equation:\n\npixel color = alpha * (this color) + (1.0 - alpha) * (background color)\n\nThis means that a value of 1.0 corresponds to a solid color, whereas\na value of 0.0 corresponds to a completely transparent color. This\nuses a wrapper message rather than a simple float scalar so that it is\npossible to distinguish between a default value and the value being unset.\nIf omitted, this color object is to be rendered as a solid color\n(as if the alpha value had been explicitly given with a value of 1.0)."]
        #[serde(
            rename = "alpha",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub alpha: ::std::option::Option<f32>,
        #[doc = "The amount of blue in the color as a value in the interval [0, 1]."]
        #[serde(
            rename = "blue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub blue: ::std::option::Option<f32>,
        #[doc = "The amount of green in the color as a value in the interval [0, 1]."]
        #[serde(
            rename = "green",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub green: ::std::option::Option<f32>,
        #[doc = "The amount of red in the color as a value in the interval [0, 1]."]
        #[serde(
            rename = "red",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub red: ::std::option::Option<f32>,
    }
    impl ::google_field_selector::FieldSelector for Color {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Color {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ColorStyle {
        #[doc = "RGB color."]
        #[serde(
            rename = "rgbColor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rgb_color: ::std::option::Option<crate::schemas::Color>,
        #[doc = "Theme color."]
        #[serde(
            rename = "themeColor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub theme_color: ::std::option::Option<crate::schemas::ColorStyleThemeColor>,
    }
    impl ::google_field_selector::FieldSelector for ColorStyle {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ColorStyle {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ColorStyleThemeColor {
        #[doc = "Represents the first accent color"]
        Accent1,
        #[doc = "Represents the second accent color"]
        Accent2,
        #[doc = "Represents the third accent color"]
        Accent3,
        #[doc = "Represents the fourth accent color"]
        Accent4,
        #[doc = "Represents the fifth accent color"]
        Accent5,
        #[doc = "Represents the sixth accent color"]
        Accent6,
        #[doc = "Represents the primary background color"]
        Background,
        #[doc = "Represents the color to use for hyperlinks"]
        Link,
        #[doc = "Represents the primary text color"]
        Text,
        #[doc = "Unspecified theme color"]
        ThemeColorTypeUnspecified,
    }
    impl ColorStyleThemeColor {
        pub fn as_str(self) -> &'static str {
            match self {
                ColorStyleThemeColor::Accent1 => "ACCENT1",
                ColorStyleThemeColor::Accent2 => "ACCENT2",
                ColorStyleThemeColor::Accent3 => "ACCENT3",
                ColorStyleThemeColor::Accent4 => "ACCENT4",
                ColorStyleThemeColor::Accent5 => "ACCENT5",
                ColorStyleThemeColor::Accent6 => "ACCENT6",
                ColorStyleThemeColor::Background => "BACKGROUND",
                ColorStyleThemeColor::Link => "LINK",
                ColorStyleThemeColor::Text => "TEXT",
                ColorStyleThemeColor::ThemeColorTypeUnspecified => "THEME_COLOR_TYPE_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ColorStyleThemeColor {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ColorStyleThemeColor {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ColorStyleThemeColor, ()> {
            Ok(match s {
                "ACCENT1" => ColorStyleThemeColor::Accent1,
                "ACCENT2" => ColorStyleThemeColor::Accent2,
                "ACCENT3" => ColorStyleThemeColor::Accent3,
                "ACCENT4" => ColorStyleThemeColor::Accent4,
                "ACCENT5" => ColorStyleThemeColor::Accent5,
                "ACCENT6" => ColorStyleThemeColor::Accent6,
                "BACKGROUND" => ColorStyleThemeColor::Background,
                "LINK" => ColorStyleThemeColor::Link,
                "TEXT" => ColorStyleThemeColor::Text,
                "THEME_COLOR_TYPE_UNSPECIFIED" => ColorStyleThemeColor::ThemeColorTypeUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ColorStyleThemeColor {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ColorStyleThemeColor {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ColorStyleThemeColor {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ACCENT1" => ColorStyleThemeColor::Accent1,
                "ACCENT2" => ColorStyleThemeColor::Accent2,
                "ACCENT3" => ColorStyleThemeColor::Accent3,
                "ACCENT4" => ColorStyleThemeColor::Accent4,
                "ACCENT5" => ColorStyleThemeColor::Accent5,
                "ACCENT6" => ColorStyleThemeColor::Accent6,
                "BACKGROUND" => ColorStyleThemeColor::Background,
                "LINK" => ColorStyleThemeColor::Link,
                "TEXT" => ColorStyleThemeColor::Text,
                "THEME_COLOR_TYPE_UNSPECIFIED" => ColorStyleThemeColor::ThemeColorTypeUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ColorStyleThemeColor {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ColorStyleThemeColor {
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
    pub struct ConditionValue {
        #[doc = "A relative date (based on the current date).\nValid only if the type is\nDATE_BEFORE,\nDATE_AFTER,\nDATE_ON_OR_BEFORE or\nDATE_ON_OR_AFTER.\n\nRelative dates are not supported in data validation.\nThey are supported only in conditional formatting and\nconditional filters."]
        #[serde(
            rename = "relativeDate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub relative_date: ::std::option::Option<crate::schemas::ConditionValueRelativeDate>,
        #[doc = "A value the condition is based on.\nThe value is parsed as if the user typed into a cell.\nFormulas are supported (and must begin with an `=` or a '+')."]
        #[serde(
            rename = "userEnteredValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub user_entered_value: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ConditionValue {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ConditionValue {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ConditionValueRelativeDate {
        #[doc = "The value is one month before today."]
        PastMonth,
        #[doc = "The value is one week before today."]
        PastWeek,
        #[doc = "The value is one year before today."]
        PastYear,
        #[doc = "Default value, do not use."]
        RelativeDateUnspecified,
        #[doc = "The value is today."]
        Today,
        #[doc = "The value is tomorrow."]
        Tomorrow,
        #[doc = "The value is yesterday."]
        Yesterday,
    }
    impl ConditionValueRelativeDate {
        pub fn as_str(self) -> &'static str {
            match self {
                ConditionValueRelativeDate::PastMonth => "PAST_MONTH",
                ConditionValueRelativeDate::PastWeek => "PAST_WEEK",
                ConditionValueRelativeDate::PastYear => "PAST_YEAR",
                ConditionValueRelativeDate::RelativeDateUnspecified => "RELATIVE_DATE_UNSPECIFIED",
                ConditionValueRelativeDate::Today => "TODAY",
                ConditionValueRelativeDate::Tomorrow => "TOMORROW",
                ConditionValueRelativeDate::Yesterday => "YESTERDAY",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ConditionValueRelativeDate {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ConditionValueRelativeDate {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ConditionValueRelativeDate, ()> {
            Ok(match s {
                "PAST_MONTH" => ConditionValueRelativeDate::PastMonth,
                "PAST_WEEK" => ConditionValueRelativeDate::PastWeek,
                "PAST_YEAR" => ConditionValueRelativeDate::PastYear,
                "RELATIVE_DATE_UNSPECIFIED" => ConditionValueRelativeDate::RelativeDateUnspecified,
                "TODAY" => ConditionValueRelativeDate::Today,
                "TOMORROW" => ConditionValueRelativeDate::Tomorrow,
                "YESTERDAY" => ConditionValueRelativeDate::Yesterday,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ConditionValueRelativeDate {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ConditionValueRelativeDate {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ConditionValueRelativeDate {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "PAST_MONTH" => ConditionValueRelativeDate::PastMonth,
                "PAST_WEEK" => ConditionValueRelativeDate::PastWeek,
                "PAST_YEAR" => ConditionValueRelativeDate::PastYear,
                "RELATIVE_DATE_UNSPECIFIED" => ConditionValueRelativeDate::RelativeDateUnspecified,
                "TODAY" => ConditionValueRelativeDate::Today,
                "TOMORROW" => ConditionValueRelativeDate::Tomorrow,
                "YESTERDAY" => ConditionValueRelativeDate::Yesterday,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ConditionValueRelativeDate {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ConditionValueRelativeDate {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ConditionalFormatRule {
        #[doc = "The formatting is either \"on\" or \"off\" according to the rule."]
        #[serde(
            rename = "booleanRule",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub boolean_rule: ::std::option::Option<crate::schemas::BooleanRule>,
        #[doc = "The formatting will vary based on the gradients in the rule."]
        #[serde(
            rename = "gradientRule",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub gradient_rule: ::std::option::Option<crate::schemas::GradientRule>,
        #[doc = "The ranges that are formatted if the condition is true.\nAll the ranges must be on the same grid."]
        #[serde(
            rename = "ranges",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ranges: ::std::option::Option<Vec<crate::schemas::GridRange>>,
    }
    impl ::google_field_selector::FieldSelector for ConditionalFormatRule {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ConditionalFormatRule {
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
    pub struct CopyPasteRequest {
        #[doc = "The location to paste to. If the range covers a span that's\na multiple of the source's height or width, then the\ndata will be repeated to fill in the destination range.\nIf the range is smaller than the source range, the entire\nsource data will still be copied (beyond the end of the destination range)."]
        #[serde(
            rename = "destination",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub destination: ::std::option::Option<crate::schemas::GridRange>,
        #[doc = "How that data should be oriented when pasting."]
        #[serde(
            rename = "pasteOrientation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub paste_orientation:
            ::std::option::Option<crate::schemas::CopyPasteRequestPasteOrientation>,
        #[doc = "What kind of data to paste."]
        #[serde(
            rename = "pasteType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub paste_type: ::std::option::Option<crate::schemas::CopyPasteRequestPasteType>,
        #[doc = "The source range to copy."]
        #[serde(
            rename = "source",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source: ::std::option::Option<crate::schemas::GridRange>,
    }
    impl ::google_field_selector::FieldSelector for CopyPasteRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CopyPasteRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CopyPasteRequestPasteOrientation {
        #[doc = "Paste normally."]
        Normal,
        #[doc = "Paste transposed, where all rows become columns and vice versa."]
        Transpose,
    }
    impl CopyPasteRequestPasteOrientation {
        pub fn as_str(self) -> &'static str {
            match self {
                CopyPasteRequestPasteOrientation::Normal => "NORMAL",
                CopyPasteRequestPasteOrientation::Transpose => "TRANSPOSE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for CopyPasteRequestPasteOrientation {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CopyPasteRequestPasteOrientation {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<CopyPasteRequestPasteOrientation, ()> {
            Ok(match s {
                "NORMAL" => CopyPasteRequestPasteOrientation::Normal,
                "TRANSPOSE" => CopyPasteRequestPasteOrientation::Transpose,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for CopyPasteRequestPasteOrientation {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CopyPasteRequestPasteOrientation {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CopyPasteRequestPasteOrientation {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "NORMAL" => CopyPasteRequestPasteOrientation::Normal,
                "TRANSPOSE" => CopyPasteRequestPasteOrientation::Transpose,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for CopyPasteRequestPasteOrientation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CopyPasteRequestPasteOrientation {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CopyPasteRequestPasteType {
        #[doc = "Paste the conditional formatting rules only."]
        PasteConditionalFormatting,
        #[doc = "Paste the data validation only."]
        PasteDataValidation,
        #[doc = "Paste the format and data validation only."]
        PasteFormat,
        #[doc = "Paste the formulas only."]
        PasteFormula,
        #[doc = "Like PASTE_NORMAL but without borders."]
        PasteNoBorders,
        #[doc = "Paste values, formulas, formats, and merges."]
        PasteNormal,
        #[doc = "Paste the values ONLY without formats, formulas, or merges."]
        PasteValues,
    }
    impl CopyPasteRequestPasteType {
        pub fn as_str(self) -> &'static str {
            match self {
                CopyPasteRequestPasteType::PasteConditionalFormatting => {
                    "PASTE_CONDITIONAL_FORMATTING"
                }
                CopyPasteRequestPasteType::PasteDataValidation => "PASTE_DATA_VALIDATION",
                CopyPasteRequestPasteType::PasteFormat => "PASTE_FORMAT",
                CopyPasteRequestPasteType::PasteFormula => "PASTE_FORMULA",
                CopyPasteRequestPasteType::PasteNoBorders => "PASTE_NO_BORDERS",
                CopyPasteRequestPasteType::PasteNormal => "PASTE_NORMAL",
                CopyPasteRequestPasteType::PasteValues => "PASTE_VALUES",
            }
        }
    }
    impl ::std::convert::AsRef<str> for CopyPasteRequestPasteType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CopyPasteRequestPasteType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<CopyPasteRequestPasteType, ()> {
            Ok(match s {
                "PASTE_CONDITIONAL_FORMATTING" => {
                    CopyPasteRequestPasteType::PasteConditionalFormatting
                }
                "PASTE_DATA_VALIDATION" => CopyPasteRequestPasteType::PasteDataValidation,
                "PASTE_FORMAT" => CopyPasteRequestPasteType::PasteFormat,
                "PASTE_FORMULA" => CopyPasteRequestPasteType::PasteFormula,
                "PASTE_NO_BORDERS" => CopyPasteRequestPasteType::PasteNoBorders,
                "PASTE_NORMAL" => CopyPasteRequestPasteType::PasteNormal,
                "PASTE_VALUES" => CopyPasteRequestPasteType::PasteValues,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for CopyPasteRequestPasteType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CopyPasteRequestPasteType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CopyPasteRequestPasteType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "PASTE_CONDITIONAL_FORMATTING" => {
                    CopyPasteRequestPasteType::PasteConditionalFormatting
                }
                "PASTE_DATA_VALIDATION" => CopyPasteRequestPasteType::PasteDataValidation,
                "PASTE_FORMAT" => CopyPasteRequestPasteType::PasteFormat,
                "PASTE_FORMULA" => CopyPasteRequestPasteType::PasteFormula,
                "PASTE_NO_BORDERS" => CopyPasteRequestPasteType::PasteNoBorders,
                "PASTE_NORMAL" => CopyPasteRequestPasteType::PasteNormal,
                "PASTE_VALUES" => CopyPasteRequestPasteType::PasteValues,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for CopyPasteRequestPasteType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CopyPasteRequestPasteType {
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
    pub struct CopySheetToAnotherSpreadsheetRequest {
        #[doc = "The ID of the spreadsheet to copy the sheet to."]
        #[serde(
            rename = "destinationSpreadsheetId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub destination_spreadsheet_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for CopySheetToAnotherSpreadsheetRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CopySheetToAnotherSpreadsheetRequest {
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
    pub struct CreateDeveloperMetadataRequest {
        #[doc = "The developer metadata to create."]
        #[serde(
            rename = "developerMetadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub developer_metadata: ::std::option::Option<crate::schemas::DeveloperMetadata>,
    }
    impl ::google_field_selector::FieldSelector for CreateDeveloperMetadataRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CreateDeveloperMetadataRequest {
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
    pub struct CreateDeveloperMetadataResponse {
        #[doc = "The developer metadata that was created."]
        #[serde(
            rename = "developerMetadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub developer_metadata: ::std::option::Option<crate::schemas::DeveloperMetadata>,
    }
    impl ::google_field_selector::FieldSelector for CreateDeveloperMetadataResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CreateDeveloperMetadataResponse {
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
    pub struct CutPasteRequest {
        #[doc = "The top-left coordinate where the data should be pasted."]
        #[serde(
            rename = "destination",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub destination: ::std::option::Option<crate::schemas::GridCoordinate>,
        #[doc = "What kind of data to paste.  All the source data will be cut, regardless\nof what is pasted."]
        #[serde(
            rename = "pasteType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub paste_type: ::std::option::Option<crate::schemas::CutPasteRequestPasteType>,
        #[doc = "The source data to cut."]
        #[serde(
            rename = "source",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source: ::std::option::Option<crate::schemas::GridRange>,
    }
    impl ::google_field_selector::FieldSelector for CutPasteRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CutPasteRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CutPasteRequestPasteType {
        #[doc = "Paste the conditional formatting rules only."]
        PasteConditionalFormatting,
        #[doc = "Paste the data validation only."]
        PasteDataValidation,
        #[doc = "Paste the format and data validation only."]
        PasteFormat,
        #[doc = "Paste the formulas only."]
        PasteFormula,
        #[doc = "Like PASTE_NORMAL but without borders."]
        PasteNoBorders,
        #[doc = "Paste values, formulas, formats, and merges."]
        PasteNormal,
        #[doc = "Paste the values ONLY without formats, formulas, or merges."]
        PasteValues,
    }
    impl CutPasteRequestPasteType {
        pub fn as_str(self) -> &'static str {
            match self {
                CutPasteRequestPasteType::PasteConditionalFormatting => {
                    "PASTE_CONDITIONAL_FORMATTING"
                }
                CutPasteRequestPasteType::PasteDataValidation => "PASTE_DATA_VALIDATION",
                CutPasteRequestPasteType::PasteFormat => "PASTE_FORMAT",
                CutPasteRequestPasteType::PasteFormula => "PASTE_FORMULA",
                CutPasteRequestPasteType::PasteNoBorders => "PASTE_NO_BORDERS",
                CutPasteRequestPasteType::PasteNormal => "PASTE_NORMAL",
                CutPasteRequestPasteType::PasteValues => "PASTE_VALUES",
            }
        }
    }
    impl ::std::convert::AsRef<str> for CutPasteRequestPasteType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CutPasteRequestPasteType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<CutPasteRequestPasteType, ()> {
            Ok(match s {
                "PASTE_CONDITIONAL_FORMATTING" => {
                    CutPasteRequestPasteType::PasteConditionalFormatting
                }
                "PASTE_DATA_VALIDATION" => CutPasteRequestPasteType::PasteDataValidation,
                "PASTE_FORMAT" => CutPasteRequestPasteType::PasteFormat,
                "PASTE_FORMULA" => CutPasteRequestPasteType::PasteFormula,
                "PASTE_NO_BORDERS" => CutPasteRequestPasteType::PasteNoBorders,
                "PASTE_NORMAL" => CutPasteRequestPasteType::PasteNormal,
                "PASTE_VALUES" => CutPasteRequestPasteType::PasteValues,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for CutPasteRequestPasteType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CutPasteRequestPasteType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CutPasteRequestPasteType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "PASTE_CONDITIONAL_FORMATTING" => {
                    CutPasteRequestPasteType::PasteConditionalFormatting
                }
                "PASTE_DATA_VALIDATION" => CutPasteRequestPasteType::PasteDataValidation,
                "PASTE_FORMAT" => CutPasteRequestPasteType::PasteFormat,
                "PASTE_FORMULA" => CutPasteRequestPasteType::PasteFormula,
                "PASTE_NO_BORDERS" => CutPasteRequestPasteType::PasteNoBorders,
                "PASTE_NORMAL" => CutPasteRequestPasteType::PasteNormal,
                "PASTE_VALUES" => CutPasteRequestPasteType::PasteValues,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for CutPasteRequestPasteType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CutPasteRequestPasteType {
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
    pub struct DataFilter {
        #[doc = "Selects data that matches the specified A1 range."]
        #[serde(
            rename = "a1Range",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub a_1_range: ::std::option::Option<String>,
        #[doc = "Selects data associated with the developer metadata matching the criteria\ndescribed by this DeveloperMetadataLookup."]
        #[serde(
            rename = "developerMetadataLookup",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub developer_metadata_lookup:
            ::std::option::Option<crate::schemas::DeveloperMetadataLookup>,
        #[doc = "Selects data that matches the range described by the GridRange."]
        #[serde(
            rename = "gridRange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub grid_range: ::std::option::Option<crate::schemas::GridRange>,
    }
    impl ::google_field_selector::FieldSelector for DataFilter {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DataFilter {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct DataFilterValueRange {
        #[doc = "The data filter describing the location of the values in the spreadsheet."]
        #[serde(
            rename = "dataFilter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub data_filter: ::std::option::Option<crate::schemas::DataFilter>,
        #[doc = "The major dimension of the values."]
        #[serde(
            rename = "majorDimension",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub major_dimension:
            ::std::option::Option<crate::schemas::DataFilterValueRangeMajorDimension>,
        #[doc = "The data to be written.  If the provided values exceed any of the ranges\nmatched by the data filter then the request fails.  If the provided values\nare less than the matched ranges only the specified values are written,\nexisting values in the matched ranges remain unaffected."]
        #[serde(
            rename = "values",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub values: ::std::option::Option<Vec<Vec<::serde_json::Value>>>,
    }
    impl ::google_field_selector::FieldSelector for DataFilterValueRange {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DataFilterValueRange {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DataFilterValueRangeMajorDimension {
        #[doc = "Operates on the columns of a sheet."]
        Columns,
        #[doc = "The default value, do not use."]
        DimensionUnspecified,
        #[doc = "Operates on the rows of a sheet."]
        Rows,
    }
    impl DataFilterValueRangeMajorDimension {
        pub fn as_str(self) -> &'static str {
            match self {
                DataFilterValueRangeMajorDimension::Columns => "COLUMNS",
                DataFilterValueRangeMajorDimension::DimensionUnspecified => "DIMENSION_UNSPECIFIED",
                DataFilterValueRangeMajorDimension::Rows => "ROWS",
            }
        }
    }
    impl ::std::convert::AsRef<str> for DataFilterValueRangeMajorDimension {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for DataFilterValueRangeMajorDimension {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<DataFilterValueRangeMajorDimension, ()> {
            Ok(match s {
                "COLUMNS" => DataFilterValueRangeMajorDimension::Columns,
                "DIMENSION_UNSPECIFIED" => DataFilterValueRangeMajorDimension::DimensionUnspecified,
                "ROWS" => DataFilterValueRangeMajorDimension::Rows,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for DataFilterValueRangeMajorDimension {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DataFilterValueRangeMajorDimension {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DataFilterValueRangeMajorDimension {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "COLUMNS" => DataFilterValueRangeMajorDimension::Columns,
                "DIMENSION_UNSPECIFIED" => DataFilterValueRangeMajorDimension::DimensionUnspecified,
                "ROWS" => DataFilterValueRangeMajorDimension::Rows,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for DataFilterValueRangeMajorDimension {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DataFilterValueRangeMajorDimension {
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
    pub struct DataValidationRule {
        #[doc = "The condition that data in the cell must match."]
        #[serde(
            rename = "condition",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub condition: ::std::option::Option<crate::schemas::BooleanCondition>,
        #[doc = "A message to show the user when adding data to the cell."]
        #[serde(
            rename = "inputMessage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub input_message: ::std::option::Option<String>,
        #[doc = "True if the UI should be customized based on the kind of condition.\nIf true, \"List\" conditions will show a dropdown."]
        #[serde(
            rename = "showCustomUi",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub show_custom_ui: ::std::option::Option<bool>,
        #[doc = "True if invalid data should be rejected."]
        #[serde(
            rename = "strict",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub strict: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for DataValidationRule {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DataValidationRule {
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
    pub struct DateTimeRule {
        #[doc = "The type of date-time grouping to apply."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::DateTimeRuleType>,
    }
    impl ::google_field_selector::FieldSelector for DateTimeRule {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DateTimeRule {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DateTimeRuleType {
        #[doc = "The default type, do not use."]
        DateTimeRuleTypeUnspecified,
        #[doc = "Group dates by day and month, for example 22-Nov. The month is\ntranslated based on the spreadsheet locale."]
        DayMonth,
        #[doc = "Group dates by day of month, from 1 to 31."]
        DayOfMonth,
        #[doc = "Group dates by day of week, for example Sunday. The days of the week will\nbe translated based on the spreadsheet locale."]
        DayOfWeek,
        #[doc = "Group dates by day of year, from 1 to 366. Note that dates after Feb. 29\nfall in different buckets in leap years than in non-leap years."]
        DayOfYear,
        #[doc = "Group dates by hour using a 24-hour system, from 0 to 23."]
        Hour,
        #[doc = "Group dates by hour and minute using a 24-hour system, for example 19:45."]
        HourMinute,
        #[doc = "Group dates by hour and minute using a 12-hour system, for example 7:45\nPM. The AM/PM designation is translated based on the spreadsheet\nlocale."]
        HourMinuteAmpm,
        #[doc = "Group dates by minute, from 0 to 59."]
        Minute,
        #[doc = "Group dates by month, for example Nov. The month is translated based\non the spreadsheet locale."]
        Month,
        #[doc = "Group dates by quarter, for example Q1 (which represents Jan-Mar)."]
        Quarter,
        #[doc = "Group dates by second, from 0 to 59."]
        Second,
        #[doc = "Group dates by year, for example 2008."]
        Year,
        #[doc = "Group dates by year and month, for example 2008-Nov. The month is\ntranslated based on the spreadsheet locale."]
        YearMonth,
        #[doc = "Group dates by year, month, and day, for example 2008-11-22."]
        YearMonthDay,
        #[doc = "Group dates by year and quarter, for example 2008 Q4."]
        YearQuarter,
    }
    impl DateTimeRuleType {
        pub fn as_str(self) -> &'static str {
            match self {
                DateTimeRuleType::DateTimeRuleTypeUnspecified => "DATE_TIME_RULE_TYPE_UNSPECIFIED",
                DateTimeRuleType::DayMonth => "DAY_MONTH",
                DateTimeRuleType::DayOfMonth => "DAY_OF_MONTH",
                DateTimeRuleType::DayOfWeek => "DAY_OF_WEEK",
                DateTimeRuleType::DayOfYear => "DAY_OF_YEAR",
                DateTimeRuleType::Hour => "HOUR",
                DateTimeRuleType::HourMinute => "HOUR_MINUTE",
                DateTimeRuleType::HourMinuteAmpm => "HOUR_MINUTE_AMPM",
                DateTimeRuleType::Minute => "MINUTE",
                DateTimeRuleType::Month => "MONTH",
                DateTimeRuleType::Quarter => "QUARTER",
                DateTimeRuleType::Second => "SECOND",
                DateTimeRuleType::Year => "YEAR",
                DateTimeRuleType::YearMonth => "YEAR_MONTH",
                DateTimeRuleType::YearMonthDay => "YEAR_MONTH_DAY",
                DateTimeRuleType::YearQuarter => "YEAR_QUARTER",
            }
        }
    }
    impl ::std::convert::AsRef<str> for DateTimeRuleType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for DateTimeRuleType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<DateTimeRuleType, ()> {
            Ok(match s {
                "DATE_TIME_RULE_TYPE_UNSPECIFIED" => DateTimeRuleType::DateTimeRuleTypeUnspecified,
                "DAY_MONTH" => DateTimeRuleType::DayMonth,
                "DAY_OF_MONTH" => DateTimeRuleType::DayOfMonth,
                "DAY_OF_WEEK" => DateTimeRuleType::DayOfWeek,
                "DAY_OF_YEAR" => DateTimeRuleType::DayOfYear,
                "HOUR" => DateTimeRuleType::Hour,
                "HOUR_MINUTE" => DateTimeRuleType::HourMinute,
                "HOUR_MINUTE_AMPM" => DateTimeRuleType::HourMinuteAmpm,
                "MINUTE" => DateTimeRuleType::Minute,
                "MONTH" => DateTimeRuleType::Month,
                "QUARTER" => DateTimeRuleType::Quarter,
                "SECOND" => DateTimeRuleType::Second,
                "YEAR" => DateTimeRuleType::Year,
                "YEAR_MONTH" => DateTimeRuleType::YearMonth,
                "YEAR_MONTH_DAY" => DateTimeRuleType::YearMonthDay,
                "YEAR_QUARTER" => DateTimeRuleType::YearQuarter,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for DateTimeRuleType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DateTimeRuleType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DateTimeRuleType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DATE_TIME_RULE_TYPE_UNSPECIFIED" => DateTimeRuleType::DateTimeRuleTypeUnspecified,
                "DAY_MONTH" => DateTimeRuleType::DayMonth,
                "DAY_OF_MONTH" => DateTimeRuleType::DayOfMonth,
                "DAY_OF_WEEK" => DateTimeRuleType::DayOfWeek,
                "DAY_OF_YEAR" => DateTimeRuleType::DayOfYear,
                "HOUR" => DateTimeRuleType::Hour,
                "HOUR_MINUTE" => DateTimeRuleType::HourMinute,
                "HOUR_MINUTE_AMPM" => DateTimeRuleType::HourMinuteAmpm,
                "MINUTE" => DateTimeRuleType::Minute,
                "MONTH" => DateTimeRuleType::Month,
                "QUARTER" => DateTimeRuleType::Quarter,
                "SECOND" => DateTimeRuleType::Second,
                "YEAR" => DateTimeRuleType::Year,
                "YEAR_MONTH" => DateTimeRuleType::YearMonth,
                "YEAR_MONTH_DAY" => DateTimeRuleType::YearMonthDay,
                "YEAR_QUARTER" => DateTimeRuleType::YearQuarter,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for DateTimeRuleType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DateTimeRuleType {
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
    pub struct DeleteBandingRequest {
        #[doc = "The ID of the banded range to delete."]
        #[serde(
            rename = "bandedRangeId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub banded_range_id: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for DeleteBandingRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DeleteBandingRequest {
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
    pub struct DeleteConditionalFormatRuleRequest {
        #[doc = "The zero-based index of the rule to be deleted."]
        #[serde(
            rename = "index",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub index: ::std::option::Option<i32>,
        #[doc = "The sheet the rule is being deleted from."]
        #[serde(
            rename = "sheetId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sheet_id: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for DeleteConditionalFormatRuleRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DeleteConditionalFormatRuleRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct DeleteConditionalFormatRuleResponse {
        #[doc = "The rule that was deleted."]
        #[serde(
            rename = "rule",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rule: ::std::option::Option<crate::schemas::ConditionalFormatRule>,
    }
    impl ::google_field_selector::FieldSelector for DeleteConditionalFormatRuleResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DeleteConditionalFormatRuleResponse {
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
    pub struct DeleteDeveloperMetadataRequest {
        #[doc = "The data filter describing the criteria used to select which developer\nmetadata entry to delete."]
        #[serde(
            rename = "dataFilter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub data_filter: ::std::option::Option<crate::schemas::DataFilter>,
    }
    impl ::google_field_selector::FieldSelector for DeleteDeveloperMetadataRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DeleteDeveloperMetadataRequest {
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
    pub struct DeleteDeveloperMetadataResponse {
        #[doc = "The metadata that was deleted."]
        #[serde(
            rename = "deletedDeveloperMetadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub deleted_developer_metadata:
            ::std::option::Option<Vec<crate::schemas::DeveloperMetadata>>,
    }
    impl ::google_field_selector::FieldSelector for DeleteDeveloperMetadataResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DeleteDeveloperMetadataResponse {
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
    pub struct DeleteDimensionGroupRequest {
        #[doc = "The range of the group to be deleted."]
        #[serde(
            rename = "range",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub range: ::std::option::Option<crate::schemas::DimensionRange>,
    }
    impl ::google_field_selector::FieldSelector for DeleteDimensionGroupRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DeleteDimensionGroupRequest {
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
    pub struct DeleteDimensionGroupResponse {
        #[doc = "All groups of a dimension after deleting a group from that dimension."]
        #[serde(
            rename = "dimensionGroups",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dimension_groups: ::std::option::Option<Vec<crate::schemas::DimensionGroup>>,
    }
    impl ::google_field_selector::FieldSelector for DeleteDimensionGroupResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DeleteDimensionGroupResponse {
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
    pub struct DeleteDimensionRequest {
        #[doc = "The dimensions to delete from the sheet."]
        #[serde(
            rename = "range",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub range: ::std::option::Option<crate::schemas::DimensionRange>,
    }
    impl ::google_field_selector::FieldSelector for DeleteDimensionRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DeleteDimensionRequest {
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
    pub struct DeleteDuplicatesRequest {
        #[doc = "The columns in the range to analyze for duplicate values. If no columns are\nselected then all columns are analyzed for duplicates."]
        #[serde(
            rename = "comparisonColumns",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub comparison_columns: ::std::option::Option<Vec<crate::schemas::DimensionRange>>,
        #[doc = "The range to remove duplicates rows from."]
        #[serde(
            rename = "range",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub range: ::std::option::Option<crate::schemas::GridRange>,
    }
    impl ::google_field_selector::FieldSelector for DeleteDuplicatesRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DeleteDuplicatesRequest {
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
    pub struct DeleteDuplicatesResponse {
        #[doc = "The number of duplicate rows removed."]
        #[serde(
            rename = "duplicatesRemovedCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub duplicates_removed_count: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for DeleteDuplicatesResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DeleteDuplicatesResponse {
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
    pub struct DeleteEmbeddedObjectRequest {
        #[doc = "The ID of the embedded object to delete."]
        #[serde(
            rename = "objectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub object_id: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for DeleteEmbeddedObjectRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DeleteEmbeddedObjectRequest {
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
    pub struct DeleteFilterViewRequest {
        #[doc = "The ID of the filter to delete."]
        #[serde(
            rename = "filterId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub filter_id: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for DeleteFilterViewRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DeleteFilterViewRequest {
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
    pub struct DeleteNamedRangeRequest {
        #[doc = "The ID of the named range to delete."]
        #[serde(
            rename = "namedRangeId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub named_range_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for DeleteNamedRangeRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DeleteNamedRangeRequest {
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
    pub struct DeleteProtectedRangeRequest {
        #[doc = "The ID of the protected range to delete."]
        #[serde(
            rename = "protectedRangeId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub protected_range_id: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for DeleteProtectedRangeRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DeleteProtectedRangeRequest {
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
    pub struct DeleteRangeRequest {
        #[doc = "The range of cells to delete."]
        #[serde(
            rename = "range",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub range: ::std::option::Option<crate::schemas::GridRange>,
        #[doc = "The dimension from which deleted cells will be replaced with.\nIf ROWS, existing cells will be shifted upward to\nreplace the deleted cells. If COLUMNS, existing cells\nwill be shifted left to replace the deleted cells."]
        #[serde(
            rename = "shiftDimension",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub shift_dimension:
            ::std::option::Option<crate::schemas::DeleteRangeRequestShiftDimension>,
    }
    impl ::google_field_selector::FieldSelector for DeleteRangeRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DeleteRangeRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DeleteRangeRequestShiftDimension {
        #[doc = "Operates on the columns of a sheet."]
        Columns,
        #[doc = "The default value, do not use."]
        DimensionUnspecified,
        #[doc = "Operates on the rows of a sheet."]
        Rows,
    }
    impl DeleteRangeRequestShiftDimension {
        pub fn as_str(self) -> &'static str {
            match self {
                DeleteRangeRequestShiftDimension::Columns => "COLUMNS",
                DeleteRangeRequestShiftDimension::DimensionUnspecified => "DIMENSION_UNSPECIFIED",
                DeleteRangeRequestShiftDimension::Rows => "ROWS",
            }
        }
    }
    impl ::std::convert::AsRef<str> for DeleteRangeRequestShiftDimension {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for DeleteRangeRequestShiftDimension {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<DeleteRangeRequestShiftDimension, ()> {
            Ok(match s {
                "COLUMNS" => DeleteRangeRequestShiftDimension::Columns,
                "DIMENSION_UNSPECIFIED" => DeleteRangeRequestShiftDimension::DimensionUnspecified,
                "ROWS" => DeleteRangeRequestShiftDimension::Rows,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for DeleteRangeRequestShiftDimension {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DeleteRangeRequestShiftDimension {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DeleteRangeRequestShiftDimension {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "COLUMNS" => DeleteRangeRequestShiftDimension::Columns,
                "DIMENSION_UNSPECIFIED" => DeleteRangeRequestShiftDimension::DimensionUnspecified,
                "ROWS" => DeleteRangeRequestShiftDimension::Rows,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for DeleteRangeRequestShiftDimension {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DeleteRangeRequestShiftDimension {
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
    pub struct DeleteSheetRequest {
        #[doc = "The ID of the sheet to delete."]
        #[serde(
            rename = "sheetId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sheet_id: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for DeleteSheetRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DeleteSheetRequest {
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
    pub struct DeveloperMetadata {
        #[doc = "The location where the metadata is associated."]
        #[serde(
            rename = "location",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub location: ::std::option::Option<crate::schemas::DeveloperMetadataLocation>,
        #[doc = "The spreadsheet-scoped unique ID that identifies the metadata. IDs may be\nspecified when metadata is created, otherwise one will be randomly\ngenerated and assigned. Must be positive."]
        #[serde(
            rename = "metadataId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metadata_id: ::std::option::Option<i32>,
        #[doc = "The metadata key. There may be multiple metadata in a spreadsheet with the\nsame key.  Developer metadata must always have a key specified."]
        #[serde(
            rename = "metadataKey",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metadata_key: ::std::option::Option<String>,
        #[doc = "Data associated with the metadata's key."]
        #[serde(
            rename = "metadataValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metadata_value: ::std::option::Option<String>,
        #[doc = "The metadata visibility.  Developer metadata must always have a visibility\nspecified."]
        #[serde(
            rename = "visibility",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub visibility: ::std::option::Option<crate::schemas::DeveloperMetadataVisibility>,
    }
    impl ::google_field_selector::FieldSelector for DeveloperMetadata {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DeveloperMetadata {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DeveloperMetadataVisibility {
        #[doc = "Default value."]
        DeveloperMetadataVisibilityUnspecified,
        #[doc = "Document-visible metadata is accessible from any developer project with\naccess to the document."]
        Document,
        #[doc = "Project-visible metadata is only visible to and accessible by the developer\nproject that created the metadata."]
        Project,
    }
    impl DeveloperMetadataVisibility {
        pub fn as_str(self) -> &'static str {
            match self {
                DeveloperMetadataVisibility::DeveloperMetadataVisibilityUnspecified => {
                    "DEVELOPER_METADATA_VISIBILITY_UNSPECIFIED"
                }
                DeveloperMetadataVisibility::Document => "DOCUMENT",
                DeveloperMetadataVisibility::Project => "PROJECT",
            }
        }
    }
    impl ::std::convert::AsRef<str> for DeveloperMetadataVisibility {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for DeveloperMetadataVisibility {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<DeveloperMetadataVisibility, ()> {
            Ok(match s {
                "DEVELOPER_METADATA_VISIBILITY_UNSPECIFIED" => {
                    DeveloperMetadataVisibility::DeveloperMetadataVisibilityUnspecified
                }
                "DOCUMENT" => DeveloperMetadataVisibility::Document,
                "PROJECT" => DeveloperMetadataVisibility::Project,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for DeveloperMetadataVisibility {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DeveloperMetadataVisibility {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DeveloperMetadataVisibility {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DEVELOPER_METADATA_VISIBILITY_UNSPECIFIED" => {
                    DeveloperMetadataVisibility::DeveloperMetadataVisibilityUnspecified
                }
                "DOCUMENT" => DeveloperMetadataVisibility::Document,
                "PROJECT" => DeveloperMetadataVisibility::Project,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for DeveloperMetadataVisibility {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DeveloperMetadataVisibility {
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
    pub struct DeveloperMetadataLocation {
        #[doc = "Represents the row or column when metadata is associated with\na dimension. The specified DimensionRange must represent a single row\nor column; it cannot be unbounded or span multiple rows or columns."]
        #[serde(
            rename = "dimensionRange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dimension_range: ::std::option::Option<crate::schemas::DimensionRange>,
        #[doc = "The type of location this object represents.  This field is read-only."]
        #[serde(
            rename = "locationType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub location_type:
            ::std::option::Option<crate::schemas::DeveloperMetadataLocationLocationType>,
        #[doc = "The ID of the sheet when metadata is associated with an entire sheet."]
        #[serde(
            rename = "sheetId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sheet_id: ::std::option::Option<i32>,
        #[doc = "True when metadata is associated with an entire spreadsheet."]
        #[serde(
            rename = "spreadsheet",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub spreadsheet: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for DeveloperMetadataLocation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DeveloperMetadataLocation {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DeveloperMetadataLocationLocationType {
        #[doc = "Developer metadata associated on an entire column dimension."]
        Column,
        #[doc = "Default value."]
        DeveloperMetadataLocationTypeUnspecified,
        #[doc = "Developer metadata associated on an entire row dimension."]
        Row,
        #[doc = "Developer metadata associated on an entire sheet."]
        Sheet,
        #[doc = "Developer metadata associated on the entire spreadsheet."]
        Spreadsheet,
    }
    impl DeveloperMetadataLocationLocationType {
        pub fn as_str(self) -> &'static str {
            match self {
                DeveloperMetadataLocationLocationType::Column => "COLUMN",
                DeveloperMetadataLocationLocationType::DeveloperMetadataLocationTypeUnspecified => {
                    "DEVELOPER_METADATA_LOCATION_TYPE_UNSPECIFIED"
                }
                DeveloperMetadataLocationLocationType::Row => "ROW",
                DeveloperMetadataLocationLocationType::Sheet => "SHEET",
                DeveloperMetadataLocationLocationType::Spreadsheet => "SPREADSHEET",
            }
        }
    }
    impl ::std::convert::AsRef<str> for DeveloperMetadataLocationLocationType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for DeveloperMetadataLocationLocationType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<DeveloperMetadataLocationLocationType, ()> {
            Ok(match s {
                "COLUMN" => DeveloperMetadataLocationLocationType::Column,
                "DEVELOPER_METADATA_LOCATION_TYPE_UNSPECIFIED" => {
                    DeveloperMetadataLocationLocationType::DeveloperMetadataLocationTypeUnspecified
                }
                "ROW" => DeveloperMetadataLocationLocationType::Row,
                "SHEET" => DeveloperMetadataLocationLocationType::Sheet,
                "SPREADSHEET" => DeveloperMetadataLocationLocationType::Spreadsheet,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for DeveloperMetadataLocationLocationType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DeveloperMetadataLocationLocationType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DeveloperMetadataLocationLocationType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "COLUMN" => DeveloperMetadataLocationLocationType::Column,
                "DEVELOPER_METADATA_LOCATION_TYPE_UNSPECIFIED" => {
                    DeveloperMetadataLocationLocationType::DeveloperMetadataLocationTypeUnspecified
                }
                "ROW" => DeveloperMetadataLocationLocationType::Row,
                "SHEET" => DeveloperMetadataLocationLocationType::Sheet,
                "SPREADSHEET" => DeveloperMetadataLocationLocationType::Spreadsheet,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for DeveloperMetadataLocationLocationType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DeveloperMetadataLocationLocationType {
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
    pub struct DeveloperMetadataLookup {
        #[doc = "Determines how this lookup matches the location.  If this field is\nspecified as EXACT, only developer metadata associated on the exact\nlocation specified is matched.  If this field is specified to INTERSECTING,\ndeveloper metadata associated on intersecting locations is also\nmatched.  If left unspecified, this field assumes a default value of\nINTERSECTING.\nIf this field is specified, a metadataLocation\nmust also be specified."]
        #[serde(
            rename = "locationMatchingStrategy",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub location_matching_strategy:
            ::std::option::Option<crate::schemas::DeveloperMetadataLookupLocationMatchingStrategy>,
        #[doc = "Limits the selected developer metadata to those entries which are\nassociated with locations of the specified type.  For example, when this\nfield is specified as ROW this lookup\nonly considers developer metadata associated on rows.  If the field is left\nunspecified, all location types are considered.  This field cannot be\nspecified as SPREADSHEET when\nthe locationMatchingStrategy\nis specified as INTERSECTING or when the\nmetadataLocation is specified as a\nnon-spreadsheet location: spreadsheet metadata cannot intersect any other\ndeveloper metadata location.  This field also must be left unspecified when\nthe locationMatchingStrategy\nis specified as EXACT."]
        #[serde(
            rename = "locationType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub location_type:
            ::std::option::Option<crate::schemas::DeveloperMetadataLookupLocationType>,
        #[doc = "Limits the selected developer metadata to that which has a matching\nDeveloperMetadata.metadata_id."]
        #[serde(
            rename = "metadataId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metadata_id: ::std::option::Option<i32>,
        #[doc = "Limits the selected developer metadata to that which has a matching\nDeveloperMetadata.metadata_key."]
        #[serde(
            rename = "metadataKey",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metadata_key: ::std::option::Option<String>,
        #[doc = "Limits the selected developer metadata to those entries associated with\nthe specified location.  This field either matches exact locations or all\nintersecting locations according the specified\nlocationMatchingStrategy."]
        #[serde(
            rename = "metadataLocation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metadata_location: ::std::option::Option<crate::schemas::DeveloperMetadataLocation>,
        #[doc = "Limits the selected developer metadata to that which has a matching\nDeveloperMetadata.metadata_value."]
        #[serde(
            rename = "metadataValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metadata_value: ::std::option::Option<String>,
        #[doc = "Limits the selected developer metadata to that which has a matching\nDeveloperMetadata.visibility.  If left unspecified, all developer\nmetadata visibile to the requesting project is considered."]
        #[serde(
            rename = "visibility",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub visibility: ::std::option::Option<crate::schemas::DeveloperMetadataLookupVisibility>,
    }
    impl ::google_field_selector::FieldSelector for DeveloperMetadataLookup {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DeveloperMetadataLookup {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DeveloperMetadataLookupLocationMatchingStrategy {
        #[doc = "Default value. This value must not be used."]
        DeveloperMetadataLocationMatchingStrategyUnspecified,
        #[doc = "Indicates that a specified location should be matched exactly.  For\nexample, if row three were specified as a location this matching strategy\nwould only match developer metadata also associated on row three.  Metadata\nassociated on other locations would not be considered."]
        ExactLocation,
        #[doc = "Indicates that a specified location should match that exact location as\nwell as any intersecting locations.  For example, if row three were\nspecified as a location this matching strategy would match developer\nmetadata associated on row three as well as metadata associated on\nlocations that intersect row three.  If, for instance, there was developer\nmetadata associated on column B, this matching strategy would also match\nthat location because column B intersects row three."]
        IntersectingLocation,
    }
    impl DeveloperMetadataLookupLocationMatchingStrategy {
        pub fn as_str(self) -> &'static str {
            match self { DeveloperMetadataLookupLocationMatchingStrategy :: DeveloperMetadataLocationMatchingStrategyUnspecified => "DEVELOPER_METADATA_LOCATION_MATCHING_STRATEGY_UNSPECIFIED" , DeveloperMetadataLookupLocationMatchingStrategy :: ExactLocation => "EXACT_LOCATION" , DeveloperMetadataLookupLocationMatchingStrategy :: IntersectingLocation => "INTERSECTING_LOCATION" , }
        }
    }
    impl ::std::convert::AsRef<str> for DeveloperMetadataLookupLocationMatchingStrategy {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for DeveloperMetadataLookupLocationMatchingStrategy {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<DeveloperMetadataLookupLocationMatchingStrategy, ()> {
            Ok ( match s { "DEVELOPER_METADATA_LOCATION_MATCHING_STRATEGY_UNSPECIFIED" => DeveloperMetadataLookupLocationMatchingStrategy :: DeveloperMetadataLocationMatchingStrategyUnspecified , "EXACT_LOCATION" => DeveloperMetadataLookupLocationMatchingStrategy :: ExactLocation , "INTERSECTING_LOCATION" => DeveloperMetadataLookupLocationMatchingStrategy :: IntersectingLocation , _ => return Err ( ( ) ) , } )
        }
    }
    impl ::std::fmt::Display for DeveloperMetadataLookupLocationMatchingStrategy {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DeveloperMetadataLookupLocationMatchingStrategy {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DeveloperMetadataLookupLocationMatchingStrategy {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok ( match value { "DEVELOPER_METADATA_LOCATION_MATCHING_STRATEGY_UNSPECIFIED" => DeveloperMetadataLookupLocationMatchingStrategy :: DeveloperMetadataLocationMatchingStrategyUnspecified , "EXACT_LOCATION" => DeveloperMetadataLookupLocationMatchingStrategy :: ExactLocation , "INTERSECTING_LOCATION" => DeveloperMetadataLookupLocationMatchingStrategy :: IntersectingLocation , _ => return Err ( :: serde :: de :: Error :: custom ( format ! ( "invalid enum for #name: {}" , value ) ) ) , } )
        }
    }
    impl ::google_field_selector::FieldSelector for DeveloperMetadataLookupLocationMatchingStrategy {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DeveloperMetadataLookupLocationMatchingStrategy {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DeveloperMetadataLookupLocationType {
        #[doc = "Developer metadata associated on an entire column dimension."]
        Column,
        #[doc = "Default value."]
        DeveloperMetadataLocationTypeUnspecified,
        #[doc = "Developer metadata associated on an entire row dimension."]
        Row,
        #[doc = "Developer metadata associated on an entire sheet."]
        Sheet,
        #[doc = "Developer metadata associated on the entire spreadsheet."]
        Spreadsheet,
    }
    impl DeveloperMetadataLookupLocationType {
        pub fn as_str(self) -> &'static str {
            match self {
                DeveloperMetadataLookupLocationType::Column => "COLUMN",
                DeveloperMetadataLookupLocationType::DeveloperMetadataLocationTypeUnspecified => {
                    "DEVELOPER_METADATA_LOCATION_TYPE_UNSPECIFIED"
                }
                DeveloperMetadataLookupLocationType::Row => "ROW",
                DeveloperMetadataLookupLocationType::Sheet => "SHEET",
                DeveloperMetadataLookupLocationType::Spreadsheet => "SPREADSHEET",
            }
        }
    }
    impl ::std::convert::AsRef<str> for DeveloperMetadataLookupLocationType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for DeveloperMetadataLookupLocationType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<DeveloperMetadataLookupLocationType, ()> {
            Ok(match s {
                "COLUMN" => DeveloperMetadataLookupLocationType::Column,
                "DEVELOPER_METADATA_LOCATION_TYPE_UNSPECIFIED" => {
                    DeveloperMetadataLookupLocationType::DeveloperMetadataLocationTypeUnspecified
                }
                "ROW" => DeveloperMetadataLookupLocationType::Row,
                "SHEET" => DeveloperMetadataLookupLocationType::Sheet,
                "SPREADSHEET" => DeveloperMetadataLookupLocationType::Spreadsheet,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for DeveloperMetadataLookupLocationType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DeveloperMetadataLookupLocationType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DeveloperMetadataLookupLocationType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "COLUMN" => DeveloperMetadataLookupLocationType::Column,
                "DEVELOPER_METADATA_LOCATION_TYPE_UNSPECIFIED" => {
                    DeveloperMetadataLookupLocationType::DeveloperMetadataLocationTypeUnspecified
                }
                "ROW" => DeveloperMetadataLookupLocationType::Row,
                "SHEET" => DeveloperMetadataLookupLocationType::Sheet,
                "SPREADSHEET" => DeveloperMetadataLookupLocationType::Spreadsheet,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for DeveloperMetadataLookupLocationType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DeveloperMetadataLookupLocationType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DeveloperMetadataLookupVisibility {
        #[doc = "Default value."]
        DeveloperMetadataVisibilityUnspecified,
        #[doc = "Document-visible metadata is accessible from any developer project with\naccess to the document."]
        Document,
        #[doc = "Project-visible metadata is only visible to and accessible by the developer\nproject that created the metadata."]
        Project,
    }
    impl DeveloperMetadataLookupVisibility {
        pub fn as_str(self) -> &'static str {
            match self {
                DeveloperMetadataLookupVisibility::DeveloperMetadataVisibilityUnspecified => {
                    "DEVELOPER_METADATA_VISIBILITY_UNSPECIFIED"
                }
                DeveloperMetadataLookupVisibility::Document => "DOCUMENT",
                DeveloperMetadataLookupVisibility::Project => "PROJECT",
            }
        }
    }
    impl ::std::convert::AsRef<str> for DeveloperMetadataLookupVisibility {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for DeveloperMetadataLookupVisibility {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<DeveloperMetadataLookupVisibility, ()> {
            Ok(match s {
                "DEVELOPER_METADATA_VISIBILITY_UNSPECIFIED" => {
                    DeveloperMetadataLookupVisibility::DeveloperMetadataVisibilityUnspecified
                }
                "DOCUMENT" => DeveloperMetadataLookupVisibility::Document,
                "PROJECT" => DeveloperMetadataLookupVisibility::Project,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for DeveloperMetadataLookupVisibility {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DeveloperMetadataLookupVisibility {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DeveloperMetadataLookupVisibility {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DEVELOPER_METADATA_VISIBILITY_UNSPECIFIED" => {
                    DeveloperMetadataLookupVisibility::DeveloperMetadataVisibilityUnspecified
                }
                "DOCUMENT" => DeveloperMetadataLookupVisibility::Document,
                "PROJECT" => DeveloperMetadataLookupVisibility::Project,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for DeveloperMetadataLookupVisibility {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DeveloperMetadataLookupVisibility {
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
    pub struct DimensionGroup {
        #[doc = "This field is true if this group is collapsed. A collapsed group remains\ncollapsed if an overlapping group at a shallower depth is expanded.\n\nA true value does not imply that all dimensions within the group are\nhidden, since a dimension's visibility can change independently from this\ngroup property. However, when this property is updated, all dimensions\nwithin it are set to hidden if this field is true, or set to visible if\nthis field is false."]
        #[serde(
            rename = "collapsed",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub collapsed: ::std::option::Option<bool>,
        #[doc = "The depth of the group, representing how many groups have a range that\nwholly contains the range of this group."]
        #[serde(
            rename = "depth",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub depth: ::std::option::Option<i32>,
        #[doc = "The range over which this group exists."]
        #[serde(
            rename = "range",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub range: ::std::option::Option<crate::schemas::DimensionRange>,
    }
    impl ::google_field_selector::FieldSelector for DimensionGroup {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DimensionGroup {
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
    pub struct DimensionProperties {
        #[doc = "The developer metadata associated with a single row or column."]
        #[serde(
            rename = "developerMetadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub developer_metadata: ::std::option::Option<Vec<crate::schemas::DeveloperMetadata>>,
        #[doc = "True if this dimension is being filtered.\nThis field is read-only."]
        #[serde(
            rename = "hiddenByFilter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub hidden_by_filter: ::std::option::Option<bool>,
        #[doc = "True if this dimension is explicitly hidden."]
        #[serde(
            rename = "hiddenByUser",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub hidden_by_user: ::std::option::Option<bool>,
        #[doc = "The height (if a row) or width (if a column) of the dimension in pixels."]
        #[serde(
            rename = "pixelSize",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub pixel_size: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for DimensionProperties {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DimensionProperties {
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
    pub struct DimensionRange {
        #[doc = "The dimension of the span."]
        #[serde(
            rename = "dimension",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dimension: ::std::option::Option<crate::schemas::DimensionRangeDimension>,
        #[doc = "The end (exclusive) of the span, or not set if unbounded."]
        #[serde(
            rename = "endIndex",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end_index: ::std::option::Option<i32>,
        #[doc = "The sheet this span is on."]
        #[serde(
            rename = "sheetId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sheet_id: ::std::option::Option<i32>,
        #[doc = "The start (inclusive) of the span, or not set if unbounded."]
        #[serde(
            rename = "startIndex",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_index: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for DimensionRange {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DimensionRange {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DimensionRangeDimension {
        #[doc = "Operates on the columns of a sheet."]
        Columns,
        #[doc = "The default value, do not use."]
        DimensionUnspecified,
        #[doc = "Operates on the rows of a sheet."]
        Rows,
    }
    impl DimensionRangeDimension {
        pub fn as_str(self) -> &'static str {
            match self {
                DimensionRangeDimension::Columns => "COLUMNS",
                DimensionRangeDimension::DimensionUnspecified => "DIMENSION_UNSPECIFIED",
                DimensionRangeDimension::Rows => "ROWS",
            }
        }
    }
    impl ::std::convert::AsRef<str> for DimensionRangeDimension {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for DimensionRangeDimension {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<DimensionRangeDimension, ()> {
            Ok(match s {
                "COLUMNS" => DimensionRangeDimension::Columns,
                "DIMENSION_UNSPECIFIED" => DimensionRangeDimension::DimensionUnspecified,
                "ROWS" => DimensionRangeDimension::Rows,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for DimensionRangeDimension {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DimensionRangeDimension {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DimensionRangeDimension {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "COLUMNS" => DimensionRangeDimension::Columns,
                "DIMENSION_UNSPECIFIED" => DimensionRangeDimension::DimensionUnspecified,
                "ROWS" => DimensionRangeDimension::Rows,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for DimensionRangeDimension {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DimensionRangeDimension {
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
    pub struct DuplicateFilterViewRequest {
        #[doc = "The ID of the filter being duplicated."]
        #[serde(
            rename = "filterId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub filter_id: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for DuplicateFilterViewRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DuplicateFilterViewRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct DuplicateFilterViewResponse {
        #[doc = "The newly created filter."]
        #[serde(
            rename = "filter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub filter: ::std::option::Option<crate::schemas::FilterView>,
    }
    impl ::google_field_selector::FieldSelector for DuplicateFilterViewResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DuplicateFilterViewResponse {
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
    pub struct DuplicateSheetRequest {
        #[doc = "The zero-based index where the new sheet should be inserted.\nThe index of all sheets after this are incremented."]
        #[serde(
            rename = "insertSheetIndex",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub insert_sheet_index: ::std::option::Option<i32>,
        #[doc = "If set, the ID of the new sheet. If not set, an ID is chosen.\nIf set, the ID must not conflict with any existing sheet ID.\nIf set, it must be non-negative."]
        #[serde(
            rename = "newSheetId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub new_sheet_id: ::std::option::Option<i32>,
        #[doc = "The name of the new sheet.  If empty, a new name is chosen for you."]
        #[serde(
            rename = "newSheetName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub new_sheet_name: ::std::option::Option<String>,
        #[doc = "The sheet to duplicate."]
        #[serde(
            rename = "sourceSheetId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source_sheet_id: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for DuplicateSheetRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DuplicateSheetRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct DuplicateSheetResponse {
        #[doc = "The properties of the duplicate sheet."]
        #[serde(
            rename = "properties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub properties: ::std::option::Option<crate::schemas::SheetProperties>,
    }
    impl ::google_field_selector::FieldSelector for DuplicateSheetResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DuplicateSheetResponse {
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
    pub struct Editors {
        #[doc = "True if anyone in the document's domain has edit access to the protected\nrange.  Domain protection is only supported on documents within a domain."]
        #[serde(
            rename = "domainUsersCanEdit",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub domain_users_can_edit: ::std::option::Option<bool>,
        #[doc = "The email addresses of groups with edit access to the protected range."]
        #[serde(
            rename = "groups",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub groups: ::std::option::Option<Vec<String>>,
        #[doc = "The email addresses of users with edit access to the protected range."]
        #[serde(
            rename = "users",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub users: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for Editors {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Editors {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct EmbeddedChart {
        #[doc = "The ID of the chart."]
        #[serde(
            rename = "chartId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub chart_id: ::std::option::Option<i32>,
        #[doc = "The position of the chart."]
        #[serde(
            rename = "position",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub position: ::std::option::Option<crate::schemas::EmbeddedObjectPosition>,
        #[doc = "The specification of the chart."]
        #[serde(
            rename = "spec",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub spec: ::std::option::Option<crate::schemas::ChartSpec>,
    }
    impl ::google_field_selector::FieldSelector for EmbeddedChart {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for EmbeddedChart {
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
    pub struct EmbeddedObjectPosition {
        #[doc = "If true, the embedded object is put on a new sheet whose ID\nis chosen for you. Used only when writing."]
        #[serde(
            rename = "newSheet",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub new_sheet: ::std::option::Option<bool>,
        #[doc = "The position at which the object is overlaid on top of a grid."]
        #[serde(
            rename = "overlayPosition",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub overlay_position: ::std::option::Option<crate::schemas::OverlayPosition>,
        #[doc = "The sheet this is on. Set only if the embedded object\nis on its own sheet. Must be non-negative."]
        #[serde(
            rename = "sheetId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sheet_id: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for EmbeddedObjectPosition {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for EmbeddedObjectPosition {
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
    pub struct ErrorValue {
        #[doc = "A message with more information about the error\n(in the spreadsheet's locale)."]
        #[serde(
            rename = "message",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub message: ::std::option::Option<String>,
        #[doc = "The type of error."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::ErrorValueType>,
    }
    impl ::google_field_selector::FieldSelector for ErrorValue {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ErrorValue {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ErrorValueType {
        #[doc = "Corresponds to the `#DIV/0` error."]
        DivideByZero,
        #[doc = "Corresponds to the `#ERROR!` error."]
        Error,
        #[doc = "The default error type, do not use this."]
        ErrorTypeUnspecified,
        #[doc = "Corresponds to the `Loading...` state."]
        Loading,
        #[doc = "Corresponds to the `#N/A` error."]
        NA,
        #[doc = "Corresponds to the `#NAME?` error."]
        Name,
        #[doc = "Corresponds to the `#NULL!` error."]
        NullValue,
        #[doc = "Corresponds to the `#NUM!` error."]
        Num,
        #[doc = "Corresponds to the `#REF!` error."]
        Ref,
        #[doc = "Corresponds to the `#VALUE!` error."]
        Value,
    }
    impl ErrorValueType {
        pub fn as_str(self) -> &'static str {
            match self {
                ErrorValueType::DivideByZero => "DIVIDE_BY_ZERO",
                ErrorValueType::Error => "ERROR",
                ErrorValueType::ErrorTypeUnspecified => "ERROR_TYPE_UNSPECIFIED",
                ErrorValueType::Loading => "LOADING",
                ErrorValueType::NA => "N_A",
                ErrorValueType::Name => "NAME",
                ErrorValueType::NullValue => "NULL_VALUE",
                ErrorValueType::Num => "NUM",
                ErrorValueType::Ref => "REF",
                ErrorValueType::Value => "VALUE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ErrorValueType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ErrorValueType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ErrorValueType, ()> {
            Ok(match s {
                "DIVIDE_BY_ZERO" => ErrorValueType::DivideByZero,
                "ERROR" => ErrorValueType::Error,
                "ERROR_TYPE_UNSPECIFIED" => ErrorValueType::ErrorTypeUnspecified,
                "LOADING" => ErrorValueType::Loading,
                "N_A" => ErrorValueType::NA,
                "NAME" => ErrorValueType::Name,
                "NULL_VALUE" => ErrorValueType::NullValue,
                "NUM" => ErrorValueType::Num,
                "REF" => ErrorValueType::Ref,
                "VALUE" => ErrorValueType::Value,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ErrorValueType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ErrorValueType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ErrorValueType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DIVIDE_BY_ZERO" => ErrorValueType::DivideByZero,
                "ERROR" => ErrorValueType::Error,
                "ERROR_TYPE_UNSPECIFIED" => ErrorValueType::ErrorTypeUnspecified,
                "LOADING" => ErrorValueType::Loading,
                "N_A" => ErrorValueType::NA,
                "NAME" => ErrorValueType::Name,
                "NULL_VALUE" => ErrorValueType::NullValue,
                "NUM" => ErrorValueType::Num,
                "REF" => ErrorValueType::Ref,
                "VALUE" => ErrorValueType::Value,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ErrorValueType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ErrorValueType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ExtendedValue {
        #[doc = "Represents a boolean value."]
        #[serde(
            rename = "boolValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bool_value: ::std::option::Option<bool>,
        #[doc = "Represents an error.\nThis field is read-only."]
        #[serde(
            rename = "errorValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub error_value: ::std::option::Option<crate::schemas::ErrorValue>,
        #[doc = "Represents a formula."]
        #[serde(
            rename = "formulaValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub formula_value: ::std::option::Option<String>,
        #[doc = "Represents a double value.\nNote: Dates, Times and DateTimes are represented as doubles in\n\"serial number\" format."]
        #[serde(
            rename = "numberValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub number_value: ::std::option::Option<f64>,
        #[doc = "Represents a string value.\nLeading single quotes are not included. For example, if the user typed\n`'123` into the UI, this would be represented as a `stringValue` of\n`\"123\"`."]
        #[serde(
            rename = "stringValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub string_value: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ExtendedValue {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ExtendedValue {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct FilterCriteria {
        #[doc = "A condition that must be true for values to be shown.\n(This does not override hidden_values -- if a value is listed there,\nit will still be hidden.)"]
        #[serde(
            rename = "condition",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub condition: ::std::option::Option<crate::schemas::BooleanCondition>,
        #[doc = "Values that should be hidden."]
        #[serde(
            rename = "hiddenValues",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub hidden_values: ::std::option::Option<Vec<String>>,
        #[doc = "The background fill color to filter by; only cells with this fill color are\nshown. Mutually exclusive with visible_foreground_color."]
        #[serde(
            rename = "visibleBackgroundColor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub visible_background_color: ::std::option::Option<crate::schemas::Color>,
        #[doc = "The background fill color to filter by; only cells with this fill color are\nshown. This field is mutually exclusive with visible_foreground_color,\nand must be set to an RGB-type color. If visible_background_color is\nalso set, this field takes precedence."]
        #[serde(
            rename = "visibleBackgroundColorStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub visible_background_color_style: ::std::option::Option<crate::schemas::ColorStyle>,
        #[doc = "The foreground color to filter by; only cells with this foreground color\nare shown. Mutually exclusive with visible_background_color."]
        #[serde(
            rename = "visibleForegroundColor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub visible_foreground_color: ::std::option::Option<crate::schemas::Color>,
        #[doc = "The foreground color to filter by; only cells with this foreground color\nare shown. This field is mutually exclusive with\nvisible_background_color, and must be set to an RGB-type color. If\nvisible_foreground_color is also set, this field takes precedence."]
        #[serde(
            rename = "visibleForegroundColorStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub visible_foreground_color_style: ::std::option::Option<crate::schemas::ColorStyle>,
    }
    impl ::google_field_selector::FieldSelector for FilterCriteria {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for FilterCriteria {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct FilterView {
        #[doc = "The criteria for showing/hiding values per column.\nThe map's key is the column index, and the value is the criteria for\nthat column."]
        #[serde(
            rename = "criteria",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub criteria: ::std::option::Option<
            ::std::collections::BTreeMap<String, crate::schemas::FilterCriteria>,
        >,
        #[doc = "The ID of the filter view."]
        #[serde(
            rename = "filterViewId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub filter_view_id: ::std::option::Option<i32>,
        #[doc = "The named range this filter view is backed by, if any.\n\nWhen writing, only one of range or named_range_id\nmay be set."]
        #[serde(
            rename = "namedRangeId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub named_range_id: ::std::option::Option<String>,
        #[doc = "The range this filter view covers.\n\nWhen writing, only one of range or named_range_id\nmay be set."]
        #[serde(
            rename = "range",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub range: ::std::option::Option<crate::schemas::GridRange>,
        #[doc = "The sort order per column. Later specifications are used when values\nare equal in the earlier specifications."]
        #[serde(
            rename = "sortSpecs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sort_specs: ::std::option::Option<Vec<crate::schemas::SortSpec>>,
        #[doc = "The name of the filter view."]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for FilterView {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for FilterView {
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
    pub struct FindReplaceRequest {
        #[doc = "True to find/replace over all sheets."]
        #[serde(
            rename = "allSheets",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub all_sheets: ::std::option::Option<bool>,
        #[doc = "The value to search."]
        #[serde(
            rename = "find",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub find: ::std::option::Option<String>,
        #[doc = "True if the search should include cells with formulas.\nFalse to skip cells with formulas."]
        #[serde(
            rename = "includeFormulas",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub include_formulas: ::std::option::Option<bool>,
        #[doc = "True if the search is case sensitive."]
        #[serde(
            rename = "matchCase",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub match_case: ::std::option::Option<bool>,
        #[doc = "True if the find value should match the entire cell."]
        #[serde(
            rename = "matchEntireCell",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub match_entire_cell: ::std::option::Option<bool>,
        #[doc = "The range to find/replace over."]
        #[serde(
            rename = "range",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub range: ::std::option::Option<crate::schemas::GridRange>,
        #[doc = "The value to use as the replacement."]
        #[serde(
            rename = "replacement",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub replacement: ::std::option::Option<String>,
        #[doc = "True if the find value is a regex.\nThe regular expression and replacement should follow Java regex rules\nat https://docs.oracle.com/javase/8/docs/api/java/util/regex/Pattern.html.\nThe replacement string is allowed to refer to capturing groups.\nFor example, if one cell has the contents `\"Google Sheets\"` and another\nhas `\"Google Docs\"`, then searching for `\"o.* (.*)\"` with a replacement of\n`\"$1 Rocks\"` would change the contents of the cells to\n`\"GSheets Rocks\"` and `\"GDocs Rocks\"` respectively."]
        #[serde(
            rename = "searchByRegex",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub search_by_regex: ::std::option::Option<bool>,
        #[doc = "The sheet to find/replace over."]
        #[serde(
            rename = "sheetId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sheet_id: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for FindReplaceRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for FindReplaceRequest {
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
    pub struct FindReplaceResponse {
        #[doc = "The number of formula cells changed."]
        #[serde(
            rename = "formulasChanged",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub formulas_changed: ::std::option::Option<i32>,
        #[doc = "The number of occurrences (possibly multiple within a cell) changed.\nFor example, if replacing `\"e\"` with `\"o\"` in `\"Google Sheets\"`, this would\nbe `\"3\"` because `\"Google Sheets\"` -> `\"Googlo Shoots\"`."]
        #[serde(
            rename = "occurrencesChanged",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub occurrences_changed: ::std::option::Option<i32>,
        #[doc = "The number of rows changed."]
        #[serde(
            rename = "rowsChanged",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rows_changed: ::std::option::Option<i32>,
        #[doc = "The number of sheets changed."]
        #[serde(
            rename = "sheetsChanged",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sheets_changed: ::std::option::Option<i32>,
        #[doc = "The number of non-formula cells changed."]
        #[serde(
            rename = "valuesChanged",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub values_changed: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for FindReplaceResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for FindReplaceResponse {
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
    pub struct GetSpreadsheetByDataFilterRequest {
        #[doc = "The DataFilters used to select which ranges to retrieve from\nthe spreadsheet."]
        #[serde(
            rename = "dataFilters",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub data_filters: ::std::option::Option<Vec<crate::schemas::DataFilter>>,
        #[doc = "True if grid data should be returned.\nThis parameter is ignored if a field mask was set in the request."]
        #[serde(
            rename = "includeGridData",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub include_grid_data: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for GetSpreadsheetByDataFilterRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GetSpreadsheetByDataFilterRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GradientRule {
        #[doc = "The final interpolation point."]
        #[serde(
            rename = "maxpoint",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub maxpoint: ::std::option::Option<crate::schemas::InterpolationPoint>,
        #[doc = "An optional midway interpolation point."]
        #[serde(
            rename = "midpoint",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub midpoint: ::std::option::Option<crate::schemas::InterpolationPoint>,
        #[doc = "The starting interpolation point."]
        #[serde(
            rename = "minpoint",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub minpoint: ::std::option::Option<crate::schemas::InterpolationPoint>,
    }
    impl ::google_field_selector::FieldSelector for GradientRule {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GradientRule {
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
    pub struct GridCoordinate {
        #[doc = "The column index of the coordinate."]
        #[serde(
            rename = "columnIndex",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub column_index: ::std::option::Option<i32>,
        #[doc = "The row index of the coordinate."]
        #[serde(
            rename = "rowIndex",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub row_index: ::std::option::Option<i32>,
        #[doc = "The sheet this coordinate is on."]
        #[serde(
            rename = "sheetId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sheet_id: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for GridCoordinate {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GridCoordinate {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GridData {
        #[doc = "Metadata about the requested columns in the grid, starting with the column\nin start_column."]
        #[serde(
            rename = "columnMetadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub column_metadata: ::std::option::Option<Vec<crate::schemas::DimensionProperties>>,
        #[doc = "The data in the grid, one entry per row,\nstarting with the row in startRow.\nThe values in RowData will correspond to columns starting\nat start_column."]
        #[serde(
            rename = "rowData",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub row_data: ::std::option::Option<Vec<crate::schemas::RowData>>,
        #[doc = "Metadata about the requested rows in the grid, starting with the row\nin start_row."]
        #[serde(
            rename = "rowMetadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub row_metadata: ::std::option::Option<Vec<crate::schemas::DimensionProperties>>,
        #[doc = "The first column this GridData refers to, zero-based."]
        #[serde(
            rename = "startColumn",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_column: ::std::option::Option<i32>,
        #[doc = "The first row this GridData refers to, zero-based."]
        #[serde(
            rename = "startRow",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_row: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for GridData {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GridData {
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
    pub struct GridProperties {
        #[doc = "The number of columns in the grid."]
        #[serde(
            rename = "columnCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub column_count: ::std::option::Option<i32>,
        #[doc = "True if the column grouping control toggle is shown after the group."]
        #[serde(
            rename = "columnGroupControlAfter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub column_group_control_after: ::std::option::Option<bool>,
        #[doc = "The number of columns that are frozen in the grid."]
        #[serde(
            rename = "frozenColumnCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub frozen_column_count: ::std::option::Option<i32>,
        #[doc = "The number of rows that are frozen in the grid."]
        #[serde(
            rename = "frozenRowCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub frozen_row_count: ::std::option::Option<i32>,
        #[doc = "True if the grid isn't showing gridlines in the UI."]
        #[serde(
            rename = "hideGridlines",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub hide_gridlines: ::std::option::Option<bool>,
        #[doc = "The number of rows in the grid."]
        #[serde(
            rename = "rowCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub row_count: ::std::option::Option<i32>,
        #[doc = "True if the row grouping control toggle is shown after the group."]
        #[serde(
            rename = "rowGroupControlAfter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub row_group_control_after: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for GridProperties {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GridProperties {
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
    pub struct GridRange {
        #[doc = "The end column (exclusive) of the range, or not set if unbounded."]
        #[serde(
            rename = "endColumnIndex",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end_column_index: ::std::option::Option<i32>,
        #[doc = "The end row (exclusive) of the range, or not set if unbounded."]
        #[serde(
            rename = "endRowIndex",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end_row_index: ::std::option::Option<i32>,
        #[doc = "The sheet this range is on."]
        #[serde(
            rename = "sheetId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sheet_id: ::std::option::Option<i32>,
        #[doc = "The start column (inclusive) of the range, or not set if unbounded."]
        #[serde(
            rename = "startColumnIndex",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_column_index: ::std::option::Option<i32>,
        #[doc = "The start row (inclusive) of the range, or not set if unbounded."]
        #[serde(
            rename = "startRowIndex",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_row_index: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for GridRange {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GridRange {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct HistogramChartSpec {
        #[doc = "By default the bucket size (the range of values stacked in a single\ncolumn) is chosen automatically, but it may be overridden here.\nE.g., A bucket size of 1.5 results in buckets from 0 - 1.5, 1.5 - 3.0, etc.\nCannot be negative.\nThis field is optional."]
        #[serde(
            rename = "bucketSize",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bucket_size: ::std::option::Option<f64>,
        #[doc = "The position of the chart legend."]
        #[serde(
            rename = "legendPosition",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub legend_position:
            ::std::option::Option<crate::schemas::HistogramChartSpecLegendPosition>,
        #[doc = "The outlier percentile is used to ensure that outliers do not adversely\naffect the calculation of bucket sizes.  For example, setting an outlier\npercentile of 0.05 indicates that the top and bottom 5% of values when\ncalculating buckets.  The values are still included in the chart, they will\nbe added to the first or last buckets instead of their own buckets.\nMust be between 0.0 and 0.5."]
        #[serde(
            rename = "outlierPercentile",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub outlier_percentile: ::std::option::Option<f64>,
        #[doc = "The series for a histogram may be either a single series of values to be\nbucketed or multiple series, each of the same length, containing the name\nof the series followed by the values to be bucketed for that series."]
        #[serde(
            rename = "series",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub series: ::std::option::Option<Vec<crate::schemas::HistogramSeries>>,
        #[doc = "Whether horizontal divider lines should be displayed between items in each\ncolumn."]
        #[serde(
            rename = "showItemDividers",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub show_item_dividers: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for HistogramChartSpec {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for HistogramChartSpec {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum HistogramChartSpecLegendPosition {
        #[doc = "The legend is rendered on the bottom of the chart."]
        BottomLegend,
        #[doc = "Default value, do not use."]
        HistogramChartLegendPositionUnspecified,
        #[doc = "The legend is rendered inside the chart area."]
        InsideLegend,
        #[doc = "The legend is rendered on the left of the chart."]
        LeftLegend,
        #[doc = "No legend is rendered."]
        NoLegend,
        #[doc = "The legend is rendered on the right of the chart."]
        RightLegend,
        #[doc = "The legend is rendered on the top of the chart."]
        TopLegend,
    }
    impl HistogramChartSpecLegendPosition {
        pub fn as_str(self) -> &'static str {
            match self {
                HistogramChartSpecLegendPosition::BottomLegend => "BOTTOM_LEGEND",
                HistogramChartSpecLegendPosition::HistogramChartLegendPositionUnspecified => {
                    "HISTOGRAM_CHART_LEGEND_POSITION_UNSPECIFIED"
                }
                HistogramChartSpecLegendPosition::InsideLegend => "INSIDE_LEGEND",
                HistogramChartSpecLegendPosition::LeftLegend => "LEFT_LEGEND",
                HistogramChartSpecLegendPosition::NoLegend => "NO_LEGEND",
                HistogramChartSpecLegendPosition::RightLegend => "RIGHT_LEGEND",
                HistogramChartSpecLegendPosition::TopLegend => "TOP_LEGEND",
            }
        }
    }
    impl ::std::convert::AsRef<str> for HistogramChartSpecLegendPosition {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for HistogramChartSpecLegendPosition {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<HistogramChartSpecLegendPosition, ()> {
            Ok(match s {
                "BOTTOM_LEGEND" => HistogramChartSpecLegendPosition::BottomLegend,
                "HISTOGRAM_CHART_LEGEND_POSITION_UNSPECIFIED" => {
                    HistogramChartSpecLegendPosition::HistogramChartLegendPositionUnspecified
                }
                "INSIDE_LEGEND" => HistogramChartSpecLegendPosition::InsideLegend,
                "LEFT_LEGEND" => HistogramChartSpecLegendPosition::LeftLegend,
                "NO_LEGEND" => HistogramChartSpecLegendPosition::NoLegend,
                "RIGHT_LEGEND" => HistogramChartSpecLegendPosition::RightLegend,
                "TOP_LEGEND" => HistogramChartSpecLegendPosition::TopLegend,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for HistogramChartSpecLegendPosition {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for HistogramChartSpecLegendPosition {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for HistogramChartSpecLegendPosition {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BOTTOM_LEGEND" => HistogramChartSpecLegendPosition::BottomLegend,
                "HISTOGRAM_CHART_LEGEND_POSITION_UNSPECIFIED" => {
                    HistogramChartSpecLegendPosition::HistogramChartLegendPositionUnspecified
                }
                "INSIDE_LEGEND" => HistogramChartSpecLegendPosition::InsideLegend,
                "LEFT_LEGEND" => HistogramChartSpecLegendPosition::LeftLegend,
                "NO_LEGEND" => HistogramChartSpecLegendPosition::NoLegend,
                "RIGHT_LEGEND" => HistogramChartSpecLegendPosition::RightLegend,
                "TOP_LEGEND" => HistogramChartSpecLegendPosition::TopLegend,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for HistogramChartSpecLegendPosition {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for HistogramChartSpecLegendPosition {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct HistogramRule {
        #[doc = "The maximum value at which items are placed into buckets\nof constant size. Values above end are lumped into a single bucket.\nThis field is optional."]
        #[serde(
            rename = "end",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end: ::std::option::Option<f64>,
        #[doc = "The size of the buckets that are created. Must be positive."]
        #[serde(
            rename = "interval",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub interval: ::std::option::Option<f64>,
        #[doc = "The minimum value at which items are placed into buckets\nof constant size. Values below start are lumped into a single bucket.\nThis field is optional."]
        #[serde(
            rename = "start",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start: ::std::option::Option<f64>,
    }
    impl ::google_field_selector::FieldSelector for HistogramRule {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for HistogramRule {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct HistogramSeries {
        #[doc = "The color of the column representing this series in each bucket.\nThis field is optional."]
        #[serde(
            rename = "barColor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bar_color: ::std::option::Option<crate::schemas::Color>,
        #[doc = "The color of the column representing this series in each bucket.\nThis field is optional.\nIf bar_color is also set, this field takes precedence."]
        #[serde(
            rename = "barColorStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bar_color_style: ::std::option::Option<crate::schemas::ColorStyle>,
        #[doc = "The data for this histogram series."]
        #[serde(
            rename = "data",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub data: ::std::option::Option<crate::schemas::ChartData>,
    }
    impl ::google_field_selector::FieldSelector for HistogramSeries {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for HistogramSeries {
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
    pub struct InsertDimensionRequest {
        #[doc = "Whether dimension properties should be extended from the dimensions\nbefore or after the newly inserted dimensions.\nTrue to inherit from the dimensions before (in which case the start\nindex must be greater than 0), and false to inherit from the dimensions\nafter.\n\nFor example, if row index 0 has red background and row index 1\nhas a green background, then inserting 2 rows at index 1 can inherit\neither the green or red background.  If `inheritFromBefore` is true,\nthe two new rows will be red (because the row before the insertion point\nwas red), whereas if `inheritFromBefore` is false, the two new rows will\nbe green (because the row after the insertion point was green)."]
        #[serde(
            rename = "inheritFromBefore",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub inherit_from_before: ::std::option::Option<bool>,
        #[doc = "The dimensions to insert.  Both the start and end indexes must be bounded."]
        #[serde(
            rename = "range",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub range: ::std::option::Option<crate::schemas::DimensionRange>,
    }
    impl ::google_field_selector::FieldSelector for InsertDimensionRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for InsertDimensionRequest {
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
    pub struct InsertRangeRequest {
        #[doc = "The range to insert new cells into."]
        #[serde(
            rename = "range",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub range: ::std::option::Option<crate::schemas::GridRange>,
        #[doc = "The dimension which will be shifted when inserting cells.\nIf ROWS, existing cells will be shifted down.\nIf COLUMNS, existing cells will be shifted right."]
        #[serde(
            rename = "shiftDimension",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub shift_dimension:
            ::std::option::Option<crate::schemas::InsertRangeRequestShiftDimension>,
    }
    impl ::google_field_selector::FieldSelector for InsertRangeRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for InsertRangeRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum InsertRangeRequestShiftDimension {
        #[doc = "Operates on the columns of a sheet."]
        Columns,
        #[doc = "The default value, do not use."]
        DimensionUnspecified,
        #[doc = "Operates on the rows of a sheet."]
        Rows,
    }
    impl InsertRangeRequestShiftDimension {
        pub fn as_str(self) -> &'static str {
            match self {
                InsertRangeRequestShiftDimension::Columns => "COLUMNS",
                InsertRangeRequestShiftDimension::DimensionUnspecified => "DIMENSION_UNSPECIFIED",
                InsertRangeRequestShiftDimension::Rows => "ROWS",
            }
        }
    }
    impl ::std::convert::AsRef<str> for InsertRangeRequestShiftDimension {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for InsertRangeRequestShiftDimension {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<InsertRangeRequestShiftDimension, ()> {
            Ok(match s {
                "COLUMNS" => InsertRangeRequestShiftDimension::Columns,
                "DIMENSION_UNSPECIFIED" => InsertRangeRequestShiftDimension::DimensionUnspecified,
                "ROWS" => InsertRangeRequestShiftDimension::Rows,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for InsertRangeRequestShiftDimension {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for InsertRangeRequestShiftDimension {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for InsertRangeRequestShiftDimension {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "COLUMNS" => InsertRangeRequestShiftDimension::Columns,
                "DIMENSION_UNSPECIFIED" => InsertRangeRequestShiftDimension::DimensionUnspecified,
                "ROWS" => InsertRangeRequestShiftDimension::Rows,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for InsertRangeRequestShiftDimension {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for InsertRangeRequestShiftDimension {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct InterpolationPoint {
        #[doc = "The color this interpolation point should use."]
        #[serde(
            rename = "color",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub color: ::std::option::Option<crate::schemas::Color>,
        #[doc = "The color this interpolation point should use.\nIf color is also set, this field takes precedence."]
        #[serde(
            rename = "colorStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub color_style: ::std::option::Option<crate::schemas::ColorStyle>,
        #[doc = "How the value should be interpreted."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::InterpolationPointType>,
        #[doc = "The value this interpolation point uses.  May be a formula.\nUnused if type is MIN or\nMAX."]
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for InterpolationPoint {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for InterpolationPoint {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum InterpolationPointType {
        #[doc = "The default value, do not use."]
        InterpolationPointTypeUnspecified,
        #[doc = "The interpolation point uses the maximum value in the\ncells over the range of the conditional format."]
        Max,
        #[doc = "The interpolation point uses the minimum value in the\ncells over the range of the conditional format."]
        Min,
        #[doc = "The interpolation point uses exactly the value in\nInterpolationPoint.value."]
        Number,
        #[doc = "The interpolation point is the given percentage over\nall the cells in the range of the conditional format.\nThis is equivalent to NUMBER if the value was:\n`=(MAX(FLATTEN(range)) * (value / 100)) + (MIN(FLATTEN(range)) * (1 - (value / 100)))`\n(where errors in the range are ignored when flattening)."]
        Percent,
        #[doc = "The interpolation point is the given percentile\nover all the cells in the range of the conditional format.\nThis is equivalent to NUMBER if the value was:\n`=PERCENTILE(FLATTEN(range), value / 100)`\n(where errors in the range are ignored when flattening)."]
        Percentile,
    }
    impl InterpolationPointType {
        pub fn as_str(self) -> &'static str {
            match self {
                InterpolationPointType::InterpolationPointTypeUnspecified => {
                    "INTERPOLATION_POINT_TYPE_UNSPECIFIED"
                }
                InterpolationPointType::Max => "MAX",
                InterpolationPointType::Min => "MIN",
                InterpolationPointType::Number => "NUMBER",
                InterpolationPointType::Percent => "PERCENT",
                InterpolationPointType::Percentile => "PERCENTILE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for InterpolationPointType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for InterpolationPointType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<InterpolationPointType, ()> {
            Ok(match s {
                "INTERPOLATION_POINT_TYPE_UNSPECIFIED" => {
                    InterpolationPointType::InterpolationPointTypeUnspecified
                }
                "MAX" => InterpolationPointType::Max,
                "MIN" => InterpolationPointType::Min,
                "NUMBER" => InterpolationPointType::Number,
                "PERCENT" => InterpolationPointType::Percent,
                "PERCENTILE" => InterpolationPointType::Percentile,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for InterpolationPointType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for InterpolationPointType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for InterpolationPointType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "INTERPOLATION_POINT_TYPE_UNSPECIFIED" => {
                    InterpolationPointType::InterpolationPointTypeUnspecified
                }
                "MAX" => InterpolationPointType::Max,
                "MIN" => InterpolationPointType::Min,
                "NUMBER" => InterpolationPointType::Number,
                "PERCENT" => InterpolationPointType::Percent,
                "PERCENTILE" => InterpolationPointType::Percentile,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for InterpolationPointType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for InterpolationPointType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct IterativeCalculationSettings {
        #[doc = "When iterative calculation is enabled and successive results differ by\nless than this threshold value, the calculation rounds stop."]
        #[serde(
            rename = "convergenceThreshold",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub convergence_threshold: ::std::option::Option<f64>,
        #[doc = "When iterative calculation is enabled, the maximum number of calculation\nrounds to perform."]
        #[serde(
            rename = "maxIterations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub max_iterations: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for IterativeCalculationSettings {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for IterativeCalculationSettings {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct KeyValueFormat {
        #[doc = "Specifies the horizontal text positioning of key value.\nThis field is optional. If not specified, default positioning is used."]
        #[serde(
            rename = "position",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub position: ::std::option::Option<crate::schemas::TextPosition>,
        #[doc = "Text formatting options for key value."]
        #[serde(
            rename = "textFormat",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text_format: ::std::option::Option<crate::schemas::TextFormat>,
    }
    impl ::google_field_selector::FieldSelector for KeyValueFormat {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for KeyValueFormat {
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
    pub struct LineStyle {
        #[doc = "The dash type of the line."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::LineStyleType>,
        #[doc = "The thickness of the line, in px."]
        #[serde(
            rename = "width",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub width: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for LineStyle {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for LineStyle {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum LineStyleType {
        #[doc = "A custom dash for a line. Modifying the exact custom dash style is\ncurrently unsupported."]
        Custom,
        #[doc = "A dotted line."]
        Dotted,
        #[doc = "No dash type, which is equivalent to a non-visible line."]
        Invisible,
        #[doc = "Default value, do not use."]
        LineDashTypeUnspecified,
        #[doc = "A dashed line where the dashes have \"long\" length."]
        LongDashed,
        #[doc = "A line that alternates between a \"long\" dash and a dot."]
        LongDashedDotted,
        #[doc = "A dashed line where the dashes have \"medium\" length."]
        MediumDashed,
        #[doc = "A line that alternates between a \"medium\" dash and a dot."]
        MediumDashedDotted,
        #[doc = "A solid line."]
        Solid,
    }
    impl LineStyleType {
        pub fn as_str(self) -> &'static str {
            match self {
                LineStyleType::Custom => "CUSTOM",
                LineStyleType::Dotted => "DOTTED",
                LineStyleType::Invisible => "INVISIBLE",
                LineStyleType::LineDashTypeUnspecified => "LINE_DASH_TYPE_UNSPECIFIED",
                LineStyleType::LongDashed => "LONG_DASHED",
                LineStyleType::LongDashedDotted => "LONG_DASHED_DOTTED",
                LineStyleType::MediumDashed => "MEDIUM_DASHED",
                LineStyleType::MediumDashedDotted => "MEDIUM_DASHED_DOTTED",
                LineStyleType::Solid => "SOLID",
            }
        }
    }
    impl ::std::convert::AsRef<str> for LineStyleType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for LineStyleType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<LineStyleType, ()> {
            Ok(match s {
                "CUSTOM" => LineStyleType::Custom,
                "DOTTED" => LineStyleType::Dotted,
                "INVISIBLE" => LineStyleType::Invisible,
                "LINE_DASH_TYPE_UNSPECIFIED" => LineStyleType::LineDashTypeUnspecified,
                "LONG_DASHED" => LineStyleType::LongDashed,
                "LONG_DASHED_DOTTED" => LineStyleType::LongDashedDotted,
                "MEDIUM_DASHED" => LineStyleType::MediumDashed,
                "MEDIUM_DASHED_DOTTED" => LineStyleType::MediumDashedDotted,
                "SOLID" => LineStyleType::Solid,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for LineStyleType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for LineStyleType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for LineStyleType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CUSTOM" => LineStyleType::Custom,
                "DOTTED" => LineStyleType::Dotted,
                "INVISIBLE" => LineStyleType::Invisible,
                "LINE_DASH_TYPE_UNSPECIFIED" => LineStyleType::LineDashTypeUnspecified,
                "LONG_DASHED" => LineStyleType::LongDashed,
                "LONG_DASHED_DOTTED" => LineStyleType::LongDashedDotted,
                "MEDIUM_DASHED" => LineStyleType::MediumDashed,
                "MEDIUM_DASHED_DOTTED" => LineStyleType::MediumDashedDotted,
                "SOLID" => LineStyleType::Solid,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for LineStyleType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for LineStyleType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ManualRule {
        #[doc = "The list of group names and the corresponding items from the source data\nthat map to each group name."]
        #[serde(
            rename = "groups",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub groups: ::std::option::Option<Vec<crate::schemas::ManualRuleGroup>>,
    }
    impl ::google_field_selector::FieldSelector for ManualRule {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ManualRule {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ManualRuleGroup {
        #[doc = "The group name, which must be a string. Each group in a given\nManualRule must have a unique group name."]
        #[serde(
            rename = "groupName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub group_name: ::std::option::Option<crate::schemas::ExtendedValue>,
        #[doc = "The items in the source data that should be placed into this group. Each\nitem may be a string, number, or boolean. Items may appear in at most one\ngroup within a given ManualRule. Items that do not appear in any\ngroup will appear on their own."]
        #[serde(
            rename = "items",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub items: ::std::option::Option<Vec<crate::schemas::ExtendedValue>>,
    }
    impl ::google_field_selector::FieldSelector for ManualRuleGroup {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ManualRuleGroup {
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
    pub struct MatchedDeveloperMetadata {
        #[doc = "All filters matching the returned developer metadata."]
        #[serde(
            rename = "dataFilters",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub data_filters: ::std::option::Option<Vec<crate::schemas::DataFilter>>,
        #[doc = "The developer metadata matching the specified filters."]
        #[serde(
            rename = "developerMetadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub developer_metadata: ::std::option::Option<crate::schemas::DeveloperMetadata>,
    }
    impl ::google_field_selector::FieldSelector for MatchedDeveloperMetadata {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MatchedDeveloperMetadata {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct MatchedValueRange {
        #[doc = "The DataFilters from the request that matched the range of\nvalues."]
        #[serde(
            rename = "dataFilters",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub data_filters: ::std::option::Option<Vec<crate::schemas::DataFilter>>,
        #[doc = "The values matched by the DataFilter."]
        #[serde(
            rename = "valueRange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value_range: ::std::option::Option<crate::schemas::ValueRange>,
    }
    impl ::google_field_selector::FieldSelector for MatchedValueRange {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MatchedValueRange {
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
    pub struct MergeCellsRequest {
        #[doc = "How the cells should be merged."]
        #[serde(
            rename = "mergeType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub merge_type: ::std::option::Option<crate::schemas::MergeCellsRequestMergeType>,
        #[doc = "The range of cells to merge."]
        #[serde(
            rename = "range",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub range: ::std::option::Option<crate::schemas::GridRange>,
    }
    impl ::google_field_selector::FieldSelector for MergeCellsRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MergeCellsRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum MergeCellsRequestMergeType {
        #[doc = "Create a single merge from the range"]
        MergeAll,
        #[doc = "Create a merge for each column in the range"]
        MergeColumns,
        #[doc = "Create a merge for each row in the range"]
        MergeRows,
    }
    impl MergeCellsRequestMergeType {
        pub fn as_str(self) -> &'static str {
            match self {
                MergeCellsRequestMergeType::MergeAll => "MERGE_ALL",
                MergeCellsRequestMergeType::MergeColumns => "MERGE_COLUMNS",
                MergeCellsRequestMergeType::MergeRows => "MERGE_ROWS",
            }
        }
    }
    impl ::std::convert::AsRef<str> for MergeCellsRequestMergeType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for MergeCellsRequestMergeType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<MergeCellsRequestMergeType, ()> {
            Ok(match s {
                "MERGE_ALL" => MergeCellsRequestMergeType::MergeAll,
                "MERGE_COLUMNS" => MergeCellsRequestMergeType::MergeColumns,
                "MERGE_ROWS" => MergeCellsRequestMergeType::MergeRows,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for MergeCellsRequestMergeType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for MergeCellsRequestMergeType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for MergeCellsRequestMergeType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "MERGE_ALL" => MergeCellsRequestMergeType::MergeAll,
                "MERGE_COLUMNS" => MergeCellsRequestMergeType::MergeColumns,
                "MERGE_ROWS" => MergeCellsRequestMergeType::MergeRows,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for MergeCellsRequestMergeType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MergeCellsRequestMergeType {
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
    pub struct MoveDimensionRequest {
        #[doc = "The zero-based start index of where to move the source data to,\nbased on the coordinates *before* the source data is removed\nfrom the grid.  Existing data will be shifted down or right\n(depending on the dimension) to make room for the moved dimensions.\nThe source dimensions are removed from the grid, so the\nthe data may end up in a different index than specified.\n\nFor example, given `A1..A5` of `0, 1, 2, 3, 4` and wanting to move\n`\"1\"` and `\"2\"` to between `\"3\"` and `\"4\"`, the source would be\n`ROWS [1..3)`,and the destination index would be `\"4\"`\n(the zero-based index of row 5).\nThe end result would be `A1..A5` of `0, 3, 1, 2, 4`."]
        #[serde(
            rename = "destinationIndex",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub destination_index: ::std::option::Option<i32>,
        #[doc = "The source dimensions to move."]
        #[serde(
            rename = "source",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source: ::std::option::Option<crate::schemas::DimensionRange>,
    }
    impl ::google_field_selector::FieldSelector for MoveDimensionRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MoveDimensionRequest {
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
    pub struct NamedRange {
        #[doc = "The name of the named range."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The ID of the named range."]
        #[serde(
            rename = "namedRangeId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub named_range_id: ::std::option::Option<String>,
        #[doc = "The range this represents."]
        #[serde(
            rename = "range",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub range: ::std::option::Option<crate::schemas::GridRange>,
    }
    impl ::google_field_selector::FieldSelector for NamedRange {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for NamedRange {
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
    pub struct NumberFormat {
        #[doc = "Pattern string used for formatting.  If not set, a default pattern based on\nthe user's locale will be used if necessary for the given type.\nSee the [Date and Number Formats guide](/sheets/api/guides/formats) for\nmore information about the supported patterns."]
        #[serde(
            rename = "pattern",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub pattern: ::std::option::Option<String>,
        #[doc = "The type of the number format.\nWhen writing, this field must be set."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::NumberFormatType>,
    }
    impl ::google_field_selector::FieldSelector for NumberFormat {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for NumberFormat {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum NumberFormatType {
        #[doc = "Currency formatting, e.g `$1,000.12`"]
        Currency,
        #[doc = "Date formatting, e.g `9/26/2008`"]
        Date,
        #[doc = "Date+Time formatting, e.g `9/26/08 15:59:00`"]
        DateTime,
        #[doc = "Number formatting, e.g, `1,000.12`"]
        Number,
        #[doc = "The number format is not specified\nand is based on the contents of the cell.\nDo not explicitly use this."]
        NumberFormatTypeUnspecified,
        #[doc = "Percent formatting, e.g `10.12%`"]
        Percent,
        #[doc = "Scientific number formatting, e.g `1.01E+03`"]
        Scientific,
        #[doc = "Text formatting, e.g `1000.12`"]
        Text,
        #[doc = "Time formatting, e.g `3:59:00 PM`"]
        Time,
    }
    impl NumberFormatType {
        pub fn as_str(self) -> &'static str {
            match self {
                NumberFormatType::Currency => "CURRENCY",
                NumberFormatType::Date => "DATE",
                NumberFormatType::DateTime => "DATE_TIME",
                NumberFormatType::Number => "NUMBER",
                NumberFormatType::NumberFormatTypeUnspecified => "NUMBER_FORMAT_TYPE_UNSPECIFIED",
                NumberFormatType::Percent => "PERCENT",
                NumberFormatType::Scientific => "SCIENTIFIC",
                NumberFormatType::Text => "TEXT",
                NumberFormatType::Time => "TIME",
            }
        }
    }
    impl ::std::convert::AsRef<str> for NumberFormatType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for NumberFormatType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<NumberFormatType, ()> {
            Ok(match s {
                "CURRENCY" => NumberFormatType::Currency,
                "DATE" => NumberFormatType::Date,
                "DATE_TIME" => NumberFormatType::DateTime,
                "NUMBER" => NumberFormatType::Number,
                "NUMBER_FORMAT_TYPE_UNSPECIFIED" => NumberFormatType::NumberFormatTypeUnspecified,
                "PERCENT" => NumberFormatType::Percent,
                "SCIENTIFIC" => NumberFormatType::Scientific,
                "TEXT" => NumberFormatType::Text,
                "TIME" => NumberFormatType::Time,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for NumberFormatType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for NumberFormatType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for NumberFormatType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CURRENCY" => NumberFormatType::Currency,
                "DATE" => NumberFormatType::Date,
                "DATE_TIME" => NumberFormatType::DateTime,
                "NUMBER" => NumberFormatType::Number,
                "NUMBER_FORMAT_TYPE_UNSPECIFIED" => NumberFormatType::NumberFormatTypeUnspecified,
                "PERCENT" => NumberFormatType::Percent,
                "SCIENTIFIC" => NumberFormatType::Scientific,
                "TEXT" => NumberFormatType::Text,
                "TIME" => NumberFormatType::Time,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for NumberFormatType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for NumberFormatType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct OrgChartSpec {
        #[doc = "The data containing the labels for all the nodes in the chart.  Labels\nmust be unique."]
        #[serde(
            rename = "labels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub labels: ::std::option::Option<crate::schemas::ChartData>,
        #[doc = "The color of the org chart nodes."]
        #[serde(
            rename = "nodeColor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub node_color: ::std::option::Option<crate::schemas::Color>,
        #[doc = "The color of the org chart nodes.\nIf node_color is also set, this field takes precedence."]
        #[serde(
            rename = "nodeColorStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub node_color_style: ::std::option::Option<crate::schemas::ColorStyle>,
        #[doc = "The size of the org chart nodes."]
        #[serde(
            rename = "nodeSize",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub node_size: ::std::option::Option<crate::schemas::OrgChartSpecNodeSize>,
        #[doc = "The data containing the label of the parent for the corresponding node.\nA blank value indicates that the node has no parent and is a top-level\nnode.\nThis field is optional."]
        #[serde(
            rename = "parentLabels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub parent_labels: ::std::option::Option<crate::schemas::ChartData>,
        #[doc = "The color of the selected org chart nodes."]
        #[serde(
            rename = "selectedNodeColor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub selected_node_color: ::std::option::Option<crate::schemas::Color>,
        #[doc = "The color of the selected org chart nodes.\nIf selected_node_color is also set, this field takes precedence."]
        #[serde(
            rename = "selectedNodeColorStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub selected_node_color_style: ::std::option::Option<crate::schemas::ColorStyle>,
        #[doc = "The data containing the tooltip for the corresponding node.  A blank value\nresults in no tooltip being displayed for the node.\nThis field is optional."]
        #[serde(
            rename = "tooltips",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub tooltips: ::std::option::Option<crate::schemas::ChartData>,
    }
    impl ::google_field_selector::FieldSelector for OrgChartSpec {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OrgChartSpec {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum OrgChartSpecNodeSize {
        #[doc = "The large org chart node size."]
        Large,
        #[doc = "The medium org chart node size."]
        Medium,
        #[doc = "Default value, do not use."]
        OrgChartLabelSizeUnspecified,
        #[doc = "The small org chart node size."]
        Small,
    }
    impl OrgChartSpecNodeSize {
        pub fn as_str(self) -> &'static str {
            match self {
                OrgChartSpecNodeSize::Large => "LARGE",
                OrgChartSpecNodeSize::Medium => "MEDIUM",
                OrgChartSpecNodeSize::OrgChartLabelSizeUnspecified => {
                    "ORG_CHART_LABEL_SIZE_UNSPECIFIED"
                }
                OrgChartSpecNodeSize::Small => "SMALL",
            }
        }
    }
    impl ::std::convert::AsRef<str> for OrgChartSpecNodeSize {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for OrgChartSpecNodeSize {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<OrgChartSpecNodeSize, ()> {
            Ok(match s {
                "LARGE" => OrgChartSpecNodeSize::Large,
                "MEDIUM" => OrgChartSpecNodeSize::Medium,
                "ORG_CHART_LABEL_SIZE_UNSPECIFIED" => {
                    OrgChartSpecNodeSize::OrgChartLabelSizeUnspecified
                }
                "SMALL" => OrgChartSpecNodeSize::Small,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for OrgChartSpecNodeSize {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for OrgChartSpecNodeSize {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for OrgChartSpecNodeSize {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LARGE" => OrgChartSpecNodeSize::Large,
                "MEDIUM" => OrgChartSpecNodeSize::Medium,
                "ORG_CHART_LABEL_SIZE_UNSPECIFIED" => {
                    OrgChartSpecNodeSize::OrgChartLabelSizeUnspecified
                }
                "SMALL" => OrgChartSpecNodeSize::Small,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for OrgChartSpecNodeSize {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OrgChartSpecNodeSize {
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
    pub struct OverlayPosition {
        #[doc = "The cell the object is anchored to."]
        #[serde(
            rename = "anchorCell",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub anchor_cell: ::std::option::Option<crate::schemas::GridCoordinate>,
        #[doc = "The height of the object, in pixels. Defaults to 371."]
        #[serde(
            rename = "heightPixels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub height_pixels: ::std::option::Option<i32>,
        #[doc = "The horizontal offset, in pixels, that the object is offset\nfrom the anchor cell."]
        #[serde(
            rename = "offsetXPixels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub offset_x_pixels: ::std::option::Option<i32>,
        #[doc = "The vertical offset, in pixels, that the object is offset\nfrom the anchor cell."]
        #[serde(
            rename = "offsetYPixels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub offset_y_pixels: ::std::option::Option<i32>,
        #[doc = "The width of the object, in pixels. Defaults to 600."]
        #[serde(
            rename = "widthPixels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub width_pixels: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for OverlayPosition {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OverlayPosition {
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
    pub struct Padding {
        #[doc = "The bottom padding of the cell."]
        #[serde(
            rename = "bottom",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bottom: ::std::option::Option<i32>,
        #[doc = "The left padding of the cell."]
        #[serde(
            rename = "left",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub left: ::std::option::Option<i32>,
        #[doc = "The right padding of the cell."]
        #[serde(
            rename = "right",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub right: ::std::option::Option<i32>,
        #[doc = "The top padding of the cell."]
        #[serde(
            rename = "top",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub top: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for Padding {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Padding {
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
    pub struct PasteDataRequest {
        #[doc = "The coordinate at which the data should start being inserted."]
        #[serde(
            rename = "coordinate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub coordinate: ::std::option::Option<crate::schemas::GridCoordinate>,
        #[doc = "The data to insert."]
        #[serde(
            rename = "data",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub data: ::std::option::Option<String>,
        #[doc = "The delimiter in the data."]
        #[serde(
            rename = "delimiter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub delimiter: ::std::option::Option<String>,
        #[doc = "True if the data is HTML."]
        #[serde(
            rename = "html",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub html: ::std::option::Option<bool>,
        #[doc = "How the data should be pasted."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::PasteDataRequestType>,
    }
    impl ::google_field_selector::FieldSelector for PasteDataRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PasteDataRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PasteDataRequestType {
        #[doc = "Paste the conditional formatting rules only."]
        PasteConditionalFormatting,
        #[doc = "Paste the data validation only."]
        PasteDataValidation,
        #[doc = "Paste the format and data validation only."]
        PasteFormat,
        #[doc = "Paste the formulas only."]
        PasteFormula,
        #[doc = "Like PASTE_NORMAL but without borders."]
        PasteNoBorders,
        #[doc = "Paste values, formulas, formats, and merges."]
        PasteNormal,
        #[doc = "Paste the values ONLY without formats, formulas, or merges."]
        PasteValues,
    }
    impl PasteDataRequestType {
        pub fn as_str(self) -> &'static str {
            match self {
                PasteDataRequestType::PasteConditionalFormatting => "PASTE_CONDITIONAL_FORMATTING",
                PasteDataRequestType::PasteDataValidation => "PASTE_DATA_VALIDATION",
                PasteDataRequestType::PasteFormat => "PASTE_FORMAT",
                PasteDataRequestType::PasteFormula => "PASTE_FORMULA",
                PasteDataRequestType::PasteNoBorders => "PASTE_NO_BORDERS",
                PasteDataRequestType::PasteNormal => "PASTE_NORMAL",
                PasteDataRequestType::PasteValues => "PASTE_VALUES",
            }
        }
    }
    impl ::std::convert::AsRef<str> for PasteDataRequestType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for PasteDataRequestType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<PasteDataRequestType, ()> {
            Ok(match s {
                "PASTE_CONDITIONAL_FORMATTING" => PasteDataRequestType::PasteConditionalFormatting,
                "PASTE_DATA_VALIDATION" => PasteDataRequestType::PasteDataValidation,
                "PASTE_FORMAT" => PasteDataRequestType::PasteFormat,
                "PASTE_FORMULA" => PasteDataRequestType::PasteFormula,
                "PASTE_NO_BORDERS" => PasteDataRequestType::PasteNoBorders,
                "PASTE_NORMAL" => PasteDataRequestType::PasteNormal,
                "PASTE_VALUES" => PasteDataRequestType::PasteValues,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for PasteDataRequestType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PasteDataRequestType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PasteDataRequestType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "PASTE_CONDITIONAL_FORMATTING" => PasteDataRequestType::PasteConditionalFormatting,
                "PASTE_DATA_VALIDATION" => PasteDataRequestType::PasteDataValidation,
                "PASTE_FORMAT" => PasteDataRequestType::PasteFormat,
                "PASTE_FORMULA" => PasteDataRequestType::PasteFormula,
                "PASTE_NO_BORDERS" => PasteDataRequestType::PasteNoBorders,
                "PASTE_NORMAL" => PasteDataRequestType::PasteNormal,
                "PASTE_VALUES" => PasteDataRequestType::PasteValues,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for PasteDataRequestType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PasteDataRequestType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct PieChartSpec {
        #[doc = "The data that covers the domain of the pie chart."]
        #[serde(
            rename = "domain",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub domain: ::std::option::Option<crate::schemas::ChartData>,
        #[doc = "Where the legend of the pie chart should be drawn."]
        #[serde(
            rename = "legendPosition",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub legend_position: ::std::option::Option<crate::schemas::PieChartSpecLegendPosition>,
        #[doc = "The size of the hole in the pie chart."]
        #[serde(
            rename = "pieHole",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub pie_hole: ::std::option::Option<f64>,
        #[doc = "The data that covers the one and only series of the pie chart."]
        #[serde(
            rename = "series",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub series: ::std::option::Option<crate::schemas::ChartData>,
        #[doc = "True if the pie is three dimensional."]
        #[serde(
            rename = "threeDimensional",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub three_dimensional: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for PieChartSpec {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PieChartSpec {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PieChartSpecLegendPosition {
        #[doc = "The legend is rendered on the bottom of the chart."]
        BottomLegend,
        #[doc = "Each pie slice has a label attached to it."]
        LabeledLegend,
        #[doc = "The legend is rendered on the left of the chart."]
        LeftLegend,
        #[doc = "No legend is rendered."]
        NoLegend,
        #[doc = "Default value, do not use."]
        PieChartLegendPositionUnspecified,
        #[doc = "The legend is rendered on the right of the chart."]
        RightLegend,
        #[doc = "The legend is rendered on the top of the chart."]
        TopLegend,
    }
    impl PieChartSpecLegendPosition {
        pub fn as_str(self) -> &'static str {
            match self {
                PieChartSpecLegendPosition::BottomLegend => "BOTTOM_LEGEND",
                PieChartSpecLegendPosition::LabeledLegend => "LABELED_LEGEND",
                PieChartSpecLegendPosition::LeftLegend => "LEFT_LEGEND",
                PieChartSpecLegendPosition::NoLegend => "NO_LEGEND",
                PieChartSpecLegendPosition::PieChartLegendPositionUnspecified => {
                    "PIE_CHART_LEGEND_POSITION_UNSPECIFIED"
                }
                PieChartSpecLegendPosition::RightLegend => "RIGHT_LEGEND",
                PieChartSpecLegendPosition::TopLegend => "TOP_LEGEND",
            }
        }
    }
    impl ::std::convert::AsRef<str> for PieChartSpecLegendPosition {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for PieChartSpecLegendPosition {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<PieChartSpecLegendPosition, ()> {
            Ok(match s {
                "BOTTOM_LEGEND" => PieChartSpecLegendPosition::BottomLegend,
                "LABELED_LEGEND" => PieChartSpecLegendPosition::LabeledLegend,
                "LEFT_LEGEND" => PieChartSpecLegendPosition::LeftLegend,
                "NO_LEGEND" => PieChartSpecLegendPosition::NoLegend,
                "PIE_CHART_LEGEND_POSITION_UNSPECIFIED" => {
                    PieChartSpecLegendPosition::PieChartLegendPositionUnspecified
                }
                "RIGHT_LEGEND" => PieChartSpecLegendPosition::RightLegend,
                "TOP_LEGEND" => PieChartSpecLegendPosition::TopLegend,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for PieChartSpecLegendPosition {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PieChartSpecLegendPosition {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PieChartSpecLegendPosition {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BOTTOM_LEGEND" => PieChartSpecLegendPosition::BottomLegend,
                "LABELED_LEGEND" => PieChartSpecLegendPosition::LabeledLegend,
                "LEFT_LEGEND" => PieChartSpecLegendPosition::LeftLegend,
                "NO_LEGEND" => PieChartSpecLegendPosition::NoLegend,
                "PIE_CHART_LEGEND_POSITION_UNSPECIFIED" => {
                    PieChartSpecLegendPosition::PieChartLegendPositionUnspecified
                }
                "RIGHT_LEGEND" => PieChartSpecLegendPosition::RightLegend,
                "TOP_LEGEND" => PieChartSpecLegendPosition::TopLegend,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for PieChartSpecLegendPosition {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PieChartSpecLegendPosition {
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
    pub struct PivotFilterCriteria {
        #[doc = "Values that should be included.  Values not listed here are excluded."]
        #[serde(
            rename = "visibleValues",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub visible_values: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for PivotFilterCriteria {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PivotFilterCriteria {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct PivotGroup {
        #[doc = "The group rule to apply to this row/column group."]
        #[serde(
            rename = "groupRule",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub group_rule: ::std::option::Option<crate::schemas::PivotGroupRule>,
        #[doc = "The labels to use for the row/column groups which can be customized. For\nexample, in the following pivot table, the row label is `Region` (which\ncould be renamed to `State`) and the column label is `Product` (which\ncould be renamed `Item`). Pivot tables created before December 2017 do\nnot have header labels. If you'd like to add header labels to an existing\npivot table, please delete the existing pivot table and then create a new\npivot table with same parameters.\n\n````text\n+--------------+---------+-------+\n| SUM of Units | Product |       |\n| Region       | Pen     | Paper |\n+--------------+---------+-------+\n| New York     |     345 |    98 |\n| Oregon       |     234 |   123 |\n| Tennessee    |     531 |   415 |\n+--------------+---------+-------+\n| Grand Total  |    1110 |   636 |\n+--------------+---------+-------+````"]
        #[serde(
            rename = "label",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub label: ::std::option::Option<String>,
        #[doc = "True if the headings in this pivot group should be repeated.\nThis is only valid for row groupings and is ignored by columns.\n\nBy default, we minimize repitition of headings by not showing higher\nlevel headings where they are the same. For example, even though the\nthird row below corresponds to \"Q1 Mar\", \"Q1\" is not shown because\nit is redundant with previous rows. Setting repeat_headings to true\nwould cause \"Q1\" to be repeated for \"Feb\" and \"Mar\".\n\n````text\n+--------------+\n| Q1     | Jan |\n|        | Feb |\n|        | Mar |\n+--------+-----+\n| Q1 Total     |\n+--------------+````"]
        #[serde(
            rename = "repeatHeadings",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub repeat_headings: ::std::option::Option<bool>,
        #[doc = "True if the pivot table should include the totals for this grouping."]
        #[serde(
            rename = "showTotals",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub show_totals: ::std::option::Option<bool>,
        #[doc = "The order the values in this group should be sorted."]
        #[serde(
            rename = "sortOrder",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sort_order: ::std::option::Option<crate::schemas::PivotGroupSortOrder>,
        #[doc = "The column offset of the source range that this grouping is based on.\n\nFor example, if the source was `C10:E15`, a `sourceColumnOffset` of `0`\nmeans this group refers to column `C`, whereas the offset `1` would\nrefer to column `D`."]
        #[serde(
            rename = "sourceColumnOffset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source_column_offset: ::std::option::Option<i32>,
        #[doc = "The bucket of the opposite pivot group to sort by.\nIf not specified, sorting is alphabetical by this group's values."]
        #[serde(
            rename = "valueBucket",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value_bucket: ::std::option::Option<crate::schemas::PivotGroupSortValueBucket>,
        #[doc = "Metadata about values in the grouping."]
        #[serde(
            rename = "valueMetadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value_metadata: ::std::option::Option<Vec<crate::schemas::PivotGroupValueMetadata>>,
    }
    impl ::google_field_selector::FieldSelector for PivotGroup {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PivotGroup {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PivotGroupSortOrder {
        #[doc = "Sort ascending."]
        Ascending,
        #[doc = "Sort descending."]
        Descending,
        #[doc = "Default value, do not use this."]
        SortOrderUnspecified,
    }
    impl PivotGroupSortOrder {
        pub fn as_str(self) -> &'static str {
            match self {
                PivotGroupSortOrder::Ascending => "ASCENDING",
                PivotGroupSortOrder::Descending => "DESCENDING",
                PivotGroupSortOrder::SortOrderUnspecified => "SORT_ORDER_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for PivotGroupSortOrder {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for PivotGroupSortOrder {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<PivotGroupSortOrder, ()> {
            Ok(match s {
                "ASCENDING" => PivotGroupSortOrder::Ascending,
                "DESCENDING" => PivotGroupSortOrder::Descending,
                "SORT_ORDER_UNSPECIFIED" => PivotGroupSortOrder::SortOrderUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for PivotGroupSortOrder {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PivotGroupSortOrder {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PivotGroupSortOrder {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ASCENDING" => PivotGroupSortOrder::Ascending,
                "DESCENDING" => PivotGroupSortOrder::Descending,
                "SORT_ORDER_UNSPECIFIED" => PivotGroupSortOrder::SortOrderUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for PivotGroupSortOrder {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PivotGroupSortOrder {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct PivotGroupRule {
        #[doc = "A DateTimeRule."]
        #[serde(
            rename = "dateTimeRule",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub date_time_rule: ::std::option::Option<crate::schemas::DateTimeRule>,
        #[doc = "A HistogramRule."]
        #[serde(
            rename = "histogramRule",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub histogram_rule: ::std::option::Option<crate::schemas::HistogramRule>,
        #[doc = "A ManualRule."]
        #[serde(
            rename = "manualRule",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub manual_rule: ::std::option::Option<crate::schemas::ManualRule>,
    }
    impl ::google_field_selector::FieldSelector for PivotGroupRule {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PivotGroupRule {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct PivotGroupSortValueBucket {
        #[doc = "Determines the bucket from which values are chosen to sort.\n\nFor example, in a pivot table with one row group & two column groups,\nthe row group can list up to two values. The first value corresponds\nto a value within the first column group, and the second value\ncorresponds to a value in the second column group.  If no values\nare listed, this would indicate that the row should be sorted according\nto the \"Grand Total\" over the column groups. If a single value is listed,\nthis would correspond to using the \"Total\" of that bucket."]
        #[serde(
            rename = "buckets",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub buckets: ::std::option::Option<Vec<crate::schemas::ExtendedValue>>,
        #[doc = "The offset in the PivotTable.values list which the values in this\ngrouping should be sorted by."]
        #[serde(
            rename = "valuesIndex",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub values_index: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for PivotGroupSortValueBucket {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PivotGroupSortValueBucket {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct PivotGroupValueMetadata {
        #[doc = "True if the data corresponding to the value is collapsed."]
        #[serde(
            rename = "collapsed",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub collapsed: ::std::option::Option<bool>,
        #[doc = "The calculated value the metadata corresponds to.\n(Note that formulaValue is not valid,\nbecause the values will be calculated.)"]
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value: ::std::option::Option<crate::schemas::ExtendedValue>,
    }
    impl ::google_field_selector::FieldSelector for PivotGroupValueMetadata {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PivotGroupValueMetadata {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct PivotTable {
        #[doc = "Each column grouping in the pivot table."]
        #[serde(
            rename = "columns",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub columns: ::std::option::Option<Vec<crate::schemas::PivotGroup>>,
        #[doc = "An optional mapping of filters per source column offset.\n\nThe filters are applied before aggregating data into the pivot table.\nThe map's key is the column offset of the source range that you want to\nfilter, and the value is the criteria for that column.\n\nFor example, if the source was `C10:E15`, a key of `0` will have the filter\nfor column `C`, whereas the key `1` is for column `D`."]
        #[serde(
            rename = "criteria",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub criteria: ::std::option::Option<
            ::std::collections::BTreeMap<String, crate::schemas::PivotFilterCriteria>,
        >,
        #[doc = "Each row grouping in the pivot table."]
        #[serde(
            rename = "rows",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rows: ::std::option::Option<Vec<crate::schemas::PivotGroup>>,
        #[doc = "The range the pivot table is reading data from."]
        #[serde(
            rename = "source",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source: ::std::option::Option<crate::schemas::GridRange>,
        #[doc = "Whether values should be listed horizontally (as columns)\nor vertically (as rows)."]
        #[serde(
            rename = "valueLayout",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value_layout: ::std::option::Option<crate::schemas::PivotTableValueLayout>,
        #[doc = "A list of values to include in the pivot table."]
        #[serde(
            rename = "values",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub values: ::std::option::Option<Vec<crate::schemas::PivotValue>>,
    }
    impl ::google_field_selector::FieldSelector for PivotTable {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PivotTable {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PivotTableValueLayout {
        #[doc = "Values are laid out horizontally (as columns)."]
        Horizontal,
        #[doc = "Values are laid out vertically (as rows)."]
        Vertical,
    }
    impl PivotTableValueLayout {
        pub fn as_str(self) -> &'static str {
            match self {
                PivotTableValueLayout::Horizontal => "HORIZONTAL",
                PivotTableValueLayout::Vertical => "VERTICAL",
            }
        }
    }
    impl ::std::convert::AsRef<str> for PivotTableValueLayout {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for PivotTableValueLayout {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<PivotTableValueLayout, ()> {
            Ok(match s {
                "HORIZONTAL" => PivotTableValueLayout::Horizontal,
                "VERTICAL" => PivotTableValueLayout::Vertical,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for PivotTableValueLayout {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PivotTableValueLayout {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PivotTableValueLayout {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "HORIZONTAL" => PivotTableValueLayout::Horizontal,
                "VERTICAL" => PivotTableValueLayout::Vertical,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for PivotTableValueLayout {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PivotTableValueLayout {
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
    pub struct PivotValue {
        #[doc = "If specified, indicates that pivot values should be displayed as\nthe result of a calculation with another pivot value. For example, if\ncalculated_display_type is specified as PERCENT_OF_GRAND_TOTAL, all the\npivot values are displayed as the percentage of the grand total. In\nthe Sheets UI, this is referred to as \"Show As\" in the value section of a\npivot table."]
        #[serde(
            rename = "calculatedDisplayType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub calculated_display_type:
            ::std::option::Option<crate::schemas::PivotValueCalculatedDisplayType>,
        #[doc = "A custom formula to calculate the value.  The formula must start\nwith an `=` character."]
        #[serde(
            rename = "formula",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub formula: ::std::option::Option<String>,
        #[doc = "A name to use for the value."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The column offset of the source range that this value reads from.\n\nFor example, if the source was `C10:E15`, a `sourceColumnOffset` of `0`\nmeans this value refers to column `C`, whereas the offset `1` would\nrefer to column `D`."]
        #[serde(
            rename = "sourceColumnOffset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source_column_offset: ::std::option::Option<i32>,
        #[doc = "A function to summarize the value.\nIf formula is set, the only supported values are\nSUM and\nCUSTOM.\nIf sourceColumnOffset is set, then `CUSTOM`\nis not supported."]
        #[serde(
            rename = "summarizeFunction",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub summarize_function: ::std::option::Option<crate::schemas::PivotValueSummarizeFunction>,
    }
    impl ::google_field_selector::FieldSelector for PivotValue {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PivotValue {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PivotValueCalculatedDisplayType {
        #[doc = "Shows the pivot values as percentage of the column total values."]
        PercentOfColumnTotal,
        #[doc = "Shows the pivot values as percentage of the grand total values."]
        PercentOfGrandTotal,
        #[doc = "Shows the pivot values as percentage of the row total values."]
        PercentOfRowTotal,
        #[doc = "Default value, do not use."]
        PivotValueCalculatedDisplayTypeUnspecified,
    }
    impl PivotValueCalculatedDisplayType {
        pub fn as_str(self) -> &'static str {
            match self {
                PivotValueCalculatedDisplayType::PercentOfColumnTotal => "PERCENT_OF_COLUMN_TOTAL",
                PivotValueCalculatedDisplayType::PercentOfGrandTotal => "PERCENT_OF_GRAND_TOTAL",
                PivotValueCalculatedDisplayType::PercentOfRowTotal => "PERCENT_OF_ROW_TOTAL",
                PivotValueCalculatedDisplayType::PivotValueCalculatedDisplayTypeUnspecified => {
                    "PIVOT_VALUE_CALCULATED_DISPLAY_TYPE_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for PivotValueCalculatedDisplayType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for PivotValueCalculatedDisplayType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<PivotValueCalculatedDisplayType, ()> {
            Ok(match s {
                "PERCENT_OF_COLUMN_TOTAL" => PivotValueCalculatedDisplayType::PercentOfColumnTotal,
                "PERCENT_OF_GRAND_TOTAL" => PivotValueCalculatedDisplayType::PercentOfGrandTotal,
                "PERCENT_OF_ROW_TOTAL" => PivotValueCalculatedDisplayType::PercentOfRowTotal,
                "PIVOT_VALUE_CALCULATED_DISPLAY_TYPE_UNSPECIFIED" => {
                    PivotValueCalculatedDisplayType::PivotValueCalculatedDisplayTypeUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for PivotValueCalculatedDisplayType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PivotValueCalculatedDisplayType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PivotValueCalculatedDisplayType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "PERCENT_OF_COLUMN_TOTAL" => PivotValueCalculatedDisplayType::PercentOfColumnTotal,
                "PERCENT_OF_GRAND_TOTAL" => PivotValueCalculatedDisplayType::PercentOfGrandTotal,
                "PERCENT_OF_ROW_TOTAL" => PivotValueCalculatedDisplayType::PercentOfRowTotal,
                "PIVOT_VALUE_CALCULATED_DISPLAY_TYPE_UNSPECIFIED" => {
                    PivotValueCalculatedDisplayType::PivotValueCalculatedDisplayTypeUnspecified
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
    impl ::google_field_selector::FieldSelector for PivotValueCalculatedDisplayType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PivotValueCalculatedDisplayType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PivotValueSummarizeFunction {
        #[doc = "Corresponds to the `AVERAGE` function."]
        Average,
        #[doc = "Corresponds to the `COUNT` function."]
        Count,
        #[doc = "Corresponds to the `COUNTA` function."]
        Counta,
        #[doc = "Corresponds to the `COUNTUNIQUE` function."]
        Countunique,
        #[doc = "Indicates the formula should be used as-is.\nOnly valid if PivotValue.formula was set."]
        Custom,
        #[doc = "Corresponds to the `MAX` function."]
        Max,
        #[doc = "Corresponds to the `MEDIAN` function."]
        Median,
        #[doc = "Corresponds to the `MIN` function."]
        Min,
        #[doc = "The default, do not use."]
        PivotStandardValueFunctionUnspecified,
        #[doc = "Corresponds to the `PRODUCT` function."]
        Product,
        #[doc = "Corresponds to the `STDEV` function."]
        Stdev,
        #[doc = "Corresponds to the `STDEVP` function."]
        Stdevp,
        #[doc = "Corresponds to the `SUM` function."]
        Sum,
        #[doc = "Corresponds to the `VAR` function."]
        Var,
        #[doc = "Corresponds to the `VARP` function."]
        Varp,
    }
    impl PivotValueSummarizeFunction {
        pub fn as_str(self) -> &'static str {
            match self {
                PivotValueSummarizeFunction::Average => "AVERAGE",
                PivotValueSummarizeFunction::Count => "COUNT",
                PivotValueSummarizeFunction::Counta => "COUNTA",
                PivotValueSummarizeFunction::Countunique => "COUNTUNIQUE",
                PivotValueSummarizeFunction::Custom => "CUSTOM",
                PivotValueSummarizeFunction::Max => "MAX",
                PivotValueSummarizeFunction::Median => "MEDIAN",
                PivotValueSummarizeFunction::Min => "MIN",
                PivotValueSummarizeFunction::PivotStandardValueFunctionUnspecified => {
                    "PIVOT_STANDARD_VALUE_FUNCTION_UNSPECIFIED"
                }
                PivotValueSummarizeFunction::Product => "PRODUCT",
                PivotValueSummarizeFunction::Stdev => "STDEV",
                PivotValueSummarizeFunction::Stdevp => "STDEVP",
                PivotValueSummarizeFunction::Sum => "SUM",
                PivotValueSummarizeFunction::Var => "VAR",
                PivotValueSummarizeFunction::Varp => "VARP",
            }
        }
    }
    impl ::std::convert::AsRef<str> for PivotValueSummarizeFunction {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for PivotValueSummarizeFunction {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<PivotValueSummarizeFunction, ()> {
            Ok(match s {
                "AVERAGE" => PivotValueSummarizeFunction::Average,
                "COUNT" => PivotValueSummarizeFunction::Count,
                "COUNTA" => PivotValueSummarizeFunction::Counta,
                "COUNTUNIQUE" => PivotValueSummarizeFunction::Countunique,
                "CUSTOM" => PivotValueSummarizeFunction::Custom,
                "MAX" => PivotValueSummarizeFunction::Max,
                "MEDIAN" => PivotValueSummarizeFunction::Median,
                "MIN" => PivotValueSummarizeFunction::Min,
                "PIVOT_STANDARD_VALUE_FUNCTION_UNSPECIFIED" => {
                    PivotValueSummarizeFunction::PivotStandardValueFunctionUnspecified
                }
                "PRODUCT" => PivotValueSummarizeFunction::Product,
                "STDEV" => PivotValueSummarizeFunction::Stdev,
                "STDEVP" => PivotValueSummarizeFunction::Stdevp,
                "SUM" => PivotValueSummarizeFunction::Sum,
                "VAR" => PivotValueSummarizeFunction::Var,
                "VARP" => PivotValueSummarizeFunction::Varp,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for PivotValueSummarizeFunction {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PivotValueSummarizeFunction {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PivotValueSummarizeFunction {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "AVERAGE" => PivotValueSummarizeFunction::Average,
                "COUNT" => PivotValueSummarizeFunction::Count,
                "COUNTA" => PivotValueSummarizeFunction::Counta,
                "COUNTUNIQUE" => PivotValueSummarizeFunction::Countunique,
                "CUSTOM" => PivotValueSummarizeFunction::Custom,
                "MAX" => PivotValueSummarizeFunction::Max,
                "MEDIAN" => PivotValueSummarizeFunction::Median,
                "MIN" => PivotValueSummarizeFunction::Min,
                "PIVOT_STANDARD_VALUE_FUNCTION_UNSPECIFIED" => {
                    PivotValueSummarizeFunction::PivotStandardValueFunctionUnspecified
                }
                "PRODUCT" => PivotValueSummarizeFunction::Product,
                "STDEV" => PivotValueSummarizeFunction::Stdev,
                "STDEVP" => PivotValueSummarizeFunction::Stdevp,
                "SUM" => PivotValueSummarizeFunction::Sum,
                "VAR" => PivotValueSummarizeFunction::Var,
                "VARP" => PivotValueSummarizeFunction::Varp,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for PivotValueSummarizeFunction {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PivotValueSummarizeFunction {
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
    pub struct ProtectedRange {
        #[doc = "The description of this protected range."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "The users and groups with edit access to the protected range.\nThis field is only visible to users with edit access to the protected\nrange and the document.\nEditors are not supported with warning_only protection."]
        #[serde(
            rename = "editors",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub editors: ::std::option::Option<crate::schemas::Editors>,
        #[doc = "The named range this protected range is backed by, if any.\n\nWhen writing, only one of range or named_range_id\nmay be set."]
        #[serde(
            rename = "namedRangeId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub named_range_id: ::std::option::Option<String>,
        #[doc = "The ID of the protected range.\nThis field is read-only."]
        #[serde(
            rename = "protectedRangeId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub protected_range_id: ::std::option::Option<i32>,
        #[doc = "The range that is being protected.\nThe range may be fully unbounded, in which case this is considered\na protected sheet.\n\nWhen writing, only one of range or named_range_id\nmay be set."]
        #[serde(
            rename = "range",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub range: ::std::option::Option<crate::schemas::GridRange>,
        #[doc = "True if the user who requested this protected range can edit the\nprotected area.\nThis field is read-only."]
        #[serde(
            rename = "requestingUserCanEdit",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub requesting_user_can_edit: ::std::option::Option<bool>,
        #[doc = "The list of unprotected ranges within a protected sheet.\nUnprotected ranges are only supported on protected sheets."]
        #[serde(
            rename = "unprotectedRanges",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub unprotected_ranges: ::std::option::Option<Vec<crate::schemas::GridRange>>,
        #[doc = "True if this protected range will show a warning when editing.\nWarning-based protection means that every user can edit data in the\nprotected range, except editing will prompt a warning asking the user\nto confirm the edit.\n\nWhen writing: if this field is true, then editors is ignored.\nAdditionally, if this field is changed from true to false and the\n`editors` field is not set (nor included in the field mask), then\nthe editors will be set to all the editors in the document."]
        #[serde(
            rename = "warningOnly",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub warning_only: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for ProtectedRange {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ProtectedRange {
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
    pub struct RandomizeRangeRequest {
        #[doc = "The range to randomize."]
        #[serde(
            rename = "range",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub range: ::std::option::Option<crate::schemas::GridRange>,
    }
    impl ::google_field_selector::FieldSelector for RandomizeRangeRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RandomizeRangeRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct RepeatCellRequest {
        #[doc = "The data to write."]
        #[serde(
            rename = "cell",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cell: ::std::option::Option<crate::schemas::CellData>,
        #[doc = "The fields that should be updated.  At least one field must be specified.\nThe root `cell` is implied and should not be specified.\nA single `\"*\"` can be used as short-hand for listing every field."]
        #[serde(
            rename = "fields",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fields: ::std::option::Option<String>,
        #[doc = "The range to repeat the cell in."]
        #[serde(
            rename = "range",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub range: ::std::option::Option<crate::schemas::GridRange>,
    }
    impl ::google_field_selector::FieldSelector for RepeatCellRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RepeatCellRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Request {
        #[doc = "Adds a new banded range"]
        #[serde(
            rename = "addBanding",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub add_banding: ::std::option::Option<crate::schemas::AddBandingRequest>,
        #[doc = "Adds a chart."]
        #[serde(
            rename = "addChart",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub add_chart: ::std::option::Option<crate::schemas::AddChartRequest>,
        #[doc = "Adds a new conditional format rule."]
        #[serde(
            rename = "addConditionalFormatRule",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub add_conditional_format_rule:
            ::std::option::Option<crate::schemas::AddConditionalFormatRuleRequest>,
        #[doc = "Creates a group over the specified range."]
        #[serde(
            rename = "addDimensionGroup",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub add_dimension_group: ::std::option::Option<crate::schemas::AddDimensionGroupRequest>,
        #[doc = "Adds a filter view."]
        #[serde(
            rename = "addFilterView",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub add_filter_view: ::std::option::Option<crate::schemas::AddFilterViewRequest>,
        #[doc = "Adds a named range."]
        #[serde(
            rename = "addNamedRange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub add_named_range: ::std::option::Option<crate::schemas::AddNamedRangeRequest>,
        #[doc = "Adds a protected range."]
        #[serde(
            rename = "addProtectedRange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub add_protected_range: ::std::option::Option<crate::schemas::AddProtectedRangeRequest>,
        #[doc = "Adds a sheet."]
        #[serde(
            rename = "addSheet",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub add_sheet: ::std::option::Option<crate::schemas::AddSheetRequest>,
        #[doc = "Adds a slicer."]
        #[serde(
            rename = "addSlicer",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub add_slicer: ::std::option::Option<crate::schemas::AddSlicerRequest>,
        #[doc = "Appends cells after the last row with data in a sheet."]
        #[serde(
            rename = "appendCells",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub append_cells: ::std::option::Option<crate::schemas::AppendCellsRequest>,
        #[doc = "Appends dimensions to the end of a sheet."]
        #[serde(
            rename = "appendDimension",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub append_dimension: ::std::option::Option<crate::schemas::AppendDimensionRequest>,
        #[doc = "Automatically fills in more data based on existing data."]
        #[serde(
            rename = "autoFill",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub auto_fill: ::std::option::Option<crate::schemas::AutoFillRequest>,
        #[doc = "Automatically resizes one or more dimensions based on the contents\nof the cells in that dimension."]
        #[serde(
            rename = "autoResizeDimensions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub auto_resize_dimensions:
            ::std::option::Option<crate::schemas::AutoResizeDimensionsRequest>,
        #[doc = "Clears the basic filter on a sheet."]
        #[serde(
            rename = "clearBasicFilter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub clear_basic_filter: ::std::option::Option<crate::schemas::ClearBasicFilterRequest>,
        #[doc = "Copies data from one area and pastes it to another."]
        #[serde(
            rename = "copyPaste",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub copy_paste: ::std::option::Option<crate::schemas::CopyPasteRequest>,
        #[doc = "Creates new developer metadata"]
        #[serde(
            rename = "createDeveloperMetadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_developer_metadata:
            ::std::option::Option<crate::schemas::CreateDeveloperMetadataRequest>,
        #[doc = "Cuts data from one area and pastes it to another."]
        #[serde(
            rename = "cutPaste",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cut_paste: ::std::option::Option<crate::schemas::CutPasteRequest>,
        #[doc = "Removes a banded range"]
        #[serde(
            rename = "deleteBanding",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub delete_banding: ::std::option::Option<crate::schemas::DeleteBandingRequest>,
        #[doc = "Deletes an existing conditional format rule."]
        #[serde(
            rename = "deleteConditionalFormatRule",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub delete_conditional_format_rule:
            ::std::option::Option<crate::schemas::DeleteConditionalFormatRuleRequest>,
        #[doc = "Deletes developer metadata"]
        #[serde(
            rename = "deleteDeveloperMetadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub delete_developer_metadata:
            ::std::option::Option<crate::schemas::DeleteDeveloperMetadataRequest>,
        #[doc = "Deletes rows or columns in a sheet."]
        #[serde(
            rename = "deleteDimension",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub delete_dimension: ::std::option::Option<crate::schemas::DeleteDimensionRequest>,
        #[doc = "Deletes a group over the specified range."]
        #[serde(
            rename = "deleteDimensionGroup",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub delete_dimension_group:
            ::std::option::Option<crate::schemas::DeleteDimensionGroupRequest>,
        #[doc = "Removes rows containing duplicate values in specified columns of a cell\nrange."]
        #[serde(
            rename = "deleteDuplicates",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub delete_duplicates: ::std::option::Option<crate::schemas::DeleteDuplicatesRequest>,
        #[doc = "Deletes an embedded object (e.g, chart, image) in a sheet."]
        #[serde(
            rename = "deleteEmbeddedObject",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub delete_embedded_object:
            ::std::option::Option<crate::schemas::DeleteEmbeddedObjectRequest>,
        #[doc = "Deletes a filter view from a sheet."]
        #[serde(
            rename = "deleteFilterView",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub delete_filter_view: ::std::option::Option<crate::schemas::DeleteFilterViewRequest>,
        #[doc = "Deletes a named range."]
        #[serde(
            rename = "deleteNamedRange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub delete_named_range: ::std::option::Option<crate::schemas::DeleteNamedRangeRequest>,
        #[doc = "Deletes a protected range."]
        #[serde(
            rename = "deleteProtectedRange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub delete_protected_range:
            ::std::option::Option<crate::schemas::DeleteProtectedRangeRequest>,
        #[doc = "Deletes a range of cells from a sheet, shifting the remaining cells."]
        #[serde(
            rename = "deleteRange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub delete_range: ::std::option::Option<crate::schemas::DeleteRangeRequest>,
        #[doc = "Deletes a sheet."]
        #[serde(
            rename = "deleteSheet",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub delete_sheet: ::std::option::Option<crate::schemas::DeleteSheetRequest>,
        #[doc = "Duplicates a filter view."]
        #[serde(
            rename = "duplicateFilterView",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub duplicate_filter_view:
            ::std::option::Option<crate::schemas::DuplicateFilterViewRequest>,
        #[doc = "Duplicates a sheet."]
        #[serde(
            rename = "duplicateSheet",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub duplicate_sheet: ::std::option::Option<crate::schemas::DuplicateSheetRequest>,
        #[doc = "Finds and replaces occurrences of some text with other text."]
        #[serde(
            rename = "findReplace",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub find_replace: ::std::option::Option<crate::schemas::FindReplaceRequest>,
        #[doc = "Inserts new rows or columns in a sheet."]
        #[serde(
            rename = "insertDimension",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub insert_dimension: ::std::option::Option<crate::schemas::InsertDimensionRequest>,
        #[doc = "Inserts new cells in a sheet, shifting the existing cells."]
        #[serde(
            rename = "insertRange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub insert_range: ::std::option::Option<crate::schemas::InsertRangeRequest>,
        #[doc = "Merges cells together."]
        #[serde(
            rename = "mergeCells",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub merge_cells: ::std::option::Option<crate::schemas::MergeCellsRequest>,
        #[doc = "Moves rows or columns to another location in a sheet."]
        #[serde(
            rename = "moveDimension",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub move_dimension: ::std::option::Option<crate::schemas::MoveDimensionRequest>,
        #[doc = "Pastes data (HTML or delimited) into a sheet."]
        #[serde(
            rename = "pasteData",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub paste_data: ::std::option::Option<crate::schemas::PasteDataRequest>,
        #[doc = "Randomizes the order of the rows in a range."]
        #[serde(
            rename = "randomizeRange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub randomize_range: ::std::option::Option<crate::schemas::RandomizeRangeRequest>,
        #[doc = "Repeats a single cell across a range."]
        #[serde(
            rename = "repeatCell",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub repeat_cell: ::std::option::Option<crate::schemas::RepeatCellRequest>,
        #[doc = "Sets the basic filter on a sheet."]
        #[serde(
            rename = "setBasicFilter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub set_basic_filter: ::std::option::Option<crate::schemas::SetBasicFilterRequest>,
        #[doc = "Sets data validation for one or more cells."]
        #[serde(
            rename = "setDataValidation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub set_data_validation: ::std::option::Option<crate::schemas::SetDataValidationRequest>,
        #[doc = "Sorts data in a range."]
        #[serde(
            rename = "sortRange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sort_range: ::std::option::Option<crate::schemas::SortRangeRequest>,
        #[doc = "Converts a column of text into many columns of text."]
        #[serde(
            rename = "textToColumns",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text_to_columns: ::std::option::Option<crate::schemas::TextToColumnsRequest>,
        #[doc = "Trims cells of whitespace (such as spaces, tabs, or new lines)."]
        #[serde(
            rename = "trimWhitespace",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub trim_whitespace: ::std::option::Option<crate::schemas::TrimWhitespaceRequest>,
        #[doc = "Unmerges merged cells."]
        #[serde(
            rename = "unmergeCells",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub unmerge_cells: ::std::option::Option<crate::schemas::UnmergeCellsRequest>,
        #[doc = "Updates a banded range"]
        #[serde(
            rename = "updateBanding",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_banding: ::std::option::Option<crate::schemas::UpdateBandingRequest>,
        #[doc = "Updates the borders in a range of cells."]
        #[serde(
            rename = "updateBorders",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_borders: ::std::option::Option<crate::schemas::UpdateBordersRequest>,
        #[doc = "Updates many cells at once."]
        #[serde(
            rename = "updateCells",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_cells: ::std::option::Option<crate::schemas::UpdateCellsRequest>,
        #[doc = "Updates a chart's specifications."]
        #[serde(
            rename = "updateChartSpec",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_chart_spec: ::std::option::Option<crate::schemas::UpdateChartSpecRequest>,
        #[doc = "Updates an existing conditional format rule."]
        #[serde(
            rename = "updateConditionalFormatRule",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_conditional_format_rule:
            ::std::option::Option<crate::schemas::UpdateConditionalFormatRuleRequest>,
        #[doc = "Updates an existing developer metadata entry"]
        #[serde(
            rename = "updateDeveloperMetadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_developer_metadata:
            ::std::option::Option<crate::schemas::UpdateDeveloperMetadataRequest>,
        #[doc = "Updates the state of the specified group."]
        #[serde(
            rename = "updateDimensionGroup",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_dimension_group:
            ::std::option::Option<crate::schemas::UpdateDimensionGroupRequest>,
        #[doc = "Updates dimensions' properties."]
        #[serde(
            rename = "updateDimensionProperties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_dimension_properties:
            ::std::option::Option<crate::schemas::UpdateDimensionPropertiesRequest>,
        #[doc = "Updates an embedded object's (e.g. chart, image) position."]
        #[serde(
            rename = "updateEmbeddedObjectPosition",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_embedded_object_position:
            ::std::option::Option<crate::schemas::UpdateEmbeddedObjectPositionRequest>,
        #[doc = "Updates the properties of a filter view."]
        #[serde(
            rename = "updateFilterView",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_filter_view: ::std::option::Option<crate::schemas::UpdateFilterViewRequest>,
        #[doc = "Updates a named range."]
        #[serde(
            rename = "updateNamedRange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_named_range: ::std::option::Option<crate::schemas::UpdateNamedRangeRequest>,
        #[doc = "Updates a protected range."]
        #[serde(
            rename = "updateProtectedRange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_protected_range:
            ::std::option::Option<crate::schemas::UpdateProtectedRangeRequest>,
        #[doc = "Updates a sheet's properties."]
        #[serde(
            rename = "updateSheetProperties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_sheet_properties:
            ::std::option::Option<crate::schemas::UpdateSheetPropertiesRequest>,
        #[doc = "Updates a slicer's specifications."]
        #[serde(
            rename = "updateSlicerSpec",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_slicer_spec: ::std::option::Option<crate::schemas::UpdateSlicerSpecRequest>,
        #[doc = "Updates the spreadsheet's properties."]
        #[serde(
            rename = "updateSpreadsheetProperties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_spreadsheet_properties:
            ::std::option::Option<crate::schemas::UpdateSpreadsheetPropertiesRequest>,
    }
    impl ::google_field_selector::FieldSelector for Request {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Request {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Response {
        #[doc = "A reply from adding a banded range."]
        #[serde(
            rename = "addBanding",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub add_banding: ::std::option::Option<crate::schemas::AddBandingResponse>,
        #[doc = "A reply from adding a chart."]
        #[serde(
            rename = "addChart",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub add_chart: ::std::option::Option<crate::schemas::AddChartResponse>,
        #[doc = "A reply from adding a dimension group."]
        #[serde(
            rename = "addDimensionGroup",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub add_dimension_group: ::std::option::Option<crate::schemas::AddDimensionGroupResponse>,
        #[doc = "A reply from adding a filter view."]
        #[serde(
            rename = "addFilterView",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub add_filter_view: ::std::option::Option<crate::schemas::AddFilterViewResponse>,
        #[doc = "A reply from adding a named range."]
        #[serde(
            rename = "addNamedRange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub add_named_range: ::std::option::Option<crate::schemas::AddNamedRangeResponse>,
        #[doc = "A reply from adding a protected range."]
        #[serde(
            rename = "addProtectedRange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub add_protected_range: ::std::option::Option<crate::schemas::AddProtectedRangeResponse>,
        #[doc = "A reply from adding a sheet."]
        #[serde(
            rename = "addSheet",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub add_sheet: ::std::option::Option<crate::schemas::AddSheetResponse>,
        #[doc = "A reply from adding a slicer."]
        #[serde(
            rename = "addSlicer",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub add_slicer: ::std::option::Option<crate::schemas::AddSlicerResponse>,
        #[doc = "A reply from creating a developer metadata entry."]
        #[serde(
            rename = "createDeveloperMetadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_developer_metadata:
            ::std::option::Option<crate::schemas::CreateDeveloperMetadataResponse>,
        #[doc = "A reply from deleting a conditional format rule."]
        #[serde(
            rename = "deleteConditionalFormatRule",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub delete_conditional_format_rule:
            ::std::option::Option<crate::schemas::DeleteConditionalFormatRuleResponse>,
        #[doc = "A reply from deleting a developer metadata entry."]
        #[serde(
            rename = "deleteDeveloperMetadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub delete_developer_metadata:
            ::std::option::Option<crate::schemas::DeleteDeveloperMetadataResponse>,
        #[doc = "A reply from deleting a dimension group."]
        #[serde(
            rename = "deleteDimensionGroup",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub delete_dimension_group:
            ::std::option::Option<crate::schemas::DeleteDimensionGroupResponse>,
        #[doc = "A reply from removing rows containing duplicate values."]
        #[serde(
            rename = "deleteDuplicates",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub delete_duplicates: ::std::option::Option<crate::schemas::DeleteDuplicatesResponse>,
        #[doc = "A reply from duplicating a filter view."]
        #[serde(
            rename = "duplicateFilterView",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub duplicate_filter_view:
            ::std::option::Option<crate::schemas::DuplicateFilterViewResponse>,
        #[doc = "A reply from duplicating a sheet."]
        #[serde(
            rename = "duplicateSheet",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub duplicate_sheet: ::std::option::Option<crate::schemas::DuplicateSheetResponse>,
        #[doc = "A reply from doing a find/replace."]
        #[serde(
            rename = "findReplace",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub find_replace: ::std::option::Option<crate::schemas::FindReplaceResponse>,
        #[doc = "A reply from trimming whitespace."]
        #[serde(
            rename = "trimWhitespace",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub trim_whitespace: ::std::option::Option<crate::schemas::TrimWhitespaceResponse>,
        #[doc = "A reply from updating a conditional format rule."]
        #[serde(
            rename = "updateConditionalFormatRule",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_conditional_format_rule:
            ::std::option::Option<crate::schemas::UpdateConditionalFormatRuleResponse>,
        #[doc = "A reply from updating a developer metadata entry."]
        #[serde(
            rename = "updateDeveloperMetadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_developer_metadata:
            ::std::option::Option<crate::schemas::UpdateDeveloperMetadataResponse>,
        #[doc = "A reply from updating an embedded object's position."]
        #[serde(
            rename = "updateEmbeddedObjectPosition",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_embedded_object_position:
            ::std::option::Option<crate::schemas::UpdateEmbeddedObjectPositionResponse>,
    }
    impl ::google_field_selector::FieldSelector for Response {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Response {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct RowData {
        #[doc = "The values in the row, one per column."]
        #[serde(
            rename = "values",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub values: ::std::option::Option<Vec<crate::schemas::CellData>>,
    }
    impl ::google_field_selector::FieldSelector for RowData {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RowData {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ScorecardChartSpec {
        #[doc = "The aggregation type for key and baseline chart data in scorecard chart.\nThis field is optional."]
        #[serde(
            rename = "aggregateType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub aggregate_type: ::std::option::Option<crate::schemas::ScorecardChartSpecAggregateType>,
        #[doc = "The data for scorecard baseline value.\nThis field is optional."]
        #[serde(
            rename = "baselineValueData",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub baseline_value_data: ::std::option::Option<crate::schemas::ChartData>,
        #[doc = "Formatting options for baseline value.\nThis field is needed only if baseline_value_data is specified."]
        #[serde(
            rename = "baselineValueFormat",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub baseline_value_format: ::std::option::Option<crate::schemas::BaselineValueFormat>,
        #[doc = "Custom formatting options for numeric key/baseline values in scorecard\nchart. This field is used only when number_format_source is set to\nCUSTOM. This field is optional."]
        #[serde(
            rename = "customFormatOptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub custom_format_options:
            ::std::option::Option<crate::schemas::ChartCustomNumberFormatOptions>,
        #[doc = "The data for scorecard key value."]
        #[serde(
            rename = "keyValueData",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub key_value_data: ::std::option::Option<crate::schemas::ChartData>,
        #[doc = "Formatting options for key value."]
        #[serde(
            rename = "keyValueFormat",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub key_value_format: ::std::option::Option<crate::schemas::KeyValueFormat>,
        #[doc = "The number format source used in the scorecard chart.\nThis field is optional."]
        #[serde(
            rename = "numberFormatSource",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub number_format_source:
            ::std::option::Option<crate::schemas::ScorecardChartSpecNumberFormatSource>,
        #[doc = "Value to scale scorecard key and baseline value. For example, a factor of\n10 can be used to divide all values in the chart by 10.\nThis field is optional."]
        #[serde(
            rename = "scaleFactor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub scale_factor: ::std::option::Option<f64>,
    }
    impl ::google_field_selector::FieldSelector for ScorecardChartSpec {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ScorecardChartSpec {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ScorecardChartSpecAggregateType {
        #[doc = "Average aggregate function."]
        Average,
        #[doc = "Default value, do not use."]
        ChartAggregateTypeUnspecified,
        #[doc = "Count aggregate function."]
        Count,
        #[doc = "Maximum aggregate function."]
        Max,
        #[doc = "Median aggregate function."]
        Median,
        #[doc = "Minimum aggregate function."]
        Min,
        #[doc = "Sum aggregate function."]
        Sum,
    }
    impl ScorecardChartSpecAggregateType {
        pub fn as_str(self) -> &'static str {
            match self {
                ScorecardChartSpecAggregateType::Average => "AVERAGE",
                ScorecardChartSpecAggregateType::ChartAggregateTypeUnspecified => {
                    "CHART_AGGREGATE_TYPE_UNSPECIFIED"
                }
                ScorecardChartSpecAggregateType::Count => "COUNT",
                ScorecardChartSpecAggregateType::Max => "MAX",
                ScorecardChartSpecAggregateType::Median => "MEDIAN",
                ScorecardChartSpecAggregateType::Min => "MIN",
                ScorecardChartSpecAggregateType::Sum => "SUM",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ScorecardChartSpecAggregateType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ScorecardChartSpecAggregateType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ScorecardChartSpecAggregateType, ()> {
            Ok(match s {
                "AVERAGE" => ScorecardChartSpecAggregateType::Average,
                "CHART_AGGREGATE_TYPE_UNSPECIFIED" => {
                    ScorecardChartSpecAggregateType::ChartAggregateTypeUnspecified
                }
                "COUNT" => ScorecardChartSpecAggregateType::Count,
                "MAX" => ScorecardChartSpecAggregateType::Max,
                "MEDIAN" => ScorecardChartSpecAggregateType::Median,
                "MIN" => ScorecardChartSpecAggregateType::Min,
                "SUM" => ScorecardChartSpecAggregateType::Sum,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ScorecardChartSpecAggregateType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ScorecardChartSpecAggregateType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ScorecardChartSpecAggregateType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "AVERAGE" => ScorecardChartSpecAggregateType::Average,
                "CHART_AGGREGATE_TYPE_UNSPECIFIED" => {
                    ScorecardChartSpecAggregateType::ChartAggregateTypeUnspecified
                }
                "COUNT" => ScorecardChartSpecAggregateType::Count,
                "MAX" => ScorecardChartSpecAggregateType::Max,
                "MEDIAN" => ScorecardChartSpecAggregateType::Median,
                "MIN" => ScorecardChartSpecAggregateType::Min,
                "SUM" => ScorecardChartSpecAggregateType::Sum,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ScorecardChartSpecAggregateType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ScorecardChartSpecAggregateType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ScorecardChartSpecNumberFormatSource {
        #[doc = "Default value, do not use."]
        ChartNumberFormatSourceUndefined,
        #[doc = "Apply custom formatting as specified by ChartCustomNumberFormatOptions."]
        Custom,
        #[doc = "Inherit number formatting from data."]
        FromData,
    }
    impl ScorecardChartSpecNumberFormatSource {
        pub fn as_str(self) -> &'static str {
            match self {
                ScorecardChartSpecNumberFormatSource::ChartNumberFormatSourceUndefined => {
                    "CHART_NUMBER_FORMAT_SOURCE_UNDEFINED"
                }
                ScorecardChartSpecNumberFormatSource::Custom => "CUSTOM",
                ScorecardChartSpecNumberFormatSource::FromData => "FROM_DATA",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ScorecardChartSpecNumberFormatSource {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ScorecardChartSpecNumberFormatSource {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ScorecardChartSpecNumberFormatSource, ()> {
            Ok(match s {
                "CHART_NUMBER_FORMAT_SOURCE_UNDEFINED" => {
                    ScorecardChartSpecNumberFormatSource::ChartNumberFormatSourceUndefined
                }
                "CUSTOM" => ScorecardChartSpecNumberFormatSource::Custom,
                "FROM_DATA" => ScorecardChartSpecNumberFormatSource::FromData,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ScorecardChartSpecNumberFormatSource {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ScorecardChartSpecNumberFormatSource {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ScorecardChartSpecNumberFormatSource {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CHART_NUMBER_FORMAT_SOURCE_UNDEFINED" => {
                    ScorecardChartSpecNumberFormatSource::ChartNumberFormatSourceUndefined
                }
                "CUSTOM" => ScorecardChartSpecNumberFormatSource::Custom,
                "FROM_DATA" => ScorecardChartSpecNumberFormatSource::FromData,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ScorecardChartSpecNumberFormatSource {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ScorecardChartSpecNumberFormatSource {
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
    pub struct SearchDeveloperMetadataRequest {
        #[doc = "The data filters describing the criteria used to determine which\nDeveloperMetadata entries to return.  DeveloperMetadata matching any of the\nspecified filters are included in the response."]
        #[serde(
            rename = "dataFilters",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub data_filters: ::std::option::Option<Vec<crate::schemas::DataFilter>>,
    }
    impl ::google_field_selector::FieldSelector for SearchDeveloperMetadataRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SearchDeveloperMetadataRequest {
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
    pub struct SearchDeveloperMetadataResponse {
        #[doc = "The metadata matching the criteria of the search request."]
        #[serde(
            rename = "matchedDeveloperMetadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub matched_developer_metadata:
            ::std::option::Option<Vec<crate::schemas::MatchedDeveloperMetadata>>,
    }
    impl ::google_field_selector::FieldSelector for SearchDeveloperMetadataResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SearchDeveloperMetadataResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct SetBasicFilterRequest {
        #[doc = "The filter to set."]
        #[serde(
            rename = "filter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub filter: ::std::option::Option<crate::schemas::BasicFilter>,
    }
    impl ::google_field_selector::FieldSelector for SetBasicFilterRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SetBasicFilterRequest {
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
    pub struct SetDataValidationRequest {
        #[doc = "The range the data validation rule should apply to."]
        #[serde(
            rename = "range",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub range: ::std::option::Option<crate::schemas::GridRange>,
        #[doc = "The data validation rule to set on each cell in the range,\nor empty to clear the data validation in the range."]
        #[serde(
            rename = "rule",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rule: ::std::option::Option<crate::schemas::DataValidationRule>,
    }
    impl ::google_field_selector::FieldSelector for SetDataValidationRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SetDataValidationRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Sheet {
        #[doc = "The banded (alternating colors) ranges on this sheet."]
        #[serde(
            rename = "bandedRanges",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub banded_ranges: ::std::option::Option<Vec<crate::schemas::BandedRange>>,
        #[doc = "The filter on this sheet, if any."]
        #[serde(
            rename = "basicFilter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub basic_filter: ::std::option::Option<crate::schemas::BasicFilter>,
        #[doc = "The specifications of every chart on this sheet."]
        #[serde(
            rename = "charts",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub charts: ::std::option::Option<Vec<crate::schemas::EmbeddedChart>>,
        #[doc = "All column groups on this sheet, ordered by increasing range start index,\nthen by group depth."]
        #[serde(
            rename = "columnGroups",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub column_groups: ::std::option::Option<Vec<crate::schemas::DimensionGroup>>,
        #[doc = "The conditional format rules in this sheet."]
        #[serde(
            rename = "conditionalFormats",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub conditional_formats: ::std::option::Option<Vec<crate::schemas::ConditionalFormatRule>>,
        #[doc = "Data in the grid, if this is a grid sheet.\n\nThe number of GridData objects returned is dependent on the number of\nranges requested on this sheet. For example, if this is representing\n`Sheet1`, and the spreadsheet was requested with ranges\n`Sheet1!A1:C10` and `Sheet1!D15:E20`, then the first GridData will have a\nstartRow/startColumn of `0`,\nwhile the second one will have `startRow 14` (zero-based row 15),\nand `startColumn 3` (zero-based column D)."]
        #[serde(
            rename = "data",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub data: ::std::option::Option<Vec<crate::schemas::GridData>>,
        #[doc = "The developer metadata associated with a sheet."]
        #[serde(
            rename = "developerMetadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub developer_metadata: ::std::option::Option<Vec<crate::schemas::DeveloperMetadata>>,
        #[doc = "The filter views in this sheet."]
        #[serde(
            rename = "filterViews",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub filter_views: ::std::option::Option<Vec<crate::schemas::FilterView>>,
        #[doc = "The ranges that are merged together."]
        #[serde(
            rename = "merges",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub merges: ::std::option::Option<Vec<crate::schemas::GridRange>>,
        #[doc = "The properties of the sheet."]
        #[serde(
            rename = "properties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub properties: ::std::option::Option<crate::schemas::SheetProperties>,
        #[doc = "The protected ranges in this sheet."]
        #[serde(
            rename = "protectedRanges",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub protected_ranges: ::std::option::Option<Vec<crate::schemas::ProtectedRange>>,
        #[doc = "All row groups on this sheet, ordered by increasing range start index, then\nby group depth."]
        #[serde(
            rename = "rowGroups",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub row_groups: ::std::option::Option<Vec<crate::schemas::DimensionGroup>>,
        #[doc = "The slicers on this sheet."]
        #[serde(
            rename = "slicers",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub slicers: ::std::option::Option<Vec<crate::schemas::Slicer>>,
    }
    impl ::google_field_selector::FieldSelector for Sheet {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Sheet {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct SheetProperties {
        #[doc = "Additional properties of the sheet if this sheet is a grid.\n(If the sheet is an object sheet, containing a chart or image, then\nthis field will be absent.)\nWhen writing it is an error to set any grid properties on non-grid sheets."]
        #[serde(
            rename = "gridProperties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub grid_properties: ::std::option::Option<crate::schemas::GridProperties>,
        #[doc = "True if the sheet is hidden in the UI, false if it's visible."]
        #[serde(
            rename = "hidden",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub hidden: ::std::option::Option<bool>,
        #[doc = "The index of the sheet within the spreadsheet.\nWhen adding or updating sheet properties, if this field\nis excluded then the sheet is added or moved to the end\nof the sheet list. When updating sheet indices or inserting\nsheets, movement is considered in \"before the move\" indexes.\nFor example, if there were 3 sheets (S1, S2, S3) in order to\nmove S1 ahead of S2 the index would have to be set to 2. A sheet\nindex update request is ignored if the requested index is\nidentical to the sheets current index or if the requested new\nindex is equal to the current sheet index + 1."]
        #[serde(
            rename = "index",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub index: ::std::option::Option<i32>,
        #[doc = "True if the sheet is an RTL sheet instead of an LTR sheet."]
        #[serde(
            rename = "rightToLeft",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub right_to_left: ::std::option::Option<bool>,
        #[doc = "The ID of the sheet. Must be non-negative.\nThis field cannot be changed once set."]
        #[serde(
            rename = "sheetId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sheet_id: ::std::option::Option<i32>,
        #[doc = "The type of sheet. Defaults to GRID.\nThis field cannot be changed once set."]
        #[serde(
            rename = "sheetType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sheet_type: ::std::option::Option<crate::schemas::SheetPropertiesSheetType>,
        #[doc = "The color of the tab in the UI."]
        #[serde(
            rename = "tabColor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub tab_color: ::std::option::Option<crate::schemas::Color>,
        #[doc = "The color of the tab in the UI.\nIf tab_color is also set, this field takes precedence."]
        #[serde(
            rename = "tabColorStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub tab_color_style: ::std::option::Option<crate::schemas::ColorStyle>,
        #[doc = "The name of the sheet."]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for SheetProperties {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SheetProperties {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum SheetPropertiesSheetType {
        #[doc = "The sheet is a grid."]
        Grid,
        #[doc = "The sheet has no grid and instead has an object like a chart or image."]
        Object,
        #[doc = "Default value, do not use."]
        SheetTypeUnspecified,
    }
    impl SheetPropertiesSheetType {
        pub fn as_str(self) -> &'static str {
            match self {
                SheetPropertiesSheetType::Grid => "GRID",
                SheetPropertiesSheetType::Object => "OBJECT",
                SheetPropertiesSheetType::SheetTypeUnspecified => "SHEET_TYPE_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for SheetPropertiesSheetType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for SheetPropertiesSheetType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<SheetPropertiesSheetType, ()> {
            Ok(match s {
                "GRID" => SheetPropertiesSheetType::Grid,
                "OBJECT" => SheetPropertiesSheetType::Object,
                "SHEET_TYPE_UNSPECIFIED" => SheetPropertiesSheetType::SheetTypeUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for SheetPropertiesSheetType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for SheetPropertiesSheetType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SheetPropertiesSheetType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "GRID" => SheetPropertiesSheetType::Grid,
                "OBJECT" => SheetPropertiesSheetType::Object,
                "SHEET_TYPE_UNSPECIFIED" => SheetPropertiesSheetType::SheetTypeUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for SheetPropertiesSheetType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SheetPropertiesSheetType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Slicer {
        #[doc = "The position of the slicer. Note that slicer can be positioned only on\nexisting sheet. Also, width and height of slicer can be automatically\nadjusted to keep it within permitted limits."]
        #[serde(
            rename = "position",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub position: ::std::option::Option<crate::schemas::EmbeddedObjectPosition>,
        #[doc = "The ID of the slicer."]
        #[serde(
            rename = "slicerId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub slicer_id: ::std::option::Option<i32>,
        #[doc = "The specification of the slicer."]
        #[serde(
            rename = "spec",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub spec: ::std::option::Option<crate::schemas::SlicerSpec>,
    }
    impl ::google_field_selector::FieldSelector for Slicer {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Slicer {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct SlicerSpec {
        #[doc = "True if the filter should apply to pivot tables.\nIf not set, default to `True`."]
        #[serde(
            rename = "applyToPivotTables",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub apply_to_pivot_tables: ::std::option::Option<bool>,
        #[doc = "The background color of the slicer."]
        #[serde(
            rename = "backgroundColor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub background_color: ::std::option::Option<crate::schemas::Color>,
        #[doc = "The background color of the slicer.\nIf background_color is also set, this field takes precedence."]
        #[serde(
            rename = "backgroundColorStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub background_color_style: ::std::option::Option<crate::schemas::ColorStyle>,
        #[doc = "The column index in the data table on which the filter is applied to."]
        #[serde(
            rename = "columnIndex",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub column_index: ::std::option::Option<i32>,
        #[doc = "The data range of the slicer."]
        #[serde(
            rename = "dataRange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub data_range: ::std::option::Option<crate::schemas::GridRange>,
        #[doc = "The filtering criteria of the slicer."]
        #[serde(
            rename = "filterCriteria",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub filter_criteria: ::std::option::Option<crate::schemas::FilterCriteria>,
        #[doc = "The horizontal alignment of title in the slicer.\nIf unspecified, defaults to `LEFT`"]
        #[serde(
            rename = "horizontalAlignment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub horizontal_alignment:
            ::std::option::Option<crate::schemas::SlicerSpecHorizontalAlignment>,
        #[doc = "The text format of title in the slicer."]
        #[serde(
            rename = "textFormat",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text_format: ::std::option::Option<crate::schemas::TextFormat>,
        #[doc = "The title of the slicer."]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for SlicerSpec {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SlicerSpec {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum SlicerSpecHorizontalAlignment {
        #[doc = "The text is explicitly aligned to the center of the cell."]
        Center,
        #[doc = "The horizontal alignment is not specified. Do not use this."]
        HorizontalAlignUnspecified,
        #[doc = "The text is explicitly aligned to the left of the cell."]
        Left,
        #[doc = "The text is explicitly aligned to the right of the cell."]
        Right,
    }
    impl SlicerSpecHorizontalAlignment {
        pub fn as_str(self) -> &'static str {
            match self {
                SlicerSpecHorizontalAlignment::Center => "CENTER",
                SlicerSpecHorizontalAlignment::HorizontalAlignUnspecified => {
                    "HORIZONTAL_ALIGN_UNSPECIFIED"
                }
                SlicerSpecHorizontalAlignment::Left => "LEFT",
                SlicerSpecHorizontalAlignment::Right => "RIGHT",
            }
        }
    }
    impl ::std::convert::AsRef<str> for SlicerSpecHorizontalAlignment {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for SlicerSpecHorizontalAlignment {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<SlicerSpecHorizontalAlignment, ()> {
            Ok(match s {
                "CENTER" => SlicerSpecHorizontalAlignment::Center,
                "HORIZONTAL_ALIGN_UNSPECIFIED" => {
                    SlicerSpecHorizontalAlignment::HorizontalAlignUnspecified
                }
                "LEFT" => SlicerSpecHorizontalAlignment::Left,
                "RIGHT" => SlicerSpecHorizontalAlignment::Right,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for SlicerSpecHorizontalAlignment {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for SlicerSpecHorizontalAlignment {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SlicerSpecHorizontalAlignment {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CENTER" => SlicerSpecHorizontalAlignment::Center,
                "HORIZONTAL_ALIGN_UNSPECIFIED" => {
                    SlicerSpecHorizontalAlignment::HorizontalAlignUnspecified
                }
                "LEFT" => SlicerSpecHorizontalAlignment::Left,
                "RIGHT" => SlicerSpecHorizontalAlignment::Right,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for SlicerSpecHorizontalAlignment {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SlicerSpecHorizontalAlignment {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct SortRangeRequest {
        #[doc = "The range to sort."]
        #[serde(
            rename = "range",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub range: ::std::option::Option<crate::schemas::GridRange>,
        #[doc = "The sort order per column. Later specifications are used when values\nare equal in the earlier specifications."]
        #[serde(
            rename = "sortSpecs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sort_specs: ::std::option::Option<Vec<crate::schemas::SortSpec>>,
    }
    impl ::google_field_selector::FieldSelector for SortRangeRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SortRangeRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct SortSpec {
        #[doc = "The background fill color to sort by; cells with this fill color are sorted\nto the top. Mutually exclusive with foreground_color."]
        #[serde(
            rename = "backgroundColor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub background_color: ::std::option::Option<crate::schemas::Color>,
        #[doc = "The background fill color to sort by; cells with this fill color are sorted\nto the top. Mutually exclusive with foreground_color, and must be an\nRGB-type color. If background_color is also set, this field takes\nprecedence."]
        #[serde(
            rename = "backgroundColorStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub background_color_style: ::std::option::Option<crate::schemas::ColorStyle>,
        #[doc = "The dimension the sort should be applied to."]
        #[serde(
            rename = "dimensionIndex",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dimension_index: ::std::option::Option<i32>,
        #[doc = "The foreground color to sort by; cells with this foreground color are\nsorted to the top. Mutually exclusive with background_color."]
        #[serde(
            rename = "foregroundColor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub foreground_color: ::std::option::Option<crate::schemas::Color>,
        #[doc = "The foreground color to sort by; cells with this foreground color are\nsorted to the top. Mutually exclusive with background_color, and must\nbe an RGB-type color. If foreground_color is also set, this field takes\nprecedence."]
        #[serde(
            rename = "foregroundColorStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub foreground_color_style: ::std::option::Option<crate::schemas::ColorStyle>,
        #[doc = "The order data should be sorted."]
        #[serde(
            rename = "sortOrder",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sort_order: ::std::option::Option<crate::schemas::SortSpecSortOrder>,
    }
    impl ::google_field_selector::FieldSelector for SortSpec {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SortSpec {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum SortSpecSortOrder {
        #[doc = "Sort ascending."]
        Ascending,
        #[doc = "Sort descending."]
        Descending,
        #[doc = "Default value, do not use this."]
        SortOrderUnspecified,
    }
    impl SortSpecSortOrder {
        pub fn as_str(self) -> &'static str {
            match self {
                SortSpecSortOrder::Ascending => "ASCENDING",
                SortSpecSortOrder::Descending => "DESCENDING",
                SortSpecSortOrder::SortOrderUnspecified => "SORT_ORDER_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for SortSpecSortOrder {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for SortSpecSortOrder {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<SortSpecSortOrder, ()> {
            Ok(match s {
                "ASCENDING" => SortSpecSortOrder::Ascending,
                "DESCENDING" => SortSpecSortOrder::Descending,
                "SORT_ORDER_UNSPECIFIED" => SortSpecSortOrder::SortOrderUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for SortSpecSortOrder {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for SortSpecSortOrder {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SortSpecSortOrder {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ASCENDING" => SortSpecSortOrder::Ascending,
                "DESCENDING" => SortSpecSortOrder::Descending,
                "SORT_ORDER_UNSPECIFIED" => SortSpecSortOrder::SortOrderUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for SortSpecSortOrder {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SortSpecSortOrder {
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
    pub struct SourceAndDestination {
        #[doc = "The dimension that data should be filled into."]
        #[serde(
            rename = "dimension",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dimension: ::std::option::Option<crate::schemas::SourceAndDestinationDimension>,
        #[doc = "The number of rows or columns that data should be filled into.\nPositive numbers expand beyond the last row or last column\nof the source.  Negative numbers expand before the first row\nor first column of the source."]
        #[serde(
            rename = "fillLength",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fill_length: ::std::option::Option<i32>,
        #[doc = "The location of the data to use as the source of the autofill."]
        #[serde(
            rename = "source",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source: ::std::option::Option<crate::schemas::GridRange>,
    }
    impl ::google_field_selector::FieldSelector for SourceAndDestination {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SourceAndDestination {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum SourceAndDestinationDimension {
        #[doc = "Operates on the columns of a sheet."]
        Columns,
        #[doc = "The default value, do not use."]
        DimensionUnspecified,
        #[doc = "Operates on the rows of a sheet."]
        Rows,
    }
    impl SourceAndDestinationDimension {
        pub fn as_str(self) -> &'static str {
            match self {
                SourceAndDestinationDimension::Columns => "COLUMNS",
                SourceAndDestinationDimension::DimensionUnspecified => "DIMENSION_UNSPECIFIED",
                SourceAndDestinationDimension::Rows => "ROWS",
            }
        }
    }
    impl ::std::convert::AsRef<str> for SourceAndDestinationDimension {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for SourceAndDestinationDimension {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<SourceAndDestinationDimension, ()> {
            Ok(match s {
                "COLUMNS" => SourceAndDestinationDimension::Columns,
                "DIMENSION_UNSPECIFIED" => SourceAndDestinationDimension::DimensionUnspecified,
                "ROWS" => SourceAndDestinationDimension::Rows,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for SourceAndDestinationDimension {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for SourceAndDestinationDimension {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SourceAndDestinationDimension {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "COLUMNS" => SourceAndDestinationDimension::Columns,
                "DIMENSION_UNSPECIFIED" => SourceAndDestinationDimension::DimensionUnspecified,
                "ROWS" => SourceAndDestinationDimension::Rows,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for SourceAndDestinationDimension {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SourceAndDestinationDimension {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Spreadsheet {
        #[doc = "The developer metadata associated with a spreadsheet."]
        #[serde(
            rename = "developerMetadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub developer_metadata: ::std::option::Option<Vec<crate::schemas::DeveloperMetadata>>,
        #[doc = "The named ranges defined in a spreadsheet."]
        #[serde(
            rename = "namedRanges",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub named_ranges: ::std::option::Option<Vec<crate::schemas::NamedRange>>,
        #[doc = "Overall properties of a spreadsheet."]
        #[serde(
            rename = "properties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub properties: ::std::option::Option<crate::schemas::SpreadsheetProperties>,
        #[doc = "The sheets that are part of a spreadsheet."]
        #[serde(
            rename = "sheets",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sheets: ::std::option::Option<Vec<crate::schemas::Sheet>>,
        #[doc = "The ID of the spreadsheet.\nThis field is read-only."]
        #[serde(
            rename = "spreadsheetId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub spreadsheet_id: ::std::option::Option<String>,
        #[doc = "The url of the spreadsheet.\nThis field is read-only."]
        #[serde(
            rename = "spreadsheetUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub spreadsheet_url: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Spreadsheet {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Spreadsheet {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct SpreadsheetProperties {
        #[doc = "The amount of time to wait before volatile functions are recalculated."]
        #[serde(
            rename = "autoRecalc",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub auto_recalc: ::std::option::Option<crate::schemas::SpreadsheetPropertiesAutoRecalc>,
        #[doc = "The default format of all cells in the spreadsheet.\nCellData.effectiveFormat will not be set if\nthe cell's format is equal to this default format. This field is read-only."]
        #[serde(
            rename = "defaultFormat",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub default_format: ::std::option::Option<crate::schemas::CellFormat>,
        #[doc = "Determines whether and how circular references are resolved with iterative\ncalculation.  Absence of this field means that circular references result\nin calculation errors."]
        #[serde(
            rename = "iterativeCalculationSettings",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub iterative_calculation_settings:
            ::std::option::Option<crate::schemas::IterativeCalculationSettings>,
        #[doc = "The locale of the spreadsheet in one of the following formats:\n\n* an ISO 639-1 language code such as `en`\n\n* an ISO 639-2 language code such as `fil`, if no 639-1 code exists\n\n* a combination of the ISO language code and country code, such as `en_US`\n\nNote: when updating this field, not all locales/languages are supported."]
        #[serde(
            rename = "locale",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub locale: ::std::option::Option<String>,
        #[doc = "Theme applied to the spreadsheet."]
        #[serde(
            rename = "spreadsheetTheme",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub spreadsheet_theme: ::std::option::Option<crate::schemas::SpreadsheetTheme>,
        #[doc = "The time zone of the spreadsheet, in CLDR format such as\n`America/New_York`. If the time zone isn't recognized, this may\nbe a custom time zone such as `GMT-07:00`."]
        #[serde(
            rename = "timeZone",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub time_zone: ::std::option::Option<String>,
        #[doc = "The title of the spreadsheet."]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for SpreadsheetProperties {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SpreadsheetProperties {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum SpreadsheetPropertiesAutoRecalc {
        #[doc = "Volatile functions are updated on every change and hourly."]
        Hour,
        #[doc = "Volatile functions are updated on every change and every minute."]
        Minute,
        #[doc = "Volatile functions are updated on every change."]
        OnChange,
        #[doc = "Default value. This value must not be used."]
        RecalculationIntervalUnspecified,
    }
    impl SpreadsheetPropertiesAutoRecalc {
        pub fn as_str(self) -> &'static str {
            match self {
                SpreadsheetPropertiesAutoRecalc::Hour => "HOUR",
                SpreadsheetPropertiesAutoRecalc::Minute => "MINUTE",
                SpreadsheetPropertiesAutoRecalc::OnChange => "ON_CHANGE",
                SpreadsheetPropertiesAutoRecalc::RecalculationIntervalUnspecified => {
                    "RECALCULATION_INTERVAL_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for SpreadsheetPropertiesAutoRecalc {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for SpreadsheetPropertiesAutoRecalc {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<SpreadsheetPropertiesAutoRecalc, ()> {
            Ok(match s {
                "HOUR" => SpreadsheetPropertiesAutoRecalc::Hour,
                "MINUTE" => SpreadsheetPropertiesAutoRecalc::Minute,
                "ON_CHANGE" => SpreadsheetPropertiesAutoRecalc::OnChange,
                "RECALCULATION_INTERVAL_UNSPECIFIED" => {
                    SpreadsheetPropertiesAutoRecalc::RecalculationIntervalUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for SpreadsheetPropertiesAutoRecalc {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for SpreadsheetPropertiesAutoRecalc {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SpreadsheetPropertiesAutoRecalc {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "HOUR" => SpreadsheetPropertiesAutoRecalc::Hour,
                "MINUTE" => SpreadsheetPropertiesAutoRecalc::Minute,
                "ON_CHANGE" => SpreadsheetPropertiesAutoRecalc::OnChange,
                "RECALCULATION_INTERVAL_UNSPECIFIED" => {
                    SpreadsheetPropertiesAutoRecalc::RecalculationIntervalUnspecified
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
    impl ::google_field_selector::FieldSelector for SpreadsheetPropertiesAutoRecalc {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SpreadsheetPropertiesAutoRecalc {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct SpreadsheetTheme {
        #[doc = "/ Name of the primary font family."]
        #[serde(
            rename = "primaryFontFamily",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub primary_font_family: ::std::option::Option<String>,
        #[doc = "The spreadsheet theme color pairs. To update you must provide all theme\ncolor pairs."]
        #[serde(
            rename = "themeColors",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub theme_colors: ::std::option::Option<Vec<crate::schemas::ThemeColorPair>>,
    }
    impl ::google_field_selector::FieldSelector for SpreadsheetTheme {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SpreadsheetTheme {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct TextFormat {
        #[doc = "True if the text is bold."]
        #[serde(
            rename = "bold",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bold: ::std::option::Option<bool>,
        #[doc = "The font family."]
        #[serde(
            rename = "fontFamily",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub font_family: ::std::option::Option<String>,
        #[doc = "The size of the font."]
        #[serde(
            rename = "fontSize",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub font_size: ::std::option::Option<i32>,
        #[doc = "The foreground color of the text."]
        #[serde(
            rename = "foregroundColor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub foreground_color: ::std::option::Option<crate::schemas::Color>,
        #[doc = "The foreground color of the text.\nIf foreground_color is also set, this field takes precedence."]
        #[serde(
            rename = "foregroundColorStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub foreground_color_style: ::std::option::Option<crate::schemas::ColorStyle>,
        #[doc = "True if the text is italicized."]
        #[serde(
            rename = "italic",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub italic: ::std::option::Option<bool>,
        #[doc = "True if the text has a strikethrough."]
        #[serde(
            rename = "strikethrough",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub strikethrough: ::std::option::Option<bool>,
        #[doc = "True if the text is underlined."]
        #[serde(
            rename = "underline",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub underline: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for TextFormat {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TextFormat {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct TextFormatRun {
        #[doc = "The format of this run.  Absent values inherit the cell's format."]
        #[serde(
            rename = "format",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub format: ::std::option::Option<crate::schemas::TextFormat>,
        #[doc = "The character index where this run starts."]
        #[serde(
            rename = "startIndex",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_index: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for TextFormatRun {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TextFormatRun {
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
    pub struct TextPosition {
        #[doc = "Horizontal alignment setting for the piece of text."]
        #[serde(
            rename = "horizontalAlignment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub horizontal_alignment:
            ::std::option::Option<crate::schemas::TextPositionHorizontalAlignment>,
    }
    impl ::google_field_selector::FieldSelector for TextPosition {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TextPosition {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum TextPositionHorizontalAlignment {
        #[doc = "The text is explicitly aligned to the center of the cell."]
        Center,
        #[doc = "The horizontal alignment is not specified. Do not use this."]
        HorizontalAlignUnspecified,
        #[doc = "The text is explicitly aligned to the left of the cell."]
        Left,
        #[doc = "The text is explicitly aligned to the right of the cell."]
        Right,
    }
    impl TextPositionHorizontalAlignment {
        pub fn as_str(self) -> &'static str {
            match self {
                TextPositionHorizontalAlignment::Center => "CENTER",
                TextPositionHorizontalAlignment::HorizontalAlignUnspecified => {
                    "HORIZONTAL_ALIGN_UNSPECIFIED"
                }
                TextPositionHorizontalAlignment::Left => "LEFT",
                TextPositionHorizontalAlignment::Right => "RIGHT",
            }
        }
    }
    impl ::std::convert::AsRef<str> for TextPositionHorizontalAlignment {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for TextPositionHorizontalAlignment {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<TextPositionHorizontalAlignment, ()> {
            Ok(match s {
                "CENTER" => TextPositionHorizontalAlignment::Center,
                "HORIZONTAL_ALIGN_UNSPECIFIED" => {
                    TextPositionHorizontalAlignment::HorizontalAlignUnspecified
                }
                "LEFT" => TextPositionHorizontalAlignment::Left,
                "RIGHT" => TextPositionHorizontalAlignment::Right,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for TextPositionHorizontalAlignment {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for TextPositionHorizontalAlignment {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for TextPositionHorizontalAlignment {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CENTER" => TextPositionHorizontalAlignment::Center,
                "HORIZONTAL_ALIGN_UNSPECIFIED" => {
                    TextPositionHorizontalAlignment::HorizontalAlignUnspecified
                }
                "LEFT" => TextPositionHorizontalAlignment::Left,
                "RIGHT" => TextPositionHorizontalAlignment::Right,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for TextPositionHorizontalAlignment {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TextPositionHorizontalAlignment {
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
    pub struct TextRotation {
        #[doc = "The angle between the standard orientation and the desired orientation.\nMeasured in degrees. Valid values are between -90 and 90. Positive\nangles are angled upwards, negative are angled downwards.\n\nNote: For LTR text direction positive angles are in the\ncounterclockwise direction, whereas for RTL they are in the clockwise\ndirection"]
        #[serde(
            rename = "angle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub angle: ::std::option::Option<i32>,
        #[doc = "If true, text reads top to bottom, but the orientation of individual\ncharacters is unchanged.\nFor example:\n\n````text\n| V |\n| e |\n| r |\n| t |\n| i |\n| c |\n| a |\n| l |````"]
        #[serde(
            rename = "vertical",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub vertical: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for TextRotation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TextRotation {
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
    pub struct TextToColumnsRequest {
        #[doc = "The delimiter to use. Used only if delimiterType is\nCUSTOM."]
        #[serde(
            rename = "delimiter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub delimiter: ::std::option::Option<String>,
        #[doc = "The delimiter type to use."]
        #[serde(
            rename = "delimiterType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub delimiter_type:
            ::std::option::Option<crate::schemas::TextToColumnsRequestDelimiterType>,
        #[doc = "The source data range.  This must span exactly one column."]
        #[serde(
            rename = "source",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source: ::std::option::Option<crate::schemas::GridRange>,
    }
    impl ::google_field_selector::FieldSelector for TextToColumnsRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TextToColumnsRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum TextToColumnsRequestDelimiterType {
        #[doc = "Automatically detect columns."]
        Autodetect,
        #[doc = "\",\""]
        Comma,
        #[doc = "A custom value as defined in delimiter."]
        Custom,
        #[doc = "Default value. This value must not be used."]
        DelimiterTypeUnspecified,
        #[doc = "\".\""]
        Period,
        #[doc = "\";\""]
        Semicolon,
        #[doc = "\" \""]
        Space,
    }
    impl TextToColumnsRequestDelimiterType {
        pub fn as_str(self) -> &'static str {
            match self {
                TextToColumnsRequestDelimiterType::Autodetect => "AUTODETECT",
                TextToColumnsRequestDelimiterType::Comma => "COMMA",
                TextToColumnsRequestDelimiterType::Custom => "CUSTOM",
                TextToColumnsRequestDelimiterType::DelimiterTypeUnspecified => {
                    "DELIMITER_TYPE_UNSPECIFIED"
                }
                TextToColumnsRequestDelimiterType::Period => "PERIOD",
                TextToColumnsRequestDelimiterType::Semicolon => "SEMICOLON",
                TextToColumnsRequestDelimiterType::Space => "SPACE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for TextToColumnsRequestDelimiterType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for TextToColumnsRequestDelimiterType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<TextToColumnsRequestDelimiterType, ()> {
            Ok(match s {
                "AUTODETECT" => TextToColumnsRequestDelimiterType::Autodetect,
                "COMMA" => TextToColumnsRequestDelimiterType::Comma,
                "CUSTOM" => TextToColumnsRequestDelimiterType::Custom,
                "DELIMITER_TYPE_UNSPECIFIED" => {
                    TextToColumnsRequestDelimiterType::DelimiterTypeUnspecified
                }
                "PERIOD" => TextToColumnsRequestDelimiterType::Period,
                "SEMICOLON" => TextToColumnsRequestDelimiterType::Semicolon,
                "SPACE" => TextToColumnsRequestDelimiterType::Space,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for TextToColumnsRequestDelimiterType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for TextToColumnsRequestDelimiterType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for TextToColumnsRequestDelimiterType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "AUTODETECT" => TextToColumnsRequestDelimiterType::Autodetect,
                "COMMA" => TextToColumnsRequestDelimiterType::Comma,
                "CUSTOM" => TextToColumnsRequestDelimiterType::Custom,
                "DELIMITER_TYPE_UNSPECIFIED" => {
                    TextToColumnsRequestDelimiterType::DelimiterTypeUnspecified
                }
                "PERIOD" => TextToColumnsRequestDelimiterType::Period,
                "SEMICOLON" => TextToColumnsRequestDelimiterType::Semicolon,
                "SPACE" => TextToColumnsRequestDelimiterType::Space,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for TextToColumnsRequestDelimiterType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TextToColumnsRequestDelimiterType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ThemeColorPair {
        #[doc = "The concrete color corresponding to the theme color type."]
        #[serde(
            rename = "color",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub color: ::std::option::Option<crate::schemas::ColorStyle>,
        #[doc = "The type of the spreadsheet theme color."]
        #[serde(
            rename = "colorType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub color_type: ::std::option::Option<crate::schemas::ThemeColorPairColorType>,
    }
    impl ::google_field_selector::FieldSelector for ThemeColorPair {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ThemeColorPair {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ThemeColorPairColorType {
        #[doc = "Represents the first accent color"]
        Accent1,
        #[doc = "Represents the second accent color"]
        Accent2,
        #[doc = "Represents the third accent color"]
        Accent3,
        #[doc = "Represents the fourth accent color"]
        Accent4,
        #[doc = "Represents the fifth accent color"]
        Accent5,
        #[doc = "Represents the sixth accent color"]
        Accent6,
        #[doc = "Represents the primary background color"]
        Background,
        #[doc = "Represents the color to use for hyperlinks"]
        Link,
        #[doc = "Represents the primary text color"]
        Text,
        #[doc = "Unspecified theme color"]
        ThemeColorTypeUnspecified,
    }
    impl ThemeColorPairColorType {
        pub fn as_str(self) -> &'static str {
            match self {
                ThemeColorPairColorType::Accent1 => "ACCENT1",
                ThemeColorPairColorType::Accent2 => "ACCENT2",
                ThemeColorPairColorType::Accent3 => "ACCENT3",
                ThemeColorPairColorType::Accent4 => "ACCENT4",
                ThemeColorPairColorType::Accent5 => "ACCENT5",
                ThemeColorPairColorType::Accent6 => "ACCENT6",
                ThemeColorPairColorType::Background => "BACKGROUND",
                ThemeColorPairColorType::Link => "LINK",
                ThemeColorPairColorType::Text => "TEXT",
                ThemeColorPairColorType::ThemeColorTypeUnspecified => {
                    "THEME_COLOR_TYPE_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for ThemeColorPairColorType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ThemeColorPairColorType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ThemeColorPairColorType, ()> {
            Ok(match s {
                "ACCENT1" => ThemeColorPairColorType::Accent1,
                "ACCENT2" => ThemeColorPairColorType::Accent2,
                "ACCENT3" => ThemeColorPairColorType::Accent3,
                "ACCENT4" => ThemeColorPairColorType::Accent4,
                "ACCENT5" => ThemeColorPairColorType::Accent5,
                "ACCENT6" => ThemeColorPairColorType::Accent6,
                "BACKGROUND" => ThemeColorPairColorType::Background,
                "LINK" => ThemeColorPairColorType::Link,
                "TEXT" => ThemeColorPairColorType::Text,
                "THEME_COLOR_TYPE_UNSPECIFIED" => {
                    ThemeColorPairColorType::ThemeColorTypeUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ThemeColorPairColorType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ThemeColorPairColorType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ThemeColorPairColorType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ACCENT1" => ThemeColorPairColorType::Accent1,
                "ACCENT2" => ThemeColorPairColorType::Accent2,
                "ACCENT3" => ThemeColorPairColorType::Accent3,
                "ACCENT4" => ThemeColorPairColorType::Accent4,
                "ACCENT5" => ThemeColorPairColorType::Accent5,
                "ACCENT6" => ThemeColorPairColorType::Accent6,
                "BACKGROUND" => ThemeColorPairColorType::Background,
                "LINK" => ThemeColorPairColorType::Link,
                "TEXT" => ThemeColorPairColorType::Text,
                "THEME_COLOR_TYPE_UNSPECIFIED" => {
                    ThemeColorPairColorType::ThemeColorTypeUnspecified
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
    impl ::google_field_selector::FieldSelector for ThemeColorPairColorType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ThemeColorPairColorType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct TreemapChartColorScale {
        #[doc = "The background color for cells with a color value greater than or equal\nto maxValue. Defaults to #109618 if not\nspecified."]
        #[serde(
            rename = "maxValueColor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub max_value_color: ::std::option::Option<crate::schemas::Color>,
        #[doc = "The background color for cells with a color value greater than or equal\nto maxValue. Defaults to #109618 if not\nspecified.\nIf max_value_color is also set, this field takes precedence."]
        #[serde(
            rename = "maxValueColorStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub max_value_color_style: ::std::option::Option<crate::schemas::ColorStyle>,
        #[doc = "The background color for cells with a color value at the midpoint between\nminValue and\nmaxValue. Defaults to #efe6dc if not\nspecified."]
        #[serde(
            rename = "midValueColor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mid_value_color: ::std::option::Option<crate::schemas::Color>,
        #[doc = "The background color for cells with a color value at the midpoint between\nminValue and\nmaxValue. Defaults to #efe6dc if not\nspecified.\nIf mid_value_color is also set, this field takes precedence."]
        #[serde(
            rename = "midValueColorStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mid_value_color_style: ::std::option::Option<crate::schemas::ColorStyle>,
        #[doc = "The background color for cells with a color value less than or equal to\nminValue. Defaults to #dc3912 if not\nspecified."]
        #[serde(
            rename = "minValueColor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub min_value_color: ::std::option::Option<crate::schemas::Color>,
        #[doc = "The background color for cells with a color value less than or equal to\nminValue. Defaults to #dc3912 if not\nspecified.\nIf min_value_color is also set, this field takes precedence."]
        #[serde(
            rename = "minValueColorStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub min_value_color_style: ::std::option::Option<crate::schemas::ColorStyle>,
        #[doc = "The background color for cells that have no color data associated with\nthem. Defaults to #000000 if not specified."]
        #[serde(
            rename = "noDataColor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub no_data_color: ::std::option::Option<crate::schemas::Color>,
        #[doc = "The background color for cells that have no color data associated with\nthem. Defaults to #000000 if not specified.\nIf no_data_color is also set, this field takes precedence."]
        #[serde(
            rename = "noDataColorStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub no_data_color_style: ::std::option::Option<crate::schemas::ColorStyle>,
    }
    impl ::google_field_selector::FieldSelector for TreemapChartColorScale {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TreemapChartColorScale {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct TreemapChartSpec {
        #[doc = "The data that determines the background color of each treemap data cell.\nThis field is optional. If not specified, size_data is used to\ndetermine background colors. If specified, the data is expected to be\nnumeric. color_scale will determine how the values in this data map to\ndata cell background colors."]
        #[serde(
            rename = "colorData",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub color_data: ::std::option::Option<crate::schemas::ChartData>,
        #[doc = "The color scale for data cells in the treemap chart. Data cells are\nassigned colors based on their color values. These color values come from\ncolor_data, or from size_data if color_data is not specified.\nCells with color values less than or equal to min_value will\nhave minValueColor as their\nbackground color. Cells with color values greater than or equal to\nmax_value will have\nmaxValueColor as their background\ncolor. Cells with color values between min_value and max_value will\nhave background colors on a gradient between\nminValueColor and\nmaxValueColor, the midpoint of\nthe gradient being midValueColor.\nCells with missing or non-numeric color values will have\nnoDataColor as their background\ncolor."]
        #[serde(
            rename = "colorScale",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub color_scale: ::std::option::Option<crate::schemas::TreemapChartColorScale>,
        #[doc = "The background color for header cells."]
        #[serde(
            rename = "headerColor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub header_color: ::std::option::Option<crate::schemas::Color>,
        #[doc = "The background color for header cells.\nIf header_color is also set, this field takes precedence."]
        #[serde(
            rename = "headerColorStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub header_color_style: ::std::option::Option<crate::schemas::ColorStyle>,
        #[doc = "True to hide tooltips."]
        #[serde(
            rename = "hideTooltips",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub hide_tooltips: ::std::option::Option<bool>,
        #[doc = "The number of additional data levels beyond the labeled levels to be shown\non the treemap chart. These levels are not interactive and are shown\nwithout their labels. Defaults to 0 if not specified."]
        #[serde(
            rename = "hintedLevels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub hinted_levels: ::std::option::Option<i32>,
        #[doc = "The data that contains the treemap cell labels."]
        #[serde(
            rename = "labels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub labels: ::std::option::Option<crate::schemas::ChartData>,
        #[doc = "The number of data levels to show on the treemap chart. These levels are\ninteractive and are shown with their labels. Defaults to 2 if not\nspecified."]
        #[serde(
            rename = "levels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub levels: ::std::option::Option<i32>,
        #[doc = "The maximum possible data value. Cells with values greater than this will\nhave the same color as cells with this value. If not specified, defaults\nto the actual maximum value from color_data, or the maximum value from\nsize_data if color_data is not specified."]
        #[serde(
            rename = "maxValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub max_value: ::std::option::Option<f64>,
        #[doc = "The minimum possible data value. Cells with values less than this will\nhave the same color as cells with this value. If not specified, defaults\nto the actual minimum value from color_data, or the minimum value from\nsize_data if color_data is not specified."]
        #[serde(
            rename = "minValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub min_value: ::std::option::Option<f64>,
        #[doc = "The data the contains the treemap cells' parent labels."]
        #[serde(
            rename = "parentLabels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub parent_labels: ::std::option::Option<crate::schemas::ChartData>,
        #[doc = "The data that determines the size of each treemap data cell. This data is\nexpected to be numeric. The cells corresponding to non-numeric or missing\ndata will not be rendered. If color_data is not specified, this data\nis used to determine data cell background colors as well."]
        #[serde(
            rename = "sizeData",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub size_data: ::std::option::Option<crate::schemas::ChartData>,
        #[doc = "The text format for all labels on the chart."]
        #[serde(
            rename = "textFormat",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text_format: ::std::option::Option<crate::schemas::TextFormat>,
    }
    impl ::google_field_selector::FieldSelector for TreemapChartSpec {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TreemapChartSpec {
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
    pub struct TrimWhitespaceRequest {
        #[doc = "The range whose cells to trim."]
        #[serde(
            rename = "range",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub range: ::std::option::Option<crate::schemas::GridRange>,
    }
    impl ::google_field_selector::FieldSelector for TrimWhitespaceRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TrimWhitespaceRequest {
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
    pub struct TrimWhitespaceResponse {
        #[doc = "The number of cells that were trimmed of whitespace."]
        #[serde(
            rename = "cellsChangedCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cells_changed_count: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for TrimWhitespaceResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TrimWhitespaceResponse {
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
    pub struct UnmergeCellsRequest {
        #[doc = "The range within which all cells should be unmerged.\nIf the range spans multiple merges, all will be unmerged.\nThe range must not partially span any merge."]
        #[serde(
            rename = "range",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub range: ::std::option::Option<crate::schemas::GridRange>,
    }
    impl ::google_field_selector::FieldSelector for UnmergeCellsRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UnmergeCellsRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct UpdateBandingRequest {
        #[doc = "The banded range to update with the new properties."]
        #[serde(
            rename = "bandedRange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub banded_range: ::std::option::Option<crate::schemas::BandedRange>,
        #[doc = "The fields that should be updated.  At least one field must be specified.\nThe root `bandedRange` is implied and should not be specified.\nA single `\"*\"` can be used as short-hand for listing every field."]
        #[serde(
            rename = "fields",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fields: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for UpdateBandingRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UpdateBandingRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct UpdateBordersRequest {
        #[doc = "The border to put at the bottom of the range."]
        #[serde(
            rename = "bottom",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bottom: ::std::option::Option<crate::schemas::Border>,
        #[doc = "The horizontal border to put within the range."]
        #[serde(
            rename = "innerHorizontal",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub inner_horizontal: ::std::option::Option<crate::schemas::Border>,
        #[doc = "The vertical border to put within the range."]
        #[serde(
            rename = "innerVertical",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub inner_vertical: ::std::option::Option<crate::schemas::Border>,
        #[doc = "The border to put at the left of the range."]
        #[serde(
            rename = "left",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub left: ::std::option::Option<crate::schemas::Border>,
        #[doc = "The range whose borders should be updated."]
        #[serde(
            rename = "range",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub range: ::std::option::Option<crate::schemas::GridRange>,
        #[doc = "The border to put at the right of the range."]
        #[serde(
            rename = "right",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub right: ::std::option::Option<crate::schemas::Border>,
        #[doc = "The border to put at the top of the range."]
        #[serde(
            rename = "top",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub top: ::std::option::Option<crate::schemas::Border>,
    }
    impl ::google_field_selector::FieldSelector for UpdateBordersRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UpdateBordersRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct UpdateCellsRequest {
        #[doc = "The fields of CellData that should be updated.\nAt least one field must be specified.\nThe root is the CellData; 'row.values.' should not be specified.\nA single `\"*\"` can be used as short-hand for listing every field."]
        #[serde(
            rename = "fields",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fields: ::std::option::Option<String>,
        #[doc = "The range to write data to.\n\nIf the data in rows does not cover the entire requested range,\nthe fields matching those set in fields will be cleared."]
        #[serde(
            rename = "range",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub range: ::std::option::Option<crate::schemas::GridRange>,
        #[doc = "The data to write."]
        #[serde(
            rename = "rows",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rows: ::std::option::Option<Vec<crate::schemas::RowData>>,
        #[doc = "The coordinate to start writing data at.\nAny number of rows and columns (including a different number of\ncolumns per row) may be written."]
        #[serde(
            rename = "start",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start: ::std::option::Option<crate::schemas::GridCoordinate>,
    }
    impl ::google_field_selector::FieldSelector for UpdateCellsRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UpdateCellsRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct UpdateChartSpecRequest {
        #[doc = "The ID of the chart to update."]
        #[serde(
            rename = "chartId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub chart_id: ::std::option::Option<i32>,
        #[doc = "The specification to apply to the chart."]
        #[serde(
            rename = "spec",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub spec: ::std::option::Option<crate::schemas::ChartSpec>,
    }
    impl ::google_field_selector::FieldSelector for UpdateChartSpecRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UpdateChartSpecRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct UpdateConditionalFormatRuleRequest {
        #[doc = "The zero-based index of the rule that should be replaced or moved."]
        #[serde(
            rename = "index",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub index: ::std::option::Option<i32>,
        #[doc = "The zero-based new index the rule should end up at."]
        #[serde(
            rename = "newIndex",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub new_index: ::std::option::Option<i32>,
        #[doc = "The rule that should replace the rule at the given index."]
        #[serde(
            rename = "rule",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rule: ::std::option::Option<crate::schemas::ConditionalFormatRule>,
        #[doc = "The sheet of the rule to move.  Required if new_index is set,\nunused otherwise."]
        #[serde(
            rename = "sheetId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sheet_id: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for UpdateConditionalFormatRuleRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UpdateConditionalFormatRuleRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct UpdateConditionalFormatRuleResponse {
        #[doc = "The index of the new rule."]
        #[serde(
            rename = "newIndex",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub new_index: ::std::option::Option<i32>,
        #[doc = "The new rule that replaced the old rule (if replacing),\nor the rule that was moved (if moved)"]
        #[serde(
            rename = "newRule",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub new_rule: ::std::option::Option<crate::schemas::ConditionalFormatRule>,
        #[doc = "The old index of the rule. Not set if a rule was replaced\n(because it is the same as new_index)."]
        #[serde(
            rename = "oldIndex",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub old_index: ::std::option::Option<i32>,
        #[doc = "The old (deleted) rule. Not set if a rule was moved\n(because it is the same as new_rule)."]
        #[serde(
            rename = "oldRule",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub old_rule: ::std::option::Option<crate::schemas::ConditionalFormatRule>,
    }
    impl ::google_field_selector::FieldSelector for UpdateConditionalFormatRuleResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UpdateConditionalFormatRuleResponse {
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
    pub struct UpdateDeveloperMetadataRequest {
        #[doc = "The filters matching the developer metadata entries to update."]
        #[serde(
            rename = "dataFilters",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub data_filters: ::std::option::Option<Vec<crate::schemas::DataFilter>>,
        #[doc = "The value that all metadata matched by the data filters will be updated to."]
        #[serde(
            rename = "developerMetadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub developer_metadata: ::std::option::Option<crate::schemas::DeveloperMetadata>,
        #[doc = "The fields that should be updated.  At least one field must be specified.\nThe root `developerMetadata` is implied and should not be specified.\nA single `\"*\"` can be used as short-hand for listing every field."]
        #[serde(
            rename = "fields",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fields: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for UpdateDeveloperMetadataRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UpdateDeveloperMetadataRequest {
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
    pub struct UpdateDeveloperMetadataResponse {
        #[doc = "The updated developer metadata."]
        #[serde(
            rename = "developerMetadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub developer_metadata: ::std::option::Option<Vec<crate::schemas::DeveloperMetadata>>,
    }
    impl ::google_field_selector::FieldSelector for UpdateDeveloperMetadataResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UpdateDeveloperMetadataResponse {
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
    pub struct UpdateDimensionGroupRequest {
        #[doc = "The group whose state should be updated. The range and depth of the group\nshould specify a valid group on the sheet, and all other fields updated."]
        #[serde(
            rename = "dimensionGroup",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dimension_group: ::std::option::Option<crate::schemas::DimensionGroup>,
        #[doc = "The fields that should be updated.  At least one field must be specified.\nThe root `dimensionGroup` is implied and should not be specified.\nA single `\"*\"` can be used as short-hand for listing every field."]
        #[serde(
            rename = "fields",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fields: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for UpdateDimensionGroupRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UpdateDimensionGroupRequest {
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
    pub struct UpdateDimensionPropertiesRequest {
        #[doc = "The fields that should be updated.  At least one field must be specified.\nThe root `properties` is implied and should not be specified.\nA single `\"*\"` can be used as short-hand for listing every field."]
        #[serde(
            rename = "fields",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fields: ::std::option::Option<String>,
        #[doc = "Properties to update."]
        #[serde(
            rename = "properties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub properties: ::std::option::Option<crate::schemas::DimensionProperties>,
        #[doc = "The rows or columns to update."]
        #[serde(
            rename = "range",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub range: ::std::option::Option<crate::schemas::DimensionRange>,
    }
    impl ::google_field_selector::FieldSelector for UpdateDimensionPropertiesRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UpdateDimensionPropertiesRequest {
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
    pub struct UpdateEmbeddedObjectPositionRequest {
        #[doc = "The fields of OverlayPosition\nthat should be updated when setting a new position. Used only if\nnewPosition.overlayPosition\nis set, in which case at least one field must\nbe specified.  The root `newPosition.overlayPosition` is implied and\nshould not be specified.\nA single `\"*\"` can be used as short-hand for listing every field."]
        #[serde(
            rename = "fields",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fields: ::std::option::Option<String>,
        #[doc = "An explicit position to move the embedded object to.\nIf newPosition.sheetId is set,\na new sheet with that ID will be created.\nIf newPosition.newSheet is set to true,\na new sheet will be created with an ID that will be chosen for you."]
        #[serde(
            rename = "newPosition",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub new_position: ::std::option::Option<crate::schemas::EmbeddedObjectPosition>,
        #[doc = "The ID of the object to moved."]
        #[serde(
            rename = "objectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub object_id: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for UpdateEmbeddedObjectPositionRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UpdateEmbeddedObjectPositionRequest {
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
    pub struct UpdateEmbeddedObjectPositionResponse {
        #[doc = "The new position of the embedded object."]
        #[serde(
            rename = "position",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub position: ::std::option::Option<crate::schemas::EmbeddedObjectPosition>,
    }
    impl ::google_field_selector::FieldSelector for UpdateEmbeddedObjectPositionResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UpdateEmbeddedObjectPositionResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct UpdateFilterViewRequest {
        #[doc = "The fields that should be updated.  At least one field must be specified.\nThe root `filter` is implied and should not be specified.\nA single `\"*\"` can be used as short-hand for listing every field."]
        #[serde(
            rename = "fields",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fields: ::std::option::Option<String>,
        #[doc = "The new properties of the filter view."]
        #[serde(
            rename = "filter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub filter: ::std::option::Option<crate::schemas::FilterView>,
    }
    impl ::google_field_selector::FieldSelector for UpdateFilterViewRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UpdateFilterViewRequest {
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
    pub struct UpdateNamedRangeRequest {
        #[doc = "The fields that should be updated.  At least one field must be specified.\nThe root `namedRange` is implied and should not be specified.\nA single `\"*\"` can be used as short-hand for listing every field."]
        #[serde(
            rename = "fields",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fields: ::std::option::Option<String>,
        #[doc = "The named range to update with the new properties."]
        #[serde(
            rename = "namedRange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub named_range: ::std::option::Option<crate::schemas::NamedRange>,
    }
    impl ::google_field_selector::FieldSelector for UpdateNamedRangeRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UpdateNamedRangeRequest {
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
    pub struct UpdateProtectedRangeRequest {
        #[doc = "The fields that should be updated.  At least one field must be specified.\nThe root `protectedRange` is implied and should not be specified.\nA single `\"*\"` can be used as short-hand for listing every field."]
        #[serde(
            rename = "fields",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fields: ::std::option::Option<String>,
        #[doc = "The protected range to update with the new properties."]
        #[serde(
            rename = "protectedRange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub protected_range: ::std::option::Option<crate::schemas::ProtectedRange>,
    }
    impl ::google_field_selector::FieldSelector for UpdateProtectedRangeRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UpdateProtectedRangeRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct UpdateSheetPropertiesRequest {
        #[doc = "The fields that should be updated.  At least one field must be specified.\nThe root `properties` is implied and should not be specified.\nA single `\"*\"` can be used as short-hand for listing every field."]
        #[serde(
            rename = "fields",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fields: ::std::option::Option<String>,
        #[doc = "The properties to update."]
        #[serde(
            rename = "properties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub properties: ::std::option::Option<crate::schemas::SheetProperties>,
    }
    impl ::google_field_selector::FieldSelector for UpdateSheetPropertiesRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UpdateSheetPropertiesRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct UpdateSlicerSpecRequest {
        #[doc = "The fields that should be updated.  At least one field must be specified.\nThe root `SlicerSpec` is implied and should not be specified. A single \"*\"`\ncan be used as short-hand for listing every field."]
        #[serde(
            rename = "fields",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fields: ::std::option::Option<String>,
        #[doc = "The id of the slicer to update."]
        #[serde(
            rename = "slicerId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub slicer_id: ::std::option::Option<i32>,
        #[doc = "The specification to apply to the slicer."]
        #[serde(
            rename = "spec",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub spec: ::std::option::Option<crate::schemas::SlicerSpec>,
    }
    impl ::google_field_selector::FieldSelector for UpdateSlicerSpecRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UpdateSlicerSpecRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct UpdateSpreadsheetPropertiesRequest {
        #[doc = "The fields that should be updated.  At least one field must be specified.\nThe root 'properties' is implied and should not be specified.\nA single `\"*\"` can be used as short-hand for listing every field."]
        #[serde(
            rename = "fields",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fields: ::std::option::Option<String>,
        #[doc = "The properties to update."]
        #[serde(
            rename = "properties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub properties: ::std::option::Option<crate::schemas::SpreadsheetProperties>,
    }
    impl ::google_field_selector::FieldSelector for UpdateSpreadsheetPropertiesRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UpdateSpreadsheetPropertiesRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct UpdateValuesByDataFilterResponse {
        #[doc = "The data filter that selected the range that was updated."]
        #[serde(
            rename = "dataFilter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub data_filter: ::std::option::Option<crate::schemas::DataFilter>,
        #[doc = "The number of cells updated."]
        #[serde(
            rename = "updatedCells",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub updated_cells: ::std::option::Option<i32>,
        #[doc = "The number of columns where at least one cell in the column was updated."]
        #[serde(
            rename = "updatedColumns",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub updated_columns: ::std::option::Option<i32>,
        #[doc = "The values of the cells in the range matched by the dataFilter after all\nupdates were applied. This is only included if the request's\n`includeValuesInResponse` field was `true`."]
        #[serde(
            rename = "updatedData",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub updated_data: ::std::option::Option<crate::schemas::ValueRange>,
        #[doc = "The range (in A1 notation) that updates were applied to."]
        #[serde(
            rename = "updatedRange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub updated_range: ::std::option::Option<String>,
        #[doc = "The number of rows where at least one cell in the row was updated."]
        #[serde(
            rename = "updatedRows",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub updated_rows: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for UpdateValuesByDataFilterResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UpdateValuesByDataFilterResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct UpdateValuesResponse {
        #[doc = "The spreadsheet the updates were applied to."]
        #[serde(
            rename = "spreadsheetId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub spreadsheet_id: ::std::option::Option<String>,
        #[doc = "The number of cells updated."]
        #[serde(
            rename = "updatedCells",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub updated_cells: ::std::option::Option<i32>,
        #[doc = "The number of columns where at least one cell in the column was updated."]
        #[serde(
            rename = "updatedColumns",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub updated_columns: ::std::option::Option<i32>,
        #[doc = "The values of the cells after updates were applied.\nThis is only included if the request's `includeValuesInResponse` field\nwas `true`."]
        #[serde(
            rename = "updatedData",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub updated_data: ::std::option::Option<crate::schemas::ValueRange>,
        #[doc = "The range (in A1 notation) that updates were applied to."]
        #[serde(
            rename = "updatedRange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub updated_range: ::std::option::Option<String>,
        #[doc = "The number of rows where at least one cell in the row was updated."]
        #[serde(
            rename = "updatedRows",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub updated_rows: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for UpdateValuesResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UpdateValuesResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct ValueRange {
        #[doc = "The major dimension of the values.\n\nFor output, if the spreadsheet data is: `A1=1,B1=2,A2=3,B2=4`,\nthen requesting `range=A1:B2,majorDimension=ROWS` will return\n`[[1,2],[3,4]]`,\nwhereas requesting `range=A1:B2,majorDimension=COLUMNS` will return\n`[[1,3],[2,4]]`.\n\nFor input, with `range=A1:B2,majorDimension=ROWS` then `[[1,2],[3,4]]`\nwill set `A1=1,B1=2,A2=3,B2=4`. With `range=A1:B2,majorDimension=COLUMNS`\nthen `[[1,2],[3,4]]` will set `A1=1,B1=3,A2=2,B2=4`.\n\nWhen writing, if this field is not set, it defaults to ROWS."]
        #[serde(
            rename = "majorDimension",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub major_dimension: ::std::option::Option<crate::schemas::ValueRangeMajorDimension>,
        #[doc = "The range the values cover, in A1 notation.\nFor output, this range indicates the entire requested range,\neven though the values will exclude trailing rows and columns.\nWhen appending values, this field represents the range to search for a\ntable, after which values will be appended."]
        #[serde(
            rename = "range",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub range: ::std::option::Option<String>,
        #[doc = "The data that was read or to be written.  This is an array of arrays,\nthe outer array representing all the data and each inner array\nrepresenting a major dimension. Each item in the inner array\ncorresponds with one cell.\n\nFor output, empty trailing rows and columns will not be included.\n\nFor input, supported value types are: bool, string, and double.\nNull values will be skipped.\nTo set a cell to an empty value, set the string value to an empty string."]
        #[serde(
            rename = "values",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub values: ::std::option::Option<Vec<Vec<::serde_json::Value>>>,
    }
    impl ::google_field_selector::FieldSelector for ValueRange {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ValueRange {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ValueRangeMajorDimension {
        #[doc = "Operates on the columns of a sheet."]
        Columns,
        #[doc = "The default value, do not use."]
        DimensionUnspecified,
        #[doc = "Operates on the rows of a sheet."]
        Rows,
    }
    impl ValueRangeMajorDimension {
        pub fn as_str(self) -> &'static str {
            match self {
                ValueRangeMajorDimension::Columns => "COLUMNS",
                ValueRangeMajorDimension::DimensionUnspecified => "DIMENSION_UNSPECIFIED",
                ValueRangeMajorDimension::Rows => "ROWS",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ValueRangeMajorDimension {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ValueRangeMajorDimension {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ValueRangeMajorDimension, ()> {
            Ok(match s {
                "COLUMNS" => ValueRangeMajorDimension::Columns,
                "DIMENSION_UNSPECIFIED" => ValueRangeMajorDimension::DimensionUnspecified,
                "ROWS" => ValueRangeMajorDimension::Rows,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ValueRangeMajorDimension {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ValueRangeMajorDimension {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ValueRangeMajorDimension {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "COLUMNS" => ValueRangeMajorDimension::Columns,
                "DIMENSION_UNSPECIFIED" => ValueRangeMajorDimension::DimensionUnspecified,
                "ROWS" => ValueRangeMajorDimension::Rows,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ValueRangeMajorDimension {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ValueRangeMajorDimension {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct WaterfallChartColumnStyle {
        #[doc = "The color of the column."]
        #[serde(
            rename = "color",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub color: ::std::option::Option<crate::schemas::Color>,
        #[doc = "The color of the column.\nIf color is also set, this field takes precedence."]
        #[serde(
            rename = "colorStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub color_style: ::std::option::Option<crate::schemas::ColorStyle>,
        #[doc = "The label of the column's legend."]
        #[serde(
            rename = "label",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub label: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for WaterfallChartColumnStyle {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for WaterfallChartColumnStyle {
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
    pub struct WaterfallChartCustomSubtotal {
        #[doc = "True if the data point at subtotal_index is the subtotal. If false,\nthe subtotal will be computed and appear after the data point."]
        #[serde(
            rename = "dataIsSubtotal",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub data_is_subtotal: ::std::option::Option<bool>,
        #[doc = "A label for the subtotal column."]
        #[serde(
            rename = "label",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub label: ::std::option::Option<String>,
        #[doc = "The 0-based index of a data point within the series. If\ndata_is_subtotal is true, the data point at this index is the\nsubtotal. Otherwise, the subtotal appears after the data point with\nthis index. A series can have multiple subtotals at arbitrary indices,\nbut subtotals do not affect the indices of the data points. For\nexample, if a series has three data points, their indices will always\nbe 0, 1, and 2, regardless of how many subtotals exist on the series or\nwhat data points they are associated with."]
        #[serde(
            rename = "subtotalIndex",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub subtotal_index: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for WaterfallChartCustomSubtotal {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for WaterfallChartCustomSubtotal {
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
    pub struct WaterfallChartDomain {
        #[doc = "The data of the WaterfallChartDomain."]
        #[serde(
            rename = "data",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub data: ::std::option::Option<crate::schemas::ChartData>,
        #[doc = "True to reverse the order of the domain values (horizontal axis)."]
        #[serde(
            rename = "reversed",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub reversed: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for WaterfallChartDomain {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for WaterfallChartDomain {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct WaterfallChartSeries {
        #[doc = "Custom subtotal columns appearing in this series. The order in which\nsubtotals are defined is not significant. Only one subtotal may be\ndefined for each data point."]
        #[serde(
            rename = "customSubtotals",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub custom_subtotals:
            ::std::option::Option<Vec<crate::schemas::WaterfallChartCustomSubtotal>>,
        #[doc = "The data being visualized in this series."]
        #[serde(
            rename = "data",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub data: ::std::option::Option<crate::schemas::ChartData>,
        #[doc = "True to hide the subtotal column from the end of the series. By default,\na subtotal column will appear at the end of each series. Setting this\nfield to true will hide that subtotal column for this series."]
        #[serde(
            rename = "hideTrailingSubtotal",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub hide_trailing_subtotal: ::std::option::Option<bool>,
        #[doc = "Styles for all columns in this series with negative values."]
        #[serde(
            rename = "negativeColumnsStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub negative_columns_style:
            ::std::option::Option<crate::schemas::WaterfallChartColumnStyle>,
        #[doc = "Styles for all columns in this series with positive values."]
        #[serde(
            rename = "positiveColumnsStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub positive_columns_style:
            ::std::option::Option<crate::schemas::WaterfallChartColumnStyle>,
        #[doc = "Styles for all subtotal columns in this series."]
        #[serde(
            rename = "subtotalColumnsStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub subtotal_columns_style:
            ::std::option::Option<crate::schemas::WaterfallChartColumnStyle>,
    }
    impl ::google_field_selector::FieldSelector for WaterfallChartSeries {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for WaterfallChartSeries {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct WaterfallChartSpec {
        #[doc = "The line style for the connector lines."]
        #[serde(
            rename = "connectorLineStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub connector_line_style: ::std::option::Option<crate::schemas::LineStyle>,
        #[doc = "The domain data (horizontal axis) for the waterfall chart."]
        #[serde(
            rename = "domain",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub domain: ::std::option::Option<crate::schemas::WaterfallChartDomain>,
        #[doc = "True to interpret the first value as a total."]
        #[serde(
            rename = "firstValueIsTotal",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub first_value_is_total: ::std::option::Option<bool>,
        #[doc = "True to hide connector lines between columns."]
        #[serde(
            rename = "hideConnectorLines",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub hide_connector_lines: ::std::option::Option<bool>,
        #[doc = "The data this waterfall chart is visualizing."]
        #[serde(
            rename = "series",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub series: ::std::option::Option<Vec<crate::schemas::WaterfallChartSeries>>,
        #[doc = "The stacked type."]
        #[serde(
            rename = "stackedType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub stacked_type: ::std::option::Option<crate::schemas::WaterfallChartSpecStackedType>,
    }
    impl ::google_field_selector::FieldSelector for WaterfallChartSpec {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for WaterfallChartSpec {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum WaterfallChartSpecStackedType {
        #[doc = "Series will spread out along the horizontal axis."]
        Sequential,
        #[doc = "Values corresponding to the same domain (horizontal axis) value will be\nstacked vertically."]
        Stacked,
        #[doc = "Default value, do not use."]
        WaterfallStackedTypeUnspecified,
    }
    impl WaterfallChartSpecStackedType {
        pub fn as_str(self) -> &'static str {
            match self {
                WaterfallChartSpecStackedType::Sequential => "SEQUENTIAL",
                WaterfallChartSpecStackedType::Stacked => "STACKED",
                WaterfallChartSpecStackedType::WaterfallStackedTypeUnspecified => {
                    "WATERFALL_STACKED_TYPE_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for WaterfallChartSpecStackedType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for WaterfallChartSpecStackedType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<WaterfallChartSpecStackedType, ()> {
            Ok(match s {
                "SEQUENTIAL" => WaterfallChartSpecStackedType::Sequential,
                "STACKED" => WaterfallChartSpecStackedType::Stacked,
                "WATERFALL_STACKED_TYPE_UNSPECIFIED" => {
                    WaterfallChartSpecStackedType::WaterfallStackedTypeUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for WaterfallChartSpecStackedType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for WaterfallChartSpecStackedType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for WaterfallChartSpecStackedType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "SEQUENTIAL" => WaterfallChartSpecStackedType::Sequential,
                "STACKED" => WaterfallChartSpecStackedType::Stacked,
                "WATERFALL_STACKED_TYPE_UNSPECIFIED" => {
                    WaterfallChartSpecStackedType::WaterfallStackedTypeUnspecified
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
    impl ::google_field_selector::FieldSelector for WaterfallChartSpecStackedType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for WaterfallChartSpecStackedType {
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
    #[doc = "Actions that can be performed on the spreadsheets resource"]
    pub fn spreadsheets(&self) -> crate::resources::spreadsheets::SpreadsheetsActions {
        crate::resources::spreadsheets::SpreadsheetsActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
}
pub mod resources {
    pub mod spreadsheets {
        pub mod params {}
        pub struct SpreadsheetsActions<'a> {
            pub(crate) reqwest: &'a reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> SpreadsheetsActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Applies one or more updates to the spreadsheet.\n\nEach request is validated before\nbeing applied. If any request is not valid then the entire request will\nfail and nothing will be applied.\n\nSome requests have replies to\ngive you some information about how\nthey are applied. The replies will mirror the requests.  For example,\nif you applied 4 updates and the 3rd one had a reply, then the\nresponse will have 2 empty replies, the actual reply, and another empty\nreply, in that order.\n\nDue to the collaborative nature of spreadsheets, it is not guaranteed that\nthe spreadsheet will reflect exactly your changes after this completes,\nhowever it is guaranteed that the updates in the request will be\napplied together atomically. Your changes may be altered with respect to\ncollaborator changes. If there are no collaborators, the spreadsheet\nshould reflect your changes."]
            pub fn batch_update(
                &self,
                request: crate::schemas::BatchUpdateSpreadsheetRequest,
                spreadsheet_id: impl Into<String>,
            ) -> BatchUpdateRequestBuilder {
                BatchUpdateRequestBuilder {
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
                    spreadsheet_id: spreadsheet_id.into(),
                }
            }
            #[doc = "Creates a spreadsheet, returning the newly created spreadsheet."]
            pub fn create(&self, request: crate::schemas::Spreadsheet) -> CreateRequestBuilder {
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
                }
            }
            #[doc = "Returns the spreadsheet at the given ID.\nThe caller must specify the spreadsheet ID.\n\nBy default, data within grids will not be returned.\nYou can include grid data one of two ways:\n\n* Specify a field mask listing your desired fields using the `fields` URL\n  parameter in HTTP\n\n* Set the includeGridData\n  URL parameter to true.  If a field mask is set, the `includeGridData`\n  parameter is ignored\n\nFor large spreadsheets, it is recommended to retrieve only the specific\nfields of the spreadsheet that you want.\n\nTo retrieve only subsets of the spreadsheet, use the\nranges URL parameter.\nMultiple ranges can be specified.  Limiting the range will\nreturn only the portions of the spreadsheet that intersect the requested\nranges. Ranges are specified using A1 notation."]
            pub fn get(&self, spreadsheet_id: impl Into<String>) -> GetRequestBuilder {
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
                    spreadsheet_id: spreadsheet_id.into(),
                    include_grid_data: None,
                    ranges: None,
                }
            }
            #[doc = "Returns the spreadsheet at the given ID.\nThe caller must specify the spreadsheet ID.\n\nThis method differs from GetSpreadsheet in that it allows selecting\nwhich subsets of spreadsheet data to return by specifying a\ndataFilters parameter.\nMultiple DataFilters can be specified.  Specifying one or\nmore data filters will return the portions of the spreadsheet that\nintersect ranges matched by any of the filters.\n\nBy default, data within grids will not be returned.\nYou can include grid data one of two ways:\n\n* Specify a field mask listing your desired fields using the `fields` URL\n  parameter in HTTP\n\n* Set the includeGridData\n  parameter to true.  If a field mask is set, the `includeGridData`\n  parameter is ignored\n\nFor large spreadsheets, it is recommended to retrieve only the specific\nfields of the spreadsheet that you want."]
            pub fn get_by_data_filter(
                &self,
                request: crate::schemas::GetSpreadsheetByDataFilterRequest,
                spreadsheet_id: impl Into<String>,
            ) -> GetByDataFilterRequestBuilder {
                GetByDataFilterRequestBuilder {
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
                    spreadsheet_id: spreadsheet_id.into(),
                }
            }
            #[doc = "Actions that can be performed on the developer_metadata resource"]
            pub fn developer_metadata(
                &self,
            ) -> crate::resources::spreadsheets::developer_metadata::DeveloperMetadataActions
            {
                crate::resources::spreadsheets::developer_metadata::DeveloperMetadataActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
            #[doc = "Actions that can be performed on the sheets resource"]
            pub fn sheets(&self) -> crate::resources::spreadsheets::sheets::SheetsActions {
                crate::resources::spreadsheets::sheets::SheetsActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
            #[doc = "Actions that can be performed on the values resource"]
            pub fn values(&self) -> crate::resources::spreadsheets::values::ValuesActions {
                crate::resources::spreadsheets::values::ValuesActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
        }
        #[doc = "Created via [SpreadsheetsActions::batch_update()](struct.SpreadsheetsActions.html#method.batch_update)"]
        #[derive(Debug, Clone)]
        pub struct BatchUpdateRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::BatchUpdateSpreadsheetRequest,
            spreadsheet_id: String,
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
        impl<'a> BatchUpdateRequestBuilder<'a> {
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
            ) -> Result<crate::schemas::BatchUpdateSpreadsheetResponse, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::BatchUpdateSpreadsheetResponse, crate::Error> {
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
                let mut output = "https://sheets.googleapis.com/".to_owned();
                output.push_str("v4/spreadsheets/");
                {
                    let var_as_str = &self.spreadsheet_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str(":batchUpdate");
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
        #[doc = "Created via [SpreadsheetsActions::create()](struct.SpreadsheetsActions.html#method.create)"]
        #[derive(Debug, Clone)]
        pub struct CreateRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::Spreadsheet,
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
            ) -> Result<crate::schemas::Spreadsheet, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Spreadsheet, crate::Error> {
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
                let mut output = "https://sheets.googleapis.com/".to_owned();
                output.push_str("v4/spreadsheets");
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
        #[doc = "Created via [SpreadsheetsActions::get()](struct.SpreadsheetsActions.html#method.get)"]
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            spreadsheet_id: String,
            include_grid_data: Option<bool>,
            ranges: Option<Vec<String>>,
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
            #[doc = "True if grid data should be returned.\nThis parameter is ignored if a field mask was set in the request."]
            pub fn include_grid_data(mut self, value: bool) -> Self {
                self.include_grid_data = Some(value);
                self
            }
            #[doc = "The ranges to retrieve from the spreadsheet."]
            pub fn ranges(mut self, value: impl Into<Vec<String>>) -> Self {
                self.ranges = Some(value.into());
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
            ) -> Result<crate::schemas::Spreadsheet, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Spreadsheet, crate::Error> {
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
                let mut output = "https://sheets.googleapis.com/".to_owned();
                output.push_str("v4/spreadsheets/");
                {
                    let var_as_str = &self.spreadsheet_id;
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
                let req = req.query(&[("includeGridData", &self.include_grid_data)]);
                let req = req.query(&[("ranges", &self.ranges)]);
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
        #[doc = "Created via [SpreadsheetsActions::get_by_data_filter()](struct.SpreadsheetsActions.html#method.get_by_data_filter)"]
        #[derive(Debug, Clone)]
        pub struct GetByDataFilterRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::GetSpreadsheetByDataFilterRequest,
            spreadsheet_id: String,
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
        impl<'a> GetByDataFilterRequestBuilder<'a> {
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
            ) -> Result<crate::schemas::Spreadsheet, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Spreadsheet, crate::Error> {
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
                let mut output = "https://sheets.googleapis.com/".to_owned();
                output.push_str("v4/spreadsheets/");
                {
                    let var_as_str = &self.spreadsheet_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str(":getByDataFilter");
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
        pub mod developer_metadata {
            pub mod params {}
            pub struct DeveloperMetadataActions<'a> {
                pub(crate) reqwest: &'a reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> DeveloperMetadataActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Returns the developer metadata with the specified ID.\nThe caller must specify the spreadsheet ID and the developer metadata's\nunique metadataId."]
                pub fn get(
                    &self,
                    spreadsheet_id: impl Into<String>,
                    metadata_id: i32,
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
                        spreadsheet_id: spreadsheet_id.into(),
                        metadata_id,
                    }
                }
                #[doc = "Returns all developer metadata matching the specified DataFilter.\nIf the provided DataFilter represents a DeveloperMetadataLookup object,\nthis will return all DeveloperMetadata entries selected by it. If the\nDataFilter represents a location in a spreadsheet, this will return all\ndeveloper metadata associated with locations intersecting that region."]
                pub fn search(
                    &self,
                    request: crate::schemas::SearchDeveloperMetadataRequest,
                    spreadsheet_id: impl Into<String>,
                ) -> SearchRequestBuilder {
                    SearchRequestBuilder {
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
                        spreadsheet_id: spreadsheet_id.into(),
                    }
                }
            }
            #[doc = "Created via [DeveloperMetadataActions::get()](struct.DeveloperMetadataActions.html#method.get)"]
            #[derive(Debug, Clone)]
            pub struct GetRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                spreadsheet_id: String,
                metadata_id: i32,
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
                ) -> Result<crate::schemas::DeveloperMetadata, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::DeveloperMetadata, crate::Error> {
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
                    let mut output = "https://sheets.googleapis.com/".to_owned();
                    output.push_str("v4/spreadsheets/");
                    {
                        let var_as_str = &self.spreadsheet_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/developerMetadata/");
                    {
                        let var_as_string = self.metadata_id.to_string();
                        let var_as_str = &var_as_string;
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
            #[doc = "Created via [DeveloperMetadataActions::search()](struct.DeveloperMetadataActions.html#method.search)"]
            #[derive(Debug, Clone)]
            pub struct SearchRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::SearchDeveloperMetadataRequest,
                spreadsheet_id: String,
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
            impl<'a> SearchRequestBuilder<'a> {
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
                ) -> Result<crate::schemas::SearchDeveloperMetadataResponse, crate::Error>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::SearchDeveloperMetadataResponse, crate::Error>
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
                    let mut output = "https://sheets.googleapis.com/".to_owned();
                    output.push_str("v4/spreadsheets/");
                    {
                        let var_as_str = &self.spreadsheet_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/developerMetadata:search");
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
        pub mod sheets {
            pub mod params {}
            pub struct SheetsActions<'a> {
                pub(crate) reqwest: &'a reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> SheetsActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Copies a single sheet from a spreadsheet to another spreadsheet.\nReturns the properties of the newly created sheet."]
                pub fn copy_to(
                    &self,
                    request: crate::schemas::CopySheetToAnotherSpreadsheetRequest,
                    spreadsheet_id: impl Into<String>,
                    sheet_id: i32,
                ) -> CopyToRequestBuilder {
                    CopyToRequestBuilder {
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
                        spreadsheet_id: spreadsheet_id.into(),
                        sheet_id,
                    }
                }
            }
            #[doc = "Created via [SheetsActions::copy_to()](struct.SheetsActions.html#method.copy_to)"]
            #[derive(Debug, Clone)]
            pub struct CopyToRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::CopySheetToAnotherSpreadsheetRequest,
                spreadsheet_id: String,
                sheet_id: i32,
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
            impl<'a> CopyToRequestBuilder<'a> {
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
                ) -> Result<crate::schemas::SheetProperties, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::SheetProperties, crate::Error> {
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
                    let mut output = "https://sheets.googleapis.com/".to_owned();
                    output.push_str("v4/spreadsheets/");
                    {
                        let var_as_str = &self.spreadsheet_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/sheets/");
                    {
                        let var_as_string = self.sheet_id.to_string();
                        let var_as_str = &var_as_string;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str(":copyTo");
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
        pub mod values {
            pub mod params {
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum AppendInsertDataOption {
                    InsertRows,
                    Overwrite,
                }
                impl AppendInsertDataOption {
                    pub fn as_str(self) -> &'static str {
                        match self {
                            AppendInsertDataOption::InsertRows => "INSERT_ROWS",
                            AppendInsertDataOption::Overwrite => "OVERWRITE",
                        }
                    }
                }
                impl ::std::convert::AsRef<str> for AppendInsertDataOption {
                    fn as_ref(&self) -> &str {
                        self.as_str()
                    }
                }
                impl ::std::str::FromStr for AppendInsertDataOption {
                    type Err = ();
                    fn from_str(s: &str) -> ::std::result::Result<AppendInsertDataOption, ()> {
                        Ok(match s {
                            "INSERT_ROWS" => AppendInsertDataOption::InsertRows,
                            "OVERWRITE" => AppendInsertDataOption::Overwrite,
                            _ => return Err(()),
                        })
                    }
                }
                impl ::std::fmt::Display for AppendInsertDataOption {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for AppendInsertDataOption {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for AppendInsertDataOption {
                    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                    where
                        D: ::serde::de::Deserializer<'de>,
                    {
                        let value: &'de str = <&str>::deserialize(deserializer)?;
                        Ok(match value {
                            "INSERT_ROWS" => AppendInsertDataOption::InsertRows,
                            "OVERWRITE" => AppendInsertDataOption::Overwrite,
                            _ => {
                                return Err(::serde::de::Error::custom(format!(
                                    "invalid enum for #name: {}",
                                    value
                                )))
                            }
                        })
                    }
                }
                impl ::google_field_selector::FieldSelector for AppendInsertDataOption {
                    fn fields() -> Vec<::google_field_selector::Field> {
                        Vec::new()
                    }
                }
                impl ::google_field_selector::ToFieldType for AppendInsertDataOption {
                    fn field_type() -> ::google_field_selector::FieldType {
                        ::google_field_selector::FieldType::Leaf
                    }
                }
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum AppendResponseDateTimeRenderOption {
                    FormattedString,
                    SerialNumber,
                }
                impl AppendResponseDateTimeRenderOption {
                    pub fn as_str(self) -> &'static str {
                        match self {
                            AppendResponseDateTimeRenderOption::FormattedString => {
                                "FORMATTED_STRING"
                            }
                            AppendResponseDateTimeRenderOption::SerialNumber => "SERIAL_NUMBER",
                        }
                    }
                }
                impl ::std::convert::AsRef<str> for AppendResponseDateTimeRenderOption {
                    fn as_ref(&self) -> &str {
                        self.as_str()
                    }
                }
                impl ::std::str::FromStr for AppendResponseDateTimeRenderOption {
                    type Err = ();
                    fn from_str(
                        s: &str,
                    ) -> ::std::result::Result<AppendResponseDateTimeRenderOption, ()>
                    {
                        Ok(match s {
                            "FORMATTED_STRING" => {
                                AppendResponseDateTimeRenderOption::FormattedString
                            }
                            "SERIAL_NUMBER" => AppendResponseDateTimeRenderOption::SerialNumber,
                            _ => return Err(()),
                        })
                    }
                }
                impl ::std::fmt::Display for AppendResponseDateTimeRenderOption {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for AppendResponseDateTimeRenderOption {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for AppendResponseDateTimeRenderOption {
                    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                    where
                        D: ::serde::de::Deserializer<'de>,
                    {
                        let value: &'de str = <&str>::deserialize(deserializer)?;
                        Ok(match value {
                            "FORMATTED_STRING" => {
                                AppendResponseDateTimeRenderOption::FormattedString
                            }
                            "SERIAL_NUMBER" => AppendResponseDateTimeRenderOption::SerialNumber,
                            _ => {
                                return Err(::serde::de::Error::custom(format!(
                                    "invalid enum for #name: {}",
                                    value
                                )))
                            }
                        })
                    }
                }
                impl ::google_field_selector::FieldSelector for AppendResponseDateTimeRenderOption {
                    fn fields() -> Vec<::google_field_selector::Field> {
                        Vec::new()
                    }
                }
                impl ::google_field_selector::ToFieldType for AppendResponseDateTimeRenderOption {
                    fn field_type() -> ::google_field_selector::FieldType {
                        ::google_field_selector::FieldType::Leaf
                    }
                }
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum AppendResponseValueRenderOption {
                    FormattedValue,
                    Formula,
                    UnformattedValue,
                }
                impl AppendResponseValueRenderOption {
                    pub fn as_str(self) -> &'static str {
                        match self {
                            AppendResponseValueRenderOption::FormattedValue => "FORMATTED_VALUE",
                            AppendResponseValueRenderOption::Formula => "FORMULA",
                            AppendResponseValueRenderOption::UnformattedValue => {
                                "UNFORMATTED_VALUE"
                            }
                        }
                    }
                }
                impl ::std::convert::AsRef<str> for AppendResponseValueRenderOption {
                    fn as_ref(&self) -> &str {
                        self.as_str()
                    }
                }
                impl ::std::str::FromStr for AppendResponseValueRenderOption {
                    type Err = ();
                    fn from_str(
                        s: &str,
                    ) -> ::std::result::Result<AppendResponseValueRenderOption, ()>
                    {
                        Ok(match s {
                            "FORMATTED_VALUE" => AppendResponseValueRenderOption::FormattedValue,
                            "FORMULA" => AppendResponseValueRenderOption::Formula,
                            "UNFORMATTED_VALUE" => {
                                AppendResponseValueRenderOption::UnformattedValue
                            }
                            _ => return Err(()),
                        })
                    }
                }
                impl ::std::fmt::Display for AppendResponseValueRenderOption {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for AppendResponseValueRenderOption {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for AppendResponseValueRenderOption {
                    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                    where
                        D: ::serde::de::Deserializer<'de>,
                    {
                        let value: &'de str = <&str>::deserialize(deserializer)?;
                        Ok(match value {
                            "FORMATTED_VALUE" => AppendResponseValueRenderOption::FormattedValue,
                            "FORMULA" => AppendResponseValueRenderOption::Formula,
                            "UNFORMATTED_VALUE" => {
                                AppendResponseValueRenderOption::UnformattedValue
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
                impl ::google_field_selector::FieldSelector for AppendResponseValueRenderOption {
                    fn fields() -> Vec<::google_field_selector::Field> {
                        Vec::new()
                    }
                }
                impl ::google_field_selector::ToFieldType for AppendResponseValueRenderOption {
                    fn field_type() -> ::google_field_selector::FieldType {
                        ::google_field_selector::FieldType::Leaf
                    }
                }
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum AppendValueInputOption {
                    InputValueOptionUnspecified,
                    Raw,
                    UserEntered,
                }
                impl AppendValueInputOption {
                    pub fn as_str(self) -> &'static str {
                        match self {
                            AppendValueInputOption::InputValueOptionUnspecified => {
                                "INPUT_VALUE_OPTION_UNSPECIFIED"
                            }
                            AppendValueInputOption::Raw => "RAW",
                            AppendValueInputOption::UserEntered => "USER_ENTERED",
                        }
                    }
                }
                impl ::std::convert::AsRef<str> for AppendValueInputOption {
                    fn as_ref(&self) -> &str {
                        self.as_str()
                    }
                }
                impl ::std::str::FromStr for AppendValueInputOption {
                    type Err = ();
                    fn from_str(s: &str) -> ::std::result::Result<AppendValueInputOption, ()> {
                        Ok(match s {
                            "INPUT_VALUE_OPTION_UNSPECIFIED" => {
                                AppendValueInputOption::InputValueOptionUnspecified
                            }
                            "RAW" => AppendValueInputOption::Raw,
                            "USER_ENTERED" => AppendValueInputOption::UserEntered,
                            _ => return Err(()),
                        })
                    }
                }
                impl ::std::fmt::Display for AppendValueInputOption {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for AppendValueInputOption {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for AppendValueInputOption {
                    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                    where
                        D: ::serde::de::Deserializer<'de>,
                    {
                        let value: &'de str = <&str>::deserialize(deserializer)?;
                        Ok(match value {
                            "INPUT_VALUE_OPTION_UNSPECIFIED" => {
                                AppendValueInputOption::InputValueOptionUnspecified
                            }
                            "RAW" => AppendValueInputOption::Raw,
                            "USER_ENTERED" => AppendValueInputOption::UserEntered,
                            _ => {
                                return Err(::serde::de::Error::custom(format!(
                                    "invalid enum for #name: {}",
                                    value
                                )))
                            }
                        })
                    }
                }
                impl ::google_field_selector::FieldSelector for AppendValueInputOption {
                    fn fields() -> Vec<::google_field_selector::Field> {
                        Vec::new()
                    }
                }
                impl ::google_field_selector::ToFieldType for AppendValueInputOption {
                    fn field_type() -> ::google_field_selector::FieldType {
                        ::google_field_selector::FieldType::Leaf
                    }
                }
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum BatchGetDateTimeRenderOption {
                    FormattedString,
                    SerialNumber,
                }
                impl BatchGetDateTimeRenderOption {
                    pub fn as_str(self) -> &'static str {
                        match self {
                            BatchGetDateTimeRenderOption::FormattedString => "FORMATTED_STRING",
                            BatchGetDateTimeRenderOption::SerialNumber => "SERIAL_NUMBER",
                        }
                    }
                }
                impl ::std::convert::AsRef<str> for BatchGetDateTimeRenderOption {
                    fn as_ref(&self) -> &str {
                        self.as_str()
                    }
                }
                impl ::std::str::FromStr for BatchGetDateTimeRenderOption {
                    type Err = ();
                    fn from_str(
                        s: &str,
                    ) -> ::std::result::Result<BatchGetDateTimeRenderOption, ()>
                    {
                        Ok(match s {
                            "FORMATTED_STRING" => BatchGetDateTimeRenderOption::FormattedString,
                            "SERIAL_NUMBER" => BatchGetDateTimeRenderOption::SerialNumber,
                            _ => return Err(()),
                        })
                    }
                }
                impl ::std::fmt::Display for BatchGetDateTimeRenderOption {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for BatchGetDateTimeRenderOption {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for BatchGetDateTimeRenderOption {
                    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                    where
                        D: ::serde::de::Deserializer<'de>,
                    {
                        let value: &'de str = <&str>::deserialize(deserializer)?;
                        Ok(match value {
                            "FORMATTED_STRING" => BatchGetDateTimeRenderOption::FormattedString,
                            "SERIAL_NUMBER" => BatchGetDateTimeRenderOption::SerialNumber,
                            _ => {
                                return Err(::serde::de::Error::custom(format!(
                                    "invalid enum for #name: {}",
                                    value
                                )))
                            }
                        })
                    }
                }
                impl ::google_field_selector::FieldSelector for BatchGetDateTimeRenderOption {
                    fn fields() -> Vec<::google_field_selector::Field> {
                        Vec::new()
                    }
                }
                impl ::google_field_selector::ToFieldType for BatchGetDateTimeRenderOption {
                    fn field_type() -> ::google_field_selector::FieldType {
                        ::google_field_selector::FieldType::Leaf
                    }
                }
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum BatchGetMajorDimension {
                    Columns,
                    DimensionUnspecified,
                    Rows,
                }
                impl BatchGetMajorDimension {
                    pub fn as_str(self) -> &'static str {
                        match self {
                            BatchGetMajorDimension::Columns => "COLUMNS",
                            BatchGetMajorDimension::DimensionUnspecified => "DIMENSION_UNSPECIFIED",
                            BatchGetMajorDimension::Rows => "ROWS",
                        }
                    }
                }
                impl ::std::convert::AsRef<str> for BatchGetMajorDimension {
                    fn as_ref(&self) -> &str {
                        self.as_str()
                    }
                }
                impl ::std::str::FromStr for BatchGetMajorDimension {
                    type Err = ();
                    fn from_str(s: &str) -> ::std::result::Result<BatchGetMajorDimension, ()> {
                        Ok(match s {
                            "COLUMNS" => BatchGetMajorDimension::Columns,
                            "DIMENSION_UNSPECIFIED" => BatchGetMajorDimension::DimensionUnspecified,
                            "ROWS" => BatchGetMajorDimension::Rows,
                            _ => return Err(()),
                        })
                    }
                }
                impl ::std::fmt::Display for BatchGetMajorDimension {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for BatchGetMajorDimension {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for BatchGetMajorDimension {
                    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                    where
                        D: ::serde::de::Deserializer<'de>,
                    {
                        let value: &'de str = <&str>::deserialize(deserializer)?;
                        Ok(match value {
                            "COLUMNS" => BatchGetMajorDimension::Columns,
                            "DIMENSION_UNSPECIFIED" => BatchGetMajorDimension::DimensionUnspecified,
                            "ROWS" => BatchGetMajorDimension::Rows,
                            _ => {
                                return Err(::serde::de::Error::custom(format!(
                                    "invalid enum for #name: {}",
                                    value
                                )))
                            }
                        })
                    }
                }
                impl ::google_field_selector::FieldSelector for BatchGetMajorDimension {
                    fn fields() -> Vec<::google_field_selector::Field> {
                        Vec::new()
                    }
                }
                impl ::google_field_selector::ToFieldType for BatchGetMajorDimension {
                    fn field_type() -> ::google_field_selector::FieldType {
                        ::google_field_selector::FieldType::Leaf
                    }
                }
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum BatchGetValueRenderOption {
                    FormattedValue,
                    Formula,
                    UnformattedValue,
                }
                impl BatchGetValueRenderOption {
                    pub fn as_str(self) -> &'static str {
                        match self {
                            BatchGetValueRenderOption::FormattedValue => "FORMATTED_VALUE",
                            BatchGetValueRenderOption::Formula => "FORMULA",
                            BatchGetValueRenderOption::UnformattedValue => "UNFORMATTED_VALUE",
                        }
                    }
                }
                impl ::std::convert::AsRef<str> for BatchGetValueRenderOption {
                    fn as_ref(&self) -> &str {
                        self.as_str()
                    }
                }
                impl ::std::str::FromStr for BatchGetValueRenderOption {
                    type Err = ();
                    fn from_str(s: &str) -> ::std::result::Result<BatchGetValueRenderOption, ()> {
                        Ok(match s {
                            "FORMATTED_VALUE" => BatchGetValueRenderOption::FormattedValue,
                            "FORMULA" => BatchGetValueRenderOption::Formula,
                            "UNFORMATTED_VALUE" => BatchGetValueRenderOption::UnformattedValue,
                            _ => return Err(()),
                        })
                    }
                }
                impl ::std::fmt::Display for BatchGetValueRenderOption {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for BatchGetValueRenderOption {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for BatchGetValueRenderOption {
                    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                    where
                        D: ::serde::de::Deserializer<'de>,
                    {
                        let value: &'de str = <&str>::deserialize(deserializer)?;
                        Ok(match value {
                            "FORMATTED_VALUE" => BatchGetValueRenderOption::FormattedValue,
                            "FORMULA" => BatchGetValueRenderOption::Formula,
                            "UNFORMATTED_VALUE" => BatchGetValueRenderOption::UnformattedValue,
                            _ => {
                                return Err(::serde::de::Error::custom(format!(
                                    "invalid enum for #name: {}",
                                    value
                                )))
                            }
                        })
                    }
                }
                impl ::google_field_selector::FieldSelector for BatchGetValueRenderOption {
                    fn fields() -> Vec<::google_field_selector::Field> {
                        Vec::new()
                    }
                }
                impl ::google_field_selector::ToFieldType for BatchGetValueRenderOption {
                    fn field_type() -> ::google_field_selector::FieldType {
                        ::google_field_selector::FieldType::Leaf
                    }
                }
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum GetDateTimeRenderOption {
                    FormattedString,
                    SerialNumber,
                }
                impl GetDateTimeRenderOption {
                    pub fn as_str(self) -> &'static str {
                        match self {
                            GetDateTimeRenderOption::FormattedString => "FORMATTED_STRING",
                            GetDateTimeRenderOption::SerialNumber => "SERIAL_NUMBER",
                        }
                    }
                }
                impl ::std::convert::AsRef<str> for GetDateTimeRenderOption {
                    fn as_ref(&self) -> &str {
                        self.as_str()
                    }
                }
                impl ::std::str::FromStr for GetDateTimeRenderOption {
                    type Err = ();
                    fn from_str(s: &str) -> ::std::result::Result<GetDateTimeRenderOption, ()> {
                        Ok(match s {
                            "FORMATTED_STRING" => GetDateTimeRenderOption::FormattedString,
                            "SERIAL_NUMBER" => GetDateTimeRenderOption::SerialNumber,
                            _ => return Err(()),
                        })
                    }
                }
                impl ::std::fmt::Display for GetDateTimeRenderOption {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for GetDateTimeRenderOption {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for GetDateTimeRenderOption {
                    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                    where
                        D: ::serde::de::Deserializer<'de>,
                    {
                        let value: &'de str = <&str>::deserialize(deserializer)?;
                        Ok(match value {
                            "FORMATTED_STRING" => GetDateTimeRenderOption::FormattedString,
                            "SERIAL_NUMBER" => GetDateTimeRenderOption::SerialNumber,
                            _ => {
                                return Err(::serde::de::Error::custom(format!(
                                    "invalid enum for #name: {}",
                                    value
                                )))
                            }
                        })
                    }
                }
                impl ::google_field_selector::FieldSelector for GetDateTimeRenderOption {
                    fn fields() -> Vec<::google_field_selector::Field> {
                        Vec::new()
                    }
                }
                impl ::google_field_selector::ToFieldType for GetDateTimeRenderOption {
                    fn field_type() -> ::google_field_selector::FieldType {
                        ::google_field_selector::FieldType::Leaf
                    }
                }
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum GetMajorDimension {
                    Columns,
                    DimensionUnspecified,
                    Rows,
                }
                impl GetMajorDimension {
                    pub fn as_str(self) -> &'static str {
                        match self {
                            GetMajorDimension::Columns => "COLUMNS",
                            GetMajorDimension::DimensionUnspecified => "DIMENSION_UNSPECIFIED",
                            GetMajorDimension::Rows => "ROWS",
                        }
                    }
                }
                impl ::std::convert::AsRef<str> for GetMajorDimension {
                    fn as_ref(&self) -> &str {
                        self.as_str()
                    }
                }
                impl ::std::str::FromStr for GetMajorDimension {
                    type Err = ();
                    fn from_str(s: &str) -> ::std::result::Result<GetMajorDimension, ()> {
                        Ok(match s {
                            "COLUMNS" => GetMajorDimension::Columns,
                            "DIMENSION_UNSPECIFIED" => GetMajorDimension::DimensionUnspecified,
                            "ROWS" => GetMajorDimension::Rows,
                            _ => return Err(()),
                        })
                    }
                }
                impl ::std::fmt::Display for GetMajorDimension {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for GetMajorDimension {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for GetMajorDimension {
                    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                    where
                        D: ::serde::de::Deserializer<'de>,
                    {
                        let value: &'de str = <&str>::deserialize(deserializer)?;
                        Ok(match value {
                            "COLUMNS" => GetMajorDimension::Columns,
                            "DIMENSION_UNSPECIFIED" => GetMajorDimension::DimensionUnspecified,
                            "ROWS" => GetMajorDimension::Rows,
                            _ => {
                                return Err(::serde::de::Error::custom(format!(
                                    "invalid enum for #name: {}",
                                    value
                                )))
                            }
                        })
                    }
                }
                impl ::google_field_selector::FieldSelector for GetMajorDimension {
                    fn fields() -> Vec<::google_field_selector::Field> {
                        Vec::new()
                    }
                }
                impl ::google_field_selector::ToFieldType for GetMajorDimension {
                    fn field_type() -> ::google_field_selector::FieldType {
                        ::google_field_selector::FieldType::Leaf
                    }
                }
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum GetValueRenderOption {
                    FormattedValue,
                    Formula,
                    UnformattedValue,
                }
                impl GetValueRenderOption {
                    pub fn as_str(self) -> &'static str {
                        match self {
                            GetValueRenderOption::FormattedValue => "FORMATTED_VALUE",
                            GetValueRenderOption::Formula => "FORMULA",
                            GetValueRenderOption::UnformattedValue => "UNFORMATTED_VALUE",
                        }
                    }
                }
                impl ::std::convert::AsRef<str> for GetValueRenderOption {
                    fn as_ref(&self) -> &str {
                        self.as_str()
                    }
                }
                impl ::std::str::FromStr for GetValueRenderOption {
                    type Err = ();
                    fn from_str(s: &str) -> ::std::result::Result<GetValueRenderOption, ()> {
                        Ok(match s {
                            "FORMATTED_VALUE" => GetValueRenderOption::FormattedValue,
                            "FORMULA" => GetValueRenderOption::Formula,
                            "UNFORMATTED_VALUE" => GetValueRenderOption::UnformattedValue,
                            _ => return Err(()),
                        })
                    }
                }
                impl ::std::fmt::Display for GetValueRenderOption {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for GetValueRenderOption {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for GetValueRenderOption {
                    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                    where
                        D: ::serde::de::Deserializer<'de>,
                    {
                        let value: &'de str = <&str>::deserialize(deserializer)?;
                        Ok(match value {
                            "FORMATTED_VALUE" => GetValueRenderOption::FormattedValue,
                            "FORMULA" => GetValueRenderOption::Formula,
                            "UNFORMATTED_VALUE" => GetValueRenderOption::UnformattedValue,
                            _ => {
                                return Err(::serde::de::Error::custom(format!(
                                    "invalid enum for #name: {}",
                                    value
                                )))
                            }
                        })
                    }
                }
                impl ::google_field_selector::FieldSelector for GetValueRenderOption {
                    fn fields() -> Vec<::google_field_selector::Field> {
                        Vec::new()
                    }
                }
                impl ::google_field_selector::ToFieldType for GetValueRenderOption {
                    fn field_type() -> ::google_field_selector::FieldType {
                        ::google_field_selector::FieldType::Leaf
                    }
                }
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum UpdateResponseDateTimeRenderOption {
                    FormattedString,
                    SerialNumber,
                }
                impl UpdateResponseDateTimeRenderOption {
                    pub fn as_str(self) -> &'static str {
                        match self {
                            UpdateResponseDateTimeRenderOption::FormattedString => {
                                "FORMATTED_STRING"
                            }
                            UpdateResponseDateTimeRenderOption::SerialNumber => "SERIAL_NUMBER",
                        }
                    }
                }
                impl ::std::convert::AsRef<str> for UpdateResponseDateTimeRenderOption {
                    fn as_ref(&self) -> &str {
                        self.as_str()
                    }
                }
                impl ::std::str::FromStr for UpdateResponseDateTimeRenderOption {
                    type Err = ();
                    fn from_str(
                        s: &str,
                    ) -> ::std::result::Result<UpdateResponseDateTimeRenderOption, ()>
                    {
                        Ok(match s {
                            "FORMATTED_STRING" => {
                                UpdateResponseDateTimeRenderOption::FormattedString
                            }
                            "SERIAL_NUMBER" => UpdateResponseDateTimeRenderOption::SerialNumber,
                            _ => return Err(()),
                        })
                    }
                }
                impl ::std::fmt::Display for UpdateResponseDateTimeRenderOption {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for UpdateResponseDateTimeRenderOption {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for UpdateResponseDateTimeRenderOption {
                    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                    where
                        D: ::serde::de::Deserializer<'de>,
                    {
                        let value: &'de str = <&str>::deserialize(deserializer)?;
                        Ok(match value {
                            "FORMATTED_STRING" => {
                                UpdateResponseDateTimeRenderOption::FormattedString
                            }
                            "SERIAL_NUMBER" => UpdateResponseDateTimeRenderOption::SerialNumber,
                            _ => {
                                return Err(::serde::de::Error::custom(format!(
                                    "invalid enum for #name: {}",
                                    value
                                )))
                            }
                        })
                    }
                }
                impl ::google_field_selector::FieldSelector for UpdateResponseDateTimeRenderOption {
                    fn fields() -> Vec<::google_field_selector::Field> {
                        Vec::new()
                    }
                }
                impl ::google_field_selector::ToFieldType for UpdateResponseDateTimeRenderOption {
                    fn field_type() -> ::google_field_selector::FieldType {
                        ::google_field_selector::FieldType::Leaf
                    }
                }
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum UpdateResponseValueRenderOption {
                    FormattedValue,
                    Formula,
                    UnformattedValue,
                }
                impl UpdateResponseValueRenderOption {
                    pub fn as_str(self) -> &'static str {
                        match self {
                            UpdateResponseValueRenderOption::FormattedValue => "FORMATTED_VALUE",
                            UpdateResponseValueRenderOption::Formula => "FORMULA",
                            UpdateResponseValueRenderOption::UnformattedValue => {
                                "UNFORMATTED_VALUE"
                            }
                        }
                    }
                }
                impl ::std::convert::AsRef<str> for UpdateResponseValueRenderOption {
                    fn as_ref(&self) -> &str {
                        self.as_str()
                    }
                }
                impl ::std::str::FromStr for UpdateResponseValueRenderOption {
                    type Err = ();
                    fn from_str(
                        s: &str,
                    ) -> ::std::result::Result<UpdateResponseValueRenderOption, ()>
                    {
                        Ok(match s {
                            "FORMATTED_VALUE" => UpdateResponseValueRenderOption::FormattedValue,
                            "FORMULA" => UpdateResponseValueRenderOption::Formula,
                            "UNFORMATTED_VALUE" => {
                                UpdateResponseValueRenderOption::UnformattedValue
                            }
                            _ => return Err(()),
                        })
                    }
                }
                impl ::std::fmt::Display for UpdateResponseValueRenderOption {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for UpdateResponseValueRenderOption {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for UpdateResponseValueRenderOption {
                    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                    where
                        D: ::serde::de::Deserializer<'de>,
                    {
                        let value: &'de str = <&str>::deserialize(deserializer)?;
                        Ok(match value {
                            "FORMATTED_VALUE" => UpdateResponseValueRenderOption::FormattedValue,
                            "FORMULA" => UpdateResponseValueRenderOption::Formula,
                            "UNFORMATTED_VALUE" => {
                                UpdateResponseValueRenderOption::UnformattedValue
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
                impl ::google_field_selector::FieldSelector for UpdateResponseValueRenderOption {
                    fn fields() -> Vec<::google_field_selector::Field> {
                        Vec::new()
                    }
                }
                impl ::google_field_selector::ToFieldType for UpdateResponseValueRenderOption {
                    fn field_type() -> ::google_field_selector::FieldType {
                        ::google_field_selector::FieldType::Leaf
                    }
                }
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum UpdateValueInputOption {
                    InputValueOptionUnspecified,
                    Raw,
                    UserEntered,
                }
                impl UpdateValueInputOption {
                    pub fn as_str(self) -> &'static str {
                        match self {
                            UpdateValueInputOption::InputValueOptionUnspecified => {
                                "INPUT_VALUE_OPTION_UNSPECIFIED"
                            }
                            UpdateValueInputOption::Raw => "RAW",
                            UpdateValueInputOption::UserEntered => "USER_ENTERED",
                        }
                    }
                }
                impl ::std::convert::AsRef<str> for UpdateValueInputOption {
                    fn as_ref(&self) -> &str {
                        self.as_str()
                    }
                }
                impl ::std::str::FromStr for UpdateValueInputOption {
                    type Err = ();
                    fn from_str(s: &str) -> ::std::result::Result<UpdateValueInputOption, ()> {
                        Ok(match s {
                            "INPUT_VALUE_OPTION_UNSPECIFIED" => {
                                UpdateValueInputOption::InputValueOptionUnspecified
                            }
                            "RAW" => UpdateValueInputOption::Raw,
                            "USER_ENTERED" => UpdateValueInputOption::UserEntered,
                            _ => return Err(()),
                        })
                    }
                }
                impl ::std::fmt::Display for UpdateValueInputOption {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for UpdateValueInputOption {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for UpdateValueInputOption {
                    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                    where
                        D: ::serde::de::Deserializer<'de>,
                    {
                        let value: &'de str = <&str>::deserialize(deserializer)?;
                        Ok(match value {
                            "INPUT_VALUE_OPTION_UNSPECIFIED" => {
                                UpdateValueInputOption::InputValueOptionUnspecified
                            }
                            "RAW" => UpdateValueInputOption::Raw,
                            "USER_ENTERED" => UpdateValueInputOption::UserEntered,
                            _ => {
                                return Err(::serde::de::Error::custom(format!(
                                    "invalid enum for #name: {}",
                                    value
                                )))
                            }
                        })
                    }
                }
                impl ::google_field_selector::FieldSelector for UpdateValueInputOption {
                    fn fields() -> Vec<::google_field_selector::Field> {
                        Vec::new()
                    }
                }
                impl ::google_field_selector::ToFieldType for UpdateValueInputOption {
                    fn field_type() -> ::google_field_selector::FieldType {
                        ::google_field_selector::FieldType::Leaf
                    }
                }
            }
            pub struct ValuesActions<'a> {
                pub(crate) reqwest: &'a reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> ValuesActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Appends values to a spreadsheet. The input range is used to search for\nexisting data and find a \"table\" within that range. Values will be\nappended to the next row of the table, starting with the first column of\nthe table. See the\n[guide](/sheets/api/guides/values#appending_values)\nand\n[sample code](/sheets/api/samples/writing#append_values)\nfor specific details of how tables are detected and data is appended.\n\nThe caller must specify the spreadsheet ID, range, and\na valueInputOption.  The `valueInputOption` only\ncontrols how the input data will be added to the sheet (column-wise or\nrow-wise), it does not influence what cell the data starts being written\nto."]
                pub fn append(
                    &self,
                    request: crate::schemas::ValueRange,
                    spreadsheet_id: impl Into<String>,
                    range: impl Into<String>,
                ) -> AppendRequestBuilder {
                    AppendRequestBuilder {
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
                        spreadsheet_id: spreadsheet_id.into(),
                        range: range.into(),
                        include_values_in_response: None,
                        insert_data_option: None,
                        response_date_time_render_option: None,
                        response_value_render_option: None,
                        value_input_option: None,
                    }
                }
                #[doc = "Clears one or more ranges of values from a spreadsheet.\nThe caller must specify the spreadsheet ID and one or more ranges.\nOnly values are cleared -- all other properties of the cell (such as\nformatting, data validation, etc..) are kept."]
                pub fn batch_clear(
                    &self,
                    request: crate::schemas::BatchClearValuesRequest,
                    spreadsheet_id: impl Into<String>,
                ) -> BatchClearRequestBuilder {
                    BatchClearRequestBuilder {
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
                        spreadsheet_id: spreadsheet_id.into(),
                    }
                }
                #[doc = "Clears one or more ranges of values from a spreadsheet.\nThe caller must specify the spreadsheet ID and one or more\nDataFilters. Ranges matching any of the specified data\nfilters will be cleared.  Only values are cleared -- all other properties\nof the cell (such as formatting, data validation, etc..) are kept."]
                pub fn batch_clear_by_data_filter(
                    &self,
                    request: crate::schemas::BatchClearValuesByDataFilterRequest,
                    spreadsheet_id: impl Into<String>,
                ) -> BatchClearByDataFilterRequestBuilder {
                    BatchClearByDataFilterRequestBuilder {
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
                        spreadsheet_id: spreadsheet_id.into(),
                    }
                }
                #[doc = "Returns one or more ranges of values from a spreadsheet.\nThe caller must specify the spreadsheet ID and one or more ranges."]
                pub fn batch_get(
                    &self,
                    spreadsheet_id: impl Into<String>,
                ) -> BatchGetRequestBuilder {
                    BatchGetRequestBuilder {
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
                        spreadsheet_id: spreadsheet_id.into(),
                        date_time_render_option: None,
                        major_dimension: None,
                        ranges: None,
                        value_render_option: None,
                    }
                }
                #[doc = "Returns one or more ranges of values that match the specified data filters.\nThe caller must specify the spreadsheet ID and one or more\nDataFilters.  Ranges that match any of the data filters in\nthe request will be returned."]
                pub fn batch_get_by_data_filter(
                    &self,
                    request: crate::schemas::BatchGetValuesByDataFilterRequest,
                    spreadsheet_id: impl Into<String>,
                ) -> BatchGetByDataFilterRequestBuilder {
                    BatchGetByDataFilterRequestBuilder {
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
                        spreadsheet_id: spreadsheet_id.into(),
                    }
                }
                #[doc = "Sets values in one or more ranges of a spreadsheet.\nThe caller must specify the spreadsheet ID,\na valueInputOption, and one or more\nValueRanges."]
                pub fn batch_update(
                    &self,
                    request: crate::schemas::BatchUpdateValuesRequest,
                    spreadsheet_id: impl Into<String>,
                ) -> BatchUpdateRequestBuilder {
                    BatchUpdateRequestBuilder {
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
                        spreadsheet_id: spreadsheet_id.into(),
                    }
                }
                #[doc = "Sets values in one or more ranges of a spreadsheet.\nThe caller must specify the spreadsheet ID,\na valueInputOption, and one or more\nDataFilterValueRanges."]
                pub fn batch_update_by_data_filter(
                    &self,
                    request: crate::schemas::BatchUpdateValuesByDataFilterRequest,
                    spreadsheet_id: impl Into<String>,
                ) -> BatchUpdateByDataFilterRequestBuilder {
                    BatchUpdateByDataFilterRequestBuilder {
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
                        spreadsheet_id: spreadsheet_id.into(),
                    }
                }
                #[doc = "Clears values from a spreadsheet.\nThe caller must specify the spreadsheet ID and range.\nOnly values are cleared -- all other properties of the cell (such as\nformatting, data validation, etc..) are kept."]
                pub fn clear(
                    &self,
                    request: crate::schemas::ClearValuesRequest,
                    spreadsheet_id: impl Into<String>,
                    range: impl Into<String>,
                ) -> ClearRequestBuilder {
                    ClearRequestBuilder {
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
                        spreadsheet_id: spreadsheet_id.into(),
                        range: range.into(),
                    }
                }
                #[doc = "Returns a range of values from a spreadsheet.\nThe caller must specify the spreadsheet ID and a range."]
                pub fn get(
                    &self,
                    spreadsheet_id: impl Into<String>,
                    range: impl Into<String>,
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
                        spreadsheet_id: spreadsheet_id.into(),
                        range: range.into(),
                        date_time_render_option: None,
                        major_dimension: None,
                        value_render_option: None,
                    }
                }
                #[doc = "Sets values in a range of a spreadsheet.\nThe caller must specify the spreadsheet ID, range, and\na valueInputOption."]
                pub fn update(
                    &self,
                    request: crate::schemas::ValueRange,
                    spreadsheet_id: impl Into<String>,
                    range: impl Into<String>,
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
                        spreadsheet_id: spreadsheet_id.into(),
                        range: range.into(),
                        include_values_in_response: None,
                        response_date_time_render_option: None,
                        response_value_render_option: None,
                        value_input_option: None,
                    }
                }
            }
            #[doc = "Created via [ValuesActions::append()](struct.ValuesActions.html#method.append)"]
            #[derive(Debug, Clone)]
            pub struct AppendRequestBuilder < 'a > { pub ( crate ) reqwest : & 'a :: reqwest :: blocking :: Client , pub ( crate ) auth : & 'a dyn :: google_api_auth :: GetAccessToken , request : crate :: schemas :: ValueRange , spreadsheet_id : String , range : String , include_values_in_response : Option < bool > , insert_data_option : Option < crate :: resources :: spreadsheets :: values :: params :: AppendInsertDataOption > , response_date_time_render_option : Option < crate :: resources :: spreadsheets :: values :: params :: AppendResponseDateTimeRenderOption > , response_value_render_option : Option < crate :: resources :: spreadsheets :: values :: params :: AppendResponseValueRenderOption > , value_input_option : Option < crate :: resources :: spreadsheets :: values :: params :: AppendValueInputOption > , access_token : Option < String > , alt : Option < crate :: params :: Alt > , callback : Option < String > , fields : Option < String > , key : Option < String > , oauth_token : Option < String > , pretty_print : Option < bool > , quota_user : Option < String > , upload_protocol : Option < String > , upload_type : Option < String > , xgafv : Option < crate :: params :: Xgafv > , }
            impl<'a> AppendRequestBuilder<'a> {
                #[doc = "Determines if the update response should include the values\nof the cells that were appended. By default, responses\ndo not include the updated values."]
                pub fn include_values_in_response(mut self, value: bool) -> Self {
                    self.include_values_in_response = Some(value);
                    self
                }
                #[doc = "How the input data should be inserted."]
                pub fn insert_data_option(
                    mut self,
                    value: crate::resources::spreadsheets::values::params::AppendInsertDataOption,
                ) -> Self {
                    self.insert_data_option = Some(value);
                    self
                }
                #[doc = "Determines how dates, times, and durations in the response should be\nrendered. This is ignored if response_value_render_option is\nFORMATTED_VALUE.\nThe default dateTime render option is [DateTimeRenderOption.SERIAL_NUMBER]."]
                pub fn response_date_time_render_option(
                    mut self,
                    value : crate :: resources :: spreadsheets :: values :: params :: AppendResponseDateTimeRenderOption,
                ) -> Self {
                    self.response_date_time_render_option = Some(value);
                    self
                }
                #[doc = "Determines how values in the response should be rendered.\nThe default render option is ValueRenderOption.FORMATTED_VALUE."]
                pub fn response_value_render_option(
                    mut self,
                    value : crate :: resources :: spreadsheets :: values :: params :: AppendResponseValueRenderOption,
                ) -> Self {
                    self.response_value_render_option = Some(value);
                    self
                }
                #[doc = "How the input data should be interpreted."]
                pub fn value_input_option(
                    mut self,
                    value: crate::resources::spreadsheets::values::params::AppendValueInputOption,
                ) -> Self {
                    self.value_input_option = Some(value);
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
                ) -> Result<crate::schemas::AppendValuesResponse, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::AppendValuesResponse, crate::Error> {
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
                    let mut output = "https://sheets.googleapis.com/".to_owned();
                    output.push_str("v4/spreadsheets/");
                    {
                        let var_as_str = &self.spreadsheet_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/values/");
                    {
                        let var_as_str = &self.range;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str(":append");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let req = self.reqwest.request(::reqwest::Method::POST, path);
                    let req =
                        req.query(&[("includeValuesInResponse", &self.include_values_in_response)]);
                    let req = req.query(&[("insertDataOption", &self.insert_data_option)]);
                    let req = req.query(&[(
                        "responseDateTimeRenderOption",
                        &self.response_date_time_render_option,
                    )]);
                    let req = req.query(&[(
                        "responseValueRenderOption",
                        &self.response_value_render_option,
                    )]);
                    let req = req.query(&[("valueInputOption", &self.value_input_option)]);
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
            #[doc = "Created via [ValuesActions::batch_clear()](struct.ValuesActions.html#method.batch_clear)"]
            #[derive(Debug, Clone)]
            pub struct BatchClearRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::BatchClearValuesRequest,
                spreadsheet_id: String,
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
            impl<'a> BatchClearRequestBuilder<'a> {
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
                ) -> Result<crate::schemas::BatchClearValuesResponse, crate::Error>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::BatchClearValuesResponse, crate::Error>
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
                    let mut output = "https://sheets.googleapis.com/".to_owned();
                    output.push_str("v4/spreadsheets/");
                    {
                        let var_as_str = &self.spreadsheet_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/values:batchClear");
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
            #[doc = "Created via [ValuesActions::batch_clear_by_data_filter()](struct.ValuesActions.html#method.batch_clear_by_data_filter)"]
            #[derive(Debug, Clone)]
            pub struct BatchClearByDataFilterRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::BatchClearValuesByDataFilterRequest,
                spreadsheet_id: String,
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
            impl<'a> BatchClearByDataFilterRequestBuilder<'a> {
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
                ) -> Result<crate::schemas::BatchClearValuesByDataFilterResponse, crate::Error>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::BatchClearValuesByDataFilterResponse, crate::Error>
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
                    let mut output = "https://sheets.googleapis.com/".to_owned();
                    output.push_str("v4/spreadsheets/");
                    {
                        let var_as_str = &self.spreadsheet_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/values:batchClearByDataFilter");
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
            #[doc = "Created via [ValuesActions::batch_get()](struct.ValuesActions.html#method.batch_get)"]
            #[derive(Debug, Clone)]
            pub struct BatchGetRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                spreadsheet_id: String,
                date_time_render_option: Option<
                    crate::resources::spreadsheets::values::params::BatchGetDateTimeRenderOption,
                >,
                major_dimension:
                    Option<crate::resources::spreadsheets::values::params::BatchGetMajorDimension>,
                ranges: Option<Vec<String>>,
                value_render_option: Option<
                    crate::resources::spreadsheets::values::params::BatchGetValueRenderOption,
                >,
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
            impl<'a> BatchGetRequestBuilder<'a> {
                #[doc = "How dates, times, and durations should be represented in the output.\nThis is ignored if value_render_option is\nFORMATTED_VALUE.\nThe default dateTime render option is [DateTimeRenderOption.SERIAL_NUMBER]."]
                pub fn date_time_render_option(
                    mut self,
                    value : crate :: resources :: spreadsheets :: values :: params :: BatchGetDateTimeRenderOption,
                ) -> Self {
                    self.date_time_render_option = Some(value);
                    self
                }
                #[doc = "The major dimension that results should use.\n\nFor example, if the spreadsheet data is: `A1=1,B1=2,A2=3,B2=4`,\nthen requesting `range=A1:B2,majorDimension=ROWS` returns `[[1,2],[3,4]]`,\nwhereas requesting `range=A1:B2,majorDimension=COLUMNS` returns\n`[[1,3],[2,4]]`."]
                pub fn major_dimension(
                    mut self,
                    value: crate::resources::spreadsheets::values::params::BatchGetMajorDimension,
                ) -> Self {
                    self.major_dimension = Some(value);
                    self
                }
                #[doc = "The A1 notation of the values to retrieve."]
                pub fn ranges(mut self, value: impl Into<Vec<String>>) -> Self {
                    self.ranges = Some(value.into());
                    self
                }
                #[doc = "How values should be represented in the output.\nThe default render option is ValueRenderOption.FORMATTED_VALUE."]
                pub fn value_render_option(
                    mut self,
                    value : crate :: resources :: spreadsheets :: values :: params :: BatchGetValueRenderOption,
                ) -> Self {
                    self.value_render_option = Some(value);
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
                ) -> Result<crate::schemas::BatchGetValuesResponse, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::BatchGetValuesResponse, crate::Error> {
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
                    let mut output = "https://sheets.googleapis.com/".to_owned();
                    output.push_str("v4/spreadsheets/");
                    {
                        let var_as_str = &self.spreadsheet_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/values:batchGet");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("dateTimeRenderOption", &self.date_time_render_option)]);
                    let req = req.query(&[("majorDimension", &self.major_dimension)]);
                    let req = req.query(&[("ranges", &self.ranges)]);
                    let req = req.query(&[("valueRenderOption", &self.value_render_option)]);
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
            #[doc = "Created via [ValuesActions::batch_get_by_data_filter()](struct.ValuesActions.html#method.batch_get_by_data_filter)"]
            #[derive(Debug, Clone)]
            pub struct BatchGetByDataFilterRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::BatchGetValuesByDataFilterRequest,
                spreadsheet_id: String,
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
            impl<'a> BatchGetByDataFilterRequestBuilder<'a> {
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
                ) -> Result<crate::schemas::BatchGetValuesByDataFilterResponse, crate::Error>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::BatchGetValuesByDataFilterResponse, crate::Error>
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
                    let mut output = "https://sheets.googleapis.com/".to_owned();
                    output.push_str("v4/spreadsheets/");
                    {
                        let var_as_str = &self.spreadsheet_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/values:batchGetByDataFilter");
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
            #[doc = "Created via [ValuesActions::batch_update()](struct.ValuesActions.html#method.batch_update)"]
            #[derive(Debug, Clone)]
            pub struct BatchUpdateRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::BatchUpdateValuesRequest,
                spreadsheet_id: String,
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
            impl<'a> BatchUpdateRequestBuilder<'a> {
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
                ) -> Result<crate::schemas::BatchUpdateValuesResponse, crate::Error>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::BatchUpdateValuesResponse, crate::Error>
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
                    let mut output = "https://sheets.googleapis.com/".to_owned();
                    output.push_str("v4/spreadsheets/");
                    {
                        let var_as_str = &self.spreadsheet_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/values:batchUpdate");
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
            #[doc = "Created via [ValuesActions::batch_update_by_data_filter()](struct.ValuesActions.html#method.batch_update_by_data_filter)"]
            #[derive(Debug, Clone)]
            pub struct BatchUpdateByDataFilterRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::BatchUpdateValuesByDataFilterRequest,
                spreadsheet_id: String,
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
            impl<'a> BatchUpdateByDataFilterRequestBuilder<'a> {
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
                ) -> Result<crate::schemas::BatchUpdateValuesByDataFilterResponse, crate::Error>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::BatchUpdateValuesByDataFilterResponse, crate::Error>
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
                    let mut output = "https://sheets.googleapis.com/".to_owned();
                    output.push_str("v4/spreadsheets/");
                    {
                        let var_as_str = &self.spreadsheet_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/values:batchUpdateByDataFilter");
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
            #[doc = "Created via [ValuesActions::clear()](struct.ValuesActions.html#method.clear)"]
            #[derive(Debug, Clone)]
            pub struct ClearRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::ClearValuesRequest,
                spreadsheet_id: String,
                range: String,
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
            impl<'a> ClearRequestBuilder<'a> {
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
                ) -> Result<crate::schemas::ClearValuesResponse, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::ClearValuesResponse, crate::Error> {
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
                    let mut output = "https://sheets.googleapis.com/".to_owned();
                    output.push_str("v4/spreadsheets/");
                    {
                        let var_as_str = &self.spreadsheet_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/values/");
                    {
                        let var_as_str = &self.range;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str(":clear");
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
            #[doc = "Created via [ValuesActions::get()](struct.ValuesActions.html#method.get)"]
            #[derive(Debug, Clone)]
            pub struct GetRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                spreadsheet_id: String,
                range: String,
                date_time_render_option:
                    Option<crate::resources::spreadsheets::values::params::GetDateTimeRenderOption>,
                major_dimension:
                    Option<crate::resources::spreadsheets::values::params::GetMajorDimension>,
                value_render_option:
                    Option<crate::resources::spreadsheets::values::params::GetValueRenderOption>,
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
                #[doc = "How dates, times, and durations should be represented in the output.\nThis is ignored if value_render_option is\nFORMATTED_VALUE.\nThe default dateTime render option is [DateTimeRenderOption.SERIAL_NUMBER]."]
                pub fn date_time_render_option(
                    mut self,
                    value: crate::resources::spreadsheets::values::params::GetDateTimeRenderOption,
                ) -> Self {
                    self.date_time_render_option = Some(value);
                    self
                }
                #[doc = "The major dimension that results should use.\n\nFor example, if the spreadsheet data is: `A1=1,B1=2,A2=3,B2=4`, then\nrequesting `range=A1:B2,majorDimension=ROWS` returns `[[1,2],[3,4]]`,\nwhereas requesting `range=A1:B2,majorDimension=COLUMNS` returns\n`[[1,3],[2,4]]`."]
                pub fn major_dimension(
                    mut self,
                    value: crate::resources::spreadsheets::values::params::GetMajorDimension,
                ) -> Self {
                    self.major_dimension = Some(value);
                    self
                }
                #[doc = "How values should be represented in the output.\nThe default render option is ValueRenderOption.FORMATTED_VALUE."]
                pub fn value_render_option(
                    mut self,
                    value: crate::resources::spreadsheets::values::params::GetValueRenderOption,
                ) -> Self {
                    self.value_render_option = Some(value);
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
                ) -> Result<crate::schemas::ValueRange, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::ValueRange, crate::Error> {
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
                    let mut output = "https://sheets.googleapis.com/".to_owned();
                    output.push_str("v4/spreadsheets/");
                    {
                        let var_as_str = &self.spreadsheet_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/values/");
                    {
                        let var_as_str = &self.range;
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
                    let req = req.query(&[("dateTimeRenderOption", &self.date_time_render_option)]);
                    let req = req.query(&[("majorDimension", &self.major_dimension)]);
                    let req = req.query(&[("valueRenderOption", &self.value_render_option)]);
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
            #[doc = "Created via [ValuesActions::update()](struct.ValuesActions.html#method.update)"]
            #[derive(Debug, Clone)]
            pub struct UpdateRequestBuilder < 'a > { pub ( crate ) reqwest : & 'a :: reqwest :: blocking :: Client , pub ( crate ) auth : & 'a dyn :: google_api_auth :: GetAccessToken , request : crate :: schemas :: ValueRange , spreadsheet_id : String , range : String , include_values_in_response : Option < bool > , response_date_time_render_option : Option < crate :: resources :: spreadsheets :: values :: params :: UpdateResponseDateTimeRenderOption > , response_value_render_option : Option < crate :: resources :: spreadsheets :: values :: params :: UpdateResponseValueRenderOption > , value_input_option : Option < crate :: resources :: spreadsheets :: values :: params :: UpdateValueInputOption > , access_token : Option < String > , alt : Option < crate :: params :: Alt > , callback : Option < String > , fields : Option < String > , key : Option < String > , oauth_token : Option < String > , pretty_print : Option < bool > , quota_user : Option < String > , upload_protocol : Option < String > , upload_type : Option < String > , xgafv : Option < crate :: params :: Xgafv > , }
            impl<'a> UpdateRequestBuilder<'a> {
                #[doc = "Determines if the update response should include the values\nof the cells that were updated. By default, responses\ndo not include the updated values.\nIf the range to write was larger than the range actually written, the\nresponse includes all values in the requested range (excluding trailing\nempty rows and columns)."]
                pub fn include_values_in_response(mut self, value: bool) -> Self {
                    self.include_values_in_response = Some(value);
                    self
                }
                #[doc = "Determines how dates, times, and durations in the response should be\nrendered. This is ignored if response_value_render_option is\nFORMATTED_VALUE.\nThe default dateTime render option is\nDateTimeRenderOption.SERIAL_NUMBER."]
                pub fn response_date_time_render_option(
                    mut self,
                    value : crate :: resources :: spreadsheets :: values :: params :: UpdateResponseDateTimeRenderOption,
                ) -> Self {
                    self.response_date_time_render_option = Some(value);
                    self
                }
                #[doc = "Determines how values in the response should be rendered.\nThe default render option is ValueRenderOption.FORMATTED_VALUE."]
                pub fn response_value_render_option(
                    mut self,
                    value : crate :: resources :: spreadsheets :: values :: params :: UpdateResponseValueRenderOption,
                ) -> Self {
                    self.response_value_render_option = Some(value);
                    self
                }
                #[doc = "How the input data should be interpreted."]
                pub fn value_input_option(
                    mut self,
                    value: crate::resources::spreadsheets::values::params::UpdateValueInputOption,
                ) -> Self {
                    self.value_input_option = Some(value);
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
                ) -> Result<crate::schemas::UpdateValuesResponse, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::UpdateValuesResponse, crate::Error> {
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
                    let mut output = "https://sheets.googleapis.com/".to_owned();
                    output.push_str("v4/spreadsheets/");
                    {
                        let var_as_str = &self.spreadsheet_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/values/");
                    {
                        let var_as_str = &self.range;
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
                    let req =
                        req.query(&[("includeValuesInResponse", &self.include_values_in_response)]);
                    let req = req.query(&[(
                        "responseDateTimeRenderOption",
                        &self.response_date_time_render_option,
                    )]);
                    let req = req.query(&[(
                        "responseValueRenderOption",
                        &self.response_value_render_option,
                    )]);
                    let req = req.query(&[("valueInputOption", &self.value_input_option)]);
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
