use anchor_lang::prelude::*;

declare_id!("6CC77FLNVygHZqEcZMCUjzJ6nAAuZa7gT4XicL2drrWC");

#[program]
pub mod without_pda {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, content: String) -> Result<()> {
        let post: &mut Account<'_, Post>= &mut ctx.accounts.post;
        post.content = content;
        Ok(())
    }

    pub fn update_post(ctx: Context<UpdatePostAccount>, content: String) -> Result<()> {
        let post_account: &mut Account<'_, Post> = &mut ctx.accounts.post;
        post_account.content = content;

        Ok(())
    }
}

#[derive(Accounts)]

pub struct UpdatePostAccount<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub post: Account<'info, Post>,
}

#[derive(Accounts)]

pub struct Initialize<'info> {
    #[account(mut)]                             // account METADATA
    pub payer: Signer<'info>,                   // payer is the one paying for the transaction
    #[account(                                  // PATTERN (that you'll always use): initialize/create a new account, set who pays, how much space is needed to be allocated
        init,                               
        payer = payer,                      
        space = 8 + Post:: INIT_SPACE
    )]
    pub post: Account<'info, Post>,
    pub system_program: Program<'info, System>, // don't have to think about system_program, only needed to create accounts
                                                //      not necessarily executable even if it has system_program, 
                                                //      literally system_program is just used for making the account
}

#[account]  // explicitly saying that this is meant for an account
            // makes the program understand that this struct Post 
            // is the structure of an account in your program
#[derive(InitSpace)]    // structure of an account
                        // dictates how much space you need
pub struct Post { // account Post = similar to a table in SQL
    #[max_len(50)]
    pub content: String,
}