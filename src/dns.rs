// https://www.iana.org/assignments/dns-parameters/dns-parameters.xhtml#dns-parameters-4;
pub const DNS_TYPE_ERROR: u16 = 0;
pub const DNS_TYPE_A: u16 = 1;
pub const DNS_TYPE_NS: u16 = 2;
pub const DNS_TYPE_MD: u16 = 3;
pub const DNS_TYPE_MF: u16 = 4;
pub const DNS_TYPE_CNAME: u16 = 5;
pub const DNS_TYPE_SOA: u16 = 6;
pub const DNS_TYPE_MB: u16 = 7;
pub const DNS_TYPE_MG: u16 = 8;
pub const DNS_TYPE_MR: u16 = 9;
pub const DNS_TYPE_NULL: u16 = 10;
pub const DNS_TYPE_WKS: u16 = 11;
pub const DNS_TYPE_PTR: u16 = 12;
pub const DNS_TYPE_HINFO: u16 = 13;
pub const DNS_TYPE_MINFO: u16 = 14;
pub const DNS_TYPE_MX: u16 = 15;
pub const DNS_TYPE_TXT: u16 = 16;
pub const DNS_TYPE_RP: u16 = 17;
pub const DNS_TYPE_AFSDB: u16 = 18;
pub const DNS_TYPE_X25: u16 = 19;
pub const DNS_TYPE_ISDN: u16 = 20;
pub const DNS_TYPE_RT: u16 = 21;
pub const DNS_TYPE_NSAP: u16 = 22;
pub const DNS_TYPE_NSAP_PTR: u16 = 23;
pub const DNS_TYPE_SIG: u16 = 24;
pub const DNS_TYPE_KEY: u16 = 25;
pub const DNS_TYPE_PX: u16 = 26;
pub const DNS_TYPE_GPOS: u16 = 27;
pub const DNS_TYPE_AAAA: u16 = 28;
pub const DNS_TYPE_LOC: u16 = 29;
pub const DNS_TYPE_NXT: u16 = 30;
pub const DNS_TYPE_EID: u16 = 31;
pub const DNS_TYPE_NIMLOC: u16 = 32;
pub const DNS_TYPE_SRV: u16 = 33;
pub const DNS_TYPE_ATMA: u16 = 34;
pub const DNS_TYPE_NAPTR: u16 = 35;
pub const DNS_TYPE_KX: u16 = 36;
pub const DNS_TYPE_CERT: u16 = 37;
pub const DNS_TYPE_A6: u16 = 38;
pub const DNS_TYPE_DNAME: u16 = 39;
pub const DNS_TYPE_SINK: u16 = 40;
pub const DNS_TYPE_OPT: u16 = 41;
pub const DNS_TYPE_APL: u16 = 42;
pub const DNS_TYPE_DS: u16 = 43;
pub const DNS_TYPE_SSHFP: u16 = 44;
pub const DNS_TYPE_IPSECKEY: u16 = 45;
pub const DNS_TYPE_RRSIG: u16 = 46;
pub const DNS_TYPE_NSEC: u16 = 47;
pub const DNS_TYPE_DNSKEY: u16 = 48;
pub const DNS_TYPE_DHCID: u16 = 49;
pub const DNS_TYPE_NSEC3: u16 = 50;
pub const DNS_TYPE_NSEC3PARAM: u16 = 51;
pub const DNS_TYPE_TLSA: u16 = 52;
pub const DNS_TYPE_SMIMEA: u16 = 53;

pub const DNS_TYPE_HIP: u16 = 55;
pub const DNS_TYPE_NINFO: u16 = 56;
pub const DNS_TYPE_RKEY: u16 = 57;
pub const DNS_TYPE_TALINK: u16 = 58;
pub const DNS_TYPE_CDS: u16 = 59;
pub const DNS_TYPE_CDNSKEY: u16 = 60;
pub const DNS_TYPE_OPENPGPKEY: u16 = 61;
pub const DNS_TYPE_CSYNC: u16 = 62;
pub const DNS_TYPE_ZONEMD: u16 = 63;
pub const DNS_TYPE_SVCB: u16 = 64;
pub const DNS_TYPE_HTTPS: u16 = 65;

