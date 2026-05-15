use anchor_lang::prelude::*;

declare_id!("AGvUE1HbzuDutZRQQS3QWJ1PQQ36wZMyqBCzE9G9WDW5");

#[program]
pub mod voting {
    use super::*;

    pub initialize_poll(ctx: Context<InitializePoll>) {
        let poll = &mut ctx.accounts.poll;
        poll.name = name;
        poll.description = description;
        poll.start_date = start_date;
        poll.end_date = end_date;
        poll.option_index = option_index;
    }
}


#[account]
#[derive(InitSpace)]
pub struct Poll {
    #[max_len(32)]
    pub name: String,

    #[max_len(32)]
    pub description: String,

    pub start_date: i64,
    pub end_date: i64,

    pub option_index: i64,
}

#[account]
#[derive(InitSpace)]
pub struct Candidate {
    #[max_len(32)]
    pub name: String,

    pub votes: u64,

}

#[derive(Accounts)]
pub struct InitializePoll<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        space = 8 + Poll::INIT_SPACE,
        seeds = [b"poll", signer.key().as_ref()],
        bump
    )]
    pub poll: Account<'info, Poll>,

    pub system_program: Program<'info, System>,
}
