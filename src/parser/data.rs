use super::{Pair, ParsingError, Rule};

fn parse_radix(pair: Pair<'_, Rule>) -> Result<u16, ParsingError> {
    let data = pair.as_span().as_str();
    if data.len() < 3 {
        return Err(ParsingError::UnexpectedToken);
    }
    let radix = match &data[0..=1] {
        "0x" => 16,
        "0b" => 2,
        _ => return Err(ParsingError::UnexpectedToken),
    };
    let value = data.trim_start_matches("0x");
    if let Ok(result) = u16::from_str_radix(value, radix) {
        Ok(result)
    } else {
        Err(ParsingError::NumberConversion(data.into()))
    }
}

fn parse_number(pair: Pair<'_, Rule>) -> Result<u16, ParsingError> {
    let data = pair.as_span().as_str();
    if let Ok(result) = data.parse::<u16>() {
        Ok(result)
    } else {
        Err(ParsingError::NumberConversion(data.into()))
    }
}

pub fn parse_data(pair: Pair<'_, Rule>) -> Result<u16, ParsingError> {
    if let Some(data) = pair.into_inner().next() {
        let mut data = data.into_inner();
        let mut negative = false;
        let mut number = None;
        while let Some(value) = data.next() {
            match value.as_rule() {
                Rule::negative => negative = true,
                Rule::number => number = Some(parse_number(value)?),
                Rule::radix => number = Some(parse_radix(value)?),
                _ => return Err(ParsingError::UnexpectedToken),
            }
        }
        match (number, negative) {
            (Some(n), false) => return Ok(n),
            (Some(n), true) => return Ok(-(n as i16) as u16),
            _ => {}
        }
    }
    return Err(ParsingError::UnexpectedToken);
}
