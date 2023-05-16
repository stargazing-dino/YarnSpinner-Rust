// @generated
# [clippy :: msrv = "1.61"] mod fallback ; # [clippy :: msrv = "1.61"] mod plurals ; # [clippy :: msrv = "1.61"] use :: icu_provider :: prelude :: * ; # [doc = r" Implement [`DataProvider<M>`] on the given struct using the data"] # [doc = r" hardcoded in this module. This allows the struct to be used with"] # [doc = r" `icu`'s `_unstable` constructors."] # [doc = r""] # [doc = r" This macro can only be called from its definition-site, i.e. right"] # [doc = r" after `include!`-ing the generated module."] # [doc = r""] # [doc = r" ```compile_fail"] # [doc = r" struct MyDataProvider;"] # [doc = r#" include!("/path/to/generated/mod.rs");"#] # [doc = r" impl_data_provider(MyDataProvider);"] # [doc = r" ```"] # [allow (unused_macros)] macro_rules ! impl_data_provider { ($ provider : path) => { # [clippy :: msrv = "1.61"] impl DataProvider < :: icu_plurals :: provider :: CardinalV1Marker > for $ provider { fn load (& self , req : DataRequest ,) -> Result < DataResponse < :: icu_plurals :: provider :: CardinalV1Marker > , DataError > { plurals :: cardinal_v1 :: lookup (& req . locale) . map (zerofrom :: ZeroFrom :: zero_from) . map (DataPayload :: from_owned) . map (| payload | { DataResponse { metadata : Default :: default () , payload : Some (payload) , } }) . ok_or_else (|| DataErrorKind :: MissingLocale . with_req (:: icu_plurals :: provider :: CardinalV1Marker :: KEY , req)) } } # [clippy :: msrv = "1.61"] impl DataProvider < :: icu_plurals :: provider :: OrdinalV1Marker > for $ provider { fn load (& self , req : DataRequest ,) -> Result < DataResponse < :: icu_plurals :: provider :: OrdinalV1Marker > , DataError > { plurals :: ordinal_v1 :: lookup (& req . locale) . map (zerofrom :: ZeroFrom :: zero_from) . map (DataPayload :: from_owned) . map (| payload | { DataResponse { metadata : Default :: default () , payload : Some (payload) , } }) . ok_or_else (|| DataErrorKind :: MissingLocale . with_req (:: icu_plurals :: provider :: OrdinalV1Marker :: KEY , req)) } } # [clippy :: msrv = "1.61"] impl DataProvider < :: icu_provider_adapters :: fallback :: provider :: CollationFallbackSupplementV1Marker > for $ provider { fn load (& self , req : DataRequest ,) -> Result < DataResponse < :: icu_provider_adapters :: fallback :: provider :: CollationFallbackSupplementV1Marker > , DataError > { fallback :: supplement :: co_v1 :: lookup (& req . locale) . map (zerofrom :: ZeroFrom :: zero_from) . map (DataPayload :: from_owned) . map (| payload | { DataResponse { metadata : Default :: default () , payload : Some (payload) , } }) . ok_or_else (|| DataErrorKind :: MissingLocale . with_req (:: icu_provider_adapters :: fallback :: provider :: CollationFallbackSupplementV1Marker :: KEY , req)) } } # [clippy :: msrv = "1.61"] impl DataProvider < :: icu_provider_adapters :: fallback :: provider :: LocaleFallbackLikelySubtagsV1Marker > for $ provider { fn load (& self , req : DataRequest ,) -> Result < DataResponse < :: icu_provider_adapters :: fallback :: provider :: LocaleFallbackLikelySubtagsV1Marker > , DataError > { fallback :: likelysubtags_v1 :: lookup (& req . locale) . map (zerofrom :: ZeroFrom :: zero_from) . map (DataPayload :: from_owned) . map (| payload | { DataResponse { metadata : Default :: default () , payload : Some (payload) , } }) . ok_or_else (|| DataErrorKind :: MissingLocale . with_req (:: icu_provider_adapters :: fallback :: provider :: LocaleFallbackLikelySubtagsV1Marker :: KEY , req)) } } # [clippy :: msrv = "1.61"] impl DataProvider < :: icu_provider_adapters :: fallback :: provider :: LocaleFallbackParentsV1Marker > for $ provider { fn load (& self , req : DataRequest ,) -> Result < DataResponse < :: icu_provider_adapters :: fallback :: provider :: LocaleFallbackParentsV1Marker > , DataError > { fallback :: parents_v1 :: lookup (& req . locale) . map (zerofrom :: ZeroFrom :: zero_from) . map (DataPayload :: from_owned) . map (| payload | { DataResponse { metadata : Default :: default () , payload : Some (payload) , } }) . ok_or_else (|| DataErrorKind :: MissingLocale . with_req (:: icu_provider_adapters :: fallback :: provider :: LocaleFallbackParentsV1Marker :: KEY , req)) } } } } # [doc = r" Implement [`AnyProvider`] on the given struct using the data"] # [doc = r" hardcoded in this module. This allows the struct to be used with"] # [doc = r" `icu`'s `_any` constructors."] # [doc = r""] # [doc = r" This macro can only be called from its definition-site, i.e. right"] # [doc = r" after `include!`-ing the generated module."] # [doc = r" "] # [doc = r" ```compile_fail"] # [doc = r" struct MyAnyProvider;"] # [doc = r#" include!("/path/to/generated/mod.rs");"#] # [doc = r" impl_any_provider(MyAnyProvider);"] # [doc = r" ```"] # [allow (unused_macros)] macro_rules ! impl_any_provider { ($ provider : path) => { # [clippy :: msrv = "1.61"] impl AnyProvider for $ provider { fn load_any (& self , key : DataKey , req : DataRequest) -> Result < AnyResponse , DataError > { const CARDINALV1MARKER : :: icu_provider :: DataKeyHash = :: icu_plurals :: provider :: CardinalV1Marker :: KEY . hashed () ; const ORDINALV1MARKER : :: icu_provider :: DataKeyHash = :: icu_plurals :: provider :: OrdinalV1Marker :: KEY . hashed () ; const COLLATIONFALLBACKSUPPLEMENTV1MARKER : :: icu_provider :: DataKeyHash = :: icu_provider_adapters :: fallback :: provider :: CollationFallbackSupplementV1Marker :: KEY . hashed () ; const LOCALEFALLBACKLIKELYSUBTAGSV1MARKER : :: icu_provider :: DataKeyHash = :: icu_provider_adapters :: fallback :: provider :: LocaleFallbackLikelySubtagsV1Marker :: KEY . hashed () ; const LOCALEFALLBACKPARENTSV1MARKER : :: icu_provider :: DataKeyHash = :: icu_provider_adapters :: fallback :: provider :: LocaleFallbackParentsV1Marker :: KEY . hashed () ; match key . hashed () { CARDINALV1MARKER => plurals :: cardinal_v1 :: lookup (& req . locale) . map (AnyPayload :: from_static_ref) , ORDINALV1MARKER => plurals :: ordinal_v1 :: lookup (& req . locale) . map (AnyPayload :: from_static_ref) , COLLATIONFALLBACKSUPPLEMENTV1MARKER => fallback :: supplement :: co_v1 :: lookup (& req . locale) . map (AnyPayload :: from_static_ref) , LOCALEFALLBACKLIKELYSUBTAGSV1MARKER => fallback :: likelysubtags_v1 :: lookup (& req . locale) . map (AnyPayload :: from_static_ref) , LOCALEFALLBACKPARENTSV1MARKER => fallback :: parents_v1 :: lookup (& req . locale) . map (AnyPayload :: from_static_ref) , _ => return Err (DataErrorKind :: MissingDataKey . with_req (key , req)) , } . map (| payload | AnyResponse { payload : Some (payload) , metadata : Default :: default () , }) . ok_or_else (|| DataErrorKind :: MissingLocale . with_req (key , req)) } } } } # [clippy :: msrv = "1.61"] pub struct BakedDataProvider ; impl_data_provider ! (BakedDataProvider) ;