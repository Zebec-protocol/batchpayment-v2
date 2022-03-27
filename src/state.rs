///into state.rs
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info:: AccountInfo,
    program_error::ProgramError,
    pubkey::Pubkey,
    borsh::try_from_slice_unchecked,}; 
#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct Payments {
     
    pub payee: Vec<Pubkey>,
    pub percent:Vec<u64>,
    pub payment:Vec<u64>,
    pub payer: Pubkey,
    pub total_amount: u64,
}
impl Payments {
    pub fn from_account(account:&AccountInfo)-> Result<Payments, ProgramError> {
            let md: Payments =try_from_slice_unchecked(&account.data.borrow_mut())?;
            Ok(md)
    }
}
