//! Instruction types
use solana_program::{program_error::ProgramError,msg};


use crate::{
    error::TokenError,
};
use std::convert::TryInto;


pub struct ProcessSet {
    pub number: u64,
    pub percents:Vec<u64>

}

pub struct ProcessDepositSol {
    pub amount: u64,

}

pub enum TokenInstruction {
    ProcessSet(ProcessSet),
    ProcessClaim,
    ProcessDepositSol(ProcessDepositSol),
}

impl TokenInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        use TokenError::InvalidInstruction;
        let (&tag, rest) = input.split_first().ok_or(InvalidInstruction)?;
        msg!("The rest is {:?}",rest);
        Ok(match tag {
            0 => {             
                let number =  (rest[4..].len() / std::mem::size_of::<u64>()) as u64;
                let mut percents: Vec<u64> = Vec::with_capacity(std::mem::size_of::<u64>()*(number as usize));
                let mut offset=4;
                for _ in 0..number {
                    let percent = rest
                        .get(offset..offset + 8)
                        .and_then(|slice| slice.try_into().ok())
                        .map(u64::from_le_bytes)
                        .ok_or(InvalidInstruction)?;
                    percents.push(percent);
                    msg!("The amount is {}",percent);
                    msg!("The offset is {}",offset);
                    offset=offset+8;
                }                
                Self::ProcessSet(ProcessSet{number,percents})
            }
            1 => {
               Self::ProcessClaim
            }
            2=> {
                let (amount,_rest) = rest.split_at(8); //amount
                let amount = amount.try_into().map(u64::from_le_bytes).or(Err(InvalidInstruction))?;
                msg!("The number is {}",amount);
                Self::ProcessDepositSol(ProcessDepositSol{amount})

            }
            _ => return Err(TokenError::InvalidInstruction.into()),
        })
    }
}