pub const DNS_TYPE_SPF: u16 = 99;
pub const DNS_TYPE_UINFO: u16 = 100;
pub const DNS_TYPE_UID: u16 = 101;
pub const DNS_TYPE_GID: u16 = 102;
pub const DNS_TYPE_UNSPEC: u16 = 103;
pub const DNS_TYPE_NID: u16 = 104;
pub const DNS_TYPE_L32: u16 = 105;
pub const DNS_TYPE_L64: u16 = 106;
pub const DNS_TYPE_LP: u16 = 107;
pub const DNS_TYPE_EUI48: u16 = 108;
pub const DNS_TYPE_EUI64: u16 = 109;

pub const DNS_TYPE_TKEY: u16 = 249;
pub const DNS_TYPE_TSIG: u16 = 250;
pub const DNS_TYPE_IXFR: u16 = 251;
pub const DNS_TYPE_AXFR: u16 = 252;
pub const DNS_TYPE_MAILB: u16 = 253;
pub const DNS_TYPE_MAILA: u16 = 254;

pub const DNS_TYPE_ANY: u16 = 255;

pub const DNS_TYPE_URI: u16 = 256;
pub const DNS_TYPE_CAA: u16 = 257;
pub const DNS_TYPE_AVC: u16 = 258;
pub const DNS_TYPE_DOA: u16 = 259;
pub const DNS_TYPE_AMTREPLAY: u16 = 260;

pub const DNS_TYPE_TA: u16 = 32768;
pub const DNS_TYPE_DLV: u16 = 32769;

