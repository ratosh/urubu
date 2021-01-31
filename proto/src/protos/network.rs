// This file is generated by rust-protobuf 2.18.1. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![rustfmt::skip]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `network.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_18_1;

#[derive(PartialEq,Clone,Default)]
pub struct Network {
    // message fields
    pub layers: ::protobuf::RepeatedField<Layer>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Network {
    fn default() -> &'a Network {
        <Network as ::protobuf::Message>::default_instance()
    }
}

impl Network {
    pub fn new() -> Network {
        ::std::default::Default::default()
    }

    // repeated .protos.Layer layers = 1;


    pub fn get_layers(&self) -> &[Layer] {
        &self.layers
    }
    pub fn clear_layers(&mut self) {
        self.layers.clear();
    }

    // Param is passed by value, moved
    pub fn set_layers(&mut self, v: ::protobuf::RepeatedField<Layer>) {
        self.layers = v;
    }

    // Mutable pointer to the field.
    pub fn mut_layers(&mut self) -> &mut ::protobuf::RepeatedField<Layer> {
        &mut self.layers
    }

    // Take field
    pub fn take_layers(&mut self) -> ::protobuf::RepeatedField<Layer> {
        ::std::mem::replace(&mut self.layers, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for Network {
    fn is_initialized(&self) -> bool {
        for v in &self.layers {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.layers)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.layers {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        for v in &self.layers {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Network {
        Network::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Layer>>(
                "layers",
                |m: &Network| { &m.layers },
                |m: &mut Network| { &mut m.layers },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Network>(
                "Network",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Network {
        static instance: ::protobuf::rt::LazyV2<Network> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Network::new)
    }
}

impl ::protobuf::Clear for Network {
    fn clear(&mut self) {
        self.layers.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Network {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Network {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Layer {
    // message fields
    pub weights: ::protobuf::RepeatedField<Weight>,
    pub bias: ::std::vec::Vec<f64>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Layer {
    fn default() -> &'a Layer {
        <Layer as ::protobuf::Message>::default_instance()
    }
}

impl Layer {
    pub fn new() -> Layer {
        ::std::default::Default::default()
    }

    // repeated .protos.Weight weights = 1;


    pub fn get_weights(&self) -> &[Weight] {
        &self.weights
    }
    pub fn clear_weights(&mut self) {
        self.weights.clear();
    }

    // Param is passed by value, moved
    pub fn set_weights(&mut self, v: ::protobuf::RepeatedField<Weight>) {
        self.weights = v;
    }

    // Mutable pointer to the field.
    pub fn mut_weights(&mut self) -> &mut ::protobuf::RepeatedField<Weight> {
        &mut self.weights
    }

    // Take field
    pub fn take_weights(&mut self) -> ::protobuf::RepeatedField<Weight> {
        ::std::mem::replace(&mut self.weights, ::protobuf::RepeatedField::new())
    }

    // repeated double bias = 2;


    pub fn get_bias(&self) -> &[f64] {
        &self.bias
    }
    pub fn clear_bias(&mut self) {
        self.bias.clear();
    }

    // Param is passed by value, moved
    pub fn set_bias(&mut self, v: ::std::vec::Vec<f64>) {
        self.bias = v;
    }

    // Mutable pointer to the field.
    pub fn mut_bias(&mut self) -> &mut ::std::vec::Vec<f64> {
        &mut self.bias
    }

    // Take field
    pub fn take_bias(&mut self) -> ::std::vec::Vec<f64> {
        ::std::mem::replace(&mut self.bias, ::std::vec::Vec::new())
    }
}

impl ::protobuf::Message for Layer {
    fn is_initialized(&self) -> bool {
        for v in &self.weights {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.weights)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_double_into(wire_type, is, &mut self.bias)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.weights {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += 9 * self.bias.len() as u32;
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        for v in &self.weights {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.bias {
            os.write_double(2, *v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Layer {
        Layer::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Weight>>(
                "weights",
                |m: &Layer| { &m.weights },
                |m: &mut Layer| { &mut m.weights },
            ));
            fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "bias",
                |m: &Layer| { &m.bias },
                |m: &mut Layer| { &mut m.bias },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Layer>(
                "Layer",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Layer {
        static instance: ::protobuf::rt::LazyV2<Layer> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Layer::new)
    }
}

impl ::protobuf::Clear for Layer {
    fn clear(&mut self) {
        self.weights.clear();
        self.bias.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Layer {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Layer {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Weight {
    // message fields
    pub values: ::std::vec::Vec<f64>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Weight {
    fn default() -> &'a Weight {
        <Weight as ::protobuf::Message>::default_instance()
    }
}

impl Weight {
    pub fn new() -> Weight {
        ::std::default::Default::default()
    }

    // repeated double values = 1;


    pub fn get_values(&self) -> &[f64] {
        &self.values
    }
    pub fn clear_values(&mut self) {
        self.values.clear();
    }

    // Param is passed by value, moved
    pub fn set_values(&mut self, v: ::std::vec::Vec<f64>) {
        self.values = v;
    }

    // Mutable pointer to the field.
    pub fn mut_values(&mut self) -> &mut ::std::vec::Vec<f64> {
        &mut self.values
    }

    // Take field
    pub fn take_values(&mut self) -> ::std::vec::Vec<f64> {
        ::std::mem::replace(&mut self.values, ::std::vec::Vec::new())
    }
}

impl ::protobuf::Message for Weight {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_double_into(wire_type, is, &mut self.values)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        my_size += 9 * self.values.len() as u32;
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        for v in &self.values {
            os.write_double(1, *v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Weight {
        Weight::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "values",
                |m: &Weight| { &m.values },
                |m: &mut Weight| { &mut m.values },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Weight>(
                "Weight",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Weight {
        static instance: ::protobuf::rt::LazyV2<Weight> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Weight::new)
    }
}

impl ::protobuf::Clear for Weight {
    fn clear(&mut self) {
        self.values.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Weight {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Weight {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\rnetwork.proto\x12\x06protos\"0\n\x07Network\x12%\n\x06layers\x18\x01\
    \x20\x03(\x0b2\r.protos.LayerR\x06layers\"E\n\x05Layer\x12(\n\x07weights\
    \x18\x01\x20\x03(\x0b2\x0e.protos.WeightR\x07weights\x12\x12\n\x04bias\
    \x18\x02\x20\x03(\x01R\x04bias\"\x20\n\x06Weight\x12\x16\n\x06values\x18\
    \x01\x20\x03(\x01R\x06values\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}