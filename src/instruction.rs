//! Instruction types
use solana_program::{program_error::ProgramError,msg};


use crate::{
    error::TokenError,
};
use std::convert::TryInto;


pub struct ProcessSet {
    pub number: u64,
    pub amounts:Vec<u64>

}

pub enum TokenInstruction {
    ProcessSet(ProcessSet),
    ProcessClaim,
}

impl TokenInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        use TokenError::InvalidInstruction;
        let (&tag, rest) = input.split_first().ok_or(InvalidInstruction)?;
        msg!("The rest is {:?}",rest);
        Ok(match tag {
            0 => {             
                let number =  (rest[4..].len() / std::mem::size_of::<u64>()) as u64;
                let mut amounts: Vec<u64> = Vec::with_capacity(std::mem::size_of::<u64>()*(number as usize));
                let mut offset=4;
                for _ in 0..number {
                    let amount = rest
                        .get(offset..offset + 8)
                        .and_then(|slice| slice.try_into().ok())
                        .map(u64::from_le_bytes)
                        .ok_or(InvalidInstruction)?;
                    amounts.push(amount);
                    msg!("The amount is {}",amount);
                    msg!("The offset is {}",offset);
                    offset=offset+8;
                }                
                Self::ProcessSet(ProcessSet{number,amounts})
            }
            1 => {
               Self::ProcessClaim
            }
            _ => return Err(TokenError::InvalidInstruction.into()),
        })
    }
}
