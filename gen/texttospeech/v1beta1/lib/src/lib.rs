#![doc = "# Resources and Methods\n    * [text](resources/text/struct.TextActions.html)\n      * [*synthesize*](resources/text/struct.SynthesizeRequestBuilder.html)\n    * [voices](resources/voices/struct.VoicesActions.html)\n      * [*list*](resources/voices/struct.ListRequestBuilder.html)\n"]
pub mod schemas {
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct AudioConfig {
        #[doc = "Required. The format of the audio byte stream."]
        #[serde(
            rename = "audioEncoding",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub audio_encoding: ::std::option::Option<crate::schemas::AudioConfigAudioEncoding>,
        #[doc = "Optional. Input only. An identifier which selects 'audio effects' profiles\nthat are applied on (post synthesized) text to speech. Effects are applied\non top of each other in the order they are given. See\n[audio\nprofiles](https://cloud.google.com/text-to-speech/docs/audio-profiles) for\ncurrent supported profile ids."]
        #[serde(
            rename = "effectsProfileId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub effects_profile_id: ::std::option::Option<Vec<String>>,
        #[doc = "Optional. Input only. Speaking pitch, in the range [-20.0, 20.0]. 20 means\nincrease 20 semitones from the original pitch. -20 means decrease 20\nsemitones from the original pitch."]
        #[serde(
            rename = "pitch",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub pitch: ::std::option::Option<f64>,
        #[doc = "The synthesis sample rate (in hertz) for this audio. Optional. When this is\nspecified in SynthesizeSpeechRequest, if this is different from the voice's\nnatural sample rate, then the synthesizer will honor this request by\nconverting to the desired sample rate (which might result in worse audio\nquality), unless the specified sample rate is not supported for the\nencoding chosen, in which case it will fail the request and return\ngoogle.rpc.Code.INVALID_ARGUMENT."]
        #[serde(
            rename = "sampleRateHertz",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sample_rate_hertz: ::std::option::Option<i32>,
        #[doc = "Optional. Input only. Speaking rate/speed, in the range [0.25, 4.0]. 1.0 is\nthe normal native speed supported by the specific voice. 2.0 is twice as\nfast, and 0.5 is half as fast. If unset(0.0), defaults to the native 1.0\nspeed. Any other values < 0.25 or > 4.0 will return an error."]
        #[serde(
            rename = "speakingRate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub speaking_rate: ::std::option::Option<f64>,
        #[doc = "Optional. Input only. Volume gain (in dB) of the normal native volume\nsupported by the specific voice, in the range [-96.0, 16.0]. If unset, or\nset to a value of 0.0 (dB), will play at normal native signal amplitude. A\nvalue of -6.0 (dB) will play at approximately half the amplitude of the\nnormal native signal amplitude. A value of +6.0 (dB) will play at\napproximately twice the amplitude of the normal native signal amplitude.\nStrongly recommend not to exceed +10 (dB) as there's usually no effective\nincrease in loudness for any value greater than that."]
        #[serde(
            rename = "volumeGainDb",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub volume_gain_db: ::std::option::Option<f64>,
    }
    impl ::google_field_selector::FieldSelector for AudioConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AudioConfig {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum AudioConfigAudioEncoding {
        #[doc = "Not specified. Will return result google.rpc.Code.INVALID_ARGUMENT."]
        AudioEncodingUnspecified,
        #[doc = "Uncompressed 16-bit signed little-endian samples (Linear PCM).\nAudio content returned as LINEAR16 also contains a WAV header."]
        Linear16,
        #[doc = "MP3 audio at 32kbps."]
        Mp3,
        #[doc = "Opus encoded audio wrapped in an ogg container. The result will be a\nfile which can be played natively on Android, and in browsers (at least\nChrome and Firefox). The quality of the encoding is considerably higher\nthan MP3 while using approximately the same bitrate."]
        OggOpus,
    }
    impl AudioConfigAudioEncoding {
        pub fn as_str(self) -> &'static str {
            match self {
                AudioConfigAudioEncoding::AudioEncodingUnspecified => "AUDIO_ENCODING_UNSPECIFIED",
                AudioConfigAudioEncoding::Linear16 => "LINEAR16",
                AudioConfigAudioEncoding::Mp3 => "MP3",
                AudioConfigAudioEncoding::OggOpus => "OGG_OPUS",
            }
        }
    }
    impl ::std::convert::AsRef<str> for AudioConfigAudioEncoding {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for AudioConfigAudioEncoding {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<AudioConfigAudioEncoding, ()> {
            Ok(match s {
                "AUDIO_ENCODING_UNSPECIFIED" => AudioConfigAudioEncoding::AudioEncodingUnspecified,
                "LINEAR16" => AudioConfigAudioEncoding::Linear16,
                "MP3" => AudioConfigAudioEncoding::Mp3,
                "OGG_OPUS" => AudioConfigAudioEncoding::OggOpus,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for AudioConfigAudioEncoding {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AudioConfigAudioEncoding {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AudioConfigAudioEncoding {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "AUDIO_ENCODING_UNSPECIFIED" => AudioConfigAudioEncoding::AudioEncodingUnspecified,
                "LINEAR16" => AudioConfigAudioEncoding::Linear16,
                "MP3" => AudioConfigAudioEncoding::Mp3,
                "OGG_OPUS" => AudioConfigAudioEncoding::OggOpus,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for AudioConfigAudioEncoding {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AudioConfigAudioEncoding {
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
    pub struct ListVoicesResponse {
        #[doc = "The list of voices."]
        #[serde(
            rename = "voices",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub voices: ::std::option::Option<Vec<crate::schemas::Voice>>,
    }
    impl ::google_field_selector::FieldSelector for ListVoicesResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListVoicesResponse {
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
    pub struct SynthesisInput {
        #[doc = "The SSML document to be synthesized. The SSML document must be valid\nand well-formed. Otherwise the RPC will fail and return\ngoogle.rpc.Code.INVALID_ARGUMENT. For more information, see\n[SSML](/speech/text-to-speech/docs/ssml)."]
        #[serde(
            rename = "ssml",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ssml: ::std::option::Option<String>,
        #[doc = "The raw text to be synthesized."]
        #[serde(
            rename = "text",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for SynthesisInput {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SynthesisInput {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct SynthesizeSpeechRequest {
        #[doc = "Required. The configuration of the synthesized audio."]
        #[serde(
            rename = "audioConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub audio_config: ::std::option::Option<crate::schemas::AudioConfig>,
        #[doc = "Required. The Synthesizer requires either plain text or SSML as input."]
        #[serde(
            rename = "input",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub input: ::std::option::Option<crate::schemas::SynthesisInput>,
        #[doc = "Required. The desired voice of the synthesized audio."]
        #[serde(
            rename = "voice",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub voice: ::std::option::Option<crate::schemas::VoiceSelectionParams>,
    }
    impl ::google_field_selector::FieldSelector for SynthesizeSpeechRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SynthesizeSpeechRequest {
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
    pub struct SynthesizeSpeechResponse {
        #[doc = "The audio data bytes encoded as specified in the request, including the\nheader for encodings that are wrapped in containers (e.g. MP3, OGG_OPUS).\nFor LINEAR16 audio, we include the WAV header. Note: as\nwith all bytes fields, protobuffers use a pure binary representation,\nwhereas JSON representations use base64."]
        #[serde(
            rename = "audioContent",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub audio_content: ::std::option::Option<::google_api_bytes::Bytes>,
    }
    impl ::google_field_selector::FieldSelector for SynthesizeSpeechResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SynthesizeSpeechResponse {
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
    pub struct Voice {
        #[doc = "The languages that this voice supports, expressed as\n[BCP-47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt) language tags (e.g.\n\"en-US\", \"es-419\", \"cmn-tw\")."]
        #[serde(
            rename = "languageCodes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub language_codes: ::std::option::Option<Vec<String>>,
        #[doc = "The name of this voice.  Each distinct voice has a unique name."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The natural sample rate (in hertz) for this voice."]
        #[serde(
            rename = "naturalSampleRateHertz",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub natural_sample_rate_hertz: ::std::option::Option<i32>,
        #[doc = "The gender of this voice."]
        #[serde(
            rename = "ssmlGender",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ssml_gender: ::std::option::Option<crate::schemas::VoiceSsmlGender>,
    }
    impl ::google_field_selector::FieldSelector for Voice {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Voice {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum VoiceSsmlGender {
        #[doc = "A female voice."]
        Female,
        #[doc = "A male voice."]
        Male,
        #[doc = "A gender-neutral voice."]
        Neutral,
        #[doc = "An unspecified gender.\nIn VoiceSelectionParams, this means that the client doesn't care which\ngender the selected voice will have. In the Voice field of\nListVoicesResponse, this may mean that the voice doesn't fit any of the\nother categories in this enum, or that the gender of the voice isn't known."]
        SsmlVoiceGenderUnspecified,
    }
    impl VoiceSsmlGender {
        pub fn as_str(self) -> &'static str {
            match self {
                VoiceSsmlGender::Female => "FEMALE",
                VoiceSsmlGender::Male => "MALE",
                VoiceSsmlGender::Neutral => "NEUTRAL",
                VoiceSsmlGender::SsmlVoiceGenderUnspecified => "SSML_VOICE_GENDER_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for VoiceSsmlGender {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for VoiceSsmlGender {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<VoiceSsmlGender, ()> {
            Ok(match s {
                "FEMALE" => VoiceSsmlGender::Female,
                "MALE" => VoiceSsmlGender::Male,
                "NEUTRAL" => VoiceSsmlGender::Neutral,
                "SSML_VOICE_GENDER_UNSPECIFIED" => VoiceSsmlGender::SsmlVoiceGenderUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for VoiceSsmlGender {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for VoiceSsmlGender {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for VoiceSsmlGender {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "FEMALE" => VoiceSsmlGender::Female,
                "MALE" => VoiceSsmlGender::Male,
                "NEUTRAL" => VoiceSsmlGender::Neutral,
                "SSML_VOICE_GENDER_UNSPECIFIED" => VoiceSsmlGender::SsmlVoiceGenderUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for VoiceSsmlGender {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for VoiceSsmlGender {
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
    pub struct VoiceSelectionParams {
        #[doc = "The language (and optionally also the region) of the voice expressed as a\n[BCP-47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt) language tag, e.g.\n\"en-US\". Required. This should not include a script tag (e.g. use\n\"cmn-cn\" rather than \"cmn-Hant-cn\"), because the script will be inferred\nfrom the input provided in the SynthesisInput.  The TTS service\nwill use this parameter to help choose an appropriate voice.  Note that\nthe TTS service may choose a voice with a slightly different language code\nthan the one selected; it may substitute a different region\n(e.g. using en-US rather than en-CA if there isn't a Canadian voice\navailable), or even a different language, e.g. using \"nb\" (Norwegian\nBokmal) instead of \"no\" (Norwegian)\"."]
        #[serde(
            rename = "languageCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub language_code: ::std::option::Option<String>,
        #[doc = "The name of the voice. Optional; if not set, the service will choose a\nvoice based on the other parameters such as language_code and gender."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The preferred gender of the voice. Optional; if not set, the service will\nchoose a voice based on the other parameters such as language_code and\nname. Note that this is only a preference, not requirement; if a\nvoice of the appropriate gender is not available, the synthesizer should\nsubstitute a voice with a different gender rather than failing the request."]
        #[serde(
            rename = "ssmlGender",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ssml_gender: ::std::option::Option<crate::schemas::VoiceSelectionParamsSsmlGender>,
    }
    impl ::google_field_selector::FieldSelector for VoiceSelectionParams {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for VoiceSelectionParams {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum VoiceSelectionParamsSsmlGender {
        #[doc = "A female voice."]
        Female,
        #[doc = "A male voice."]
        Male,
        #[doc = "A gender-neutral voice."]
        Neutral,
        #[doc = "An unspecified gender.\nIn VoiceSelectionParams, this means that the client doesn't care which\ngender the selected voice will have. In the Voice field of\nListVoicesResponse, this may mean that the voice doesn't fit any of the\nother categories in this enum, or that the gender of the voice isn't known."]
        SsmlVoiceGenderUnspecified,
    }
    impl VoiceSelectionParamsSsmlGender {
        pub fn as_str(self) -> &'static str {
            match self {
                VoiceSelectionParamsSsmlGender::Female => "FEMALE",
                VoiceSelectionParamsSsmlGender::Male => "MALE",
                VoiceSelectionParamsSsmlGender::Neutral => "NEUTRAL",
                VoiceSelectionParamsSsmlGender::SsmlVoiceGenderUnspecified => {
                    "SSML_VOICE_GENDER_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for VoiceSelectionParamsSsmlGender {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for VoiceSelectionParamsSsmlGender {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<VoiceSelectionParamsSsmlGender, ()> {
            Ok(match s {
                "FEMALE" => VoiceSelectionParamsSsmlGender::Female,
                "MALE" => VoiceSelectionParamsSsmlGender::Male,
                "NEUTRAL" => VoiceSelectionParamsSsmlGender::Neutral,
                "SSML_VOICE_GENDER_UNSPECIFIED" => {
                    VoiceSelectionParamsSsmlGender::SsmlVoiceGenderUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for VoiceSelectionParamsSsmlGender {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for VoiceSelectionParamsSsmlGender {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for VoiceSelectionParamsSsmlGender {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "FEMALE" => VoiceSelectionParamsSsmlGender::Female,
                "MALE" => VoiceSelectionParamsSsmlGender::Male,
                "NEUTRAL" => VoiceSelectionParamsSsmlGender::Neutral,
                "SSML_VOICE_GENDER_UNSPECIFIED" => {
                    VoiceSelectionParamsSsmlGender::SsmlVoiceGenderUnspecified
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
    impl ::google_field_selector::FieldSelector for VoiceSelectionParamsSsmlGender {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for VoiceSelectionParamsSsmlGender {
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
    reqwest: ::reqwest::Client,
    auth: Box<dyn ::google_api_auth::GetAccessToken>,
}
impl Client {
    pub fn new<A>(auth: A) -> Self
    where
        A: Into<Box<dyn ::google_api_auth::GetAccessToken>>,
    {
        Client {
            reqwest: ::reqwest::Client::builder().timeout(None).build().unwrap(),
            auth: auth.into(),
        }
    }
    fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
        self.auth.as_ref()
    }
    #[doc = "Actions that can be performed on the text resource"]
    pub fn text(&self) -> crate::resources::text::TextActions {
        crate::resources::text::TextActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the voices resource"]
    pub fn voices(&self) -> crate::resources::voices::VoicesActions {
        crate::resources::voices::VoicesActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
}
pub mod resources {
    pub mod text {
        pub mod params {}
        pub struct TextActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> TextActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Synthesizes speech synchronously: receive results after all text input\nhas been processed."]
            pub fn synthesize(
                &self,
                request: crate::schemas::SynthesizeSpeechRequest,
            ) -> SynthesizeRequestBuilder {
                SynthesizeRequestBuilder {
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
        }
        #[doc = "Created via [TextActions::synthesize()](struct.TextActions.html#method.synthesize)"]
        #[derive(Debug, Clone)]
        pub struct SynthesizeRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::SynthesizeSpeechRequest,
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
        impl<'a> SynthesizeRequestBuilder<'a> {
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
            ) -> Result<crate::schemas::SynthesizeSpeechResponse, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::SynthesizeSpeechResponse, crate::Error> {
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
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://texttospeech.googleapis.com/".to_owned();
                output.push_str("v1beta1/text:synthesize");
                output
            }
            fn _request(&self, path: &str) -> Result<::reqwest::RequestBuilder, crate::Error> {
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
    pub mod voices {
        pub mod params {}
        pub struct VoicesActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> VoicesActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Returns a list of Voice supported for synthesis."]
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
                    language_code: None,
                }
            }
        }
        #[doc = "Created via [VoicesActions::list()](struct.VoicesActions.html#method.list)"]
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            language_code: Option<String>,
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
            #[doc = "Optional (but recommended)\n[BCP-47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt) language tag. If\nspecified, the ListVoices call will only return voices that can be used to\nsynthesize this language_code. E.g. when specifying \"en-NZ\", you will get\nsupported \"en-*\" voices; when specifying \"no\", you will get supported\n\"no-*\" (Norwegian) and \"nb-*\" (Norwegian Bokmal) voices; specifying \"zh\"\nwill also get supported \"cmn-*\" voices; specifying \"zh-hk\" will also get\nsupported \"yue-*\" voices."]
            pub fn language_code(mut self, value: impl Into<String>) -> Self {
                self.language_code = Some(value.into());
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
            ) -> Result<crate::schemas::ListVoicesResponse, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::ListVoicesResponse, crate::Error> {
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
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://texttospeech.googleapis.com/".to_owned();
                output.push_str("v1beta1/voices");
                output
            }
            fn _request(&self, path: &str) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("languageCode", &self.language_code)]);
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
#[derive(Debug)]
pub enum Error {
    OAuth2(Box<dyn ::std::error::Error + Send + Sync>),
    JSON(::serde_json::Error),
    Reqwest(::reqwest::Error),
    Other(Box<dyn ::std::error::Error + Send + Sync>),
}

impl Error {
    pub fn json_error(&self) -> Option<&::serde_json::Error> {
        match self {
            Error::OAuth2(_) => None,
            Error::JSON(err) => Some(err),
            Error::Reqwest(err) => err
                .get_ref()
                .and_then(|err| err.downcast_ref::<::serde_json::Error>()),
            Error::Other(_) => None,
        }
    }
}

impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            Error::OAuth2(err) => write!(f, "OAuth2 Error: {}", err),
            Error::JSON(err) => write!(f, "JSON Error: {}", err),
            Error::Reqwest(err) => write!(f, "Reqwest Error: {}", err),
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
    fn from(err: ::reqwest::Error) -> Error {
        Error::Reqwest(err)
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
