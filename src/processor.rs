//! Program state processor

use borsh::{BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::{invoke,invoke_signed},
    sysvar::{Sysvar,rent::Rent},
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
};

use crate::{
    instruction::{ProcessSet,ProcessDepositSol,TokenInstruction},
    state::{Payments},
};

pub const PREFIX: &str = "batchv2";
pub struct Processor {}

impl Processor {

    pub fn process_set(program_id: &Pubkey,accounts: &[AccountInfo],number:u64,percent:Vec<u64>) -> ProgramResult 
    {
        //executed once
        let account_info_iter = &mut accounts.iter();
        let payer_account = next_account_info(account_info_iter)?; // payer
        let system_program = next_account_info(account_info_iter)?;
        let vault =next_account_info(account_info_iter)?;
        let pda_data =next_account_info(account_info_iter)?; //account to save data 
     
        //Was the transaction signed by payer account's private key
        if !payer_account.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
        let (vault_address, _bump_seed) = Pubkey::find_program_address(
            &[
                &payer_account.key.to_bytes(),
                PREFIX.as_bytes(),
            ],
            program_id,
        );
        if vault_address!=*vault.key
        {
            return Err(ProgramError::MissingRequiredSignature);
        }
        msg!("The instruction is signed");        
        let rent = Rent::get()?;
        let size: u64=std::mem::size_of::<Payments>() as u64 + number*(std::mem::size_of::<Pubkey>()+2*std::mem::size_of::<u64>()) as u64;
        
        let transfer_amount =  rent.minimum_balance (size as usize);
       //creating the data  account
       msg!("The payment data account is created...");
 
        invoke(
            &system_instruction::create_account(
                payer_account.key,
                pda_data.key,
                transfer_amount,
                size,
                program_id,
            ),
            &[
                payer_account.clone(),
                pda_data.clone(),
                system_program.clone(),
            ],
        )?;

        msg!("The payment account is complete being created");
        let mut pda_start = Payments::from_account(pda_data)?;
        msg!("Data writing...");
        let mut sum:u64=0;

       for i in 0..number as usize
        {
            let payeeee = next_account_info(account_info_iter)?;
           pda_start.payee.push(*payeeee.key);
            msg!("The paying account is :{}",*payeeee.key);
            pda_start.percent.push(percent[i]);
            msg!("The percent is :{}",percent[i]);
            sum+=percent[i];
            pda_start.payment.push(0);

        }
        if sum !=1000000
        {
            msg!("The sum of percentages is not 1000 ");
            return Err(ProgramError::MissingRequiredSignature);
        }
        pda_start.payer=*payer_account.key;
        pda_start.total_amount=0;
        pda_start.serialize(&mut *pda_data.data.borrow_mut())?;
        msg!("Data writing complete");
        Ok(())
    }
    
    pub fn process_claim(program_id: &Pubkey,accounts: &[AccountInfo],)->ProgramResult
    {  
        //claiming the amount

        let account_info_iter = &mut accounts.iter();
        let payee_account =next_account_info(account_info_iter)?;
        let payer_account = next_account_info(account_info_iter)?; 
        let pda_data =next_account_info(account_info_iter)?; 
        let vault=next_account_info(account_info_iter)?; 
        let system_program=next_account_info(account_info_iter)?;
        msg!("Verifying ...");
        let (vault_address, bump_seed) = Pubkey::find_program_address(
            &[
                &payer_account.key.to_bytes(),
                PREFIX.as_bytes(),
            ],
            program_id,
        );
        let pda_signer_seeds: &[&[_]] = &[
            &payer_account.key.to_bytes(),
            PREFIX.as_bytes(),
            &[bump_seed],
        ];
        if pda_data.owner != program_id
        {
            return Err(ProgramError::MissingRequiredSignature);
        } 

        if vault_address!=*vault.key
        {
            return Err(ProgramError::MissingRequiredSignature);
        }
        let mut pda_start = Payments::from_account(pda_data)?;

        if *payer_account.key !=pda_start.payer
        {
            return Err(ProgramError::MissingRequiredSignature);
        }
        let mut total_paid_amount=0;
        let mut percent=0;
        let mut index:usize=0;

        for i in 0..pda_start.payment.len()
        {
           
         if *payee_account.key == pda_start.payee[i]
            {
                percent=pda_start.percent[i];
                index=i;
            }
        total_paid_amount+=pda_start.payment[i];
        }
        let lamports = **vault.try_borrow_lamports()?;
        if total_paid_amount+lamports > pda_start.total_amount
        {
            pda_start.total_amount=total_paid_amount+lamports;
        }
        let amount_to_pay:u64=pda_start.total_amount*percent/1000000-pda_start.payment[index]; //provide the percent in similar fashion

        if percent>0 && amount_to_pay>0
        {
            invoke_signed(
                &system_instruction::transfer(
                    vault.key,
                    payee_account.key,
                    amount_to_pay,
                ),
                &[
                    payee_account.clone(),
                    vault.clone(),
                    system_program.clone()
                ],&[&pda_signer_seeds],
            )?;
            pda_start.payment[index]+=amount_to_pay;

        }
        else
        {
            msg!("Your Account is not valid or you have already taken the payment ");
            return Err(ProgramError::MissingRequiredSignature);
        }      
        pda_start.serialize(&mut *pda_data.data.borrow_mut())?;
        msg!("Successfully Done");
        Ok(())

    }
    pub fn process_deposit_sol(program_id: &Pubkey,accounts: &[AccountInfo],amount:u64,) -> ProgramResult 
    {
        //executed once
        let account_info_iter = &mut accounts.iter();
        let sender=next_account_info(account_info_iter)?; //signer and amount sender
        let original_payer_account = next_account_info(account_info_iter)?; // payer
        let system_program = next_account_info(account_info_iter)?;
        let vault =next_account_info(account_info_iter)?;
    
         //Was the transaction signed by payer account's private key
         if !sender.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
        let (vault_address, _bump_seed) = Pubkey::find_program_address(
            &[
                &original_payer_account.key.to_bytes(),
                PREFIX.as_bytes(),
            ],
            program_id,
        );
        if vault_address!=*vault.key
        {
            return Err(ProgramError::MissingRequiredSignature);
        }
        invoke(
            &system_instruction::transfer(
                sender.key, 
                vault.key,
                 amount, ),
             &[
                 sender.clone(),
                 vault.clone(),
                 system_program.clone(),
             ])?;
             
        Ok(())
    }

     
        
    pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], input: &[u8]) -> ProgramResult {
        let instruction = TokenInstruction::unpack(input)?;
        match instruction {
            TokenInstruction::ProcessSet(ProcessSet{number,percents}) => {
                msg!("Instruction: Sending");
                Self::process_set(program_id, accounts,number,percents)
            }
            TokenInstruction::ProcessClaim => {
                msg!("Instruction: Claim");
                Self::process_claim(program_id, accounts)
            }
            TokenInstruction::ProcessDepositSol(ProcessDepositSol{amount}) => {
                msg!("Instruction: Sending");
                Self::process_deposit_sol(program_id, accounts,amount)
            }
        }
    }
}
