use anchor_lang::prelude::*;

declare_id!("35WSnRPLrqKeZWcXmq65vfiJ4ns1CAAEtSiC91vYhApr");

#[program]
pub mod order {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>, 
        nam: String, 
        contact: String,
        sizeb: String,
        quanb: String,
        sizew: String,
        quanw: String,
        quanl: String,
        quanp: String
    ) 
    -> Result<()> {
        let purchase: &mut Account<'_, purchase> = &mut ctx.accounts.purchase;
        purchase.nam = nam;
        purchase.contact = contact;
        purchase.sizeb = sizeb;
        purchase.quanb = quanb;
        purchase.sizew = sizew;
        purchase.quanw = quanw;
        purchase.quanl = quanl;
        purchase.quanp = quanp;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        space = 8 + purchase::INIT_SPACE
    )]

    pub purchase: Account<'info, purchase>,
    pub system_program: Program<'info, System>
}

#[account]
#[derive(InitSpace)]
pub struct purchase {
    #[max_len(100)]
    pub nam: String,
    #[max_len(100)]
    pub contact: String,
    #[max_len(100)]
    pub sizeb: String,
    #[max_len(100)]
    pub quanb: String,
    #[max_len(100)]
    pub sizew: String,
    #[max_len(100)]
    pub quanw: String,
    #[max_len(100)]
    pub quanl: String,
    #[max_len(100)]
    pub quanp: String,
}
