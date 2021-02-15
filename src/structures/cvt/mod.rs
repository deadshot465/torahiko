use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ConversionTable {
    pub length: HashMap<String, HashMap<String, f64>>,
    pub weight: HashMap<String, HashMap<String, f64>>,
    pub temperature: HashMap<String, HashMap<String, f64>>,
}
