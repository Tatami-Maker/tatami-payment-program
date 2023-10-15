use anchor_lang::{prelude::*, solana_program::{pubkey, pubkey::Pubkey}};

declare_id!("BTy2uHY6iynWB9EJDVwasG9pxMf2mpEeMBLS9C8yu3UA");

#[constant]
pub const OWNER: Pubkey = pubkey!("3DvJWcHhtdhNLWMeBCh2Rma5chxyDWxoMmVvBFLihMZe");

#[program]
pub mod tatami_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, _index: u64) -> Result<()> {
        ctx.accounts.receipt.initialized = true;
        Ok(())
    }

    pub fn withdraw_payment(ctx: Context<Withdraw>, _index: u64) -> Result<()> {
        let balance = ctx.accounts.receipt.lamports();

        **ctx.accounts.receipt.to_account_info().try_borrow_mut_lamports()? -= balance;
        **ctx.accounts.signer.to_account_info().try_borrow_mut_lamports()? -= balance;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(index: u64)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = signer,
        space = 8 + 1,
        seeds = [
            &index.to_le_bytes(),
            b"receipt".as_ref()
        ],
        bump
    )]
    pub receipt: Account<'info, Receipt>,

    #[account(
        mut,
        address = OWNER
    )]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
#[instruction(index: u64)]
pub struct Withdraw<'info> {
    #[account(
        mut,
        seeds = [
            &index.to_le_bytes(),
            b"receipt"
        ],
        bump
    )]
    pub receipt: SystemAccount<'info>,

    #[account(
        mut,
        address = OWNER
    )]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>
}

#[account]
pub struct Receipt {
    initialized: bool
}