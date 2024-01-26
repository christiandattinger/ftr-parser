use std::collections::HashMap;
use crate::types::DataType::NONE;

#[derive(Debug)]
pub struct TxStream {
    pub id: i64,
    pub name: String,
    pub kind: String,
}

#[derive(Debug)]
pub struct TxGenerator {
    pub id: i64,
    pub name: String,
    pub stream_id: i64, // TODO make reference to stream not id of stream
}

#[derive(Debug)]
pub struct TxRelation {
    pub name: String,
    pub source_tx_id: i64,
    pub sink_tx_id: i64,
    pub source_stream_id: i64,
    pub sink_stream_id: i64,
}

#[derive(Debug)]
pub struct TxBlock {
    pub stream_id: i64,
    pub start_time: i64,
    pub end_time: i64,
    pub transactions: Vec<Transaction>,
}

#[derive(Debug)]
pub struct Transaction {
    pub event: Event,
    pub begin: Attribute,
    pub record: Attribute,
    pub end: Attribute,
}

impl Transaction {
    pub fn new() -> Self{
        let event = Event::new();
        let begin = Attribute::new();
        let record = Attribute::new();
        let end = Attribute::new();

        Self {
            event,
            begin,
            record,
            end,
        }
    }
}

#[derive(Debug)]
pub struct Event {
    pub tx_id: i64,
    pub gen_id: i64,
    pub start_time: i64,
    pub end_time: i64,
}

impl Event {
    pub fn new() -> Self{
        let tx_id = -1;
        let gen_id = -1;
        let start_time = -1;
        let end_time = -1;
        Self {
            tx_id,
            gen_id,
            start_time,
            end_time
        }
    }
}

#[derive(Debug)]
pub struct Attribute {
    pub name: String,
    pub data_type: DataType, // TODO make it so enum carries the value of the respective data_type
    pub value: i64,
}

impl Attribute {
    pub fn new() -> Self{
        let name = String::new();
        let data_type = NONE;
        let value = -1;
        Self {
            name,
            data_type,
            value
        }
    }
}

#[derive(Debug)]
pub enum DataType {
    BOOLEAN,
    ENUMERATION,
    INTEGER,
    UNSIGNED,
    FLOATING_POINT_NUMBER,
    BIT_VECTOR,
    LOGIC_VECTOR,
    FIXED_POINT_INTERGER,
    UNSIGNED_FIXED_POINT_INTEGER,
    POINTER,
    STRING,
    TIME,
    NONE,
}

pub struct FTR {
    pub str_dict: HashMap<i64, String>,
    pub tx_streams: Vec<TxStream>,
    pub tx_generators: Vec<TxGenerator>,
    pub tx_blocks: Vec<TxBlock>,
    pub tx_relations: Vec<TxRelation>,
}