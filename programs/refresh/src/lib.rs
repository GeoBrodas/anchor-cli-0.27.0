use anchor_lang::error::ErrorCode;
use anchor_lang::prelude::*;

declare_id!("AWHB3t2wToeEkSXNBMJuf8qWuYURcrJcuZ8Lb3gQ2wt2");

#[program]
pub mod refresh {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn create_message(ctx: Context<CreateMessage>, message: String) -> Result<()> {
        let message_account = &mut ctx.accounts.message;

        message_account.author = *ctx.accounts.user.key;

        message_account.message = message;

        Ok(())
    }

    pub fn update_message(ctx: Context<UpdateMessage>, message: String) -> Result<()> {
        let message_account = &mut ctx.accounts.message;

        if message_account.author != *ctx.accounts.user.key {
            return Err(ErrorCode::AccountNotSigner.into());
        }

        message_account.message = message;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct CreateMessage<'info> {
    #[account(init, payer=user, space=200)]
    pub message: Account<'info, Message>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateMessage<'info> {
    #[account(mut)]
    pub message: Account<'info, Message>,

    pub user: Signer<'info>,
}

#[account]
pub struct Message {
    pub author: Pubkey,
    pub message: String,
}
