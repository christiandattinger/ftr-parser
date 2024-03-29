use std::collections::HashMap;
use std::io::Read;
use num_bigint::BigInt;
use crate::ftr_parser::FtrParser;
use crate::types::{FTR, Timescale};


pub fn parse_ftr<R: Read>(reader: R) -> Result<FTR, String>{

    let mut ftr = FTR{
        time_scale: Timescale::None,
        str_dict: HashMap::new(),
        tx_streams: vec![],
        max_timestamp: BigInt::from(0),
        /*tx_generators: vec![],
        tx_blocks: vec![],
        tx_relations: vec![],*/
    };
    let mut ftr_parser = FtrParser::new(&mut ftr);

    ftr_parser.load(reader);

    Ok(ftr)


}