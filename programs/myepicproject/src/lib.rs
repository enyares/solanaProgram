use anchor_lang::prelude::*;

declare_id!("CJ9gp6GkxwseDmEQ1fA5BLN2frsciAzUC5TtQvU4idwf");

#[program]
pub mod myepicproject {
    use super::*;

    pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> Result<()> {
        // Get a reference to the account.
        let base_account = &mut ctx.accounts.base_account;
        // Initialize total_gifs.

        base_account.total_gifs = 0;
        Ok(())
    }
    pub fn add_gif(ctx: Context<AddGif>, gif_link: String, another_value: String) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;

        // Build the struct.
        let item = ItemStruct {
            gif_link: gif_link.to_string(),
            user_address: *user.to_account_info().key,
            another_value: another_value.to_string(),
        };

        // Add it to the gif_list vector.
        base_account.gif_list.push(item);
        base_account.total_gifs += 1;
        Ok(())
    }
    pub fn del_gif(ctx: Context<AddGif>) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;

        // Build the struct.

        // Add it to the gif_list vector.
        base_account.gif_list.pop();
        base_account.total_gifs -= 1;
        Ok(())
    }
    pub fn update_gif(ctx: Context<AddGif>, another_value: String) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;
        let user_address = *user.to_account_info().key;
        // Find the index of the first GIF with the matching user_address.
        let index = base_account
            .gif_list
            .iter()
            .position(|item| item.user_address == user_address);

        // Check if the GIF was found.
        match index {
            Some(i) => {
                // Access the GIF with the matching user_address.
                let gif = &mut base_account.gif_list[i];
                // Update the another_value field of the GIF.
                gif.another_value = another_value.to_string();
                // Do something with the updated GIF.
                println!(
                    "Updated GIF with user_address {}: {}",
                    user_address, gif.gif_link
                );
            }
            None => {
                // Handle the case where the GIF wasn't found.
                println!("No GIF found with user_address {}", user_address);
            }
        }
        Ok(())
    }
}

// Attach certain variables to the StartStuffOff context.
#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    #[account(init, payer = user, space = 10000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
// Add the signer who calls the AddGif method to the struct so that we can save it
#[derive(Accounts)]
pub struct AddGif<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}
// Create a custom struct for us to work with.
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub gif_link: String,
    pub user_address: Pubkey,
    pub another_value: String,
}
// Tell Solana what we want to store on this account.
#[account]
pub struct BaseAccount {
    // Attach a Vector of type ItemStruct to the account.
    pub gif_list: Vec<ItemStruct>,
    pub total_gifs: u64,
}
