initSidebarItems({"constant":[["GAS_PER_SECOND","Current approximation of the gas/s consumption considering EVM execution over compiled WASM (on 4.4Ghz CPU). Given the 500ms Weight, from which 75% only are used for transactions, the total EVM execution gas limit is: GAS_PER_SECOND * 0.500 * 0.75 ~= 15_000_000."],["GLMR","GLMR, the native token, uses 18 decimals of precision."],["MAXIMUM_BLOCK_WEIGHT","Maximum weight per block"],["VERSION","This runtime version."],["WASM_BINARY",""],["WASM_BINARY_BLOATY",""],["WEIGHT_PER_GAS","Approximate ratio of the amount of Weight per Gas. u64 works for approximations because Weight is a very small unit compared to gas."]],"enum":[["Call",""],["Event",""],["OriginCaller",""]],"fn":[["native_version","The version information used to identify this runtime when compiled natively."]],"mod":[["api",""],["opaque","Opaque types. These are used by the CLI to instantiate machinery that don't need to know the specifics of the runtime. They can then be made to be agnostic over specific formats of data like extrinsics, allowing for them to continue syncing the network through upgrades to even the core datastructures."]],"struct":[["BlockGasLimit",""],["BlockHashCount",""],["BlockLength","We allow for 5 MB blocks."],["BlockWeights","We allow for one half second of compute with a 6 second average block time. These values are dictated by Polkadot for the parachain."],["BondDuration","Reward payments and collator exit requests are delayed by 2 hours (2 * 600 * block_time)"],["CooloffPeriod",""],["CouncilMaxMembers","The maximum number of council members."],["CouncilMaxProposals","The maximum number of Proposlas that can be open in the council at once."],["CouncilMotionDuration","The maximum amount of time (in blocks) for council members to vote on motions. Motions may end in fewer blocks if enough votes are cast to determine the result."],["DefaultBlocksPerRound","Default BlocksPerRound is every hour (600 * 6 second block times)"],["DefaultCollatorCommission","The fixed percent a collator takes off the top of due rewards is 20%"],["EnactmentPeriod",""],["EthereumFindAuthor",""],["ExistentialDeposit",""],["FastTrackVotingPeriod",""],["GenesisConfig",""],["InflationInfo",""],["InstantAllowed",""],["LaunchPeriod",""],["MaxCollatorsPerNominator","Maximum 25 collators per nominator"],["MaxLocks",""],["MaxNominatorsPerCollator","Maximum 10 nominators per collator"],["MaxProposals",""],["MaxVotes",""],["MaximumSchedulerWeight",""],["MinBlocksPerRound","Minimum round length is 2 minutes (20 * 6 second block times)"],["MinCollatorStk","Minimum stake required to be reserved to be a collator is 1_000"],["MinNominatorStk","Minimum stake required to be reserved to be a nominator is 5"],["MinSelectedCandidates","Minimum 8 collators selected per round, default at genesis and minimum forever after"],["MinimumDeposit",""],["MinimumPeriod",""],["MoonbeamGasWeightMapping",""],["Origin",""],["PalletInfo","Provides an implementation of `PalletInfo` to provide information about the pallet setup in the runtime."],["PreimageByteDeposit",""],["Range",""],["Runtime",""],["RuntimeApi",""],["RuntimeApiImpl","Implements all runtime apis for the client side."],["SS58Prefix",""],["TechComitteeMaxMembers","The maximum number of technical committee members."],["TechComitteeMaxProposals","The maximum number of Proposlas that can be open in the technical committee at once."],["TechComitteeMotionDuration","The maximum amount of time (in blocks) for technical committee members to vote on motions. Motions may end in fewer blocks if enough votes are cast to determine the result."],["TransactionByteFee",""],["TransactionConverter",""],["Version",""],["VotingPeriod",""]],"trait":[["BuildStorage","Complex storage builder stuff."]],"type":[["AccountId","Some way of identifying an account on the chain. We intentionally make it equivalent to the public key of our transaction signing scheme."],["AccountIndex","The type for looking up accounts. We don't expect more than 4 billion of them, but you never know..."],["Address","The address format for describing accounts."],["AllModules","All modules included in the runtime as a nested tuple of types. Excludes the System pallet."],["AllModulesWithSystem","All modules included in the runtime as a nested tuple of types."],["AllPallets","All pallets included in the runtime as a nested tuple of types. Excludes the System pallet."],["AllPalletsWithSystem","All pallets included in the runtime as a nested tuple of types."],["AuthorFilter",""],["AuthorInherent",""],["Balance","Balance of an account."],["Balances",""],["BalancesConfig",""],["Block","Block type as expected by this runtime."],["BlockId","BlockId type as expected by this runtime."],["BlockNumber","An index to a block."],["CheckedExtrinsic","Extrinsic type that has already been checked."],["CouncilCollective",""],["CouncilCollectiveConfig",""],["Democracy",""],["DemocracyConfig",""],["DigestItem","Digest item type."],["EVM",""],["EVMConfig",""],["Ethereum",""],["EthereumChainId",""],["EthereumChainIdConfig",""],["EthereumConfig",""],["Executive","Executive: handles dispatch to the various pallets."],["Hash","A hash of some data used by the chain."],["Header","Block header type as expected by this runtime."],["Index","Index of a transaction in the chain."],["ParachainInfo",""],["ParachainInfoConfig",""],["ParachainStaking",""],["ParachainStakingConfig",""],["ParachainSystem",""],["RandomnessCollectiveFlip",""],["Scheduler",""],["SchedulerConfig",""],["Signature","Alias to 512-bit hash when used in the context of a transaction signature on the chain."],["SignedBlock","A Block signed with a Justification"],["SignedExtra","The SignedExtension to the basic transaction logic."],["Sudo",""],["SudoConfig",""],["System",""],["SystemConfig",""],["TechComitteeCollective",""],["TechComitteeCollectiveConfig",""],["Timestamp",""],["TransactionPayment",""],["UncheckedExtrinsic","Unchecked extrinsic type as expected by this runtime."],["Utility",""]]});