pub const DNS_STR_TYPE_A: &str = "A";
pub const DNS_STR_TYPE_NS: &str = "NS";
pub const DNS_STR_TYPE_MD: &str = "MD";
pub const DNS_STR_TYPE_MF: &str = "MF";
pub const DNS_STR_TYPE_CNAME: &str = "CNAME";
pub const DNS_STR_TYPE_SOA: &str = "SOA";
pub const DNS_STR_TYPE_MB: &str = "MB";
pub const DNS_STR_TYPE_MG: &str = "MG";
pub const DNS_STR_TYPE_MR: &str = "MR";
pub const DNS_STR_TYPE_NULL: &str = "NULL";
pub const DNS_STR_TYPE_WKS: &str = "WKS";
pub const DNS_STR_TYPE_PTR: &str = "PTR";
pub const DNS_STR_TYPE_HINFO: &str = "HINFO";
pub const DNS_STR_TYPE_MINFO: &str = "MINFO";
pub const DNS_STR_TYPE_MX: &str = "MX";
pub const DNS_STR_TYPE_TXT: &str = "TXT";
pub const DNS_STR_TYPE_RP: &str = "RP";
pub const DNS_STR_TYPE_AFSDB: &str = "AFSDB";
pub const DNS_STR_TYPE_X25: &str = "X25";
pub const DNS_STR_TYPE_ISDN: &str = "ISDN";
pub const DNS_STR_TYPE_RT: &str = "RT";
pub const DNS_STR_TYPE_NSAP: &str = "NSAP";
pub const DNS_STR_TYPE_NSAP_PTR: &str = "NSAP-PTR";
pub const DNS_STR_TYPE_SIG: &str = "SIG";
pub const DNS_STR_TYPE_KEY: &str = "KEY";
pub const DNS_STR_TYPE_PX: &str = "PX";
pub const DNS_STR_TYPE_GPOS: &str = "GPOS";
pub const DNS_STR_TYPE_AAAA: &str = "AAAA";
pub const DNS_STR_TYPE_LOC: &str = "LOC";
pub const DNS_STR_TYPE_NXT: &str = "NXT";
pub const DNS_STR_TYPE_EID: &str = "EID";
pub const DNS_STR_TYPE_NIMLOC: &str = "NIMLOC";
pub const DNS_STR_TYPE_SRV: &str = "SRV";
pub const DNS_STR_TYPE_ATMA: &str = "ATMA";
pub const DNS_STR_TYPE_NAPTR: &str = "NAPTR";
pub const DNS_STR_TYPE_KX: &str = "KX";
pub const DNS_STR_TYPE_CERT: &str = "CERT";
pub const DNS_STR_TYPE_A6: &str = "A6";
pub const DNS_STR_TYPE_DNAME: &str = "DNAME";
pub const DNS_STR_TYPE_SINK: &str = "SINK";
pub const DNS_STR_TYPE_OPT: &str = "OPT";
pub const DNS_STR_TYPE_APL: &str = "APL";
pub const DNS_STR_TYPE_DS: &str = "DS";
pub const DNS_STR_TYPE_SSHFP: &str = "SSHFP";
pub const DNS_STR_TYPE_IPSECKEY: &str = "IPSECKEY";
pub const DNS_STR_TYPE_RRSIG: &str = "RRSIG";
pub const DNS_STR_TYPE_NSEC: &str = "NSEC";
pub const DNS_STR_TYPE_DNSKEY: &str = "DNSKEY";
pub const DNS_STR_TYPE_DHCID: &str = "DHCID";
pub const DNS_STR_TYPE_NSEC3: &str = "NSEC3";
pub const DNS_STR_TYPE_NSEC3PARAM: &str = "NSEC3PARAM";
pub const DNS_STR_TYPE_TLSA: &str = "TLSA";
pub const DNS_STR_TYPE_SMIMEA: &str = "SMIMEA";
pub const DNS_STR_TYPE_HIP: &str = "HIP";
pub const DNS_STR_TYPE_NINFO: &str = "NINFO";
pub const DNS_STR_TYPE_RKEY: &str = "RKEY";
pub const DNS_STR_TYPE_TALINK: &str = "TALINK";
pub const DNS_STR_TYPE_CDS: &str = "CDS";
pub const DNS_STR_TYPE_CDNSKEY: &str = "CDNSKEY";
pub const DNS_STR_TYPE_OPENPGPKEY: &str = "OPENPGPKEY";
pub const DNS_STR_TYPE_CSYNC: &str = "CSYNC";
pub const DNS_STR_TYPE_ZONEMD: &str = "ZONEMD";
pub const DNS_STR_TYPE_SVCB: &str = "SVCB";
pub const DNS_STR_TYPE_HTTPS: &str = "HTTPS";
pub const DNS_STR_TYPE_SPF: &str = "SPF";
pub const DNS_STR_TYPE_UINFO: &str = "UINFO";
pub const DNS_STR_TYPE_UID: &str = "UID";
pub const DNS_STR_TYPE_GID: &str = "GID";
pub const DNS_STR_TYPE_UNSPEC: &str = "UNSPEC";
pub const DNS_STR_TYPE_NID: &str = "NID";
pub const DNS_STR_TYPE_L32: &str = "L32";
pub const DNS_STR_TYPE_L64: &str = "L64";
pub const DNS_STR_TYPE_LP: &str = "LP";
pub const DNS_STR_TYPE_EUI48: &str = "EUI48";
pub const DNS_STR_TYPE_EUI64: &str = "EUI64";
pub const DNS_STR_TYPE_TKEY: &str = "TKEY";
pub const DNS_STR_TYPE_TSIG: &str = "TSIG";
pub const DNS_STR_TYPE_IXFR: &str = "IXFR";
pub const DNS_STR_TYPE_AXFR: &str = "AXFR";
pub const DNS_STR_TYPE_MAILB: &str = "MAILB";
pub const DNS_STR_TYPE_MAILA: &str = "MAILA";

pub const DNS_STR_TYPE_ANY: &str = "ANY";

