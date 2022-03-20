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
     
    pub payment: Vec<Payee>,
    pub payer: Pubkey,
    pub total_amount: u64,
}
#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct Payee {
     
    pub payee: Pubkey,
    pub percent:u64, // 100% = 1000000
    pub payment:u64,
}
impl Payments {
    pub fn from_account(account:&AccountInfo)-> Result<Payments, ProgramError> {
            let md: Payments =try_from_slice_unchecked(&account.data.borrow_mut())?;
            Ok(md)
    }
}
