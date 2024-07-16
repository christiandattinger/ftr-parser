use std::collections::HashMap;
use std::fmt::{Debug};
use num_bigint::BigInt;
use serde::{Deserialize, Serialize};
use crate::types::DataType::Error;
use crate::types::Timescale::{Fs, Ms, Ns, Ps, S, Unit, Us};
use core::fmt;
use crate::ftr_parser::FtrParser;

type IsCompressed = bool;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TxStream {
    pub id: usize,
    pub name: String,
    pub kind: String,
    pub generators: Vec<usize>,
    pub(super) tx_block_ids: Vec<(u64, IsCompressed)>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TxGenerator {
    pub id: usize,
    pub stream_id: usize,
    pub name: String,
    pub transactions: Vec<Transaction>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TxRelation {
    pub name: String,
    pub source_tx_id: usize,
    pub sink_tx_id: usize,
    pub source_stream_id: usize,
    pub sink_stream_id: usize
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub event: Event,
    pub attributes: Vec<Attribute>,
    pub inc_relations: Vec<TxRelation>,
    pub out_relations: Vec<TxRelation>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Event {
    pub tx_id: usize,
    pub gen_id: usize,
    pub start_time: i64,
    pub end_time: i64,
}

impl Event {
    pub fn new() -> Self{
        let tx_id = 0;
        let gen_id = 0;
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attribute {
    pub kind: AttributeType,
    pub name: String,
    pub data_type: DataType, // TODO make it so enum carries the value of the respective data_type
    pub value: i64,
}

impl Attribute {
    pub fn new() -> Self{
        let kind = AttributeType::NONE;
        let name = String::new();
        let data_type = Error;
        let value = -1;
        Self {
            kind,
            name,
            data_type,
            value
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataType {
    Boolean,
    Enumeration,
    Integer,
    Unsigned,
    FloatingPointNumber,
    BitVector,
    LogicVector,
    FixedPointInteger,
    UnsignedFixedPointInteger,
    Pointer,
    String,
    Time,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AttributeType {
    BEGIN,
    RECORD,
    END,
    NONE,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FTR {
    pub time_scale: Timescale,
    pub max_timestamp: BigInt,
    pub str_dict: HashMap<usize, String>,
    pub tx_streams: HashMap<usize, TxStream>,
    pub tx_generators: HashMap<usize, TxGenerator>,
    pub tx_relations: Vec<TxRelation>,
    pub(crate) file_name: String,
}

impl FTR {
    // Takes a stream id and loads all associated transactions into memory
    pub fn load_stream_into_memory(&mut self, stream_id: usize) -> color_eyre::Result<()>{
        let mut ftr_parser = FtrParser::new(self);

        ftr_parser.load_transactions(stream_id)?;

        Ok(())
    }

    // drops all transactions from this stream from memory, but the stream itself doesn't get deleted
    pub fn drop_stream_from_memory(&mut self, stream_id: usize) {
        for gen_id in &self.tx_streams.get(&stream_id).expect("").generators {
            self.tx_generators.get_mut(gen_id).unwrap().transactions = vec![];
        }
    }

    pub fn get_stream(&self, stream_id: usize) -> Option<&TxStream> {
        self.tx_streams.get(&stream_id)
    }

    pub fn get_stream_from_name(&self, name: String) -> Option<&TxStream> {
        self.tx_streams
            .values()
            .find(|t| t.name == name)
    }

    pub fn get_generator(&self, gen_id: usize) -> Option<&TxGenerator> {
        self.tx_generators.get(&gen_id)
    }

    /// Returns the `TxGenerator` with the name `gen_name` from the stream with id `stream_id`.
    pub fn get_generator_from_name(&self, stream_id: usize, gen_name: String) -> Option<&TxGenerator> {
        self.tx_streams
            .get(&stream_id)
            .unwrap()
            .generators
            .iter()
            .map(|id| self.tx_generators.get(id).unwrap())
            .find(|gen| gen.name == gen_name)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum Timescale {
    Fs,
    Ps,
    Ns,
    Us,
    Ms,
    S,
    Unit,
    None,
}

impl Timescale {
    pub fn get_timescale(exponent: i64) -> Timescale{
        match exponent {
            0 => S,
            -4 => Ms,
            -8 => Us,
            -12 => Ns,
            -16 => Ps,
            -20 => Fs,
            _ => Unit,
        }
    }
}

impl fmt::Display for Timescale {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}