use aorist_primitives::{define_attribute, register_attribute,
                        TAttribute, TOrcAttribute, TPrestoAttribute, TSQLAttribute};
use aorist_derive::{
    OrcBigint, OrcFloat, OrcString, PrestoBigint, PrestoReal, PrestoVarchar, SQLBigint, SQLReal,
    SQLVarchar,
};
use serde::{Deserialize, Serialize};
use sqlparser::ast::{DataType};

define_attribute!(KeyStringIdentifier, PrestoVarchar, OrcString, SQLVarchar);
define_attribute!(
    NullableStringIdentifier,
    PrestoVarchar,
    OrcString,
    SQLVarchar
);
define_attribute!(NullablePOSIXTimestamp, PrestoBigint, OrcBigint, SQLBigint);
define_attribute!(NullableInt64, PrestoBigint, OrcBigint, SQLBigint);
define_attribute!(NullableString, PrestoVarchar, OrcString, SQLVarchar);
define_attribute!(FloatLatitude, PrestoReal, OrcFloat, SQLReal);
define_attribute!(FloatLongitude, PrestoReal, OrcFloat, SQLReal);
define_attribute!(URI, PrestoVarchar, OrcString, SQLVarchar);
register_attribute!(
    Attribute,
    KeyStringIdentifier,
    NullableStringIdentifier,
    NullablePOSIXTimestamp,
    NullableInt64,
    NullableString,
    FloatLatitude,
    FloatLongitude,
    URI
);
