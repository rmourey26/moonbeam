// Copyright 2019-2020 PureStake Inc.
// This file is part of Moonbeam.

// Moonbeam is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Moonbeam is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Moonbeam.  If not, see <http://www.gnu.org/licenses/>.

use crate::chain_spec;
use crate::cli::{Cli, Subcommand};
use crate::service;
use crate::service::new_partial;
use sc_cli::{ChainSpec, Role, RuntimeVersion, SubstrateCli};
use sc_service::PartialComponents;
use account::EthereumSigner;
use sp_core::{ecdsa, Pair};
use sp_runtime::traits::IdentifyAccount;

impl SubstrateCli for Cli {
	fn impl_name() -> String {
		"Substrate Node".into()
	}

	fn impl_version() -> String {
		env!("SUBSTRATE_CLI_IMPL_VERSION").into()
	}

	fn description() -> String {
		env!("CARGO_PKG_DESCRIPTION").into()
	}

	fn author() -> String {
		env!("CARGO_PKG_AUTHORS").into()
	}

	fn support_url() -> String {
		"support.anonymous.an".into()
	}

	fn copyright_start_year() -> i32 {
		2017
	}

	fn load_spec(&self, id: &str) -> Result<Box<dyn sc_service::ChainSpec>, String> {
		Ok(match id {
			"dev" | "development" => Box::new(chain_spec::development_chain_spec()?),
			"" | "local" => Box::new(chain_spec::local_testnet_chain_spec()?),
			path => Box::new(chain_spec::ChainSpec::from_json_file(
				std::path::PathBuf::from(path),
			)?),
		})
	}

	fn native_runtime_version(_: &Box<dyn ChainSpec>) -> &'static RuntimeVersion {
		&moonbeam_runtime::VERSION
	}
}

/// Parse and run command line arguments
pub fn run() -> sc_cli::Result<()> {
	let cli = Cli::from_args();
	match &cli.subcommand {
		Some(Subcommand::BuildSpec(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.sync_run(|config| cmd.run(config.chain_spec, config.network))
		}
		Some(Subcommand::CheckBlock(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|config| {
				let PartialComponents {
					client,
					task_manager,
					import_queue,
					..
				} = new_partial(&config, cli.run.manual_seal, None)?;
				Ok((cmd.run(client, import_queue), task_manager))
			})
		}
		Some(Subcommand::ExportBlocks(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|config| {
				let PartialComponents {
					client,
					task_manager,
					..
				} = new_partial(&config, cli.run.manual_seal, None)?;
				Ok((cmd.run(client, config.database), task_manager))
			})
		}
		Some(Subcommand::ExportState(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|config| {
				let PartialComponents {
					client,
					task_manager,
					..
				} = new_partial(&config, cli.run.manual_seal, None)?;
				Ok((cmd.run(client, config.chain_spec), task_manager))
			})
		}
		Some(Subcommand::ImportBlocks(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|config| {
				let PartialComponents {
					client,
					task_manager,
					import_queue,
					..
				} = new_partial(&config, cli.run.manual_seal, None)?;
				Ok((cmd.run(client, import_queue), task_manager))
			})
		}
		Some(Subcommand::PurgeChain(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.sync_run(|config| cmd.run(config.database))
		}
		Some(Subcommand::Revert(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|config| {
				let PartialComponents {
					client,
					task_manager,
					backend,
					..
				} = new_partial(&config, cli.run.manual_seal, None)?;
				Ok((cmd.run(client, backend), task_manager))
			})
		}
		None => {
			let runner = cli.create_runner(&cli.run.base)?;

			// Supply the correct author id for wellknown validators.
			// This isn't super elegant, but the alternative is modifying Substrate
			// and this will go away when we start signing blocks
			let author_id = if cli.run.base.shared_params.dev {
				let alice_public = ecdsa::Pair::from_string("//Alice", None)
					.expect("Alice is a valid phrase")
					.public();
				Some(EthereumSigner::from(alice_public).into_account())
			} else if cli.run.base.alice {
				let alice_public = ecdsa::Pair::from_string("//Alice", None)
					.expect("Alice is a valid phrase")
					.public();
				Some(EthereumSigner::from(alice_public).into_account())
			} else if cli.run.base.bob {
				let alice_public = ecdsa::Pair::from_string("//Bob", None)
					.expect("Alice is a valid phrase")
					.public();
				Some(EthereumSigner::from(alice_public).into_account())
			} else if cli.run.base.charlie {
				let alice_public = ecdsa::Pair::from_string("//Charlie", None)
					.expect("Alice is a valid phrase")
					.public();
				Some(EthereumSigner::from(alice_public).into_account())
			} else if cli.run.base.dave {
				let alice_public = ecdsa::Pair::from_string("//Dave", None)
					.expect("Alice is a valid phrase")
					.public();
				Some(EthereumSigner::from(alice_public).into_account())
			} else if cli.run.base.eve {
				let alice_public = ecdsa::Pair::from_string("//Eve", None)
					.expect("Alice is a valid phrase")
					.public();
				Some(EthereumSigner::from(alice_public).into_account())
			} else if cli.run.base.ferdie {
				let alice_public = ecdsa::Pair::from_string("//Ferdie", None)
					.expect("Alice is a valid phrase")
					.public();
				Some(EthereumSigner::from(alice_public).into_account())
			} else {
				// Leave it as it was
				cli.run.author_id
			};

			if cli.run.base.validator && author_id.is_none() {
				return Err("Authoring nodes must specify an author account id".into());
			}

			runner.run_node_until_exit(|config| async move {
				match config.role {
					Role::Light => service::new_light(config),
					_ => service::new_full(config, cli.run.manual_seal, author_id),
				}
			})
		}
	}
}
