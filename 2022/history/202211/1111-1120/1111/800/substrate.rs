//frame_benchingとは、フレームワークのベンチマークを実行するためのモジュールです。
frama_benchmarking::benchmarks! {
    where_clause { where T: Config, }
    _ { }
}
//frame_executiveとは、フレームワークの実行を行うためのモジュールです。
frame_executive
//frame_supportとは、フレームワークのサポートを行うためのモジュールです。
frame_support
//frame_systemとは、フレームワークのシステムを行うためのモジュールです。
frame_system
//frame_support_proceduralとは、フレームワークのサポートを行うためのプロシージャルモジュールです。
frame_support_procedural
//frama_try_runtimeとは、フレームワークの実行を試すためのモジュールです。
frama_try_runtime
//node_executorとは、ノードの実行を行うためのモジュールです。
node_executor
//node_rpcとは、ノードのRPCを行うためのモジュールです。
node_rpc
//pallet_allianceとは、パレットのアライアンスを行うためのモジュールです。
pallet_alliance
//pallet_beefy_mmrとは、パレットのビーフィーMMRを行うためのモジュールです。
pallet_beefy_mmr
//pallet_sessionとは、パレットのセッションを行うためのモジュールです。
pallet_session
//pallet_stakingとは、パレットのステーキングを行うためのモジュールです。
pallet_staking
//pallet_timestampとは、パレットのタイムスタンプを行うためのモジュールです。
pallet_timestamp
//pallet_transaction_paymentとは、パレットのトランザクションペイメントを行うためのモジュールです。
pallet_transaction_payment
//pallet_sudoとは、パレットのスーダーを行うためのモジュールです。
pallet_sudo
//pallet_tipsとは、パレットのチップを行うためのモジュールです。
pallet_tips
//pallet_utilityとは、パレットのユーティリティを行うためのモジュールです。
pallet_utility
//pallet_scored_poolとは、パレットのスコアドプールを行うためのモジュールです。
pallet_scored_pool
//sc_chain_specとは、SCのチェーンスペックを行うためのモジュールです。
sc_chain_spec
//sc_chain_spec_deriveとは、SCのチェーンスペックを派生するためのモジュールです。
sc_chain_spec_derive
//sc_client_dbとは、SCのクライアントDBを行うためのモジュールです。
sc_client_db
//sc_consensusとは、SCのコンセンサスを行うためのモジュールです。
sc_consensus
//sc_executor_wasmiとは、SCの実行者を行うためのモジュールです。
sc_executor_wasmi
//sc_keystoreとは、SCのキーストアを行うためのモジュールです。
sc_keystore
//sc_networkとは、SCのネットワークを行うためのモジュールです。
sc_network
//sc_network_gossipとは、SCのネットワークのゴシップを行うためのモジュールです。
sc_network_gossip
//sc_network_testとは、SCのネットワークのテストを行うためのモジュールです。
sc_network_test
//sc_offchainとは、SCのオフチェーンを行うためのモジュールです。
sc_offchain
//sc_offchain_testingとは、SCのオフチェーンのテストを行うためのモジュールです。
sc_offchain_testing
//sc_serviceとは、SCのサービスを行うためのモジュールです。  これは、パレットの実行を行うためのモジュールです。
sc_service
//sc_peersetとは、SCのピアセットを行うためのモジュールです。
sc_peerset
//sc_proposer_metricsとは、SCのプロポーザーのメトリックスを行うためのモジュールです。
sc_proposer_metrics
//sc_rpc_apiとは、SCのRPC APIを行うためのモジュールです。
sc_rpc_api
//sc_rpc_serverとは、SCのRPCサーバーを行うためのモジュールです。
sc_rpc_server
//sc_serviceとは、SCのサービスを行うためのモジュールです。
sc_service
//sc_state_dbとは、SCのステートDBを行うためのモジュールです。
sc_state_db
//sc_telemetryとは、SCのテレメトリーを行うためのモジュールです。
sc_telemetry
//sc_transaction_poolとは、SCのトランザクションプールを行うためのモジュールです。
sc_transaction_pool
//sc_sync_state_rpcとは、SCの同期ステートRPCを行うためのモジュールです。
sc_sync_state_rpc
//sc_tracingとは、SCのトレーシングを行うためのモジュールです。
sc_tracing
//sc_transaction_pool_apiとは、SCのトランザクションプールAPIを行うためのモジュールです。
sc_transaction_pool_api
//sp_api_proc_macroとは、SPのAPIのプロシージャマクロを行うためのモジュールです。
sp_api_proc_macro
//sp_application_cryptoとは、SPのアプリケーション暗号を行うためのモジュールです。
sp_application_crypto
//sp_arithmeticとは、SPの算術を行うためのモジュールです。
sp_arithmetic
//sp_authority_discoveryとは、SPの権限発見を行うためのモジュールです。
sp_authority_discovery
//sp_block_builderとは、SPのブロックビルダーを行うためのモジュールです。
sp_block_builder
//sp_blockchainとは、SPのブロックチェーンを行うためのモジュールです。
sp_blockchain
//sp_coreとは、SPのコアを行うためのモジュールです。
sp_core
//sp_consensusとは、SPのコンセンサスを行うためのモジュールです。
sp_consensus
//sp_consensus_auraとは、SPのコンセンサスのオーラを行うためのモジュールです。
sp_consensus_aura
//sp_consensus_babeとは、SPのコンセンサスのベイベを行うためのモジュールです。
sp_consensus_babe
//sp_consensus_vrfとは、SPのコンセンサスのVRFを行うためのモジュールです。
sp_consensus_vrf
//sp_ioとは、SPのIOを行うためのモジュールです。
sp_io
//sp_runtimeとは、SPのランタイムを行うためのモジュールです。
sp_runtime
//sp_serializerとは、SPのシリアライザーを行うためのモジュールです。
sp_serializer
//sp_versionとは、SPのバージョンを行うためのモジュールです。
sp_version
//sp_wasm-interfaceとは、SPのWASMインターフェースを行うためのモジュールです。
sp_wasm_interface
//sp_storageとは、SPのストレージを行うためのモジュールです。
sp_storage
//substrate_frame_cliとは、SubstrateのフレームのCLIを行うためのモジュールです。
substrate_frame_cli
//substrate_frame_rpc_systemとは、SubstrateのフレームのRPCシステムを行うためのモジュールです。
substrate_frame_rpc_system
//substrate_frame_rpc_supportとは、SubstrateのフレームのRPCサポートを行うためのモジュールです。
substrate_frame_rpc_support
//substrate_rpc_clientとは、SubstrateのRPCクライアントを行うためのモジュールです。
substrate_rpc_client
//chain_specとは、チェーンの仕様を行うためのモジュールです。
chain_spec
//

