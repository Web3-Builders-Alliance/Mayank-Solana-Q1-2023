use anchor_lang::prelude::*;

//declare_id! macro is used to generate a unique identifier for the program
declare_id!("EnjN3cm7xYqYHNUZbQfhJYj5S5RBrSU9tc5aHwQ6LqvT");

//an achor program with 2 functions "initialize" with and "switch_power"
#[program]
pub mod lever {
    use super::*;
    pub fn initialize(_ctx: Context<InitializeLever>) -> Result<()> {
        Ok(())
    } 

    //function to change is_on's value
    pub fn switch_power(ctx: Context<SetPowerStatus>, name: String) -> Result<()> {
        
        let power = &mut ctx.accounts.power;
        power.is_on = !power.is_on; 

        msg!("{} is pulling the power switch!", &name);

        match power.is_on {
            true => msg!("The power is now on."),
            false => msg!("The power is now off!"),
        };

        Ok(())
    }
}


#[derive(Accounts)]
pub struct InitializeLever<'info> {
    #[account(init, payer = user, space = 8 + 8)] //This account is created and initialized with the #[account(init)] attribute,
    pub power: Account<'info, PowerStatus>, //which specifies that the account should be created if it does not exist and initialized with the PowerStatus struct
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetPowerStatus<'info> {
    #[account(mut)]
    pub power: Account<'info, PowerStatus>,
}

#[account]
pub struct PowerStatus {
    pub is_on: bool,
}