pub const DNS_STR_TYPE_URI: &str = "URI";
pub const DNS_STR_TYPE_CAA: &str = "CAA";
pub const DNS_STR_TYPE_AVC: &str = "AVC";
pub const DNS_STR_TYPE_DOA: &str = "DOA";
pub const DNS_STR_TYPE_AMTREPLAY: &str = "AMTREPLAY";
pub const DNS_STR_TYPE_TA: &str = "TA";
pub const DNS_STR_TYPE_DLV: &str = "DLV";

// https://www.rfc-editor.org/rfc/rfc1035 4.1.1
pub struct Header {
    // Identifier.
    pub id: u16,

    // A one bit field that specifies whether this message is a query (0), or a response (1).
    pub qr: u8,

    // A four bit field that specifies kind of query.
    pub opcode: u8,

    // Authoritative Answer.
    pub aa: u8,

    // Specifies that this message was truncated.
    pub tc: u8,

    // Recursion Desired.
    pub rd: u8,

    // Recursion Available.
    pub ra: u8,

    // Reserved for future use. Must be zero in all queries and responses.
    pub z: u8,

    // Response code.
    pub rcode: u8,

    // Number of entries in the question section.
    pub qdcount: u16,

    // Number of resource records in the answer section.
    pub ancount: u16,

    // Number of name server resource records in the authority records section.
    pub nscount: u16,

    // Number of resource records in the additional records section.
    pub arcount: u16,
}

// https://www.rfc-editor.org/rfc/rfc1035 4.1.2
pub struct Question {
    pub qname: String,
    pub qtype: u16,
    pub qclass: u16
}

// https://www.rfc-editor.org/rfc/rfc1035 4.1.3
pub struct ResourceRecord {
    pub name: String,
    pub _type: u16,
    pub class: u16,
    pub ttl: u32,
    pub rdlength: u16,
    pub rdata: String,
}

pub struct Message {
    pub hdr: Header,
    pub questions: Vec<Question>,
    pub answers: Vec<ResourceRecord>,
    pub authority_records: Vec<ResourceRecord>,
    pub additional_records: Vec<ResourceRecord>
}

/*fn read_u8(buf: &Vec<u8>, pos: usize) -> u8 {
    return buf[pos];
}*/

fn read_u16(buf: &Vec<u8>, pos: usize) -> u16 {
    return u16::from(buf[pos]) * 0x100 + u16::from(buf[pos+ 1]);
}

fn read_u32(buf: &Vec<u8>, pos: usize) -> u32 {
    return 
        u32::from(buf[pos]) * 0x1000000 + 
        u32::from(buf[pos+ 1]) * 0x10000 +
        u32::from(buf[pos + 2]) *0x100 + 
        u32::from(buf[pos + 3]);
}

fn read_header(buf: &Vec<u8>) -> Header {
    let flags = read_u16(&buf, 2);

    let header = Header{
        id: read_u16(&buf, 0),
        qr: ((flags & (1 << 15)) >> 15) as u8,
        opcode: ((flags & (0xF << 11)) >> 11) as u8,
        aa: ((flags & (1 << 10)) >> 10) as u8,
        tc: ((flags & (1 << 9)) >> 9) as u8,
        rd: ((flags & (1 << 8)) >> 8) as u8,
        ra: ((flags & (1 << 7)) >> 7) as u8,
        z: ((flags & (0x7 << 4)) >> 4) as u8,
        rcode: (flags & 0xF) as u8,

        qdcount: read_u16(&buf, 4),
        ancount: read_u16(&buf, 6),
        nscount: read_u16(&buf, 8),
        arcount: read_u16(&buf, 10),
    };

    return header;
}

