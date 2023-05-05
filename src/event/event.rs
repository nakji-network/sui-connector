// This file is generated by rust-protobuf 3.2.0. Do not edit
// .proto file is parsed by protoc --rust-out=...
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `event.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_2_0;

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:nakji.sui.event.SwappedEvent)
pub struct SwappedEvent {
    // message fields
    // @@protoc_insertion_point(field:nakji.sui.event.SwappedEvent.ts)
    pub ts: ::protobuf::MessageField<::protobuf::well_known_types::timestamp::Timestamp>,
    // @@protoc_insertion_point(field:nakji.sui.event.SwappedEvent.tx_digest)
    pub tx_digest: ::std::vec::Vec<u8>,
    // @@protoc_insertion_point(field:nakji.sui.event.SwappedEvent.event_seq)
    pub event_seq: u64,
    // @@protoc_insertion_point(field:nakji.sui.event.SwappedEvent.global)
    pub global: ::std::vec::Vec<u8>,
    // @@protoc_insertion_point(field:nakji.sui.event.SwappedEvent.lp_name)
    pub lp_name: ::std::string::String,
    // @@protoc_insertion_point(field:nakji.sui.event.SwappedEvent.coin_x_in)
    pub coin_x_in: u64,
    // @@protoc_insertion_point(field:nakji.sui.event.SwappedEvent.coin_x_out)
    pub coin_x_out: u64,
    // @@protoc_insertion_point(field:nakji.sui.event.SwappedEvent.coin_y_in)
    pub coin_y_in: u64,
    // @@protoc_insertion_point(field:nakji.sui.event.SwappedEvent.coin_y_out)
    pub coin_y_out: u64,
    // special fields
    // @@protoc_insertion_point(special_field:nakji.sui.event.SwappedEvent.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a SwappedEvent {
    fn default() -> &'a SwappedEvent {
        <SwappedEvent as ::protobuf::Message>::default_instance()
    }
}

impl SwappedEvent {
    pub fn new() -> SwappedEvent {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(9);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, ::protobuf::well_known_types::timestamp::Timestamp>(
            "ts",
            |m: &SwappedEvent| { &m.ts },
            |m: &mut SwappedEvent| { &mut m.ts },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "tx_digest",
            |m: &SwappedEvent| { &m.tx_digest },
            |m: &mut SwappedEvent| { &mut m.tx_digest },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "event_seq",
            |m: &SwappedEvent| { &m.event_seq },
            |m: &mut SwappedEvent| { &mut m.event_seq },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "global",
            |m: &SwappedEvent| { &m.global },
            |m: &mut SwappedEvent| { &mut m.global },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "lp_name",
            |m: &SwappedEvent| { &m.lp_name },
            |m: &mut SwappedEvent| { &mut m.lp_name },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "coin_x_in",
            |m: &SwappedEvent| { &m.coin_x_in },
            |m: &mut SwappedEvent| { &mut m.coin_x_in },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "coin_x_out",
            |m: &SwappedEvent| { &m.coin_x_out },
            |m: &mut SwappedEvent| { &mut m.coin_x_out },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "coin_y_in",
            |m: &SwappedEvent| { &m.coin_y_in },
            |m: &mut SwappedEvent| { &mut m.coin_y_in },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "coin_y_out",
            |m: &SwappedEvent| { &m.coin_y_out },
            |m: &mut SwappedEvent| { &mut m.coin_y_out },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<SwappedEvent>(
            "SwappedEvent",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for SwappedEvent {
    const NAME: &'static str = "SwappedEvent";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.ts)?;
                },
                18 => {
                    self.tx_digest = is.read_bytes()?;
                },
                24 => {
                    self.event_seq = is.read_uint64()?;
                },
                34 => {
                    self.global = is.read_bytes()?;
                },
                42 => {
                    self.lp_name = is.read_string()?;
                },
                48 => {
                    self.coin_x_in = is.read_uint64()?;
                },
                56 => {
                    self.coin_x_out = is.read_uint64()?;
                },
                64 => {
                    self.coin_y_in = is.read_uint64()?;
                },
                72 => {
                    self.coin_y_out = is.read_uint64()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if let Some(v) = self.ts.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if !self.tx_digest.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.tx_digest);
        }
        if self.event_seq != 0 {
            my_size += ::protobuf::rt::uint64_size(3, self.event_seq);
        }
        if !self.global.is_empty() {
            my_size += ::protobuf::rt::bytes_size(4, &self.global);
        }
        if !self.lp_name.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.lp_name);
        }
        if self.coin_x_in != 0 {
            my_size += ::protobuf::rt::uint64_size(6, self.coin_x_in);
        }
        if self.coin_x_out != 0 {
            my_size += ::protobuf::rt::uint64_size(7, self.coin_x_out);
        }
        if self.coin_y_in != 0 {
            my_size += ::protobuf::rt::uint64_size(8, self.coin_y_in);
        }
        if self.coin_y_out != 0 {
            my_size += ::protobuf::rt::uint64_size(9, self.coin_y_out);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.ts.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        if !self.tx_digest.is_empty() {
            os.write_bytes(2, &self.tx_digest)?;
        }
        if self.event_seq != 0 {
            os.write_uint64(3, self.event_seq)?;
        }
        if !self.global.is_empty() {
            os.write_bytes(4, &self.global)?;
        }
        if !self.lp_name.is_empty() {
            os.write_string(5, &self.lp_name)?;
        }
        if self.coin_x_in != 0 {
            os.write_uint64(6, self.coin_x_in)?;
        }
        if self.coin_x_out != 0 {
            os.write_uint64(7, self.coin_x_out)?;
        }
        if self.coin_y_in != 0 {
            os.write_uint64(8, self.coin_y_in)?;
        }
        if self.coin_y_out != 0 {
            os.write_uint64(9, self.coin_y_out)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> SwappedEvent {
        SwappedEvent::new()
    }

    fn clear(&mut self) {
        self.ts.clear();
        self.tx_digest.clear();
        self.event_seq = 0;
        self.global.clear();
        self.lp_name.clear();
        self.coin_x_in = 0;
        self.coin_x_out = 0;
        self.coin_y_in = 0;
        self.coin_y_out = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static SwappedEvent {
        static instance: SwappedEvent = SwappedEvent {
            ts: ::protobuf::MessageField::none(),
            tx_digest: ::std::vec::Vec::new(),
            event_seq: 0,
            global: ::std::vec::Vec::new(),
            lp_name: ::std::string::String::new(),
            coin_x_in: 0,
            coin_x_out: 0,
            coin_y_in: 0,
            coin_y_out: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for SwappedEvent {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("SwappedEvent").unwrap()).clone()
    }
}

impl ::std::fmt::Display for SwappedEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SwappedEvent {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0bevent.proto\x12\x0fnakji.sui.event\x1a\x1fgoogle/protobuf/timestam\
    p.proto\"\x99\x02\n\x0cSwappedEvent\x12*\n\x02ts\x18\x01\x20\x01(\x0b2\
    \x1a.google.protobuf.TimestampR\x02ts\x12\x1b\n\ttx_digest\x18\x02\x20\
    \x01(\x0cR\x08txDigest\x12\x1b\n\tevent_seq\x18\x03\x20\x01(\x04R\x08eve\
    ntSeq\x12\x16\n\x06global\x18\x04\x20\x01(\x0cR\x06global\x12\x17\n\x07l\
    p_name\x18\x05\x20\x01(\tR\x06lpName\x12\x1a\n\tcoin_x_in\x18\x06\x20\
    \x01(\x04R\x07coinXIn\x12\x1c\n\ncoin_x_out\x18\x07\x20\x01(\x04R\x08coi\
    nXOut\x12\x1a\n\tcoin_y_in\x18\x08\x20\x01(\x04R\x07coinYIn\x12\x1c\n\nc\
    oin_y_out\x18\t\x20\x01(\x04R\x08coinYOutB(Z&github.com/nakji-network/su\
    i-connectorJ\xc3\x04\n\x06\x12\x04\0\0\x12\x01\n\x08\n\x01\x0c\x12\x03\0\
    \0\x12\n\t\n\x02\x03\0\x12\x03\x02\0)\n\x08\n\x01\x02\x12\x03\x04\0\x18\
    \n\x08\n\x01\x08\x12\x03\x06\0=\n\t\n\x02\x08\x0b\x12\x03\x06\0=\n\n\n\
    \x02\x04\0\x12\x04\x08\0\x12\x01\n\n\n\x03\x04\0\x01\x12\x03\x08\x08\x14\
    \n\x0b\n\x04\x04\0\x02\0\x12\x03\t\x04%\n\x0c\n\x05\x04\0\x02\0\x06\x12\
    \x03\t\x04\x1d\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\t\x1e\x20\n\x0c\n\x05\
    \x04\0\x02\0\x03\x12\x03\t#$\n\x0b\n\x04\x04\0\x02\x01\x12\x03\n\x04\x18\
    \n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\n\x04\t\n\x0c\n\x05\x04\0\x02\x01\
    \x01\x12\x03\n\n\x13\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\n\x16\x17\n\
    \x0b\n\x04\x04\0\x02\x02\x12\x03\x0b\x04\x19\n\x0c\n\x05\x04\0\x02\x02\
    \x05\x12\x03\x0b\x04\n\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03\x0b\x0b\x14\
    \n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03\x0b\x17\x18\n\x0b\n\x04\x04\0\x02\
    \x03\x12\x03\x0c\x04\x15\n\x0c\n\x05\x04\0\x02\x03\x05\x12\x03\x0c\x04\t\
    \n\x0c\n\x05\x04\0\x02\x03\x01\x12\x03\x0c\n\x10\n\x0c\n\x05\x04\0\x02\
    \x03\x03\x12\x03\x0c\x13\x14\n\x0b\n\x04\x04\0\x02\x04\x12\x03\r\x04\x17\
    \n\x0c\n\x05\x04\0\x02\x04\x05\x12\x03\r\x04\n\n\x0c\n\x05\x04\0\x02\x04\
    \x01\x12\x03\r\x0b\x12\n\x0c\n\x05\x04\0\x02\x04\x03\x12\x03\r\x15\x16\n\
    \x0b\n\x04\x04\0\x02\x05\x12\x03\x0e\x04\x19\n\x0c\n\x05\x04\0\x02\x05\
    \x05\x12\x03\x0e\x04\n\n\x0c\n\x05\x04\0\x02\x05\x01\x12\x03\x0e\x0b\x14\
    \n\x0c\n\x05\x04\0\x02\x05\x03\x12\x03\x0e\x17\x18\n\x0b\n\x04\x04\0\x02\
    \x06\x12\x03\x0f\x04\x1a\n\x0c\n\x05\x04\0\x02\x06\x05\x12\x03\x0f\x04\n\
    \n\x0c\n\x05\x04\0\x02\x06\x01\x12\x03\x0f\x0b\x15\n\x0c\n\x05\x04\0\x02\
    \x06\x03\x12\x03\x0f\x18\x19\n\x0b\n\x04\x04\0\x02\x07\x12\x03\x10\x04\
    \x19\n\x0c\n\x05\x04\0\x02\x07\x05\x12\x03\x10\x04\n\n\x0c\n\x05\x04\0\
    \x02\x07\x01\x12\x03\x10\x0b\x14\n\x0c\n\x05\x04\0\x02\x07\x03\x12\x03\
    \x10\x17\x18\n\x0b\n\x04\x04\0\x02\x08\x12\x03\x11\x04\x1a\n\x0c\n\x05\
    \x04\0\x02\x08\x05\x12\x03\x11\x04\n\n\x0c\n\x05\x04\0\x02\x08\x01\x12\
    \x03\x11\x0b\x15\n\x0c\n\x05\x04\0\x02\x08\x03\x12\x03\x11\x18\x19b\x06p\
    roto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(1);
            deps.push(::protobuf::well_known_types::timestamp::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(SwappedEvent::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(0);
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}
