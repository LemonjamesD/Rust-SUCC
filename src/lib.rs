use serde::{ser::{Serializer, SerializeSeq, SerializeTuple, SerializeTupleStruct, SerializeTupleVariant, SerializeMap, SerializeStruct, SerializeStructVariant}, Serialize};

mod error_handler;
use error_handler::{Result, Error};

mod serializer;
use serializer::*;