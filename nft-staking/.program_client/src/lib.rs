// DO NOT EDIT - automatically generated file (except `use` statements inside the `*_instruction` module
pub mod nft_staking_instruction {
    use trident_client::*;
    pub static PROGRAM_ID: Pubkey = Pubkey::new_from_array([
        155u8, 177u8, 131u8, 164u8, 215u8, 4u8, 26u8, 37u8, 80u8, 219u8, 248u8, 142u8, 85u8, 213u8,
        122u8, 203u8, 83u8, 204u8, 38u8, 60u8, 229u8, 213u8, 238u8, 54u8, 12u8, 198u8, 167u8,
        251u8, 124u8, 41u8, 84u8, 104u8,
    ]);
    pub async fn initialize_config(
        client: &Client,
        i_points_per_stake: u8,
        i_max_stake: u8,
        i_freeze_period: u32,
        a_admin: Pubkey,
        a_config: Pubkey,
        a_rewards_mint: Pubkey,
        a_system_program: Pubkey,
        a_token_program: Pubkey,
        signers: impl IntoIterator<Item = Keypair> + Send + 'static,
    ) -> Result<EncodedConfirmedTransactionWithStatusMeta, ClientError> {
        client
            .send_instruction(
                PROGRAM_ID,
                nft_staking::instruction::InitializeConfig {
                    points_per_stake: i_points_per_stake,
                    max_stake: i_max_stake,
                    freeze_period: i_freeze_period,
                },
                nft_staking::accounts::InitializeConfig {
                    admin: a_admin,
                    config: a_config,
                    rewards_mint: a_rewards_mint,
                    system_program: a_system_program,
                    token_program: a_token_program,
                },
                signers,
            )
            .await
    }
    pub fn initialize_config_ix(
        i_points_per_stake: u8,
        i_max_stake: u8,
        i_freeze_period: u32,
        a_admin: Pubkey,
        a_config: Pubkey,
        a_rewards_mint: Pubkey,
        a_system_program: Pubkey,
        a_token_program: Pubkey,
    ) -> Instruction {
        Instruction {
            program_id: PROGRAM_ID,
            data: nft_staking::instruction::InitializeConfig {
                points_per_stake: i_points_per_stake,
                max_stake: i_max_stake,
                freeze_period: i_freeze_period,
            }
            .data(),
            accounts: nft_staking::accounts::InitializeConfig {
                admin: a_admin,
                config: a_config,
                rewards_mint: a_rewards_mint,
                system_program: a_system_program,
                token_program: a_token_program,
            }
            .to_account_metas(None),
        }
    }
    pub async fn initialize_user_account(
        client: &Client,
        a_user: Pubkey,
        a_user_account: Pubkey,
        a_system_program: Pubkey,
        signers: impl IntoIterator<Item = Keypair> + Send + 'static,
    ) -> Result<EncodedConfirmedTransactionWithStatusMeta, ClientError> {
        client
            .send_instruction(
                PROGRAM_ID,
                nft_staking::instruction::InitializeUserAccount {},
                nft_staking::accounts::Initialize {
                    user: a_user,
                    user_account: a_user_account,
                    system_program: a_system_program,
                },
                signers,
            )
            .await
    }
    pub fn initialize_user_account_ix(
        a_user: Pubkey,
        a_user_account: Pubkey,
        a_system_program: Pubkey,
    ) -> Instruction {
        Instruction {
            program_id: PROGRAM_ID,
            data: nft_staking::instruction::InitializeUserAccount {}.data(),
            accounts: nft_staking::accounts::Initialize {
                user: a_user,
                user_account: a_user_account,
                system_program: a_system_program,
            }
            .to_account_metas(None),
        }
    }
    pub async fn stake(
        client: &Client,
        a_user: Pubkey,
        a_mint: Pubkey,
        a_collection: Pubkey,
        a_mint_ata: Pubkey,
        a_metadata: Pubkey,
        a_edition: Pubkey,
        a_config: Pubkey,
        a_user_account: Pubkey,
        a_stake_account: Pubkey,
        a_token_program: Pubkey,
        a_system_program: Pubkey,
        a_metadata_program: Pubkey,
        signers: impl IntoIterator<Item = Keypair> + Send + 'static,
    ) -> Result<EncodedConfirmedTransactionWithStatusMeta, ClientError> {
        client
            .send_instruction(
                PROGRAM_ID,
                nft_staking::instruction::Stake {},
                nft_staking::accounts::Stake {
                    user: a_user,
                    mint: a_mint,
                    collection: a_collection,
                    mint_ata: a_mint_ata,
                    metadata: a_metadata,
                    edition: a_edition,
                    config: a_config,
                    user_account: a_user_account,
                    stake_account: a_stake_account,
                    token_program: a_token_program,
                    system_program: a_system_program,
                    metadata_program: a_metadata_program,
                },
                signers,
            )
            .await
    }
    pub fn stake_ix(
        a_user: Pubkey,
        a_mint: Pubkey,
        a_collection: Pubkey,
        a_mint_ata: Pubkey,
        a_metadata: Pubkey,
        a_edition: Pubkey,
        a_config: Pubkey,
        a_user_account: Pubkey,
        a_stake_account: Pubkey,
        a_token_program: Pubkey,
        a_system_program: Pubkey,
        a_metadata_program: Pubkey,
    ) -> Instruction {
        Instruction {
            program_id: PROGRAM_ID,
            data: nft_staking::instruction::Stake {}.data(),
            accounts: nft_staking::accounts::Stake {
                user: a_user,
                mint: a_mint,
                collection: a_collection,
                mint_ata: a_mint_ata,
                metadata: a_metadata,
                edition: a_edition,
                config: a_config,
                user_account: a_user_account,
                stake_account: a_stake_account,
                token_program: a_token_program,
                system_program: a_system_program,
                metadata_program: a_metadata_program,
            }
            .to_account_metas(None),
        }
    }
    pub async fn unstake(
        client: &Client,
        a_user: Pubkey,
        a_mint: Pubkey,
        a_collection: Pubkey,
        a_mint_ata: Pubkey,
        a_metadata: Pubkey,
        a_edition: Pubkey,
        a_config: Pubkey,
        a_user_account: Pubkey,
        a_stake_account: Pubkey,
        a_token_program: Pubkey,
        a_system_program: Pubkey,
        a_metadata_program: Pubkey,
        signers: impl IntoIterator<Item = Keypair> + Send + 'static,
    ) -> Result<EncodedConfirmedTransactionWithStatusMeta, ClientError> {
        client
            .send_instruction(
                PROGRAM_ID,
                nft_staking::instruction::Unstake {},
                nft_staking::accounts::Stake {
                    user: a_user,
                    mint: a_mint,
                    collection: a_collection,
                    mint_ata: a_mint_ata,
                    metadata: a_metadata,
                    edition: a_edition,
                    config: a_config,
                    user_account: a_user_account,
                    stake_account: a_stake_account,
                    token_program: a_token_program,
                    system_program: a_system_program,
                    metadata_program: a_metadata_program,
                },
                signers,
            )
            .await
    }
    pub fn unstake_ix(
        a_user: Pubkey,
        a_mint: Pubkey,
        a_collection: Pubkey,
        a_mint_ata: Pubkey,
        a_metadata: Pubkey,
        a_edition: Pubkey,
        a_config: Pubkey,
        a_user_account: Pubkey,
        a_stake_account: Pubkey,
        a_token_program: Pubkey,
        a_system_program: Pubkey,
        a_metadata_program: Pubkey,
    ) -> Instruction {
        Instruction {
            program_id: PROGRAM_ID,
            data: nft_staking::instruction::Unstake {}.data(),
            accounts: nft_staking::accounts::Stake {
                user: a_user,
                mint: a_mint,
                collection: a_collection,
                mint_ata: a_mint_ata,
                metadata: a_metadata,
                edition: a_edition,
                config: a_config,
                user_account: a_user_account,
                stake_account: a_stake_account,
                token_program: a_token_program,
                system_program: a_system_program,
                metadata_program: a_metadata_program,
            }
            .to_account_metas(None),
        }
    }
    pub async fn claim(
        client: &Client,
        a_user: Pubkey,
        a_user_account: Pubkey,
        a_stake_account: Pubkey,
        a_rewards_mint: Pubkey,
        a_user_ata: Pubkey,
        a_config: Pubkey,
        a_system_program: Pubkey,
        a_token_program: Pubkey,
        a_associated_token_program: Pubkey,
        signers: impl IntoIterator<Item = Keypair> + Send + 'static,
    ) -> Result<EncodedConfirmedTransactionWithStatusMeta, ClientError> {
        client
            .send_instruction(
                PROGRAM_ID,
                nft_staking::instruction::Claim {},
                nft_staking::accounts::Claim {
                    user: a_user,
                    user_account: a_user_account,
                    stake_account: a_stake_account,
                    rewards_mint: a_rewards_mint,
                    user_ata: a_user_ata,
                    config: a_config,
                    system_program: a_system_program,
                    token_program: a_token_program,
                    associated_token_program: a_associated_token_program,
                },
                signers,
            )
            .await
    }
    pub fn claim_ix(
        a_user: Pubkey,
        a_user_account: Pubkey,
        a_stake_account: Pubkey,
        a_rewards_mint: Pubkey,
        a_user_ata: Pubkey,
        a_config: Pubkey,
        a_system_program: Pubkey,
        a_token_program: Pubkey,
        a_associated_token_program: Pubkey,
    ) -> Instruction {
        Instruction {
            program_id: PROGRAM_ID,
            data: nft_staking::instruction::Claim {}.data(),
            accounts: nft_staking::accounts::Claim {
                user: a_user,
                user_account: a_user_account,
                stake_account: a_stake_account,
                rewards_mint: a_rewards_mint,
                user_ata: a_user_ata,
                config: a_config,
                system_program: a_system_program,
                token_program: a_token_program,
                associated_token_program: a_associated_token_program,
            }
            .to_account_metas(None),
        }
    }
}
