mod ParsedResult;
mod ResultParser;
mod TelParsedResult;
mod TextParsedResult;
mod ParsedResultType;
mod TelResultParser;
mod ISBNParsedResult;
mod ISBNResultParser;
mod WifiParsedResult;
mod WifiResultParser;
mod GeoResultParser;
mod GeoParsedResult;

use std::fmt;

pub use ParsedResultType::*;
pub use ResultParser::*;
pub use TelParsedResult::*;
pub use TextParsedResult::*;
pub use ParsedResult::*;
pub use TelResultParser::*;
pub use ISBNParsedResult::*;
pub use ISBNResultParser::*;
pub use WifiParsedResult::*;
pub use WifiResultParser::*;
pub use GeoParsedResult::*;
pub use GeoResultParser::*;


#[cfg(test)]
mod TelParsedResultTestCase;
#[cfg(test)]
mod ISBNParsedResultTestCase;
#[cfg(test)]
mod WifiParsedResultTestCase;
#[cfg(test)]
mod GeoParsedResultTestCase;

pub enum ParsedClientResult {
    TextResult(TextParsedRXingResult),
    TelResult(TelParsedRXingResult),
    ISBNResult(ISBNParsedRXingResult),
    WiFiResult(WifiParsedRXingResult),
    GeoResult(GeoParsedRXingResult),
}

impl ParsedRXingResult for ParsedClientResult {
    fn getType(&self) -> ParsedRXingResultType {
        match self {
            ParsedClientResult::TextResult(a) => a.getType(),
            ParsedClientResult::TelResult(a) => a.getType(),
            ParsedClientResult::ISBNResult(a) => a.getType(),
            ParsedClientResult::WiFiResult(a) => a.getType(),
            ParsedClientResult::GeoResult(a) => a.getType(),
            
            
        }
    }

    fn getDisplayRXingResult(&self) -> String {
        match self {
            ParsedClientResult::TextResult(a) => a.getDisplayRXingResult(),
            ParsedClientResult::TelResult(a) => a.getDisplayRXingResult(),
            ParsedClientResult::ISBNResult(a) => a.getDisplayRXingResult(),
            ParsedClientResult::WiFiResult(a) => a.getDisplayRXingResult(),
            ParsedClientResult::GeoResult(a) => a.getDisplayRXingResult(),

            
            
        }
    }
}

impl fmt::Display for ParsedClientResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{}", self.getDisplayRXingResult())
    }
}