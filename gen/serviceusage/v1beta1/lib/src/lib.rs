#![doc = "# Resources and Methods\n    * [operations](resources/operations/struct.OperationsActions.html)\n      * [*get*](resources/operations/struct.GetRequestBuilder.html), [*list*](resources/operations/struct.ListRequestBuilder.html)\n    * [services](resources/services/struct.ServicesActions.html)\n      * [*batchEnable*](resources/services/struct.BatchEnableRequestBuilder.html), [*disable*](resources/services/struct.DisableRequestBuilder.html), [*enable*](resources/services/struct.EnableRequestBuilder.html), [*get*](resources/services/struct.GetRequestBuilder.html), [*list*](resources/services/struct.ListRequestBuilder.html)\n      * [consumer_quota_metrics](resources/services/consumer_quota_metrics/struct.ConsumerQuotaMetricsActions.html)\n        * [*get*](resources/services/consumer_quota_metrics/struct.GetRequestBuilder.html), [*list*](resources/services/consumer_quota_metrics/struct.ListRequestBuilder.html)\n        * [limits](resources/services/consumer_quota_metrics/limits/struct.LimitsActions.html)\n          * [*get*](resources/services/consumer_quota_metrics/limits/struct.GetRequestBuilder.html)\n          * [admin_overrides](resources/services/consumer_quota_metrics/limits/admin_overrides/struct.AdminOverridesActions.html)\n            * [*create*](resources/services/consumer_quota_metrics/limits/admin_overrides/struct.CreateRequestBuilder.html), [*delete*](resources/services/consumer_quota_metrics/limits/admin_overrides/struct.DeleteRequestBuilder.html), [*list*](resources/services/consumer_quota_metrics/limits/admin_overrides/struct.ListRequestBuilder.html), [*patch*](resources/services/consumer_quota_metrics/limits/admin_overrides/struct.PatchRequestBuilder.html)\n          * [consumer_overrides](resources/services/consumer_quota_metrics/limits/consumer_overrides/struct.ConsumerOverridesActions.html)\n            * [*create*](resources/services/consumer_quota_metrics/limits/consumer_overrides/struct.CreateRequestBuilder.html), [*delete*](resources/services/consumer_quota_metrics/limits/consumer_overrides/struct.DeleteRequestBuilder.html), [*list*](resources/services/consumer_quota_metrics/limits/consumer_overrides/struct.ListRequestBuilder.html), [*patch*](resources/services/consumer_quota_metrics/limits/consumer_overrides/struct.PatchRequestBuilder.html)\n"]
pub mod scopes {
    #[doc = "View and manage your data across Google Cloud Platform services\n\n`https://www.googleapis.com/auth/cloud-platform`"]
    pub const CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";
    #[doc = "View your data across Google Cloud Platform services\n\n`https://www.googleapis.com/auth/cloud-platform.read-only`"]
    pub const CLOUD_PLATFORM_READ_ONLY: &str =
        "https://www.googleapis.com/auth/cloud-platform.read-only";
    #[doc = "Manage your Google API service configuration\n\n`https://www.googleapis.com/auth/service.management`"]
    pub const SERVICE_MANAGEMENT: &str = "https://www.googleapis.com/auth/service.management";
}
pub mod schemas {
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Api {
        #[doc = "The methods of this interface, in unspecified order."]
        #[serde(
            rename = "methods",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub methods: ::std::option::Option<Vec<crate::schemas::Method>>,
        #[doc = "Included interfaces. See Mixin."]
        #[serde(
            rename = "mixins",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mixins: ::std::option::Option<Vec<crate::schemas::Mixin>>,
        #[doc = "The fully qualified name of this interface, including package name\nfollowed by the interface's simple name."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Any metadata attached to the interface."]
        #[serde(
            rename = "options",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub options: ::std::option::Option<Vec<crate::schemas::Option>>,
        #[doc = "Source context for the protocol buffer service represented by this\nmessage."]
        #[serde(
            rename = "sourceContext",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source_context: ::std::option::Option<crate::schemas::SourceContext>,
        #[doc = "The source syntax of the service."]
        #[serde(
            rename = "syntax",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub syntax: ::std::option::Option<crate::schemas::ApiSyntax>,
        #[doc = "A version string for this interface. If specified, must have the form\n`major-version.minor-version`, as in `1.10`. If the minor version is\nomitted, it defaults to zero. If the entire version field is empty, the\nmajor version is derived from the package name, as outlined below. If the\nfield is not empty, the version in the package name will be verified to be\nconsistent with what is provided here.\n\nThe versioning schema uses [semantic\nversioning](http://semver.org) where the major version number\nindicates a breaking change and the minor version an additive,\nnon-breaking change. Both version numbers are signals to users\nwhat to expect from different versions, and should be carefully\nchosen based on the product plan.\n\nThe major version is also reflected in the package name of the\ninterface, which must end in `v<major-version>`, as in\n`google.feature.v1`. For major versions 0 and 1, the suffix can\nbe omitted. Zero major versions must only be used for\nexperimental, non-GA interfaces."]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Api {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Api {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ApiSyntax {
        #[doc = "Syntax `proto2`."]
        SyntaxProto2,
        #[doc = "Syntax `proto3`."]
        SyntaxProto3,
    }
    impl ApiSyntax {
        pub fn as_str(self) -> &'static str {
            match self {
                ApiSyntax::SyntaxProto2 => "SYNTAX_PROTO2",
                ApiSyntax::SyntaxProto3 => "SYNTAX_PROTO3",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ApiSyntax {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ApiSyntax {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ApiSyntax, ()> {
            Ok(match s {
                "SYNTAX_PROTO2" => ApiSyntax::SyntaxProto2,
                "SYNTAX_PROTO3" => ApiSyntax::SyntaxProto3,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ApiSyntax {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ApiSyntax {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ApiSyntax {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "SYNTAX_PROTO2" => ApiSyntax::SyntaxProto2,
                "SYNTAX_PROTO3" => ApiSyntax::SyntaxProto3,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ApiSyntax {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ApiSyntax {
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
    pub struct AuthProvider {
        #[doc = "The list of JWT\n[audiences](https://tools.ietf.org/html/draft-ietf-oauth-json-web-token-32#section-4.1.3).\nthat are allowed to access. A JWT containing any of these audiences will\nbe accepted. When this setting is absent, JWTs with audiences:\n\n* \"https://[service.name]/[google.protobuf.Api.name]\"\n* \"https://[service.name]/\"\n  will be accepted.\n  For example, if no audiences are in the setting, LibraryService API will\n  accept JWTs with the following audiences:\n* \n\nhttps://library-example.googleapis.com/google.example.library.v1.LibraryService\n\n* https://library-example.googleapis.com/\n\nExample:\n\n````text\naudiences: bookstore_android.apps.googleusercontent.com,\n           bookstore_web.apps.googleusercontent.com````"]
        #[serde(
            rename = "audiences",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub audiences: ::std::option::Option<String>,
        #[doc = "Redirect URL if JWT token is required but not present or is expired.\nImplement authorizationUrl of securityDefinitions in OpenAPI spec."]
        #[serde(
            rename = "authorizationUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub authorization_url: ::std::option::Option<String>,
        #[doc = "The unique identifier of the auth provider. It will be referred to by\n`AuthRequirement.provider_id`.\n\nExample: \"bookstore_auth\"."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "Identifies the principal that issued the JWT. See\nhttps://tools.ietf.org/html/draft-ietf-oauth-json-web-token-32#section-4.1.1\nUsually a URL or an email address.\n\nExample: https://securetoken.google.com\nExample: 1234567-compute@developer.gserviceaccount.com"]
        #[serde(
            rename = "issuer",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub issuer: ::std::option::Option<String>,
        #[doc = "URL of the provider's public key set to validate signature of the JWT. See\n[OpenID\nDiscovery](https://openid.net/specs/openid-connect-discovery-1_0.html#ProviderMetadata).\nOptional if the key set document:\n\n* can be retrieved from\n  [OpenID\n  Discovery](https://openid.net/specs/openid-connect-discovery-1_0.html of\n  the issuer.\n* can be inferred from the email domain of the issuer (e.g. a Google\n  service account).\n\nExample: https://www.googleapis.com/oauth2/v1/certs"]
        #[serde(
            rename = "jwksUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub jwks_uri: ::std::option::Option<String>,
        #[doc = "Defines the locations to extract the JWT.\n\nJWT locations can be either from HTTP headers or URL query parameters.\nThe rule is that the first match wins. The checking order is: checking\nall headers first, then URL query parameters.\n\nIf not specified,  default to use following 3 locations:\n\n1. Authorization: Bearer\n1. x-goog-iap-jwt-assertion\n1. access_token query parameter\n\nDefault locations can be specified as followings:\njwt_locations:\n\n* header: Authorization\n  value_prefix: \"Bearer \"\n* header: x-goog-iap-jwt-assertion\n* query: access_token"]
        #[serde(
            rename = "jwtLocations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub jwt_locations: ::std::option::Option<Vec<crate::schemas::JwtLocation>>,
    }
    impl ::google_field_selector::FieldSelector for AuthProvider {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AuthProvider {
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
    pub struct AuthRequirement {
        #[doc = "NOTE: This will be deprecated soon, once AuthProvider.audiences is\nimplemented and accepted in all the runtime components.\n\nThe list of JWT\n[audiences](https://tools.ietf.org/html/draft-ietf-oauth-json-web-token-32#section-4.1.3).\nthat are allowed to access. A JWT containing any of these audiences will\nbe accepted. When this setting is absent, only JWTs with audience\n\"https://Service_name/API_name\"\nwill be accepted. For example, if no audiences are in the setting,\nLibraryService API will only accept JWTs with the following audience\n\"https://library-example.googleapis.com/google.example.library.v1.LibraryService\".\n\nExample:\n\n````text\naudiences: bookstore_android.apps.googleusercontent.com,\n           bookstore_web.apps.googleusercontent.com````"]
        #[serde(
            rename = "audiences",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub audiences: ::std::option::Option<String>,
        #[doc = "id from authentication provider.\n\nExample:\n\n````text\nprovider_id: bookstore_auth````"]
        #[serde(
            rename = "providerId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub provider_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for AuthRequirement {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AuthRequirement {
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
    pub struct Authentication {
        #[doc = "Defines a set of authentication providers that a service supports."]
        #[serde(
            rename = "providers",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub providers: ::std::option::Option<Vec<crate::schemas::AuthProvider>>,
        #[doc = "A list of authentication rules that apply to individual API methods.\n\n**NOTE:** All service configuration rules follow \"last one wins\" order."]
        #[serde(
            rename = "rules",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rules: ::std::option::Option<Vec<crate::schemas::AuthenticationRule>>,
    }
    impl ::google_field_selector::FieldSelector for Authentication {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Authentication {
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
    pub struct AuthenticationRule {
        #[doc = "If true, the service accepts API keys without any other credential."]
        #[serde(
            rename = "allowWithoutCredential",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub allow_without_credential: ::std::option::Option<bool>,
        #[doc = "The requirements for OAuth credentials."]
        #[serde(
            rename = "oauth",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub oauth: ::std::option::Option<crate::schemas::OauthRequirements>,
        #[doc = "Requirements for additional authentication providers."]
        #[serde(
            rename = "requirements",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub requirements: ::std::option::Option<Vec<crate::schemas::AuthRequirement>>,
        #[doc = "Selects the methods to which this rule applies.\n\nRefer to selector for syntax details."]
        #[serde(
            rename = "selector",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub selector: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for AuthenticationRule {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AuthenticationRule {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Backend {
        #[doc = "A list of API backend rules that apply to individual API methods.\n\n**NOTE:** All service configuration rules follow \"last one wins\" order."]
        #[serde(
            rename = "rules",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rules: ::std::option::Option<Vec<crate::schemas::BackendRule>>,
    }
    impl ::google_field_selector::FieldSelector for Backend {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Backend {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct BackendRule {
        #[doc = "The address of the API backend.\n\nThe scheme is used to determine the backend protocol and security.\nThe following schemes are accepted:\n\nSCHEME        PROTOCOL    SECURITY\nhttp://       HTTP        None\nhttps://      HTTP        TLS\ngrpc://       gRPC        None\ngrpcs://      gRPC        TLS\n\nIt is recommended to explicitly include a scheme. Leaving out the scheme\nmay cause constrasting behaviors across platforms.\n\nIf the port is unspecified, the default is:\n\n* 80 for schemes without TLS\n* 443 for schemes with TLS\n\nFor HTTP backends, use protocol\nto specify the protocol version."]
        #[serde(
            rename = "address",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub address: ::std::option::Option<String>,
        #[doc = "The number of seconds to wait for a response from a request. The default\nvaries based on the request protocol and deployment environment."]
        #[serde(
            rename = "deadline",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub deadline: ::std::option::Option<f64>,
        #[doc = "When disable_auth is true, a JWT ID token won't be generated and the\noriginal \"Authorization\" HTTP header will be preserved. If the header is\nused to carry the original token and is expected by the backend, this\nfield must be set to true to preserve the header."]
        #[serde(
            rename = "disableAuth",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub disable_auth: ::std::option::Option<bool>,
        #[doc = "The JWT audience is used when generating a JWT ID token for the backend.\nThis ID token will be added in the HTTP \"authorization\" header, and sent\nto the backend."]
        #[serde(
            rename = "jwtAudience",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub jwt_audience: ::std::option::Option<String>,
        #[doc = "Minimum deadline in seconds needed for this method. Calls having deadline\nvalue lower than this will be rejected."]
        #[serde(
            rename = "minDeadline",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub min_deadline: ::std::option::Option<f64>,
        #[doc = "The number of seconds to wait for the completion of a long running\noperation. The default is no deadline."]
        #[serde(
            rename = "operationDeadline",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub operation_deadline: ::std::option::Option<f64>,
        #[serde(
            rename = "pathTranslation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub path_translation: ::std::option::Option<crate::schemas::BackendRulePathTranslation>,
        #[doc = "The protocol used for sending a request to the backend.\nThe supported values are \"http/1.1\" and \"h2\".\n\nThe default value is inferred from the scheme in the\naddress field:\n\nSCHEME        PROTOCOL\nhttp://       http/1.1\nhttps://      http/1.1\ngrpc://       h2\ngrpcs://      h2\n\nFor secure HTTP backends (https://) that support HTTP/2, set this field\nto \"h2\" for improved performance.\n\nConfiguring this field to non-default values is only supported for secure\nHTTP backends. This field will be ignored for all other backends.\n\nSee\nhttps://www.iana.org/assignments/tls-extensiontype-values/tls-extensiontype-values.xhtml#alpn-protocol-ids\nfor more details on the supported values."]
        #[serde(
            rename = "protocol",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub protocol: ::std::option::Option<String>,
        #[doc = "Selects the methods to which this rule applies.\n\nRefer to selector for syntax details."]
        #[serde(
            rename = "selector",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub selector: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for BackendRule {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BackendRule {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum BackendRulePathTranslation {
        #[doc = "The request path will be appended to the backend address.\n\n# Examples\n\nGiven the following operation config:\n\n````text\nMethod path:        /api/company/{cid}/user/{uid}\nBackend address:    https://example.appspot.com\n````\n\nRequests to the following request paths will call the backend at the\ntranslated path:\n\n````text\nRequest path: /api/company/widgetworks/user/johndoe\nTranslated:\nhttps://example.appspot.com/api/company/widgetworks/user/johndoe\n\nRequest path: /api/company/widgetworks/user/johndoe?timezone=EST\nTranslated:\nhttps://example.appspot.com/api/company/widgetworks/user/johndoe?timezone=EST````"]
        AppendPathToAddress,
        #[doc = "Use the backend address as-is, with no modification to the path. If the\nURL pattern contains variables, the variable names and values will be\nappended to the query string. If a query string parameter and a URL\npattern variable have the same name, this may result in duplicate keys in\nthe query string.\n\n# Examples\n\nGiven the following operation config:\n\n````text\nMethod path:        /api/company/{cid}/user/{uid}\nBackend address:    https://example.cloudfunctions.net/getUser\n````\n\nRequests to the following request paths will call the backend at the\ntranslated path:\n\n````text\nRequest path: /api/company/widgetworks/user/johndoe\nTranslated:\nhttps://example.cloudfunctions.net/getUser?cid=widgetworks&uid=johndoe\n\nRequest path: /api/company/widgetworks/user/johndoe?timezone=EST\nTranslated:\nhttps://example.cloudfunctions.net/getUser?timezone=EST&cid=widgetworks&uid=johndoe````"]
        ConstantAddress,
        PathTranslationUnspecified,
    }
    impl BackendRulePathTranslation {
        pub fn as_str(self) -> &'static str {
            match self {
                BackendRulePathTranslation::AppendPathToAddress => "APPEND_PATH_TO_ADDRESS",
                BackendRulePathTranslation::ConstantAddress => "CONSTANT_ADDRESS",
                BackendRulePathTranslation::PathTranslationUnspecified => {
                    "PATH_TRANSLATION_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for BackendRulePathTranslation {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for BackendRulePathTranslation {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<BackendRulePathTranslation, ()> {
            Ok(match s {
                "APPEND_PATH_TO_ADDRESS" => BackendRulePathTranslation::AppendPathToAddress,
                "CONSTANT_ADDRESS" => BackendRulePathTranslation::ConstantAddress,
                "PATH_TRANSLATION_UNSPECIFIED" => {
                    BackendRulePathTranslation::PathTranslationUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for BackendRulePathTranslation {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for BackendRulePathTranslation {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for BackendRulePathTranslation {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "APPEND_PATH_TO_ADDRESS" => BackendRulePathTranslation::AppendPathToAddress,
                "CONSTANT_ADDRESS" => BackendRulePathTranslation::ConstantAddress,
                "PATH_TRANSLATION_UNSPECIFIED" => {
                    BackendRulePathTranslation::PathTranslationUnspecified
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
    impl ::google_field_selector::FieldSelector for BackendRulePathTranslation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BackendRulePathTranslation {
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
    pub struct BatchCreateAdminOverridesResponse {
        #[doc = "The overrides that were created."]
        #[serde(
            rename = "overrides",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub overrides: ::std::option::Option<Vec<crate::schemas::QuotaOverride>>,
    }
    impl ::google_field_selector::FieldSelector for BatchCreateAdminOverridesResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BatchCreateAdminOverridesResponse {
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
    pub struct BatchCreateConsumerOverridesResponse {
        #[doc = "The overrides that were created."]
        #[serde(
            rename = "overrides",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub overrides: ::std::option::Option<Vec<crate::schemas::QuotaOverride>>,
    }
    impl ::google_field_selector::FieldSelector for BatchCreateConsumerOverridesResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BatchCreateConsumerOverridesResponse {
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
    pub struct BatchEnableServicesRequest {
        #[doc = "The identifiers of the services to enable on the project.\n\nA valid identifier would be:\nserviceusage.googleapis.com\n\nEnabling services requires that each service is public or is shared with\nthe user enabling the service.\n\nTwo or more services must be specified. To enable a single service,\nuse the `EnableService` method instead.\n\nA single request can enable a maximum of 20 services at a time. If more\nthan 20 services are specified, the request will fail, and no state changes\nwill occur."]
        #[serde(
            rename = "serviceIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub service_ids: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for BatchEnableServicesRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BatchEnableServicesRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct BatchEnableServicesResponse {
        #[doc = "If allow_partial_success is true, and one or more services could not be\nenabled, this field contains the details about each failure."]
        #[serde(
            rename = "failures",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub failures: ::std::option::Option<Vec<crate::schemas::EnableFailure>>,
        #[doc = "The new state of the services after enabling."]
        #[serde(
            rename = "services",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub services: ::std::option::Option<Vec<crate::schemas::GoogleApiServiceusageV1Service>>,
    }
    impl ::google_field_selector::FieldSelector for BatchEnableServicesResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BatchEnableServicesResponse {
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
    pub struct Billing {
        #[doc = "Billing configurations for sending metrics to the consumer project.\nThere can be multiple consumer destinations per service, each one must have\na different monitored resource type. A metric can be used in at most\none consumer destination."]
        #[serde(
            rename = "consumerDestinations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub consumer_destinations: ::std::option::Option<Vec<crate::schemas::BillingDestination>>,
    }
    impl ::google_field_selector::FieldSelector for Billing {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Billing {
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
    pub struct BillingDestination {
        #[doc = "Names of the metrics to report to this billing destination.\nEach name must be defined in Service.metrics section."]
        #[serde(
            rename = "metrics",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metrics: ::std::option::Option<Vec<String>>,
        #[doc = "The monitored resource type. The type must be defined in\nService.monitored_resources section."]
        #[serde(
            rename = "monitoredResource",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub monitored_resource: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for BillingDestination {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BillingDestination {
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
    pub struct ConsumerQuotaLimit {
        #[doc = "Whether admin overrides are allowed on this limit"]
        #[serde(
            rename = "allowsAdminOverrides",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub allows_admin_overrides: ::std::option::Option<bool>,
        #[doc = "Whether this limit is precise or imprecise."]
        #[serde(
            rename = "isPrecise",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub is_precise: ::std::option::Option<bool>,
        #[doc = "The name of the parent metric of this limit.\n\nAn example name would be:\n`compute.googleapis.com/cpus`"]
        #[serde(
            rename = "metric",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metric: ::std::option::Option<String>,
        #[doc = "The resource name of the quota limit.\n\nAn example name would be:\n`projects/123/services/compute.googleapis.com/consumerQuotaMetrics/compute.googleapis.com%2Fcpus/limits/%2Fproject%2Fregion`\n\nThe resource name is intended to be opaque and should not be parsed for\nits component strings, since its representation could change in the future."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Summary of the enforced quota buckets, organized by quota dimension,\nordered from least specific to most specific (for example, the global\ndefault bucket, with no quota dimensions, will always appear first)."]
        #[serde(
            rename = "quotaBuckets",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub quota_buckets: ::std::option::Option<Vec<crate::schemas::QuotaBucket>>,
        #[doc = "The limit unit.\n\nAn example unit would be\n`1/{project}/{region}`\nNote that `{project}` and `{region}` are not placeholders in this example;\nthe literal characters `{` and `}` occur in the string."]
        #[serde(
            rename = "unit",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub unit: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ConsumerQuotaLimit {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ConsumerQuotaLimit {
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
    pub struct ConsumerQuotaMetric {
        #[doc = "The consumer quota for each quota limit defined on the metric."]
        #[serde(
            rename = "consumerQuotaLimits",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub consumer_quota_limits: ::std::option::Option<Vec<crate::schemas::ConsumerQuotaLimit>>,
        #[doc = "The display name of the metric.\n\nAn example name would be:\n\"CPUs\""]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "The name of the metric.\n\nAn example name would be:\n`compute.googleapis.com/cpus`"]
        #[serde(
            rename = "metric",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metric: ::std::option::Option<String>,
        #[doc = "The resource name of the quota settings on this metric for this consumer.\n\nAn example name would be:\n`projects/123/services/compute.googleapis.com/consumerQuotaMetrics/compute.googleapis.com%2Fcpus\n\nThe resource name is intended to be opaque and should not be parsed for\nits component strings, since its representation could change in the future."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ConsumerQuotaMetric {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ConsumerQuotaMetric {
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
    pub struct Context {
        #[doc = "A list of RPC context rules that apply to individual API methods.\n\n**NOTE:** All service configuration rules follow \"last one wins\" order."]
        #[serde(
            rename = "rules",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rules: ::std::option::Option<Vec<crate::schemas::ContextRule>>,
    }
    impl ::google_field_selector::FieldSelector for Context {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Context {
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
    pub struct ContextRule {
        #[doc = "A list of full type names or extension IDs of extensions allowed in grpc\nside channel from client to backend."]
        #[serde(
            rename = "allowedRequestExtensions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub allowed_request_extensions: ::std::option::Option<Vec<String>>,
        #[doc = "A list of full type names or extension IDs of extensions allowed in grpc\nside channel from backend to client."]
        #[serde(
            rename = "allowedResponseExtensions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub allowed_response_extensions: ::std::option::Option<Vec<String>>,
        #[doc = "A list of full type names of provided contexts."]
        #[serde(
            rename = "provided",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub provided: ::std::option::Option<Vec<String>>,
        #[doc = "A list of full type names of requested contexts."]
        #[serde(
            rename = "requested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub requested: ::std::option::Option<Vec<String>>,
        #[doc = "Selects the methods to which this rule applies.\n\nRefer to selector for syntax details."]
        #[serde(
            rename = "selector",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub selector: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ContextRule {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ContextRule {
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
    pub struct Control {
        #[doc = "The service control environment to use. If empty, no control plane\nfeature (like quota and billing) will be enabled."]
        #[serde(
            rename = "environment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub environment: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Control {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Control {
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
    pub struct CustomError {
        #[doc = "The list of custom error rules that apply to individual API messages.\n\n**NOTE:** All service configuration rules follow \"last one wins\" order."]
        #[serde(
            rename = "rules",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rules: ::std::option::Option<Vec<crate::schemas::CustomErrorRule>>,
        #[doc = "The list of custom error detail types, e.g. 'google.foo.v1.CustomError'."]
        #[serde(
            rename = "types",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub types: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for CustomError {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CustomError {
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
    pub struct CustomErrorRule {
        #[doc = "Mark this message as possible payload in error response.  Otherwise,\nobjects of this type will be filtered when they appear in error payload."]
        #[serde(
            rename = "isErrorType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub is_error_type: ::std::option::Option<bool>,
        #[doc = "Selects messages to which this rule applies.\n\nRefer to selector for syntax details."]
        #[serde(
            rename = "selector",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub selector: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for CustomErrorRule {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CustomErrorRule {
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
    pub struct CustomHttpPattern {
        #[doc = "The name of this custom HTTP verb."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "The path matched by this custom verb."]
        #[serde(
            rename = "path",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub path: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for CustomHttpPattern {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CustomHttpPattern {
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
    pub struct DisableServiceRequest {}
    impl ::google_field_selector::FieldSelector for DisableServiceRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DisableServiceRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct DisableServiceResponse {
        #[doc = "The new state of the service after disabling."]
        #[serde(
            rename = "service",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub service: ::std::option::Option<crate::schemas::GoogleApiServiceusageV1Service>,
    }
    impl ::google_field_selector::FieldSelector for DisableServiceResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DisableServiceResponse {
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
    pub struct Documentation {
        #[doc = "The URL to the root of documentation."]
        #[serde(
            rename = "documentationRootUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub documentation_root_url: ::std::option::Option<String>,
        #[doc = "Declares a single overview page. For example:\n\n<pre><code>documentation:\n  summary: ...\n  overview: &#40;== include overview.md ==&#41;\n</code></pre>\n\nThis is a shortcut for the following declaration (using pages style):\n\n<pre><code>documentation:\n  summary: ...\n  pages:\n  - name: Overview\n    content: &#40;== include overview.md ==&#41;\n</code></pre>\n\nNote: you cannot specify both `overview` field and `pages` field."]
        #[serde(
            rename = "overview",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub overview: ::std::option::Option<String>,
        #[doc = "The top level pages for the documentation set."]
        #[serde(
            rename = "pages",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub pages: ::std::option::Option<Vec<crate::schemas::Page>>,
        #[doc = "A list of documentation rules that apply to individual API elements.\n\n**NOTE:** All service configuration rules follow \"last one wins\" order."]
        #[serde(
            rename = "rules",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rules: ::std::option::Option<Vec<crate::schemas::DocumentationRule>>,
        #[doc = "Specifies the service root url if the default one (the service name\nfrom the yaml file) is not suitable. This can be seen in any fully\nspecified service urls as well as sections that show a base that other\nurls are relative to."]
        #[serde(
            rename = "serviceRootUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub service_root_url: ::std::option::Option<String>,
        #[doc = "A short summary of what the service does. Can only be provided by\nplain text."]
        #[serde(
            rename = "summary",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub summary: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Documentation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Documentation {
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
    pub struct DocumentationRule {
        #[doc = "Deprecation description of the selected element(s). It can be provided if\nan element is marked as `deprecated`."]
        #[serde(
            rename = "deprecationDescription",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub deprecation_description: ::std::option::Option<String>,
        #[doc = "Description of the selected API(s)."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "The selector is a comma-separated list of patterns. Each pattern is a\nqualified name of the element which may end in \"*\", indicating a wildcard.\nWildcards are only allowed at the end and for a whole component of the\nqualified name, i.e. \"foo.*\" is ok, but not \"foo.b*\" or \"foo.*.bar\". A\nwildcard will match one or more components. To specify a default for all\napplicable elements, the whole pattern \"*\" is used."]
        #[serde(
            rename = "selector",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub selector: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for DocumentationRule {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DocumentationRule {
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
    pub struct Empty {}
    impl ::google_field_selector::FieldSelector for Empty {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Empty {
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
    pub struct EnableFailure {
        #[doc = "An error message describing why the service could not be enabled."]
        #[serde(
            rename = "errorMessage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub error_message: ::std::option::Option<String>,
        #[doc = "The service id of a service that could not be enabled."]
        #[serde(
            rename = "serviceId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub service_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for EnableFailure {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for EnableFailure {
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
    pub struct EnableServiceRequest {}
    impl ::google_field_selector::FieldSelector for EnableServiceRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for EnableServiceRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct EnableServiceResponse {
        #[doc = "The new state of the service after enabling."]
        #[serde(
            rename = "service",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub service: ::std::option::Option<crate::schemas::GoogleApiServiceusageV1Service>,
    }
    impl ::google_field_selector::FieldSelector for EnableServiceResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for EnableServiceResponse {
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
    pub struct Endpoint {
        #[doc = "DEPRECATED: This field is no longer supported. Instead of using aliases,\nplease specify multiple google.api.Endpoint for each of the intended\naliases.\n\nAdditional names that this endpoint will be hosted on."]
        #[serde(
            rename = "aliases",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub aliases: ::std::option::Option<Vec<String>>,
        #[doc = "Allowing\n[CORS](https://en.wikipedia.org/wiki/Cross-origin_resource_sharing), aka\ncross-domain traffic, would allow the backends served from this endpoint to\nreceive and respond to HTTP OPTIONS requests. The response will be used by\nthe browser to determine whether the subsequent cross-origin request is\nallowed to proceed."]
        #[serde(
            rename = "allowCors",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub allow_cors: ::std::option::Option<bool>,
        #[doc = "The list of features enabled on this endpoint."]
        #[serde(
            rename = "features",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub features: ::std::option::Option<Vec<String>>,
        #[doc = "The canonical name of this endpoint."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The specification of an Internet routable address of API frontend that will\nhandle requests to this [API\nEndpoint](https://cloud.google.com/apis/design/glossary). It should be\neither a valid IPv4 address or a fully-qualified domain name. For example,\n\"8.8.8.8\" or \"myservice.appspot.com\"."]
        #[serde(
            rename = "target",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub target: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Endpoint {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Endpoint {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Enum {
        #[doc = "Enum value definitions."]
        #[serde(
            rename = "enumvalue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub enumvalue: ::std::option::Option<Vec<crate::schemas::EnumValue>>,
        #[doc = "Enum type name."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Protocol buffer options."]
        #[serde(
            rename = "options",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub options: ::std::option::Option<Vec<crate::schemas::Option>>,
        #[doc = "The source context."]
        #[serde(
            rename = "sourceContext",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source_context: ::std::option::Option<crate::schemas::SourceContext>,
        #[doc = "The source syntax."]
        #[serde(
            rename = "syntax",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub syntax: ::std::option::Option<crate::schemas::EnumSyntax>,
    }
    impl ::google_field_selector::FieldSelector for Enum {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Enum {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum EnumSyntax {
        #[doc = "Syntax `proto2`."]
        SyntaxProto2,
        #[doc = "Syntax `proto3`."]
        SyntaxProto3,
    }
    impl EnumSyntax {
        pub fn as_str(self) -> &'static str {
            match self {
                EnumSyntax::SyntaxProto2 => "SYNTAX_PROTO2",
                EnumSyntax::SyntaxProto3 => "SYNTAX_PROTO3",
            }
        }
    }
    impl ::std::convert::AsRef<str> for EnumSyntax {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for EnumSyntax {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<EnumSyntax, ()> {
            Ok(match s {
                "SYNTAX_PROTO2" => EnumSyntax::SyntaxProto2,
                "SYNTAX_PROTO3" => EnumSyntax::SyntaxProto3,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for EnumSyntax {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for EnumSyntax {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for EnumSyntax {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "SYNTAX_PROTO2" => EnumSyntax::SyntaxProto2,
                "SYNTAX_PROTO3" => EnumSyntax::SyntaxProto3,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for EnumSyntax {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for EnumSyntax {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct EnumValue {
        #[doc = "Enum value name."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Enum value number."]
        #[serde(
            rename = "number",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub number: ::std::option::Option<i32>,
        #[doc = "Protocol buffer options."]
        #[serde(
            rename = "options",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub options: ::std::option::Option<Vec<crate::schemas::Option>>,
    }
    impl ::google_field_selector::FieldSelector for EnumValue {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for EnumValue {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Field {
        #[doc = "The field cardinality."]
        #[serde(
            rename = "cardinality",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cardinality: ::std::option::Option<crate::schemas::FieldCardinality>,
        #[doc = "The string value of the default value of this field. Proto2 syntax only."]
        #[serde(
            rename = "defaultValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub default_value: ::std::option::Option<String>,
        #[doc = "The field JSON name."]
        #[serde(
            rename = "jsonName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub json_name: ::std::option::Option<String>,
        #[doc = "The field type."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<crate::schemas::FieldKind>,
        #[doc = "The field name."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The field number."]
        #[serde(
            rename = "number",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub number: ::std::option::Option<i32>,
        #[doc = "The index of the field type in `Type.oneofs`, for message or enumeration\ntypes. The first type has index 1; zero means the type is not in the list."]
        #[serde(
            rename = "oneofIndex",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub oneof_index: ::std::option::Option<i32>,
        #[doc = "The protocol buffer options."]
        #[serde(
            rename = "options",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub options: ::std::option::Option<Vec<crate::schemas::Option>>,
        #[doc = "Whether to use alternative packed wire representation."]
        #[serde(
            rename = "packed",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub packed: ::std::option::Option<bool>,
        #[doc = "The field type URL, without the scheme, for message or enumeration\ntypes. Example: `\"type.googleapis.com/google.protobuf.Timestamp\"`."]
        #[serde(
            rename = "typeUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub type_url: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Field {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Field {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum FieldCardinality {
        #[doc = "For optional fields."]
        CardinalityOptional,
        #[doc = "For repeated fields."]
        CardinalityRepeated,
        #[doc = "For required fields. Proto2 syntax only."]
        CardinalityRequired,
        #[doc = "For fields with unknown cardinality."]
        CardinalityUnknown,
    }
    impl FieldCardinality {
        pub fn as_str(self) -> &'static str {
            match self {
                FieldCardinality::CardinalityOptional => "CARDINALITY_OPTIONAL",
                FieldCardinality::CardinalityRepeated => "CARDINALITY_REPEATED",
                FieldCardinality::CardinalityRequired => "CARDINALITY_REQUIRED",
                FieldCardinality::CardinalityUnknown => "CARDINALITY_UNKNOWN",
            }
        }
    }
    impl ::std::convert::AsRef<str> for FieldCardinality {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for FieldCardinality {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<FieldCardinality, ()> {
            Ok(match s {
                "CARDINALITY_OPTIONAL" => FieldCardinality::CardinalityOptional,
                "CARDINALITY_REPEATED" => FieldCardinality::CardinalityRepeated,
                "CARDINALITY_REQUIRED" => FieldCardinality::CardinalityRequired,
                "CARDINALITY_UNKNOWN" => FieldCardinality::CardinalityUnknown,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for FieldCardinality {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for FieldCardinality {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for FieldCardinality {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CARDINALITY_OPTIONAL" => FieldCardinality::CardinalityOptional,
                "CARDINALITY_REPEATED" => FieldCardinality::CardinalityRepeated,
                "CARDINALITY_REQUIRED" => FieldCardinality::CardinalityRequired,
                "CARDINALITY_UNKNOWN" => FieldCardinality::CardinalityUnknown,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for FieldCardinality {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for FieldCardinality {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum FieldKind {
        #[doc = "Field type bool."]
        TypeBool,
        #[doc = "Field type bytes."]
        TypeBytes,
        #[doc = "Field type double."]
        TypeDouble,
        #[doc = "Field type enum."]
        TypeEnum,
        #[doc = "Field type fixed32."]
        TypeFixed32,
        #[doc = "Field type fixed64."]
        TypeFixed64,
        #[doc = "Field type float."]
        TypeFloat,
        #[doc = "Field type group. Proto2 syntax only, and deprecated."]
        TypeGroup,
        #[doc = "Field type int32."]
        TypeInt32,
        #[doc = "Field type int64."]
        TypeInt64,
        #[doc = "Field type message."]
        TypeMessage,
        #[doc = "Field type sfixed32."]
        TypeSfixed32,
        #[doc = "Field type sfixed64."]
        TypeSfixed64,
        #[doc = "Field type sint32."]
        TypeSint32,
        #[doc = "Field type sint64."]
        TypeSint64,
        #[doc = "Field type string."]
        TypeString,
        #[doc = "Field type uint32."]
        TypeUint32,
        #[doc = "Field type uint64."]
        TypeUint64,
        #[doc = "Field type unknown."]
        TypeUnknown,
    }
    impl FieldKind {
        pub fn as_str(self) -> &'static str {
            match self {
                FieldKind::TypeBool => "TYPE_BOOL",
                FieldKind::TypeBytes => "TYPE_BYTES",
                FieldKind::TypeDouble => "TYPE_DOUBLE",
                FieldKind::TypeEnum => "TYPE_ENUM",
                FieldKind::TypeFixed32 => "TYPE_FIXED32",
                FieldKind::TypeFixed64 => "TYPE_FIXED64",
                FieldKind::TypeFloat => "TYPE_FLOAT",
                FieldKind::TypeGroup => "TYPE_GROUP",
                FieldKind::TypeInt32 => "TYPE_INT32",
                FieldKind::TypeInt64 => "TYPE_INT64",
                FieldKind::TypeMessage => "TYPE_MESSAGE",
                FieldKind::TypeSfixed32 => "TYPE_SFIXED32",
                FieldKind::TypeSfixed64 => "TYPE_SFIXED64",
                FieldKind::TypeSint32 => "TYPE_SINT32",
                FieldKind::TypeSint64 => "TYPE_SINT64",
                FieldKind::TypeString => "TYPE_STRING",
                FieldKind::TypeUint32 => "TYPE_UINT32",
                FieldKind::TypeUint64 => "TYPE_UINT64",
                FieldKind::TypeUnknown => "TYPE_UNKNOWN",
            }
        }
    }
    impl ::std::convert::AsRef<str> for FieldKind {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for FieldKind {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<FieldKind, ()> {
            Ok(match s {
                "TYPE_BOOL" => FieldKind::TypeBool,
                "TYPE_BYTES" => FieldKind::TypeBytes,
                "TYPE_DOUBLE" => FieldKind::TypeDouble,
                "TYPE_ENUM" => FieldKind::TypeEnum,
                "TYPE_FIXED32" => FieldKind::TypeFixed32,
                "TYPE_FIXED64" => FieldKind::TypeFixed64,
                "TYPE_FLOAT" => FieldKind::TypeFloat,
                "TYPE_GROUP" => FieldKind::TypeGroup,
                "TYPE_INT32" => FieldKind::TypeInt32,
                "TYPE_INT64" => FieldKind::TypeInt64,
                "TYPE_MESSAGE" => FieldKind::TypeMessage,
                "TYPE_SFIXED32" => FieldKind::TypeSfixed32,
                "TYPE_SFIXED64" => FieldKind::TypeSfixed64,
                "TYPE_SINT32" => FieldKind::TypeSint32,
                "TYPE_SINT64" => FieldKind::TypeSint64,
                "TYPE_STRING" => FieldKind::TypeString,
                "TYPE_UINT32" => FieldKind::TypeUint32,
                "TYPE_UINT64" => FieldKind::TypeUint64,
                "TYPE_UNKNOWN" => FieldKind::TypeUnknown,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for FieldKind {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for FieldKind {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for FieldKind {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "TYPE_BOOL" => FieldKind::TypeBool,
                "TYPE_BYTES" => FieldKind::TypeBytes,
                "TYPE_DOUBLE" => FieldKind::TypeDouble,
                "TYPE_ENUM" => FieldKind::TypeEnum,
                "TYPE_FIXED32" => FieldKind::TypeFixed32,
                "TYPE_FIXED64" => FieldKind::TypeFixed64,
                "TYPE_FLOAT" => FieldKind::TypeFloat,
                "TYPE_GROUP" => FieldKind::TypeGroup,
                "TYPE_INT32" => FieldKind::TypeInt32,
                "TYPE_INT64" => FieldKind::TypeInt64,
                "TYPE_MESSAGE" => FieldKind::TypeMessage,
                "TYPE_SFIXED32" => FieldKind::TypeSfixed32,
                "TYPE_SFIXED64" => FieldKind::TypeSfixed64,
                "TYPE_SINT32" => FieldKind::TypeSint32,
                "TYPE_SINT64" => FieldKind::TypeSint64,
                "TYPE_STRING" => FieldKind::TypeString,
                "TYPE_UINT32" => FieldKind::TypeUint32,
                "TYPE_UINT64" => FieldKind::TypeUint64,
                "TYPE_UNKNOWN" => FieldKind::TypeUnknown,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for FieldKind {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for FieldKind {
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
    pub struct GetServiceIdentityResponse {
        #[doc = "Service identity that service producer can use to access consumer\nresources. If exists is true, it contains email and unique_id. If exists is\nfalse, it contains pre-constructed email and empty unique_id."]
        #[serde(
            rename = "identity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub identity: ::std::option::Option<crate::schemas::ServiceIdentity>,
        #[doc = "Service identity state."]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state: ::std::option::Option<crate::schemas::GetServiceIdentityResponseState>,
    }
    impl ::google_field_selector::FieldSelector for GetServiceIdentityResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GetServiceIdentityResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GetServiceIdentityResponseState {
        #[doc = "Service identity has been created and can be used."]
        Active,
        #[doc = "Default service identity state. This value is used if the state is\nomitted."]
        IdentityStateUnspecified,
    }
    impl GetServiceIdentityResponseState {
        pub fn as_str(self) -> &'static str {
            match self {
                GetServiceIdentityResponseState::Active => "ACTIVE",
                GetServiceIdentityResponseState::IdentityStateUnspecified => {
                    "IDENTITY_STATE_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for GetServiceIdentityResponseState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GetServiceIdentityResponseState {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<GetServiceIdentityResponseState, ()> {
            Ok(match s {
                "ACTIVE" => GetServiceIdentityResponseState::Active,
                "IDENTITY_STATE_UNSPECIFIED" => {
                    GetServiceIdentityResponseState::IdentityStateUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GetServiceIdentityResponseState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GetServiceIdentityResponseState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GetServiceIdentityResponseState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ACTIVE" => GetServiceIdentityResponseState::Active,
                "IDENTITY_STATE_UNSPECIFIED" => {
                    GetServiceIdentityResponseState::IdentityStateUnspecified
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
    impl ::google_field_selector::FieldSelector for GetServiceIdentityResponseState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GetServiceIdentityResponseState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleApiService {
        #[doc = "A list of API interfaces exported by this service. Only the `name` field\nof the google.protobuf.Api needs to be provided by the configuration\nauthor, as the remaining fields will be derived from the IDL during the\nnormalization process. It is an error to specify an API interface here\nwhich cannot be resolved against the associated IDL files."]
        #[serde(
            rename = "apis",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub apis: ::std::option::Option<Vec<crate::schemas::Api>>,
        #[doc = "Auth configuration."]
        #[serde(
            rename = "authentication",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub authentication: ::std::option::Option<crate::schemas::Authentication>,
        #[doc = "API backend configuration."]
        #[serde(
            rename = "backend",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub backend: ::std::option::Option<crate::schemas::Backend>,
        #[doc = "Billing configuration."]
        #[serde(
            rename = "billing",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub billing: ::std::option::Option<crate::schemas::Billing>,
        #[doc = "The semantic version of the service configuration. The config version\naffects the interpretation of the service configuration. For example,\ncertain features are enabled by default for certain config versions.\n\nThe latest config version is `3`."]
        #[serde(
            rename = "configVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub config_version: ::std::option::Option<u32>,
        #[doc = "Context configuration."]
        #[serde(
            rename = "context",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub context: ::std::option::Option<crate::schemas::Context>,
        #[doc = "Configuration for the service control plane."]
        #[serde(
            rename = "control",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub control: ::std::option::Option<crate::schemas::Control>,
        #[doc = "Custom error configuration."]
        #[serde(
            rename = "customError",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub custom_error: ::std::option::Option<crate::schemas::CustomError>,
        #[doc = "Additional API documentation."]
        #[serde(
            rename = "documentation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub documentation: ::std::option::Option<crate::schemas::Documentation>,
        #[doc = "Configuration for network endpoints.  If this is empty, then an endpoint\nwith the same name as the service is automatically generated to service all\ndefined APIs."]
        #[serde(
            rename = "endpoints",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub endpoints: ::std::option::Option<Vec<crate::schemas::Endpoint>>,
        #[doc = "A list of all enum types included in this API service.  Enums\nreferenced directly or indirectly by the `apis` are automatically\nincluded.  Enums which are not referenced but shall be included\nshould be listed here by name. Example:\n\n````text\nenums:\n- name: google.someapi.v1.SomeEnum````"]
        #[serde(
            rename = "enums",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub enums: ::std::option::Option<Vec<crate::schemas::Enum>>,
        #[doc = "HTTP configuration."]
        #[serde(
            rename = "http",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub http: ::std::option::Option<crate::schemas::Http>,
        #[doc = "A unique ID for a specific instance of this message, typically assigned\nby the client for tracking purpose. Must be no longer than 63 characters\nand only lower case letters, digits, '.', '_' and '-' are allowed. If\nempty, the server may choose to generate one instead."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "Logging configuration."]
        #[serde(
            rename = "logging",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub logging: ::std::option::Option<crate::schemas::Logging>,
        #[doc = "Defines the logs used by this service."]
        #[serde(
            rename = "logs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub logs: ::std::option::Option<Vec<crate::schemas::LogDescriptor>>,
        #[doc = "Defines the metrics used by this service."]
        #[serde(
            rename = "metrics",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metrics: ::std::option::Option<Vec<crate::schemas::MetricDescriptor>>,
        #[doc = "Defines the monitored resources used by this service. This is required\nby the Service.monitoring and Service.logging configurations."]
        #[serde(
            rename = "monitoredResources",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub monitored_resources:
            ::std::option::Option<Vec<crate::schemas::MonitoredResourceDescriptor>>,
        #[doc = "Monitoring configuration."]
        #[serde(
            rename = "monitoring",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub monitoring: ::std::option::Option<crate::schemas::Monitoring>,
        #[doc = "The service name, which is a DNS-like logical identifier for the\nservice, such as `calendar.googleapis.com`. The service name\ntypically goes through DNS verification to make sure the owner\nof the service also owns the DNS name."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The Google project that owns this service."]
        #[serde(
            rename = "producerProjectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub producer_project_id: ::std::option::Option<String>,
        #[doc = "Quota configuration."]
        #[serde(
            rename = "quota",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub quota: ::std::option::Option<crate::schemas::Quota>,
        #[doc = "Output only. The source information for this configuration if available."]
        #[serde(
            rename = "sourceInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source_info: ::std::option::Option<crate::schemas::SourceInfo>,
        #[doc = "System parameter configuration."]
        #[serde(
            rename = "systemParameters",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub system_parameters: ::std::option::Option<crate::schemas::SystemParameters>,
        #[doc = "A list of all proto message types included in this API service.\nIt serves similar purpose as [google.api.Service.types], except that\nthese types are not needed by user-defined APIs. Therefore, they will not\nshow up in the generated discovery doc. This field should only be used\nto define system APIs in ESF."]
        #[serde(
            rename = "systemTypes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub system_types: ::std::option::Option<Vec<crate::schemas::Type>>,
        #[doc = "The product title for this service."]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
        #[doc = "A list of all proto message types included in this API service.\nTypes referenced directly or indirectly by the `apis` are\nautomatically included.  Messages which are not referenced but\nshall be included, such as types used by the `google.protobuf.Any` type,\nshould be listed here by name. Example:\n\n````text\ntypes:\n- name: google.protobuf.Int32````"]
        #[serde(
            rename = "types",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub types: ::std::option::Option<Vec<crate::schemas::Type>>,
        #[doc = "Configuration controlling usage of this service."]
        #[serde(
            rename = "usage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub usage: ::std::option::Option<crate::schemas::Usage>,
    }
    impl ::google_field_selector::FieldSelector for GoogleApiService {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleApiService {
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
    pub struct GoogleApiServiceIdentity {
        #[doc = "Optional. A user-specified opaque description of the service account.\nMust be less than or equal to 256 UTF-8 bytes."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "Optional. A user-specified name for the service account.\nMust be less than or equal to 100 UTF-8 bytes."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "A service account project that hosts the service accounts.\n\nAn example name would be:\n`projects/123456789`"]
        #[serde(
            rename = "serviceAccountParent",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub service_account_parent: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleApiServiceIdentity {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleApiServiceIdentity {
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
    pub struct GoogleApiServiceusageV1Beta1GetServiceIdentityResponse {
        #[doc = "Service identity that service producer can use to access consumer\nresources. If exists is true, it contains email and unique_id. If exists is\nfalse, it contains pre-constructed email and empty unique_id."]
        #[serde(
            rename = "identity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub identity:
            ::std::option::Option<crate::schemas::GoogleApiServiceusageV1Beta1ServiceIdentity>,
        #[doc = "Service identity state."]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state: ::std::option::Option<
            crate::schemas::GoogleApiServiceusageV1Beta1GetServiceIdentityResponseState,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleApiServiceusageV1Beta1GetServiceIdentityResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleApiServiceusageV1Beta1GetServiceIdentityResponse
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleApiServiceusageV1Beta1GetServiceIdentityResponseState {
        #[doc = "Service identity has been created and can be used."]
        Active,
        #[doc = "Default service identity state. This value is used if the state is\nomitted."]
        IdentityStateUnspecified,
    }
    impl GoogleApiServiceusageV1Beta1GetServiceIdentityResponseState {
        pub fn as_str(self) -> &'static str {
            match self { GoogleApiServiceusageV1Beta1GetServiceIdentityResponseState :: Active => "ACTIVE" , GoogleApiServiceusageV1Beta1GetServiceIdentityResponseState :: IdentityStateUnspecified => "IDENTITY_STATE_UNSPECIFIED" , }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleApiServiceusageV1Beta1GetServiceIdentityResponseState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleApiServiceusageV1Beta1GetServiceIdentityResponseState {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleApiServiceusageV1Beta1GetServiceIdentityResponseState, ()>
        {
            Ok ( match s { "ACTIVE" => GoogleApiServiceusageV1Beta1GetServiceIdentityResponseState :: Active , "IDENTITY_STATE_UNSPECIFIED" => GoogleApiServiceusageV1Beta1GetServiceIdentityResponseState :: IdentityStateUnspecified , _ => return Err ( ( ) ) , } )
        }
    }
    impl ::std::fmt::Display for GoogleApiServiceusageV1Beta1GetServiceIdentityResponseState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleApiServiceusageV1Beta1GetServiceIdentityResponseState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleApiServiceusageV1Beta1GetServiceIdentityResponseState
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok ( match value { "ACTIVE" => GoogleApiServiceusageV1Beta1GetServiceIdentityResponseState :: Active , "IDENTITY_STATE_UNSPECIFIED" => GoogleApiServiceusageV1Beta1GetServiceIdentityResponseState :: IdentityStateUnspecified , _ => return Err ( :: serde :: de :: Error :: custom ( format ! ( "invalid enum for #name: {}" , value ) ) ) , } )
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleApiServiceusageV1Beta1GetServiceIdentityResponseState
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleApiServiceusageV1Beta1GetServiceIdentityResponseState
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
    pub struct GoogleApiServiceusageV1Beta1ServiceIdentity {
        #[doc = "The email address of the service account that a service producer would use\nto access consumer resources."]
        #[serde(
            rename = "email",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub email: ::std::option::Option<String>,
        #[doc = "The unique and stable id of the service account.\nhttps://cloud.google.com/iam/reference/rest/v1/projects.serviceAccounts#ServiceAccount"]
        #[serde(
            rename = "uniqueId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub unique_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleApiServiceusageV1Beta1ServiceIdentity {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleApiServiceusageV1Beta1ServiceIdentity {
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
    pub struct GoogleApiServiceusageV1OperationMetadata {
        #[doc = "The full name of the resources that this operation is directly\nassociated with."]
        #[serde(
            rename = "resourceNames",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_names: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleApiServiceusageV1OperationMetadata {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleApiServiceusageV1OperationMetadata {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleApiServiceusageV1Service {
        #[doc = "The service configuration of the available service.\nSome fields may be filtered out of the configuration in responses to\nthe `ListServices` method. These fields are present only in responses to\nthe `GetService` method."]
        #[serde(
            rename = "config",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub config: ::std::option::Option<crate::schemas::GoogleApiServiceusageV1ServiceConfig>,
        #[doc = "The resource name of the consumer and service.\n\nA valid name would be:\n\n* projects/123/services/serviceusage.googleapis.com"]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The resource name of the consumer.\n\nA valid name would be:\n\n* projects/123"]
        #[serde(
            rename = "parent",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub parent: ::std::option::Option<String>,
        #[doc = "Whether or not the service has been enabled for use by the consumer."]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state: ::std::option::Option<crate::schemas::GoogleApiServiceusageV1ServiceState>,
    }
    impl ::google_field_selector::FieldSelector for GoogleApiServiceusageV1Service {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleApiServiceusageV1Service {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleApiServiceusageV1ServiceState {
        #[doc = "The service cannot be used by this consumer. It has either been explicitly\ndisabled, or has never been enabled."]
        Disabled,
        #[doc = "The service has been explicitly enabled for use by this consumer."]
        Enabled,
        #[doc = "The default value, which indicates that the enabled state of the service\nis unspecified or not meaningful. Currently, all consumers other than\nprojects (such as folders and organizations) are always in this state."]
        StateUnspecified,
    }
    impl GoogleApiServiceusageV1ServiceState {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleApiServiceusageV1ServiceState::Disabled => "DISABLED",
                GoogleApiServiceusageV1ServiceState::Enabled => "ENABLED",
                GoogleApiServiceusageV1ServiceState::StateUnspecified => "STATE_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleApiServiceusageV1ServiceState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleApiServiceusageV1ServiceState {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<GoogleApiServiceusageV1ServiceState, ()> {
            Ok(match s {
                "DISABLED" => GoogleApiServiceusageV1ServiceState::Disabled,
                "ENABLED" => GoogleApiServiceusageV1ServiceState::Enabled,
                "STATE_UNSPECIFIED" => GoogleApiServiceusageV1ServiceState::StateUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleApiServiceusageV1ServiceState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleApiServiceusageV1ServiceState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleApiServiceusageV1ServiceState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DISABLED" => GoogleApiServiceusageV1ServiceState::Disabled,
                "ENABLED" => GoogleApiServiceusageV1ServiceState::Enabled,
                "STATE_UNSPECIFIED" => GoogleApiServiceusageV1ServiceState::StateUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleApiServiceusageV1ServiceState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleApiServiceusageV1ServiceState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleApiServiceusageV1ServiceConfig {
        #[doc = "A list of API interfaces exported by this service. Contains only the names,\nversions, and method names of the interfaces."]
        #[serde(
            rename = "apis",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub apis: ::std::option::Option<Vec<crate::schemas::Api>>,
        #[doc = "Auth configuration. Contains only the OAuth rules."]
        #[serde(
            rename = "authentication",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub authentication: ::std::option::Option<crate::schemas::Authentication>,
        #[doc = "Additional API documentation. Contains only the summary and the\ndocumentation URL."]
        #[serde(
            rename = "documentation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub documentation: ::std::option::Option<crate::schemas::Documentation>,
        #[doc = "Configuration for network endpoints. Contains only the names and aliases\nof the endpoints."]
        #[serde(
            rename = "endpoints",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub endpoints: ::std::option::Option<Vec<crate::schemas::Endpoint>>,
        #[doc = "The DNS address at which this service is available.\n\nAn example DNS address would be:\n`calendar.googleapis.com`."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Quota configuration."]
        #[serde(
            rename = "quota",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub quota: ::std::option::Option<crate::schemas::Quota>,
        #[doc = "The product title for this service."]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
        #[doc = "Configuration controlling usage of this service."]
        #[serde(
            rename = "usage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub usage: ::std::option::Option<crate::schemas::Usage>,
    }
    impl ::google_field_selector::FieldSelector for GoogleApiServiceusageV1ServiceConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleApiServiceusageV1ServiceConfig {
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
    pub struct Http {
        #[doc = "When set to true, URL path parameters will be fully URI-decoded except in\ncases of single segment matches in reserved expansion, where \"%2F\" will be\nleft encoded.\n\nThe default behavior is to not decode RFC 6570 reserved characters in multi\nsegment matches."]
        #[serde(
            rename = "fullyDecodeReservedExpansion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fully_decode_reserved_expansion: ::std::option::Option<bool>,
        #[doc = "A list of HTTP configuration rules that apply to individual API methods.\n\n**NOTE:** All service configuration rules follow \"last one wins\" order."]
        #[serde(
            rename = "rules",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rules: ::std::option::Option<Vec<crate::schemas::HttpRule>>,
    }
    impl ::google_field_selector::FieldSelector for Http {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Http {
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
    pub struct HttpRule {
        #[doc = "Additional HTTP bindings for the selector. Nested bindings must\nnot contain an `additional_bindings` field themselves (that is,\nthe nesting may only be one level deep)."]
        #[serde(
            rename = "additionalBindings",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub additional_bindings: ::std::option::Option<Vec<crate::schemas::HttpRule>>,
        #[doc = "When this flag is set to true, HTTP requests will be allowed to invoke a\nhalf-duplex streaming method."]
        #[serde(
            rename = "allowHalfDuplex",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub allow_half_duplex: ::std::option::Option<bool>,
        #[doc = "The name of the request field whose value is mapped to the HTTP request\nbody, or `*` for mapping all request fields not captured by the path\npattern to the HTTP body, or omitted for not having any HTTP request body.\n\nNOTE: the referred field must be present at the top-level of the request\nmessage type."]
        #[serde(
            rename = "body",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub body: ::std::option::Option<String>,
        #[doc = "The custom pattern is used for specifying an HTTP method that is not\nincluded in the `pattern` field, such as HEAD, or \"*\" to leave the\nHTTP method unspecified for this rule. The wild-card rule is useful\nfor services that provide content to Web (HTML) clients."]
        #[serde(
            rename = "custom",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub custom: ::std::option::Option<crate::schemas::CustomHttpPattern>,
        #[doc = "Maps to HTTP DELETE. Used for deleting a resource."]
        #[serde(
            rename = "delete",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub delete: ::std::option::Option<String>,
        #[doc = "Maps to HTTP GET. Used for listing and getting information about\nresources."]
        #[serde(
            rename = "get",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub get: ::std::option::Option<String>,
        #[doc = "Maps to HTTP PATCH. Used for updating a resource."]
        #[serde(
            rename = "patch",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub patch: ::std::option::Option<String>,
        #[doc = "Maps to HTTP POST. Used for creating a resource or performing an action."]
        #[serde(
            rename = "post",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub post: ::std::option::Option<String>,
        #[doc = "Maps to HTTP PUT. Used for replacing a resource."]
        #[serde(
            rename = "put",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub put: ::std::option::Option<String>,
        #[doc = "Optional. The name of the response field whose value is mapped to the HTTP\nresponse body. When omitted, the entire response message will be used\nas the HTTP response body.\n\nNOTE: The referred field must be present at the top-level of the response\nmessage type."]
        #[serde(
            rename = "responseBody",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub response_body: ::std::option::Option<String>,
        #[doc = "Selects a method to which this rule applies.\n\nRefer to selector for syntax details."]
        #[serde(
            rename = "selector",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub selector: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for HttpRule {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for HttpRule {
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
    pub struct ImportAdminOverridesResponse {
        #[doc = "The overrides that were created from the imported data."]
        #[serde(
            rename = "overrides",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub overrides: ::std::option::Option<Vec<crate::schemas::QuotaOverride>>,
    }
    impl ::google_field_selector::FieldSelector for ImportAdminOverridesResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ImportAdminOverridesResponse {
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
    pub struct ImportConsumerOverridesResponse {
        #[doc = "The overrides that were created from the imported data."]
        #[serde(
            rename = "overrides",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub overrides: ::std::option::Option<Vec<crate::schemas::QuotaOverride>>,
    }
    impl ::google_field_selector::FieldSelector for ImportConsumerOverridesResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ImportConsumerOverridesResponse {
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
    pub struct JwtLocation {
        #[doc = "Specifies HTTP header name to extract JWT token."]
        #[serde(
            rename = "header",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub header: ::std::option::Option<String>,
        #[doc = "Specifies URL query parameter name to extract JWT token."]
        #[serde(
            rename = "query",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub query: ::std::option::Option<String>,
        #[doc = "The value prefix. The value format is \"value_prefix{token}\"\nOnly applies to \"in\" header type. Must be empty for \"in\" query type.\nIf not empty, the header value has to match (case sensitive) this prefix.\nIf not matched, JWT will not be extracted. If matched, JWT will be\nextracted after the prefix is removed.\n\nFor example, for \"Authorization: Bearer {JWT}\",\nvalue_prefix=\"Bearer \" with a space at the end."]
        #[serde(
            rename = "valuePrefix",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value_prefix: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for JwtLocation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for JwtLocation {
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
    pub struct LabelDescriptor {
        #[doc = "A human-readable description for the label."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "The label key."]
        #[serde(
            rename = "key",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub key: ::std::option::Option<String>,
        #[doc = "The type of data that can be assigned to the label."]
        #[serde(
            rename = "valueType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value_type: ::std::option::Option<crate::schemas::LabelDescriptorValueType>,
    }
    impl ::google_field_selector::FieldSelector for LabelDescriptor {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for LabelDescriptor {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum LabelDescriptorValueType {
        #[doc = "Boolean; true or false."]
        Bool,
        #[doc = "A 64-bit signed integer."]
        Int64,
        #[doc = "A variable-length string. This is the default."]
        String,
    }
    impl LabelDescriptorValueType {
        pub fn as_str(self) -> &'static str {
            match self {
                LabelDescriptorValueType::Bool => "BOOL",
                LabelDescriptorValueType::Int64 => "INT64",
                LabelDescriptorValueType::String => "STRING",
            }
        }
    }
    impl ::std::convert::AsRef<str> for LabelDescriptorValueType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for LabelDescriptorValueType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<LabelDescriptorValueType, ()> {
            Ok(match s {
                "BOOL" => LabelDescriptorValueType::Bool,
                "INT64" => LabelDescriptorValueType::Int64,
                "STRING" => LabelDescriptorValueType::String,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for LabelDescriptorValueType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for LabelDescriptorValueType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for LabelDescriptorValueType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BOOL" => LabelDescriptorValueType::Bool,
                "INT64" => LabelDescriptorValueType::Int64,
                "STRING" => LabelDescriptorValueType::String,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for LabelDescriptorValueType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for LabelDescriptorValueType {
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
    pub struct ListAdminOverridesResponse {
        #[doc = "Token identifying which result to start with; returned by a previous list\ncall."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "Admin overrides on this limit."]
        #[serde(
            rename = "overrides",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub overrides: ::std::option::Option<Vec<crate::schemas::QuotaOverride>>,
    }
    impl ::google_field_selector::FieldSelector for ListAdminOverridesResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListAdminOverridesResponse {
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
    pub struct ListConsumerOverridesResponse {
        #[doc = "Token identifying which result to start with; returned by a previous list\ncall."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "Consumer overrides on this limit."]
        #[serde(
            rename = "overrides",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub overrides: ::std::option::Option<Vec<crate::schemas::QuotaOverride>>,
    }
    impl ::google_field_selector::FieldSelector for ListConsumerOverridesResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListConsumerOverridesResponse {
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
    pub struct ListConsumerQuotaMetricsResponse {
        #[doc = "Quota settings for the consumer, organized by quota metric."]
        #[serde(
            rename = "metrics",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metrics: ::std::option::Option<Vec<crate::schemas::ConsumerQuotaMetric>>,
        #[doc = "Token identifying which result to start with; returned by a previous list\ncall."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ListConsumerQuotaMetricsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListConsumerQuotaMetricsResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct ListOperationsResponse {
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
        pub operations: ::std::option::Option<Vec<crate::schemas::Operation>>,
    }
    impl ::google_field_selector::FieldSelector for ListOperationsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListOperationsResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct ListServicesResponse {
        #[doc = "Token that can be passed to `ListServices` to resume a paginated\nquery."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "The available services for the requested project."]
        #[serde(
            rename = "services",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub services: ::std::option::Option<Vec<crate::schemas::Service>>,
    }
    impl ::google_field_selector::FieldSelector for ListServicesResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListServicesResponse {
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
    pub struct LogDescriptor {
        #[doc = "A human-readable description of this log. This information appears in\nthe documentation and can contain details."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "The human-readable name for this log. This information appears on\nthe user interface and should be concise."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "The set of labels that are available to describe a specific log entry.\nRuntime requests that contain labels not specified here are\nconsidered invalid."]
        #[serde(
            rename = "labels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub labels: ::std::option::Option<Vec<crate::schemas::LabelDescriptor>>,
        #[doc = "The name of the log. It must be less than 512 characters long and can\ninclude the following characters: upper- and lower-case alphanumeric\ncharacters [A-Za-z0-9], and punctuation characters including\nslash, underscore, hyphen, period [/_-.]."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for LogDescriptor {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for LogDescriptor {
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
    pub struct Logging {
        #[doc = "Logging configurations for sending logs to the consumer project.\nThere can be multiple consumer destinations, each one must have a\ndifferent monitored resource type. A log can be used in at most\none consumer destination."]
        #[serde(
            rename = "consumerDestinations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub consumer_destinations: ::std::option::Option<Vec<crate::schemas::LoggingDestination>>,
        #[doc = "Logging configurations for sending logs to the producer project.\nThere can be multiple producer destinations, each one must have a\ndifferent monitored resource type. A log can be used in at most\none producer destination."]
        #[serde(
            rename = "producerDestinations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub producer_destinations: ::std::option::Option<Vec<crate::schemas::LoggingDestination>>,
    }
    impl ::google_field_selector::FieldSelector for Logging {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Logging {
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
    pub struct LoggingDestination {
        #[doc = "Names of the logs to be sent to this destination. Each name must\nbe defined in the Service.logs section. If the log name is\nnot a domain scoped name, it will be automatically prefixed with\nthe service name followed by \"/\"."]
        #[serde(
            rename = "logs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub logs: ::std::option::Option<Vec<String>>,
        #[doc = "The monitored resource type. The type must be defined in the\nService.monitored_resources section."]
        #[serde(
            rename = "monitoredResource",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub monitored_resource: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for LoggingDestination {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for LoggingDestination {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Method {
        #[doc = "The simple name of this method."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Any metadata attached to the method."]
        #[serde(
            rename = "options",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub options: ::std::option::Option<Vec<crate::schemas::Option>>,
        #[doc = "If true, the request is streamed."]
        #[serde(
            rename = "requestStreaming",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub request_streaming: ::std::option::Option<bool>,
        #[doc = "A URL of the input message type."]
        #[serde(
            rename = "requestTypeUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub request_type_url: ::std::option::Option<String>,
        #[doc = "If true, the response is streamed."]
        #[serde(
            rename = "responseStreaming",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub response_streaming: ::std::option::Option<bool>,
        #[doc = "The URL of the output message type."]
        #[serde(
            rename = "responseTypeUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub response_type_url: ::std::option::Option<String>,
        #[doc = "The source syntax of this method."]
        #[serde(
            rename = "syntax",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub syntax: ::std::option::Option<crate::schemas::MethodSyntax>,
    }
    impl ::google_field_selector::FieldSelector for Method {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Method {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum MethodSyntax {
        #[doc = "Syntax `proto2`."]
        SyntaxProto2,
        #[doc = "Syntax `proto3`."]
        SyntaxProto3,
    }
    impl MethodSyntax {
        pub fn as_str(self) -> &'static str {
            match self {
                MethodSyntax::SyntaxProto2 => "SYNTAX_PROTO2",
                MethodSyntax::SyntaxProto3 => "SYNTAX_PROTO3",
            }
        }
    }
    impl ::std::convert::AsRef<str> for MethodSyntax {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for MethodSyntax {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<MethodSyntax, ()> {
            Ok(match s {
                "SYNTAX_PROTO2" => MethodSyntax::SyntaxProto2,
                "SYNTAX_PROTO3" => MethodSyntax::SyntaxProto3,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for MethodSyntax {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for MethodSyntax {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for MethodSyntax {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "SYNTAX_PROTO2" => MethodSyntax::SyntaxProto2,
                "SYNTAX_PROTO3" => MethodSyntax::SyntaxProto3,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for MethodSyntax {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MethodSyntax {
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
    pub struct MetricDescriptor {
        #[doc = "A detailed description of the metric, which can be used in documentation."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "A concise name for the metric, which can be displayed in user interfaces.\nUse sentence case without an ending period, for example \"Request count\".\nThis field is optional but it is recommended to be set for any metrics\nassociated with user-visible concepts, such as Quota."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "The set of labels that can be used to describe a specific\ninstance of this metric type. For example, the\n`appengine.googleapis.com/http/server/response_latencies` metric\ntype has a label for the HTTP response code, `response_code`, so\nyou can look at latencies for successful responses or just\nfor responses that failed."]
        #[serde(
            rename = "labels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub labels: ::std::option::Option<Vec<crate::schemas::LabelDescriptor>>,
        #[doc = "Optional. The launch stage of the metric definition."]
        #[serde(
            rename = "launchStage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub launch_stage: ::std::option::Option<crate::schemas::MetricDescriptorLaunchStage>,
        #[doc = "Optional. Metadata which can be used to guide usage of the metric."]
        #[serde(
            rename = "metadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metadata: ::std::option::Option<crate::schemas::MetricDescriptorMetadata>,
        #[doc = "Whether the metric records instantaneous values, changes to a value, etc.\nSome combinations of `metric_kind` and `value_type` might not be supported."]
        #[serde(
            rename = "metricKind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metric_kind: ::std::option::Option<crate::schemas::MetricDescriptorMetricKind>,
        #[doc = "Read-only. If present, then a time\nseries, which is identified partially by\na metric type and a MonitoredResourceDescriptor, that is associated\nwith this metric type can only be associated with one of the monitored\nresource types listed here."]
        #[serde(
            rename = "monitoredResourceTypes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub monitored_resource_types: ::std::option::Option<Vec<String>>,
        #[doc = "The resource name of the metric descriptor."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The metric type, including its DNS name prefix. The type is not\nURL-encoded.  All user-defined metric types have the DNS name\n`custom.googleapis.com` or `external.googleapis.com`.  Metric types should\nuse a natural hierarchical grouping. For example:\n\n````text\n\"custom.googleapis.com/invoice/paid/amount\"\n\"external.googleapis.com/prometheus/up\"\n\"appengine.googleapis.com/http/server/response_latencies\"````"]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<String>,
        #[doc = "The units in which the metric value is reported. It is only applicable\nif the `value_type` is `INT64`, `DOUBLE`, or `DISTRIBUTION`. The `unit`\ndefines the representation of the stored metric values.\n\nDifferent systems may scale the values to be more easily displayed (so a\nvalue of `0.02KBy` *might* be displayed as `20By`, and a value of\n`3523KBy` *might* be displayed as `3.5MBy`). However, if the `unit` is\n`KBy`, then the value of the metric is always in thousands of bytes, no\nmatter how it may be displayed..\n\nIf you want a custom metric to record the exact number of CPU-seconds used\nby a job, you can create an `INT64 CUMULATIVE` metric whose `unit` is\n`s{CPU}` (or equivalently `1s{CPU}` or just `s`). If the job uses 12,005\nCPU-seconds, then the value is written as `12005`.\n\nAlternatively, if you want a custom metric to record data in a more\ngranular way, you can create a `DOUBLE CUMULATIVE` metric whose `unit` is\n`ks{CPU}`, and then write the value `12.005` (which is `12005/1000`),\nor use `Kis{CPU}` and write `11.723` (which is `12005/1024`).\n\nThe supported units are a subset of [The Unified Code for Units of\nMeasure](http://unitsofmeasure.org/ucum.html) standard:\n\n**Basic units (UNIT)**\n\n* `bit`   bit\n* `By`    byte\n* `s`     second\n* `min`   minute\n* `h`     hour\n* `d`     day\n\n**Prefixes (PREFIX)**\n\n* `k`     kilo    (10^3)\n\n* `M`     mega    (10^6)\n\n* `G`     giga    (10^9)\n\n* `T`     tera    (10^12)\n\n* `P`     peta    (10^15)\n\n* `E`     exa     (10^18)\n\n* `Z`     zetta   (10^21)\n\n* `Y`     yotta   (10^24)\n\n* `m`     milli   (10^-3)\n\n* `u`     micro   (10^-6)\n\n* `n`     nano    (10^-9)\n\n* `p`     pico    (10^-12)\n\n* `f`     femto   (10^-15)\n\n* `a`     atto    (10^-18)\n\n* `z`     zepto   (10^-21)\n\n* `y`     yocto   (10^-24)\n\n* `Ki`    kibi    (2^10)\n\n* `Mi`    mebi    (2^20)\n\n* `Gi`    gibi    (2^30)\n\n* `Ti`    tebi    (2^40)\n\n* `Pi`    pebi    (2^50)\n\n**Grammar**\n\nThe grammar also includes these connectors:\n\n* `/`    division or ratio (as an infix operator). For examples,\n  `kBy/{email}` or `MiBy/10ms` (although you should almost never\n  have `/s` in a metric `unit`; rates should always be computed at\n  query time from the underlying cumulative or delta value).\n* `.`    multiplication or composition (as an infix operator). For\n  examples, `GBy.d` or `k{watt}.h`.\n\nThe grammar for a unit is as follows:\n\n````text\nExpression = Component { \".\" Component } { \"/\" Component } ;\n\nComponent = ( [ PREFIX ] UNIT | \"%\" ) [ Annotation ]\n          | Annotation\n          | \"1\"\n          ;\n\nAnnotation = \"{\" NAME \"}\" ;\n````\n\nNotes:\n\n* `Annotation` is just a comment if it follows a `UNIT`. If the annotation\n  is used alone, then the unit is equivalent to `1`. For examples,\n  `{request}/s == 1/s`, `By{transmitted}/s == By/s`.\n* `NAME` is a sequence of non-blank printable ASCII characters not\n  containing `{` or `}`.\n* `1` represents a unitary [dimensionless\n  unit](https://en.wikipedia.org/wiki/Dimensionless_quantity) of 1, such\n  as in `1/s`. It is typically used when none of the basic units are\n  appropriate. For example, \"new users per day\" can be represented as\n  `1/d` or `{new-users}/d` (and a metric value `5` would mean \"5 new\n  users). Alternatively, \"thousands of page views per day\" would be\n  represented as `1000/d` or `k1/d` or `k{page_views}/d` (and a metric\n  value of `5.3` would mean \"5300 page views per day\").\n* `%` represents dimensionless value of 1/100, and annotates values giving\n  a percentage (so the metric values are typically in the range of 0..100,\n  and a metric value `3` means \"3 percent\").\n* `10^2.%` indicates a metric contains a ratio, typically in the range\n  0..1, that will be multiplied by 100 and displayed as a percentage\n  (so a metric value `0.03` means \"3 percent\")."]
        #[serde(
            rename = "unit",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub unit: ::std::option::Option<String>,
        #[doc = "Whether the measurement is an integer, a floating-point number, etc.\nSome combinations of `metric_kind` and `value_type` might not be supported."]
        #[serde(
            rename = "valueType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value_type: ::std::option::Option<crate::schemas::MetricDescriptorValueType>,
    }
    impl ::google_field_selector::FieldSelector for MetricDescriptor {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MetricDescriptor {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum MetricDescriptorLaunchStage {
        #[doc = "Alpha is a limited availability test for releases before they are cleared\nfor widespread use. By Alpha, all significant design issues are resolved\nand we are in the process of verifying functionality. Alpha customers\nneed to apply for access, agree to applicable terms, and have their\nprojects whitelisted. Alpha releases don’t have to be feature complete,\nno SLAs are provided, and there are no technical support obligations, but\nthey will be far enough along that customers can actually use them in\ntest environments or for limited-use tests -- just like they would in\nnormal production cases."]
        Alpha,
        #[doc = "Beta is the point at which we are ready to open a release for any\ncustomer to use. There are no SLA or technical support obligations in a\nBeta release. Products will be complete from a feature perspective, but\nmay have some open outstanding issues. Beta releases are suitable for\nlimited production use cases."]
        Beta,
        #[doc = "Deprecated features are scheduled to be shut down and removed. For more\ninformation, see the “Deprecation Policy” section of our [Terms of\nService](https://cloud.google.com/terms/)\nand the [Google Cloud Platform Subject to the Deprecation\nPolicy](https://cloud.google.com/terms/deprecation) documentation."]
        Deprecated,
        #[doc = "Early Access features are limited to a closed group of testers. To use\nthese features, you must sign up in advance and sign a Trusted Tester\nagreement (which includes confidentiality provisions). These features may\nbe unstable, changed in backward-incompatible ways, and are not\nguaranteed to be released."]
        EarlyAccess,
        #[doc = "GA features are open to all developers and are considered stable and\nfully qualified for production use."]
        Ga,
        #[doc = "Do not use this default value."]
        LaunchStageUnspecified,
        #[doc = "Prelaunch features are hidden from users and are only visible internally."]
        Prelaunch,
        #[doc = "The feature is not yet implemented. Users can not use it."]
        Unimplemented,
    }
    impl MetricDescriptorLaunchStage {
        pub fn as_str(self) -> &'static str {
            match self {
                MetricDescriptorLaunchStage::Alpha => "ALPHA",
                MetricDescriptorLaunchStage::Beta => "BETA",
                MetricDescriptorLaunchStage::Deprecated => "DEPRECATED",
                MetricDescriptorLaunchStage::EarlyAccess => "EARLY_ACCESS",
                MetricDescriptorLaunchStage::Ga => "GA",
                MetricDescriptorLaunchStage::LaunchStageUnspecified => "LAUNCH_STAGE_UNSPECIFIED",
                MetricDescriptorLaunchStage::Prelaunch => "PRELAUNCH",
                MetricDescriptorLaunchStage::Unimplemented => "UNIMPLEMENTED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for MetricDescriptorLaunchStage {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for MetricDescriptorLaunchStage {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<MetricDescriptorLaunchStage, ()> {
            Ok(match s {
                "ALPHA" => MetricDescriptorLaunchStage::Alpha,
                "BETA" => MetricDescriptorLaunchStage::Beta,
                "DEPRECATED" => MetricDescriptorLaunchStage::Deprecated,
                "EARLY_ACCESS" => MetricDescriptorLaunchStage::EarlyAccess,
                "GA" => MetricDescriptorLaunchStage::Ga,
                "LAUNCH_STAGE_UNSPECIFIED" => MetricDescriptorLaunchStage::LaunchStageUnspecified,
                "PRELAUNCH" => MetricDescriptorLaunchStage::Prelaunch,
                "UNIMPLEMENTED" => MetricDescriptorLaunchStage::Unimplemented,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for MetricDescriptorLaunchStage {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for MetricDescriptorLaunchStage {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for MetricDescriptorLaunchStage {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ALPHA" => MetricDescriptorLaunchStage::Alpha,
                "BETA" => MetricDescriptorLaunchStage::Beta,
                "DEPRECATED" => MetricDescriptorLaunchStage::Deprecated,
                "EARLY_ACCESS" => MetricDescriptorLaunchStage::EarlyAccess,
                "GA" => MetricDescriptorLaunchStage::Ga,
                "LAUNCH_STAGE_UNSPECIFIED" => MetricDescriptorLaunchStage::LaunchStageUnspecified,
                "PRELAUNCH" => MetricDescriptorLaunchStage::Prelaunch,
                "UNIMPLEMENTED" => MetricDescriptorLaunchStage::Unimplemented,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for MetricDescriptorLaunchStage {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MetricDescriptorLaunchStage {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum MetricDescriptorMetricKind {
        #[doc = "A value accumulated over a time interval.  Cumulative\nmeasurements in a time series should have the same start time\nand increasing end times, until an event resets the cumulative\nvalue to zero and sets a new start time for the following\npoints."]
        Cumulative,
        #[doc = "The change in a value during a time interval."]
        Delta,
        #[doc = "An instantaneous measurement of a value."]
        Gauge,
        #[doc = "Do not use this default value."]
        MetricKindUnspecified,
    }
    impl MetricDescriptorMetricKind {
        pub fn as_str(self) -> &'static str {
            match self {
                MetricDescriptorMetricKind::Cumulative => "CUMULATIVE",
                MetricDescriptorMetricKind::Delta => "DELTA",
                MetricDescriptorMetricKind::Gauge => "GAUGE",
                MetricDescriptorMetricKind::MetricKindUnspecified => "METRIC_KIND_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for MetricDescriptorMetricKind {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for MetricDescriptorMetricKind {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<MetricDescriptorMetricKind, ()> {
            Ok(match s {
                "CUMULATIVE" => MetricDescriptorMetricKind::Cumulative,
                "DELTA" => MetricDescriptorMetricKind::Delta,
                "GAUGE" => MetricDescriptorMetricKind::Gauge,
                "METRIC_KIND_UNSPECIFIED" => MetricDescriptorMetricKind::MetricKindUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for MetricDescriptorMetricKind {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for MetricDescriptorMetricKind {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for MetricDescriptorMetricKind {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CUMULATIVE" => MetricDescriptorMetricKind::Cumulative,
                "DELTA" => MetricDescriptorMetricKind::Delta,
                "GAUGE" => MetricDescriptorMetricKind::Gauge,
                "METRIC_KIND_UNSPECIFIED" => MetricDescriptorMetricKind::MetricKindUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for MetricDescriptorMetricKind {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MetricDescriptorMetricKind {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum MetricDescriptorValueType {
        #[doc = "The value is a boolean.\nThis value type can be used only if the metric kind is `GAUGE`."]
        Bool,
        #[doc = "The value is a `Distribution`."]
        Distribution,
        #[doc = "The value is a double precision floating point number."]
        Double,
        #[doc = "The value is a signed 64-bit integer."]
        Int64,
        #[doc = "The value is money."]
        Money,
        #[doc = "The value is a text string.\nThis value type can be used only if the metric kind is `GAUGE`."]
        String,
        #[doc = "Do not use this default value."]
        ValueTypeUnspecified,
    }
    impl MetricDescriptorValueType {
        pub fn as_str(self) -> &'static str {
            match self {
                MetricDescriptorValueType::Bool => "BOOL",
                MetricDescriptorValueType::Distribution => "DISTRIBUTION",
                MetricDescriptorValueType::Double => "DOUBLE",
                MetricDescriptorValueType::Int64 => "INT64",
                MetricDescriptorValueType::Money => "MONEY",
                MetricDescriptorValueType::String => "STRING",
                MetricDescriptorValueType::ValueTypeUnspecified => "VALUE_TYPE_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for MetricDescriptorValueType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for MetricDescriptorValueType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<MetricDescriptorValueType, ()> {
            Ok(match s {
                "BOOL" => MetricDescriptorValueType::Bool,
                "DISTRIBUTION" => MetricDescriptorValueType::Distribution,
                "DOUBLE" => MetricDescriptorValueType::Double,
                "INT64" => MetricDescriptorValueType::Int64,
                "MONEY" => MetricDescriptorValueType::Money,
                "STRING" => MetricDescriptorValueType::String,
                "VALUE_TYPE_UNSPECIFIED" => MetricDescriptorValueType::ValueTypeUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for MetricDescriptorValueType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for MetricDescriptorValueType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for MetricDescriptorValueType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BOOL" => MetricDescriptorValueType::Bool,
                "DISTRIBUTION" => MetricDescriptorValueType::Distribution,
                "DOUBLE" => MetricDescriptorValueType::Double,
                "INT64" => MetricDescriptorValueType::Int64,
                "MONEY" => MetricDescriptorValueType::Money,
                "STRING" => MetricDescriptorValueType::String,
                "VALUE_TYPE_UNSPECIFIED" => MetricDescriptorValueType::ValueTypeUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for MetricDescriptorValueType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MetricDescriptorValueType {
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
    pub struct MetricDescriptorMetadata {
        #[doc = "The delay of data points caused by ingestion. Data points older than this\nage are guaranteed to be ingested and available to be read, excluding\ndata loss due to errors."]
        #[serde(
            rename = "ingestDelay",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ingest_delay: ::std::option::Option<String>,
        #[doc = "Deprecated. Must use the MetricDescriptor.launch_stage instead."]
        #[serde(
            rename = "launchStage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub launch_stage:
            ::std::option::Option<crate::schemas::MetricDescriptorMetadataLaunchStage>,
        #[doc = "The sampling period of metric data points. For metrics which are written\nperiodically, consecutive data points are stored at this time interval,\nexcluding data loss due to errors. Metrics with a higher granularity have\na smaller sampling period."]
        #[serde(
            rename = "samplePeriod",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sample_period: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for MetricDescriptorMetadata {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MetricDescriptorMetadata {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum MetricDescriptorMetadataLaunchStage {
        #[doc = "Alpha is a limited availability test for releases before they are cleared\nfor widespread use. By Alpha, all significant design issues are resolved\nand we are in the process of verifying functionality. Alpha customers\nneed to apply for access, agree to applicable terms, and have their\nprojects whitelisted. Alpha releases don’t have to be feature complete,\nno SLAs are provided, and there are no technical support obligations, but\nthey will be far enough along that customers can actually use them in\ntest environments or for limited-use tests -- just like they would in\nnormal production cases."]
        Alpha,
        #[doc = "Beta is the point at which we are ready to open a release for any\ncustomer to use. There are no SLA or technical support obligations in a\nBeta release. Products will be complete from a feature perspective, but\nmay have some open outstanding issues. Beta releases are suitable for\nlimited production use cases."]
        Beta,
        #[doc = "Deprecated features are scheduled to be shut down and removed. For more\ninformation, see the “Deprecation Policy” section of our [Terms of\nService](https://cloud.google.com/terms/)\nand the [Google Cloud Platform Subject to the Deprecation\nPolicy](https://cloud.google.com/terms/deprecation) documentation."]
        Deprecated,
        #[doc = "Early Access features are limited to a closed group of testers. To use\nthese features, you must sign up in advance and sign a Trusted Tester\nagreement (which includes confidentiality provisions). These features may\nbe unstable, changed in backward-incompatible ways, and are not\nguaranteed to be released."]
        EarlyAccess,
        #[doc = "GA features are open to all developers and are considered stable and\nfully qualified for production use."]
        Ga,
        #[doc = "Do not use this default value."]
        LaunchStageUnspecified,
        #[doc = "Prelaunch features are hidden from users and are only visible internally."]
        Prelaunch,
        #[doc = "The feature is not yet implemented. Users can not use it."]
        Unimplemented,
    }
    impl MetricDescriptorMetadataLaunchStage {
        pub fn as_str(self) -> &'static str {
            match self {
                MetricDescriptorMetadataLaunchStage::Alpha => "ALPHA",
                MetricDescriptorMetadataLaunchStage::Beta => "BETA",
                MetricDescriptorMetadataLaunchStage::Deprecated => "DEPRECATED",
                MetricDescriptorMetadataLaunchStage::EarlyAccess => "EARLY_ACCESS",
                MetricDescriptorMetadataLaunchStage::Ga => "GA",
                MetricDescriptorMetadataLaunchStage::LaunchStageUnspecified => {
                    "LAUNCH_STAGE_UNSPECIFIED"
                }
                MetricDescriptorMetadataLaunchStage::Prelaunch => "PRELAUNCH",
                MetricDescriptorMetadataLaunchStage::Unimplemented => "UNIMPLEMENTED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for MetricDescriptorMetadataLaunchStage {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for MetricDescriptorMetadataLaunchStage {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<MetricDescriptorMetadataLaunchStage, ()> {
            Ok(match s {
                "ALPHA" => MetricDescriptorMetadataLaunchStage::Alpha,
                "BETA" => MetricDescriptorMetadataLaunchStage::Beta,
                "DEPRECATED" => MetricDescriptorMetadataLaunchStage::Deprecated,
                "EARLY_ACCESS" => MetricDescriptorMetadataLaunchStage::EarlyAccess,
                "GA" => MetricDescriptorMetadataLaunchStage::Ga,
                "LAUNCH_STAGE_UNSPECIFIED" => {
                    MetricDescriptorMetadataLaunchStage::LaunchStageUnspecified
                }
                "PRELAUNCH" => MetricDescriptorMetadataLaunchStage::Prelaunch,
                "UNIMPLEMENTED" => MetricDescriptorMetadataLaunchStage::Unimplemented,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for MetricDescriptorMetadataLaunchStage {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for MetricDescriptorMetadataLaunchStage {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for MetricDescriptorMetadataLaunchStage {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ALPHA" => MetricDescriptorMetadataLaunchStage::Alpha,
                "BETA" => MetricDescriptorMetadataLaunchStage::Beta,
                "DEPRECATED" => MetricDescriptorMetadataLaunchStage::Deprecated,
                "EARLY_ACCESS" => MetricDescriptorMetadataLaunchStage::EarlyAccess,
                "GA" => MetricDescriptorMetadataLaunchStage::Ga,
                "LAUNCH_STAGE_UNSPECIFIED" => {
                    MetricDescriptorMetadataLaunchStage::LaunchStageUnspecified
                }
                "PRELAUNCH" => MetricDescriptorMetadataLaunchStage::Prelaunch,
                "UNIMPLEMENTED" => MetricDescriptorMetadataLaunchStage::Unimplemented,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for MetricDescriptorMetadataLaunchStage {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MetricDescriptorMetadataLaunchStage {
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
    pub struct MetricRule {
        #[doc = "Metrics to update when the selected methods are called, and the associated\ncost applied to each metric.\n\nThe key of the map is the metric name, and the values are the amount\nincreased for the metric against which the quota limits are defined.\nThe value must not be negative."]
        #[serde(
            rename = "metricCosts",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metric_costs: ::std::option::Option<::std::collections::BTreeMap<String, i64>>,
        #[doc = "Selects the methods to which this rule applies.\n\nRefer to selector for syntax details."]
        #[serde(
            rename = "selector",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub selector: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for MetricRule {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MetricRule {
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
    pub struct Mixin {
        #[doc = "The fully qualified name of the interface which is included."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "If non-empty specifies a path under which inherited HTTP paths\nare rooted."]
        #[serde(
            rename = "root",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub root: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Mixin {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Mixin {
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
    pub struct MonitoredResourceDescriptor {
        #[doc = "Optional. A detailed description of the monitored resource type that might\nbe used in documentation."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "Optional. A concise name for the monitored resource type that might be\ndisplayed in user interfaces. It should be a Title Cased Noun Phrase,\nwithout any article or other determiners. For example,\n`\"Google Cloud SQL Database\"`."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "Required. A set of labels used to describe instances of this monitored\nresource type. For example, an individual Google Cloud SQL database is\nidentified by values for the labels `\"database_id\"` and `\"zone\"`."]
        #[serde(
            rename = "labels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub labels: ::std::option::Option<Vec<crate::schemas::LabelDescriptor>>,
        #[doc = "Optional. The launch stage of the monitored resource definition."]
        #[serde(
            rename = "launchStage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub launch_stage:
            ::std::option::Option<crate::schemas::MonitoredResourceDescriptorLaunchStage>,
        #[doc = "Optional. The resource name of the monitored resource descriptor:\n`\"projects/{project_id}/monitoredResourceDescriptors/{type}\"` where\n{type} is the value of the `type` field in this object and\n{project_id} is a project ID that provides API-specific context for\naccessing the type.  APIs that do not use project information can use the\nresource name format `\"monitoredResourceDescriptors/{type}\"`."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Required. The monitored resource type. For example, the type\n`\"cloudsql_database\"` represents databases in Google Cloud SQL.\nThe maximum length of this value is 256 characters."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for MonitoredResourceDescriptor {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MonitoredResourceDescriptor {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum MonitoredResourceDescriptorLaunchStage {
        #[doc = "Alpha is a limited availability test for releases before they are cleared\nfor widespread use. By Alpha, all significant design issues are resolved\nand we are in the process of verifying functionality. Alpha customers\nneed to apply for access, agree to applicable terms, and have their\nprojects whitelisted. Alpha releases don’t have to be feature complete,\nno SLAs are provided, and there are no technical support obligations, but\nthey will be far enough along that customers can actually use them in\ntest environments or for limited-use tests -- just like they would in\nnormal production cases."]
        Alpha,
        #[doc = "Beta is the point at which we are ready to open a release for any\ncustomer to use. There are no SLA or technical support obligations in a\nBeta release. Products will be complete from a feature perspective, but\nmay have some open outstanding issues. Beta releases are suitable for\nlimited production use cases."]
        Beta,
        #[doc = "Deprecated features are scheduled to be shut down and removed. For more\ninformation, see the “Deprecation Policy” section of our [Terms of\nService](https://cloud.google.com/terms/)\nand the [Google Cloud Platform Subject to the Deprecation\nPolicy](https://cloud.google.com/terms/deprecation) documentation."]
        Deprecated,
        #[doc = "Early Access features are limited to a closed group of testers. To use\nthese features, you must sign up in advance and sign a Trusted Tester\nagreement (which includes confidentiality provisions). These features may\nbe unstable, changed in backward-incompatible ways, and are not\nguaranteed to be released."]
        EarlyAccess,
        #[doc = "GA features are open to all developers and are considered stable and\nfully qualified for production use."]
        Ga,
        #[doc = "Do not use this default value."]
        LaunchStageUnspecified,
        #[doc = "Prelaunch features are hidden from users and are only visible internally."]
        Prelaunch,
        #[doc = "The feature is not yet implemented. Users can not use it."]
        Unimplemented,
    }
    impl MonitoredResourceDescriptorLaunchStage {
        pub fn as_str(self) -> &'static str {
            match self {
                MonitoredResourceDescriptorLaunchStage::Alpha => "ALPHA",
                MonitoredResourceDescriptorLaunchStage::Beta => "BETA",
                MonitoredResourceDescriptorLaunchStage::Deprecated => "DEPRECATED",
                MonitoredResourceDescriptorLaunchStage::EarlyAccess => "EARLY_ACCESS",
                MonitoredResourceDescriptorLaunchStage::Ga => "GA",
                MonitoredResourceDescriptorLaunchStage::LaunchStageUnspecified => {
                    "LAUNCH_STAGE_UNSPECIFIED"
                }
                MonitoredResourceDescriptorLaunchStage::Prelaunch => "PRELAUNCH",
                MonitoredResourceDescriptorLaunchStage::Unimplemented => "UNIMPLEMENTED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for MonitoredResourceDescriptorLaunchStage {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for MonitoredResourceDescriptorLaunchStage {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<MonitoredResourceDescriptorLaunchStage, ()> {
            Ok(match s {
                "ALPHA" => MonitoredResourceDescriptorLaunchStage::Alpha,
                "BETA" => MonitoredResourceDescriptorLaunchStage::Beta,
                "DEPRECATED" => MonitoredResourceDescriptorLaunchStage::Deprecated,
                "EARLY_ACCESS" => MonitoredResourceDescriptorLaunchStage::EarlyAccess,
                "GA" => MonitoredResourceDescriptorLaunchStage::Ga,
                "LAUNCH_STAGE_UNSPECIFIED" => {
                    MonitoredResourceDescriptorLaunchStage::LaunchStageUnspecified
                }
                "PRELAUNCH" => MonitoredResourceDescriptorLaunchStage::Prelaunch,
                "UNIMPLEMENTED" => MonitoredResourceDescriptorLaunchStage::Unimplemented,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for MonitoredResourceDescriptorLaunchStage {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for MonitoredResourceDescriptorLaunchStage {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for MonitoredResourceDescriptorLaunchStage {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ALPHA" => MonitoredResourceDescriptorLaunchStage::Alpha,
                "BETA" => MonitoredResourceDescriptorLaunchStage::Beta,
                "DEPRECATED" => MonitoredResourceDescriptorLaunchStage::Deprecated,
                "EARLY_ACCESS" => MonitoredResourceDescriptorLaunchStage::EarlyAccess,
                "GA" => MonitoredResourceDescriptorLaunchStage::Ga,
                "LAUNCH_STAGE_UNSPECIFIED" => {
                    MonitoredResourceDescriptorLaunchStage::LaunchStageUnspecified
                }
                "PRELAUNCH" => MonitoredResourceDescriptorLaunchStage::Prelaunch,
                "UNIMPLEMENTED" => MonitoredResourceDescriptorLaunchStage::Unimplemented,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for MonitoredResourceDescriptorLaunchStage {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MonitoredResourceDescriptorLaunchStage {
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
    pub struct Monitoring {
        #[doc = "Monitoring configurations for sending metrics to the consumer project.\nThere can be multiple consumer destinations. A monitored resouce type may\nappear in multiple monitoring destinations if different aggregations are\nneeded for different sets of metrics associated with that monitored\nresource type. A monitored resource and metric pair may only be used once\nin the Monitoring configuration."]
        #[serde(
            rename = "consumerDestinations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub consumer_destinations:
            ::std::option::Option<Vec<crate::schemas::MonitoringDestination>>,
        #[doc = "Monitoring configurations for sending metrics to the producer project.\nThere can be multiple producer destinations. A monitored resouce type may\nappear in multiple monitoring destinations if different aggregations are\nneeded for different sets of metrics associated with that monitored\nresource type. A monitored resource and metric pair may only be used once\nin the Monitoring configuration."]
        #[serde(
            rename = "producerDestinations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub producer_destinations:
            ::std::option::Option<Vec<crate::schemas::MonitoringDestination>>,
    }
    impl ::google_field_selector::FieldSelector for Monitoring {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Monitoring {
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
    pub struct MonitoringDestination {
        #[doc = "Types of the metrics to report to this monitoring destination.\nEach type must be defined in Service.metrics section."]
        #[serde(
            rename = "metrics",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metrics: ::std::option::Option<Vec<String>>,
        #[doc = "The monitored resource type. The type must be defined in\nService.monitored_resources section."]
        #[serde(
            rename = "monitoredResource",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub monitored_resource: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for MonitoringDestination {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MonitoringDestination {
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
    pub struct OauthRequirements {
        #[doc = "The list of publicly documented OAuth scopes that are allowed access. An\nOAuth token containing any of these scopes will be accepted.\n\nExample:\n\n````text\n canonical_scopes: https://www.googleapis.com/auth/calendar,\n                   https://www.googleapis.com/auth/calendar.read````"]
        #[serde(
            rename = "canonicalScopes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub canonical_scopes: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for OauthRequirements {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OauthRequirements {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Operation {
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
        pub error: ::std::option::Option<crate::schemas::Status>,
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
    impl ::google_field_selector::FieldSelector for Operation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Operation {
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
    pub struct OperationMetadata {
        #[doc = "The full name of the resources that this operation is directly\nassociated with."]
        #[serde(
            rename = "resourceNames",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_names: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for OperationMetadata {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OperationMetadata {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Option {
        #[doc = "The option's name. For protobuf built-in options (options defined in\ndescriptor.proto), this is the short name. For example, `\"map_entry\"`.\nFor custom options, it should be the fully-qualified name. For example,\n`\"google.api.http\"`."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The option's value packed in an Any message. If the value is a primitive,\nthe corresponding wrapper type defined in google/protobuf/wrappers.proto\nshould be used. If the value is an enum, it should be stored as an int32\nvalue using the google.protobuf.Int32Value type."]
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value: ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl ::google_field_selector::FieldSelector for Option {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Option {
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
    pub struct Page {
        #[doc = "The Markdown content of the page. You can use <code>(== include {path}\n==)</code> to include content from a Markdown file."]
        #[serde(
            rename = "content",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub content: ::std::option::Option<String>,
        #[doc = "The name of the page. It will be used as an identity of the page to\ngenerate URI of the page, text of the link to this page in navigation,\netc. The full page name (start from the root page name to this page\nconcatenated with `.`) can be used as reference to the page in your\ndocumentation. For example:\n\n<pre><code>pages:\n- name: Tutorial\n  content: &#40;== include tutorial.md ==&#41;\n  subpages:\n  - name: Java\n    content: &#40;== include tutorial_java.md ==&#41;\n</code></pre>\n\nYou can reference `Java` page using Markdown reference link syntax:\n`Java`."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Subpages of this page. The order of subpages specified here will be\nhonored in the generated docset."]
        #[serde(
            rename = "subpages",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub subpages: ::std::option::Option<Vec<crate::schemas::Page>>,
    }
    impl ::google_field_selector::FieldSelector for Page {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Page {
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
    pub struct Quota {
        #[doc = "List of `QuotaLimit` definitions for the service."]
        #[serde(
            rename = "limits",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub limits: ::std::option::Option<Vec<crate::schemas::QuotaLimit>>,
        #[doc = "List of `MetricRule` definitions, each one mapping a selected method to one\nor more metrics."]
        #[serde(
            rename = "metricRules",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metric_rules: ::std::option::Option<Vec<crate::schemas::MetricRule>>,
    }
    impl ::google_field_selector::FieldSelector for Quota {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Quota {
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
    pub struct QuotaBucket {
        #[doc = "Admin override on this quota bucket."]
        #[serde(
            rename = "adminOverride",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub admin_override: ::std::option::Option<crate::schemas::QuotaOverride>,
        #[doc = "Consumer override on this quota bucket."]
        #[serde(
            rename = "consumerOverride",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub consumer_override: ::std::option::Option<crate::schemas::QuotaOverride>,
        #[doc = "The default limit of this quota bucket, as specified by the service\nconfiguration."]
        #[serde(
            rename = "defaultLimit",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub default_limit: ::std::option::Option<i64>,
        #[doc = "The dimensions of this quota bucket.\n\nIf this map is empty, this is the global bucket, which is the default quota\nvalue applied to all requests that do not have a more specific override.\n\nIf this map is nonempty, the default limit, effective limit, and quota\noverrides apply only to requests that have the dimensions given in the map.\n\nFor example, if the map has key \"region\" and value \"us-east-1\", then the\nspecified effective limit is only effective in that region, and the\nspecified overrides apply only in that region."]
        #[serde(
            rename = "dimensions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dimensions: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "The effective limit of this quota bucket. Equal to default_limit if there\nare no overrides."]
        #[serde(
            rename = "effectiveLimit",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub effective_limit: ::std::option::Option<i64>,
        #[doc = "Producer override on this quota bucket."]
        #[serde(
            rename = "producerOverride",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub producer_override: ::std::option::Option<crate::schemas::QuotaOverride>,
    }
    impl ::google_field_selector::FieldSelector for QuotaBucket {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for QuotaBucket {
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
    pub struct QuotaLimit {
        #[doc = "Default number of tokens that can be consumed during the specified\nduration. This is the number of tokens assigned when a client\napplication developer activates the service for his/her project.\n\nSpecifying a value of 0 will block all requests. This can be used if you\nare provisioning quota to selected consumers and blocking others.\nSimilarly, a value of -1 will indicate an unlimited quota. No other\nnegative values are allowed.\n\nUsed by group-based quotas only."]
        #[serde(
            rename = "defaultLimit",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub default_limit: ::std::option::Option<i64>,
        #[doc = "Optional. User-visible, extended description for this quota limit.\nShould be used only when more context is needed to understand this limit\nthan provided by the limit's display name (see: `display_name`)."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "User-visible display name for this limit.\nOptional. If not set, the UI will provide a default display name based on\nthe quota configuration. This field can be used to override the default\ndisplay name generated from the configuration."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "Duration of this limit in textual notation. Must be \"100s\" or \"1d\".\n\nUsed by group-based quotas only."]
        #[serde(
            rename = "duration",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub duration: ::std::option::Option<String>,
        #[doc = "Free tier value displayed in the Developers Console for this limit.\nThe free tier is the number of tokens that will be subtracted from the\nbilled amount when billing is enabled.\nThis field can only be set on a limit with duration \"1d\", in a billable\ngroup; it is invalid on any other limit. If this field is not set, it\ndefaults to 0, indicating that there is no free tier for this service.\n\nUsed by group-based quotas only."]
        #[serde(
            rename = "freeTier",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub free_tier: ::std::option::Option<i64>,
        #[doc = "Maximum number of tokens that can be consumed during the specified\nduration. Client application developers can override the default limit up\nto this maximum. If specified, this value cannot be set to a value less\nthan the default limit. If not specified, it is set to the default limit.\n\nTo allow clients to apply overrides with no upper bound, set this to -1,\nindicating unlimited maximum quota.\n\nUsed by group-based quotas only."]
        #[serde(
            rename = "maxLimit",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub max_limit: ::std::option::Option<i64>,
        #[doc = "The name of the metric this quota limit applies to. The quota limits with\nthe same metric will be checked together during runtime. The metric must be\ndefined within the service config."]
        #[serde(
            rename = "metric",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metric: ::std::option::Option<String>,
        #[doc = "Name of the quota limit.\n\nThe name must be provided, and it must be unique within the service. The\nname can only include alphanumeric characters as well as '-'.\n\nThe maximum length of the limit name is 64 characters."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Specify the unit of the quota limit. It uses the same syntax as\nMetric.unit. The supported unit kinds are determined by the quota\nbackend system.\n\nHere are some examples:\n\n* \"1/min/{project}\" for quota per minute per project.\n\nNote: the order of unit components is insignificant.\nThe \"1\" at the beginning is required to follow the metric unit syntax."]
        #[serde(
            rename = "unit",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub unit: ::std::option::Option<String>,
        #[doc = "Tiered limit values. You must specify this as a key:value pair, with an\ninteger value that is the maximum number of requests allowed for the\nspecified unit. Currently only STANDARD is supported."]
        #[serde(
            rename = "values",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub values: ::std::option::Option<::std::collections::BTreeMap<String, i64>>,
    }
    impl ::google_field_selector::FieldSelector for QuotaLimit {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for QuotaLimit {
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
    pub struct QuotaOverride {
        #[doc = "If this map is nonempty, then this override applies only to specific values\nfor dimensions defined in the limit unit.\n\nFor example, an override on a limit with the unit 1/{project}/{region}\ncould contain an entry with the key \"region\" and the value \"us-east-1\";\nthe override is only applied to quota consumed in that region.\n\nThis map has the following restrictions:\n\n* Keys that are not defined in the limit's unit are not valid keys.\n  Any string appearing in {brackets} in the unit (besides {project} or\n  {user}) is a defined key.\n* \"project\" is not a valid key; the project is already specified in\n  the parent resource name.\n* \"user\" is not a valid key; the API does not support quota overrides\n  that apply only to a specific user.\n* If \"region\" appears as a key, its value must be a valid Cloud region.\n* If \"zone\" appears as a key, its value must be a valid Cloud zone.\n* If any valid key other than \"region\" or \"zone\" appears in the map, then\n  all valid keys other than \"region\" or \"zone\" must also appear in the\n  map."]
        #[serde(
            rename = "dimensions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dimensions: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "The name of the metric to which this override applies.\n\nAn example name would be:\n`compute.googleapis.com/cpus`"]
        #[serde(
            rename = "metric",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metric: ::std::option::Option<String>,
        #[doc = "The resource name of the override.\nThis name is generated by the server when the override is created.\n\nExample names would be:\n`projects/123/services/compute.googleapis.com/consumerQuotaMetrics/compute.googleapis.com%2Fcpus/limits/%2Fproject%2Fregion/adminOverrides/4a3f2c1d`\n`projects/123/services/compute.googleapis.com/consumerQuotaMetrics/compute.googleapis.com%2Fcpus/limits/%2Fproject%2Fregion/consumerOverrides/4a3f2c1d`\n\nThe resource name is intended to be opaque and should not be parsed for\nits component strings, since its representation could change in the future."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The overriding quota limit value.\nCan be any nonnegative integer, or -1 (unlimited quota)."]
        #[serde(
            rename = "overrideValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub override_value: ::std::option::Option<i64>,
        #[doc = "The limit unit of the limit to which this override applies.\n\nAn example unit would be:\n`1/{project}/{region}`\nNote that `{project}` and `{region}` are not placeholders in this example;\nthe literal characters `{` and `}` occur in the string."]
        #[serde(
            rename = "unit",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub unit: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for QuotaOverride {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for QuotaOverride {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Service {
        #[doc = "The service configuration of the available service.\nSome fields may be filtered out of the configuration in responses to\nthe `ListServices` method. These fields are present only in responses to\nthe `GetService` method."]
        #[serde(
            rename = "config",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub config: ::std::option::Option<crate::schemas::ServiceConfig>,
        #[doc = "The resource name of the consumer and service.\n\nA valid name would be:\n\n* projects/123/services/serviceusage.googleapis.com"]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The resource name of the consumer.\n\nA valid name would be:\n\n* projects/123"]
        #[serde(
            rename = "parent",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub parent: ::std::option::Option<String>,
        #[doc = "Whether or not the service has been enabled for use by the consumer."]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state: ::std::option::Option<crate::schemas::ServiceState>,
    }
    impl ::google_field_selector::FieldSelector for Service {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Service {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ServiceState {
        #[doc = "The service cannot be used by this consumer. It has either been explicitly\ndisabled, or has never been enabled."]
        Disabled,
        #[doc = "The service has been explicitly enabled for use by this consumer."]
        Enabled,
        #[doc = "The default value, which indicates that the enabled state of the service\nis unspecified or not meaningful. Currently, all consumers other than\nprojects (such as folders and organizations) are always in this state."]
        StateUnspecified,
    }
    impl ServiceState {
        pub fn as_str(self) -> &'static str {
            match self {
                ServiceState::Disabled => "DISABLED",
                ServiceState::Enabled => "ENABLED",
                ServiceState::StateUnspecified => "STATE_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ServiceState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ServiceState {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ServiceState, ()> {
            Ok(match s {
                "DISABLED" => ServiceState::Disabled,
                "ENABLED" => ServiceState::Enabled,
                "STATE_UNSPECIFIED" => ServiceState::StateUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ServiceState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ServiceState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ServiceState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DISABLED" => ServiceState::Disabled,
                "ENABLED" => ServiceState::Enabled,
                "STATE_UNSPECIFIED" => ServiceState::StateUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ServiceState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ServiceState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct ServiceConfig {
        #[doc = "A list of API interfaces exported by this service. Contains only the names,\nversions, and method names of the interfaces."]
        #[serde(
            rename = "apis",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub apis: ::std::option::Option<Vec<crate::schemas::Api>>,
        #[doc = "Auth configuration. Contains only the OAuth rules."]
        #[serde(
            rename = "authentication",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub authentication: ::std::option::Option<crate::schemas::Authentication>,
        #[doc = "Additional API documentation. Contains only the summary and the\ndocumentation URL."]
        #[serde(
            rename = "documentation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub documentation: ::std::option::Option<crate::schemas::Documentation>,
        #[doc = "Configuration for network endpoints. Contains only the names and aliases\nof the endpoints."]
        #[serde(
            rename = "endpoints",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub endpoints: ::std::option::Option<Vec<crate::schemas::Endpoint>>,
        #[doc = "The DNS address at which this service is available.\n\nAn example DNS address would be:\n`calendar.googleapis.com`."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Quota configuration."]
        #[serde(
            rename = "quota",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub quota: ::std::option::Option<crate::schemas::Quota>,
        #[doc = "The product title for this service."]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
        #[doc = "Configuration controlling usage of this service."]
        #[serde(
            rename = "usage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub usage: ::std::option::Option<crate::schemas::Usage>,
    }
    impl ::google_field_selector::FieldSelector for ServiceConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ServiceConfig {
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
    pub struct ServiceIdentity {
        #[doc = "The email address of the service account that a service producer would use\nto access consumer resources."]
        #[serde(
            rename = "email",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub email: ::std::option::Option<String>,
        #[doc = "The unique and stable id of the service account.\nhttps://cloud.google.com/iam/reference/rest/v1/projects.serviceAccounts#ServiceAccount"]
        #[serde(
            rename = "uniqueId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub unique_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ServiceIdentity {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ServiceIdentity {
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
    pub struct SourceContext {
        #[doc = "The path-qualified name of the .proto file that contained the associated\nprotobuf element.  For example: `\"google/protobuf/source_context.proto\"`."]
        #[serde(
            rename = "fileName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub file_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for SourceContext {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SourceContext {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct SourceInfo {
        #[doc = "All files used during config generation."]
        #[serde(
            rename = "sourceFiles",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source_files:
            ::std::option::Option<Vec<::std::collections::BTreeMap<String, ::serde_json::Value>>>,
    }
    impl ::google_field_selector::FieldSelector for SourceInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SourceInfo {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Status {
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
    impl ::google_field_selector::FieldSelector for Status {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Status {
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
    pub struct SystemParameter {
        #[doc = "Define the HTTP header name to use for the parameter. It is case\ninsensitive."]
        #[serde(
            rename = "httpHeader",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub http_header: ::std::option::Option<String>,
        #[doc = "Define the name of the parameter, such as \"api_key\" . It is case sensitive."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Define the URL query parameter name to use for the parameter. It is case\nsensitive."]
        #[serde(
            rename = "urlQueryParameter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub url_query_parameter: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for SystemParameter {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SystemParameter {
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
    pub struct SystemParameterRule {
        #[doc = "Define parameters. Multiple names may be defined for a parameter.\nFor a given method call, only one of them should be used. If multiple\nnames are used the behavior is implementation-dependent.\nIf none of the specified names are present the behavior is\nparameter-dependent."]
        #[serde(
            rename = "parameters",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub parameters: ::std::option::Option<Vec<crate::schemas::SystemParameter>>,
        #[doc = "Selects the methods to which this rule applies. Use '*' to indicate all\nmethods in all APIs.\n\nRefer to selector for syntax details."]
        #[serde(
            rename = "selector",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub selector: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for SystemParameterRule {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SystemParameterRule {
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
    pub struct SystemParameters {
        #[doc = "Define system parameters.\n\nThe parameters defined here will override the default parameters\nimplemented by the system. If this field is missing from the service\nconfig, default system parameters will be used. Default system parameters\nand names is implementation-dependent.\n\nExample: define api key for all methods\n\n````text\nsystem_parameters\n  rules:\n    - selector: \"*\"\n      parameters:\n        - name: api_key\n          url_query_parameter: api_key\n````\n\nExample: define 2 api key names for a specific method.\n\n````text\nsystem_parameters\n  rules:\n    - selector: \"/ListShelves\"\n      parameters:\n        - name: api_key\n          http_header: Api-Key1\n        - name: api_key\n          http_header: Api-Key2\n````\n\n**NOTE:** All service configuration rules follow \"last one wins\" order."]
        #[serde(
            rename = "rules",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rules: ::std::option::Option<Vec<crate::schemas::SystemParameterRule>>,
    }
    impl ::google_field_selector::FieldSelector for SystemParameters {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SystemParameters {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Type {
        #[doc = "The list of fields."]
        #[serde(
            rename = "fields",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fields: ::std::option::Option<Vec<crate::schemas::Field>>,
        #[doc = "The fully qualified message name."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The list of types appearing in `oneof` definitions in this type."]
        #[serde(
            rename = "oneofs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub oneofs: ::std::option::Option<Vec<String>>,
        #[doc = "The protocol buffer options."]
        #[serde(
            rename = "options",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub options: ::std::option::Option<Vec<crate::schemas::Option>>,
        #[doc = "The source context."]
        #[serde(
            rename = "sourceContext",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source_context: ::std::option::Option<crate::schemas::SourceContext>,
        #[doc = "The source syntax."]
        #[serde(
            rename = "syntax",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub syntax: ::std::option::Option<crate::schemas::TypeSyntax>,
    }
    impl ::google_field_selector::FieldSelector for Type {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Type {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum TypeSyntax {
        #[doc = "Syntax `proto2`."]
        SyntaxProto2,
        #[doc = "Syntax `proto3`."]
        SyntaxProto3,
    }
    impl TypeSyntax {
        pub fn as_str(self) -> &'static str {
            match self {
                TypeSyntax::SyntaxProto2 => "SYNTAX_PROTO2",
                TypeSyntax::SyntaxProto3 => "SYNTAX_PROTO3",
            }
        }
    }
    impl ::std::convert::AsRef<str> for TypeSyntax {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for TypeSyntax {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<TypeSyntax, ()> {
            Ok(match s {
                "SYNTAX_PROTO2" => TypeSyntax::SyntaxProto2,
                "SYNTAX_PROTO3" => TypeSyntax::SyntaxProto3,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for TypeSyntax {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for TypeSyntax {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for TypeSyntax {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "SYNTAX_PROTO2" => TypeSyntax::SyntaxProto2,
                "SYNTAX_PROTO3" => TypeSyntax::SyntaxProto3,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for TypeSyntax {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TypeSyntax {
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
    pub struct Usage {
        #[doc = "The full resource name of a channel used for sending notifications to the\nservice producer.\n\nGoogle Service Management currently only supports\n[Google Cloud Pub/Sub](https://cloud.google.com/pubsub) as a notification\nchannel. To use Google Cloud Pub/Sub as the channel, this must be the name\nof a Cloud Pub/Sub topic that uses the Cloud Pub/Sub topic name format\ndocumented in https://cloud.google.com/pubsub/docs/overview."]
        #[serde(
            rename = "producerNotificationChannel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub producer_notification_channel: ::std::option::Option<String>,
        #[doc = "Requirements that must be satisfied before a consumer project can use the\nservice. Each requirement is of the form <service.name>/<requirement-id>;\nfor example 'serviceusage.googleapis.com/billing-enabled'."]
        #[serde(
            rename = "requirements",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub requirements: ::std::option::Option<Vec<String>>,
        #[doc = "A list of usage rules that apply to individual API methods.\n\n**NOTE:** All service configuration rules follow \"last one wins\" order."]
        #[serde(
            rename = "rules",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rules: ::std::option::Option<Vec<crate::schemas::UsageRule>>,
        #[doc = "The configuration of a per-product per-project service identity."]
        #[serde(
            rename = "serviceIdentity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub service_identity: ::std::option::Option<crate::schemas::GoogleApiServiceIdentity>,
    }
    impl ::google_field_selector::FieldSelector for Usage {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Usage {
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
    pub struct UsageRule {
        #[doc = "If true, the selected method allows unregistered calls, e.g. calls\nthat don't identify any user or application."]
        #[serde(
            rename = "allowUnregisteredCalls",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub allow_unregistered_calls: ::std::option::Option<bool>,
        #[doc = "Selects the methods to which this rule applies. Use '*' to indicate all\nmethods in all APIs.\n\nRefer to selector for syntax details."]
        #[serde(
            rename = "selector",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub selector: ::std::option::Option<String>,
        #[doc = "If true, the selected method should skip service control and the control\nplane features, such as quota and billing, will not be available.\nThis flag is used by Google Cloud Endpoints to bypass checks for internal\nmethods, such as service health check methods."]
        #[serde(
            rename = "skipServiceControl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub skip_service_control: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for UsageRule {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UsageRule {
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
    #[doc = "Actions that can be performed on the operations resource"]
    pub fn operations(&self) -> crate::resources::operations::OperationsActions {
        crate::resources::operations::OperationsActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the services resource"]
    pub fn services(&self) -> crate::resources::services::ServicesActions {
        crate::resources::services::ServicesActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
}
pub mod resources {
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
            pub fn list(&self) -> ListRequestBuilder {
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
                    filter: None,
                    name: None,
                    page_size: None,
                    page_token: None,
                }
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
            ) -> Result<crate::schemas::Operation, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Operation, crate::Error> {
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
                let mut output = "https://serviceusage.googleapis.com/".to_owned();
                output.push_str("v1beta1/");
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
            filter: Option<String>,
            name: Option<String>,
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
            #[doc = "The name of the operation's parent resource."]
            pub fn name(mut self, value: impl Into<String>) -> Self {
                self.name = Some(value.into());
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
            ) -> crate::iter::PageItemIter<Self, crate::schemas::Operation> {
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
            ) -> crate::iter::PageItemIter<Self, crate::schemas::Operation> {
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
            ) -> crate::iter::PageIter<Self, crate::schemas::ListOperationsResponse> {
                self.iter_with_fields(None::<&str>)
            }
            pub fn iter_with_all_fields(
                self,
            ) -> crate::iter::PageIter<Self, crate::schemas::ListOperationsResponse> {
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
            ) -> Result<crate::schemas::ListOperationsResponse, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::ListOperationsResponse, crate::Error> {
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
                let mut output = "https://serviceusage.googleapis.com/".to_owned();
                output.push_str("v1beta1/operations");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("filter", &self.filter)]);
                let req = req.query(&[("name", &self.name)]);
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
    pub mod services {
        pub mod params {}
        pub struct ServicesActions<'a> {
            pub(crate) reqwest: &'a reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> ServicesActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Enable multiple services on a project. The operation is atomic: if enabling\nany service fails, then the entire batch fails, and no state changes occur.\n\nOperation<response: google.protobuf.Empty>"]
            pub fn batch_enable(
                &self,
                request: crate::schemas::BatchEnableServicesRequest,
                parent: impl Into<String>,
            ) -> BatchEnableRequestBuilder {
                BatchEnableRequestBuilder {
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
            #[doc = "Disable a service so that it can no longer be used with a project.\nThis prevents unintended usage that may cause unexpected billing\ncharges or security leaks.\n\nIt is not valid to call the disable method on a service that is not\ncurrently enabled. Callers will receive a `FAILED_PRECONDITION` status if\nthe target service is not currently enabled.\n\nOperation<response: google.protobuf.Empty>"]
            pub fn disable(
                &self,
                request: crate::schemas::DisableServiceRequest,
                name: impl Into<String>,
            ) -> DisableRequestBuilder {
                DisableRequestBuilder {
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
            #[doc = "Enable a service so that it can be used with a project.\n\nOperation<response: google.protobuf.Empty>"]
            pub fn enable(
                &self,
                request: crate::schemas::EnableServiceRequest,
                name: impl Into<String>,
            ) -> EnableRequestBuilder {
                EnableRequestBuilder {
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
            #[doc = "Returns the service configuration and enabled state for a given service."]
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
            #[doc = "List all services available to the specified project, and the current\nstate of those services with respect to the project. The list includes\nall public services, all services for which the calling user has the\n`servicemanagement.services.bind` permission, and all services that have\nalready been enabled on the project. The list can be filtered to\nonly include services in a specific state, for example to only include\nservices enabled on the project."]
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
            #[doc = "Actions that can be performed on the consumer_quota_metrics resource"]
            pub fn consumer_quota_metrics(
                &self,
            ) -> crate::resources::services::consumer_quota_metrics::ConsumerQuotaMetricsActions
            {
                crate::resources::services::consumer_quota_metrics::ConsumerQuotaMetricsActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
        }
        #[doc = "Created via [ServicesActions::batch_enable()](struct.ServicesActions.html#method.batch_enable)"]
        #[derive(Debug, Clone)]
        pub struct BatchEnableRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::BatchEnableServicesRequest,
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
        impl<'a> BatchEnableRequestBuilder<'a> {
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
            ) -> Result<crate::schemas::Operation, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Operation, crate::Error> {
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
                let mut output = "https://serviceusage.googleapis.com/".to_owned();
                output.push_str("v1beta1/");
                {
                    let var_as_str = &self.parent;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str("/services:batchEnable");
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
        #[doc = "Created via [ServicesActions::disable()](struct.ServicesActions.html#method.disable)"]
        #[derive(Debug, Clone)]
        pub struct DisableRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::DisableServiceRequest,
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
        impl<'a> DisableRequestBuilder<'a> {
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
            ) -> Result<crate::schemas::Operation, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Operation, crate::Error> {
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
                let mut output = "https://serviceusage.googleapis.com/".to_owned();
                output.push_str("v1beta1/");
                {
                    let var_as_str = &self.name;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str(":disable");
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
        #[doc = "Created via [ServicesActions::enable()](struct.ServicesActions.html#method.enable)"]
        #[derive(Debug, Clone)]
        pub struct EnableRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::EnableServiceRequest,
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
        impl<'a> EnableRequestBuilder<'a> {
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
            ) -> Result<crate::schemas::Operation, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Operation, crate::Error> {
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
                let mut output = "https://serviceusage.googleapis.com/".to_owned();
                output.push_str("v1beta1/");
                {
                    let var_as_str = &self.name;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str(":enable");
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
        #[doc = "Created via [ServicesActions::get()](struct.ServicesActions.html#method.get)"]
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
            ) -> Result<crate::schemas::Service, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(self) -> Result<crate::schemas::Service, crate::Error> {
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
                let mut output = "https://serviceusage.googleapis.com/".to_owned();
                output.push_str("v1beta1/");
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
        #[doc = "Created via [ServicesActions::list()](struct.ServicesActions.html#method.list)"]
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
            #[doc = "Only list services that conform to the given filter.\nThe allowed filter strings are `state:ENABLED` and `state:DISABLED`."]
            pub fn filter(mut self, value: impl Into<String>) -> Self {
                self.filter = Some(value.into());
                self
            }
            #[doc = "Requested size of the next page of data.\nRequested page size cannot exceed 200.\nIf not set, the default page size is 50."]
            pub fn page_size(mut self, value: i32) -> Self {
                self.page_size = Some(value);
                self
            }
            #[doc = "Token identifying which result to start with, which is returned by a\nprevious list call."]
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
            pub fn iter_services<T>(self) -> crate::iter::PageItemIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.iter_services_with_fields(fields)
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be the default fields populated by"]
            #[doc = r" the server."]
            pub fn iter_services_with_default_fields(
                self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::Service> {
                self.iter_services_with_fields(None::<String>)
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be all fields available. This should"]
            #[doc = r" primarily be used during developement and debugging as fetching"]
            #[doc = r" all fields can be expensive both in bandwidth and server"]
            #[doc = r" resources."]
            pub fn iter_services_with_all_fields(
                self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::Service> {
                self.iter_services_with_fields(Some("*"))
            }
            pub fn iter_services_with_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> crate::iter::PageItemIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned,
                F: AsRef<str>,
            {
                self.fields = Some({
                    let mut selector = concat!("nextPageToken,", "services").to_owned();
                    let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                    if !items_fields.is_empty() {
                        selector.push_str("(");
                        selector.push_str(items_fields);
                        selector.push_str(")");
                    }
                    selector
                });
                crate::iter::PageItemIter::new(self, "services")
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
            ) -> crate::iter::PageIter<Self, crate::schemas::ListServicesResponse> {
                self.iter_with_fields(None::<&str>)
            }
            pub fn iter_with_all_fields(
                self,
            ) -> crate::iter::PageIter<Self, crate::schemas::ListServicesResponse> {
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
            ) -> Result<crate::schemas::ListServicesResponse, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::ListServicesResponse, crate::Error> {
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
                let mut output = "https://serviceusage.googleapis.com/".to_owned();
                output.push_str("v1beta1/");
                {
                    let var_as_str = &self.parent;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str("/services");
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
        pub mod consumer_quota_metrics {
            pub mod params {
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum GetView {
                    Basic,
                    Full,
                    QuotaViewUnspecified,
                }
                impl GetView {
                    pub fn as_str(self) -> &'static str {
                        match self {
                            GetView::Basic => "BASIC",
                            GetView::Full => "FULL",
                            GetView::QuotaViewUnspecified => "QUOTA_VIEW_UNSPECIFIED",
                        }
                    }
                }
                impl ::std::convert::AsRef<str> for GetView {
                    fn as_ref(&self) -> &str {
                        self.as_str()
                    }
                }
                impl ::std::str::FromStr for GetView {
                    type Err = ();
                    fn from_str(s: &str) -> ::std::result::Result<GetView, ()> {
                        Ok(match s {
                            "BASIC" => GetView::Basic,
                            "FULL" => GetView::Full,
                            "QUOTA_VIEW_UNSPECIFIED" => GetView::QuotaViewUnspecified,
                            _ => return Err(()),
                        })
                    }
                }
                impl ::std::fmt::Display for GetView {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for GetView {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for GetView {
                    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                    where
                        D: ::serde::de::Deserializer<'de>,
                    {
                        let value: &'de str = <&str>::deserialize(deserializer)?;
                        Ok(match value {
                            "BASIC" => GetView::Basic,
                            "FULL" => GetView::Full,
                            "QUOTA_VIEW_UNSPECIFIED" => GetView::QuotaViewUnspecified,
                            _ => {
                                return Err(::serde::de::Error::custom(format!(
                                    "invalid enum for #name: {}",
                                    value
                                )))
                            }
                        })
                    }
                }
                impl ::google_field_selector::FieldSelector for GetView {
                    fn fields() -> Vec<::google_field_selector::Field> {
                        Vec::new()
                    }
                }
                impl ::google_field_selector::ToFieldType for GetView {
                    fn field_type() -> ::google_field_selector::FieldType {
                        ::google_field_selector::FieldType::Leaf
                    }
                }
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum ListView {
                    Basic,
                    Full,
                    QuotaViewUnspecified,
                }
                impl ListView {
                    pub fn as_str(self) -> &'static str {
                        match self {
                            ListView::Basic => "BASIC",
                            ListView::Full => "FULL",
                            ListView::QuotaViewUnspecified => "QUOTA_VIEW_UNSPECIFIED",
                        }
                    }
                }
                impl ::std::convert::AsRef<str> for ListView {
                    fn as_ref(&self) -> &str {
                        self.as_str()
                    }
                }
                impl ::std::str::FromStr for ListView {
                    type Err = ();
                    fn from_str(s: &str) -> ::std::result::Result<ListView, ()> {
                        Ok(match s {
                            "BASIC" => ListView::Basic,
                            "FULL" => ListView::Full,
                            "QUOTA_VIEW_UNSPECIFIED" => ListView::QuotaViewUnspecified,
                            _ => return Err(()),
                        })
                    }
                }
                impl ::std::fmt::Display for ListView {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for ListView {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for ListView {
                    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                    where
                        D: ::serde::de::Deserializer<'de>,
                    {
                        let value: &'de str = <&str>::deserialize(deserializer)?;
                        Ok(match value {
                            "BASIC" => ListView::Basic,
                            "FULL" => ListView::Full,
                            "QUOTA_VIEW_UNSPECIFIED" => ListView::QuotaViewUnspecified,
                            _ => {
                                return Err(::serde::de::Error::custom(format!(
                                    "invalid enum for #name: {}",
                                    value
                                )))
                            }
                        })
                    }
                }
                impl ::google_field_selector::FieldSelector for ListView {
                    fn fields() -> Vec<::google_field_selector::Field> {
                        Vec::new()
                    }
                }
                impl ::google_field_selector::ToFieldType for ListView {
                    fn field_type() -> ::google_field_selector::FieldType {
                        ::google_field_selector::FieldType::Leaf
                    }
                }
            }
            pub struct ConsumerQuotaMetricsActions<'a> {
                pub(crate) reqwest: &'a reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> ConsumerQuotaMetricsActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Retrieves a summary of quota information for a specific quota metric"]
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
                        view: None,
                    }
                }
                #[doc = "Retrieves a summary of all quota information visible to the service\nconsumer, organized by service metric. Each metric includes information\nabout all of its defined limits. Each limit includes the limit\nconfiguration (quota unit, preciseness, default value), the current\neffective limit value, and all of the overrides applied to the limit."]
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
                        view: None,
                    }
                }
                #[doc = "Actions that can be performed on the limits resource"]
                pub fn limits(
                    &self,
                ) -> crate::resources::services::consumer_quota_metrics::limits::LimitsActions
                {
                    crate::resources::services::consumer_quota_metrics::limits::LimitsActions {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                    }
                }
            }
            #[doc = "Created via [ConsumerQuotaMetricsActions::get()](struct.ConsumerQuotaMetricsActions.html#method.get)"]
            #[derive(Debug, Clone)]
            pub struct GetRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                name: String,
                view: Option<crate::resources::services::consumer_quota_metrics::params::GetView>,
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
                #[doc = "Specifies the level of detail for quota information in the response."]
                pub fn view(
                    mut self,
                    value: crate::resources::services::consumer_quota_metrics::params::GetView,
                ) -> Self {
                    self.view = Some(value);
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
                ) -> Result<crate::schemas::ConsumerQuotaMetric, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::ConsumerQuotaMetric, crate::Error> {
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
                    let mut output = "https://serviceusage.googleapis.com/".to_owned();
                    output.push_str("v1beta1/");
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
                    let req = req.query(&[("view", &self.view)]);
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
            #[doc = "Created via [ConsumerQuotaMetricsActions::list()](struct.ConsumerQuotaMetricsActions.html#method.list)"]
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                parent: String,
                page_size: Option<i32>,
                page_token: Option<String>,
                view: Option<crate::resources::services::consumer_quota_metrics::params::ListView>,
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
                #[doc = "Requested size of the next page of data."]
                pub fn page_size(mut self, value: i32) -> Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "Token identifying which result to start with; returned by a previous list\ncall."]
                pub fn page_token(mut self, value: impl Into<String>) -> Self {
                    self.page_token = Some(value.into());
                    self
                }
                #[doc = "Specifies the level of detail for quota information in the response."]
                pub fn view(
                    mut self,
                    value: crate::resources::services::consumer_quota_metrics::params::ListView,
                ) -> Self {
                    self.view = Some(value);
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
                pub fn iter_metrics<T>(self) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.iter_metrics_with_fields(fields)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be the default fields populated by"]
                #[doc = r" the server."]
                pub fn iter_metrics_with_default_fields(
                    self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::ConsumerQuotaMetric>
                {
                    self.iter_metrics_with_fields(None::<String>)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be all fields available. This should"]
                #[doc = r" primarily be used during developement and debugging as fetching"]
                #[doc = r" all fields can be expensive both in bandwidth and server"]
                #[doc = r" resources."]
                pub fn iter_metrics_with_all_fields(
                    self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::ConsumerQuotaMetric>
                {
                    self.iter_metrics_with_fields(Some("*"))
                }
                pub fn iter_metrics_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: AsRef<str>,
                {
                    self.fields = Some({
                        let mut selector = concat!("nextPageToken,", "metrics").to_owned();
                        let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                        if !items_fields.is_empty() {
                            selector.push_str("(");
                            selector.push_str(items_fields);
                            selector.push_str(")");
                        }
                        selector
                    });
                    crate::iter::PageItemIter::new(self, "metrics")
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
                ) -> crate::iter::PageIter<Self, crate::schemas::ListConsumerQuotaMetricsResponse>
                {
                    self.iter_with_fields(None::<&str>)
                }
                pub fn iter_with_all_fields(
                    self,
                ) -> crate::iter::PageIter<Self, crate::schemas::ListConsumerQuotaMetricsResponse>
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
                ) -> Result<crate::schemas::ListConsumerQuotaMetricsResponse, crate::Error>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::ListConsumerQuotaMetricsResponse, crate::Error>
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
                    let mut output = "https://serviceusage.googleapis.com/".to_owned();
                    output.push_str("v1beta1/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/consumerQuotaMetrics");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("pageSize", &self.page_size)]);
                    let req = req.query(&[("pageToken", &self.page_token)]);
                    let req = req.query(&[("view", &self.view)]);
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
            pub mod limits {
                pub mod params {
                    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                    pub enum GetView {
                        Basic,
                        Full,
                        QuotaViewUnspecified,
                    }
                    impl GetView {
                        pub fn as_str(self) -> &'static str {
                            match self {
                                GetView::Basic => "BASIC",
                                GetView::Full => "FULL",
                                GetView::QuotaViewUnspecified => "QUOTA_VIEW_UNSPECIFIED",
                            }
                        }
                    }
                    impl ::std::convert::AsRef<str> for GetView {
                        fn as_ref(&self) -> &str {
                            self.as_str()
                        }
                    }
                    impl ::std::str::FromStr for GetView {
                        type Err = ();
                        fn from_str(s: &str) -> ::std::result::Result<GetView, ()> {
                            Ok(match s {
                                "BASIC" => GetView::Basic,
                                "FULL" => GetView::Full,
                                "QUOTA_VIEW_UNSPECIFIED" => GetView::QuotaViewUnspecified,
                                _ => return Err(()),
                            })
                        }
                    }
                    impl ::std::fmt::Display for GetView {
                        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                            f.write_str(self.as_str())
                        }
                    }
                    impl ::serde::Serialize for GetView {
                        fn serialize<S>(
                            &self,
                            serializer: S,
                        ) -> ::std::result::Result<S::Ok, S::Error>
                        where
                            S: ::serde::ser::Serializer,
                        {
                            serializer.serialize_str(self.as_str())
                        }
                    }
                    impl<'de> ::serde::Deserialize<'de> for GetView {
                        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                        where
                            D: ::serde::de::Deserializer<'de>,
                        {
                            let value: &'de str = <&str>::deserialize(deserializer)?;
                            Ok(match value {
                                "BASIC" => GetView::Basic,
                                "FULL" => GetView::Full,
                                "QUOTA_VIEW_UNSPECIFIED" => GetView::QuotaViewUnspecified,
                                _ => {
                                    return Err(::serde::de::Error::custom(format!(
                                        "invalid enum for #name: {}",
                                        value
                                    )))
                                }
                            })
                        }
                    }
                    impl ::google_field_selector::FieldSelector for GetView {
                        fn fields() -> Vec<::google_field_selector::Field> {
                            Vec::new()
                        }
                    }
                    impl ::google_field_selector::ToFieldType for GetView {
                        fn field_type() -> ::google_field_selector::FieldType {
                            ::google_field_selector::FieldType::Leaf
                        }
                    }
                }
                pub struct LimitsActions<'a> {
                    pub(crate) reqwest: &'a reqwest::blocking::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                }
                impl<'a> LimitsActions<'a> {
                    fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                        self.auth
                    }
                    #[doc = "Retrieves a summary of quota information for a specific quota limit."]
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
                            view: None,
                        }
                    }
                    #[doc = "Actions that can be performed on the admin_overrides resource"]pub fn admin_overrides ( & self ) -> crate :: resources :: services :: consumer_quota_metrics :: limits :: admin_overrides :: AdminOverridesActions{
                        crate :: resources :: services :: consumer_quota_metrics :: limits :: admin_overrides :: AdminOverridesActions { reqwest : & self . reqwest , auth : self . auth_ref ( ) , }
                    }
                    #[doc = "Actions that can be performed on the consumer_overrides resource"]pub fn consumer_overrides ( & self ) -> crate :: resources :: services :: consumer_quota_metrics :: limits :: consumer_overrides :: ConsumerOverridesActions{
                        crate :: resources :: services :: consumer_quota_metrics :: limits :: consumer_overrides :: ConsumerOverridesActions { reqwest : & self . reqwest , auth : self . auth_ref ( ) , }
                    }
                }
                #[doc = "Created via [LimitsActions::get()](struct.LimitsActions.html#method.get)"]
                #[derive(Debug, Clone)]
                pub struct GetRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    name: String,
                    view: Option<
                        crate::resources::services::consumer_quota_metrics::limits::params::GetView,
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
                impl<'a> GetRequestBuilder<'a> {
                    #[doc = "Specifies the level of detail for quota information in the response."]
                    pub fn view(
                        mut self,
                        value : crate :: resources :: services :: consumer_quota_metrics :: limits :: params :: GetView,
                    ) -> Self {
                        self.view = Some(value);
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
                    ) -> Result<crate::schemas::ConsumerQuotaLimit, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::ConsumerQuotaLimit, crate::Error>
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
                        let mut output = "https://serviceusage.googleapis.com/".to_owned();
                        output.push_str("v1beta1/");
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
                        let req = req.query(&[("view", &self.view)]);
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
                pub mod admin_overrides {
                    pub mod params {}
                    pub struct AdminOverridesActions<'a> {
                        pub(crate) reqwest: &'a reqwest::blocking::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    }
                    impl<'a> AdminOverridesActions<'a> {
                        fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                            self.auth
                        }
                        #[doc = "Creates an admin override.\nAn admin override is applied by an administrator of a parent folder or\nparent organization of the consumer receiving the override. An admin\noverride is intended to limit the amount of quota the consumer can use out\nof the total quota pool allocated to all children of the folder or\norganization."]
                        pub fn create(
                            &self,
                            request: crate::schemas::QuotaOverride,
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
                                force: None,
                            }
                        }
                        #[doc = "Deletes an admin override."]
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
                                force: None,
                            }
                        }
                        #[doc = "Lists all admin overrides on this limit."]
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
                        #[doc = "Updates an admin override."]
                        pub fn patch(
                            &self,
                            request: crate::schemas::QuotaOverride,
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
                                force: None,
                                update_mask: None,
                            }
                        }
                    }
                    #[doc = "Created via [AdminOverridesActions::create()](struct.AdminOverridesActions.html#method.create)"]
                    #[derive(Debug, Clone)]
                    pub struct CreateRequestBuilder<'a> {
                        pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                        request: crate::schemas::QuotaOverride,
                        parent: String,
                        force: Option<bool>,
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
                        #[doc = "Whether to force the creation of the quota override.\nIf creating an override would cause the effective quota for the consumer to\ndecrease by more than 10 percent, the call is rejected, as a safety measure\nto avoid accidentally decreasing quota too quickly. Setting the force\nparameter to true ignores this restriction."]
                        pub fn force(mut self, value: bool) -> Self {
                            self.force = Some(value);
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
                        ) -> Result<crate::schemas::Operation, crate::Error>
                        {
                            self.execute_with_fields(None::<&str>)
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]
                        pub fn execute_with_all_fields(
                            self,
                        ) -> Result<crate::schemas::Operation, crate::Error>
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
                            let mut output = "https://serviceusage.googleapis.com/".to_owned();
                            output.push_str("v1beta1/");
                            {
                                let var_as_str = &self.parent;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::RESERVED,
                                ));
                            }
                            output.push_str("/adminOverrides");
                            output
                        }
                        fn _request(
                            &self,
                            path: &str,
                        ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error>
                        {
                            let req = self.reqwest.request(::reqwest::Method::POST, path);
                            let req = req.query(&[("force", &self.force)]);
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
                    #[doc = "Created via [AdminOverridesActions::delete()](struct.AdminOverridesActions.html#method.delete)"]
                    #[derive(Debug, Clone)]
                    pub struct DeleteRequestBuilder<'a> {
                        pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                        name: String,
                        force: Option<bool>,
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
                        #[doc = "Whether to force the deletion of the quota override.\nIf deleting an override would cause the effective quota for the consumer to\ndecrease by more than 10 percent, the call is rejected, as a safety measure\nto avoid accidentally decreasing quota too quickly. Setting the force\nparameter to true ignores this restriction."]
                        pub fn force(mut self, value: bool) -> Self {
                            self.force = Some(value);
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
                        ) -> Result<crate::schemas::Operation, crate::Error>
                        {
                            self.execute_with_fields(None::<&str>)
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]
                        pub fn execute_with_all_fields(
                            self,
                        ) -> Result<crate::schemas::Operation, crate::Error>
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
                            let mut output = "https://serviceusage.googleapis.com/".to_owned();
                            output.push_str("v1beta1/");
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
                            let req = req.query(&[("force", &self.force)]);
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
                    #[doc = "Created via [AdminOverridesActions::list()](struct.AdminOverridesActions.html#method.list)"]
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
                        #[doc = "Requested size of the next page of data."]
                        pub fn page_size(mut self, value: i32) -> Self {
                            self.page_size = Some(value);
                            self
                        }
                        #[doc = "Token identifying which result to start with; returned by a previous list\ncall."]
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
                        pub fn iter_overrides<T>(self) -> crate::iter::PageItemIter<Self, T>
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
                            self.iter_overrides_with_fields(fields)
                        }
                        #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                        #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                        #[doc = r" fields in `#items_type` will be the default fields populated by"]
                        #[doc = r" the server."]
                        pub fn iter_overrides_with_default_fields(
                            self,
                        ) -> crate::iter::PageItemIter<Self, crate::schemas::QuotaOverride>
                        {
                            self.iter_overrides_with_fields(None::<String>)
                        }
                        #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                        #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                        #[doc = r" fields in `#items_type` will be all fields available. This should"]
                        #[doc = r" primarily be used during developement and debugging as fetching"]
                        #[doc = r" all fields can be expensive both in bandwidth and server"]
                        #[doc = r" resources."]
                        pub fn iter_overrides_with_all_fields(
                            self,
                        ) -> crate::iter::PageItemIter<Self, crate::schemas::QuotaOverride>
                        {
                            self.iter_overrides_with_fields(Some("*"))
                        }
                        pub fn iter_overrides_with_fields<T, F>(
                            mut self,
                            fields: Option<F>,
                        ) -> crate::iter::PageItemIter<Self, T>
                        where
                            T: ::serde::de::DeserializeOwned,
                            F: AsRef<str>,
                        {
                            self.fields = Some({
                                let mut selector =
                                    concat!("nextPageToken,", "overrides").to_owned();
                                let items_fields =
                                    fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                                if !items_fields.is_empty() {
                                    selector.push_str("(");
                                    selector.push_str(items_fields);
                                    selector.push_str(")");
                                }
                                selector
                            });
                            crate::iter::PageItemIter::new(self, "overrides")
                        }
                        pub fn iter<T>(self) -> crate::iter::PageIter<Self, T>
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
                            self.iter_with_fields(fields)
                        }
                        pub fn iter_with_default_fields(
                            self,
                        ) -> crate::iter::PageIter<Self, crate::schemas::ListAdminOverridesResponse>
                        {
                            self.iter_with_fields(None::<&str>)
                        }
                        pub fn iter_with_all_fields(
                            self,
                        ) -> crate::iter::PageIter<Self, crate::schemas::ListAdminOverridesResponse>
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
                        ) -> Result<crate::schemas::ListAdminOverridesResponse, crate::Error>
                        {
                            self.execute_with_fields(None::<&str>)
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]
                        pub fn execute_with_all_fields(
                            self,
                        ) -> Result<crate::schemas::ListAdminOverridesResponse, crate::Error>
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
                            let mut output = "https://serviceusage.googleapis.com/".to_owned();
                            output.push_str("v1beta1/");
                            {
                                let var_as_str = &self.parent;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::RESERVED,
                                ));
                            }
                            output.push_str("/adminOverrides");
                            output
                        }
                        fn _request(
                            &self,
                            path: &str,
                        ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error>
                        {
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
                    #[doc = "Created via [AdminOverridesActions::patch()](struct.AdminOverridesActions.html#method.patch)"]
                    #[derive(Debug, Clone)]
                    pub struct PatchRequestBuilder<'a> {
                        pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                        request: crate::schemas::QuotaOverride,
                        name: String,
                        force: Option<bool>,
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
                        #[doc = "Whether to force the update of the quota override.\nIf updating an override would cause the effective quota for the consumer to\ndecrease by more than 10 percent, the call is rejected, as a safety measure\nto avoid accidentally decreasing quota too quickly. Setting the force\nparameter to true ignores this restriction."]
                        pub fn force(mut self, value: bool) -> Self {
                            self.force = Some(value);
                            self
                        }
                        #[doc = "Update only the specified fields of the override.\nIf unset, all fields will be updated."]
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
                        ) -> Result<crate::schemas::Operation, crate::Error>
                        {
                            self.execute_with_fields(None::<&str>)
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]
                        pub fn execute_with_all_fields(
                            self,
                        ) -> Result<crate::schemas::Operation, crate::Error>
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
                            let mut output = "https://serviceusage.googleapis.com/".to_owned();
                            output.push_str("v1beta1/");
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
                            let req = req.query(&[("force", &self.force)]);
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
                }
                pub mod consumer_overrides {
                    pub mod params {}
                    pub struct ConsumerOverridesActions<'a> {
                        pub(crate) reqwest: &'a reqwest::blocking::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    }
                    impl<'a> ConsumerOverridesActions<'a> {
                        fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                            self.auth
                        }
                        #[doc = "Creates a consumer override.\nA consumer override is applied to the consumer on its own authority to\nlimit its own quota usage. Consumer overrides cannot be used to grant more\nquota than would be allowed by admin overrides, producer overrides, or the\ndefault limit of the service."]
                        pub fn create(
                            &self,
                            request: crate::schemas::QuotaOverride,
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
                                force: None,
                            }
                        }
                        #[doc = "Deletes a consumer override."]
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
                                force: None,
                            }
                        }
                        #[doc = "Lists all consumer overrides on this limit."]
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
                        #[doc = "Updates a consumer override."]
                        pub fn patch(
                            &self,
                            request: crate::schemas::QuotaOverride,
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
                                force: None,
                                update_mask: None,
                            }
                        }
                    }
                    #[doc = "Created via [ConsumerOverridesActions::create()](struct.ConsumerOverridesActions.html#method.create)"]
                    #[derive(Debug, Clone)]
                    pub struct CreateRequestBuilder<'a> {
                        pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                        request: crate::schemas::QuotaOverride,
                        parent: String,
                        force: Option<bool>,
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
                        #[doc = "Whether to force the creation of the quota override.\nIf creating an override would cause the effective quota for the consumer to\ndecrease by more than 10 percent, the call is rejected, as a safety measure\nto avoid accidentally decreasing quota too quickly. Setting the force\nparameter to true ignores this restriction."]
                        pub fn force(mut self, value: bool) -> Self {
                            self.force = Some(value);
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
                        ) -> Result<crate::schemas::Operation, crate::Error>
                        {
                            self.execute_with_fields(None::<&str>)
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]
                        pub fn execute_with_all_fields(
                            self,
                        ) -> Result<crate::schemas::Operation, crate::Error>
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
                            let mut output = "https://serviceusage.googleapis.com/".to_owned();
                            output.push_str("v1beta1/");
                            {
                                let var_as_str = &self.parent;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::RESERVED,
                                ));
                            }
                            output.push_str("/consumerOverrides");
                            output
                        }
                        fn _request(
                            &self,
                            path: &str,
                        ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error>
                        {
                            let req = self.reqwest.request(::reqwest::Method::POST, path);
                            let req = req.query(&[("force", &self.force)]);
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
                    #[doc = "Created via [ConsumerOverridesActions::delete()](struct.ConsumerOverridesActions.html#method.delete)"]
                    #[derive(Debug, Clone)]
                    pub struct DeleteRequestBuilder<'a> {
                        pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                        name: String,
                        force: Option<bool>,
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
                        #[doc = "Whether to force the deletion of the quota override.\nIf deleting an override would cause the effective quota for the consumer to\ndecrease by more than 10 percent, the call is rejected, as a safety measure\nto avoid accidentally decreasing quota too quickly. Setting the force\nparameter to true ignores this restriction."]
                        pub fn force(mut self, value: bool) -> Self {
                            self.force = Some(value);
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
                        ) -> Result<crate::schemas::Operation, crate::Error>
                        {
                            self.execute_with_fields(None::<&str>)
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]
                        pub fn execute_with_all_fields(
                            self,
                        ) -> Result<crate::schemas::Operation, crate::Error>
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
                            let mut output = "https://serviceusage.googleapis.com/".to_owned();
                            output.push_str("v1beta1/");
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
                            let req = req.query(&[("force", &self.force)]);
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
                    #[doc = "Created via [ConsumerOverridesActions::list()](struct.ConsumerOverridesActions.html#method.list)"]
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
                        #[doc = "Requested size of the next page of data."]
                        pub fn page_size(mut self, value: i32) -> Self {
                            self.page_size = Some(value);
                            self
                        }
                        #[doc = "Token identifying which result to start with; returned by a previous list\ncall."]
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
                        pub fn iter_overrides<T>(self) -> crate::iter::PageItemIter<Self, T>
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
                            self.iter_overrides_with_fields(fields)
                        }
                        #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                        #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                        #[doc = r" fields in `#items_type` will be the default fields populated by"]
                        #[doc = r" the server."]
                        pub fn iter_overrides_with_default_fields(
                            self,
                        ) -> crate::iter::PageItemIter<Self, crate::schemas::QuotaOverride>
                        {
                            self.iter_overrides_with_fields(None::<String>)
                        }
                        #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                        #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                        #[doc = r" fields in `#items_type` will be all fields available. This should"]
                        #[doc = r" primarily be used during developement and debugging as fetching"]
                        #[doc = r" all fields can be expensive both in bandwidth and server"]
                        #[doc = r" resources."]
                        pub fn iter_overrides_with_all_fields(
                            self,
                        ) -> crate::iter::PageItemIter<Self, crate::schemas::QuotaOverride>
                        {
                            self.iter_overrides_with_fields(Some("*"))
                        }
                        pub fn iter_overrides_with_fields<T, F>(
                            mut self,
                            fields: Option<F>,
                        ) -> crate::iter::PageItemIter<Self, T>
                        where
                            T: ::serde::de::DeserializeOwned,
                            F: AsRef<str>,
                        {
                            self.fields = Some({
                                let mut selector =
                                    concat!("nextPageToken,", "overrides").to_owned();
                                let items_fields =
                                    fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                                if !items_fields.is_empty() {
                                    selector.push_str("(");
                                    selector.push_str(items_fields);
                                    selector.push_str(")");
                                }
                                selector
                            });
                            crate::iter::PageItemIter::new(self, "overrides")
                        }
                        pub fn iter<T>(self) -> crate::iter::PageIter<Self, T>
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
                            self.iter_with_fields(fields)
                        }
                        pub fn iter_with_default_fields(
                            self,
                        ) -> crate::iter::PageIter<
                            Self,
                            crate::schemas::ListConsumerOverridesResponse,
                        > {
                            self.iter_with_fields(None::<&str>)
                        }
                        pub fn iter_with_all_fields(
                            self,
                        ) -> crate::iter::PageIter<
                            Self,
                            crate::schemas::ListConsumerOverridesResponse,
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
                        ) -> Result<crate::schemas::ListConsumerOverridesResponse, crate::Error>
                        {
                            self.execute_with_fields(None::<&str>)
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]
                        pub fn execute_with_all_fields(
                            self,
                        ) -> Result<crate::schemas::ListConsumerOverridesResponse, crate::Error>
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
                            let mut output = "https://serviceusage.googleapis.com/".to_owned();
                            output.push_str("v1beta1/");
                            {
                                let var_as_str = &self.parent;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::RESERVED,
                                ));
                            }
                            output.push_str("/consumerOverrides");
                            output
                        }
                        fn _request(
                            &self,
                            path: &str,
                        ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error>
                        {
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
                    #[doc = "Created via [ConsumerOverridesActions::patch()](struct.ConsumerOverridesActions.html#method.patch)"]
                    #[derive(Debug, Clone)]
                    pub struct PatchRequestBuilder<'a> {
                        pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                        request: crate::schemas::QuotaOverride,
                        name: String,
                        force: Option<bool>,
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
                        #[doc = "Whether to force the update of the quota override.\nIf updating an override would cause the effective quota for the consumer to\ndecrease by more than 10 percent, the call is rejected, as a safety measure\nto avoid accidentally decreasing quota too quickly. Setting the force\nparameter to true ignores this restriction."]
                        pub fn force(mut self, value: bool) -> Self {
                            self.force = Some(value);
                            self
                        }
                        #[doc = "Update only the specified fields of the override.\nIf unset, all fields will be updated."]
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
                        ) -> Result<crate::schemas::Operation, crate::Error>
                        {
                            self.execute_with_fields(None::<&str>)
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]
                        pub fn execute_with_all_fields(
                            self,
                        ) -> Result<crate::schemas::Operation, crate::Error>
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
                            let mut output = "https://serviceusage.googleapis.com/".to_owned();
                            output.push_str("v1beta1/");
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
                            let req = req.query(&[("force", &self.force)]);
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
