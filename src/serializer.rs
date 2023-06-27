use serde::{ser::{Serializer, SerializeSeq, SerializeTuple, SerializeTupleStruct, SerializeTupleVariant, SerializeMap, SerializeStruct, SerializeStructVariant}, Serialize};

pub enum BoolStyle {
    TrueFalse,
    OnOff,
    YesNo,
    YN,
}

impl Default for BoolStyle {
    fn default() -> Self {
        Self::TrueFalse
    }
}

impl BoolStyle {
    pub fn to_string(&self, value: bool) -> String {
        let bools = match self {
            BoolStyle::TrueFalse => ["true", "false"],
            BoolStyle::OnOff => ["on", "off"],
            BoolStyle::YesNo => ["yes", "no"],
            BoolStyle::YN => ["y", "n"]
        };
        let idx = if value { 0 } else { 1 };
        bools[idx].to_string()
    }
}

#[derive(Default)]
pub struct SUCCSerializer {
    bool_style: BoolStyle,
    output: String,
    tab: String,

    field_name: String,
    unindent_flag: bool,
    field_flag: bool,
    raw_flag: bool,
}

impl SUCCSerializer {
    pub fn increase_tab(&mut self) {
        self.tab += "    ";
    }
    pub fn decrease_tab(&mut self) {
        if self.tab.is_empty() {
            return;
        }
        let mut chars = self.tab.as_str();
        let chars = &chars[0..(chars.len()-4)];
        self.tab = chars.to_string();
    }

    pub fn add_tab(&mut self) {
        if !self.raw_flag {
            self.output += &self.tab;
        }
    }
    pub fn add_newline(&mut self) {
        if !self.raw_flag {
            self.output += "\n";
        }
    }
    pub fn colon_or(&mut self) {
        if !self.raw_flag {
            if self.field_flag {
                self.output += &self.field_name;
                self.output += ": ";
            } else {
                self.output += "- ";
            }
        }
    }
}

pub fn to_string<T>(value: &T) -> Result<String>
where
    T: Serialize
{
    let mut serializer = SUCCSerializer {
        output: String::new(),
        field_flag: true,
        ..Default::default()
    };
    value.serialize(&mut serializer)?;
    Ok(serializer.output)
}

impl<'a> Serializer for &'a mut SUCCSerializer {
    type Ok = ();

    type Error = Error;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<()> {
        self.output += &self.bool_style.to_string(v);

        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i16(self, v: i16) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i32(self, v: i32) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i64(self, v: i64) -> Result<()> {
        self.add_tab();
        if self.raw_flag {
            self.output += &v.to_string();
            self.add_newline();
        } else {
            self.colon_or();
            self.output += &v.to_string();
            self.add_newline();
        }
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u16(self, v: u16) -> Result<()> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u32(self, v: u32) -> Result<()> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u64(self, v: u64) -> Result<()> {
        self.add_tab();
        if self.raw_flag {
            self.output += &v.to_string();
        } else {
            self.colon_or();
            self.output += &v.to_string();
        }
        self.add_newline();
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<()> {
        self.serialize_f64(f64::from(v))
    }

    fn serialize_f64(self, v: f64) -> Result<()> {
        self.add_tab();
        self.colon_or();
        self.output += &v.to_string();
        self.add_newline();
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<()> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_str(self, v: &str) -> Result<()> {
        self.add_tab();
        if !v.contains("\n") {
            if v.as_bytes()[0] as char == ' ' || v.as_bytes()[v.len() - 1] as char == ' ' {
                self.colon_or();
                self.output += "\"";
                self.output += v;
                self.output += "\"";
            } else {
                self.colon_or();
                self.output += v;
            }
        } else {
            self.colon_or();
            self.output += "\"\"\"\n";
            self.output += v;
            self.output += "\n\"\"\"";
            self.add_newline();
        }
        self.add_newline();
        
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<()> {
        for x in v {
            self.add_tab();
            self.colon_or();
            self.output += &x.to_string();
            self.add_newline();
        }
        
        Ok(())
    }

    fn serialize_none(self) -> Result<()> {
        self.serialize_unit()
    }

    fn serialize_some<T>(self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<()> {
        self.output += "null";
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<()> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self);
        
        Ok(())
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        self.add_tab();
        self.output += &self.field_name;
        self.output += ": # Gen: Vec\n";
        self.tab += "    ";
        self.unindent_flag = true;
        self.field_flag = false;
        Ok(self)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        self.add_tab();
        self.output += &self.field_name;
        self.output += ": # Gen: Tuple\n";
        self.tab += "    ";
        self.unindent_flag = true;
        self.field_flag = false;
        Ok(self)
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        self.add_tab();
        self.output += &self.field_name;
        self.output += ": # Gen: Tuple Struct\n";
        self.unindent_flag = true;
        self.field_flag = false;
        Ok(self)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        self.output += variant;
        self.output += ":\n";
        self.add_tab();
        self.output += "- ";
        
        Ok(self)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        self.decrease_tab();
        self.output += &self.tab;
        self.output += &self.field_name;
        self.output += ": # Gen: HashMap\n";
        self.increase_tab();
        Ok(self)
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct> {
        self.add_tab();
        self.output += if &self.field_name == "" { name } else { &self.field_name };
        self.output += ": # Gen: ";
        self.output += name;
        self.output += " Struct\n";
        self.tab += "    ";
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        self.output += &self.tab;
        self.output += variant;
        self.output += ":\n";
        self.tab += "    ";
        Ok(self)
    }
}

impl<'a> SerializeSeq for &'a mut SUCCSerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a> SerializeTuple for &'a mut SUCCSerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a> SerializeTupleStruct for &'a mut SUCCSerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a> SerializeTupleVariant for &'a mut SUCCSerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a> SerializeMap for &'a mut SUCCSerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.output += &self.tab;
        self.raw_flag = true;
        key.serialize(&mut **self)?;
        self.output += ": ";
        self.raw_flag = false;
        Ok(())
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.raw_flag = true;
        value.serialize(&mut **self)?;
        self.output += "\n";
        self.raw_flag = false;
        Ok(())
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a> SerializeStruct for &'a mut SUCCSerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.unindent_flag = false;
        self.field_name = key.to_string();
        value.serialize(&mut **self)?;
        if self.unindent_flag {
            self.unindent_flag = true;
            self.decrease_tab();
        }
        self.field_flag = true;
        Ok(())
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a> SerializeStructVariant for &'a mut SUCCSerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}