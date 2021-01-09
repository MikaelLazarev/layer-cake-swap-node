use frame_system::offchain::{Signer, SendSignedTransaction};
use frame_support::debug;

use super::{Error, Module, Trait, Call};
use crate::methods::ContractMethod;

impl<T: Trait> Module<T> {
	pub fn offchain_signed_tx(block_number: T::BlockNumber) -> Result<(), Error<T>> {


		// Translating the current block number to number and submit it on-chain
		// let number: u64 = block_number.try_into().unwrap_or(0) as u64;
		let number = Self::get_last_eth_block()?;

		let txs = Self::get_erc20transfer_events("6b175474e89094c44da98b954eedeac495271d0f", number - 3, number)?;

		debug::info!("[Offchain worker]:: Got {} commands!", txs.len());

		// Flag that error happens during sending tx
		let mut error_occured = false;


		// ToDo: Add state to store all commands
		for cmd in txs {
			debug::info!("Got results: {}", &cmd);
			match Self::send_command(cmd) {
				Ok(_) => {}
				Err(e) => {
					debug::error!("Error occured during sending transactions");
					error_occured = true;
				}
			}
		}

		match error_occured {
			false => Ok(()),
			true => Err(<Error<T>>::OffchainSignedTxError)
		}
	}

	fn send_command(cmd: ContractMethod) -> Result<(), Error<T>> {
		// We retrieve a signer and check if it is valid.
		//   Since this pallet only has one key in the keystore. We use `any_account()1 to
		//   retrieve it. If there are multiple keys and we want to pinpoint it, `with_filter()` can be chained,
		//   ref: https://substrate.dev/rustdocs/v2.0.0/frame_system/offchain/struct.Signer.html
		let signer = Signer::<T, T::AuthorityId>::any_account();

		debug::info!("Send command: {}", &cmd);

		// `result` is in the type of `Option<(Account<T>, Result<(), ()>)>`. It is:
		//   - `None`: no account is available for sending transaction
		//   - `Some((account, Ok(())))`: transaction is successfully sent
		//   - `Some((account, Err(())))`: error occured when sending the transaction
		let result = match cmd {
			ContractMethod::DepositToken(sa) => {
				debug::info!("Try to deposit_token in match!");
				signer.send_signed_transaction(|_acct|
					// This is the on-chain function
					Call::deposit_token(sa.clone())
				)
			}
			ContractMethod::DepositETH(sa) => {
				signer.send_signed_transaction(|_acct|
					// This is the on-chain function
					Call::deposit_token(sa.clone())
				)
			}
			ContractMethod::Withdraw(_) => {
				signer.send_signed_transaction(|_acct|
					// This is the on-chain function
					Call::set_value(32u32))
			}
			ContractMethod::SwapToToken(_) => {
				signer.send_signed_transaction(|_acct|
					// This is the on-chain function
					Call::set_value(32u32))
			}
			ContractMethod::SwapToETH(_) => {
				signer.send_signed_transaction(|_acct|
					// This is the on-chain function
					Call::set_value(32u32))
			}
			ContractMethod::AddLiquidity(_) => {
				signer.send_signed_transaction(|_acct|
					// This is the on-chain function
					Call::set_value(32u32))
			}
			ContractMethod::WithdrawLiquidity => {
				signer.send_signed_transaction(|_acct|
					// This is the on-chain function
					Call::set_value(32u32))
			}
		};

		// Display error if the signed tx fails.
		if let Some((acc, res)) = result {
			if res.is_err() {
				debug::error!("failure: offchain_signed_tx: tx sent: {:?}", acc.id);
				return Err(<Error<T>>::OffchainSignedTxError);
			}
			// Transaction is sent successfully
			return Ok(());
		}

		// The case of `None`: no account is available for sending
		debug::error!("No local account available");
		Err(<Error<T>>::NoLocalAcctForSigning)
	}
}