fn read_qname(buf: &Vec<u8>, pos: usize, qname: &mut String) -> usize {
    let mut max_pos = pos;
    let mut length: u8;
    let mut tmp_pos = pos;
    if buf[pos] == 0 {
        qname.push('.');
        return pos;
    }

    while buf[tmp_pos] != 0 {
        length = buf[tmp_pos];
        if ((length & (0x3 << 6)) >> 6) == 0x3 {
            if max_pos < tmp_pos {
                max_pos = tmp_pos + 1;
            }
            tmp_pos = usize::from(read_u16(buf, tmp_pos) - (0x3 << 14));
            length = buf[tmp_pos];
            for i in 0 .. length {
                qname.push(char::from(buf[tmp_pos + 1 + usize::from(i)]))
            }
            qname.push('.');
            tmp_pos += usize::from(length) + 1;
        } else {
            if max_pos < tmp_pos {
                max_pos = tmp_pos;
            }

            for i in 0 .. length {
                qname.push(char::from(buf[tmp_pos + 1 + usize::from(i)]))
            }
            qname.push('.');
            tmp_pos += usize::from(length) + 1;
        }
    }

    if max_pos < tmp_pos {
        max_pos = tmp_pos;
    }
    return max_pos + 1;
}

fn read_question(buf: &Vec<u8>, pos: usize) -> (Question, usize) {
    let mut qname = String::new();
    let current_pos = read_qname(buf, pos, &mut qname);

    let q = Question{
        qname: qname,
        qtype: read_u16(&buf, current_pos),
        qclass: read_u16(&buf, current_pos+ 2),
    };

    return (q, pos + 4);
}

fn read_ipv4(buf: &Vec<u8>, pos: usize) -> String {
    return format!(
        "{}.{}.{}.{}", 
        buf[pos].to_string(),
        buf[pos + 1].to_string(),
        buf[pos + 2].to_string(),
        buf[pos + 3].to_string()
    );
}


fn read_resource_record(buf: &Vec<u8>, pos: usize) -> (ResourceRecord, usize) {
    let mut qname = String::new();
    let tmp_current_pos = read_qname(buf, pos, &mut qname);

    let resource_record_type = read_u16(buf, tmp_current_pos + 1);

    let mut rdata: String = String::from("");
    match resource_record_type {
        DNS_TYPE_A => { rdata = read_ipv4(buf, tmp_current_pos + 11);},
        /*DNS_TYPE_AAAA => { rdata = read_ipv6(buf, tmp_current_pos + 11);},*/
        _ =>  { read_qname(buf, tmp_current_pos + 11, &mut rdata); },
    }

    let resource_record = ResourceRecord{ 
        name: qname, 
        _type: resource_record_type, 
        class: read_u16(buf, tmp_current_pos + 3), 
        ttl: read_u32(buf, tmp_current_pos + 5), 
        rdlength: read_u16(buf, tmp_current_pos + 9), 
        rdata: rdata, 
    };
    return (resource_record, tmp_current_pos);
}

pub fn get_message(buf: &Vec<u8>) -> Message {
    let header = read_header(buf);

    let mut pos = 12;

    let mut questions: Vec<Question> = Vec::new();
    for _ in 0 .. header.qdcount {
        let question: Question;
        (question, pos) = read_question(&buf, pos);
        questions.push(question)
    }

    let mut answers: Vec<ResourceRecord> = Vec::new();
    for _ in 0 .. header.ancount {
        let resource_record: ResourceRecord;
        (resource_record, pos) = read_resource_record(&buf, pos);
        answers.push(resource_record)
    }

    let mut authority_records: Vec<ResourceRecord> = Vec::new();
    for _ in 0 .. header.ancount {
        let resource_record: ResourceRecord;
        (resource_record, pos) = read_resource_record(&buf, pos);
        authority_records.push(resource_record)
    }

    let mut additional_records: Vec<ResourceRecord> = Vec::new();
    for _ in 0 .. header.ancount {
        let resource_record: ResourceRecord;
        (resource_record, pos) = read_resource_record(&buf, pos);
        additional_records.push(resource_record)
    }

    return Message { 
        hdr: header, 
        questions: questions, 
        answers: answers, 
        authority_records: authority_records, 
        additional_records: additional_records,
    };
}