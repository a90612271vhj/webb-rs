#[allow(dead_code, unused_imports, non_camel_case_types)]
pub mod api {
    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
    pub enum Event {
        #[codec(index = 0)]
        System(system::Event),
        #[codec(index = 5)]
        Indices(indices::Event),
        #[codec(index = 6)]
        Balances(balances::Event),
        #[codec(index = 8)]
        ElectionProviderMultiPhase(election_provider_multi_phase::Event),
        #[codec(index = 9)]
        Staking(staking::Event),
        #[codec(index = 10)]
        Session(session::Event),
        #[codec(index = 11)]
        Democracy(democracy::Event),
        #[codec(index = 12)]
        Council(council::Event),
        #[codec(index = 13)]
        Elections(elections::Event),
        #[codec(index = 14)]
        Grandpa(grandpa::Event),
        #[codec(index = 15)]
        Treasury(treasury::Event),
        #[codec(index = 16)]
        Utility(utility::Event),
        #[codec(index = 17)]
        Multisig(multisig::Event),
        #[codec(index = 18)]
        Scheduler(scheduler::Event),
        #[codec(index = 19)]
        Proxy(proxy::Event),
        #[codec(index = 20)]
        Assets(assets::Event),
        #[codec(index = 21)]
        Sudo(sudo::Event),
        #[codec(index = 22)]
        ImOnline(im_online::Event),
        #[codec(index = 24)]
        Offences(offences::Event),
        #[codec(index = 26)]
        Bounties(bounties::Event),
        #[codec(index = 27)]
        BagsList(bags_list::Event),
        #[codec(index = 28)]
        HasherBn254(hasher_bn254::Event),
        #[codec(index = 29)]
        HasherBls381(hasher_bls381::Event),
        #[codec(index = 30)]
        AssetRegistry(asset_registry::Event),
        #[codec(index = 31)]
        Currencies(currencies::Event),
        #[codec(index = 32)]
        Tokens(tokens::Event),
        #[codec(index = 33)]
        TokenWrapper(token_wrapper::Event),
        #[codec(index = 34)]
        VerifierBn254(verifier_bn254::Event),
        #[codec(index = 35)]
        VerifierBls381(verifier_bls381::Event),
        #[codec(index = 36)]
        MerkleTreeBn254(merkle_tree_bn254::Event),
        #[codec(index = 37)]
        MerkleTreeBls381(merkle_tree_bls381::Event),
        #[codec(index = 38)]
        LinkableTreeBn254(linkable_tree_bn254::Event),
        #[codec(index = 39)]
        LinkableTreeBls381(linkable_tree_bls381::Event),
        #[codec(index = 40)]
        MixerBn254(mixer_bn254::Event),
        #[codec(index = 41)]
        MixerBls381(mixer_bls381::Event),
        #[codec(index = 42)]
        AnchorBn254(anchor_bn254::Event),
        #[codec(index = 43)]
        AnchorBls381(anchor_bls381::Event),
        #[codec(index = 44)]
        AnchorHandlerBn254(anchor_handler_bn254::Event),
        #[codec(index = 45)]
        AnchorHandlerBls381(anchor_handler_bls381::Event),
        #[codec(index = 46)]
        Bridge(bridge::Event),
    }
    pub mod system {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct FillBlock {
                pub ratio: runtime_types::sp_arithmetic::per_things::Perbill,
            }
            impl ::subxt::Call for FillBlock {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "fill_block";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Remark {
                pub remark: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for Remark {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "remark";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct SetHeapPages {
                pub pages: ::core::primitive::u64,
            }
            impl ::subxt::Call for SetHeapPages {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "set_heap_pages";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct SetCode {
                pub code: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for SetCode {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "set_code";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct SetCodeWithoutChecks {
                pub code: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for SetCodeWithoutChecks {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "set_code_without_checks";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct SetChangesTrieConfig { pub changes_trie_config : :: core :: option :: Option < runtime_types :: sp_core :: changes_trie :: ChangesTrieConfiguration > }
            impl ::subxt::Call for SetChangesTrieConfig {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "set_changes_trie_config";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct SetStorage {
                pub items: ::std::vec::Vec<(
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::std::vec::Vec<::core::primitive::u8>,
                )>,
            }
            impl ::subxt::Call for SetStorage {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "set_storage";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct KillStorage {
                pub keys:
                    ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
            }
            impl ::subxt::Call for KillStorage {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "kill_storage";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct KillPrefix {
                pub prefix: ::std::vec::Vec<::core::primitive::u8>,
                pub subkeys: ::core::primitive::u32,
            }
            impl ::subxt::Call for KillPrefix {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "kill_prefix";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct RemarkWithEvent {
                pub remark: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for RemarkWithEvent {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "remark_with_event";
            }
            pub struct TransactionApi<
                'a,
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            > {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn fill_block(
                    &self,
                    ratio: runtime_types::sp_arithmetic::per_things::Perbill,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, FillBlock>
                {
                    let call = FillBlock { ratio };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn remark(
                    &self,
                    remark: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, Remark>
                {
                    let call = Remark { remark };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_heap_pages(
                    &self,
                    pages: ::core::primitive::u64,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, SetHeapPages>
                {
                    let call = SetHeapPages { pages };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_code(
                    &self,
                    code: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, SetCode>
                {
                    let call = SetCode { code };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_code_without_checks(
                    &self,
                    code: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, SetCodeWithoutChecks>
                {
                    let call = SetCodeWithoutChecks { code };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_changes_trie_config(
                    &self,
                    changes_trie_config : :: core :: option :: Option < runtime_types :: sp_core :: changes_trie :: ChangesTrieConfiguration >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, SetChangesTrieConfig>
                {
                    let call = SetChangesTrieConfig {
                        changes_trie_config,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_storage(
                    &self,
                    items: ::std::vec::Vec<(
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::std::vec::Vec<::core::primitive::u8>,
                    )>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, SetStorage>
                {
                    let call = SetStorage { items };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn kill_storage(
                    &self,
                    keys: ::std::vec::Vec<
                        ::std::vec::Vec<::core::primitive::u8>,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, KillStorage>
                {
                    let call = KillStorage { keys };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn kill_prefix(
                    &self,
                    prefix: ::std::vec::Vec<::core::primitive::u8>,
                    subkeys: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, KillPrefix>
                {
                    let call = KillPrefix { prefix, subkeys };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn remark_with_event(
                    &self,
                    remark: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, RemarkWithEvent>
                {
                    let call = RemarkWithEvent { remark };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::frame_system::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ExtrinsicSuccess(
                pub runtime_types::frame_support::weights::DispatchInfo,
            );
            impl ::subxt::Event for ExtrinsicSuccess {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "ExtrinsicSuccess";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ExtrinsicFailed(
                pub runtime_types::sp_runtime::DispatchError,
                pub runtime_types::frame_support::weights::DispatchInfo,
            );
            impl ::subxt::Event for ExtrinsicFailed {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "ExtrinsicFailed";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct CodeUpdated {}
            impl ::subxt::Event for CodeUpdated {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "CodeUpdated";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct NewAccount(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::Event for NewAccount {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "NewAccount";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct KilledAccount(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::Event for KilledAccount {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "KilledAccount";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Remarked(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::H256,
            );
            impl ::subxt::Event for Remarked {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "Remarked";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Account(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Account {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "Account";
                type Value = runtime_types::frame_system::AccountInfo<
                    ::core::primitive::u32,
                    runtime_types::pallet_balances::AccountData<
                        ::core::primitive::u128,
                    >,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct ExtrinsicCount;
            impl ::subxt::StorageEntry for ExtrinsicCount {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "ExtrinsicCount";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct BlockWeight;
            impl ::subxt::StorageEntry for BlockWeight {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "BlockWeight";
                type Value =
                    runtime_types::frame_support::weights::PerDispatchClass<
                        ::core::primitive::u64,
                    >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct AllExtrinsicsLen;
            impl ::subxt::StorageEntry for AllExtrinsicsLen {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "AllExtrinsicsLen";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct BlockHash(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for BlockHash {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "BlockHash";
                type Value = ::subxt::sp_core::H256;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                    ])
                }
            }
            pub struct ExtrinsicData(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for ExtrinsicData {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "ExtrinsicData";
                type Value = ::std::vec::Vec<::core::primitive::u8>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                    ])
                }
            }
            pub struct Number;
            impl ::subxt::StorageEntry for Number {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "Number";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ParentHash;
            impl ::subxt::StorageEntry for ParentHash {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "ParentHash";
                type Value = ::subxt::sp_core::H256;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Digest;
            impl ::subxt::StorageEntry for Digest {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "Digest";
                type Value = runtime_types::sp_runtime::generic::digest::Digest<
                    ::subxt::sp_core::H256,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Events;
            impl ::subxt::StorageEntry for Events {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "Events";
                type Value = ::std::vec::Vec<
                    runtime_types::frame_system::EventRecord<
                        runtime_types::darkwebb_standalone_runtime::Event,
                        ::subxt::sp_core::H256,
                    >,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct EventCount;
            impl ::subxt::StorageEntry for EventCount {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "EventCount";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct EventTopics(pub ::subxt::sp_core::H256);
            impl ::subxt::StorageEntry for EventTopics {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "EventTopics";
                type Value = ::std::vec::Vec<(
                    ::core::primitive::u32,
                    ::core::primitive::u32,
                )>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct LastRuntimeUpgrade;
            impl ::subxt::StorageEntry for LastRuntimeUpgrade {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "LastRuntimeUpgrade";
                type Value =
                    runtime_types::frame_system::LastRuntimeUpgradeInfo;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct UpgradedToU32RefCount;
            impl ::subxt::StorageEntry for UpgradedToU32RefCount {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "UpgradedToU32RefCount";
                type Value = ::core::primitive::bool;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct UpgradedToTripleRefCount;
            impl ::subxt::StorageEntry for UpgradedToTripleRefCount {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "UpgradedToTripleRefCount";
                type Value = ::core::primitive::bool;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ExecutionPhase;
            impl ::subxt::StorageEntry for ExecutionPhase {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "ExecutionPhase";
                type Value = runtime_types::frame_system::Phase;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn account(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_system::AccountInfo<
                        ::core::primitive::u32,
                        runtime_types::pallet_balances::AccountData<
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Account(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn account_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Account>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn extrinsic_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::Error,
                > {
                    let entry = ExtrinsicCount;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn block_weight(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::weights::PerDispatchClass<
                        ::core::primitive::u64,
                    >,
                    ::subxt::Error,
                > {
                    let entry = BlockWeight;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn all_extrinsics_len(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::Error,
                > {
                    let entry = AllExtrinsicsLen;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn block_hash(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::sp_core::H256,
                    ::subxt::Error,
                > {
                    let entry = BlockHash(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn block_hash_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, BlockHash>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn extrinsic_data(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::subxt::Error,
                > {
                    let entry = ExtrinsicData(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn extrinsic_data_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ExtrinsicData>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn number(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::u32,
                    ::subxt::Error,
                > {
                    let entry = Number;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn parent_hash(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::sp_core::H256,
                    ::subxt::Error,
                > {
                    let entry = ParentHash;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn digest(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::sp_runtime::generic::digest::Digest<
                        ::subxt::sp_core::H256,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Digest;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn events(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<
                        runtime_types::frame_system::EventRecord<
                            runtime_types::darkwebb_standalone_runtime::Event,
                            ::subxt::sp_core::H256,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Events;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn event_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::u32,
                    ::subxt::Error,
                > {
                    let entry = EventCount;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn event_topics(
                    &self,
                    _0: ::subxt::sp_core::H256,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<(
                        ::core::primitive::u32,
                        ::core::primitive::u32,
                    )>,
                    ::subxt::Error,
                > {
                    let entry = EventTopics(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn event_topics_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, EventTopics>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn last_runtime_upgrade(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::frame_system::LastRuntimeUpgradeInfo,
                    >,
                    ::subxt::Error,
                > {
                    let entry = LastRuntimeUpgrade;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn upgraded_to_u32_ref_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::bool,
                    ::subxt::Error,
                > {
                    let entry = UpgradedToU32RefCount;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn upgraded_to_triple_ref_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::bool,
                    ::subxt::Error,
                > {
                    let entry = UpgradedToTripleRefCount;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn execution_phase(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<runtime_types::frame_system::Phase>,
                    ::subxt::Error,
                > {
                    let entry = ExecutionPhase;
                    self.client.storage().fetch(&entry, hash).await
                }
            }
        }
    }
    pub mod randomness_collective_flip {
        use super::runtime_types;
        pub mod storage {
            use super::runtime_types;
            pub struct RandomMaterial;
            impl ::subxt::StorageEntry for RandomMaterial {
                const PALLET: &'static str = "RandomnessCollectiveFlip";
                const STORAGE: &'static str = "RandomMaterial";
                type Value = ::std::vec::Vec<::subxt::sp_core::H256>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn random_material(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::subxt::sp_core::H256>,
                    ::subxt::Error,
                > {
                    let entry = RandomMaterial;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod timestamp {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Set {
                #[codec(compact)]
                pub now: ::core::primitive::u64,
            }
            impl ::subxt::Call for Set {
                const PALLET: &'static str = "Timestamp";
                const FUNCTION: &'static str = "set";
            }
            pub struct TransactionApi<
                'a,
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            > {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn set(
                    &self,
                    now: ::core::primitive::u64,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, Set> {
                    let call = Set { now };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Now;
            impl ::subxt::StorageEntry for Now {
                const PALLET: &'static str = "Timestamp";
                const STORAGE: &'static str = "Now";
                type Value = ::core::primitive::u64;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct DidUpdate;
            impl ::subxt::StorageEntry for DidUpdate {
                const PALLET: &'static str = "Timestamp";
                const STORAGE: &'static str = "DidUpdate";
                type Value = ::core::primitive::bool;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn now(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::u64,
                    ::subxt::Error,
                > {
                    let entry = Now;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn did_update(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::bool,
                    ::subxt::Error,
                > {
                    let entry = DidUpdate;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod babe {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ReportEquivocation {
                pub equivocation_proof:
                    runtime_types::sp_consensus_slots::EquivocationProof<
                        runtime_types::sp_runtime::generic::header::Header<
                            ::core::primitive::u32,
                            runtime_types::sp_runtime::traits::BlakeTwo256,
                        >,
                        runtime_types::sp_consensus_babe::app::Public,
                    >,
                pub key_owner_proof: runtime_types::sp_session::MembershipProof,
            }
            impl ::subxt::Call for ReportEquivocation {
                const PALLET: &'static str = "Babe";
                const FUNCTION: &'static str = "report_equivocation";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ReportEquivocationUnsigned {
                pub equivocation_proof:
                    runtime_types::sp_consensus_slots::EquivocationProof<
                        runtime_types::sp_runtime::generic::header::Header<
                            ::core::primitive::u32,
                            runtime_types::sp_runtime::traits::BlakeTwo256,
                        >,
                        runtime_types::sp_consensus_babe::app::Public,
                    >,
                pub key_owner_proof: runtime_types::sp_session::MembershipProof,
            }
            impl ::subxt::Call for ReportEquivocationUnsigned {
                const PALLET: &'static str = "Babe";
                const FUNCTION: &'static str = "report_equivocation_unsigned";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct PlanConfigChange { pub config : runtime_types :: sp_consensus_babe :: digests :: NextConfigDescriptor }
            impl ::subxt::Call for PlanConfigChange {
                const PALLET: &'static str = "Babe";
                const FUNCTION: &'static str = "plan_config_change";
            }
            pub struct TransactionApi<
                'a,
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            > {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn report_equivocation(
                    &self,
                    equivocation_proof : runtime_types :: sp_consensus_slots :: EquivocationProof < runtime_types :: sp_runtime :: generic :: header :: Header < :: core :: primitive :: u32 , runtime_types :: sp_runtime :: traits :: BlakeTwo256 > , runtime_types :: sp_consensus_babe :: app :: Public >,
                    key_owner_proof: runtime_types::sp_session::MembershipProof,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, ReportEquivocation>
                {
                    let call = ReportEquivocation {
                        equivocation_proof,
                        key_owner_proof,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn report_equivocation_unsigned(
                    &self,
                    equivocation_proof : runtime_types :: sp_consensus_slots :: EquivocationProof < runtime_types :: sp_runtime :: generic :: header :: Header < :: core :: primitive :: u32 , runtime_types :: sp_runtime :: traits :: BlakeTwo256 > , runtime_types :: sp_consensus_babe :: app :: Public >,
                    key_owner_proof: runtime_types::sp_session::MembershipProof,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    ReportEquivocationUnsigned,
                > {
                    let call = ReportEquivocationUnsigned {
                        equivocation_proof,
                        key_owner_proof,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn plan_config_change(
                    &self,
                    config : runtime_types :: sp_consensus_babe :: digests :: NextConfigDescriptor,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, PlanConfigChange>
                {
                    let call = PlanConfigChange { config };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct EpochIndex;
            impl ::subxt::StorageEntry for EpochIndex {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "EpochIndex";
                type Value = ::core::primitive::u64;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Authorities;
            impl ::subxt::StorageEntry for Authorities {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "Authorities";
                type Value = runtime_types :: frame_support :: storage :: weak_bounded_vec :: WeakBoundedVec < (runtime_types :: sp_consensus_babe :: app :: Public , :: core :: primitive :: u64 ,) > ;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct GenesisSlot;
            impl ::subxt::StorageEntry for GenesisSlot {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "GenesisSlot";
                type Value = runtime_types::sp_consensus_slots::Slot;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct CurrentSlot;
            impl ::subxt::StorageEntry for CurrentSlot {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "CurrentSlot";
                type Value = runtime_types::sp_consensus_slots::Slot;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Randomness;
            impl ::subxt::StorageEntry for Randomness {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "Randomness";
                type Value = [::core::primitive::u8; 32usize];
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct PendingEpochConfigChange;
            impl ::subxt::StorageEntry for PendingEpochConfigChange {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "PendingEpochConfigChange";
                type Value = runtime_types :: sp_consensus_babe :: digests :: NextConfigDescriptor ;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NextRandomness;
            impl ::subxt::StorageEntry for NextRandomness {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "NextRandomness";
                type Value = [::core::primitive::u8; 32usize];
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NextAuthorities;
            impl ::subxt::StorageEntry for NextAuthorities {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "NextAuthorities";
                type Value = runtime_types :: frame_support :: storage :: weak_bounded_vec :: WeakBoundedVec < (runtime_types :: sp_consensus_babe :: app :: Public , :: core :: primitive :: u64 ,) > ;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct SegmentIndex;
            impl ::subxt::StorageEntry for SegmentIndex {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "SegmentIndex";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct UnderConstruction(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for UnderConstruction {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "UnderConstruction";
                type Value = runtime_types :: frame_support :: storage :: bounded_vec :: BoundedVec < [:: core :: primitive :: u8 ; 32usize] > ;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                    ])
                }
            }
            pub struct Initialized;
            impl ::subxt::StorageEntry for Initialized {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "Initialized";
                type Value =
                    ::core::option::Option<[::core::primitive::u8; 32usize]>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct AuthorVrfRandomness;
            impl ::subxt::StorageEntry for AuthorVrfRandomness {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "AuthorVrfRandomness";
                type Value =
                    ::core::option::Option<[::core::primitive::u8; 32usize]>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct EpochStart;
            impl ::subxt::StorageEntry for EpochStart {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "EpochStart";
                type Value = (::core::primitive::u32, ::core::primitive::u32);
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Lateness;
            impl ::subxt::StorageEntry for Lateness {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "Lateness";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct EpochConfig;
            impl ::subxt::StorageEntry for EpochConfig {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "EpochConfig";
                type Value =
                    runtime_types::sp_consensus_babe::BabeEpochConfiguration;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NextEpochConfig;
            impl ::subxt::StorageEntry for NextEpochConfig {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "NextEpochConfig";
                type Value =
                    runtime_types::sp_consensus_babe::BabeEpochConfiguration;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn epoch_index(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::u64,
                    ::subxt::Error,
                > {
                    let entry = EpochIndex;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }                pub async fn authorities (& self , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < runtime_types :: frame_support :: storage :: weak_bounded_vec :: WeakBoundedVec < (runtime_types :: sp_consensus_babe :: app :: Public , :: core :: primitive :: u64 ,) > , :: subxt :: Error >{
                    let entry = Authorities;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn genesis_slot(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::sp_consensus_slots::Slot,
                    ::subxt::Error,
                > {
                    let entry = GenesisSlot;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn current_slot(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::sp_consensus_slots::Slot,
                    ::subxt::Error,
                > {
                    let entry = CurrentSlot;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn randomness(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    [::core::primitive::u8; 32usize],
                    ::subxt::Error,
                > {
                    let entry = Randomness;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }                pub async fn pending_epoch_config_change (& self , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < :: core :: option :: Option < runtime_types :: sp_consensus_babe :: digests :: NextConfigDescriptor > , :: subxt :: Error >{
                    let entry = PendingEpochConfigChange;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn next_randomness(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    [::core::primitive::u8; 32usize],
                    ::subxt::Error,
                > {
                    let entry = NextRandomness;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }                pub async fn next_authorities (& self , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < runtime_types :: frame_support :: storage :: weak_bounded_vec :: WeakBoundedVec < (runtime_types :: sp_consensus_babe :: app :: Public , :: core :: primitive :: u64 ,) > , :: subxt :: Error >{
                    let entry = NextAuthorities;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn segment_index(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::u32,
                    ::subxt::Error,
                > {
                    let entry = SegmentIndex;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }                pub async fn under_construction (& self , _0 : :: core :: primitive :: u32 , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < runtime_types :: frame_support :: storage :: bounded_vec :: BoundedVec < [:: core :: primitive :: u8 ; 32usize] > , :: subxt :: Error >{
                    let entry = UnderConstruction(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn under_construction_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, UnderConstruction>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn initialized(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        ::core::option::Option<
                            [::core::primitive::u8; 32usize],
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Initialized;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn author_vrf_randomness(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<[::core::primitive::u8; 32usize]>,
                    ::subxt::Error,
                > {
                    let entry = AuthorVrfRandomness;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn epoch_start(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    (::core::primitive::u32, ::core::primitive::u32),
                    ::subxt::Error,
                > {
                    let entry = EpochStart;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn lateness(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::u32,
                    ::subxt::Error,
                > {
                    let entry = Lateness;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }                pub async fn epoch_config (& self , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < :: core :: option :: Option < runtime_types :: sp_consensus_babe :: BabeEpochConfiguration > , :: subxt :: Error >{
                    let entry = EpochConfig;
                    self.client.storage().fetch(&entry, hash).await
                }                pub async fn next_epoch_config (& self , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < :: core :: option :: Option < runtime_types :: sp_consensus_babe :: BabeEpochConfiguration > , :: subxt :: Error >{
                    let entry = NextEpochConfig;
                    self.client.storage().fetch(&entry, hash).await
                }
            }
        }
    }
    pub mod authorship {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct SetUncles {
                pub new_uncles: ::std::vec::Vec<
                    runtime_types::sp_runtime::generic::header::Header<
                        ::core::primitive::u32,
                        runtime_types::sp_runtime::traits::BlakeTwo256,
                    >,
                >,
            }
            impl ::subxt::Call for SetUncles {
                const PALLET: &'static str = "Authorship";
                const FUNCTION: &'static str = "set_uncles";
            }
            pub struct TransactionApi<
                'a,
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            > {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn set_uncles(
                    &self,
                    new_uncles: ::std::vec::Vec<
                        runtime_types::sp_runtime::generic::header::Header<
                            ::core::primitive::u32,
                            runtime_types::sp_runtime::traits::BlakeTwo256,
                        >,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, SetUncles>
                {
                    let call = SetUncles { new_uncles };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Uncles;
            impl ::subxt::StorageEntry for Uncles {
                const PALLET: &'static str = "Authorship";
                const STORAGE: &'static str = "Uncles";
                type Value = ::std::vec::Vec<
                    runtime_types::pallet_authorship::UncleEntryItem<
                        ::core::primitive::u32,
                        ::subxt::sp_core::H256,
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Author;
            impl ::subxt::StorageEntry for Author {
                const PALLET: &'static str = "Authorship";
                const STORAGE: &'static str = "Author";
                type Value = ::subxt::sp_core::crypto::AccountId32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct DidSetUncles;
            impl ::subxt::StorageEntry for DidSetUncles {
                const PALLET: &'static str = "Authorship";
                const STORAGE: &'static str = "DidSetUncles";
                type Value = ::core::primitive::bool;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn uncles(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<
                        runtime_types::pallet_authorship::UncleEntryItem<
                            ::core::primitive::u32,
                            ::subxt::sp_core::H256,
                            ::subxt::sp_core::crypto::AccountId32,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Uncles;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn author(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Author;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn did_set_uncles(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::bool,
                    ::subxt::Error,
                > {
                    let entry = DidSetUncles;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod indices {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Claim {
                pub index: ::core::primitive::u32,
            }
            impl ::subxt::Call for Claim {
                const PALLET: &'static str = "Indices";
                const FUNCTION: &'static str = "claim";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Transfer {
                pub new: ::subxt::sp_core::crypto::AccountId32,
                pub index: ::core::primitive::u32,
            }
            impl ::subxt::Call for Transfer {
                const PALLET: &'static str = "Indices";
                const FUNCTION: &'static str = "transfer";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Free {
                pub index: ::core::primitive::u32,
            }
            impl ::subxt::Call for Free {
                const PALLET: &'static str = "Indices";
                const FUNCTION: &'static str = "free";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ForceTransfer {
                pub new: ::subxt::sp_core::crypto::AccountId32,
                pub index: ::core::primitive::u32,
                pub freeze: ::core::primitive::bool,
            }
            impl ::subxt::Call for ForceTransfer {
                const PALLET: &'static str = "Indices";
                const FUNCTION: &'static str = "force_transfer";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Freeze {
                pub index: ::core::primitive::u32,
            }
            impl ::subxt::Call for Freeze {
                const PALLET: &'static str = "Indices";
                const FUNCTION: &'static str = "freeze";
            }
            pub struct TransactionApi<
                'a,
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            > {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn claim(
                    &self,
                    index: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, Claim>
                {
                    let call = Claim { index };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn transfer(
                    &self,
                    new: ::subxt::sp_core::crypto::AccountId32,
                    index: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, Transfer>
                {
                    let call = Transfer { new, index };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn free(
                    &self,
                    index: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, Free>
                {
                    let call = Free { index };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_transfer(
                    &self,
                    new: ::subxt::sp_core::crypto::AccountId32,
                    index: ::core::primitive::u32,
                    freeze: ::core::primitive::bool,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, ForceTransfer>
                {
                    let call = ForceTransfer { new, index, freeze };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn freeze(
                    &self,
                    index: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, Freeze>
                {
                    let call = Freeze { index };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_indices::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct IndexAssigned(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u32,
            );
            impl ::subxt::Event for IndexAssigned {
                const PALLET: &'static str = "Indices";
                const EVENT: &'static str = "IndexAssigned";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct IndexFreed(pub ::core::primitive::u32);
            impl ::subxt::Event for IndexFreed {
                const PALLET: &'static str = "Indices";
                const EVENT: &'static str = "IndexFreed";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct IndexFrozen(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for IndexFrozen {
                const PALLET: &'static str = "Indices";
                const EVENT: &'static str = "IndexFrozen";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Accounts(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for Accounts {
                const PALLET: &'static str = "Indices";
                const STORAGE: &'static str = "Accounts";
                type Value = (
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u128,
                    ::core::primitive::bool,
                );
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn accounts(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                        ::core::primitive::bool,
                    )>,
                    ::subxt::Error,
                > {
                    let entry = Accounts(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn accounts_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Accounts>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod balances {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Transfer {
                pub dest: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                #[codec(compact)]
                pub value: ::core::primitive::u128,
            }
            impl ::subxt::Call for Transfer {
                const PALLET: &'static str = "Balances";
                const FUNCTION: &'static str = "transfer";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct SetBalance {
                pub who: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                #[codec(compact)]
                pub new_free: ::core::primitive::u128,
                #[codec(compact)]
                pub new_reserved: ::core::primitive::u128,
            }
            impl ::subxt::Call for SetBalance {
                const PALLET: &'static str = "Balances";
                const FUNCTION: &'static str = "set_balance";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ForceTransfer {
                pub source: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                pub dest: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                #[codec(compact)]
                pub value: ::core::primitive::u128,
            }
            impl ::subxt::Call for ForceTransfer {
                const PALLET: &'static str = "Balances";
                const FUNCTION: &'static str = "force_transfer";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct TransferKeepAlive {
                pub dest: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                #[codec(compact)]
                pub value: ::core::primitive::u128,
            }
            impl ::subxt::Call for TransferKeepAlive {
                const PALLET: &'static str = "Balances";
                const FUNCTION: &'static str = "transfer_keep_alive";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct TransferAll {
                pub dest: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                pub keep_alive: ::core::primitive::bool,
            }
            impl ::subxt::Call for TransferAll {
                const PALLET: &'static str = "Balances";
                const FUNCTION: &'static str = "transfer_all";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ForceUnreserve {
                pub who: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Call for ForceUnreserve {
                const PALLET: &'static str = "Balances";
                const FUNCTION: &'static str = "force_unreserve";
            }
            pub struct TransactionApi<
                'a,
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            > {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn transfer(
                    &self,
                    dest: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    value: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, Transfer>
                {
                    let call = Transfer { dest, value };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_balance(
                    &self,
                    who: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    new_free: ::core::primitive::u128,
                    new_reserved: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, SetBalance>
                {
                    let call = SetBalance {
                        who,
                        new_free,
                        new_reserved,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_transfer(
                    &self,
                    source: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    dest: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    value: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, ForceTransfer>
                {
                    let call = ForceTransfer {
                        source,
                        dest,
                        value,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn transfer_keep_alive(
                    &self,
                    dest: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    value: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, TransferKeepAlive>
                {
                    let call = TransferKeepAlive { dest, value };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn transfer_all(
                    &self,
                    dest: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    keep_alive: ::core::primitive::bool,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, TransferAll>
                {
                    let call = TransferAll { dest, keep_alive };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_unreserve(
                    &self,
                    who: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    amount: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, ForceUnreserve>
                {
                    let call = ForceUnreserve { who, amount };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_balances::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Endowed(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Endowed {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Endowed";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct DustLost(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for DustLost {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "DustLost";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Transfer(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Transfer {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Transfer";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct BalanceSet(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for BalanceSet {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "BalanceSet";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Reserved(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Reserved {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Reserved";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Unreserved(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Unreserved {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Unreserved";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ReserveRepatriated (pub :: subxt :: sp_core :: crypto :: AccountId32 , pub :: subxt :: sp_core :: crypto :: AccountId32 , pub :: core :: primitive :: u128 , pub runtime_types :: frame_support :: traits :: tokens :: misc :: BalanceStatus) ;
            impl ::subxt::Event for ReserveRepatriated {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "ReserveRepatriated";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Deposit(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Deposit {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Deposit";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Withdraw(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Withdraw {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Withdraw";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Slashed(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Slashed {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Slashed";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct TotalIssuance;
            impl ::subxt::StorageEntry for TotalIssuance {
                const PALLET: &'static str = "Balances";
                const STORAGE: &'static str = "TotalIssuance";
                type Value = ::core::primitive::u128;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Account(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Account {
                const PALLET: &'static str = "Balances";
                const STORAGE: &'static str = "Account";
                type Value = runtime_types::pallet_balances::AccountData<
                    ::core::primitive::u128,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct Locks(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Locks {
                const PALLET: &'static str = "Balances";
                const STORAGE: &'static str = "Locks";
                type Value = runtime_types :: frame_support :: storage :: weak_bounded_vec :: WeakBoundedVec < runtime_types :: pallet_balances :: BalanceLock < :: core :: primitive :: u128 > > ;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct Reserves(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Reserves {
                const PALLET: &'static str = "Balances";
                const STORAGE: &'static str = "Reserves";
                type Value = runtime_types :: frame_support :: storage :: bounded_vec :: BoundedVec < runtime_types :: pallet_balances :: ReserveData < [:: core :: primitive :: u8 ; 8usize] , :: core :: primitive :: u128 > > ;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct StorageVersion;
            impl ::subxt::StorageEntry for StorageVersion {
                const PALLET: &'static str = "Balances";
                const STORAGE: &'static str = "StorageVersion";
                type Value = runtime_types::pallet_balances::Releases;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn total_issuance(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::u128,
                    ::subxt::Error,
                > {
                    let entry = TotalIssuance;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn account(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_balances::AccountData<
                        ::core::primitive::u128,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Account(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn account_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Account>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }                pub async fn locks (& self , _0 : :: subxt :: sp_core :: crypto :: AccountId32 , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < runtime_types :: frame_support :: storage :: weak_bounded_vec :: WeakBoundedVec < runtime_types :: pallet_balances :: BalanceLock < :: core :: primitive :: u128 > > , :: subxt :: Error >{
                    let entry = Locks(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn locks_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Locks>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }                pub async fn reserves (& self , _0 : :: subxt :: sp_core :: crypto :: AccountId32 , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < runtime_types :: frame_support :: storage :: bounded_vec :: BoundedVec < runtime_types :: pallet_balances :: ReserveData < [:: core :: primitive :: u8 ; 8usize] , :: core :: primitive :: u128 > > , :: subxt :: Error >{
                    let entry = Reserves(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn reserves_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Reserves>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn storage_version(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_balances::Releases,
                    ::subxt::Error,
                > {
                    let entry = StorageVersion;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod transaction_payment {
        use super::runtime_types;
        pub mod storage {
            use super::runtime_types;
            pub struct NextFeeMultiplier;
            impl ::subxt::StorageEntry for NextFeeMultiplier {
                const PALLET: &'static str = "TransactionPayment";
                const STORAGE: &'static str = "NextFeeMultiplier";
                type Value =
                    runtime_types::sp_arithmetic::fixed_point::FixedU128;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageVersion;
            impl ::subxt::StorageEntry for StorageVersion {
                const PALLET: &'static str = "TransactionPayment";
                const STORAGE: &'static str = "StorageVersion";
                type Value =
                    runtime_types::pallet_transaction_payment::Releases;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn next_fee_multiplier(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::sp_arithmetic::fixed_point::FixedU128,
                    ::subxt::Error,
                > {
                    let entry = NextFeeMultiplier;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn storage_version(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_transaction_payment::Releases,
                    ::subxt::Error,
                > {
                    let entry = StorageVersion;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod election_provider_multi_phase {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct SubmitUnsigned { pub raw_solution : runtime_types :: pallet_election_provider_multi_phase :: RawSolution < runtime_types :: darkwebb_standalone_runtime :: NposSolution16 > , pub witness : runtime_types :: pallet_election_provider_multi_phase :: SolutionOrSnapshotSize }
            impl ::subxt::Call for SubmitUnsigned {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const FUNCTION: &'static str = "submit_unsigned";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct SetMinimumUntrustedScore {
                pub maybe_next_score:
                    ::core::option::Option<[::core::primitive::u128; 3usize]>,
            }
            impl ::subxt::Call for SetMinimumUntrustedScore {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const FUNCTION: &'static str = "set_minimum_untrusted_score";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct SetEmergencyElectionResult {
                pub supports: ::std::vec::Vec<(
                    ::subxt::sp_core::crypto::AccountId32,
                    runtime_types::sp_npos_elections::Support<
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                )>,
            }
            impl ::subxt::Call for SetEmergencyElectionResult {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const FUNCTION: &'static str = "set_emergency_election_result";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Submit { pub raw_solution : runtime_types :: pallet_election_provider_multi_phase :: RawSolution < runtime_types :: darkwebb_standalone_runtime :: NposSolution16 > , pub num_signed_submissions : :: core :: primitive :: u32 }
            impl ::subxt::Call for Submit {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const FUNCTION: &'static str = "submit";
            }
            pub struct TransactionApi<
                'a,
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            > {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn submit_unsigned(
                    &self,
                    raw_solution : runtime_types :: pallet_election_provider_multi_phase :: RawSolution < runtime_types :: darkwebb_standalone_runtime :: NposSolution16 >,
                    witness : runtime_types :: pallet_election_provider_multi_phase :: SolutionOrSnapshotSize,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, SubmitUnsigned>
                {
                    let call = SubmitUnsigned {
                        raw_solution,
                        witness,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_minimum_untrusted_score(
                    &self,
                    maybe_next_score: ::core::option::Option<
                        [::core::primitive::u128; 3usize],
                    >,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    SetMinimumUntrustedScore,
                > {
                    let call = SetMinimumUntrustedScore { maybe_next_score };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_emergency_election_result(
                    &self,
                    supports: ::std::vec::Vec<(
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::sp_npos_elections::Support<
                            ::subxt::sp_core::crypto::AccountId32,
                        >,
                    )>,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    SetEmergencyElectionResult,
                > {
                    let call = SetEmergencyElectionResult { supports };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn submit(
                    &self,
                    raw_solution : runtime_types :: pallet_election_provider_multi_phase :: RawSolution < runtime_types :: darkwebb_standalone_runtime :: NposSolution16 >,
                    num_signed_submissions: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, Submit>
                {
                    let call = Submit {
                        raw_solution,
                        num_signed_submissions,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event =
            runtime_types::pallet_election_provider_multi_phase::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct SolutionStored (pub runtime_types :: pallet_election_provider_multi_phase :: ElectionCompute , pub :: core :: primitive :: bool) ;
            impl ::subxt::Event for SolutionStored {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const EVENT: &'static str = "SolutionStored";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ElectionFinalized (pub :: core :: option :: Option < runtime_types :: pallet_election_provider_multi_phase :: ElectionCompute >) ;
            impl ::subxt::Event for ElectionFinalized {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const EVENT: &'static str = "ElectionFinalized";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Rewarded(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Rewarded {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const EVENT: &'static str = "Rewarded";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Slashed(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Slashed {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const EVENT: &'static str = "Slashed";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct SignedPhaseStarted(pub ::core::primitive::u32);
            impl ::subxt::Event for SignedPhaseStarted {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const EVENT: &'static str = "SignedPhaseStarted";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct UnsignedPhaseStarted(pub ::core::primitive::u32);
            impl ::subxt::Event for UnsignedPhaseStarted {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const EVENT: &'static str = "UnsignedPhaseStarted";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Round;
            impl ::subxt::StorageEntry for Round {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const STORAGE: &'static str = "Round";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct CurrentPhase;
            impl ::subxt::StorageEntry for CurrentPhase {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const STORAGE: &'static str = "CurrentPhase";
                type Value =
                    runtime_types::pallet_election_provider_multi_phase::Phase<
                        ::core::primitive::u32,
                    >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct QueuedSolution;
            impl ::subxt::StorageEntry for QueuedSolution {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const STORAGE: &'static str = "QueuedSolution";
                type Value = runtime_types :: pallet_election_provider_multi_phase :: ReadySolution < :: subxt :: sp_core :: crypto :: AccountId32 > ;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Snapshot;
            impl ::subxt::StorageEntry for Snapshot {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const STORAGE: &'static str = "Snapshot";
                type Value = runtime_types :: pallet_election_provider_multi_phase :: RoundSnapshot < :: subxt :: sp_core :: crypto :: AccountId32 > ;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct DesiredTargets;
            impl ::subxt::StorageEntry for DesiredTargets {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const STORAGE: &'static str = "DesiredTargets";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct SnapshotMetadata;
            impl ::subxt::StorageEntry for SnapshotMetadata {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const STORAGE: &'static str = "SnapshotMetadata";
                type Value = runtime_types :: pallet_election_provider_multi_phase :: SolutionOrSnapshotSize ;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct SignedSubmissionNextIndex;
            impl ::subxt::StorageEntry for SignedSubmissionNextIndex {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const STORAGE: &'static str = "SignedSubmissionNextIndex";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct SignedSubmissionIndices;
            impl ::subxt::StorageEntry for SignedSubmissionIndices {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const STORAGE: &'static str = "SignedSubmissionIndices";
                type Value = runtime_types :: frame_support :: storage :: bounded_btree_map :: BoundedBTreeMap < [:: core :: primitive :: u128 ; 3usize] , :: core :: primitive :: u32 > ;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct SignedSubmissionsMap(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for SignedSubmissionsMap {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const STORAGE: &'static str = "SignedSubmissionsMap";
                type Value = runtime_types :: pallet_election_provider_multi_phase :: signed :: SignedSubmission < :: subxt :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u128 , runtime_types :: darkwebb_standalone_runtime :: NposSolution16 > ;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                    ])
                }
            }
            pub struct MinimumUntrustedScore;
            impl ::subxt::StorageEntry for MinimumUntrustedScore {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const STORAGE: &'static str = "MinimumUntrustedScore";
                type Value = [::core::primitive::u128; 3usize];
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn round(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::u32,
                    ::subxt::Error,
                > {
                    let entry = Round;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn current_phase(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_election_provider_multi_phase::Phase<
                        ::core::primitive::u32,
                    >,
                    ::subxt::Error,
                > {
                    let entry = CurrentPhase;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }                pub async fn queued_solution (& self , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < :: core :: option :: Option < runtime_types :: pallet_election_provider_multi_phase :: ReadySolution < :: subxt :: sp_core :: crypto :: AccountId32 > > , :: subxt :: Error >{
                    let entry = QueuedSolution;
                    self.client.storage().fetch(&entry, hash).await
                }                pub async fn snapshot (& self , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < :: core :: option :: Option < runtime_types :: pallet_election_provider_multi_phase :: RoundSnapshot < :: subxt :: sp_core :: crypto :: AccountId32 > > , :: subxt :: Error >{
                    let entry = Snapshot;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn desired_targets(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::Error,
                > {
                    let entry = DesiredTargets;
                    self.client.storage().fetch(&entry, hash).await
                }                pub async fn snapshot_metadata (& self , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < :: core :: option :: Option < runtime_types :: pallet_election_provider_multi_phase :: SolutionOrSnapshotSize > , :: subxt :: Error >{
                    let entry = SnapshotMetadata;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn signed_submission_next_index(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::u32,
                    ::subxt::Error,
                > {
                    let entry = SignedSubmissionNextIndex;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }                pub async fn signed_submission_indices (& self , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < runtime_types :: frame_support :: storage :: bounded_btree_map :: BoundedBTreeMap < [:: core :: primitive :: u128 ; 3usize] , :: core :: primitive :: u32 > , :: subxt :: Error >{
                    let entry = SignedSubmissionIndices;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }                pub async fn signed_submissions_map (& self , _0 : :: core :: primitive :: u32 , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < runtime_types :: pallet_election_provider_multi_phase :: signed :: SignedSubmission < :: subxt :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u128 , runtime_types :: darkwebb_standalone_runtime :: NposSolution16 > , :: subxt :: Error >{
                    let entry = SignedSubmissionsMap(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn signed_submissions_map_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, SignedSubmissionsMap>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn minimum_untrusted_score(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<[::core::primitive::u128; 3usize]>,
                    ::subxt::Error,
                > {
                    let entry = MinimumUntrustedScore;
                    self.client.storage().fetch(&entry, hash).await
                }
            }
        }
    }
    pub mod staking {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Bond {
                pub controller: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                #[codec(compact)]
                pub value: ::core::primitive::u128,
                pub payee: runtime_types::pallet_staking::RewardDestination<
                    ::subxt::sp_core::crypto::AccountId32,
                >,
            }
            impl ::subxt::Call for Bond {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "bond";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct BondExtra {
                #[codec(compact)]
                pub max_additional: ::core::primitive::u128,
            }
            impl ::subxt::Call for BondExtra {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "bond_extra";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Unbond {
                #[codec(compact)]
                pub value: ::core::primitive::u128,
            }
            impl ::subxt::Call for Unbond {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "unbond";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct WithdrawUnbonded {
                pub num_slashing_spans: ::core::primitive::u32,
            }
            impl ::subxt::Call for WithdrawUnbonded {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "withdraw_unbonded";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Validate {
                pub prefs: runtime_types::pallet_staking::ValidatorPrefs,
            }
            impl ::subxt::Call for Validate {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "validate";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Nominate {
                pub targets: ::std::vec::Vec<
                    ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                >,
            }
            impl ::subxt::Call for Nominate {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "nominate";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Chill {}
            impl ::subxt::Call for Chill {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "chill";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct SetPayee {
                pub payee: runtime_types::pallet_staking::RewardDestination<
                    ::subxt::sp_core::crypto::AccountId32,
                >,
            }
            impl ::subxt::Call for SetPayee {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "set_payee";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct SetController {
                pub controller: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
            }
            impl ::subxt::Call for SetController {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "set_controller";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct SetValidatorCount {
                #[codec(compact)]
                pub new: ::core::primitive::u32,
            }
            impl ::subxt::Call for SetValidatorCount {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "set_validator_count";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct IncreaseValidatorCount {
                #[codec(compact)]
                pub additional: ::core::primitive::u32,
            }
            impl ::subxt::Call for IncreaseValidatorCount {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "increase_validator_count";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ScaleValidatorCount {
                pub factor: runtime_types::sp_arithmetic::per_things::Percent,
            }
            impl ::subxt::Call for ScaleValidatorCount {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "scale_validator_count";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ForceNoEras {}
            impl ::subxt::Call for ForceNoEras {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "force_no_eras";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ForceNewEra {}
            impl ::subxt::Call for ForceNewEra {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "force_new_era";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct SetInvulnerables {
                pub invulnerables:
                    ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
            }
            impl ::subxt::Call for SetInvulnerables {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "set_invulnerables";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ForceUnstake {
                pub stash: ::subxt::sp_core::crypto::AccountId32,
                pub num_slashing_spans: ::core::primitive::u32,
            }
            impl ::subxt::Call for ForceUnstake {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "force_unstake";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ForceNewEraAlways {}
            impl ::subxt::Call for ForceNewEraAlways {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "force_new_era_always";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct CancelDeferredSlash {
                pub era: ::core::primitive::u32,
                pub slash_indices: ::std::vec::Vec<::core::primitive::u32>,
            }
            impl ::subxt::Call for CancelDeferredSlash {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "cancel_deferred_slash";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct PayoutStakers {
                pub validator_stash: ::subxt::sp_core::crypto::AccountId32,
                pub era: ::core::primitive::u32,
            }
            impl ::subxt::Call for PayoutStakers {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "payout_stakers";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Rebond {
                #[codec(compact)]
                pub value: ::core::primitive::u128,
            }
            impl ::subxt::Call for Rebond {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "rebond";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct SetHistoryDepth {
                #[codec(compact)]
                pub new_history_depth: ::core::primitive::u32,
                #[codec(compact)]
                pub era_items_deleted: ::core::primitive::u32,
            }
            impl ::subxt::Call for SetHistoryDepth {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "set_history_depth";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ReapStash {
                pub stash: ::subxt::sp_core::crypto::AccountId32,
                pub num_slashing_spans: ::core::primitive::u32,
            }
            impl ::subxt::Call for ReapStash {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "reap_stash";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Kick {
                pub who: ::std::vec::Vec<
                    ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                >,
            }
            impl ::subxt::Call for Kick {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "kick";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct SetStakingLimits {
                pub min_nominator_bond: ::core::primitive::u128,
                pub min_validator_bond: ::core::primitive::u128,
                pub max_nominator_count:
                    ::core::option::Option<::core::primitive::u32>,
                pub max_validator_count:
                    ::core::option::Option<::core::primitive::u32>,
                pub threshold: ::core::option::Option<
                    runtime_types::sp_arithmetic::per_things::Percent,
                >,
            }
            impl ::subxt::Call for SetStakingLimits {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "set_staking_limits";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ChillOther {
                pub controller: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Call for ChillOther {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "chill_other";
            }
            pub struct TransactionApi<
                'a,
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            > {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn bond(
                    &self,
                    controller: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    value: ::core::primitive::u128,
                    payee: runtime_types::pallet_staking::RewardDestination<
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, Bond>
                {
                    let call = Bond {
                        controller,
                        value,
                        payee,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn bond_extra(
                    &self,
                    max_additional: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, BondExtra>
                {
                    let call = BondExtra { max_additional };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn unbond(
                    &self,
                    value: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, Unbond>
                {
                    let call = Unbond { value };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn withdraw_unbonded(
                    &self,
                    num_slashing_spans: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, WithdrawUnbonded>
                {
                    let call = WithdrawUnbonded { num_slashing_spans };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn validate(
                    &self,
                    prefs: runtime_types::pallet_staking::ValidatorPrefs,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, Validate>
                {
                    let call = Validate { prefs };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn nominate(
                    &self,
                    targets: ::std::vec::Vec<
                        ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, Nominate>
                {
                    let call = Nominate { targets };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn chill(
                    &self,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, Chill>
                {
                    let call = Chill {};
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_payee(
                    &self,
                    payee: runtime_types::pallet_staking::RewardDestination<
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, SetPayee>
                {
                    let call = SetPayee { payee };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_controller(
                    &self,
                    controller: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, SetController>
                {
                    let call = SetController { controller };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_validator_count(
                    &self,
                    new: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, SetValidatorCount>
                {
                    let call = SetValidatorCount { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn increase_validator_count(
                    &self,
                    additional: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, IncreaseValidatorCount>
                {
                    let call = IncreaseValidatorCount { additional };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn scale_validator_count(
                    &self,
                    factor: runtime_types::sp_arithmetic::per_things::Percent,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, ScaleValidatorCount>
                {
                    let call = ScaleValidatorCount { factor };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_no_eras(
                    &self,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, ForceNoEras>
                {
                    let call = ForceNoEras {};
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_new_era(
                    &self,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, ForceNewEra>
                {
                    let call = ForceNewEra {};
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_invulnerables(
                    &self,
                    invulnerables: ::std::vec::Vec<
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, SetInvulnerables>
                {
                    let call = SetInvulnerables { invulnerables };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_unstake(
                    &self,
                    stash: ::subxt::sp_core::crypto::AccountId32,
                    num_slashing_spans: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, ForceUnstake>
                {
                    let call = ForceUnstake {
                        stash,
                        num_slashing_spans,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_new_era_always(
                    &self,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, ForceNewEraAlways>
                {
                    let call = ForceNewEraAlways {};
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn cancel_deferred_slash(
                    &self,
                    era: ::core::primitive::u32,
                    slash_indices: ::std::vec::Vec<::core::primitive::u32>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, CancelDeferredSlash>
                {
                    let call = CancelDeferredSlash { era, slash_indices };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn payout_stakers(
                    &self,
                    validator_stash: ::subxt::sp_core::crypto::AccountId32,
                    era: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, PayoutStakers>
                {
                    let call = PayoutStakers {
                        validator_stash,
                        era,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn rebond(
                    &self,
                    value: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, Rebond>
                {
                    let call = Rebond { value };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_history_depth(
                    &self,
                    new_history_depth: ::core::primitive::u32,
                    era_items_deleted: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, SetHistoryDepth>
                {
                    let call = SetHistoryDepth {
                        new_history_depth,
                        era_items_deleted,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn reap_stash(
                    &self,
                    stash: ::subxt::sp_core::crypto::AccountId32,
                    num_slashing_spans: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, ReapStash>
                {
                    let call = ReapStash {
                        stash,
                        num_slashing_spans,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn kick(
                    &self,
                    who: ::std::vec::Vec<
                        ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, Kick>
                {
                    let call = Kick { who };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_staking_limits(
                    &self,
                    min_nominator_bond: ::core::primitive::u128,
                    min_validator_bond: ::core::primitive::u128,
                    max_nominator_count: ::core::option::Option<
                        ::core::primitive::u32,
                    >,
                    max_validator_count: ::core::option::Option<
                        ::core::primitive::u32,
                    >,
                    threshold: ::core::option::Option<
                        runtime_types::sp_arithmetic::per_things::Percent,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, SetStakingLimits>
                {
                    let call = SetStakingLimits {
                        min_nominator_bond,
                        min_validator_bond,
                        max_nominator_count,
                        max_validator_count,
                        threshold,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn chill_other(
                    &self,
                    controller: ::subxt::sp_core::crypto::AccountId32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, ChillOther>
                {
                    let call = ChillOther { controller };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_staking::pallet::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct EraPaid(
                pub ::core::primitive::u32,
                pub ::core::primitive::u128,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for EraPaid {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "EraPaid";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Rewarded(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Rewarded {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "Rewarded";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Slashed(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Slashed {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "Slashed";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct OldSlashingReportDiscarded(pub ::core::primitive::u32);
            impl ::subxt::Event for OldSlashingReportDiscarded {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "OldSlashingReportDiscarded";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct StakersElected {}
            impl ::subxt::Event for StakersElected {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "StakersElected";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Bonded(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Bonded {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "Bonded";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Unbonded(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Unbonded {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "Unbonded";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Withdrawn(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Withdrawn {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "Withdrawn";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Kicked(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for Kicked {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "Kicked";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct StakingElectionFailed {}
            impl ::subxt::Event for StakingElectionFailed {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "StakingElectionFailed";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Chilled(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::Event for Chilled {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "Chilled";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct PayoutStarted(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for PayoutStarted {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "PayoutStarted";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct HistoryDepth;
            impl ::subxt::StorageEntry for HistoryDepth {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "HistoryDepth";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ValidatorCount;
            impl ::subxt::StorageEntry for ValidatorCount {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "ValidatorCount";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct MinimumValidatorCount;
            impl ::subxt::StorageEntry for MinimumValidatorCount {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "MinimumValidatorCount";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Invulnerables;
            impl ::subxt::StorageEntry for Invulnerables {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "Invulnerables";
                type Value =
                    ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Bonded(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Bonded {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "Bonded";
                type Value = ::subxt::sp_core::crypto::AccountId32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                    ])
                }
            }
            pub struct MinNominatorBond;
            impl ::subxt::StorageEntry for MinNominatorBond {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "MinNominatorBond";
                type Value = ::core::primitive::u128;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct MinValidatorBond;
            impl ::subxt::StorageEntry for MinValidatorBond {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "MinValidatorBond";
                type Value = ::core::primitive::u128;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Ledger(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Ledger {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "Ledger";
                type Value = runtime_types::pallet_staking::StakingLedger<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u128,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct Payee(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Payee {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "Payee";
                type Value = runtime_types::pallet_staking::RewardDestination<
                    ::subxt::sp_core::crypto::AccountId32,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                    ])
                }
            }
            pub struct Validators(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Validators {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "Validators";
                type Value = runtime_types::pallet_staking::ValidatorPrefs;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                    ])
                }
            }
            pub struct CounterForValidators;
            impl ::subxt::StorageEntry for CounterForValidators {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "CounterForValidators";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct MaxValidatorsCount;
            impl ::subxt::StorageEntry for MaxValidatorsCount {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "MaxValidatorsCount";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Nominators(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Nominators {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "Nominators";
                type Value = runtime_types::pallet_staking::Nominations<
                    ::subxt::sp_core::crypto::AccountId32,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                    ])
                }
            }
            pub struct CounterForNominators;
            impl ::subxt::StorageEntry for CounterForNominators {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "CounterForNominators";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct MaxNominatorsCount;
            impl ::subxt::StorageEntry for MaxNominatorsCount {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "MaxNominatorsCount";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct CurrentEra;
            impl ::subxt::StorageEntry for CurrentEra {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "CurrentEra";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ActiveEra;
            impl ::subxt::StorageEntry for ActiveEra {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "ActiveEra";
                type Value = runtime_types::pallet_staking::ActiveEraInfo;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ErasStartSessionIndex(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for ErasStartSessionIndex {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "ErasStartSessionIndex";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                    ])
                }
            }
            pub struct ErasStakers(
                ::core::primitive::u32,
                ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::StorageEntry for ErasStakers {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "ErasStakers";
                type Value = runtime_types::pallet_staking::Exposure<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u128,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                    ])
                }
            }
            pub struct ErasStakersClipped(
                ::core::primitive::u32,
                ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::StorageEntry for ErasStakersClipped {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "ErasStakersClipped";
                type Value = runtime_types::pallet_staking::Exposure<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u128,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                    ])
                }
            }
            pub struct ErasValidatorPrefs(
                ::core::primitive::u32,
                ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::StorageEntry for ErasValidatorPrefs {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "ErasValidatorPrefs";
                type Value = runtime_types::pallet_staking::ValidatorPrefs;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                    ])
                }
            }
            pub struct ErasValidatorReward(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for ErasValidatorReward {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "ErasValidatorReward";
                type Value = ::core::primitive::u128;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                    ])
                }
            }
            pub struct ErasRewardPoints(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for ErasRewardPoints {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "ErasRewardPoints";
                type Value = runtime_types::pallet_staking::EraRewardPoints<
                    ::subxt::sp_core::crypto::AccountId32,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                    ])
                }
            }
            pub struct ErasTotalStake(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for ErasTotalStake {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "ErasTotalStake";
                type Value = ::core::primitive::u128;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                    ])
                }
            }
            pub struct ForceEra;
            impl ::subxt::StorageEntry for ForceEra {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "ForceEra";
                type Value = runtime_types::pallet_staking::Forcing;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct SlashRewardFraction;
            impl ::subxt::StorageEntry for SlashRewardFraction {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "SlashRewardFraction";
                type Value = runtime_types::sp_arithmetic::per_things::Perbill;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct CanceledSlashPayout;
            impl ::subxt::StorageEntry for CanceledSlashPayout {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "CanceledSlashPayout";
                type Value = ::core::primitive::u128;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct UnappliedSlashes(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for UnappliedSlashes {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "UnappliedSlashes";
                type Value = ::std::vec::Vec<
                    runtime_types::pallet_staking::UnappliedSlash<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    >,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                    ])
                }
            }
            pub struct BondedEras;
            impl ::subxt::StorageEntry for BondedEras {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "BondedEras";
                type Value = ::std::vec::Vec<(
                    ::core::primitive::u32,
                    ::core::primitive::u32,
                )>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ValidatorSlashInEra(
                ::core::primitive::u32,
                ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::StorageEntry for ValidatorSlashInEra {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "ValidatorSlashInEra";
                type Value = (
                    runtime_types::sp_arithmetic::per_things::Perbill,
                    ::core::primitive::u128,
                );
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                    ])
                }
            }
            pub struct NominatorSlashInEra(
                ::core::primitive::u32,
                ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::StorageEntry for NominatorSlashInEra {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "NominatorSlashInEra";
                type Value = ::core::primitive::u128;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                    ])
                }
            }
            pub struct SlashingSpans(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for SlashingSpans {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "SlashingSpans";
                type Value =
                    runtime_types::pallet_staking::slashing::SlashingSpans;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                    ])
                }
            }
            pub struct SpanSlash(
                ::subxt::sp_core::crypto::AccountId32,
                ::core::primitive::u32,
            );
            impl ::subxt::StorageEntry for SpanSlash {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "SpanSlash";
                type Value =
                    runtime_types::pallet_staking::slashing::SpanRecord<
                        ::core::primitive::u128,
                    >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                    ])
                }
            }
            pub struct EarliestUnappliedSlash;
            impl ::subxt::StorageEntry for EarliestUnappliedSlash {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "EarliestUnappliedSlash";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct CurrentPlannedSession;
            impl ::subxt::StorageEntry for CurrentPlannedSession {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "CurrentPlannedSession";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct OffendingValidators;
            impl ::subxt::StorageEntry for OffendingValidators {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "OffendingValidators";
                type Value = ::std::vec::Vec<(
                    ::core::primitive::u32,
                    ::core::primitive::bool,
                )>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageVersion;
            impl ::subxt::StorageEntry for StorageVersion {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "StorageVersion";
                type Value = runtime_types::pallet_staking::Releases;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ChillThreshold;
            impl ::subxt::StorageEntry for ChillThreshold {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "ChillThreshold";
                type Value = runtime_types::sp_arithmetic::per_things::Percent;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn history_depth(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::u32,
                    ::subxt::Error,
                > {
                    let entry = HistoryDepth;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn validator_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::u32,
                    ::subxt::Error,
                > {
                    let entry = ValidatorCount;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn minimum_validator_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::u32,
                    ::subxt::Error,
                > {
                    let entry = MinimumValidatorCount;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn invulnerables(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                    ::subxt::Error,
                > {
                    let entry = Invulnerables;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn bonded(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Bonded(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn bonded_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Bonded>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn min_nominator_bond(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::u128,
                    ::subxt::Error,
                > {
                    let entry = MinNominatorBond;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn min_validator_bond(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::u128,
                    ::subxt::Error,
                > {
                    let entry = MinValidatorBond;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn ledger(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_staking::StakingLedger<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Ledger(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn ledger_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Ledger>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn payee(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_staking::RewardDestination<
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Payee(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn payee_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Payee>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn validators(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_staking::ValidatorPrefs,
                    ::subxt::Error,
                > {
                    let entry = Validators(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn validators_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Validators>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn counter_for_validators(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::u32,
                    ::subxt::Error,
                > {
                    let entry = CounterForValidators;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn max_validators_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::Error,
                > {
                    let entry = MaxValidatorsCount;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn nominators(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_staking::Nominations<
                            ::subxt::sp_core::crypto::AccountId32,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Nominators(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn nominators_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Nominators>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn counter_for_nominators(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::u32,
                    ::subxt::Error,
                > {
                    let entry = CounterForNominators;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn max_nominators_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::Error,
                > {
                    let entry = MaxNominatorsCount;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn current_era(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::Error,
                > {
                    let entry = CurrentEra;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn active_era(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_staking::ActiveEraInfo,
                    >,
                    ::subxt::Error,
                > {
                    let entry = ActiveEra;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn eras_start_session_index(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::Error,
                > {
                    let entry = ErasStartSessionIndex(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn eras_start_session_index_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ErasStartSessionIndex>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn eras_stakers(
                    &self,
                    _0: ::core::primitive::u32,
                    _1: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_staking::Exposure<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    >,
                    ::subxt::Error,
                > {
                    let entry = ErasStakers(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn eras_stakers_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ErasStakers>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn eras_stakers_clipped(
                    &self,
                    _0: ::core::primitive::u32,
                    _1: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_staking::Exposure<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    >,
                    ::subxt::Error,
                > {
                    let entry = ErasStakersClipped(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn eras_stakers_clipped_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ErasStakersClipped>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn eras_validator_prefs(
                    &self,
                    _0: ::core::primitive::u32,
                    _1: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_staking::ValidatorPrefs,
                    ::subxt::Error,
                > {
                    let entry = ErasValidatorPrefs(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn eras_validator_prefs_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ErasValidatorPrefs>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn eras_validator_reward(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u128>,
                    ::subxt::Error,
                > {
                    let entry = ErasValidatorReward(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn eras_validator_reward_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ErasValidatorReward>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn eras_reward_points(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_staking::EraRewardPoints<
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                    ::subxt::Error,
                > {
                    let entry = ErasRewardPoints(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn eras_reward_points_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ErasRewardPoints>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn eras_total_stake(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::u128,
                    ::subxt::Error,
                > {
                    let entry = ErasTotalStake(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn eras_total_stake_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ErasTotalStake>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn force_era(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_staking::Forcing,
                    ::subxt::Error,
                > {
                    let entry = ForceEra;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn slash_reward_fraction(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::sp_arithmetic::per_things::Perbill,
                    ::subxt::Error,
                > {
                    let entry = SlashRewardFraction;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn canceled_slash_payout(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::u128,
                    ::subxt::Error,
                > {
                    let entry = CanceledSlashPayout;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn unapplied_slashes(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<
                        runtime_types::pallet_staking::UnappliedSlash<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = UnappliedSlashes(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn unapplied_slashes_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, UnappliedSlashes>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn bonded_eras(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<(
                        ::core::primitive::u32,
                        ::core::primitive::u32,
                    )>,
                    ::subxt::Error,
                > {
                    let entry = BondedEras;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn validator_slash_in_era(
                    &self,
                    _0: ::core::primitive::u32,
                    _1: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<(
                        runtime_types::sp_arithmetic::per_things::Perbill,
                        ::core::primitive::u128,
                    )>,
                    ::subxt::Error,
                > {
                    let entry = ValidatorSlashInEra(_0, _1);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn validator_slash_in_era_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ValidatorSlashInEra>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn nominator_slash_in_era(
                    &self,
                    _0: ::core::primitive::u32,
                    _1: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u128>,
                    ::subxt::Error,
                > {
                    let entry = NominatorSlashInEra(_0, _1);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn nominator_slash_in_era_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, NominatorSlashInEra>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn slashing_spans(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_staking::slashing::SlashingSpans,
                    >,
                    ::subxt::Error,
                > {
                    let entry = SlashingSpans(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn slashing_spans_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, SlashingSpans>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn span_slash(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    _1: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_staking::slashing::SpanRecord<
                        ::core::primitive::u128,
                    >,
                    ::subxt::Error,
                > {
                    let entry = SpanSlash(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn span_slash_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, SpanSlash>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn earliest_unapplied_slash(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::Error,
                > {
                    let entry = EarliestUnappliedSlash;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn current_planned_session(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::u32,
                    ::subxt::Error,
                > {
                    let entry = CurrentPlannedSession;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn offending_validators(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<(
                        ::core::primitive::u32,
                        ::core::primitive::bool,
                    )>,
                    ::subxt::Error,
                > {
                    let entry = OffendingValidators;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn storage_version(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_staking::Releases,
                    ::subxt::Error,
                > {
                    let entry = StorageVersion;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn chill_threshold(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::sp_arithmetic::per_things::Percent,
                    >,
                    ::subxt::Error,
                > {
                    let entry = ChillThreshold;
                    self.client.storage().fetch(&entry, hash).await
                }
            }
        }
    }
    pub mod session {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct SetKeys {
                pub keys:
                    runtime_types::darkwebb_standalone_runtime::SessionKeys,
                pub proof: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for SetKeys {
                const PALLET: &'static str = "Session";
                const FUNCTION: &'static str = "set_keys";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct PurgeKeys {}
            impl ::subxt::Call for PurgeKeys {
                const PALLET: &'static str = "Session";
                const FUNCTION: &'static str = "purge_keys";
            }
            pub struct TransactionApi<
                'a,
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            > {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn set_keys(
                    &self,
                    keys : runtime_types :: darkwebb_standalone_runtime :: SessionKeys,
                    proof: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, SetKeys>
                {
                    let call = SetKeys { keys, proof };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn purge_keys(
                    &self,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, PurgeKeys>
                {
                    let call = PurgeKeys {};
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_session::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct NewSession(pub ::core::primitive::u32);
            impl ::subxt::Event for NewSession {
                const PALLET: &'static str = "Session";
                const EVENT: &'static str = "NewSession";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Validators;
            impl ::subxt::StorageEntry for Validators {
                const PALLET: &'static str = "Session";
                const STORAGE: &'static str = "Validators";
                type Value =
                    ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct CurrentIndex;
            impl ::subxt::StorageEntry for CurrentIndex {
                const PALLET: &'static str = "Session";
                const STORAGE: &'static str = "CurrentIndex";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct QueuedChanged;
            impl ::subxt::StorageEntry for QueuedChanged {
                const PALLET: &'static str = "Session";
                const STORAGE: &'static str = "QueuedChanged";
                type Value = ::core::primitive::bool;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct QueuedKeys;
            impl ::subxt::StorageEntry for QueuedKeys {
                const PALLET: &'static str = "Session";
                const STORAGE: &'static str = "QueuedKeys";
                type Value = ::std::vec::Vec<(
                    ::subxt::sp_core::crypto::AccountId32,
                    runtime_types::darkwebb_standalone_runtime::SessionKeys,
                )>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct DisabledValidators;
            impl ::subxt::StorageEntry for DisabledValidators {
                const PALLET: &'static str = "Session";
                const STORAGE: &'static str = "DisabledValidators";
                type Value = ::std::vec::Vec<::core::primitive::u32>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NextKeys(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for NextKeys {
                const PALLET: &'static str = "Session";
                const STORAGE: &'static str = "NextKeys";
                type Value =
                    runtime_types::darkwebb_standalone_runtime::SessionKeys;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                    ])
                }
            }
            pub struct KeyOwner(
                runtime_types::sp_core::crypto::KeyTypeId,
                ::std::vec::Vec<::core::primitive::u8>,
            );
            impl ::subxt::StorageEntry for KeyOwner {
                const PALLET: &'static str = "Session";
                const STORAGE: &'static str = "KeyOwner";
                type Value = ::subxt::sp_core::crypto::AccountId32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                    ])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn validators(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                    ::subxt::Error,
                > {
                    let entry = Validators;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn current_index(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::u32,
                    ::subxt::Error,
                > {
                    let entry = CurrentIndex;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn queued_changed(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::bool,
                    ::subxt::Error,
                > {
                    let entry = QueuedChanged;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn queued_keys(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<(
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::darkwebb_standalone_runtime::SessionKeys,
                    )>,
                    ::subxt::Error,
                > {
                    let entry = QueuedKeys;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn disabled_validators(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::core::primitive::u32>,
                    ::subxt::Error,
                > {
                    let entry = DisabledValidators;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn next_keys(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::darkwebb_standalone_runtime::SessionKeys,
                    >,
                    ::subxt::Error,
                > {
                    let entry = NextKeys(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn next_keys_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, NextKeys>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn key_owner(
                    &self,
                    _0: runtime_types::sp_core::crypto::KeyTypeId,
                    _1: ::std::vec::Vec<::core::primitive::u8>,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                    ::subxt::Error,
                > {
                    let entry = KeyOwner(_0, _1);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn key_owner_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, KeyOwner>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod democracy {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Propose {
                pub proposal_hash: ::subxt::sp_core::H256,
                #[codec(compact)]
                pub value: ::core::primitive::u128,
            }
            impl ::subxt::Call for Propose {
                const PALLET: &'static str = "Democracy";
                const FUNCTION: &'static str = "propose";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Second {
                #[codec(compact)]
                pub proposal: ::core::primitive::u32,
                #[codec(compact)]
                pub seconds_upper_bound: ::core::primitive::u32,
            }
            impl ::subxt::Call for Second {
                const PALLET: &'static str = "Democracy";
                const FUNCTION: &'static str = "second";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Vote {
                #[codec(compact)]
                pub ref_index: ::core::primitive::u32,
                pub vote: runtime_types::pallet_democracy::vote::AccountVote<
                    ::core::primitive::u128,
                >,
            }
            impl ::subxt::Call for Vote {
                const PALLET: &'static str = "Democracy";
                const FUNCTION: &'static str = "vote";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct EmergencyCancel {
                pub ref_index: ::core::primitive::u32,
            }
            impl ::subxt::Call for EmergencyCancel {
                const PALLET: &'static str = "Democracy";
                const FUNCTION: &'static str = "emergency_cancel";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ExternalPropose {
                pub proposal_hash: ::subxt::sp_core::H256,
            }
            impl ::subxt::Call for ExternalPropose {
                const PALLET: &'static str = "Democracy";
                const FUNCTION: &'static str = "external_propose";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ExternalProposeMajority {
                pub proposal_hash: ::subxt::sp_core::H256,
            }
            impl ::subxt::Call for ExternalProposeMajority {
                const PALLET: &'static str = "Democracy";
                const FUNCTION: &'static str = "external_propose_majority";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ExternalProposeDefault {
                pub proposal_hash: ::subxt::sp_core::H256,
            }
            impl ::subxt::Call for ExternalProposeDefault {
                const PALLET: &'static str = "Democracy";
                const FUNCTION: &'static str = "external_propose_default";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct FastTrack {
                pub proposal_hash: ::subxt::sp_core::H256,
                pub voting_period: ::core::primitive::u32,
                pub delay: ::core::primitive::u32,
            }
            impl ::subxt::Call for FastTrack {
                const PALLET: &'static str = "Democracy";
                const FUNCTION: &'static str = "fast_track";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct VetoExternal {
                pub proposal_hash: ::subxt::sp_core::H256,
            }
            impl ::subxt::Call for VetoExternal {
                const PALLET: &'static str = "Democracy";
                const FUNCTION: &'static str = "veto_external";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct CancelReferendum {
                #[codec(compact)]
                pub ref_index: ::core::primitive::u32,
            }
            impl ::subxt::Call for CancelReferendum {
                const PALLET: &'static str = "Democracy";
                const FUNCTION: &'static str = "cancel_referendum";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct CancelQueued {
                pub which: ::core::primitive::u32,
            }
            impl ::subxt::Call for CancelQueued {
                const PALLET: &'static str = "Democracy";
                const FUNCTION: &'static str = "cancel_queued";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Delegate {
                pub to: ::subxt::sp_core::crypto::AccountId32,
                pub conviction:
                    runtime_types::pallet_democracy::conviction::Conviction,
                pub balance: ::core::primitive::u128,
            }
            impl ::subxt::Call for Delegate {
                const PALLET: &'static str = "Democracy";
                const FUNCTION: &'static str = "delegate";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Undelegate {}
            impl ::subxt::Call for Undelegate {
                const PALLET: &'static str = "Democracy";
                const FUNCTION: &'static str = "undelegate";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ClearPublicProposals {}
            impl ::subxt::Call for ClearPublicProposals {
                const PALLET: &'static str = "Democracy";
                const FUNCTION: &'static str = "clear_public_proposals";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct NotePreimage {
                pub encoded_proposal: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for NotePreimage {
                const PALLET: &'static str = "Democracy";
                const FUNCTION: &'static str = "note_preimage";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct NotePreimageOperational {
                pub encoded_proposal: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for NotePreimageOperational {
                const PALLET: &'static str = "Democracy";
                const FUNCTION: &'static str = "note_preimage_operational";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct NoteImminentPreimage {
                pub encoded_proposal: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for NoteImminentPreimage {
                const PALLET: &'static str = "Democracy";
                const FUNCTION: &'static str = "note_imminent_preimage";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct NoteImminentPreimageOperational {
                pub encoded_proposal: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for NoteImminentPreimageOperational {
                const PALLET: &'static str = "Democracy";
                const FUNCTION: &'static str =
                    "note_imminent_preimage_operational";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ReapPreimage {
                pub proposal_hash: ::subxt::sp_core::H256,
                #[codec(compact)]
                pub proposal_len_upper_bound: ::core::primitive::u32,
            }
            impl ::subxt::Call for ReapPreimage {
                const PALLET: &'static str = "Democracy";
                const FUNCTION: &'static str = "reap_preimage";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Unlock {
                pub target: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Call for Unlock {
                const PALLET: &'static str = "Democracy";
                const FUNCTION: &'static str = "unlock";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct RemoveVote {
                pub index: ::core::primitive::u32,
            }
            impl ::subxt::Call for RemoveVote {
                const PALLET: &'static str = "Democracy";
                const FUNCTION: &'static str = "remove_vote";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct RemoveOtherVote {
                pub target: ::subxt::sp_core::crypto::AccountId32,
                pub index: ::core::primitive::u32,
            }
            impl ::subxt::Call for RemoveOtherVote {
                const PALLET: &'static str = "Democracy";
                const FUNCTION: &'static str = "remove_other_vote";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct EnactProposal {
                pub proposal_hash: ::subxt::sp_core::H256,
                pub index: ::core::primitive::u32,
            }
            impl ::subxt::Call for EnactProposal {
                const PALLET: &'static str = "Democracy";
                const FUNCTION: &'static str = "enact_proposal";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Blacklist {
                pub proposal_hash: ::subxt::sp_core::H256,
                pub maybe_ref_index:
                    ::core::option::Option<::core::primitive::u32>,
            }
            impl ::subxt::Call for Blacklist {
                const PALLET: &'static str = "Democracy";
                const FUNCTION: &'static str = "blacklist";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct CancelProposal {
                #[codec(compact)]
                pub prop_index: ::core::primitive::u32,
            }
            impl ::subxt::Call for CancelProposal {
                const PALLET: &'static str = "Democracy";
                const FUNCTION: &'static str = "cancel_proposal";
            }
            pub struct TransactionApi<
                'a,
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            > {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn propose(
                    &self,
                    proposal_hash: ::subxt::sp_core::H256,
                    value: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, Propose>
                {
                    let call = Propose {
                        proposal_hash,
                        value,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn second(
                    &self,
                    proposal: ::core::primitive::u32,
                    seconds_upper_bound: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, Second>
                {
                    let call = Second {
                        proposal,
                        seconds_upper_bound,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn vote(
                    &self,
                    ref_index: ::core::primitive::u32,
                    vote: runtime_types::pallet_democracy::vote::AccountVote<
                        ::core::primitive::u128,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, Vote>
                {
                    let call = Vote { ref_index, vote };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn emergency_cancel(
                    &self,
                    ref_index: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, EmergencyCancel>
                {
                    let call = EmergencyCancel { ref_index };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn external_propose(
                    &self,
                    proposal_hash: ::subxt::sp_core::H256,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, ExternalPropose>
                {
                    let call = ExternalPropose { proposal_hash };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn external_propose_majority(
                    &self,
                    proposal_hash: ::subxt::sp_core::H256,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, ExternalProposeMajority>
                {
                    let call = ExternalProposeMajority { proposal_hash };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn external_propose_default(
                    &self,
                    proposal_hash: ::subxt::sp_core::H256,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, ExternalProposeDefault>
                {
                    let call = ExternalProposeDefault { proposal_hash };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn fast_track(
                    &self,
                    proposal_hash: ::subxt::sp_core::H256,
                    voting_period: ::core::primitive::u32,
                    delay: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, FastTrack>
                {
                    let call = FastTrack {
                        proposal_hash,
                        voting_period,
                        delay,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn veto_external(
                    &self,
                    proposal_hash: ::subxt::sp_core::H256,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, VetoExternal>
                {
                    let call = VetoExternal { proposal_hash };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn cancel_referendum(
                    &self,
                    ref_index: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, CancelReferendum>
                {
                    let call = CancelReferendum { ref_index };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn cancel_queued(
                    &self,
                    which: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, CancelQueued>
                {
                    let call = CancelQueued { which };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn delegate(
                    &self,
                    to: ::subxt::sp_core::crypto::AccountId32,
                    conviction : runtime_types :: pallet_democracy :: conviction :: Conviction,
                    balance: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, Delegate>
                {
                    let call = Delegate {
                        to,
                        conviction,
                        balance,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn undelegate(
                    &self,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, Undelegate>
                {
                    let call = Undelegate {};
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn clear_public_proposals(
                    &self,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, ClearPublicProposals>
                {
                    let call = ClearPublicProposals {};
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn note_preimage(
                    &self,
                    encoded_proposal: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, NotePreimage>
                {
                    let call = NotePreimage { encoded_proposal };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn note_preimage_operational(
                    &self,
                    encoded_proposal: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, NotePreimageOperational>
                {
                    let call = NotePreimageOperational { encoded_proposal };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn note_imminent_preimage(
                    &self,
                    encoded_proposal: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, NoteImminentPreimage>
                {
                    let call = NoteImminentPreimage { encoded_proposal };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn note_imminent_preimage_operational(
                    &self,
                    encoded_proposal: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    NoteImminentPreimageOperational,
                > {
                    let call =
                        NoteImminentPreimageOperational { encoded_proposal };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn reap_preimage(
                    &self,
                    proposal_hash: ::subxt::sp_core::H256,
                    proposal_len_upper_bound: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, ReapPreimage>
                {
                    let call = ReapPreimage {
                        proposal_hash,
                        proposal_len_upper_bound,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn unlock(
                    &self,
                    target: ::subxt::sp_core::crypto::AccountId32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, Unlock>
                {
                    let call = Unlock { target };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn remove_vote(
                    &self,
                    index: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, RemoveVote>
                {
                    let call = RemoveVote { index };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn remove_other_vote(
                    &self,
                    target: ::subxt::sp_core::crypto::AccountId32,
                    index: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, RemoveOtherVote>
                {
                    let call = RemoveOtherVote { target, index };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn enact_proposal(
                    &self,
                    proposal_hash: ::subxt::sp_core::H256,
                    index: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, EnactProposal>
                {
                    let call = EnactProposal {
                        proposal_hash,
                        index,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn blacklist(
                    &self,
                    proposal_hash: ::subxt::sp_core::H256,
                    maybe_ref_index: ::core::option::Option<
                        ::core::primitive::u32,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, Blacklist>
                {
                    let call = Blacklist {
                        proposal_hash,
                        maybe_ref_index,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn cancel_proposal(
                    &self,
                    prop_index: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, CancelProposal>
                {
                    let call = CancelProposal { prop_index };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_democracy::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Proposed(
                pub ::core::primitive::u32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Proposed {
                const PALLET: &'static str = "Democracy";
                const EVENT: &'static str = "Proposed";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Tabled(
                pub ::core::primitive::u32,
                pub ::core::primitive::u128,
                pub ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
            );
            impl ::subxt::Event for Tabled {
                const PALLET: &'static str = "Democracy";
                const EVENT: &'static str = "Tabled";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ExternalTabled {}
            impl ::subxt::Event for ExternalTabled {
                const PALLET: &'static str = "Democracy";
                const EVENT: &'static str = "ExternalTabled";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Started (pub :: core :: primitive :: u32 , pub runtime_types :: pallet_democracy :: vote_threshold :: VoteThreshold) ;
            impl ::subxt::Event for Started {
                const PALLET: &'static str = "Democracy";
                const EVENT: &'static str = "Started";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Passed(pub ::core::primitive::u32);
            impl ::subxt::Event for Passed {
                const PALLET: &'static str = "Democracy";
                const EVENT: &'static str = "Passed";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct NotPassed(pub ::core::primitive::u32);
            impl ::subxt::Event for NotPassed {
                const PALLET: &'static str = "Democracy";
                const EVENT: &'static str = "NotPassed";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Cancelled(pub ::core::primitive::u32);
            impl ::subxt::Event for Cancelled {
                const PALLET: &'static str = "Democracy";
                const EVENT: &'static str = "Cancelled";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Executed(
                pub ::core::primitive::u32,
                pub  ::core::result::Result<
                    (),
                    runtime_types::sp_runtime::DispatchError,
                >,
            );
            impl ::subxt::Event for Executed {
                const PALLET: &'static str = "Democracy";
                const EVENT: &'static str = "Executed";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Delegated(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for Delegated {
                const PALLET: &'static str = "Democracy";
                const EVENT: &'static str = "Delegated";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Undelegated(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::Event for Undelegated {
                const PALLET: &'static str = "Democracy";
                const EVENT: &'static str = "Undelegated";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Vetoed(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::H256,
                pub ::core::primitive::u32,
            );
            impl ::subxt::Event for Vetoed {
                const PALLET: &'static str = "Democracy";
                const EVENT: &'static str = "Vetoed";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct PreimageNoted(
                pub ::subxt::sp_core::H256,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for PreimageNoted {
                const PALLET: &'static str = "Democracy";
                const EVENT: &'static str = "PreimageNoted";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct PreimageUsed(
                pub ::subxt::sp_core::H256,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for PreimageUsed {
                const PALLET: &'static str = "Democracy";
                const EVENT: &'static str = "PreimageUsed";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct PreimageInvalid(
                pub ::subxt::sp_core::H256,
                pub ::core::primitive::u32,
            );
            impl ::subxt::Event for PreimageInvalid {
                const PALLET: &'static str = "Democracy";
                const EVENT: &'static str = "PreimageInvalid";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct PreimageMissing(
                pub ::subxt::sp_core::H256,
                pub ::core::primitive::u32,
            );
            impl ::subxt::Event for PreimageMissing {
                const PALLET: &'static str = "Democracy";
                const EVENT: &'static str = "PreimageMissing";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct PreimageReaped(
                pub ::subxt::sp_core::H256,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for PreimageReaped {
                const PALLET: &'static str = "Democracy";
                const EVENT: &'static str = "PreimageReaped";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Blacklisted(pub ::subxt::sp_core::H256);
            impl ::subxt::Event for Blacklisted {
                const PALLET: &'static str = "Democracy";
                const EVENT: &'static str = "Blacklisted";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct PublicPropCount;
            impl ::subxt::StorageEntry for PublicPropCount {
                const PALLET: &'static str = "Democracy";
                const STORAGE: &'static str = "PublicPropCount";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct PublicProps;
            impl ::subxt::StorageEntry for PublicProps {
                const PALLET: &'static str = "Democracy";
                const STORAGE: &'static str = "PublicProps";
                type Value = ::std::vec::Vec<(
                    ::core::primitive::u32,
                    ::subxt::sp_core::H256,
                    ::subxt::sp_core::crypto::AccountId32,
                )>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct DepositOf(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for DepositOf {
                const PALLET: &'static str = "Democracy";
                const STORAGE: &'static str = "DepositOf";
                type Value = (
                    ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                    ::core::primitive::u128,
                );
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                    ])
                }
            }
            pub struct Preimages(pub ::subxt::sp_core::H256);
            impl ::subxt::StorageEntry for Preimages {
                const PALLET: &'static str = "Democracy";
                const STORAGE: &'static str = "Preimages";
                type Value = runtime_types::pallet_democracy::PreimageStatus<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u128,
                    ::core::primitive::u32,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Identity,
                        ),
                    ])
                }
            }
            pub struct ReferendumCount;
            impl ::subxt::StorageEntry for ReferendumCount {
                const PALLET: &'static str = "Democracy";
                const STORAGE: &'static str = "ReferendumCount";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct LowestUnbaked;
            impl ::subxt::StorageEntry for LowestUnbaked {
                const PALLET: &'static str = "Democracy";
                const STORAGE: &'static str = "LowestUnbaked";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ReferendumInfoOf(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for ReferendumInfoOf {
                const PALLET: &'static str = "Democracy";
                const STORAGE: &'static str = "ReferendumInfoOf";
                type Value =
                    runtime_types::pallet_democracy::types::ReferendumInfo<
                        ::core::primitive::u32,
                        ::subxt::sp_core::H256,
                        ::core::primitive::u128,
                    >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                    ])
                }
            }
            pub struct VotingOf(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for VotingOf {
                const PALLET: &'static str = "Democracy";
                const STORAGE: &'static str = "VotingOf";
                type Value = runtime_types::pallet_democracy::vote::Voting<
                    ::core::primitive::u128,
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                    ])
                }
            }
            pub struct Locks(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Locks {
                const PALLET: &'static str = "Democracy";
                const STORAGE: &'static str = "Locks";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                    ])
                }
            }
            pub struct LastTabledWasExternal;
            impl ::subxt::StorageEntry for LastTabledWasExternal {
                const PALLET: &'static str = "Democracy";
                const STORAGE: &'static str = "LastTabledWasExternal";
                type Value = ::core::primitive::bool;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NextExternal;
            impl ::subxt::StorageEntry for NextExternal {
                const PALLET: &'static str = "Democracy";
                const STORAGE: &'static str = "NextExternal";
                type Value = (:: subxt :: sp_core :: H256 , runtime_types :: pallet_democracy :: vote_threshold :: VoteThreshold ,) ;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Blacklist(pub ::subxt::sp_core::H256);
            impl ::subxt::StorageEntry for Blacklist {
                const PALLET: &'static str = "Democracy";
                const STORAGE: &'static str = "Blacklist";
                type Value = (
                    ::core::primitive::u32,
                    ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                );
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Identity,
                        ),
                    ])
                }
            }
            pub struct Cancellations(pub ::subxt::sp_core::H256);
            impl ::subxt::StorageEntry for Cancellations {
                const PALLET: &'static str = "Democracy";
                const STORAGE: &'static str = "Cancellations";
                type Value = ::core::primitive::bool;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Identity,
                        ),
                    ])
                }
            }
            pub struct StorageVersion;
            impl ::subxt::StorageEntry for StorageVersion {
                const PALLET: &'static str = "Democracy";
                const STORAGE: &'static str = "StorageVersion";
                type Value = runtime_types::pallet_democracy::Releases;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn public_prop_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::u32,
                    ::subxt::Error,
                > {
                    let entry = PublicPropCount;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn public_props(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<(
                        ::core::primitive::u32,
                        ::subxt::sp_core::H256,
                        ::subxt::sp_core::crypto::AccountId32,
                    )>,
                    ::subxt::Error,
                > {
                    let entry = PublicProps;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn deposit_of(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<(
                        ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                        ::core::primitive::u128,
                    )>,
                    ::subxt::Error,
                > {
                    let entry = DepositOf(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn deposit_of_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, DepositOf>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn preimages(
                    &self,
                    _0: ::subxt::sp_core::H256,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_democracy::PreimageStatus<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                            ::core::primitive::u32,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Preimages(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn preimages_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Preimages>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn referendum_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::u32,
                    ::subxt::Error,
                > {
                    let entry = ReferendumCount;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn lowest_unbaked(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::u32,
                    ::subxt::Error,
                > {
                    let entry = LowestUnbaked;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn referendum_info_of(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_democracy::types::ReferendumInfo<
                            ::core::primitive::u32,
                            ::subxt::sp_core::H256,
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = ReferendumInfoOf(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn referendum_info_of_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ReferendumInfoOf>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn voting_of(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_democracy::vote::Voting<
                        ::core::primitive::u128,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    ::subxt::Error,
                > {
                    let entry = VotingOf(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn voting_of_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, VotingOf>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn locks(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::Error,
                > {
                    let entry = Locks(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn locks_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Locks>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn last_tabled_was_external(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::bool,
                    ::subxt::Error,
                > {
                    let entry = LastTabledWasExternal;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }                pub async fn next_external (& self , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < :: core :: option :: Option < (:: subxt :: sp_core :: H256 , runtime_types :: pallet_democracy :: vote_threshold :: VoteThreshold ,) > , :: subxt :: Error >{
                    let entry = NextExternal;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn blacklist(
                    &self,
                    _0: ::subxt::sp_core::H256,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<(
                        ::core::primitive::u32,
                        ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                    )>,
                    ::subxt::Error,
                > {
                    let entry = Blacklist(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn blacklist_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Blacklist>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn cancellations(
                    &self,
                    _0: ::subxt::sp_core::H256,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::bool,
                    ::subxt::Error,
                > {
                    let entry = Cancellations(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn cancellations_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Cancellations>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn storage_version(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_democracy::Releases,
                    >,
                    ::subxt::Error,
                > {
                    let entry = StorageVersion;
                    self.client.storage().fetch(&entry, hash).await
                }
            }
        }
    }
    pub mod council {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct SetMembers {
                pub new_members:
                    ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                pub prime: ::core::option::Option<
                    ::subxt::sp_core::crypto::AccountId32,
                >,
                pub old_count: ::core::primitive::u32,
            }
            impl ::subxt::Call for SetMembers {
                const PALLET: &'static str = "Council";
                const FUNCTION: &'static str = "set_members";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Execute {
                pub proposal: runtime_types::darkwebb_standalone_runtime::Call,
                #[codec(compact)]
                pub length_bound: ::core::primitive::u32,
            }
            impl ::subxt::Call for Execute {
                const PALLET: &'static str = "Council";
                const FUNCTION: &'static str = "execute";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Propose {
                #[codec(compact)]
                pub threshold: ::core::primitive::u32,
                pub proposal: runtime_types::darkwebb_standalone_runtime::Call,
                #[codec(compact)]
                pub length_bound: ::core::primitive::u32,
            }
            impl ::subxt::Call for Propose {
                const PALLET: &'static str = "Council";
                const FUNCTION: &'static str = "propose";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Vote {
                pub proposal: ::subxt::sp_core::H256,
                #[codec(compact)]
                pub index: ::core::primitive::u32,
                pub approve: ::core::primitive::bool,
            }
            impl ::subxt::Call for Vote {
                const PALLET: &'static str = "Council";
                const FUNCTION: &'static str = "vote";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Close {
                pub proposal_hash: ::subxt::sp_core::H256,
                #[codec(compact)]
                pub index: ::core::primitive::u32,
                #[codec(compact)]
                pub proposal_weight_bound: ::core::primitive::u64,
                #[codec(compact)]
                pub length_bound: ::core::primitive::u32,
            }
            impl ::subxt::Call for Close {
                const PALLET: &'static str = "Council";
                const FUNCTION: &'static str = "close";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct DisapproveProposal {
                pub proposal_hash: ::subxt::sp_core::H256,
            }
            impl ::subxt::Call for DisapproveProposal {
                const PALLET: &'static str = "Council";
                const FUNCTION: &'static str = "disapprove_proposal";
            }
            pub struct TransactionApi<
                'a,
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            > {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn set_members(
                    &self,
                    new_members: ::std::vec::Vec<
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                    prime: ::core::option::Option<
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                    old_count: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, SetMembers>
                {
                    let call = SetMembers {
                        new_members,
                        prime,
                        old_count,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn execute(
                    &self,
                    proposal: runtime_types::darkwebb_standalone_runtime::Call,
                    length_bound: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, Execute>
                {
                    let call = Execute {
                        proposal,
                        length_bound,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn propose(
                    &self,
                    threshold: ::core::primitive::u32,
                    proposal: runtime_types::darkwebb_standalone_runtime::Call,
                    length_bound: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, Propose>
                {
                    let call = Propose {
                        threshold,
                        proposal,
                        length_bound,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn vote(
                    &self,
                    proposal: ::subxt::sp_core::H256,
                    index: ::core::primitive::u32,
                    approve: ::core::primitive::bool,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, Vote>
                {
                    let call = Vote {
                        proposal,
                        index,
                        approve,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn close(
                    &self,
                    proposal_hash: ::subxt::sp_core::H256,
                    index: ::core::primitive::u32,
                    proposal_weight_bound: ::core::primitive::u64,
                    length_bound: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, Close>
                {
                    let call = Close {
                        proposal_hash,
                        index,
                        proposal_weight_bound,
                        length_bound,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn disapprove_proposal(
                    &self,
                    proposal_hash: ::subxt::sp_core::H256,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, DisapproveProposal>
                {
                    let call = DisapproveProposal { proposal_hash };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_collective::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Proposed(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::H256,
                pub ::core::primitive::u32,
            );
            impl ::subxt::Event for Proposed {
                const PALLET: &'static str = "Council";
                const EVENT: &'static str = "Proposed";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Voted(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::H256,
                pub ::core::primitive::bool,
                pub ::core::primitive::u32,
                pub ::core::primitive::u32,
            );
            impl ::subxt::Event for Voted {
                const PALLET: &'static str = "Council";
                const EVENT: &'static str = "Voted";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Approved(pub ::subxt::sp_core::H256);
            impl ::subxt::Event for Approved {
                const PALLET: &'static str = "Council";
                const EVENT: &'static str = "Approved";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Disapproved(pub ::subxt::sp_core::H256);
            impl ::subxt::Event for Disapproved {
                const PALLET: &'static str = "Council";
                const EVENT: &'static str = "Disapproved";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Executed(
                pub ::subxt::sp_core::H256,
                pub  ::core::result::Result<
                    (),
                    runtime_types::sp_runtime::DispatchError,
                >,
            );
            impl ::subxt::Event for Executed {
                const PALLET: &'static str = "Council";
                const EVENT: &'static str = "Executed";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct MemberExecuted(
                pub ::subxt::sp_core::H256,
                pub  ::core::result::Result<
                    (),
                    runtime_types::sp_runtime::DispatchError,
                >,
            );
            impl ::subxt::Event for MemberExecuted {
                const PALLET: &'static str = "Council";
                const EVENT: &'static str = "MemberExecuted";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Closed(
                pub ::subxt::sp_core::H256,
                pub ::core::primitive::u32,
                pub ::core::primitive::u32,
            );
            impl ::subxt::Event for Closed {
                const PALLET: &'static str = "Council";
                const EVENT: &'static str = "Closed";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Proposals;
            impl ::subxt::StorageEntry for Proposals {
                const PALLET: &'static str = "Council";
                const STORAGE: &'static str = "Proposals";
                type Value = runtime_types :: frame_support :: storage :: bounded_vec :: BoundedVec < :: subxt :: sp_core :: H256 > ;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ProposalOf(pub ::subxt::sp_core::H256);
            impl ::subxt::StorageEntry for ProposalOf {
                const PALLET: &'static str = "Council";
                const STORAGE: &'static str = "ProposalOf";
                type Value = runtime_types::darkwebb_standalone_runtime::Call;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Identity,
                        ),
                    ])
                }
            }
            pub struct Voting(pub ::subxt::sp_core::H256);
            impl ::subxt::StorageEntry for Voting {
                const PALLET: &'static str = "Council";
                const STORAGE: &'static str = "Voting";
                type Value = runtime_types::pallet_collective::Votes<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Identity,
                        ),
                    ])
                }
            }
            pub struct ProposalCount;
            impl ::subxt::StorageEntry for ProposalCount {
                const PALLET: &'static str = "Council";
                const STORAGE: &'static str = "ProposalCount";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Members;
            impl ::subxt::StorageEntry for Members {
                const PALLET: &'static str = "Council";
                const STORAGE: &'static str = "Members";
                type Value =
                    ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Prime;
            impl ::subxt::StorageEntry for Prime {
                const PALLET: &'static str = "Council";
                const STORAGE: &'static str = "Prime";
                type Value = ::subxt::sp_core::crypto::AccountId32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }                pub async fn proposals (& self , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < runtime_types :: frame_support :: storage :: bounded_vec :: BoundedVec < :: subxt :: sp_core :: H256 > , :: subxt :: Error >{
                    let entry = Proposals;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn proposal_of(
                    &self,
                    _0: ::subxt::sp_core::H256,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::darkwebb_standalone_runtime::Call,
                    >,
                    ::subxt::Error,
                > {
                    let entry = ProposalOf(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn proposal_of_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ProposalOf>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn voting(
                    &self,
                    _0: ::subxt::sp_core::H256,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_collective::Votes<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Voting(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn voting_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Voting>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn proposal_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::u32,
                    ::subxt::Error,
                > {
                    let entry = ProposalCount;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn members(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                    ::subxt::Error,
                > {
                    let entry = Members;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn prime(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Prime;
                    self.client.storage().fetch(&entry, hash).await
                }
            }
        }
    }
    pub mod elections {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Vote {
                pub votes:
                    ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                #[codec(compact)]
                pub value: ::core::primitive::u128,
            }
            impl ::subxt::Call for Vote {
                const PALLET: &'static str = "Elections";
                const FUNCTION: &'static str = "vote";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct RemoveVoter {}
            impl ::subxt::Call for RemoveVoter {
                const PALLET: &'static str = "Elections";
                const FUNCTION: &'static str = "remove_voter";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct SubmitCandidacy {
                #[codec(compact)]
                pub candidate_count: ::core::primitive::u32,
            }
            impl ::subxt::Call for SubmitCandidacy {
                const PALLET: &'static str = "Elections";
                const FUNCTION: &'static str = "submit_candidacy";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct RenounceCandidacy {
                pub renouncing:
                    runtime_types::pallet_elections_phragmen::Renouncing,
            }
            impl ::subxt::Call for RenounceCandidacy {
                const PALLET: &'static str = "Elections";
                const FUNCTION: &'static str = "renounce_candidacy";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct RemoveMember {
                pub who: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                pub has_replacement: ::core::primitive::bool,
            }
            impl ::subxt::Call for RemoveMember {
                const PALLET: &'static str = "Elections";
                const FUNCTION: &'static str = "remove_member";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct CleanDefunctVoters {
                pub num_voters: ::core::primitive::u32,
                pub num_defunct: ::core::primitive::u32,
            }
            impl ::subxt::Call for CleanDefunctVoters {
                const PALLET: &'static str = "Elections";
                const FUNCTION: &'static str = "clean_defunct_voters";
            }
            pub struct TransactionApi<
                'a,
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            > {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn vote(
                    &self,
                    votes: ::std::vec::Vec<
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                    value: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, Vote>
                {
                    let call = Vote { votes, value };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn remove_voter(
                    &self,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, RemoveVoter>
                {
                    let call = RemoveVoter {};
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn submit_candidacy(
                    &self,
                    candidate_count: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, SubmitCandidacy>
                {
                    let call = SubmitCandidacy { candidate_count };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn renounce_candidacy(
                    &self,
                    renouncing : runtime_types :: pallet_elections_phragmen :: Renouncing,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, RenounceCandidacy>
                {
                    let call = RenounceCandidacy { renouncing };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn remove_member(
                    &self,
                    who: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    has_replacement: ::core::primitive::bool,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, RemoveMember>
                {
                    let call = RemoveMember {
                        who,
                        has_replacement,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn clean_defunct_voters(
                    &self,
                    num_voters: ::core::primitive::u32,
                    num_defunct: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, CleanDefunctVoters>
                {
                    let call = CleanDefunctVoters {
                        num_voters,
                        num_defunct,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event =
            runtime_types::pallet_elections_phragmen::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct NewTerm(
                pub  ::std::vec::Vec<(
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u128,
                )>,
            );
            impl ::subxt::Event for NewTerm {
                const PALLET: &'static str = "Elections";
                const EVENT: &'static str = "NewTerm";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct EmptyTerm {}
            impl ::subxt::Event for EmptyTerm {
                const PALLET: &'static str = "Elections";
                const EVENT: &'static str = "EmptyTerm";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ElectionError {}
            impl ::subxt::Event for ElectionError {
                const PALLET: &'static str = "Elections";
                const EVENT: &'static str = "ElectionError";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct MemberKicked(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::Event for MemberKicked {
                const PALLET: &'static str = "Elections";
                const EVENT: &'static str = "MemberKicked";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Renounced(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::Event for Renounced {
                const PALLET: &'static str = "Elections";
                const EVENT: &'static str = "Renounced";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct CandidateSlashed(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for CandidateSlashed {
                const PALLET: &'static str = "Elections";
                const EVENT: &'static str = "CandidateSlashed";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct SeatHolderSlashed(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for SeatHolderSlashed {
                const PALLET: &'static str = "Elections";
                const EVENT: &'static str = "SeatHolderSlashed";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Members;
            impl ::subxt::StorageEntry for Members {
                const PALLET: &'static str = "Elections";
                const STORAGE: &'static str = "Members";
                type Value = ::std::vec::Vec<
                    runtime_types::pallet_elections_phragmen::SeatHolder<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    >,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct RunnersUp;
            impl ::subxt::StorageEntry for RunnersUp {
                const PALLET: &'static str = "Elections";
                const STORAGE: &'static str = "RunnersUp";
                type Value = ::std::vec::Vec<
                    runtime_types::pallet_elections_phragmen::SeatHolder<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    >,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Candidates;
            impl ::subxt::StorageEntry for Candidates {
                const PALLET: &'static str = "Elections";
                const STORAGE: &'static str = "Candidates";
                type Value = ::std::vec::Vec<(
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u128,
                )>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ElectionRounds;
            impl ::subxt::StorageEntry for ElectionRounds {
                const PALLET: &'static str = "Elections";
                const STORAGE: &'static str = "ElectionRounds";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Voting(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Voting {
                const PALLET: &'static str = "Elections";
                const STORAGE: &'static str = "Voting";
                type Value = runtime_types::pallet_elections_phragmen::Voter<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u128,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                    ])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn members(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<
                        runtime_types::pallet_elections_phragmen::SeatHolder<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Members;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn runners_up(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<
                        runtime_types::pallet_elections_phragmen::SeatHolder<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = RunnersUp;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn candidates(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    )>,
                    ::subxt::Error,
                > {
                    let entry = Candidates;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn election_rounds(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::u32,
                    ::subxt::Error,
                > {
                    let entry = ElectionRounds;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn voting(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_elections_phragmen::Voter<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Voting(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn voting_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Voting>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod grandpa {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ReportEquivocation {
                pub equivocation_proof:
                    runtime_types::sp_finality_grandpa::EquivocationProof<
                        ::subxt::sp_core::H256,
                        ::core::primitive::u32,
                    >,
                pub key_owner_proof: runtime_types::sp_session::MembershipProof,
            }
            impl ::subxt::Call for ReportEquivocation {
                const PALLET: &'static str = "Grandpa";
                const FUNCTION: &'static str = "report_equivocation";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ReportEquivocationUnsigned {
                pub equivocation_proof:
                    runtime_types::sp_finality_grandpa::EquivocationProof<
                        ::subxt::sp_core::H256,
                        ::core::primitive::u32,
                    >,
                pub key_owner_proof: runtime_types::sp_session::MembershipProof,
            }
            impl ::subxt::Call for ReportEquivocationUnsigned {
                const PALLET: &'static str = "Grandpa";
                const FUNCTION: &'static str = "report_equivocation_unsigned";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct NoteStalled {
                pub delay: ::core::primitive::u32,
                pub best_finalized_block_number: ::core::primitive::u32,
            }
            impl ::subxt::Call for NoteStalled {
                const PALLET: &'static str = "Grandpa";
                const FUNCTION: &'static str = "note_stalled";
            }
            pub struct TransactionApi<
                'a,
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            > {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn report_equivocation(
                    &self,
                    equivocation_proof : runtime_types :: sp_finality_grandpa :: EquivocationProof < :: subxt :: sp_core :: H256 , :: core :: primitive :: u32 >,
                    key_owner_proof: runtime_types::sp_session::MembershipProof,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, ReportEquivocation>
                {
                    let call = ReportEquivocation {
                        equivocation_proof,
                        key_owner_proof,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn report_equivocation_unsigned(
                    &self,
                    equivocation_proof : runtime_types :: sp_finality_grandpa :: EquivocationProof < :: subxt :: sp_core :: H256 , :: core :: primitive :: u32 >,
                    key_owner_proof: runtime_types::sp_session::MembershipProof,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    ReportEquivocationUnsigned,
                > {
                    let call = ReportEquivocationUnsigned {
                        equivocation_proof,
                        key_owner_proof,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn note_stalled(
                    &self,
                    delay: ::core::primitive::u32,
                    best_finalized_block_number: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, NoteStalled>
                {
                    let call = NoteStalled {
                        delay,
                        best_finalized_block_number,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_grandpa::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct NewAuthorities(
                pub  ::std::vec::Vec<(
                    runtime_types::sp_finality_grandpa::app::Public,
                    ::core::primitive::u64,
                )>,
            );
            impl ::subxt::Event for NewAuthorities {
                const PALLET: &'static str = "Grandpa";
                const EVENT: &'static str = "NewAuthorities";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Paused {}
            impl ::subxt::Event for Paused {
                const PALLET: &'static str = "Grandpa";
                const EVENT: &'static str = "Paused";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Resumed {}
            impl ::subxt::Event for Resumed {
                const PALLET: &'static str = "Grandpa";
                const EVENT: &'static str = "Resumed";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct State;
            impl ::subxt::StorageEntry for State {
                const PALLET: &'static str = "Grandpa";
                const STORAGE: &'static str = "State";
                type Value = runtime_types::pallet_grandpa::StoredState<
                    ::core::primitive::u32,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct PendingChange;
            impl ::subxt::StorageEntry for PendingChange {
                const PALLET: &'static str = "Grandpa";
                const STORAGE: &'static str = "PendingChange";
                type Value = runtime_types::pallet_grandpa::StoredPendingChange<
                    ::core::primitive::u32,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NextForced;
            impl ::subxt::StorageEntry for NextForced {
                const PALLET: &'static str = "Grandpa";
                const STORAGE: &'static str = "NextForced";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Stalled;
            impl ::subxt::StorageEntry for Stalled {
                const PALLET: &'static str = "Grandpa";
                const STORAGE: &'static str = "Stalled";
                type Value = (::core::primitive::u32, ::core::primitive::u32);
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct CurrentSetId;
            impl ::subxt::StorageEntry for CurrentSetId {
                const PALLET: &'static str = "Grandpa";
                const STORAGE: &'static str = "CurrentSetId";
                type Value = ::core::primitive::u64;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct SetIdSession(pub ::core::primitive::u64);
            impl ::subxt::StorageEntry for SetIdSession {
                const PALLET: &'static str = "Grandpa";
                const STORAGE: &'static str = "SetIdSession";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                    ])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn state(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_grandpa::StoredState<
                        ::core::primitive::u32,
                    >,
                    ::subxt::Error,
                > {
                    let entry = State;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn pending_change(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_grandpa::StoredPendingChange<
                            ::core::primitive::u32,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = PendingChange;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn next_forced(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::Error,
                > {
                    let entry = NextForced;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn stalled(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<(
                        ::core::primitive::u32,
                        ::core::primitive::u32,
                    )>,
                    ::subxt::Error,
                > {
                    let entry = Stalled;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn current_set_id(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::u64,
                    ::subxt::Error,
                > {
                    let entry = CurrentSetId;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn set_id_session(
                    &self,
                    _0: ::core::primitive::u64,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::Error,
                > {
                    let entry = SetIdSession(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn set_id_session_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, SetIdSession>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod treasury {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ProposeSpend {
                #[codec(compact)]
                pub value: ::core::primitive::u128,
                pub beneficiary: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
            }
            impl ::subxt::Call for ProposeSpend {
                const PALLET: &'static str = "Treasury";
                const FUNCTION: &'static str = "propose_spend";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct RejectProposal {
                #[codec(compact)]
                pub proposal_id: ::core::primitive::u32,
            }
            impl ::subxt::Call for RejectProposal {
                const PALLET: &'static str = "Treasury";
                const FUNCTION: &'static str = "reject_proposal";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ApproveProposal {
                #[codec(compact)]
                pub proposal_id: ::core::primitive::u32,
            }
            impl ::subxt::Call for ApproveProposal {
                const PALLET: &'static str = "Treasury";
                const FUNCTION: &'static str = "approve_proposal";
            }
            pub struct TransactionApi<
                'a,
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            > {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn propose_spend(
                    &self,
                    value: ::core::primitive::u128,
                    beneficiary: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, ProposeSpend>
                {
                    let call = ProposeSpend { value, beneficiary };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn reject_proposal(
                    &self,
                    proposal_id: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, RejectProposal>
                {
                    let call = RejectProposal { proposal_id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn approve_proposal(
                    &self,
                    proposal_id: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, ApproveProposal>
                {
                    let call = ApproveProposal { proposal_id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_treasury::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Proposed(pub ::core::primitive::u32);
            impl ::subxt::Event for Proposed {
                const PALLET: &'static str = "Treasury";
                const EVENT: &'static str = "Proposed";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Spending(pub ::core::primitive::u128);
            impl ::subxt::Event for Spending {
                const PALLET: &'static str = "Treasury";
                const EVENT: &'static str = "Spending";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Awarded(
                pub ::core::primitive::u32,
                pub ::core::primitive::u128,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for Awarded {
                const PALLET: &'static str = "Treasury";
                const EVENT: &'static str = "Awarded";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Rejected(
                pub ::core::primitive::u32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Rejected {
                const PALLET: &'static str = "Treasury";
                const EVENT: &'static str = "Rejected";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Burnt(pub ::core::primitive::u128);
            impl ::subxt::Event for Burnt {
                const PALLET: &'static str = "Treasury";
                const EVENT: &'static str = "Burnt";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Rollover(pub ::core::primitive::u128);
            impl ::subxt::Event for Rollover {
                const PALLET: &'static str = "Treasury";
                const EVENT: &'static str = "Rollover";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Deposit(pub ::core::primitive::u128);
            impl ::subxt::Event for Deposit {
                const PALLET: &'static str = "Treasury";
                const EVENT: &'static str = "Deposit";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct ProposalCount;
            impl ::subxt::StorageEntry for ProposalCount {
                const PALLET: &'static str = "Treasury";
                const STORAGE: &'static str = "ProposalCount";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Proposals(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for Proposals {
                const PALLET: &'static str = "Treasury";
                const STORAGE: &'static str = "Proposals";
                type Value = runtime_types::pallet_treasury::Proposal<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u128,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                    ])
                }
            }
            pub struct Approvals;
            impl ::subxt::StorageEntry for Approvals {
                const PALLET: &'static str = "Treasury";
                const STORAGE: &'static str = "Approvals";
                type Value = runtime_types :: frame_support :: storage :: bounded_vec :: BoundedVec < :: core :: primitive :: u32 > ;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn proposal_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::u32,
                    ::subxt::Error,
                > {
                    let entry = ProposalCount;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn proposals(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_treasury::Proposal<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Proposals(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn proposals_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Proposals>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }                pub async fn approvals (& self , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < runtime_types :: frame_support :: storage :: bounded_vec :: BoundedVec < :: core :: primitive :: u32 > , :: subxt :: Error >{
                    let entry = Approvals;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod utility {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Batch {
                pub calls: ::std::vec::Vec<
                    runtime_types::darkwebb_standalone_runtime::Call,
                >,
            }
            impl ::subxt::Call for Batch {
                const PALLET: &'static str = "Utility";
                const FUNCTION: &'static str = "batch";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct AsDerivative {
                pub index: ::core::primitive::u16,
                pub call: runtime_types::darkwebb_standalone_runtime::Call,
            }
            impl ::subxt::Call for AsDerivative {
                const PALLET: &'static str = "Utility";
                const FUNCTION: &'static str = "as_derivative";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct BatchAll {
                pub calls: ::std::vec::Vec<
                    runtime_types::darkwebb_standalone_runtime::Call,
                >,
            }
            impl ::subxt::Call for BatchAll {
                const PALLET: &'static str = "Utility";
                const FUNCTION: &'static str = "batch_all";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct DispatchAs {
                pub as_origin:
                    runtime_types::darkwebb_standalone_runtime::OriginCaller,
                pub call: runtime_types::darkwebb_standalone_runtime::Call,
            }
            impl ::subxt::Call for DispatchAs {
                const PALLET: &'static str = "Utility";
                const FUNCTION: &'static str = "dispatch_as";
            }
            pub struct TransactionApi<
                'a,
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            > {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn batch(
                    &self,
                    calls: ::std::vec::Vec<
                        runtime_types::darkwebb_standalone_runtime::Call,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, Batch>
                {
                    let call = Batch { calls };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn as_derivative(
                    &self,
                    index: ::core::primitive::u16,
                    call: runtime_types::darkwebb_standalone_runtime::Call,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, AsDerivative>
                {
                    let call = AsDerivative { index, call };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn batch_all(
                    &self,
                    calls: ::std::vec::Vec<
                        runtime_types::darkwebb_standalone_runtime::Call,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, BatchAll>
                {
                    let call = BatchAll { calls };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn dispatch_as(
                    &self,
                    as_origin : runtime_types :: darkwebb_standalone_runtime :: OriginCaller,
                    call: runtime_types::darkwebb_standalone_runtime::Call,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, DispatchAs>
                {
                    let call = DispatchAs { as_origin, call };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_utility::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct BatchInterrupted(
                pub ::core::primitive::u32,
                pub runtime_types::sp_runtime::DispatchError,
            );
            impl ::subxt::Event for BatchInterrupted {
                const PALLET: &'static str = "Utility";
                const EVENT: &'static str = "BatchInterrupted";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct BatchCompleted {}
            impl ::subxt::Event for BatchCompleted {
                const PALLET: &'static str = "Utility";
                const EVENT: &'static str = "BatchCompleted";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ItemCompleted {}
            impl ::subxt::Event for ItemCompleted {
                const PALLET: &'static str = "Utility";
                const EVENT: &'static str = "ItemCompleted";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct DispatchedAs(
                pub  ::core::result::Result<
                    (),
                    runtime_types::sp_runtime::DispatchError,
                >,
            );
            impl ::subxt::Event for DispatchedAs {
                const PALLET: &'static str = "Utility";
                const EVENT: &'static str = "DispatchedAs";
            }
        }
    }
    pub mod multisig {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct AsMultiThreshold1 {
                pub other_signatories:
                    ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                pub call: runtime_types::darkwebb_standalone_runtime::Call,
            }
            impl ::subxt::Call for AsMultiThreshold1 {
                const PALLET: &'static str = "Multisig";
                const FUNCTION: &'static str = "as_multi_threshold1";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct AsMulti {
                pub threshold: ::core::primitive::u16,
                pub other_signatories:
                    ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                pub maybe_timepoint: ::core::option::Option<
                    runtime_types::pallet_multisig::Timepoint<
                        ::core::primitive::u32,
                    >,
                >,
                pub call: ::subxt::WrapperKeepOpaque<
                    runtime_types::darkwebb_standalone_runtime::Call,
                >,
                pub store_call: ::core::primitive::bool,
                pub max_weight: ::core::primitive::u64,
            }
            impl ::subxt::Call for AsMulti {
                const PALLET: &'static str = "Multisig";
                const FUNCTION: &'static str = "as_multi";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ApproveAsMulti {
                pub threshold: ::core::primitive::u16,
                pub other_signatories:
                    ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                pub maybe_timepoint: ::core::option::Option<
                    runtime_types::pallet_multisig::Timepoint<
                        ::core::primitive::u32,
                    >,
                >,
                pub call_hash: [::core::primitive::u8; 32usize],
                pub max_weight: ::core::primitive::u64,
            }
            impl ::subxt::Call for ApproveAsMulti {
                const PALLET: &'static str = "Multisig";
                const FUNCTION: &'static str = "approve_as_multi";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct CancelAsMulti {
                pub threshold: ::core::primitive::u16,
                pub other_signatories:
                    ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                pub timepoint: runtime_types::pallet_multisig::Timepoint<
                    ::core::primitive::u32,
                >,
                pub call_hash: [::core::primitive::u8; 32usize],
            }
            impl ::subxt::Call for CancelAsMulti {
                const PALLET: &'static str = "Multisig";
                const FUNCTION: &'static str = "cancel_as_multi";
            }
            pub struct TransactionApi<
                'a,
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            > {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn as_multi_threshold1(
                    &self,
                    other_signatories: ::std::vec::Vec<
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                    call: runtime_types::darkwebb_standalone_runtime::Call,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, AsMultiThreshold1>
                {
                    let call = AsMultiThreshold1 {
                        other_signatories,
                        call,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn as_multi(
                    &self,
                    threshold: ::core::primitive::u16,
                    other_signatories: ::std::vec::Vec<
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                    maybe_timepoint: ::core::option::Option<
                        runtime_types::pallet_multisig::Timepoint<
                            ::core::primitive::u32,
                        >,
                    >,
                    call: ::subxt::WrapperKeepOpaque<
                        runtime_types::darkwebb_standalone_runtime::Call,
                    >,
                    store_call: ::core::primitive::bool,
                    max_weight: ::core::primitive::u64,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, AsMulti>
                {
                    let call = AsMulti {
                        threshold,
                        other_signatories,
                        maybe_timepoint,
                        call,
                        store_call,
                        max_weight,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn approve_as_multi(
                    &self,
                    threshold: ::core::primitive::u16,
                    other_signatories: ::std::vec::Vec<
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                    maybe_timepoint: ::core::option::Option<
                        runtime_types::pallet_multisig::Timepoint<
                            ::core::primitive::u32,
                        >,
                    >,
                    call_hash: [::core::primitive::u8; 32usize],
                    max_weight: ::core::primitive::u64,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, ApproveAsMulti>
                {
                    let call = ApproveAsMulti {
                        threshold,
                        other_signatories,
                        maybe_timepoint,
                        call_hash,
                        max_weight,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn cancel_as_multi(
                    &self,
                    threshold: ::core::primitive::u16,
                    other_signatories: ::std::vec::Vec<
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                    timepoint: runtime_types::pallet_multisig::Timepoint<
                        ::core::primitive::u32,
                    >,
                    call_hash: [::core::primitive::u8; 32usize],
                ) -> ::subxt::SubmittableExtrinsic<'a, T, CancelAsMulti>
                {
                    let call = CancelAsMulti {
                        threshold,
                        other_signatories,
                        timepoint,
                        call_hash,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_multisig::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct NewMultisig(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub [::core::primitive::u8; 32usize],
            );
            impl ::subxt::Event for NewMultisig {
                const PALLET: &'static str = "Multisig";
                const EVENT: &'static str = "NewMultisig";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct MultisigApproval(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub  runtime_types::pallet_multisig::Timepoint<
                    ::core::primitive::u32,
                >,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub [::core::primitive::u8; 32usize],
            );
            impl ::subxt::Event for MultisigApproval {
                const PALLET: &'static str = "Multisig";
                const EVENT: &'static str = "MultisigApproval";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct MultisigExecuted(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub  runtime_types::pallet_multisig::Timepoint<
                    ::core::primitive::u32,
                >,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub [::core::primitive::u8; 32usize],
                pub  ::core::result::Result<
                    (),
                    runtime_types::sp_runtime::DispatchError,
                >,
            );
            impl ::subxt::Event for MultisigExecuted {
                const PALLET: &'static str = "Multisig";
                const EVENT: &'static str = "MultisigExecuted";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct MultisigCancelled(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub  runtime_types::pallet_multisig::Timepoint<
                    ::core::primitive::u32,
                >,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub [::core::primitive::u8; 32usize],
            );
            impl ::subxt::Event for MultisigCancelled {
                const PALLET: &'static str = "Multisig";
                const EVENT: &'static str = "MultisigCancelled";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Multisigs(
                ::subxt::sp_core::crypto::AccountId32,
                [::core::primitive::u8; 32usize],
            );
            impl ::subxt::StorageEntry for Multisigs {
                const PALLET: &'static str = "Multisig";
                const STORAGE: &'static str = "Multisigs";
                type Value = runtime_types::pallet_multisig::Multisig<
                    ::core::primitive::u32,
                    ::core::primitive::u128,
                    ::subxt::sp_core::crypto::AccountId32,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct Calls(pub [::core::primitive::u8; 32usize]);
            impl ::subxt::StorageEntry for Calls {
                const PALLET: &'static str = "Multisig";
                const STORAGE: &'static str = "Calls";
                type Value = (
                    ::subxt::WrapperKeepOpaque<
                        runtime_types::darkwebb_standalone_runtime::Call,
                    >,
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u128,
                );
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Identity,
                        ),
                    ])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn multisigs(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    _1: [::core::primitive::u8; 32usize],
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_multisig::Multisig<
                            ::core::primitive::u32,
                            ::core::primitive::u128,
                            ::subxt::sp_core::crypto::AccountId32,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Multisigs(_0, _1);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn multisigs_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Multisigs>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn calls(
                    &self,
                    _0: [::core::primitive::u8; 32usize],
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<(
                        ::subxt::WrapperKeepOpaque<
                            runtime_types::darkwebb_standalone_runtime::Call,
                        >,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    )>,
                    ::subxt::Error,
                > {
                    let entry = Calls(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn calls_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Calls>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod scheduler {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Schedule {
                pub when: ::core::primitive::u32,
                pub maybe_periodic: ::core::option::Option<(
                    ::core::primitive::u32,
                    ::core::primitive::u32,
                )>,
                pub priority: ::core::primitive::u8,
                pub call: runtime_types::darkwebb_standalone_runtime::Call,
            }
            impl ::subxt::Call for Schedule {
                const PALLET: &'static str = "Scheduler";
                const FUNCTION: &'static str = "schedule";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Cancel {
                pub when: ::core::primitive::u32,
                pub index: ::core::primitive::u32,
            }
            impl ::subxt::Call for Cancel {
                const PALLET: &'static str = "Scheduler";
                const FUNCTION: &'static str = "cancel";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ScheduleNamed {
                pub id: ::std::vec::Vec<::core::primitive::u8>,
                pub when: ::core::primitive::u32,
                pub maybe_periodic: ::core::option::Option<(
                    ::core::primitive::u32,
                    ::core::primitive::u32,
                )>,
                pub priority: ::core::primitive::u8,
                pub call: runtime_types::darkwebb_standalone_runtime::Call,
            }
            impl ::subxt::Call for ScheduleNamed {
                const PALLET: &'static str = "Scheduler";
                const FUNCTION: &'static str = "schedule_named";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct CancelNamed {
                pub id: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for CancelNamed {
                const PALLET: &'static str = "Scheduler";
                const FUNCTION: &'static str = "cancel_named";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ScheduleAfter {
                pub after: ::core::primitive::u32,
                pub maybe_periodic: ::core::option::Option<(
                    ::core::primitive::u32,
                    ::core::primitive::u32,
                )>,
                pub priority: ::core::primitive::u8,
                pub call: runtime_types::darkwebb_standalone_runtime::Call,
            }
            impl ::subxt::Call for ScheduleAfter {
                const PALLET: &'static str = "Scheduler";
                const FUNCTION: &'static str = "schedule_after";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ScheduleNamedAfter {
                pub id: ::std::vec::Vec<::core::primitive::u8>,
                pub after: ::core::primitive::u32,
                pub maybe_periodic: ::core::option::Option<(
                    ::core::primitive::u32,
                    ::core::primitive::u32,
                )>,
                pub priority: ::core::primitive::u8,
                pub call: runtime_types::darkwebb_standalone_runtime::Call,
            }
            impl ::subxt::Call for ScheduleNamedAfter {
                const PALLET: &'static str = "Scheduler";
                const FUNCTION: &'static str = "schedule_named_after";
            }
            pub struct TransactionApi<
                'a,
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            > {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn schedule(
                    &self,
                    when: ::core::primitive::u32,
                    maybe_periodic: ::core::option::Option<(
                        ::core::primitive::u32,
                        ::core::primitive::u32,
                    )>,
                    priority: ::core::primitive::u8,
                    call: runtime_types::darkwebb_standalone_runtime::Call,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, Schedule>
                {
                    let call = Schedule {
                        when,
                        maybe_periodic,
                        priority,
                        call,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn cancel(
                    &self,
                    when: ::core::primitive::u32,
                    index: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, Cancel>
                {
                    let call = Cancel { when, index };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn schedule_named(
                    &self,
                    id: ::std::vec::Vec<::core::primitive::u8>,
                    when: ::core::primitive::u32,
                    maybe_periodic: ::core::option::Option<(
                        ::core::primitive::u32,
                        ::core::primitive::u32,
                    )>,
                    priority: ::core::primitive::u8,
                    call: runtime_types::darkwebb_standalone_runtime::Call,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, ScheduleNamed>
                {
                    let call = ScheduleNamed {
                        id,
                        when,
                        maybe_periodic,
                        priority,
                        call,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn cancel_named(
                    &self,
                    id: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, CancelNamed>
                {
                    let call = CancelNamed { id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn schedule_after(
                    &self,
                    after: ::core::primitive::u32,
                    maybe_periodic: ::core::option::Option<(
                        ::core::primitive::u32,
                        ::core::primitive::u32,
                    )>,
                    priority: ::core::primitive::u8,
                    call: runtime_types::darkwebb_standalone_runtime::Call,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, ScheduleAfter>
                {
                    let call = ScheduleAfter {
                        after,
                        maybe_periodic,
                        priority,
                        call,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn schedule_named_after(
                    &self,
                    id: ::std::vec::Vec<::core::primitive::u8>,
                    after: ::core::primitive::u32,
                    maybe_periodic: ::core::option::Option<(
                        ::core::primitive::u32,
                        ::core::primitive::u32,
                    )>,
                    priority: ::core::primitive::u8,
                    call: runtime_types::darkwebb_standalone_runtime::Call,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, ScheduleNamedAfter>
                {
                    let call = ScheduleNamedAfter {
                        id,
                        after,
                        maybe_periodic,
                        priority,
                        call,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_scheduler::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Scheduled(
                pub ::core::primitive::u32,
                pub ::core::primitive::u32,
            );
            impl ::subxt::Event for Scheduled {
                const PALLET: &'static str = "Scheduler";
                const EVENT: &'static str = "Scheduled";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Canceled(
                pub ::core::primitive::u32,
                pub ::core::primitive::u32,
            );
            impl ::subxt::Event for Canceled {
                const PALLET: &'static str = "Scheduler";
                const EVENT: &'static str = "Canceled";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Dispatched(
                pub (::core::primitive::u32, ::core::primitive::u32),
                pub  ::core::option::Option<
                    ::std::vec::Vec<::core::primitive::u8>,
                >,
                pub  ::core::result::Result<
                    (),
                    runtime_types::sp_runtime::DispatchError,
                >,
            );
            impl ::subxt::Event for Dispatched {
                const PALLET: &'static str = "Scheduler";
                const EVENT: &'static str = "Dispatched";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Agenda(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for Agenda {
                const PALLET: &'static str = "Scheduler";
                const STORAGE: &'static str = "Agenda";
                type Value = :: std :: vec :: Vec < :: core :: option :: Option < runtime_types :: pallet_scheduler :: ScheduledV2 < runtime_types :: darkwebb_standalone_runtime :: Call , :: core :: primitive :: u32 , runtime_types :: darkwebb_standalone_runtime :: OriginCaller , :: subxt :: sp_core :: crypto :: AccountId32 > > > ;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                    ])
                }
            }
            pub struct Lookup(pub ::std::vec::Vec<::core::primitive::u8>);
            impl ::subxt::StorageEntry for Lookup {
                const PALLET: &'static str = "Scheduler";
                const STORAGE: &'static str = "Lookup";
                type Value = (::core::primitive::u32, ::core::primitive::u32);
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                    ])
                }
            }
            pub struct StorageVersion;
            impl ::subxt::StorageEntry for StorageVersion {
                const PALLET: &'static str = "Scheduler";
                const STORAGE: &'static str = "StorageVersion";
                type Value = runtime_types::pallet_scheduler::Releases;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }                pub async fn agenda (& self , _0 : :: core :: primitive :: u32 , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < :: std :: vec :: Vec < :: core :: option :: Option < runtime_types :: pallet_scheduler :: ScheduledV2 < runtime_types :: darkwebb_standalone_runtime :: Call , :: core :: primitive :: u32 , runtime_types :: darkwebb_standalone_runtime :: OriginCaller , :: subxt :: sp_core :: crypto :: AccountId32 > > > , :: subxt :: Error >{
                    let entry = Agenda(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn agenda_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Agenda>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn lookup(
                    &self,
                    _0: ::std::vec::Vec<::core::primitive::u8>,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<(
                        ::core::primitive::u32,
                        ::core::primitive::u32,
                    )>,
                    ::subxt::Error,
                > {
                    let entry = Lookup(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn lookup_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Lookup>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn storage_version(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_scheduler::Releases,
                    ::subxt::Error,
                > {
                    let entry = StorageVersion;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod proxy {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Proxy {
                pub real: ::subxt::sp_core::crypto::AccountId32,
                pub force_proxy_type: ::core::option::Option<
                    runtime_types::darkwebb_standalone_runtime::ProxyType,
                >,
                pub call: runtime_types::darkwebb_standalone_runtime::Call,
            }
            impl ::subxt::Call for Proxy {
                const PALLET: &'static str = "Proxy";
                const FUNCTION: &'static str = "proxy";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct AddProxy {
                pub delegate: ::subxt::sp_core::crypto::AccountId32,
                pub proxy_type:
                    runtime_types::darkwebb_standalone_runtime::ProxyType,
                pub delay: ::core::primitive::u32,
            }
            impl ::subxt::Call for AddProxy {
                const PALLET: &'static str = "Proxy";
                const FUNCTION: &'static str = "add_proxy";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct RemoveProxy {
                pub delegate: ::subxt::sp_core::crypto::AccountId32,
                pub proxy_type:
                    runtime_types::darkwebb_standalone_runtime::ProxyType,
                pub delay: ::core::primitive::u32,
            }
            impl ::subxt::Call for RemoveProxy {
                const PALLET: &'static str = "Proxy";
                const FUNCTION: &'static str = "remove_proxy";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct RemoveProxies {}
            impl ::subxt::Call for RemoveProxies {
                const PALLET: &'static str = "Proxy";
                const FUNCTION: &'static str = "remove_proxies";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Anonymous {
                pub proxy_type:
                    runtime_types::darkwebb_standalone_runtime::ProxyType,
                pub delay: ::core::primitive::u32,
                pub index: ::core::primitive::u16,
            }
            impl ::subxt::Call for Anonymous {
                const PALLET: &'static str = "Proxy";
                const FUNCTION: &'static str = "anonymous";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct KillAnonymous {
                pub spawner: ::subxt::sp_core::crypto::AccountId32,
                pub proxy_type:
                    runtime_types::darkwebb_standalone_runtime::ProxyType,
                pub index: ::core::primitive::u16,
                #[codec(compact)]
                pub height: ::core::primitive::u32,
                #[codec(compact)]
                pub ext_index: ::core::primitive::u32,
            }
            impl ::subxt::Call for KillAnonymous {
                const PALLET: &'static str = "Proxy";
                const FUNCTION: &'static str = "kill_anonymous";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Announce {
                pub real: ::subxt::sp_core::crypto::AccountId32,
                pub call_hash: ::subxt::sp_core::H256,
            }
            impl ::subxt::Call for Announce {
                const PALLET: &'static str = "Proxy";
                const FUNCTION: &'static str = "announce";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct RemoveAnnouncement {
                pub real: ::subxt::sp_core::crypto::AccountId32,
                pub call_hash: ::subxt::sp_core::H256,
            }
            impl ::subxt::Call for RemoveAnnouncement {
                const PALLET: &'static str = "Proxy";
                const FUNCTION: &'static str = "remove_announcement";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct RejectAnnouncement {
                pub delegate: ::subxt::sp_core::crypto::AccountId32,
                pub call_hash: ::subxt::sp_core::H256,
            }
            impl ::subxt::Call for RejectAnnouncement {
                const PALLET: &'static str = "Proxy";
                const FUNCTION: &'static str = "reject_announcement";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ProxyAnnounced {
                pub delegate: ::subxt::sp_core::crypto::AccountId32,
                pub real: ::subxt::sp_core::crypto::AccountId32,
                pub force_proxy_type: ::core::option::Option<
                    runtime_types::darkwebb_standalone_runtime::ProxyType,
                >,
                pub call: runtime_types::darkwebb_standalone_runtime::Call,
            }
            impl ::subxt::Call for ProxyAnnounced {
                const PALLET: &'static str = "Proxy";
                const FUNCTION: &'static str = "proxy_announced";
            }
            pub struct TransactionApi<
                'a,
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            > {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn proxy(
                    &self,
                    real: ::subxt::sp_core::crypto::AccountId32,
                    force_proxy_type: ::core::option::Option<
                        runtime_types::darkwebb_standalone_runtime::ProxyType,
                    >,
                    call: runtime_types::darkwebb_standalone_runtime::Call,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, Proxy>
                {
                    let call = Proxy {
                        real,
                        force_proxy_type,
                        call,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn add_proxy(
                    &self,
                    delegate: ::subxt::sp_core::crypto::AccountId32,
                    proxy_type : runtime_types :: darkwebb_standalone_runtime :: ProxyType,
                    delay: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, AddProxy>
                {
                    let call = AddProxy {
                        delegate,
                        proxy_type,
                        delay,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn remove_proxy(
                    &self,
                    delegate: ::subxt::sp_core::crypto::AccountId32,
                    proxy_type : runtime_types :: darkwebb_standalone_runtime :: ProxyType,
                    delay: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, RemoveProxy>
                {
                    let call = RemoveProxy {
                        delegate,
                        proxy_type,
                        delay,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn remove_proxies(
                    &self,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, RemoveProxies>
                {
                    let call = RemoveProxies {};
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn anonymous(
                    &self,
                    proxy_type : runtime_types :: darkwebb_standalone_runtime :: ProxyType,
                    delay: ::core::primitive::u32,
                    index: ::core::primitive::u16,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, Anonymous>
                {
                    let call = Anonymous {
                        proxy_type,
                        delay,
                        index,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn kill_anonymous(
                    &self,
                    spawner: ::subxt::sp_core::crypto::AccountId32,
                    proxy_type : runtime_types :: darkwebb_standalone_runtime :: ProxyType,
                    index: ::core::primitive::u16,
                    height: ::core::primitive::u32,
                    ext_index: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, KillAnonymous>
                {
                    let call = KillAnonymous {
                        spawner,
                        proxy_type,
                        index,
                        height,
                        ext_index,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn announce(
                    &self,
                    real: ::subxt::sp_core::crypto::AccountId32,
                    call_hash: ::subxt::sp_core::H256,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, Announce>
                {
                    let call = Announce { real, call_hash };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn remove_announcement(
                    &self,
                    real: ::subxt::sp_core::crypto::AccountId32,
                    call_hash: ::subxt::sp_core::H256,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, RemoveAnnouncement>
                {
                    let call = RemoveAnnouncement { real, call_hash };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn reject_announcement(
                    &self,
                    delegate: ::subxt::sp_core::crypto::AccountId32,
                    call_hash: ::subxt::sp_core::H256,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, RejectAnnouncement>
                {
                    let call = RejectAnnouncement {
                        delegate,
                        call_hash,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn proxy_announced(
                    &self,
                    delegate: ::subxt::sp_core::crypto::AccountId32,
                    real: ::subxt::sp_core::crypto::AccountId32,
                    force_proxy_type: ::core::option::Option<
                        runtime_types::darkwebb_standalone_runtime::ProxyType,
                    >,
                    call: runtime_types::darkwebb_standalone_runtime::Call,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, ProxyAnnounced>
                {
                    let call = ProxyAnnounced {
                        delegate,
                        real,
                        force_proxy_type,
                        call,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_proxy::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ProxyExecuted(
                pub  ::core::result::Result<
                    (),
                    runtime_types::sp_runtime::DispatchError,
                >,
            );
            impl ::subxt::Event for ProxyExecuted {
                const PALLET: &'static str = "Proxy";
                const EVENT: &'static str = "ProxyExecuted";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct AnonymousCreated(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub runtime_types::darkwebb_standalone_runtime::ProxyType,
                pub ::core::primitive::u16,
            );
            impl ::subxt::Event for AnonymousCreated {
                const PALLET: &'static str = "Proxy";
                const EVENT: &'static str = "AnonymousCreated";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Announced(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::H256,
            );
            impl ::subxt::Event for Announced {
                const PALLET: &'static str = "Proxy";
                const EVENT: &'static str = "Announced";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ProxyAdded(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub runtime_types::darkwebb_standalone_runtime::ProxyType,
                pub ::core::primitive::u32,
            );
            impl ::subxt::Event for ProxyAdded {
                const PALLET: &'static str = "Proxy";
                const EVENT: &'static str = "ProxyAdded";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Proxies(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Proxies {
                const PALLET: &'static str = "Proxy";
                const STORAGE: &'static str = "Proxies";
                type Value = (runtime_types :: frame_support :: storage :: bounded_vec :: BoundedVec < runtime_types :: pallet_proxy :: ProxyDefinition < :: subxt :: sp_core :: crypto :: AccountId32 , runtime_types :: darkwebb_standalone_runtime :: ProxyType , :: core :: primitive :: u32 > > , :: core :: primitive :: u128 ,) ;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                    ])
                }
            }
            pub struct Announcements(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Announcements {
                const PALLET: &'static str = "Proxy";
                const STORAGE: &'static str = "Announcements";
                type Value = (runtime_types :: frame_support :: storage :: bounded_vec :: BoundedVec < runtime_types :: pallet_proxy :: Announcement < :: subxt :: sp_core :: crypto :: AccountId32 , :: subxt :: sp_core :: H256 , :: core :: primitive :: u32 > > , :: core :: primitive :: u128 ,) ;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                    ])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }                pub async fn proxies (& self , _0 : :: subxt :: sp_core :: crypto :: AccountId32 , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < (runtime_types :: frame_support :: storage :: bounded_vec :: BoundedVec < runtime_types :: pallet_proxy :: ProxyDefinition < :: subxt :: sp_core :: crypto :: AccountId32 , runtime_types :: darkwebb_standalone_runtime :: ProxyType , :: core :: primitive :: u32 > > , :: core :: primitive :: u128 ,) , :: subxt :: Error >{
                    let entry = Proxies(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn proxies_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Proxies>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }                pub async fn announcements (& self , _0 : :: subxt :: sp_core :: crypto :: AccountId32 , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < (runtime_types :: frame_support :: storage :: bounded_vec :: BoundedVec < runtime_types :: pallet_proxy :: Announcement < :: subxt :: sp_core :: crypto :: AccountId32 , :: subxt :: sp_core :: H256 , :: core :: primitive :: u32 > > , :: core :: primitive :: u128 ,) , :: subxt :: Error >{
                    let entry = Announcements(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn announcements_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Announcements>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod assets {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Create {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub admin: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                pub min_balance: ::core::primitive::u128,
            }
            impl ::subxt::Call for Create {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "create";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ForceCreate {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub owner: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                pub is_sufficient: ::core::primitive::bool,
                #[codec(compact)]
                pub min_balance: ::core::primitive::u128,
            }
            impl ::subxt::Call for ForceCreate {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "force_create";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Destroy {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub witness:
                    runtime_types::pallet_assets::types::DestroyWitness,
            }
            impl ::subxt::Call for Destroy {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "destroy";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Mint {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub beneficiary: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                #[codec(compact)]
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Call for Mint {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "mint";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Burn {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub who: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                #[codec(compact)]
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Call for Burn {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "burn";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Transfer {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub target: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                #[codec(compact)]
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Call for Transfer {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "transfer";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct TransferKeepAlive {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub target: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                #[codec(compact)]
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Call for TransferKeepAlive {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "transfer_keep_alive";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ForceTransfer {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub source: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                pub dest: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                #[codec(compact)]
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Call for ForceTransfer {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "force_transfer";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Freeze {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub who: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
            }
            impl ::subxt::Call for Freeze {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "freeze";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Thaw {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub who: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
            }
            impl ::subxt::Call for Thaw {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "thaw";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct FreezeAsset {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
            }
            impl ::subxt::Call for FreezeAsset {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "freeze_asset";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ThawAsset {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
            }
            impl ::subxt::Call for ThawAsset {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "thaw_asset";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct TransferOwnership {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub owner: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
            }
            impl ::subxt::Call for TransferOwnership {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "transfer_ownership";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct SetTeam {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub issuer: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                pub admin: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                pub freezer: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
            }
            impl ::subxt::Call for SetTeam {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "set_team";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct SetMetadata {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub name: ::std::vec::Vec<::core::primitive::u8>,
                pub symbol: ::std::vec::Vec<::core::primitive::u8>,
                pub decimals: ::core::primitive::u8,
            }
            impl ::subxt::Call for SetMetadata {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "set_metadata";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ClearMetadata {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
            }
            impl ::subxt::Call for ClearMetadata {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "clear_metadata";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ForceSetMetadata {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub name: ::std::vec::Vec<::core::primitive::u8>,
                pub symbol: ::std::vec::Vec<::core::primitive::u8>,
                pub decimals: ::core::primitive::u8,
                pub is_frozen: ::core::primitive::bool,
            }
            impl ::subxt::Call for ForceSetMetadata {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "force_set_metadata";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ForceClearMetadata {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
            }
            impl ::subxt::Call for ForceClearMetadata {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "force_clear_metadata";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ForceAssetStatus {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub owner: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                pub issuer: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                pub admin: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                pub freezer: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                #[codec(compact)]
                pub min_balance: ::core::primitive::u128,
                pub is_sufficient: ::core::primitive::bool,
                pub is_frozen: ::core::primitive::bool,
            }
            impl ::subxt::Call for ForceAssetStatus {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "force_asset_status";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ApproveTransfer {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub delegate: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                #[codec(compact)]
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Call for ApproveTransfer {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "approve_transfer";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct CancelApproval {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub delegate: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
            }
            impl ::subxt::Call for CancelApproval {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "cancel_approval";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ForceCancelApproval {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub owner: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                pub delegate: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
            }
            impl ::subxt::Call for ForceCancelApproval {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "force_cancel_approval";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct TransferApproved {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub owner: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                pub destination: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                #[codec(compact)]
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Call for TransferApproved {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "transfer_approved";
            }
            pub struct TransactionApi<
                'a,
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            > {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn create(
                    &self,
                    id: ::core::primitive::u32,
                    admin: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    min_balance: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, Create>
                {
                    let call = Create {
                        id,
                        admin,
                        min_balance,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_create(
                    &self,
                    id: ::core::primitive::u32,
                    owner: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    is_sufficient: ::core::primitive::bool,
                    min_balance: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, ForceCreate>
                {
                    let call = ForceCreate {
                        id,
                        owner,
                        is_sufficient,
                        min_balance,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn destroy(
                    &self,
                    id: ::core::primitive::u32,
                    witness : runtime_types :: pallet_assets :: types :: DestroyWitness,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, Destroy>
                {
                    let call = Destroy { id, witness };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn mint(
                    &self,
                    id: ::core::primitive::u32,
                    beneficiary: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    amount: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, Mint>
                {
                    let call = Mint {
                        id,
                        beneficiary,
                        amount,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn burn(
                    &self,
                    id: ::core::primitive::u32,
                    who: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    amount: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, Burn>
                {
                    let call = Burn { id, who, amount };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn transfer(
                    &self,
                    id: ::core::primitive::u32,
                    target: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    amount: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, Transfer>
                {
                    let call = Transfer { id, target, amount };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn transfer_keep_alive(
                    &self,
                    id: ::core::primitive::u32,
                    target: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    amount: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, TransferKeepAlive>
                {
                    let call = TransferKeepAlive { id, target, amount };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_transfer(
                    &self,
                    id: ::core::primitive::u32,
                    source: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    dest: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    amount: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, ForceTransfer>
                {
                    let call = ForceTransfer {
                        id,
                        source,
                        dest,
                        amount,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn freeze(
                    &self,
                    id: ::core::primitive::u32,
                    who: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, Freeze>
                {
                    let call = Freeze { id, who };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn thaw(
                    &self,
                    id: ::core::primitive::u32,
                    who: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, Thaw>
                {
                    let call = Thaw { id, who };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn freeze_asset(
                    &self,
                    id: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, FreezeAsset>
                {
                    let call = FreezeAsset { id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn thaw_asset(
                    &self,
                    id: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, ThawAsset>
                {
                    let call = ThawAsset { id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn transfer_ownership(
                    &self,
                    id: ::core::primitive::u32,
                    owner: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, TransferOwnership>
                {
                    let call = TransferOwnership { id, owner };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_team(
                    &self,
                    id: ::core::primitive::u32,
                    issuer: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    admin: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    freezer: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, SetTeam>
                {
                    let call = SetTeam {
                        id,
                        issuer,
                        admin,
                        freezer,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_metadata(
                    &self,
                    id: ::core::primitive::u32,
                    name: ::std::vec::Vec<::core::primitive::u8>,
                    symbol: ::std::vec::Vec<::core::primitive::u8>,
                    decimals: ::core::primitive::u8,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, SetMetadata>
                {
                    let call = SetMetadata {
                        id,
                        name,
                        symbol,
                        decimals,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn clear_metadata(
                    &self,
                    id: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, ClearMetadata>
                {
                    let call = ClearMetadata { id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_set_metadata(
                    &self,
                    id: ::core::primitive::u32,
                    name: ::std::vec::Vec<::core::primitive::u8>,
                    symbol: ::std::vec::Vec<::core::primitive::u8>,
                    decimals: ::core::primitive::u8,
                    is_frozen: ::core::primitive::bool,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, ForceSetMetadata>
                {
                    let call = ForceSetMetadata {
                        id,
                        name,
                        symbol,
                        decimals,
                        is_frozen,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_clear_metadata(
                    &self,
                    id: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, ForceClearMetadata>
                {
                    let call = ForceClearMetadata { id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_asset_status(
                    &self,
                    id: ::core::primitive::u32,
                    owner: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    issuer: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    admin: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    freezer: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    min_balance: ::core::primitive::u128,
                    is_sufficient: ::core::primitive::bool,
                    is_frozen: ::core::primitive::bool,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, ForceAssetStatus>
                {
                    let call = ForceAssetStatus {
                        id,
                        owner,
                        issuer,
                        admin,
                        freezer,
                        min_balance,
                        is_sufficient,
                        is_frozen,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn approve_transfer(
                    &self,
                    id: ::core::primitive::u32,
                    delegate: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    amount: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, ApproveTransfer>
                {
                    let call = ApproveTransfer {
                        id,
                        delegate,
                        amount,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn cancel_approval(
                    &self,
                    id: ::core::primitive::u32,
                    delegate: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, CancelApproval>
                {
                    let call = CancelApproval { id, delegate };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_cancel_approval(
                    &self,
                    id: ::core::primitive::u32,
                    owner: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    delegate: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, ForceCancelApproval>
                {
                    let call = ForceCancelApproval {
                        id,
                        owner,
                        delegate,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn transfer_approved(
                    &self,
                    id: ::core::primitive::u32,
                    owner: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    destination: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    amount: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, TransferApproved>
                {
                    let call = TransferApproved {
                        id,
                        owner,
                        destination,
                        amount,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_assets::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Created(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for Created {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "Created";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Issued(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Issued {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "Issued";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Transferred(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Transferred {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "Transferred";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Burned(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Burned {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "Burned";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct TeamChanged(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for TeamChanged {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "TeamChanged";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct OwnerChanged(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for OwnerChanged {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "OwnerChanged";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Frozen(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for Frozen {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "Frozen";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Thawed(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for Thawed {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "Thawed";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct AssetFrozen(pub ::core::primitive::u32);
            impl ::subxt::Event for AssetFrozen {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "AssetFrozen";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct AssetThawed(pub ::core::primitive::u32);
            impl ::subxt::Event for AssetThawed {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "AssetThawed";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Destroyed(pub ::core::primitive::u32);
            impl ::subxt::Event for Destroyed {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "Destroyed";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ForceCreated(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for ForceCreated {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "ForceCreated";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct MetadataSet(
                pub ::core::primitive::u32,
                pub ::std::vec::Vec<::core::primitive::u8>,
                pub ::std::vec::Vec<::core::primitive::u8>,
                pub ::core::primitive::u8,
                pub ::core::primitive::bool,
            );
            impl ::subxt::Event for MetadataSet {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "MetadataSet";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct MetadataCleared(pub ::core::primitive::u32);
            impl ::subxt::Event for MetadataCleared {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "MetadataCleared";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ApprovedTransfer(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for ApprovedTransfer {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "ApprovedTransfer";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ApprovalCancelled(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for ApprovalCancelled {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "ApprovalCancelled";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct TransferredApproved(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for TransferredApproved {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "TransferredApproved";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct AssetStatusChanged(pub ::core::primitive::u32);
            impl ::subxt::Event for AssetStatusChanged {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "AssetStatusChanged";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Asset(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for Asset {
                const PALLET: &'static str = "Assets";
                const STORAGE: &'static str = "Asset";
                type Value = runtime_types::pallet_assets::types::AssetDetails<
                    ::core::primitive::u128,
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u128,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct Account(
                ::core::primitive::u32,
                ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::StorageEntry for Account {
                const PALLET: &'static str = "Assets";
                const STORAGE: &'static str = "Account";
                type Value = runtime_types::pallet_assets::types::AssetBalance<
                    ::core::primitive::u128,
                    (),
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct Approvals(
                ::core::primitive::u32,
                ::subxt::sp_core::crypto::AccountId32,
                ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::StorageEntry for Approvals {
                const PALLET: &'static str = "Assets";
                const STORAGE: &'static str = "Approvals";
                type Value = runtime_types::pallet_assets::types::Approval<
                    ::core::primitive::u128,
                    ::core::primitive::u128,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.2,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct Metadata(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for Metadata {
                const PALLET: &'static str = "Assets";
                const STORAGE: &'static str = "Metadata";
                type Value = runtime_types :: pallet_assets :: types :: AssetMetadata < :: core :: primitive :: u128 , runtime_types :: frame_support :: storage :: bounded_vec :: BoundedVec < :: core :: primitive :: u8 > > ;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn asset(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_assets::types::AssetDetails<
                            ::core::primitive::u128,
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Asset(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn asset_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Asset>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn account(
                    &self,
                    _0: ::core::primitive::u32,
                    _1: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_assets::types::AssetBalance<
                        ::core::primitive::u128,
                        (),
                    >,
                    ::subxt::Error,
                > {
                    let entry = Account(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn account_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Account>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn approvals(
                    &self,
                    _0: ::core::primitive::u32,
                    _1: ::subxt::sp_core::crypto::AccountId32,
                    _2: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_assets::types::Approval<
                            ::core::primitive::u128,
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Approvals(_0, _1, _2);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn approvals_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Approvals>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }                pub async fn metadata (& self , _0 : :: core :: primitive :: u32 , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < runtime_types :: pallet_assets :: types :: AssetMetadata < :: core :: primitive :: u128 , runtime_types :: frame_support :: storage :: bounded_vec :: BoundedVec < :: core :: primitive :: u8 > > , :: subxt :: Error >{
                    let entry = Metadata(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn metadata_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Metadata>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod sudo {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Sudo {
                pub call: runtime_types::darkwebb_standalone_runtime::Call,
            }
            impl ::subxt::Call for Sudo {
                const PALLET: &'static str = "Sudo";
                const FUNCTION: &'static str = "sudo";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct SudoUncheckedWeight {
                pub call: runtime_types::darkwebb_standalone_runtime::Call,
                pub weight: ::core::primitive::u64,
            }
            impl ::subxt::Call for SudoUncheckedWeight {
                const PALLET: &'static str = "Sudo";
                const FUNCTION: &'static str = "sudo_unchecked_weight";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct SetKey {
                pub new: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
            }
            impl ::subxt::Call for SetKey {
                const PALLET: &'static str = "Sudo";
                const FUNCTION: &'static str = "set_key";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct SudoAs {
                pub who: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                pub call: runtime_types::darkwebb_standalone_runtime::Call,
            }
            impl ::subxt::Call for SudoAs {
                const PALLET: &'static str = "Sudo";
                const FUNCTION: &'static str = "sudo_as";
            }
            pub struct TransactionApi<
                'a,
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            > {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn sudo(
                    &self,
                    call: runtime_types::darkwebb_standalone_runtime::Call,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, Sudo>
                {
                    let call = Sudo { call };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn sudo_unchecked_weight(
                    &self,
                    call: runtime_types::darkwebb_standalone_runtime::Call,
                    weight: ::core::primitive::u64,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, SudoUncheckedWeight>
                {
                    let call = SudoUncheckedWeight { call, weight };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_key(
                    &self,
                    new: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, SetKey>
                {
                    let call = SetKey { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn sudo_as(
                    &self,
                    who: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    call: runtime_types::darkwebb_standalone_runtime::Call,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, SudoAs>
                {
                    let call = SudoAs { who, call };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_sudo::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Sudid(
                pub  ::core::result::Result<
                    (),
                    runtime_types::sp_runtime::DispatchError,
                >,
            );
            impl ::subxt::Event for Sudid {
                const PALLET: &'static str = "Sudo";
                const EVENT: &'static str = "Sudid";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct KeyChanged(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::Event for KeyChanged {
                const PALLET: &'static str = "Sudo";
                const EVENT: &'static str = "KeyChanged";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct SudoAsDone(
                pub  ::core::result::Result<
                    (),
                    runtime_types::sp_runtime::DispatchError,
                >,
            );
            impl ::subxt::Event for SudoAsDone {
                const PALLET: &'static str = "Sudo";
                const EVENT: &'static str = "SudoAsDone";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Key;
            impl ::subxt::StorageEntry for Key {
                const PALLET: &'static str = "Sudo";
                const STORAGE: &'static str = "Key";
                type Value = ::subxt::sp_core::crypto::AccountId32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn key(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::subxt::Error,
                > {
                    let entry = Key;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod im_online {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Heartbeat { pub heartbeat : runtime_types :: pallet_im_online :: Heartbeat < :: core :: primitive :: u32 > , pub signature : runtime_types :: pallet_im_online :: sr25519 :: app_sr25519 :: Signature }
            impl ::subxt::Call for Heartbeat {
                const PALLET: &'static str = "ImOnline";
                const FUNCTION: &'static str = "heartbeat";
            }
            pub struct TransactionApi<
                'a,
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            > {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn heartbeat(
                    &self,
                    heartbeat: runtime_types::pallet_im_online::Heartbeat<
                        ::core::primitive::u32,
                    >,
                    signature : runtime_types :: pallet_im_online :: sr25519 :: app_sr25519 :: Signature,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, Heartbeat>
                {
                    let call = Heartbeat {
                        heartbeat,
                        signature,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_im_online::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct HeartbeatReceived (pub runtime_types :: pallet_im_online :: sr25519 :: app_sr25519 :: Public) ;
            impl ::subxt::Event for HeartbeatReceived {
                const PALLET: &'static str = "ImOnline";
                const EVENT: &'static str = "HeartbeatReceived";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct AllGood {}
            impl ::subxt::Event for AllGood {
                const PALLET: &'static str = "ImOnline";
                const EVENT: &'static str = "AllGood";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct SomeOffline(
                pub  ::std::vec::Vec<(
                    ::subxt::sp_core::crypto::AccountId32,
                    runtime_types::pallet_staking::Exposure<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    >,
                )>,
            );
            impl ::subxt::Event for SomeOffline {
                const PALLET: &'static str = "ImOnline";
                const EVENT: &'static str = "SomeOffline";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct HeartbeatAfter;
            impl ::subxt::StorageEntry for HeartbeatAfter {
                const PALLET: &'static str = "ImOnline";
                const STORAGE: &'static str = "HeartbeatAfter";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Keys;
            impl ::subxt::StorageEntry for Keys {
                const PALLET: &'static str = "ImOnline";
                const STORAGE: &'static str = "Keys";
                type Value = runtime_types :: frame_support :: storage :: weak_bounded_vec :: WeakBoundedVec < runtime_types :: pallet_im_online :: sr25519 :: app_sr25519 :: Public > ;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ReceivedHeartbeats(
                ::core::primitive::u32,
                ::core::primitive::u32,
            );
            impl ::subxt::StorageEntry for ReceivedHeartbeats {
                const PALLET: &'static str = "ImOnline";
                const STORAGE: &'static str = "ReceivedHeartbeats";
                type Value = runtime_types :: frame_support :: traits :: misc :: WrapperOpaque < runtime_types :: pallet_im_online :: BoundedOpaqueNetworkState > ;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                    ])
                }
            }
            pub struct AuthoredBlocks(
                ::core::primitive::u32,
                ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::StorageEntry for AuthoredBlocks {
                const PALLET: &'static str = "ImOnline";
                const STORAGE: &'static str = "AuthoredBlocks";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                    ])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn heartbeat_after(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::u32,
                    ::subxt::Error,
                > {
                    let entry = HeartbeatAfter;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }                pub async fn keys (& self , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < runtime_types :: frame_support :: storage :: weak_bounded_vec :: WeakBoundedVec < runtime_types :: pallet_im_online :: sr25519 :: app_sr25519 :: Public > , :: subxt :: Error >{
                    let entry = Keys;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }                pub async fn received_heartbeats (& self , _0 : :: core :: primitive :: u32 , _1 : :: core :: primitive :: u32 , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < :: core :: option :: Option < runtime_types :: frame_support :: traits :: misc :: WrapperOpaque < runtime_types :: pallet_im_online :: BoundedOpaqueNetworkState > > , :: subxt :: Error >{
                    let entry = ReceivedHeartbeats(_0, _1);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn received_heartbeats_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ReceivedHeartbeats>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn authored_blocks(
                    &self,
                    _0: ::core::primitive::u32,
                    _1: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::u32,
                    ::subxt::Error,
                > {
                    let entry = AuthoredBlocks(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn authored_blocks_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, AuthoredBlocks>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod authority_discovery {
        use super::runtime_types;
    }
    pub mod offences {
        use super::runtime_types;
        pub type Event = runtime_types::pallet_offences::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Offence(
                pub [::core::primitive::u8; 16usize],
                pub ::std::vec::Vec<::core::primitive::u8>,
            );
            impl ::subxt::Event for Offence {
                const PALLET: &'static str = "Offences";
                const EVENT: &'static str = "Offence";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Reports(pub ::subxt::sp_core::H256);
            impl ::subxt::StorageEntry for Reports {
                const PALLET: &'static str = "Offences";
                const STORAGE: &'static str = "Reports";
                type Value = runtime_types::sp_staking::offence::OffenceDetails<
                    ::subxt::sp_core::crypto::AccountId32,
                    (
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::pallet_staking::Exposure<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        >,
                    ),
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                    ])
                }
            }
            pub struct ConcurrentReportsIndex(
                [::core::primitive::u8; 16usize],
                ::std::vec::Vec<::core::primitive::u8>,
            );
            impl ::subxt::StorageEntry for ConcurrentReportsIndex {
                const PALLET: &'static str = "Offences";
                const STORAGE: &'static str = "ConcurrentReportsIndex";
                type Value = ::std::vec::Vec<::subxt::sp_core::H256>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                    ])
                }
            }
            pub struct ReportsByKindIndex(pub [::core::primitive::u8; 16usize]);
            impl ::subxt::StorageEntry for ReportsByKindIndex {
                const PALLET: &'static str = "Offences";
                const STORAGE: &'static str = "ReportsByKindIndex";
                type Value = ::std::vec::Vec<::core::primitive::u8>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                    ])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn reports(
                    &self,
                    _0: ::subxt::sp_core::H256,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::sp_staking::offence::OffenceDetails<
                            ::subxt::sp_core::crypto::AccountId32,
                            (
                                ::subxt::sp_core::crypto::AccountId32,
                                runtime_types::pallet_staking::Exposure<
                                    ::subxt::sp_core::crypto::AccountId32,
                                    ::core::primitive::u128,
                                >,
                            ),
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Reports(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn reports_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Reports>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn concurrent_reports_index(
                    &self,
                    _0: [::core::primitive::u8; 16usize],
                    _1: ::std::vec::Vec<::core::primitive::u8>,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::subxt::sp_core::H256>,
                    ::subxt::Error,
                > {
                    let entry = ConcurrentReportsIndex(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn concurrent_reports_index_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ConcurrentReportsIndex>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn reports_by_kind_index(
                    &self,
                    _0: [::core::primitive::u8; 16usize],
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::subxt::Error,
                > {
                    let entry = ReportsByKindIndex(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn reports_by_kind_index_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ReportsByKindIndex>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod historical {
        use super::runtime_types;
    }
    pub mod bounties {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ProposeBounty {
                #[codec(compact)]
                pub value: ::core::primitive::u128,
                pub description: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for ProposeBounty {
                const PALLET: &'static str = "Bounties";
                const FUNCTION: &'static str = "propose_bounty";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ApproveBounty {
                #[codec(compact)]
                pub bounty_id: ::core::primitive::u32,
            }
            impl ::subxt::Call for ApproveBounty {
                const PALLET: &'static str = "Bounties";
                const FUNCTION: &'static str = "approve_bounty";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ProposeCurator {
                #[codec(compact)]
                pub bounty_id: ::core::primitive::u32,
                pub curator: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                #[codec(compact)]
                pub fee: ::core::primitive::u128,
            }
            impl ::subxt::Call for ProposeCurator {
                const PALLET: &'static str = "Bounties";
                const FUNCTION: &'static str = "propose_curator";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct UnassignCurator {
                #[codec(compact)]
                pub bounty_id: ::core::primitive::u32,
            }
            impl ::subxt::Call for UnassignCurator {
                const PALLET: &'static str = "Bounties";
                const FUNCTION: &'static str = "unassign_curator";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct AcceptCurator {
                #[codec(compact)]
                pub bounty_id: ::core::primitive::u32,
            }
            impl ::subxt::Call for AcceptCurator {
                const PALLET: &'static str = "Bounties";
                const FUNCTION: &'static str = "accept_curator";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct AwardBounty {
                #[codec(compact)]
                pub bounty_id: ::core::primitive::u32,
                pub beneficiary: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
            }
            impl ::subxt::Call for AwardBounty {
                const PALLET: &'static str = "Bounties";
                const FUNCTION: &'static str = "award_bounty";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ClaimBounty {
                #[codec(compact)]
                pub bounty_id: ::core::primitive::u32,
            }
            impl ::subxt::Call for ClaimBounty {
                const PALLET: &'static str = "Bounties";
                const FUNCTION: &'static str = "claim_bounty";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct CloseBounty {
                #[codec(compact)]
                pub bounty_id: ::core::primitive::u32,
            }
            impl ::subxt::Call for CloseBounty {
                const PALLET: &'static str = "Bounties";
                const FUNCTION: &'static str = "close_bounty";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ExtendBountyExpiry {
                #[codec(compact)]
                pub bounty_id: ::core::primitive::u32,
                pub remark: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for ExtendBountyExpiry {
                const PALLET: &'static str = "Bounties";
                const FUNCTION: &'static str = "extend_bounty_expiry";
            }
            pub struct TransactionApi<
                'a,
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            > {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn propose_bounty(
                    &self,
                    value: ::core::primitive::u128,
                    description: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, ProposeBounty>
                {
                    let call = ProposeBounty { value, description };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn approve_bounty(
                    &self,
                    bounty_id: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, ApproveBounty>
                {
                    let call = ApproveBounty { bounty_id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn propose_curator(
                    &self,
                    bounty_id: ::core::primitive::u32,
                    curator: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    fee: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, ProposeCurator>
                {
                    let call = ProposeCurator {
                        bounty_id,
                        curator,
                        fee,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn unassign_curator(
                    &self,
                    bounty_id: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, UnassignCurator>
                {
                    let call = UnassignCurator { bounty_id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn accept_curator(
                    &self,
                    bounty_id: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, AcceptCurator>
                {
                    let call = AcceptCurator { bounty_id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn award_bounty(
                    &self,
                    bounty_id: ::core::primitive::u32,
                    beneficiary: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, AwardBounty>
                {
                    let call = AwardBounty {
                        bounty_id,
                        beneficiary,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn claim_bounty(
                    &self,
                    bounty_id: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, ClaimBounty>
                {
                    let call = ClaimBounty { bounty_id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn close_bounty(
                    &self,
                    bounty_id: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, CloseBounty>
                {
                    let call = CloseBounty { bounty_id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn extend_bounty_expiry(
                    &self,
                    bounty_id: ::core::primitive::u32,
                    remark: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, ExtendBountyExpiry>
                {
                    let call = ExtendBountyExpiry { bounty_id, remark };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_bounties::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct BountyProposed(pub ::core::primitive::u32);
            impl ::subxt::Event for BountyProposed {
                const PALLET: &'static str = "Bounties";
                const EVENT: &'static str = "BountyProposed";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct BountyRejected(
                pub ::core::primitive::u32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for BountyRejected {
                const PALLET: &'static str = "Bounties";
                const EVENT: &'static str = "BountyRejected";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct BountyBecameActive(pub ::core::primitive::u32);
            impl ::subxt::Event for BountyBecameActive {
                const PALLET: &'static str = "Bounties";
                const EVENT: &'static str = "BountyBecameActive";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct BountyAwarded(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for BountyAwarded {
                const PALLET: &'static str = "Bounties";
                const EVENT: &'static str = "BountyAwarded";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct BountyClaimed(
                pub ::core::primitive::u32,
                pub ::core::primitive::u128,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for BountyClaimed {
                const PALLET: &'static str = "Bounties";
                const EVENT: &'static str = "BountyClaimed";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct BountyCanceled(pub ::core::primitive::u32);
            impl ::subxt::Event for BountyCanceled {
                const PALLET: &'static str = "Bounties";
                const EVENT: &'static str = "BountyCanceled";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct BountyExtended(pub ::core::primitive::u32);
            impl ::subxt::Event for BountyExtended {
                const PALLET: &'static str = "Bounties";
                const EVENT: &'static str = "BountyExtended";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct BountyCount;
            impl ::subxt::StorageEntry for BountyCount {
                const PALLET: &'static str = "Bounties";
                const STORAGE: &'static str = "BountyCount";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Bounties(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for Bounties {
                const PALLET: &'static str = "Bounties";
                const STORAGE: &'static str = "Bounties";
                type Value = runtime_types::pallet_bounties::Bounty<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u128,
                    ::core::primitive::u32,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                    ])
                }
            }
            pub struct BountyDescriptions(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for BountyDescriptions {
                const PALLET: &'static str = "Bounties";
                const STORAGE: &'static str = "BountyDescriptions";
                type Value = ::std::vec::Vec<::core::primitive::u8>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                    ])
                }
            }
            pub struct BountyApprovals;
            impl ::subxt::StorageEntry for BountyApprovals {
                const PALLET: &'static str = "Bounties";
                const STORAGE: &'static str = "BountyApprovals";
                type Value = ::std::vec::Vec<::core::primitive::u32>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn bounty_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::u32,
                    ::subxt::Error,
                > {
                    let entry = BountyCount;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn bounties(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_bounties::Bounty<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                            ::core::primitive::u32,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Bounties(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn bounties_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Bounties>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn bounty_descriptions(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        ::std::vec::Vec<::core::primitive::u8>,
                    >,
                    ::subxt::Error,
                > {
                    let entry = BountyDescriptions(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn bounty_descriptions_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, BountyDescriptions>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn bounty_approvals(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::core::primitive::u32>,
                    ::subxt::Error,
                > {
                    let entry = BountyApprovals;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod bags_list {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Rebag {
                pub dislocated: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Call for Rebag {
                const PALLET: &'static str = "BagsList";
                const FUNCTION: &'static str = "rebag";
            }
            pub struct TransactionApi<
                'a,
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            > {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn rebag(
                    &self,
                    dislocated: ::subxt::sp_core::crypto::AccountId32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, Rebag>
                {
                    let call = Rebag { dislocated };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_bags_list::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Rebagged(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u64,
                pub ::core::primitive::u64,
            );
            impl ::subxt::Event for Rebagged {
                const PALLET: &'static str = "BagsList";
                const EVENT: &'static str = "Rebagged";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct CounterForListNodes;
            impl ::subxt::StorageEntry for CounterForListNodes {
                const PALLET: &'static str = "BagsList";
                const STORAGE: &'static str = "CounterForListNodes";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ListNodes(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for ListNodes {
                const PALLET: &'static str = "BagsList";
                const STORAGE: &'static str = "ListNodes";
                type Value = runtime_types::pallet_bags_list::list::Node;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                    ])
                }
            }
            pub struct ListBags(pub ::core::primitive::u64);
            impl ::subxt::StorageEntry for ListBags {
                const PALLET: &'static str = "BagsList";
                const STORAGE: &'static str = "ListBags";
                type Value = runtime_types::pallet_bags_list::list::Bag;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                    ])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn counter_for_list_nodes(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::u32,
                    ::subxt::Error,
                > {
                    let entry = CounterForListNodes;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn list_nodes(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_bags_list::list::Node,
                    >,
                    ::subxt::Error,
                > {
                    let entry = ListNodes(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn list_nodes_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ListNodes>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn list_bags(
                    &self,
                    _0: ::core::primitive::u64,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_bags_list::list::Bag,
                    >,
                    ::subxt::Error,
                > {
                    let entry = ListBags(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn list_bags_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ListBags>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod hasher_bn254 {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct SetParameters {
                pub parameters: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for SetParameters {
                const PALLET: &'static str = "HasherBn254";
                const FUNCTION: &'static str = "set_parameters";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct SetMaintainer {
                pub new_maintainer: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Call for SetMaintainer {
                const PALLET: &'static str = "HasherBn254";
                const FUNCTION: &'static str = "set_maintainer";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ForceSetParameters {
                pub parameters: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for ForceSetParameters {
                const PALLET: &'static str = "HasherBn254";
                const FUNCTION: &'static str = "force_set_parameters";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ForceSetMaintainer {
                pub new_maintainer: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Call for ForceSetMaintainer {
                const PALLET: &'static str = "HasherBn254";
                const FUNCTION: &'static str = "force_set_maintainer";
            }
            pub struct TransactionApi<
                'a,
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            > {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn set_parameters(
                    &self,
                    parameters: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, SetParameters>
                {
                    let call = SetParameters { parameters };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_maintainer(
                    &self,
                    new_maintainer: ::subxt::sp_core::crypto::AccountId32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, SetMaintainer>
                {
                    let call = SetMaintainer { new_maintainer };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_set_parameters(
                    &self,
                    parameters: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, ForceSetParameters>
                {
                    let call = ForceSetParameters { parameters };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_set_maintainer(
                    &self,
                    new_maintainer: ::subxt::sp_core::crypto::AccountId32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, ForceSetMaintainer>
                {
                    let call = ForceSetMaintainer { new_maintainer };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_hasher::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ParametersSet {
                pub who: ::subxt::sp_core::crypto::AccountId32,
                pub parameters: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Event for ParametersSet {
                const PALLET: &'static str = "HasherBn254";
                const EVENT: &'static str = "ParametersSet";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct MaintainerSet {
                pub old_maintainer: ::subxt::sp_core::crypto::AccountId32,
                pub new_maintainer: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for MaintainerSet {
                const PALLET: &'static str = "HasherBn254";
                const EVENT: &'static str = "MaintainerSet";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Parameters;
            impl ::subxt::StorageEntry for Parameters {
                const PALLET: &'static str = "HasherBn254";
                const STORAGE: &'static str = "Parameters";
                type Value = ::std::vec::Vec<::core::primitive::u8>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Deposit;
            impl ::subxt::StorageEntry for Deposit {
                const PALLET: &'static str = "HasherBn254";
                const STORAGE: &'static str = "Deposit";
                type Value = ::core::option::Option<
                    runtime_types::darkwebb_primitives::types::DepositDetails<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    >,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Maintainer;
            impl ::subxt::StorageEntry for Maintainer {
                const PALLET: &'static str = "HasherBn254";
                const STORAGE: &'static str = "Maintainer";
                type Value = ::subxt::sp_core::crypto::AccountId32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn parameters(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::subxt::Error,
                > {
                    let entry = Parameters;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }                pub async fn deposit (& self , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < :: core :: option :: Option < runtime_types :: darkwebb_primitives :: types :: DepositDetails < :: subxt :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u128 > > , :: subxt :: Error >{
                    let entry = Deposit;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn maintainer(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::subxt::Error,
                > {
                    let entry = Maintainer;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod hasher_bls381 {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct SetParameters {
                pub parameters: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for SetParameters {
                const PALLET: &'static str = "HasherBls381";
                const FUNCTION: &'static str = "set_parameters";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct SetMaintainer {
                pub new_maintainer: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Call for SetMaintainer {
                const PALLET: &'static str = "HasherBls381";
                const FUNCTION: &'static str = "set_maintainer";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ForceSetParameters {
                pub parameters: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for ForceSetParameters {
                const PALLET: &'static str = "HasherBls381";
                const FUNCTION: &'static str = "force_set_parameters";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ForceSetMaintainer {
                pub new_maintainer: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Call for ForceSetMaintainer {
                const PALLET: &'static str = "HasherBls381";
                const FUNCTION: &'static str = "force_set_maintainer";
            }
            pub struct TransactionApi<
                'a,
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            > {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn set_parameters(
                    &self,
                    parameters: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, SetParameters>
                {
                    let call = SetParameters { parameters };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_maintainer(
                    &self,
                    new_maintainer: ::subxt::sp_core::crypto::AccountId32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, SetMaintainer>
                {
                    let call = SetMaintainer { new_maintainer };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_set_parameters(
                    &self,
                    parameters: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, ForceSetParameters>
                {
                    let call = ForceSetParameters { parameters };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_set_maintainer(
                    &self,
                    new_maintainer: ::subxt::sp_core::crypto::AccountId32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, ForceSetMaintainer>
                {
                    let call = ForceSetMaintainer { new_maintainer };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_hasher::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ParametersSet {
                pub who: ::subxt::sp_core::crypto::AccountId32,
                pub parameters: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Event for ParametersSet {
                const PALLET: &'static str = "HasherBls381";
                const EVENT: &'static str = "ParametersSet";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct MaintainerSet {
                pub old_maintainer: ::subxt::sp_core::crypto::AccountId32,
                pub new_maintainer: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for MaintainerSet {
                const PALLET: &'static str = "HasherBls381";
                const EVENT: &'static str = "MaintainerSet";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Parameters;
            impl ::subxt::StorageEntry for Parameters {
                const PALLET: &'static str = "HasherBls381";
                const STORAGE: &'static str = "Parameters";
                type Value = ::std::vec::Vec<::core::primitive::u8>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Deposit;
            impl ::subxt::StorageEntry for Deposit {
                const PALLET: &'static str = "HasherBls381";
                const STORAGE: &'static str = "Deposit";
                type Value = ::core::option::Option<
                    runtime_types::darkwebb_primitives::types::DepositDetails<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    >,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Maintainer;
            impl ::subxt::StorageEntry for Maintainer {
                const PALLET: &'static str = "HasherBls381";
                const STORAGE: &'static str = "Maintainer";
                type Value = ::subxt::sp_core::crypto::AccountId32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn parameters(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::subxt::Error,
                > {
                    let entry = Parameters;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }                pub async fn deposit (& self , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < :: core :: option :: Option < runtime_types :: darkwebb_primitives :: types :: DepositDetails < :: subxt :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u128 > > , :: subxt :: Error >{
                    let entry = Deposit;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn maintainer(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::subxt::Error,
                > {
                    let entry = Maintainer;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod asset_registry {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Register {
                pub name: ::std::vec::Vec<::core::primitive::u8>,
                pub asset_type:
                    runtime_types::pallet_asset_registry::types::AssetType<
                        ::core::primitive::u32,
                    >,
                pub existential_deposit: ::core::primitive::u128,
            }
            impl ::subxt::Call for Register {
                const PALLET: &'static str = "AssetRegistry";
                const FUNCTION: &'static str = "register";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Update {
                pub asset_id: ::core::primitive::u32,
                pub name: ::std::vec::Vec<::core::primitive::u8>,
                pub asset_type:
                    runtime_types::pallet_asset_registry::types::AssetType<
                        ::core::primitive::u32,
                    >,
                pub existential_deposit:
                    ::core::option::Option<::core::primitive::u128>,
            }
            impl ::subxt::Call for Update {
                const PALLET: &'static str = "AssetRegistry";
                const FUNCTION: &'static str = "update";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct SetMetadata {
                pub asset_id: ::core::primitive::u32,
                pub symbol: ::std::vec::Vec<::core::primitive::u8>,
                pub decimals: ::core::primitive::u8,
            }
            impl ::subxt::Call for SetMetadata {
                const PALLET: &'static str = "AssetRegistry";
                const FUNCTION: &'static str = "set_metadata";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct SetLocation {
                pub asset_id: ::core::primitive::u32,
                pub location: (),
            }
            impl ::subxt::Call for SetLocation {
                const PALLET: &'static str = "AssetRegistry";
                const FUNCTION: &'static str = "set_location";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct AddAssetToPool {
                pub pool: ::std::vec::Vec<::core::primitive::u8>,
                pub asset_id: ::core::primitive::u32,
            }
            impl ::subxt::Call for AddAssetToPool {
                const PALLET: &'static str = "AssetRegistry";
                const FUNCTION: &'static str = "add_asset_to_pool";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct DeleteAssetFromPool {
                pub pool: ::std::vec::Vec<::core::primitive::u8>,
                pub asset_id: ::core::primitive::u32,
            }
            impl ::subxt::Call for DeleteAssetFromPool {
                const PALLET: &'static str = "AssetRegistry";
                const FUNCTION: &'static str = "delete_asset_from_pool";
            }
            pub struct TransactionApi<
                'a,
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            > {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn register(
                    &self,
                    name: ::std::vec::Vec<::core::primitive::u8>,
                    asset_type : runtime_types :: pallet_asset_registry :: types :: AssetType < :: core :: primitive :: u32 >,
                    existential_deposit: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, Register>
                {
                    let call = Register {
                        name,
                        asset_type,
                        existential_deposit,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn update(
                    &self,
                    asset_id: ::core::primitive::u32,
                    name: ::std::vec::Vec<::core::primitive::u8>,
                    asset_type : runtime_types :: pallet_asset_registry :: types :: AssetType < :: core :: primitive :: u32 >,
                    existential_deposit: ::core::option::Option<
                        ::core::primitive::u128,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, Update>
                {
                    let call = Update {
                        asset_id,
                        name,
                        asset_type,
                        existential_deposit,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_metadata(
                    &self,
                    asset_id: ::core::primitive::u32,
                    symbol: ::std::vec::Vec<::core::primitive::u8>,
                    decimals: ::core::primitive::u8,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, SetMetadata>
                {
                    let call = SetMetadata {
                        asset_id,
                        symbol,
                        decimals,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_location(
                    &self,
                    asset_id: ::core::primitive::u32,
                    location: (),
                ) -> ::subxt::SubmittableExtrinsic<'a, T, SetLocation>
                {
                    let call = SetLocation { asset_id, location };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn add_asset_to_pool(
                    &self,
                    pool: ::std::vec::Vec<::core::primitive::u8>,
                    asset_id: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, AddAssetToPool>
                {
                    let call = AddAssetToPool { pool, asset_id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn delete_asset_from_pool(
                    &self,
                    pool: ::std::vec::Vec<::core::primitive::u8>,
                    asset_id: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, DeleteAssetFromPool>
                {
                    let call = DeleteAssetFromPool { pool, asset_id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_asset_registry::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Registered { pub asset_id : :: core :: primitive :: u32 , pub name : runtime_types :: frame_support :: storage :: bounded_vec :: BoundedVec < :: core :: primitive :: u8 > , pub asset_type : runtime_types :: pallet_asset_registry :: types :: AssetType < :: core :: primitive :: u32 > }
            impl ::subxt::Event for Registered {
                const PALLET: &'static str = "AssetRegistry";
                const EVENT: &'static str = "Registered";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Updated { pub asset_id : :: core :: primitive :: u32 , pub name : runtime_types :: frame_support :: storage :: bounded_vec :: BoundedVec < :: core :: primitive :: u8 > , pub asset_type : runtime_types :: pallet_asset_registry :: types :: AssetType < :: core :: primitive :: u32 > }
            impl ::subxt::Event for Updated {
                const PALLET: &'static str = "AssetRegistry";
                const EVENT: &'static str = "Updated";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct MetadataSet { pub asset_id : :: core :: primitive :: u32 , pub symbol : runtime_types :: frame_support :: storage :: bounded_vec :: BoundedVec < :: core :: primitive :: u8 > , pub decimals : :: core :: primitive :: u8 }
            impl ::subxt::Event for MetadataSet {
                const PALLET: &'static str = "AssetRegistry";
                const EVENT: &'static str = "MetadataSet";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct LocationSet {
                pub asset_id: ::core::primitive::u32,
                pub location: (),
            }
            impl ::subxt::Event for LocationSet {
                const PALLET: &'static str = "AssetRegistry";
                const EVENT: &'static str = "LocationSet";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Assets(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for Assets {
                const PALLET: &'static str = "AssetRegistry";
                const STORAGE: &'static str = "Assets";
                type Value = runtime_types :: pallet_asset_registry :: types :: AssetDetails < :: core :: primitive :: u32 , :: core :: primitive :: u128 , runtime_types :: frame_support :: storage :: bounded_vec :: BoundedVec < :: core :: primitive :: u8 > > ;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                    ])
                }
            }
            pub struct NextAssetId;
            impl ::subxt::StorageEntry for NextAssetId {
                const PALLET: &'static str = "AssetRegistry";
                const STORAGE: &'static str = "NextAssetId";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct AssetIds (pub runtime_types :: frame_support :: storage :: bounded_vec :: BoundedVec < :: core :: primitive :: u8 >) ;
            impl ::subxt::StorageEntry for AssetIds {
                const PALLET: &'static str = "AssetRegistry";
                const STORAGE: &'static str = "AssetIds";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct AssetLocations(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for AssetLocations {
                const PALLET: &'static str = "AssetRegistry";
                const STORAGE: &'static str = "AssetLocations";
                type Value = ();
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                    ])
                }
            }
            pub struct LocationAssets();
            impl ::subxt::StorageEntry for LocationAssets {
                const PALLET: &'static str = "AssetRegistry";
                const STORAGE: &'static str = "LocationAssets";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![])
                }
            }
            pub struct AssetMetadataMap(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for AssetMetadataMap {
                const PALLET: &'static str = "AssetRegistry";
                const STORAGE: &'static str = "AssetMetadataMap";
                type Value = runtime_types :: pallet_asset_registry :: types :: AssetMetadata < runtime_types :: frame_support :: storage :: bounded_vec :: BoundedVec < :: core :: primitive :: u8 > > ;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                    ])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }                pub async fn assets (& self , _0 : :: core :: primitive :: u32 , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < :: core :: option :: Option < runtime_types :: pallet_asset_registry :: types :: AssetDetails < :: core :: primitive :: u32 , :: core :: primitive :: u128 , runtime_types :: frame_support :: storage :: bounded_vec :: BoundedVec < :: core :: primitive :: u8 > > > , :: subxt :: Error >{
                    let entry = Assets(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn assets_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Assets>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn next_asset_id(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::u32,
                    ::subxt::Error,
                > {
                    let entry = NextAssetId;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn asset_ids(
                    &self,
                    _0 : runtime_types :: frame_support :: storage :: bounded_vec :: BoundedVec < :: core :: primitive :: u8 >,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::Error,
                > {
                    let entry = AssetIds(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn asset_ids_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, AssetIds>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn asset_locations(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<()>,
                    ::subxt::Error,
                > {
                    let entry = AssetLocations(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn asset_locations_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, AssetLocations>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn location_assets(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::Error,
                > {
                    let entry = LocationAssets();
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn location_assets_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, LocationAssets>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }                pub async fn asset_metadata_map (& self , _0 : :: core :: primitive :: u32 , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < :: core :: option :: Option < runtime_types :: pallet_asset_registry :: types :: AssetMetadata < runtime_types :: frame_support :: storage :: bounded_vec :: BoundedVec < :: core :: primitive :: u8 > > > , :: subxt :: Error >{
                    let entry = AssetMetadataMap(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn asset_metadata_map_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, AssetMetadataMap>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod currencies {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Transfer {
                pub dest: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                pub currency_id: ::core::primitive::u32,
                #[codec(compact)]
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Call for Transfer {
                const PALLET: &'static str = "Currencies";
                const FUNCTION: &'static str = "transfer";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct TransferNativeCurrency {
                pub dest: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                #[codec(compact)]
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Call for TransferNativeCurrency {
                const PALLET: &'static str = "Currencies";
                const FUNCTION: &'static str = "transfer_native_currency";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct UpdateBalance {
                pub who: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                pub currency_id: ::core::primitive::u32,
                pub amount: ::core::primitive::i128,
            }
            impl ::subxt::Call for UpdateBalance {
                const PALLET: &'static str = "Currencies";
                const FUNCTION: &'static str = "update_balance";
            }
            pub struct TransactionApi<
                'a,
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            > {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn transfer(
                    &self,
                    dest: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    currency_id: ::core::primitive::u32,
                    amount: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, Transfer>
                {
                    let call = Transfer {
                        dest,
                        currency_id,
                        amount,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn transfer_native_currency(
                    &self,
                    dest: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    amount: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, TransferNativeCurrency>
                {
                    let call = TransferNativeCurrency { dest, amount };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn update_balance(
                    &self,
                    who: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    currency_id: ::core::primitive::u32,
                    amount: ::core::primitive::i128,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, UpdateBalance>
                {
                    let call = UpdateBalance {
                        who,
                        currency_id,
                        amount,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::orml_currencies::module::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Transferred(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Transferred {
                const PALLET: &'static str = "Currencies";
                const EVENT: &'static str = "Transferred";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct BalanceUpdated(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::i128,
            );
            impl ::subxt::Event for BalanceUpdated {
                const PALLET: &'static str = "Currencies";
                const EVENT: &'static str = "BalanceUpdated";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Deposited(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Deposited {
                const PALLET: &'static str = "Currencies";
                const EVENT: &'static str = "Deposited";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Withdrawn(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Withdrawn {
                const PALLET: &'static str = "Currencies";
                const EVENT: &'static str = "Withdrawn";
            }
        }
    }
    pub mod tokens {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Transfer {
                pub dest: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                pub currency_id: ::core::primitive::u32,
                #[codec(compact)]
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Call for Transfer {
                const PALLET: &'static str = "Tokens";
                const FUNCTION: &'static str = "transfer";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct TransferAll {
                pub dest: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                pub currency_id: ::core::primitive::u32,
                pub keep_alive: ::core::primitive::bool,
            }
            impl ::subxt::Call for TransferAll {
                const PALLET: &'static str = "Tokens";
                const FUNCTION: &'static str = "transfer_all";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct TransferKeepAlive {
                pub dest: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                pub currency_id: ::core::primitive::u32,
                #[codec(compact)]
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Call for TransferKeepAlive {
                const PALLET: &'static str = "Tokens";
                const FUNCTION: &'static str = "transfer_keep_alive";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ForceTransfer {
                pub source: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                pub dest: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                pub currency_id: ::core::primitive::u32,
                #[codec(compact)]
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Call for ForceTransfer {
                const PALLET: &'static str = "Tokens";
                const FUNCTION: &'static str = "force_transfer";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct SetBalance {
                pub who: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                pub currency_id: ::core::primitive::u32,
                #[codec(compact)]
                pub new_free: ::core::primitive::u128,
                #[codec(compact)]
                pub new_reserved: ::core::primitive::u128,
            }
            impl ::subxt::Call for SetBalance {
                const PALLET: &'static str = "Tokens";
                const FUNCTION: &'static str = "set_balance";
            }
            pub struct TransactionApi<
                'a,
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            > {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn transfer(
                    &self,
                    dest: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    currency_id: ::core::primitive::u32,
                    amount: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, Transfer>
                {
                    let call = Transfer {
                        dest,
                        currency_id,
                        amount,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn transfer_all(
                    &self,
                    dest: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    currency_id: ::core::primitive::u32,
                    keep_alive: ::core::primitive::bool,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, TransferAll>
                {
                    let call = TransferAll {
                        dest,
                        currency_id,
                        keep_alive,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn transfer_keep_alive(
                    &self,
                    dest: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    currency_id: ::core::primitive::u32,
                    amount: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, TransferKeepAlive>
                {
                    let call = TransferKeepAlive {
                        dest,
                        currency_id,
                        amount,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_transfer(
                    &self,
                    source: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    dest: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    currency_id: ::core::primitive::u32,
                    amount: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, ForceTransfer>
                {
                    let call = ForceTransfer {
                        source,
                        dest,
                        currency_id,
                        amount,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_balance(
                    &self,
                    who: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    currency_id: ::core::primitive::u32,
                    new_free: ::core::primitive::u128,
                    new_reserved: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, SetBalance>
                {
                    let call = SetBalance {
                        who,
                        currency_id,
                        new_free,
                        new_reserved,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::orml_tokens::module::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Endowed(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Endowed {
                const PALLET: &'static str = "Tokens";
                const EVENT: &'static str = "Endowed";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct DustLost(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for DustLost {
                const PALLET: &'static str = "Tokens";
                const EVENT: &'static str = "DustLost";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Transfer(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Transfer {
                const PALLET: &'static str = "Tokens";
                const EVENT: &'static str = "Transfer";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Reserved(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Reserved {
                const PALLET: &'static str = "Tokens";
                const EVENT: &'static str = "Reserved";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Unreserved(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Unreserved {
                const PALLET: &'static str = "Tokens";
                const EVENT: &'static str = "Unreserved";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct BalanceSet(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for BalanceSet {
                const PALLET: &'static str = "Tokens";
                const EVENT: &'static str = "BalanceSet";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct TotalIssuance(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for TotalIssuance {
                const PALLET: &'static str = "Tokens";
                const STORAGE: &'static str = "TotalIssuance";
                type Value = ::core::primitive::u128;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                    ])
                }
            }
            pub struct Locks(
                ::subxt::sp_core::crypto::AccountId32,
                ::core::primitive::u32,
            );
            impl ::subxt::StorageEntry for Locks {
                const PALLET: &'static str = "Tokens";
                const STORAGE: &'static str = "Locks";
                type Value = runtime_types :: frame_support :: storage :: bounded_vec :: BoundedVec < runtime_types :: orml_tokens :: BalanceLock < :: core :: primitive :: u128 > > ;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                    ])
                }
            }
            pub struct Accounts(
                ::subxt::sp_core::crypto::AccountId32,
                ::core::primitive::u32,
            );
            impl ::subxt::StorageEntry for Accounts {
                const PALLET: &'static str = "Tokens";
                const STORAGE: &'static str = "Accounts";
                type Value = runtime_types::orml_tokens::AccountData<
                    ::core::primitive::u128,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                    ])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn total_issuance(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::u128,
                    ::subxt::Error,
                > {
                    let entry = TotalIssuance(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn total_issuance_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, TotalIssuance>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }                pub async fn locks (& self , _0 : :: subxt :: sp_core :: crypto :: AccountId32 , _1 : :: core :: primitive :: u32 , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < runtime_types :: frame_support :: storage :: bounded_vec :: BoundedVec < runtime_types :: orml_tokens :: BalanceLock < :: core :: primitive :: u128 > > , :: subxt :: Error >{
                    let entry = Locks(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn locks_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Locks>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn accounts(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    _1: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::orml_tokens::AccountData<
                        ::core::primitive::u128,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Accounts(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn accounts_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Accounts>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod token_wrapper {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct SetWrappingFee {
                pub fee: ::core::primitive::u128,
            }
            impl ::subxt::Call for SetWrappingFee {
                const PALLET: &'static str = "TokenWrapper";
                const FUNCTION: &'static str = "set_wrapping_fee";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Wrap {
                pub from_asset_id: ::core::primitive::u32,
                pub into_pool_share_id: ::core::primitive::u32,
                pub amount: ::core::primitive::u128,
                pub recipient: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Call for Wrap {
                const PALLET: &'static str = "TokenWrapper";
                const FUNCTION: &'static str = "wrap";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Unwrap {
                pub from_pool_share_id: ::core::primitive::u32,
                pub into_asset_id: ::core::primitive::u32,
                pub amount: ::core::primitive::u128,
                pub recipient: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Call for Unwrap {
                const PALLET: &'static str = "TokenWrapper";
                const FUNCTION: &'static str = "unwrap";
            }
            pub struct TransactionApi<
                'a,
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            > {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn set_wrapping_fee(
                    &self,
                    fee: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, SetWrappingFee>
                {
                    let call = SetWrappingFee { fee };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn wrap(
                    &self,
                    from_asset_id: ::core::primitive::u32,
                    into_pool_share_id: ::core::primitive::u32,
                    amount: ::core::primitive::u128,
                    recipient: ::subxt::sp_core::crypto::AccountId32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, Wrap>
                {
                    let call = Wrap {
                        from_asset_id,
                        into_pool_share_id,
                        amount,
                        recipient,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn unwrap(
                    &self,
                    from_pool_share_id: ::core::primitive::u32,
                    into_asset_id: ::core::primitive::u32,
                    amount: ::core::primitive::u128,
                    recipient: ::subxt::sp_core::crypto::AccountId32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, Unwrap>
                {
                    let call = Unwrap {
                        from_pool_share_id,
                        into_asset_id,
                        amount,
                        recipient,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_token_wrapper::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct WrappedToken {
                pub pool_share_asset: ::core::primitive::u32,
                pub asset_id: ::core::primitive::u32,
                pub amount: ::core::primitive::u128,
                pub recipient: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for WrappedToken {
                const PALLET: &'static str = "TokenWrapper";
                const EVENT: &'static str = "WrappedToken";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct UnwrappedToken {
                pub pool_share_asset: ::core::primitive::u32,
                pub asset_id: ::core::primitive::u32,
                pub amount: ::core::primitive::u128,
                pub recipient: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for UnwrappedToken {
                const PALLET: &'static str = "TokenWrapper";
                const EVENT: &'static str = "UnwrappedToken";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct UpdatedWrappingFeePercent {
                pub wrapping_fee_percent: ::core::primitive::u128,
            }
            impl ::subxt::Event for UpdatedWrappingFeePercent {
                const PALLET: &'static str = "TokenWrapper";
                const EVENT: &'static str = "UpdatedWrappingFeePercent";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct WrappingFeePercent;
            impl ::subxt::StorageEntry for WrappingFeePercent {
                const PALLET: &'static str = "TokenWrapper";
                const STORAGE: &'static str = "WrappingFeePercent";
                type Value = ::core::primitive::u128;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn wrapping_fee_percent(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::u128,
                    ::subxt::Error,
                > {
                    let entry = WrappingFeePercent;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod verifier_bn254 {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct SetParameters {
                pub parameters: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for SetParameters {
                const PALLET: &'static str = "VerifierBn254";
                const FUNCTION: &'static str = "set_parameters";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct SetMaintainer {
                pub new_maintainer: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Call for SetMaintainer {
                const PALLET: &'static str = "VerifierBn254";
                const FUNCTION: &'static str = "set_maintainer";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ForceSetParameters {
                pub parameters: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for ForceSetParameters {
                const PALLET: &'static str = "VerifierBn254";
                const FUNCTION: &'static str = "force_set_parameters";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ForceSetMaintainer {
                pub new_maintainer: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Call for ForceSetMaintainer {
                const PALLET: &'static str = "VerifierBn254";
                const FUNCTION: &'static str = "force_set_maintainer";
            }
            pub struct TransactionApi<
                'a,
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            > {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn set_parameters(
                    &self,
                    parameters: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, SetParameters>
                {
                    let call = SetParameters { parameters };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_maintainer(
                    &self,
                    new_maintainer: ::subxt::sp_core::crypto::AccountId32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, SetMaintainer>
                {
                    let call = SetMaintainer { new_maintainer };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_set_parameters(
                    &self,
                    parameters: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, ForceSetParameters>
                {
                    let call = ForceSetParameters { parameters };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_set_maintainer(
                    &self,
                    new_maintainer: ::subxt::sp_core::crypto::AccountId32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, ForceSetMaintainer>
                {
                    let call = ForceSetMaintainer { new_maintainer };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_verifier::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ParametersSet {
                pub who: ::subxt::sp_core::crypto::AccountId32,
                pub parameters: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Event for ParametersSet {
                const PALLET: &'static str = "VerifierBn254";
                const EVENT: &'static str = "ParametersSet";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct MaintainerSet {
                pub old_maintainer: ::subxt::sp_core::crypto::AccountId32,
                pub new_maintainer: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for MaintainerSet {
                const PALLET: &'static str = "VerifierBn254";
                const EVENT: &'static str = "MaintainerSet";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Parameters;
            impl ::subxt::StorageEntry for Parameters {
                const PALLET: &'static str = "VerifierBn254";
                const STORAGE: &'static str = "Parameters";
                type Value = ::std::vec::Vec<::core::primitive::u8>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Deposit;
            impl ::subxt::StorageEntry for Deposit {
                const PALLET: &'static str = "VerifierBn254";
                const STORAGE: &'static str = "Deposit";
                type Value = ::core::option::Option<
                    runtime_types::darkwebb_primitives::types::DepositDetails<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    >,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Maintainer;
            impl ::subxt::StorageEntry for Maintainer {
                const PALLET: &'static str = "VerifierBn254";
                const STORAGE: &'static str = "Maintainer";
                type Value = ::subxt::sp_core::crypto::AccountId32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn parameters(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::subxt::Error,
                > {
                    let entry = Parameters;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }                pub async fn deposit (& self , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < :: core :: option :: Option < runtime_types :: darkwebb_primitives :: types :: DepositDetails < :: subxt :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u128 > > , :: subxt :: Error >{
                    let entry = Deposit;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn maintainer(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::subxt::Error,
                > {
                    let entry = Maintainer;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod verifier_bls381 {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct SetParameters {
                pub parameters: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for SetParameters {
                const PALLET: &'static str = "VerifierBls381";
                const FUNCTION: &'static str = "set_parameters";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct SetMaintainer {
                pub new_maintainer: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Call for SetMaintainer {
                const PALLET: &'static str = "VerifierBls381";
                const FUNCTION: &'static str = "set_maintainer";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ForceSetParameters {
                pub parameters: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for ForceSetParameters {
                const PALLET: &'static str = "VerifierBls381";
                const FUNCTION: &'static str = "force_set_parameters";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ForceSetMaintainer {
                pub new_maintainer: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Call for ForceSetMaintainer {
                const PALLET: &'static str = "VerifierBls381";
                const FUNCTION: &'static str = "force_set_maintainer";
            }
            pub struct TransactionApi<
                'a,
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            > {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn set_parameters(
                    &self,
                    parameters: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, SetParameters>
                {
                    let call = SetParameters { parameters };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_maintainer(
                    &self,
                    new_maintainer: ::subxt::sp_core::crypto::AccountId32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, SetMaintainer>
                {
                    let call = SetMaintainer { new_maintainer };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_set_parameters(
                    &self,
                    parameters: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, ForceSetParameters>
                {
                    let call = ForceSetParameters { parameters };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_set_maintainer(
                    &self,
                    new_maintainer: ::subxt::sp_core::crypto::AccountId32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, ForceSetMaintainer>
                {
                    let call = ForceSetMaintainer { new_maintainer };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_verifier::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ParametersSet {
                pub who: ::subxt::sp_core::crypto::AccountId32,
                pub parameters: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Event for ParametersSet {
                const PALLET: &'static str = "VerifierBls381";
                const EVENT: &'static str = "ParametersSet";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct MaintainerSet {
                pub old_maintainer: ::subxt::sp_core::crypto::AccountId32,
                pub new_maintainer: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for MaintainerSet {
                const PALLET: &'static str = "VerifierBls381";
                const EVENT: &'static str = "MaintainerSet";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Parameters;
            impl ::subxt::StorageEntry for Parameters {
                const PALLET: &'static str = "VerifierBls381";
                const STORAGE: &'static str = "Parameters";
                type Value = ::std::vec::Vec<::core::primitive::u8>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Deposit;
            impl ::subxt::StorageEntry for Deposit {
                const PALLET: &'static str = "VerifierBls381";
                const STORAGE: &'static str = "Deposit";
                type Value = ::core::option::Option<
                    runtime_types::darkwebb_primitives::types::DepositDetails<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    >,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Maintainer;
            impl ::subxt::StorageEntry for Maintainer {
                const PALLET: &'static str = "VerifierBls381";
                const STORAGE: &'static str = "Maintainer";
                type Value = ::subxt::sp_core::crypto::AccountId32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn parameters(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::subxt::Error,
                > {
                    let entry = Parameters;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }                pub async fn deposit (& self , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < :: core :: option :: Option < runtime_types :: darkwebb_primitives :: types :: DepositDetails < :: subxt :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u128 > > , :: subxt :: Error >{
                    let entry = Deposit;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn maintainer(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::subxt::Error,
                > {
                    let entry = Maintainer;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod merkle_tree_bn254 {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Create {
                pub depth: ::core::primitive::u8,
            }
            impl ::subxt::Call for Create {
                const PALLET: &'static str = "MerkleTreeBn254";
                const FUNCTION: &'static str = "create";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Insert {
                pub tree_id: ::core::primitive::u32,
                pub leaf: runtime_types::darkwebb_standalone_runtime::Element,
            }
            impl ::subxt::Call for Insert {
                const PALLET: &'static str = "MerkleTreeBn254";
                const FUNCTION: &'static str = "insert";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct SetMaintainer {
                pub new_maintainer: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Call for SetMaintainer {
                const PALLET: &'static str = "MerkleTreeBn254";
                const FUNCTION: &'static str = "set_maintainer";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ForceSetMaintainer {
                pub new_maintainer: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Call for ForceSetMaintainer {
                const PALLET: &'static str = "MerkleTreeBn254";
                const FUNCTION: &'static str = "force_set_maintainer";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ForceSetDefaultHashes {
                pub default_hashes: ::std::vec::Vec<
                    runtime_types::darkwebb_standalone_runtime::Element,
                >,
            }
            impl ::subxt::Call for ForceSetDefaultHashes {
                const PALLET: &'static str = "MerkleTreeBn254";
                const FUNCTION: &'static str = "force_set_default_hashes";
            }
            pub struct TransactionApi<
                'a,
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            > {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn create(
                    &self,
                    depth: ::core::primitive::u8,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, Create>
                {
                    let call = Create { depth };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn insert(
                    &self,
                    tree_id: ::core::primitive::u32,
                    leaf: runtime_types::darkwebb_standalone_runtime::Element,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, Insert>
                {
                    let call = Insert { tree_id, leaf };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_maintainer(
                    &self,
                    new_maintainer: ::subxt::sp_core::crypto::AccountId32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, SetMaintainer>
                {
                    let call = SetMaintainer { new_maintainer };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_set_maintainer(
                    &self,
                    new_maintainer: ::subxt::sp_core::crypto::AccountId32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, ForceSetMaintainer>
                {
                    let call = ForceSetMaintainer { new_maintainer };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_set_default_hashes(
                    &self,
                    default_hashes: ::std::vec::Vec<
                        runtime_types::darkwebb_standalone_runtime::Element,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, ForceSetDefaultHashes>
                {
                    let call = ForceSetDefaultHashes { default_hashes };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_mt::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct MaintainerSet {
                pub old_maintainer: ::subxt::sp_core::crypto::AccountId32,
                pub new_maintainer: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for MaintainerSet {
                const PALLET: &'static str = "MerkleTreeBn254";
                const EVENT: &'static str = "MaintainerSet";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct TreeCreation {
                pub tree_id: ::core::primitive::u32,
                pub who: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for TreeCreation {
                const PALLET: &'static str = "MerkleTreeBn254";
                const EVENT: &'static str = "TreeCreation";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct LeafInsertion {
                pub tree_id: ::core::primitive::u32,
                pub leaf_index: ::core::primitive::u32,
                pub leaf: runtime_types::darkwebb_standalone_runtime::Element,
            }
            impl ::subxt::Event for LeafInsertion {
                const PALLET: &'static str = "MerkleTreeBn254";
                const EVENT: &'static str = "LeafInsertion";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Deposit;
            impl ::subxt::StorageEntry for Deposit {
                const PALLET: &'static str = "MerkleTreeBn254";
                const STORAGE: &'static str = "Deposit";
                type Value = ::core::option::Option<
                    runtime_types::darkwebb_primitives::types::DepositDetails<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    >,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Maintainer;
            impl ::subxt::StorageEntry for Maintainer {
                const PALLET: &'static str = "MerkleTreeBn254";
                const STORAGE: &'static str = "Maintainer";
                type Value = ::subxt::sp_core::crypto::AccountId32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NextTreeId;
            impl ::subxt::StorageEntry for NextTreeId {
                const PALLET: &'static str = "MerkleTreeBn254";
                const STORAGE: &'static str = "NextTreeId";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Trees(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for Trees {
                const PALLET: &'static str = "MerkleTreeBn254";
                const STORAGE: &'static str = "Trees";
                type Value = runtime_types::pallet_mt::types::TreeMetadata<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                    runtime_types::darkwebb_standalone_runtime::Element,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct DefaultHashes;
            impl ::subxt::StorageEntry for DefaultHashes {
                const PALLET: &'static str = "MerkleTreeBn254";
                const STORAGE: &'static str = "DefaultHashes";
                type Value = ::std::vec::Vec<
                    runtime_types::darkwebb_standalone_runtime::Element,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Leaves(::core::primitive::u32, ::core::primitive::u32);
            impl ::subxt::StorageEntry for Leaves {
                const PALLET: &'static str = "MerkleTreeBn254";
                const STORAGE: &'static str = "Leaves";
                type Value =
                    runtime_types::darkwebb_standalone_runtime::Element;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct NextRootIndex;
            impl ::subxt::StorageEntry for NextRootIndex {
                const PALLET: &'static str = "MerkleTreeBn254";
                const STORAGE: &'static str = "NextRootIndex";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NextLeafIndex(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for NextLeafIndex {
                const PALLET: &'static str = "MerkleTreeBn254";
                const STORAGE: &'static str = "NextLeafIndex";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct CachedRoots(
                ::core::primitive::u32,
                ::core::primitive::u32,
            );
            impl ::subxt::StorageEntry for CachedRoots {
                const PALLET: &'static str = "MerkleTreeBn254";
                const STORAGE: &'static str = "CachedRoots";
                type Value =
                    runtime_types::darkwebb_standalone_runtime::Element;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }                pub async fn deposit (& self , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < :: core :: option :: Option < runtime_types :: darkwebb_primitives :: types :: DepositDetails < :: subxt :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u128 > > , :: subxt :: Error >{
                    let entry = Deposit;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn maintainer(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::subxt::Error,
                > {
                    let entry = Maintainer;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn next_tree_id(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::u32,
                    ::subxt::Error,
                > {
                    let entry = NextTreeId;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn trees(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_mt::types::TreeMetadata<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                        runtime_types::darkwebb_standalone_runtime::Element,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Trees(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn trees_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Trees>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn default_hashes(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<
                        runtime_types::darkwebb_standalone_runtime::Element,
                    >,
                    ::subxt::Error,
                > {
                    let entry = DefaultHashes;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn leaves(
                    &self,
                    _0: ::core::primitive::u32,
                    _1: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::darkwebb_standalone_runtime::Element,
                    ::subxt::Error,
                > {
                    let entry = Leaves(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn leaves_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Leaves>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn next_root_index(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::u32,
                    ::subxt::Error,
                > {
                    let entry = NextRootIndex;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn next_leaf_index(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::u32,
                    ::subxt::Error,
                > {
                    let entry = NextLeafIndex(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn next_leaf_index_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, NextLeafIndex>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn cached_roots(
                    &self,
                    _0: ::core::primitive::u32,
                    _1: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::darkwebb_standalone_runtime::Element,
                    ::subxt::Error,
                > {
                    let entry = CachedRoots(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn cached_roots_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, CachedRoots>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod merkle_tree_bls381 {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Create {
                pub depth: ::core::primitive::u8,
            }
            impl ::subxt::Call for Create {
                const PALLET: &'static str = "MerkleTreeBls381";
                const FUNCTION: &'static str = "create";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Insert {
                pub tree_id: ::core::primitive::u32,
                pub leaf: runtime_types::darkwebb_standalone_runtime::Element,
            }
            impl ::subxt::Call for Insert {
                const PALLET: &'static str = "MerkleTreeBls381";
                const FUNCTION: &'static str = "insert";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct SetMaintainer {
                pub new_maintainer: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Call for SetMaintainer {
                const PALLET: &'static str = "MerkleTreeBls381";
                const FUNCTION: &'static str = "set_maintainer";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ForceSetMaintainer {
                pub new_maintainer: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Call for ForceSetMaintainer {
                const PALLET: &'static str = "MerkleTreeBls381";
                const FUNCTION: &'static str = "force_set_maintainer";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ForceSetDefaultHashes {
                pub default_hashes: ::std::vec::Vec<
                    runtime_types::darkwebb_standalone_runtime::Element,
                >,
            }
            impl ::subxt::Call for ForceSetDefaultHashes {
                const PALLET: &'static str = "MerkleTreeBls381";
                const FUNCTION: &'static str = "force_set_default_hashes";
            }
            pub struct TransactionApi<
                'a,
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            > {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn create(
                    &self,
                    depth: ::core::primitive::u8,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, Create>
                {
                    let call = Create { depth };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn insert(
                    &self,
                    tree_id: ::core::primitive::u32,
                    leaf: runtime_types::darkwebb_standalone_runtime::Element,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, Insert>
                {
                    let call = Insert { tree_id, leaf };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_maintainer(
                    &self,
                    new_maintainer: ::subxt::sp_core::crypto::AccountId32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, SetMaintainer>
                {
                    let call = SetMaintainer { new_maintainer };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_set_maintainer(
                    &self,
                    new_maintainer: ::subxt::sp_core::crypto::AccountId32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, ForceSetMaintainer>
                {
                    let call = ForceSetMaintainer { new_maintainer };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_set_default_hashes(
                    &self,
                    default_hashes: ::std::vec::Vec<
                        runtime_types::darkwebb_standalone_runtime::Element,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, ForceSetDefaultHashes>
                {
                    let call = ForceSetDefaultHashes { default_hashes };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_mt::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct MaintainerSet {
                pub old_maintainer: ::subxt::sp_core::crypto::AccountId32,
                pub new_maintainer: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for MaintainerSet {
                const PALLET: &'static str = "MerkleTreeBls381";
                const EVENT: &'static str = "MaintainerSet";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct TreeCreation {
                pub tree_id: ::core::primitive::u32,
                pub who: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for TreeCreation {
                const PALLET: &'static str = "MerkleTreeBls381";
                const EVENT: &'static str = "TreeCreation";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct LeafInsertion {
                pub tree_id: ::core::primitive::u32,
                pub leaf_index: ::core::primitive::u32,
                pub leaf: runtime_types::darkwebb_standalone_runtime::Element,
            }
            impl ::subxt::Event for LeafInsertion {
                const PALLET: &'static str = "MerkleTreeBls381";
                const EVENT: &'static str = "LeafInsertion";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Deposit;
            impl ::subxt::StorageEntry for Deposit {
                const PALLET: &'static str = "MerkleTreeBls381";
                const STORAGE: &'static str = "Deposit";
                type Value = ::core::option::Option<
                    runtime_types::darkwebb_primitives::types::DepositDetails<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    >,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Maintainer;
            impl ::subxt::StorageEntry for Maintainer {
                const PALLET: &'static str = "MerkleTreeBls381";
                const STORAGE: &'static str = "Maintainer";
                type Value = ::subxt::sp_core::crypto::AccountId32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NextTreeId;
            impl ::subxt::StorageEntry for NextTreeId {
                const PALLET: &'static str = "MerkleTreeBls381";
                const STORAGE: &'static str = "NextTreeId";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Trees(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for Trees {
                const PALLET: &'static str = "MerkleTreeBls381";
                const STORAGE: &'static str = "Trees";
                type Value = runtime_types::pallet_mt::types::TreeMetadata<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                    runtime_types::darkwebb_standalone_runtime::Element,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct DefaultHashes;
            impl ::subxt::StorageEntry for DefaultHashes {
                const PALLET: &'static str = "MerkleTreeBls381";
                const STORAGE: &'static str = "DefaultHashes";
                type Value = ::std::vec::Vec<
                    runtime_types::darkwebb_standalone_runtime::Element,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Leaves(::core::primitive::u32, ::core::primitive::u32);
            impl ::subxt::StorageEntry for Leaves {
                const PALLET: &'static str = "MerkleTreeBls381";
                const STORAGE: &'static str = "Leaves";
                type Value =
                    runtime_types::darkwebb_standalone_runtime::Element;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct NextRootIndex;
            impl ::subxt::StorageEntry for NextRootIndex {
                const PALLET: &'static str = "MerkleTreeBls381";
                const STORAGE: &'static str = "NextRootIndex";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NextLeafIndex(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for NextLeafIndex {
                const PALLET: &'static str = "MerkleTreeBls381";
                const STORAGE: &'static str = "NextLeafIndex";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct CachedRoots(
                ::core::primitive::u32,
                ::core::primitive::u32,
            );
            impl ::subxt::StorageEntry for CachedRoots {
                const PALLET: &'static str = "MerkleTreeBls381";
                const STORAGE: &'static str = "CachedRoots";
                type Value =
                    runtime_types::darkwebb_standalone_runtime::Element;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }                pub async fn deposit (& self , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < :: core :: option :: Option < runtime_types :: darkwebb_primitives :: types :: DepositDetails < :: subxt :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u128 > > , :: subxt :: Error >{
                    let entry = Deposit;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn maintainer(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::subxt::Error,
                > {
                    let entry = Maintainer;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn next_tree_id(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::u32,
                    ::subxt::Error,
                > {
                    let entry = NextTreeId;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn trees(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_mt::types::TreeMetadata<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                        runtime_types::darkwebb_standalone_runtime::Element,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Trees(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn trees_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Trees>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn default_hashes(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<
                        runtime_types::darkwebb_standalone_runtime::Element,
                    >,
                    ::subxt::Error,
                > {
                    let entry = DefaultHashes;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn leaves(
                    &self,
                    _0: ::core::primitive::u32,
                    _1: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::darkwebb_standalone_runtime::Element,
                    ::subxt::Error,
                > {
                    let entry = Leaves(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn leaves_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Leaves>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn next_root_index(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::u32,
                    ::subxt::Error,
                > {
                    let entry = NextRootIndex;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn next_leaf_index(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::u32,
                    ::subxt::Error,
                > {
                    let entry = NextLeafIndex(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn next_leaf_index_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, NextLeafIndex>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn cached_roots(
                    &self,
                    _0: ::core::primitive::u32,
                    _1: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::darkwebb_standalone_runtime::Element,
                    ::subxt::Error,
                > {
                    let entry = CachedRoots(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn cached_roots_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, CachedRoots>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod linkable_tree_bn254 {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Create {
                pub max_edges: ::core::primitive::u32,
                pub depth: ::core::primitive::u8,
            }
            impl ::subxt::Call for Create {
                const PALLET: &'static str = "LinkableTreeBn254";
                const FUNCTION: &'static str = "create";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct SetMaintainer {
                pub new_maintainer: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Call for SetMaintainer {
                const PALLET: &'static str = "LinkableTreeBn254";
                const FUNCTION: &'static str = "set_maintainer";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ForceSetMaintainer {
                pub new_maintainer: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Call for ForceSetMaintainer {
                const PALLET: &'static str = "LinkableTreeBn254";
                const FUNCTION: &'static str = "force_set_maintainer";
            }
            pub struct TransactionApi<
                'a,
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            > {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn create(
                    &self,
                    max_edges: ::core::primitive::u32,
                    depth: ::core::primitive::u8,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, Create>
                {
                    let call = Create { max_edges, depth };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_maintainer(
                    &self,
                    new_maintainer: ::subxt::sp_core::crypto::AccountId32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, SetMaintainer>
                {
                    let call = SetMaintainer { new_maintainer };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_set_maintainer(
                    &self,
                    new_maintainer: ::subxt::sp_core::crypto::AccountId32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, ForceSetMaintainer>
                {
                    let call = ForceSetMaintainer { new_maintainer };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_linkable_tree::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct MaintainerSet {
                pub old_maintainer: ::subxt::sp_core::crypto::AccountId32,
                pub new_maintainer: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for MaintainerSet {
                const PALLET: &'static str = "LinkableTreeBn254";
                const EVENT: &'static str = "MaintainerSet";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct LinkableTreeCreation {
                pub tree_id: ::core::primitive::u32,
            }
            impl ::subxt::Event for LinkableTreeCreation {
                const PALLET: &'static str = "LinkableTreeBn254";
                const EVENT: &'static str = "LinkableTreeCreation";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Maintainer;
            impl ::subxt::StorageEntry for Maintainer {
                const PALLET: &'static str = "LinkableTreeBn254";
                const STORAGE: &'static str = "Maintainer";
                type Value = ::subxt::sp_core::crypto::AccountId32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct MaxEdges(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for MaxEdges {
                const PALLET: &'static str = "LinkableTreeBn254";
                const STORAGE: &'static str = "MaxEdges";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct EdgeList(::core::primitive::u32, ::core::primitive::u32);
            impl ::subxt::StorageEntry for EdgeList {
                const PALLET: &'static str = "LinkableTreeBn254";
                const STORAGE: &'static str = "EdgeList";
                type Value =
                    runtime_types::pallet_linkable_tree::types::EdgeMetadata<
                        ::core::primitive::u32,
                        runtime_types::darkwebb_standalone_runtime::Element,
                        ::core::primitive::u32,
                    >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct LinkableTreeHasEdge(
                ::core::primitive::u32,
                ::core::primitive::u32,
            );
            impl ::subxt::StorageEntry for LinkableTreeHasEdge {
                const PALLET: &'static str = "LinkableTreeBn254";
                const STORAGE: &'static str = "LinkableTreeHasEdge";
                type Value = ::core::primitive::bool;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct NeighborRoots(
                (::core::primitive::u32, ::core::primitive::u32),
                ::core::primitive::u32,
            );
            impl ::subxt::StorageEntry for NeighborRoots {
                const PALLET: &'static str = "LinkableTreeBn254";
                const STORAGE: &'static str = "NeighborRoots";
                type Value =
                    runtime_types::darkwebb_standalone_runtime::Element;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct CurrentNeighborRootIndex(
                ::core::primitive::u32,
                ::core::primitive::u32,
            );
            impl ::subxt::StorageEntry for CurrentNeighborRootIndex {
                const PALLET: &'static str = "LinkableTreeBn254";
                const STORAGE: &'static str = "CurrentNeighborRootIndex";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn maintainer(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::subxt::Error,
                > {
                    let entry = Maintainer;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn max_edges(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::u32,
                    ::subxt::Error,
                > {
                    let entry = MaxEdges(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn max_edges_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, MaxEdges>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn edge_list(
                    &self,
                    _0: ::core::primitive::u32,
                    _1: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_linkable_tree::types::EdgeMetadata<
                        ::core::primitive::u32,
                        runtime_types::darkwebb_standalone_runtime::Element,
                        ::core::primitive::u32,
                    >,
                    ::subxt::Error,
                > {
                    let entry = EdgeList(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn edge_list_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, EdgeList>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn linkable_tree_has_edge(
                    &self,
                    _0: ::core::primitive::u32,
                    _1: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::bool,
                    ::subxt::Error,
                > {
                    let entry = LinkableTreeHasEdge(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn linkable_tree_has_edge_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, LinkableTreeHasEdge>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn neighbor_roots(
                    &self,
                    _0: (::core::primitive::u32, ::core::primitive::u32),
                    _1: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::darkwebb_standalone_runtime::Element,
                    >,
                    ::subxt::Error,
                > {
                    let entry = NeighborRoots(_0, _1);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn neighbor_roots_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, NeighborRoots>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn current_neighbor_root_index(
                    &self,
                    _0: ::core::primitive::u32,
                    _1: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::u32,
                    ::subxt::Error,
                > {
                    let entry = CurrentNeighborRootIndex(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn current_neighbor_root_index_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, CurrentNeighborRootIndex>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod linkable_tree_bls381 {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Create {
                pub max_edges: ::core::primitive::u32,
                pub depth: ::core::primitive::u8,
            }
            impl ::subxt::Call for Create {
                const PALLET: &'static str = "LinkableTreeBls381";
                const FUNCTION: &'static str = "create";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct SetMaintainer {
                pub new_maintainer: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Call for SetMaintainer {
                const PALLET: &'static str = "LinkableTreeBls381";
                const FUNCTION: &'static str = "set_maintainer";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ForceSetMaintainer {
                pub new_maintainer: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Call for ForceSetMaintainer {
                const PALLET: &'static str = "LinkableTreeBls381";
                const FUNCTION: &'static str = "force_set_maintainer";
            }
            pub struct TransactionApi<
                'a,
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            > {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn create(
                    &self,
                    max_edges: ::core::primitive::u32,
                    depth: ::core::primitive::u8,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, Create>
                {
                    let call = Create { max_edges, depth };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_maintainer(
                    &self,
                    new_maintainer: ::subxt::sp_core::crypto::AccountId32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, SetMaintainer>
                {
                    let call = SetMaintainer { new_maintainer };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_set_maintainer(
                    &self,
                    new_maintainer: ::subxt::sp_core::crypto::AccountId32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, ForceSetMaintainer>
                {
                    let call = ForceSetMaintainer { new_maintainer };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_linkable_tree::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct MaintainerSet {
                pub old_maintainer: ::subxt::sp_core::crypto::AccountId32,
                pub new_maintainer: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for MaintainerSet {
                const PALLET: &'static str = "LinkableTreeBls381";
                const EVENT: &'static str = "MaintainerSet";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct LinkableTreeCreation {
                pub tree_id: ::core::primitive::u32,
            }
            impl ::subxt::Event for LinkableTreeCreation {
                const PALLET: &'static str = "LinkableTreeBls381";
                const EVENT: &'static str = "LinkableTreeCreation";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Maintainer;
            impl ::subxt::StorageEntry for Maintainer {
                const PALLET: &'static str = "LinkableTreeBls381";
                const STORAGE: &'static str = "Maintainer";
                type Value = ::subxt::sp_core::crypto::AccountId32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct MaxEdges(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for MaxEdges {
                const PALLET: &'static str = "LinkableTreeBls381";
                const STORAGE: &'static str = "MaxEdges";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct EdgeList(::core::primitive::u32, ::core::primitive::u32);
            impl ::subxt::StorageEntry for EdgeList {
                const PALLET: &'static str = "LinkableTreeBls381";
                const STORAGE: &'static str = "EdgeList";
                type Value =
                    runtime_types::pallet_linkable_tree::types::EdgeMetadata<
                        ::core::primitive::u32,
                        runtime_types::darkwebb_standalone_runtime::Element,
                        ::core::primitive::u32,
                    >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct LinkableTreeHasEdge(
                ::core::primitive::u32,
                ::core::primitive::u32,
            );
            impl ::subxt::StorageEntry for LinkableTreeHasEdge {
                const PALLET: &'static str = "LinkableTreeBls381";
                const STORAGE: &'static str = "LinkableTreeHasEdge";
                type Value = ::core::primitive::bool;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct NeighborRoots(
                (::core::primitive::u32, ::core::primitive::u32),
                ::core::primitive::u32,
            );
            impl ::subxt::StorageEntry for NeighborRoots {
                const PALLET: &'static str = "LinkableTreeBls381";
                const STORAGE: &'static str = "NeighborRoots";
                type Value =
                    runtime_types::darkwebb_standalone_runtime::Element;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct CurrentNeighborRootIndex(
                ::core::primitive::u32,
                ::core::primitive::u32,
            );
            impl ::subxt::StorageEntry for CurrentNeighborRootIndex {
                const PALLET: &'static str = "LinkableTreeBls381";
                const STORAGE: &'static str = "CurrentNeighborRootIndex";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn maintainer(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::subxt::Error,
                > {
                    let entry = Maintainer;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn max_edges(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::u32,
                    ::subxt::Error,
                > {
                    let entry = MaxEdges(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn max_edges_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, MaxEdges>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn edge_list(
                    &self,
                    _0: ::core::primitive::u32,
                    _1: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_linkable_tree::types::EdgeMetadata<
                        ::core::primitive::u32,
                        runtime_types::darkwebb_standalone_runtime::Element,
                        ::core::primitive::u32,
                    >,
                    ::subxt::Error,
                > {
                    let entry = EdgeList(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn edge_list_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, EdgeList>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn linkable_tree_has_edge(
                    &self,
                    _0: ::core::primitive::u32,
                    _1: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::bool,
                    ::subxt::Error,
                > {
                    let entry = LinkableTreeHasEdge(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn linkable_tree_has_edge_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, LinkableTreeHasEdge>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn neighbor_roots(
                    &self,
                    _0: (::core::primitive::u32, ::core::primitive::u32),
                    _1: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::darkwebb_standalone_runtime::Element,
                    >,
                    ::subxt::Error,
                > {
                    let entry = NeighborRoots(_0, _1);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn neighbor_roots_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, NeighborRoots>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn current_neighbor_root_index(
                    &self,
                    _0: ::core::primitive::u32,
                    _1: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::u32,
                    ::subxt::Error,
                > {
                    let entry = CurrentNeighborRootIndex(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn current_neighbor_root_index_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, CurrentNeighborRootIndex>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod mixer_bn254 {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Create {
                pub deposit_size: ::core::primitive::u128,
                pub depth: ::core::primitive::u8,
                pub asset: ::core::primitive::u32,
            }
            impl ::subxt::Call for Create {
                const PALLET: &'static str = "MixerBn254";
                const FUNCTION: &'static str = "create";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct SetMaintainer {
                pub new_maintainer: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Call for SetMaintainer {
                const PALLET: &'static str = "MixerBn254";
                const FUNCTION: &'static str = "set_maintainer";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ForceSetMaintainer {
                pub new_maintainer: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Call for ForceSetMaintainer {
                const PALLET: &'static str = "MixerBn254";
                const FUNCTION: &'static str = "force_set_maintainer";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Deposit {
                pub tree_id: ::core::primitive::u32,
                pub leaf: runtime_types::darkwebb_standalone_runtime::Element,
            }
            impl ::subxt::Call for Deposit {
                const PALLET: &'static str = "MixerBn254";
                const FUNCTION: &'static str = "deposit";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Withdraw {
                pub id: ::core::primitive::u32,
                pub proof_bytes: ::std::vec::Vec<::core::primitive::u8>,
                pub root: runtime_types::darkwebb_standalone_runtime::Element,
                pub nullifier_hash:
                    runtime_types::darkwebb_standalone_runtime::Element,
                pub recipient: ::subxt::sp_core::crypto::AccountId32,
                pub relayer: ::subxt::sp_core::crypto::AccountId32,
                pub fee: ::core::primitive::u128,
                pub refund: ::core::primitive::u128,
            }
            impl ::subxt::Call for Withdraw {
                const PALLET: &'static str = "MixerBn254";
                const FUNCTION: &'static str = "withdraw";
            }
            pub struct TransactionApi<
                'a,
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            > {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn create(
                    &self,
                    deposit_size: ::core::primitive::u128,
                    depth: ::core::primitive::u8,
                    asset: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, Create>
                {
                    let call = Create {
                        deposit_size,
                        depth,
                        asset,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_maintainer(
                    &self,
                    new_maintainer: ::subxt::sp_core::crypto::AccountId32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, SetMaintainer>
                {
                    let call = SetMaintainer { new_maintainer };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_set_maintainer(
                    &self,
                    new_maintainer: ::subxt::sp_core::crypto::AccountId32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, ForceSetMaintainer>
                {
                    let call = ForceSetMaintainer { new_maintainer };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn deposit(
                    &self,
                    tree_id: ::core::primitive::u32,
                    leaf: runtime_types::darkwebb_standalone_runtime::Element,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, Deposit>
                {
                    let call = Deposit { tree_id, leaf };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn withdraw(
                    &self,
                    id: ::core::primitive::u32,
                    proof_bytes: ::std::vec::Vec<::core::primitive::u8>,
                    root: runtime_types::darkwebb_standalone_runtime::Element,
                    nullifier_hash : runtime_types :: darkwebb_standalone_runtime :: Element,
                    recipient: ::subxt::sp_core::crypto::AccountId32,
                    relayer: ::subxt::sp_core::crypto::AccountId32,
                    fee: ::core::primitive::u128,
                    refund: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, Withdraw>
                {
                    let call = Withdraw {
                        id,
                        proof_bytes,
                        root,
                        nullifier_hash,
                        recipient,
                        relayer,
                        fee,
                        refund,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_mixer::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct MaintainerSet {
                pub old_maintainer: ::subxt::sp_core::crypto::AccountId32,
                pub new_maintainer: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for MaintainerSet {
                const PALLET: &'static str = "MixerBn254";
                const EVENT: &'static str = "MaintainerSet";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct MixerCreation {
                pub tree_id: ::core::primitive::u32,
            }
            impl ::subxt::Event for MixerCreation {
                const PALLET: &'static str = "MixerBn254";
                const EVENT: &'static str = "MixerCreation";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Maintainer;
            impl ::subxt::StorageEntry for Maintainer {
                const PALLET: &'static str = "MixerBn254";
                const STORAGE: &'static str = "Maintainer";
                type Value = ::subxt::sp_core::crypto::AccountId32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Mixers(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for Mixers {
                const PALLET: &'static str = "MixerBn254";
                const STORAGE: &'static str = "Mixers";
                type Value = ::core::option::Option<
                    runtime_types::pallet_mixer::types::MixerMetadata<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                        ::core::primitive::u32,
                    >,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct NullifierHashes(
                ::core::primitive::u32,
                runtime_types::darkwebb_standalone_runtime::Element,
            );
            impl ::subxt::StorageEntry for NullifierHashes {
                const PALLET: &'static str = "MixerBn254";
                const STORAGE: &'static str = "NullifierHashes";
                type Value = ::core::primitive::bool;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn maintainer(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::subxt::Error,
                > {
                    let entry = Maintainer;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn mixers(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_mixer::types::MixerMetadata<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                            ::core::primitive::u32,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Mixers(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn mixers_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Mixers>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn nullifier_hashes(
                    &self,
                    _0: ::core::primitive::u32,
                    _1: runtime_types::darkwebb_standalone_runtime::Element,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::bool,
                    ::subxt::Error,
                > {
                    let entry = NullifierHashes(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn nullifier_hashes_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, NullifierHashes>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod mixer_bls381 {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Create {
                pub deposit_size: ::core::primitive::u128,
                pub depth: ::core::primitive::u8,
                pub asset: ::core::primitive::u32,
            }
            impl ::subxt::Call for Create {
                const PALLET: &'static str = "MixerBls381";
                const FUNCTION: &'static str = "create";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct SetMaintainer {
                pub new_maintainer: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Call for SetMaintainer {
                const PALLET: &'static str = "MixerBls381";
                const FUNCTION: &'static str = "set_maintainer";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ForceSetMaintainer {
                pub new_maintainer: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Call for ForceSetMaintainer {
                const PALLET: &'static str = "MixerBls381";
                const FUNCTION: &'static str = "force_set_maintainer";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Deposit {
                pub tree_id: ::core::primitive::u32,
                pub leaf: runtime_types::darkwebb_standalone_runtime::Element,
            }
            impl ::subxt::Call for Deposit {
                const PALLET: &'static str = "MixerBls381";
                const FUNCTION: &'static str = "deposit";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Withdraw {
                pub id: ::core::primitive::u32,
                pub proof_bytes: ::std::vec::Vec<::core::primitive::u8>,
                pub root: runtime_types::darkwebb_standalone_runtime::Element,
                pub nullifier_hash:
                    runtime_types::darkwebb_standalone_runtime::Element,
                pub recipient: ::subxt::sp_core::crypto::AccountId32,
                pub relayer: ::subxt::sp_core::crypto::AccountId32,
                pub fee: ::core::primitive::u128,
                pub refund: ::core::primitive::u128,
            }
            impl ::subxt::Call for Withdraw {
                const PALLET: &'static str = "MixerBls381";
                const FUNCTION: &'static str = "withdraw";
            }
            pub struct TransactionApi<
                'a,
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            > {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn create(
                    &self,
                    deposit_size: ::core::primitive::u128,
                    depth: ::core::primitive::u8,
                    asset: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, Create>
                {
                    let call = Create {
                        deposit_size,
                        depth,
                        asset,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_maintainer(
                    &self,
                    new_maintainer: ::subxt::sp_core::crypto::AccountId32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, SetMaintainer>
                {
                    let call = SetMaintainer { new_maintainer };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_set_maintainer(
                    &self,
                    new_maintainer: ::subxt::sp_core::crypto::AccountId32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, ForceSetMaintainer>
                {
                    let call = ForceSetMaintainer { new_maintainer };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn deposit(
                    &self,
                    tree_id: ::core::primitive::u32,
                    leaf: runtime_types::darkwebb_standalone_runtime::Element,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, Deposit>
                {
                    let call = Deposit { tree_id, leaf };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn withdraw(
                    &self,
                    id: ::core::primitive::u32,
                    proof_bytes: ::std::vec::Vec<::core::primitive::u8>,
                    root: runtime_types::darkwebb_standalone_runtime::Element,
                    nullifier_hash : runtime_types :: darkwebb_standalone_runtime :: Element,
                    recipient: ::subxt::sp_core::crypto::AccountId32,
                    relayer: ::subxt::sp_core::crypto::AccountId32,
                    fee: ::core::primitive::u128,
                    refund: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, Withdraw>
                {
                    let call = Withdraw {
                        id,
                        proof_bytes,
                        root,
                        nullifier_hash,
                        recipient,
                        relayer,
                        fee,
                        refund,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_mixer::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct MaintainerSet {
                pub old_maintainer: ::subxt::sp_core::crypto::AccountId32,
                pub new_maintainer: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for MaintainerSet {
                const PALLET: &'static str = "MixerBls381";
                const EVENT: &'static str = "MaintainerSet";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct MixerCreation {
                pub tree_id: ::core::primitive::u32,
            }
            impl ::subxt::Event for MixerCreation {
                const PALLET: &'static str = "MixerBls381";
                const EVENT: &'static str = "MixerCreation";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Maintainer;
            impl ::subxt::StorageEntry for Maintainer {
                const PALLET: &'static str = "MixerBls381";
                const STORAGE: &'static str = "Maintainer";
                type Value = ::subxt::sp_core::crypto::AccountId32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Mixers(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for Mixers {
                const PALLET: &'static str = "MixerBls381";
                const STORAGE: &'static str = "Mixers";
                type Value = ::core::option::Option<
                    runtime_types::pallet_mixer::types::MixerMetadata<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                        ::core::primitive::u32,
                    >,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct NullifierHashes(
                ::core::primitive::u32,
                runtime_types::darkwebb_standalone_runtime::Element,
            );
            impl ::subxt::StorageEntry for NullifierHashes {
                const PALLET: &'static str = "MixerBls381";
                const STORAGE: &'static str = "NullifierHashes";
                type Value = ::core::primitive::bool;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn maintainer(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::subxt::Error,
                > {
                    let entry = Maintainer;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn mixers(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_mixer::types::MixerMetadata<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                            ::core::primitive::u32,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Mixers(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn mixers_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Mixers>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn nullifier_hashes(
                    &self,
                    _0: ::core::primitive::u32,
                    _1: runtime_types::darkwebb_standalone_runtime::Element,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::bool,
                    ::subxt::Error,
                > {
                    let entry = NullifierHashes(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn nullifier_hashes_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, NullifierHashes>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod anchor_bn254 {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Create {
                pub deposit_size: ::core::primitive::u128,
                pub max_edges: ::core::primitive::u32,
                pub depth: ::core::primitive::u8,
                pub asset: ::core::primitive::u32,
            }
            impl ::subxt::Call for Create {
                const PALLET: &'static str = "AnchorBn254";
                const FUNCTION: &'static str = "create";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Deposit {
                pub tree_id: ::core::primitive::u32,
                pub leaf: runtime_types::darkwebb_standalone_runtime::Element,
            }
            impl ::subxt::Call for Deposit {
                const PALLET: &'static str = "AnchorBn254";
                const FUNCTION: &'static str = "deposit";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct DepositAndUpdateLinkedAnchors {
                pub tree_id: ::core::primitive::u32,
                pub leaf: runtime_types::darkwebb_standalone_runtime::Element,
            }
            impl ::subxt::Call for DepositAndUpdateLinkedAnchors {
                const PALLET: &'static str = "AnchorBn254";
                const FUNCTION: &'static str =
                    "deposit_and_update_linked_anchors";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Withdraw {
                pub id: ::core::primitive::u32,
                pub proof_bytes: ::std::vec::Vec<::core::primitive::u8>,
                pub chain_id: ::core::primitive::u32,
                pub roots: ::std::vec::Vec<
                    runtime_types::darkwebb_standalone_runtime::Element,
                >,
                pub nullifier_hash:
                    runtime_types::darkwebb_standalone_runtime::Element,
                pub recipient: ::subxt::sp_core::crypto::AccountId32,
                pub relayer: ::subxt::sp_core::crypto::AccountId32,
                pub fee: ::core::primitive::u128,
                pub refund: ::core::primitive::u128,
                pub commitment:
                    runtime_types::darkwebb_standalone_runtime::Element,
            }
            impl ::subxt::Call for Withdraw {
                const PALLET: &'static str = "AnchorBn254";
                const FUNCTION: &'static str = "withdraw";
            }
            pub struct TransactionApi<
                'a,
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            > {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn create(
                    &self,
                    deposit_size: ::core::primitive::u128,
                    max_edges: ::core::primitive::u32,
                    depth: ::core::primitive::u8,
                    asset: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, Create>
                {
                    let call = Create {
                        deposit_size,
                        max_edges,
                        depth,
                        asset,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn deposit(
                    &self,
                    tree_id: ::core::primitive::u32,
                    leaf: runtime_types::darkwebb_standalone_runtime::Element,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, Deposit>
                {
                    let call = Deposit { tree_id, leaf };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn deposit_and_update_linked_anchors(
                    &self,
                    tree_id: ::core::primitive::u32,
                    leaf: runtime_types::darkwebb_standalone_runtime::Element,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    DepositAndUpdateLinkedAnchors,
                > {
                    let call = DepositAndUpdateLinkedAnchors { tree_id, leaf };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn withdraw(
                    &self,
                    id: ::core::primitive::u32,
                    proof_bytes: ::std::vec::Vec<::core::primitive::u8>,
                    chain_id: ::core::primitive::u32,
                    roots: ::std::vec::Vec<
                        runtime_types::darkwebb_standalone_runtime::Element,
                    >,
                    nullifier_hash : runtime_types :: darkwebb_standalone_runtime :: Element,
                    recipient: ::subxt::sp_core::crypto::AccountId32,
                    relayer: ::subxt::sp_core::crypto::AccountId32,
                    fee: ::core::primitive::u128,
                    refund: ::core::primitive::u128,
                    commitment : runtime_types :: darkwebb_standalone_runtime :: Element,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, Withdraw>
                {
                    let call = Withdraw {
                        id,
                        proof_bytes,
                        chain_id,
                        roots,
                        nullifier_hash,
                        recipient,
                        relayer,
                        fee,
                        refund,
                        commitment,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_anchor::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct AnchorCreation {
                pub tree_id: ::core::primitive::u32,
            }
            impl ::subxt::Event for AnchorCreation {
                const PALLET: &'static str = "AnchorBn254";
                const EVENT: &'static str = "AnchorCreation";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Withdraw {
                pub who: ::subxt::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Event for Withdraw {
                const PALLET: &'static str = "AnchorBn254";
                const EVENT: &'static str = "Withdraw";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Deposit {
                pub depositor: ::subxt::sp_core::crypto::AccountId32,
                pub tree_id: ::core::primitive::u32,
                pub leaf: runtime_types::darkwebb_standalone_runtime::Element,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Event for Deposit {
                const PALLET: &'static str = "AnchorBn254";
                const EVENT: &'static str = "Deposit";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct PostDeposit {
                pub depositor: ::subxt::sp_core::crypto::AccountId32,
                pub tree_id: ::core::primitive::u32,
                pub leaf: runtime_types::darkwebb_standalone_runtime::Element,
            }
            impl ::subxt::Event for PostDeposit {
                const PALLET: &'static str = "AnchorBn254";
                const EVENT: &'static str = "PostDeposit";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Anchors(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for Anchors {
                const PALLET: &'static str = "AnchorBn254";
                const STORAGE: &'static str = "Anchors";
                type Value = ::core::option::Option<
                    runtime_types::pallet_anchor::types::AnchorMetadata<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                        ::core::primitive::u32,
                    >,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct NullifierHashes(
                ::core::primitive::u32,
                runtime_types::darkwebb_standalone_runtime::Element,
            );
            impl ::subxt::StorageEntry for NullifierHashes {
                const PALLET: &'static str = "AnchorBn254";
                const STORAGE: &'static str = "NullifierHashes";
                type Value = ::core::primitive::bool;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn anchors(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_anchor::types::AnchorMetadata<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                            ::core::primitive::u32,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Anchors(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn anchors_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Anchors>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn nullifier_hashes(
                    &self,
                    _0: ::core::primitive::u32,
                    _1: runtime_types::darkwebb_standalone_runtime::Element,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::bool,
                    ::subxt::Error,
                > {
                    let entry = NullifierHashes(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn nullifier_hashes_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, NullifierHashes>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod anchor_bls381 {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Create {
                pub deposit_size: ::core::primitive::u128,
                pub max_edges: ::core::primitive::u32,
                pub depth: ::core::primitive::u8,
                pub asset: ::core::primitive::u32,
            }
            impl ::subxt::Call for Create {
                const PALLET: &'static str = "AnchorBls381";
                const FUNCTION: &'static str = "create";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Deposit {
                pub tree_id: ::core::primitive::u32,
                pub leaf: runtime_types::darkwebb_standalone_runtime::Element,
            }
            impl ::subxt::Call for Deposit {
                const PALLET: &'static str = "AnchorBls381";
                const FUNCTION: &'static str = "deposit";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct DepositAndUpdateLinkedAnchors {
                pub tree_id: ::core::primitive::u32,
                pub leaf: runtime_types::darkwebb_standalone_runtime::Element,
            }
            impl ::subxt::Call for DepositAndUpdateLinkedAnchors {
                const PALLET: &'static str = "AnchorBls381";
                const FUNCTION: &'static str =
                    "deposit_and_update_linked_anchors";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Withdraw {
                pub id: ::core::primitive::u32,
                pub proof_bytes: ::std::vec::Vec<::core::primitive::u8>,
                pub chain_id: ::core::primitive::u32,
                pub roots: ::std::vec::Vec<
                    runtime_types::darkwebb_standalone_runtime::Element,
                >,
                pub nullifier_hash:
                    runtime_types::darkwebb_standalone_runtime::Element,
                pub recipient: ::subxt::sp_core::crypto::AccountId32,
                pub relayer: ::subxt::sp_core::crypto::AccountId32,
                pub fee: ::core::primitive::u128,
                pub refund: ::core::primitive::u128,
                pub commitment:
                    runtime_types::darkwebb_standalone_runtime::Element,
            }
            impl ::subxt::Call for Withdraw {
                const PALLET: &'static str = "AnchorBls381";
                const FUNCTION: &'static str = "withdraw";
            }
            pub struct TransactionApi<
                'a,
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            > {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn create(
                    &self,
                    deposit_size: ::core::primitive::u128,
                    max_edges: ::core::primitive::u32,
                    depth: ::core::primitive::u8,
                    asset: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, Create>
                {
                    let call = Create {
                        deposit_size,
                        max_edges,
                        depth,
                        asset,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn deposit(
                    &self,
                    tree_id: ::core::primitive::u32,
                    leaf: runtime_types::darkwebb_standalone_runtime::Element,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, Deposit>
                {
                    let call = Deposit { tree_id, leaf };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn deposit_and_update_linked_anchors(
                    &self,
                    tree_id: ::core::primitive::u32,
                    leaf: runtime_types::darkwebb_standalone_runtime::Element,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    DepositAndUpdateLinkedAnchors,
                > {
                    let call = DepositAndUpdateLinkedAnchors { tree_id, leaf };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn withdraw(
                    &self,
                    id: ::core::primitive::u32,
                    proof_bytes: ::std::vec::Vec<::core::primitive::u8>,
                    chain_id: ::core::primitive::u32,
                    roots: ::std::vec::Vec<
                        runtime_types::darkwebb_standalone_runtime::Element,
                    >,
                    nullifier_hash : runtime_types :: darkwebb_standalone_runtime :: Element,
                    recipient: ::subxt::sp_core::crypto::AccountId32,
                    relayer: ::subxt::sp_core::crypto::AccountId32,
                    fee: ::core::primitive::u128,
                    refund: ::core::primitive::u128,
                    commitment : runtime_types :: darkwebb_standalone_runtime :: Element,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, Withdraw>
                {
                    let call = Withdraw {
                        id,
                        proof_bytes,
                        chain_id,
                        roots,
                        nullifier_hash,
                        recipient,
                        relayer,
                        fee,
                        refund,
                        commitment,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_anchor::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct AnchorCreation {
                pub tree_id: ::core::primitive::u32,
            }
            impl ::subxt::Event for AnchorCreation {
                const PALLET: &'static str = "AnchorBls381";
                const EVENT: &'static str = "AnchorCreation";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Withdraw {
                pub who: ::subxt::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Event for Withdraw {
                const PALLET: &'static str = "AnchorBls381";
                const EVENT: &'static str = "Withdraw";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Deposit {
                pub depositor: ::subxt::sp_core::crypto::AccountId32,
                pub tree_id: ::core::primitive::u32,
                pub leaf: runtime_types::darkwebb_standalone_runtime::Element,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Event for Deposit {
                const PALLET: &'static str = "AnchorBls381";
                const EVENT: &'static str = "Deposit";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct PostDeposit {
                pub depositor: ::subxt::sp_core::crypto::AccountId32,
                pub tree_id: ::core::primitive::u32,
                pub leaf: runtime_types::darkwebb_standalone_runtime::Element,
            }
            impl ::subxt::Event for PostDeposit {
                const PALLET: &'static str = "AnchorBls381";
                const EVENT: &'static str = "PostDeposit";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Anchors(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for Anchors {
                const PALLET: &'static str = "AnchorBls381";
                const STORAGE: &'static str = "Anchors";
                type Value = ::core::option::Option<
                    runtime_types::pallet_anchor::types::AnchorMetadata<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                        ::core::primitive::u32,
                    >,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct NullifierHashes(
                ::core::primitive::u32,
                runtime_types::darkwebb_standalone_runtime::Element,
            );
            impl ::subxt::StorageEntry for NullifierHashes {
                const PALLET: &'static str = "AnchorBls381";
                const STORAGE: &'static str = "NullifierHashes";
                type Value = ::core::primitive::bool;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn anchors(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_anchor::types::AnchorMetadata<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                            ::core::primitive::u32,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Anchors(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn anchors_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Anchors>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn nullifier_hashes(
                    &self,
                    _0: ::core::primitive::u32,
                    _1: runtime_types::darkwebb_standalone_runtime::Element,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::bool,
                    ::subxt::Error,
                > {
                    let entry = NullifierHashes(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn nullifier_hashes_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, NullifierHashes>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod anchor_handler_bn254 {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ExecuteAnchorCreateProposal {
                pub deposit_size: ::core::primitive::u128,
                pub src_chain_id: ::core::primitive::u32,
                pub r_id: [::core::primitive::u8; 32usize],
                pub max_edges: ::core::primitive::u32,
                pub tree_depth: ::core::primitive::u8,
                pub asset: ::core::primitive::u32,
            }
            impl ::subxt::Call for ExecuteAnchorCreateProposal {
                const PALLET: &'static str = "AnchorHandlerBn254";
                const FUNCTION: &'static str = "execute_anchor_create_proposal";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ExecuteAnchorUpdateProposal {
                pub r_id: [::core::primitive::u8; 32usize],
                pub anchor_metadata:
                    runtime_types::pallet_linkable_tree::types::EdgeMetadata<
                        ::core::primitive::u32,
                        runtime_types::darkwebb_standalone_runtime::Element,
                        ::core::primitive::u32,
                    >,
            }
            impl ::subxt::Call for ExecuteAnchorUpdateProposal {
                const PALLET: &'static str = "AnchorHandlerBn254";
                const FUNCTION: &'static str = "execute_anchor_update_proposal";
            }
            pub struct TransactionApi<
                'a,
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            > {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn execute_anchor_create_proposal(
                    &self,
                    deposit_size: ::core::primitive::u128,
                    src_chain_id: ::core::primitive::u32,
                    r_id: [::core::primitive::u8; 32usize],
                    max_edges: ::core::primitive::u32,
                    tree_depth: ::core::primitive::u8,
                    asset: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    ExecuteAnchorCreateProposal,
                > {
                    let call = ExecuteAnchorCreateProposal {
                        deposit_size,
                        src_chain_id,
                        r_id,
                        max_edges,
                        tree_depth,
                        asset,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn execute_anchor_update_proposal(
                    &self,
                    r_id: [::core::primitive::u8; 32usize],
                    anchor_metadata : runtime_types :: pallet_linkable_tree :: types :: EdgeMetadata < :: core :: primitive :: u32 , runtime_types :: darkwebb_standalone_runtime :: Element , :: core :: primitive :: u32 >,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    ExecuteAnchorUpdateProposal,
                > {
                    let call = ExecuteAnchorUpdateProposal {
                        r_id,
                        anchor_metadata,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_anchor_handler::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct AnchorCreated {}
            impl ::subxt::Event for AnchorCreated {
                const PALLET: &'static str = "AnchorHandlerBn254";
                const EVENT: &'static str = "AnchorCreated";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct AnchorEdgeAdded {}
            impl ::subxt::Event for AnchorEdgeAdded {
                const PALLET: &'static str = "AnchorHandlerBn254";
                const EVENT: &'static str = "AnchorEdgeAdded";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct AnchorEdgeUpdated {}
            impl ::subxt::Event for AnchorEdgeUpdated {
                const PALLET: &'static str = "AnchorHandlerBn254";
                const EVENT: &'static str = "AnchorEdgeUpdated";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct AnchorList(pub [::core::primitive::u8; 32usize]);
            impl ::subxt::StorageEntry for AnchorList {
                const PALLET: &'static str = "AnchorHandlerBn254";
                const STORAGE: &'static str = "AnchorList";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct UpdateRecords(
                ::core::primitive::u32,
                ::core::primitive::u64,
            );
            impl ::subxt::StorageEntry for UpdateRecords {
                const PALLET: &'static str = "AnchorHandlerBn254";
                const STORAGE: &'static str = "UpdateRecords";
                type Value =
                    runtime_types::pallet_anchor_handler::types::UpdateRecord<
                        ::core::primitive::u32,
                        [::core::primitive::u8; 32usize],
                        ::core::primitive::u32,
                        runtime_types::darkwebb_standalone_runtime::Element,
                        ::core::primitive::u32,
                    >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct Counts(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for Counts {
                const PALLET: &'static str = "AnchorHandlerBn254";
                const STORAGE: &'static str = "Counts";
                type Value = ::core::primitive::u64;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn anchor_list(
                    &self,
                    _0: [::core::primitive::u8; 32usize],
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::u32,
                    ::subxt::Error,
                > {
                    let entry = AnchorList(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn anchor_list_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, AnchorList>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn update_records(
                    &self,
                    _0: ::core::primitive::u32,
                    _1: ::core::primitive::u64,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_anchor_handler::types::UpdateRecord<
                        ::core::primitive::u32,
                        [::core::primitive::u8; 32usize],
                        ::core::primitive::u32,
                        runtime_types::darkwebb_standalone_runtime::Element,
                        ::core::primitive::u32,
                    >,
                    ::subxt::Error,
                > {
                    let entry = UpdateRecords(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn update_records_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, UpdateRecords>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn counts(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::u64,
                    ::subxt::Error,
                > {
                    let entry = Counts(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn counts_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Counts>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod anchor_handler_bls381 {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ExecuteAnchorCreateProposal {
                pub deposit_size: ::core::primitive::u128,
                pub src_chain_id: ::core::primitive::u32,
                pub r_id: [::core::primitive::u8; 32usize],
                pub max_edges: ::core::primitive::u32,
                pub tree_depth: ::core::primitive::u8,
                pub asset: ::core::primitive::u32,
            }
            impl ::subxt::Call for ExecuteAnchorCreateProposal {
                const PALLET: &'static str = "AnchorHandlerBls381";
                const FUNCTION: &'static str = "execute_anchor_create_proposal";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ExecuteAnchorUpdateProposal {
                pub r_id: [::core::primitive::u8; 32usize],
                pub anchor_metadata:
                    runtime_types::pallet_linkable_tree::types::EdgeMetadata<
                        ::core::primitive::u32,
                        runtime_types::darkwebb_standalone_runtime::Element,
                        ::core::primitive::u32,
                    >,
            }
            impl ::subxt::Call for ExecuteAnchorUpdateProposal {
                const PALLET: &'static str = "AnchorHandlerBls381";
                const FUNCTION: &'static str = "execute_anchor_update_proposal";
            }
            pub struct TransactionApi<
                'a,
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            > {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn execute_anchor_create_proposal(
                    &self,
                    deposit_size: ::core::primitive::u128,
                    src_chain_id: ::core::primitive::u32,
                    r_id: [::core::primitive::u8; 32usize],
                    max_edges: ::core::primitive::u32,
                    tree_depth: ::core::primitive::u8,
                    asset: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    ExecuteAnchorCreateProposal,
                > {
                    let call = ExecuteAnchorCreateProposal {
                        deposit_size,
                        src_chain_id,
                        r_id,
                        max_edges,
                        tree_depth,
                        asset,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn execute_anchor_update_proposal(
                    &self,
                    r_id: [::core::primitive::u8; 32usize],
                    anchor_metadata : runtime_types :: pallet_linkable_tree :: types :: EdgeMetadata < :: core :: primitive :: u32 , runtime_types :: darkwebb_standalone_runtime :: Element , :: core :: primitive :: u32 >,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    ExecuteAnchorUpdateProposal,
                > {
                    let call = ExecuteAnchorUpdateProposal {
                        r_id,
                        anchor_metadata,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_anchor_handler::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct AnchorCreated {}
            impl ::subxt::Event for AnchorCreated {
                const PALLET: &'static str = "AnchorHandlerBls381";
                const EVENT: &'static str = "AnchorCreated";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct AnchorEdgeAdded {}
            impl ::subxt::Event for AnchorEdgeAdded {
                const PALLET: &'static str = "AnchorHandlerBls381";
                const EVENT: &'static str = "AnchorEdgeAdded";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct AnchorEdgeUpdated {}
            impl ::subxt::Event for AnchorEdgeUpdated {
                const PALLET: &'static str = "AnchorHandlerBls381";
                const EVENT: &'static str = "AnchorEdgeUpdated";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct AnchorList(pub [::core::primitive::u8; 32usize]);
            impl ::subxt::StorageEntry for AnchorList {
                const PALLET: &'static str = "AnchorHandlerBls381";
                const STORAGE: &'static str = "AnchorList";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct UpdateRecords(
                ::core::primitive::u32,
                ::core::primitive::u64,
            );
            impl ::subxt::StorageEntry for UpdateRecords {
                const PALLET: &'static str = "AnchorHandlerBls381";
                const STORAGE: &'static str = "UpdateRecords";
                type Value =
                    runtime_types::pallet_anchor_handler::types::UpdateRecord<
                        ::core::primitive::u32,
                        [::core::primitive::u8; 32usize],
                        ::core::primitive::u32,
                        runtime_types::darkwebb_standalone_runtime::Element,
                        ::core::primitive::u32,
                    >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct Counts(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for Counts {
                const PALLET: &'static str = "AnchorHandlerBls381";
                const STORAGE: &'static str = "Counts";
                type Value = ::core::primitive::u64;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn anchor_list(
                    &self,
                    _0: [::core::primitive::u8; 32usize],
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::u32,
                    ::subxt::Error,
                > {
                    let entry = AnchorList(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn anchor_list_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, AnchorList>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn update_records(
                    &self,
                    _0: ::core::primitive::u32,
                    _1: ::core::primitive::u64,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_anchor_handler::types::UpdateRecord<
                        ::core::primitive::u32,
                        [::core::primitive::u8; 32usize],
                        ::core::primitive::u32,
                        runtime_types::darkwebb_standalone_runtime::Element,
                        ::core::primitive::u32,
                    >,
                    ::subxt::Error,
                > {
                    let entry = UpdateRecords(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn update_records_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, UpdateRecords>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn counts(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::u64,
                    ::subxt::Error,
                > {
                    let entry = Counts(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn counts_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Counts>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod bridge {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct SetMaintainer {
                pub new_maintainer: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Call for SetMaintainer {
                const PALLET: &'static str = "Bridge";
                const FUNCTION: &'static str = "set_maintainer";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ForceSetMaintainer {
                pub new_maintainer: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Call for ForceSetMaintainer {
                const PALLET: &'static str = "Bridge";
                const FUNCTION: &'static str = "force_set_maintainer";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct SetThreshold {
                pub threshold: ::core::primitive::u32,
            }
            impl ::subxt::Call for SetThreshold {
                const PALLET: &'static str = "Bridge";
                const FUNCTION: &'static str = "set_threshold";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct SetResource {
                pub id: [::core::primitive::u8; 32usize],
                pub method: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for SetResource {
                const PALLET: &'static str = "Bridge";
                const FUNCTION: &'static str = "set_resource";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct RemoveResource {
                pub id: [::core::primitive::u8; 32usize],
            }
            impl ::subxt::Call for RemoveResource {
                const PALLET: &'static str = "Bridge";
                const FUNCTION: &'static str = "remove_resource";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct WhitelistChain {
                pub id: ::core::primitive::u32,
            }
            impl ::subxt::Call for WhitelistChain {
                const PALLET: &'static str = "Bridge";
                const FUNCTION: &'static str = "whitelist_chain";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct AddRelayer {
                pub v: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Call for AddRelayer {
                const PALLET: &'static str = "Bridge";
                const FUNCTION: &'static str = "add_relayer";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct RemoveRelayer {
                pub v: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Call for RemoveRelayer {
                const PALLET: &'static str = "Bridge";
                const FUNCTION: &'static str = "remove_relayer";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct AcknowledgeProposal {
                pub nonce: ::core::primitive::u64,
                pub src_id: ::core::primitive::u32,
                pub r_id: [::core::primitive::u8; 32usize],
                pub call: runtime_types::darkwebb_standalone_runtime::Call,
            }
            impl ::subxt::Call for AcknowledgeProposal {
                const PALLET: &'static str = "Bridge";
                const FUNCTION: &'static str = "acknowledge_proposal";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct RejectProposal {
                pub nonce: ::core::primitive::u64,
                pub src_id: ::core::primitive::u32,
                pub r_id: [::core::primitive::u8; 32usize],
                pub call: runtime_types::darkwebb_standalone_runtime::Call,
            }
            impl ::subxt::Call for RejectProposal {
                const PALLET: &'static str = "Bridge";
                const FUNCTION: &'static str = "reject_proposal";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct EvalVoteState {
                pub nonce: ::core::primitive::u64,
                pub src_id: ::core::primitive::u32,
                pub prop: runtime_types::darkwebb_standalone_runtime::Call,
            }
            impl ::subxt::Call for EvalVoteState {
                const PALLET: &'static str = "Bridge";
                const FUNCTION: &'static str = "eval_vote_state";
            }
            pub struct TransactionApi<
                'a,
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            > {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn set_maintainer(
                    &self,
                    new_maintainer: ::subxt::sp_core::crypto::AccountId32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, SetMaintainer>
                {
                    let call = SetMaintainer { new_maintainer };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_set_maintainer(
                    &self,
                    new_maintainer: ::subxt::sp_core::crypto::AccountId32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, ForceSetMaintainer>
                {
                    let call = ForceSetMaintainer { new_maintainer };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_threshold(
                    &self,
                    threshold: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, SetThreshold>
                {
                    let call = SetThreshold { threshold };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_resource(
                    &self,
                    id: [::core::primitive::u8; 32usize],
                    method: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, SetResource>
                {
                    let call = SetResource { id, method };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn remove_resource(
                    &self,
                    id: [::core::primitive::u8; 32usize],
                ) -> ::subxt::SubmittableExtrinsic<'a, T, RemoveResource>
                {
                    let call = RemoveResource { id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn whitelist_chain(
                    &self,
                    id: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, WhitelistChain>
                {
                    let call = WhitelistChain { id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn add_relayer(
                    &self,
                    v: ::subxt::sp_core::crypto::AccountId32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, AddRelayer>
                {
                    let call = AddRelayer { v };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn remove_relayer(
                    &self,
                    v: ::subxt::sp_core::crypto::AccountId32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, RemoveRelayer>
                {
                    let call = RemoveRelayer { v };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn acknowledge_proposal(
                    &self,
                    nonce: ::core::primitive::u64,
                    src_id: ::core::primitive::u32,
                    r_id: [::core::primitive::u8; 32usize],
                    call: runtime_types::darkwebb_standalone_runtime::Call,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, AcknowledgeProposal>
                {
                    let call = AcknowledgeProposal {
                        nonce,
                        src_id,
                        r_id,
                        call,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn reject_proposal(
                    &self,
                    nonce: ::core::primitive::u64,
                    src_id: ::core::primitive::u32,
                    r_id: [::core::primitive::u8; 32usize],
                    call: runtime_types::darkwebb_standalone_runtime::Call,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, RejectProposal>
                {
                    let call = RejectProposal {
                        nonce,
                        src_id,
                        r_id,
                        call,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn eval_vote_state(
                    &self,
                    nonce: ::core::primitive::u64,
                    src_id: ::core::primitive::u32,
                    prop: runtime_types::darkwebb_standalone_runtime::Call,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, EvalVoteState>
                {
                    let call = EvalVoteState {
                        nonce,
                        src_id,
                        prop,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_bridge::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct MaintainerSet {
                pub old_maintainer: ::subxt::sp_core::crypto::AccountId32,
                pub new_maintainer: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for MaintainerSet {
                const PALLET: &'static str = "Bridge";
                const EVENT: &'static str = "MaintainerSet";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct RelayerThresholdChanged {
                pub new_threshold: ::core::primitive::u32,
            }
            impl ::subxt::Event for RelayerThresholdChanged {
                const PALLET: &'static str = "Bridge";
                const EVENT: &'static str = "RelayerThresholdChanged";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ChainWhitelisted {
                pub chain_id: ::core::primitive::u32,
            }
            impl ::subxt::Event for ChainWhitelisted {
                const PALLET: &'static str = "Bridge";
                const EVENT: &'static str = "ChainWhitelisted";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct RelayerAdded {
                pub relayer_id: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for RelayerAdded {
                const PALLET: &'static str = "Bridge";
                const EVENT: &'static str = "RelayerAdded";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct RelayerRemoved {
                pub relayer_id: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for RelayerRemoved {
                const PALLET: &'static str = "Bridge";
                const EVENT: &'static str = "RelayerRemoved";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct VoteFor {
                pub chain_id: ::core::primitive::u32,
                pub deposit_nonce: ::core::primitive::u64,
                pub who: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for VoteFor {
                const PALLET: &'static str = "Bridge";
                const EVENT: &'static str = "VoteFor";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct VoteAgainst {
                pub chain_id: ::core::primitive::u32,
                pub deposit_nonce: ::core::primitive::u64,
                pub who: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for VoteAgainst {
                const PALLET: &'static str = "Bridge";
                const EVENT: &'static str = "VoteAgainst";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ProposalApproved {
                pub chain_id: ::core::primitive::u32,
                pub deposit_nonce: ::core::primitive::u64,
            }
            impl ::subxt::Event for ProposalApproved {
                const PALLET: &'static str = "Bridge";
                const EVENT: &'static str = "ProposalApproved";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ProposalRejected {
                pub chain_id: ::core::primitive::u32,
                pub deposit_nonce: ::core::primitive::u64,
            }
            impl ::subxt::Event for ProposalRejected {
                const PALLET: &'static str = "Bridge";
                const EVENT: &'static str = "ProposalRejected";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ProposalSucceeded {
                pub chain_id: ::core::primitive::u32,
                pub deposit_nonce: ::core::primitive::u64,
            }
            impl ::subxt::Event for ProposalSucceeded {
                const PALLET: &'static str = "Bridge";
                const EVENT: &'static str = "ProposalSucceeded";
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ProposalFailed {
                pub chain_id: ::core::primitive::u32,
                pub deposit_nonce: ::core::primitive::u64,
            }
            impl ::subxt::Event for ProposalFailed {
                const PALLET: &'static str = "Bridge";
                const EVENT: &'static str = "ProposalFailed";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Maintainer;
            impl ::subxt::StorageEntry for Maintainer {
                const PALLET: &'static str = "Bridge";
                const STORAGE: &'static str = "Maintainer";
                type Value = ::subxt::sp_core::crypto::AccountId32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ChainNonces(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for ChainNonces {
                const PALLET: &'static str = "Bridge";
                const STORAGE: &'static str = "ChainNonces";
                type Value = ::core::primitive::u64;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_256,
                        ),
                    ])
                }
            }
            pub struct RelayerThreshold;
            impl ::subxt::StorageEntry for RelayerThreshold {
                const PALLET: &'static str = "Bridge";
                const STORAGE: &'static str = "RelayerThreshold";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Relayers(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Relayers {
                const PALLET: &'static str = "Bridge";
                const STORAGE: &'static str = "Relayers";
                type Value = ::core::primitive::bool;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_256,
                        ),
                    ])
                }
            }
            pub struct RelayerCount;
            impl ::subxt::StorageEntry for RelayerCount {
                const PALLET: &'static str = "Bridge";
                const STORAGE: &'static str = "RelayerCount";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Votes(
                ::core::primitive::u32,
                (
                    ::core::primitive::u64,
                    runtime_types::darkwebb_standalone_runtime::Call,
                ),
            );
            impl ::subxt::StorageEntry for Votes {
                const PALLET: &'static str = "Bridge";
                const STORAGE: &'static str = "Votes";
                type Value = runtime_types::pallet_bridge::types::ProposalVotes<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_256,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Blake2_256,
                        ),
                    ])
                }
            }
            pub struct Resources(pub [::core::primitive::u8; 32usize]);
            impl ::subxt::StorageEntry for Resources {
                const PALLET: &'static str = "Bridge";
                const STORAGE: &'static str = "Resources";
                type Value = ::std::vec::Vec<::core::primitive::u8>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_256,
                        ),
                    ])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn maintainer(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::subxt::Error,
                > {
                    let entry = Maintainer;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn chain_nonces(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u64>,
                    ::subxt::Error,
                > {
                    let entry = ChainNonces(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn chain_nonces_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ChainNonces>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn relayer_threshold(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::u32,
                    ::subxt::Error,
                > {
                    let entry = RelayerThreshold;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn relayers(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::bool,
                    ::subxt::Error,
                > {
                    let entry = Relayers(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn relayers_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Relayers>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn relayer_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::u32,
                    ::subxt::Error,
                > {
                    let entry = RelayerCount;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn votes(
                    &self,
                    _0: ::core::primitive::u32,
                    _1: (
                        ::core::primitive::u64,
                        runtime_types::darkwebb_standalone_runtime::Call,
                    ),
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_bridge::types::ProposalVotes<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Votes(_0, _1);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn votes_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Votes>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn resources(
                    &self,
                    _0: [::core::primitive::u8; 32usize],
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        ::std::vec::Vec<::core::primitive::u8>,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Resources(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn resources_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Resources>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod runtime_types {
        use super::runtime_types;
        pub mod darkwebb_primitives {
            use super::runtime_types;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub struct DepositDetails<_0, _1> {
                    pub depositor: _0,
                    pub deposit: _1,
                }
            }
        }
        pub mod darkwebb_standalone_runtime {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub enum Call {
                # [codec (index = 0)] System (runtime_types :: frame_system :: pallet :: Call ,) , # [codec (index = 2)] Timestamp (runtime_types :: pallet_timestamp :: pallet :: Call ,) , # [codec (index = 3)] Babe (runtime_types :: pallet_babe :: pallet :: Call ,) , # [codec (index = 4)] Authorship (runtime_types :: pallet_authorship :: pallet :: Call ,) , # [codec (index = 5)] Indices (runtime_types :: pallet_indices :: pallet :: Call ,) , # [codec (index = 6)] Balances (runtime_types :: pallet_balances :: pallet :: Call ,) , # [codec (index = 8)] ElectionProviderMultiPhase (runtime_types :: pallet_election_provider_multi_phase :: pallet :: Call ,) , # [codec (index = 9)] Staking (runtime_types :: pallet_staking :: pallet :: pallet :: Call ,) , # [codec (index = 10)] Session (runtime_types :: pallet_session :: pallet :: Call ,) , # [codec (index = 11)] Democracy (runtime_types :: pallet_democracy :: pallet :: Call ,) , # [codec (index = 12)] Council (runtime_types :: pallet_collective :: pallet :: Call ,) , # [codec (index = 13)] Elections (runtime_types :: pallet_elections_phragmen :: pallet :: Call ,) , # [codec (index = 14)] Grandpa (runtime_types :: pallet_grandpa :: pallet :: Call ,) , # [codec (index = 15)] Treasury (runtime_types :: pallet_treasury :: pallet :: Call ,) , # [codec (index = 16)] Utility (runtime_types :: pallet_utility :: pallet :: Call ,) , # [codec (index = 17)] Multisig (runtime_types :: pallet_multisig :: pallet :: Call ,) , # [codec (index = 18)] Scheduler (runtime_types :: pallet_scheduler :: pallet :: Call ,) , # [codec (index = 19)] Proxy (runtime_types :: pallet_proxy :: pallet :: Call ,) , # [codec (index = 20)] Assets (runtime_types :: pallet_assets :: pallet :: Call ,) , # [codec (index = 21)] Sudo (runtime_types :: pallet_sudo :: pallet :: Call ,) , # [codec (index = 22)] ImOnline (runtime_types :: pallet_im_online :: pallet :: Call ,) , # [codec (index = 26)] Bounties (runtime_types :: pallet_bounties :: pallet :: Call ,) , # [codec (index = 27)] BagsList (runtime_types :: pallet_bags_list :: pallet :: Call ,) , # [codec (index = 28)] HasherBn254 (runtime_types :: pallet_hasher :: pallet :: Call ,) , # [codec (index = 29)] HasherBls381 (runtime_types :: pallet_hasher :: pallet :: Call ,) , # [codec (index = 30)] AssetRegistry (runtime_types :: pallet_asset_registry :: pallet :: Call ,) , # [codec (index = 31)] Currencies (runtime_types :: orml_currencies :: module :: Call ,) , # [codec (index = 32)] Tokens (runtime_types :: orml_tokens :: module :: Call ,) , # [codec (index = 33)] TokenWrapper (runtime_types :: pallet_token_wrapper :: pallet :: Call ,) , # [codec (index = 34)] VerifierBn254 (runtime_types :: pallet_verifier :: pallet :: Call ,) , # [codec (index = 35)] VerifierBls381 (runtime_types :: pallet_verifier :: pallet :: Call ,) , # [codec (index = 36)] MerkleTreeBn254 (runtime_types :: pallet_mt :: pallet :: Call ,) , # [codec (index = 37)] MerkleTreeBls381 (runtime_types :: pallet_mt :: pallet :: Call ,) , # [codec (index = 38)] LinkableTreeBn254 (runtime_types :: pallet_linkable_tree :: pallet :: Call ,) , # [codec (index = 39)] LinkableTreeBls381 (runtime_types :: pallet_linkable_tree :: pallet :: Call ,) , # [codec (index = 40)] MixerBn254 (runtime_types :: pallet_mixer :: pallet :: Call ,) , # [codec (index = 41)] MixerBls381 (runtime_types :: pallet_mixer :: pallet :: Call ,) , # [codec (index = 42)] AnchorBn254 (runtime_types :: pallet_anchor :: pallet :: Call ,) , # [codec (index = 43)] AnchorBls381 (runtime_types :: pallet_anchor :: pallet :: Call ,) , # [codec (index = 44)] AnchorHandlerBn254 (runtime_types :: pallet_anchor_handler :: pallet :: Call ,) , # [codec (index = 45)] AnchorHandlerBls381 (runtime_types :: pallet_anchor_handler :: pallet :: Call ,) , # [codec (index = 46)] Bridge (runtime_types :: pallet_bridge :: pallet :: Call ,) , }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Element(pub [::core::primitive::u8; 32usize]);
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub enum Event {
                # [codec (index = 0)] System (runtime_types :: frame_system :: pallet :: Event ,) , # [codec (index = 5)] Indices (runtime_types :: pallet_indices :: pallet :: Event ,) , # [codec (index = 6)] Balances (runtime_types :: pallet_balances :: pallet :: Event ,) , # [codec (index = 8)] ElectionProviderMultiPhase (runtime_types :: pallet_election_provider_multi_phase :: pallet :: Event ,) , # [codec (index = 9)] Staking (runtime_types :: pallet_staking :: pallet :: pallet :: Event ,) , # [codec (index = 10)] Session (runtime_types :: pallet_session :: pallet :: Event ,) , # [codec (index = 11)] Democracy (runtime_types :: pallet_democracy :: pallet :: Event ,) , # [codec (index = 12)] Council (runtime_types :: pallet_collective :: pallet :: Event ,) , # [codec (index = 13)] Elections (runtime_types :: pallet_elections_phragmen :: pallet :: Event ,) , # [codec (index = 14)] Grandpa (runtime_types :: pallet_grandpa :: pallet :: Event ,) , # [codec (index = 15)] Treasury (runtime_types :: pallet_treasury :: pallet :: Event ,) , # [codec (index = 16)] Utility (runtime_types :: pallet_utility :: pallet :: Event ,) , # [codec (index = 17)] Multisig (runtime_types :: pallet_multisig :: pallet :: Event ,) , # [codec (index = 18)] Scheduler (runtime_types :: pallet_scheduler :: pallet :: Event ,) , # [codec (index = 19)] Proxy (runtime_types :: pallet_proxy :: pallet :: Event ,) , # [codec (index = 20)] Assets (runtime_types :: pallet_assets :: pallet :: Event ,) , # [codec (index = 21)] Sudo (runtime_types :: pallet_sudo :: pallet :: Event ,) , # [codec (index = 22)] ImOnline (runtime_types :: pallet_im_online :: pallet :: Event ,) , # [codec (index = 24)] Offences (runtime_types :: pallet_offences :: pallet :: Event ,) , # [codec (index = 26)] Bounties (runtime_types :: pallet_bounties :: pallet :: Event ,) , # [codec (index = 27)] BagsList (runtime_types :: pallet_bags_list :: pallet :: Event ,) , # [codec (index = 28)] HasherBn254 (runtime_types :: pallet_hasher :: pallet :: Event ,) , # [codec (index = 29)] HasherBls381 (runtime_types :: pallet_hasher :: pallet :: Event ,) , # [codec (index = 30)] AssetRegistry (runtime_types :: pallet_asset_registry :: pallet :: Event ,) , # [codec (index = 31)] Currencies (runtime_types :: orml_currencies :: module :: Event ,) , # [codec (index = 32)] Tokens (runtime_types :: orml_tokens :: module :: Event ,) , # [codec (index = 33)] TokenWrapper (runtime_types :: pallet_token_wrapper :: pallet :: Event ,) , # [codec (index = 34)] VerifierBn254 (runtime_types :: pallet_verifier :: pallet :: Event ,) , # [codec (index = 35)] VerifierBls381 (runtime_types :: pallet_verifier :: pallet :: Event ,) , # [codec (index = 36)] MerkleTreeBn254 (runtime_types :: pallet_mt :: pallet :: Event ,) , # [codec (index = 37)] MerkleTreeBls381 (runtime_types :: pallet_mt :: pallet :: Event ,) , # [codec (index = 38)] LinkableTreeBn254 (runtime_types :: pallet_linkable_tree :: pallet :: Event ,) , # [codec (index = 39)] LinkableTreeBls381 (runtime_types :: pallet_linkable_tree :: pallet :: Event ,) , # [codec (index = 40)] MixerBn254 (runtime_types :: pallet_mixer :: pallet :: Event ,) , # [codec (index = 41)] MixerBls381 (runtime_types :: pallet_mixer :: pallet :: Event ,) , # [codec (index = 42)] AnchorBn254 (runtime_types :: pallet_anchor :: pallet :: Event ,) , # [codec (index = 43)] AnchorBls381 (runtime_types :: pallet_anchor :: pallet :: Event ,) , # [codec (index = 44)] AnchorHandlerBn254 (runtime_types :: pallet_anchor_handler :: pallet :: Event ,) , # [codec (index = 45)] AnchorHandlerBls381 (runtime_types :: pallet_anchor_handler :: pallet :: Event ,) , # [codec (index = 46)] Bridge (runtime_types :: pallet_bridge :: pallet :: Event ,) , }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct NposSolution16 {
                votes1: ::std::vec::Vec<(
                    ::core::primitive::u32,
                    ::core::primitive::u16,
                )>,
                votes2: ::std::vec::Vec<(
                    ::core::primitive::u32,
                    (
                        ::core::primitive::u16,
                        runtime_types::sp_arithmetic::per_things::PerU16,
                    ),
                    ::core::primitive::u16,
                )>,
                votes3: ::std::vec::Vec<(
                    ::core::primitive::u32,
                    [(
                        ::core::primitive::u16,
                        runtime_types::sp_arithmetic::per_things::PerU16,
                    ); 2usize],
                    ::core::primitive::u16,
                )>,
                votes4: ::std::vec::Vec<(
                    ::core::primitive::u32,
                    [(
                        ::core::primitive::u16,
                        runtime_types::sp_arithmetic::per_things::PerU16,
                    ); 3usize],
                    ::core::primitive::u16,
                )>,
                votes5: ::std::vec::Vec<(
                    ::core::primitive::u32,
                    [(
                        ::core::primitive::u16,
                        runtime_types::sp_arithmetic::per_things::PerU16,
                    ); 4usize],
                    ::core::primitive::u16,
                )>,
                votes6: ::std::vec::Vec<(
                    ::core::primitive::u32,
                    [(
                        ::core::primitive::u16,
                        runtime_types::sp_arithmetic::per_things::PerU16,
                    ); 5usize],
                    ::core::primitive::u16,
                )>,
                votes7: ::std::vec::Vec<(
                    ::core::primitive::u32,
                    [(
                        ::core::primitive::u16,
                        runtime_types::sp_arithmetic::per_things::PerU16,
                    ); 6usize],
                    ::core::primitive::u16,
                )>,
                votes8: ::std::vec::Vec<(
                    ::core::primitive::u32,
                    [(
                        ::core::primitive::u16,
                        runtime_types::sp_arithmetic::per_things::PerU16,
                    ); 7usize],
                    ::core::primitive::u16,
                )>,
                votes9: ::std::vec::Vec<(
                    ::core::primitive::u32,
                    [(
                        ::core::primitive::u16,
                        runtime_types::sp_arithmetic::per_things::PerU16,
                    ); 8usize],
                    ::core::primitive::u16,
                )>,
                votes10: ::std::vec::Vec<(
                    ::core::primitive::u32,
                    [(
                        ::core::primitive::u16,
                        runtime_types::sp_arithmetic::per_things::PerU16,
                    ); 9usize],
                    ::core::primitive::u16,
                )>,
                votes11: ::std::vec::Vec<(
                    ::core::primitive::u32,
                    [(
                        ::core::primitive::u16,
                        runtime_types::sp_arithmetic::per_things::PerU16,
                    ); 10usize],
                    ::core::primitive::u16,
                )>,
                votes12: ::std::vec::Vec<(
                    ::core::primitive::u32,
                    [(
                        ::core::primitive::u16,
                        runtime_types::sp_arithmetic::per_things::PerU16,
                    ); 11usize],
                    ::core::primitive::u16,
                )>,
                votes13: ::std::vec::Vec<(
                    ::core::primitive::u32,
                    [(
                        ::core::primitive::u16,
                        runtime_types::sp_arithmetic::per_things::PerU16,
                    ); 12usize],
                    ::core::primitive::u16,
                )>,
                votes14: ::std::vec::Vec<(
                    ::core::primitive::u32,
                    [(
                        ::core::primitive::u16,
                        runtime_types::sp_arithmetic::per_things::PerU16,
                    ); 13usize],
                    ::core::primitive::u16,
                )>,
                votes15: ::std::vec::Vec<(
                    ::core::primitive::u32,
                    [(
                        ::core::primitive::u16,
                        runtime_types::sp_arithmetic::per_things::PerU16,
                    ); 14usize],
                    ::core::primitive::u16,
                )>,
                votes16: ::std::vec::Vec<(
                    ::core::primitive::u32,
                    [(
                        ::core::primitive::u16,
                        runtime_types::sp_arithmetic::per_things::PerU16,
                    ); 15usize],
                    ::core::primitive::u16,
                )>,
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub enum OriginCaller {
                #[codec(index = 0)]
                system(
                    runtime_types::frame_system::RawOrigin<
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                ),
                #[codec(index = 12)]
                Council(
                    runtime_types::pallet_collective::RawOrigin<
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                ),
                #[codec(index = 2)]
                Void(runtime_types::sp_core::Void),
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub enum ProxyType {
                #[codec(index = 0)]
                Any,
                #[codec(index = 1)]
                NonTransfer,
                #[codec(index = 2)]
                CancelProxy,
                #[codec(index = 3)]
                Assets,
                #[codec(index = 4)]
                AssetOwner,
                #[codec(index = 5)]
                AssetManager,
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Runtime {}
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct SessionKeys { pub grandpa : runtime_types :: sp_finality_grandpa :: app :: Public , pub babe : runtime_types :: sp_consensus_babe :: app :: Public , pub im_online : runtime_types :: pallet_im_online :: sr25519 :: app_sr25519 :: Public , pub authority_discovery : runtime_types :: sp_authority_discovery :: app :: Public , }
        }
        pub mod finality_grandpa {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Equivocation<_0, _1, _2> {
                pub round_number: ::core::primitive::u64,
                pub identity: _0,
                pub first: (_1, _2),
                pub second: (_1, _2),
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Precommit<_0, _1> {
                pub target_hash: _0,
                pub target_number: _1,
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Prevote<_0, _1> {
                pub target_hash: _0,
                pub target_number: _1,
            }
        }
        pub mod frame_support {
            use super::runtime_types;
            pub mod storage {
                use super::runtime_types;
                pub mod bounded_btree_map {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                    )]
                    pub struct BoundedBTreeMap<_0, _1>(
                        pub ::std::collections::BTreeMap<_0, _1>,
                    );
                }
                pub mod bounded_vec {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                    )]
                    pub struct BoundedVec<_0>(pub ::std::vec::Vec<_0>);
                }
                pub mod weak_bounded_vec {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                    )]
                    pub struct WeakBoundedVec<_0>(pub ::std::vec::Vec<_0>);
                }
            }
            pub mod traits {
                use super::runtime_types;
                pub mod misc {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                    )]
                    pub struct WrapperKeepOpaque<_0>(
                        #[codec(compact)] ::core::primitive::u32,
                        pub _0,
                    );
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                    )]
                    pub struct WrapperOpaque<_0>(
                        #[codec(compact)] ::core::primitive::u32,
                        pub _0,
                    );
                }
                pub mod tokens {
                    use super::runtime_types;
                    pub mod misc {
                        use super::runtime_types;
                        #[derive(
                            :: subxt :: codec :: Encode,
                            :: subxt :: codec :: Decode,
                        )]
                        pub enum BalanceStatus {
                            #[codec(index = 0)]
                            Free,
                            #[codec(index = 1)]
                            Reserved,
                        }
                    }
                }
            }
            pub mod weights {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum DispatchClass {
                    #[codec(index = 0)]
                    Normal,
                    #[codec(index = 1)]
                    Operational,
                    #[codec(index = 2)]
                    Mandatory,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub struct DispatchInfo {
                    pub weight: ::core::primitive::u64,
                    pub class:
                        runtime_types::frame_support::weights::DispatchClass,
                    pub pays_fee: runtime_types::frame_support::weights::Pays,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Pays {
                    #[codec(index = 0)]
                    Yes,
                    #[codec(index = 1)]
                    No,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub struct PerDispatchClass<_0> {
                    pub normal: _0,
                    pub operational: _0,
                    pub mandatory: _0,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub struct RuntimeDbWeight {
                    pub read: ::core::primitive::u64,
                    pub write: ::core::primitive::u64,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub struct WeightToFeeCoefficient<_0> {
                    pub coeff_integer: _0,
                    pub coeff_frac:
                        runtime_types::sp_arithmetic::per_things::Perbill,
                    pub negative: ::core::primitive::bool,
                    pub degree: ::core::primitive::u8,
                }
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct PalletId(pub [::core::primitive::u8; 8usize]);
        }
        pub mod frame_system {
            use super::runtime_types;
            pub mod extensions {
                use super::runtime_types;
                pub mod check_genesis {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                    )]
                    pub struct CheckGenesis {}
                }
                pub mod check_mortality {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                    )]
                    pub struct CheckMortality(
                        pub runtime_types::sp_runtime::generic::era::Era,
                    );
                }
                pub mod check_nonce {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                    )]
                    pub struct CheckNonce(
                        #[codec(compact)] pub ::core::primitive::u32,
                    );
                }
                pub mod check_spec_version {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                    )]
                    pub struct CheckSpecVersion {}
                }
                pub mod check_tx_version {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                    )]
                    pub struct CheckTxVersion {}
                }
                pub mod check_weight {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                    )]
                    pub struct CheckWeight {}
                }
            }
            pub mod limits {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub struct BlockLength {
                    pub max:
                        runtime_types::frame_support::weights::PerDispatchClass<
                            ::core::primitive::u32,
                        >,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub struct BlockWeights { pub base_block : :: core :: primitive :: u64 , pub max_block : :: core :: primitive :: u64 , pub per_class : runtime_types :: frame_support :: weights :: PerDispatchClass < runtime_types :: frame_system :: limits :: WeightsPerClass > , }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub struct WeightsPerClass {
                    pub base_extrinsic: ::core::primitive::u64,
                    pub max_extrinsic:
                        ::core::option::Option<::core::primitive::u64>,
                    pub max_total:
                        ::core::option::Option<::core::primitive::u64>,
                    pub reserved:
                        ::core::option::Option<::core::primitive::u64>,
                }
            }
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Call {
                    # [codec (index = 0)] fill_block { ratio : runtime_types :: sp_arithmetic :: per_things :: Perbill , } , # [codec (index = 1)] remark { remark : :: std :: vec :: Vec < :: core :: primitive :: u8 > , } , # [codec (index = 2)] set_heap_pages { pages : :: core :: primitive :: u64 , } , # [codec (index = 3)] set_code { code : :: std :: vec :: Vec < :: core :: primitive :: u8 > , } , # [codec (index = 4)] set_code_without_checks { code : :: std :: vec :: Vec < :: core :: primitive :: u8 > , } , # [codec (index = 5)] set_changes_trie_config { changes_trie_config : :: core :: option :: Option < runtime_types :: sp_core :: changes_trie :: ChangesTrieConfiguration > , } , # [codec (index = 6)] set_storage { items : :: std :: vec :: Vec < (:: std :: vec :: Vec < :: core :: primitive :: u8 > , :: std :: vec :: Vec < :: core :: primitive :: u8 > ,) > , } , # [codec (index = 7)] kill_storage { keys : :: std :: vec :: Vec < :: std :: vec :: Vec < :: core :: primitive :: u8 > > , } , # [codec (index = 8)] kill_prefix { prefix : :: std :: vec :: Vec < :: core :: primitive :: u8 > , subkeys : :: core :: primitive :: u32 , } , # [codec (index = 9)] remark_with_event { remark : :: std :: vec :: Vec < :: core :: primitive :: u8 > , } , }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    InvalidSpecName,
                    #[codec(index = 1)]
                    SpecVersionNeedsToIncrease,
                    #[codec(index = 2)]
                    FailedToExtractRuntimeVersion,
                    #[codec(index = 3)]
                    NonDefaultComposite,
                    #[codec(index = 4)]
                    NonZeroRefCount,
                    #[codec(index = 5)]
                    CallFiltered,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Event {
                    #[codec(index = 0)]
                    ExtrinsicSuccess(
                        runtime_types::frame_support::weights::DispatchInfo,
                    ),
                    #[codec(index = 1)]
                    ExtrinsicFailed(
                        runtime_types::sp_runtime::DispatchError,
                        runtime_types::frame_support::weights::DispatchInfo,
                    ),
                    #[codec(index = 2)]
                    CodeUpdated,
                    #[codec(index = 3)]
                    NewAccount(::subxt::sp_core::crypto::AccountId32),
                    #[codec(index = 4)]
                    KilledAccount(::subxt::sp_core::crypto::AccountId32),
                    #[codec(index = 5)]
                    Remarked(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::H256,
                    ),
                }
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct AccountInfo<_0, _1> {
                pub nonce: _0,
                pub consumers: _0,
                pub providers: _0,
                pub sufficients: _0,
                pub data: _1,
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct EventRecord<_0, _1> {
                pub phase: runtime_types::frame_system::Phase,
                pub event: _0,
                pub topics: ::std::vec::Vec<_1>,
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct LastRuntimeUpgradeInfo {
                #[codec(compact)]
                pub spec_version: ::core::primitive::u32,
                pub spec_name: ::std::string::String,
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub enum Phase {
                #[codec(index = 0)]
                ApplyExtrinsic(::core::primitive::u32),
                #[codec(index = 1)]
                Finalization,
                #[codec(index = 2)]
                Initialization,
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub enum RawOrigin<_0> {
                #[codec(index = 0)]
                Root,
                #[codec(index = 1)]
                Signed(_0),
                #[codec(index = 2)]
                None,
            }
        }
        pub mod orml_currencies {
            use super::runtime_types;
            pub mod module {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Call {
                    #[codec(index = 0)]
                    transfer {
                        dest: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        currency_id: ::core::primitive::u32,
                        #[codec(compact)]
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 1)]
                    transfer_native_currency {
                        dest: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        #[codec(compact)]
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 2)]
                    update_balance {
                        who: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        currency_id: ::core::primitive::u32,
                        amount: ::core::primitive::i128,
                    },
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    AmountIntoBalanceFailed,
                    #[codec(index = 1)]
                    BalanceTooLow,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Event {
                    #[codec(index = 0)]
                    Transferred(
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 1)]
                    BalanceUpdated(
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::i128,
                    ),
                    #[codec(index = 2)]
                    Deposited(
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 3)]
                    Withdrawn(
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                }
            }
        }
        pub mod orml_tokens {
            use super::runtime_types;
            pub mod module {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Call {
                    #[codec(index = 0)]
                    transfer {
                        dest: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        currency_id: ::core::primitive::u32,
                        #[codec(compact)]
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 1)]
                    transfer_all {
                        dest: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        currency_id: ::core::primitive::u32,
                        keep_alive: ::core::primitive::bool,
                    },
                    #[codec(index = 2)]
                    transfer_keep_alive {
                        dest: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        currency_id: ::core::primitive::u32,
                        #[codec(compact)]
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 3)]
                    force_transfer {
                        source: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        dest: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        currency_id: ::core::primitive::u32,
                        #[codec(compact)]
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 4)]
                    set_balance {
                        who: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        currency_id: ::core::primitive::u32,
                        #[codec(compact)]
                        new_free: ::core::primitive::u128,
                        #[codec(compact)]
                        new_reserved: ::core::primitive::u128,
                    },
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    BalanceTooLow,
                    #[codec(index = 1)]
                    AmountIntoBalanceFailed,
                    #[codec(index = 2)]
                    LiquidityRestrictions,
                    #[codec(index = 3)]
                    MaxLocksExceeded,
                    #[codec(index = 4)]
                    KeepAlive,
                    #[codec(index = 5)]
                    ExistentialDeposit,
                    #[codec(index = 6)]
                    DeadAccount,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Event {
                    #[codec(index = 0)]
                    Endowed(
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 1)]
                    DustLost(
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 2)]
                    Transfer(
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 3)]
                    Reserved(
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 4)]
                    Unreserved(
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 5)]
                    BalanceSet(
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                        ::core::primitive::u128,
                    ),
                }
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct AccountData<_0> {
                pub free: _0,
                pub reserved: _0,
                pub frozen: _0,
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct BalanceLock<_0> {
                pub id: [::core::primitive::u8; 8usize],
                pub amount: _0,
            }
        }
        pub mod pallet_anchor {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Call {
                    #[codec(index = 0)]
                    create {
                        deposit_size: ::core::primitive::u128,
                        max_edges: ::core::primitive::u32,
                        depth: ::core::primitive::u8,
                        asset: ::core::primitive::u32,
                    },
                    #[codec(index = 1)]
                    deposit {
                        tree_id: ::core::primitive::u32,
                        leaf:
                            runtime_types::darkwebb_standalone_runtime::Element,
                    },
                    #[codec(index = 2)]
                    deposit_and_update_linked_anchors {
                        tree_id: ::core::primitive::u32,
                        leaf:
                            runtime_types::darkwebb_standalone_runtime::Element,
                    },
                    #[codec(index = 3)]
                    withdraw {
                        id: ::core::primitive::u32,
                        proof_bytes: ::std::vec::Vec<::core::primitive::u8>,
                        chain_id: ::core::primitive::u32,
                        roots: ::std::vec::Vec<
                            runtime_types::darkwebb_standalone_runtime::Element,
                        >,
                        nullifier_hash:
                            runtime_types::darkwebb_standalone_runtime::Element,
                        recipient: ::subxt::sp_core::crypto::AccountId32,
                        relayer: ::subxt::sp_core::crypto::AccountId32,
                        fee: ::core::primitive::u128,
                        refund: ::core::primitive::u128,
                        commitment:
                            runtime_types::darkwebb_standalone_runtime::Element,
                    },
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    InvalidMerkleRoots,
                    #[codec(index = 1)]
                    UnknownRoot,
                    #[codec(index = 2)]
                    InvalidWithdrawProof,
                    #[codec(index = 3)]
                    NoAnchorFound,
                    #[codec(index = 4)]
                    AlreadyRevealedNullifier,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Event {
                    #[codec(index = 0)]
                    AnchorCreation { tree_id: ::core::primitive::u32 },
                    #[codec(index = 1)]
                    Withdraw {
                        who: ::subxt::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 2)]
                    Deposit {
                        depositor: ::subxt::sp_core::crypto::AccountId32,
                        tree_id: ::core::primitive::u32,
                        leaf:
                            runtime_types::darkwebb_standalone_runtime::Element,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 3)]
                    PostDeposit {
                        depositor: ::subxt::sp_core::crypto::AccountId32,
                        tree_id: ::core::primitive::u32,
                        leaf:
                            runtime_types::darkwebb_standalone_runtime::Element,
                    },
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub struct AnchorMetadata<_0, _1, _2> {
                    pub creator: _0,
                    pub deposit_size: _1,
                    pub asset: _2,
                }
            }
        }
        pub mod pallet_anchor_handler {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Call {
                    # [codec (index = 0)] execute_anchor_create_proposal { deposit_size : :: core :: primitive :: u128 , src_chain_id : :: core :: primitive :: u32 , r_id : [:: core :: primitive :: u8 ; 32usize] , max_edges : :: core :: primitive :: u32 , tree_depth : :: core :: primitive :: u8 , asset : :: core :: primitive :: u32 , } , # [codec (index = 1)] execute_anchor_update_proposal { r_id : [:: core :: primitive :: u8 ; 32usize] , anchor_metadata : runtime_types :: pallet_linkable_tree :: types :: EdgeMetadata < :: core :: primitive :: u32 , runtime_types :: darkwebb_standalone_runtime :: Element , :: core :: primitive :: u32 > , } , }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    InvalidPermissions,
                    #[codec(index = 1)]
                    ResourceIsAlreadyAnchored,
                    #[codec(index = 2)]
                    AnchorHandlerNotFound,
                    #[codec(index = 3)]
                    SourceChainIdNotFound,
                    #[codec(index = 4)]
                    StorageOverflow,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Event {
                    #[codec(index = 0)]
                    AnchorCreated,
                    #[codec(index = 1)]
                    AnchorEdgeAdded,
                    #[codec(index = 2)]
                    AnchorEdgeUpdated,
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub struct UpdateRecord < _0 , _1 , _2 , _3 , _4 > { pub tree_id : _0 , pub resource_id : _1 , pub edge_metadata : runtime_types :: pallet_linkable_tree :: types :: EdgeMetadata < _0 , _3 , _0 > , # [codec (skip)] pub __subxt_unused_type_params : :: core :: marker :: PhantomData < (_2 , _4) > , }
            }
        }
        pub mod pallet_asset_registry {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Call {
                    # [codec (index = 0)] register { name : :: std :: vec :: Vec < :: core :: primitive :: u8 > , asset_type : runtime_types :: pallet_asset_registry :: types :: AssetType < :: core :: primitive :: u32 > , existential_deposit : :: core :: primitive :: u128 , } , # [codec (index = 1)] update { asset_id : :: core :: primitive :: u32 , name : :: std :: vec :: Vec < :: core :: primitive :: u8 > , asset_type : runtime_types :: pallet_asset_registry :: types :: AssetType < :: core :: primitive :: u32 > , existential_deposit : :: core :: option :: Option < :: core :: primitive :: u128 > , } , # [codec (index = 2)] set_metadata { asset_id : :: core :: primitive :: u32 , symbol : :: std :: vec :: Vec < :: core :: primitive :: u8 > , decimals : :: core :: primitive :: u8 , } , # [codec (index = 3)] set_location { asset_id : :: core :: primitive :: u32 , location : () , } , # [codec (index = 4)] add_asset_to_pool { pool : :: std :: vec :: Vec < :: core :: primitive :: u8 > , asset_id : :: core :: primitive :: u32 , } , # [codec (index = 5)] delete_asset_from_pool { pool : :: std :: vec :: Vec < :: core :: primitive :: u8 > , asset_id : :: core :: primitive :: u32 , } , }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    NoIdAvailable,
                    #[codec(index = 1)]
                    AssetNotFound,
                    #[codec(index = 2)]
                    TooLong,
                    #[codec(index = 3)]
                    AssetNotRegistered,
                    #[codec(index = 4)]
                    AssetAlreadyRegistered,
                    #[codec(index = 5)]
                    InvalidSharedAssetLen,
                    #[codec(index = 6)]
                    AssetExistsInPool,
                    #[codec(index = 7)]
                    AssetNotFoundInPool,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Event {
                    # [codec (index = 0)] Registered { asset_id : :: core :: primitive :: u32 , name : runtime_types :: frame_support :: storage :: bounded_vec :: BoundedVec < :: core :: primitive :: u8 > , asset_type : runtime_types :: pallet_asset_registry :: types :: AssetType < :: core :: primitive :: u32 > , } , # [codec (index = 1)] Updated { asset_id : :: core :: primitive :: u32 , name : runtime_types :: frame_support :: storage :: bounded_vec :: BoundedVec < :: core :: primitive :: u8 > , asset_type : runtime_types :: pallet_asset_registry :: types :: AssetType < :: core :: primitive :: u32 > , } , # [codec (index = 2)] MetadataSet { asset_id : :: core :: primitive :: u32 , symbol : runtime_types :: frame_support :: storage :: bounded_vec :: BoundedVec < :: core :: primitive :: u8 > , decimals : :: core :: primitive :: u8 , } , # [codec (index = 3)] LocationSet { asset_id : :: core :: primitive :: u32 , location : () , } , }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub struct AssetDetails<_0, _1, _2> {
                    pub name: _2,
                    pub asset_type:
                        runtime_types::pallet_asset_registry::types::AssetType<
                            _0,
                        >,
                    pub existential_deposit: _1,
                    pub locked: ::core::primitive::bool,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub struct AssetMetadata<_0> {
                    pub symbol: _0,
                    pub decimals: ::core::primitive::u8,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum AssetType<_0> {
                    #[codec(index = 0)]
                    Token,
                    #[codec(index = 1)]
                    PoolShare(::std::vec::Vec<_0>),
                }
            }
        }
        pub mod pallet_assets {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Call {
                    #[codec(index = 0)]
                    create {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        admin: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        min_balance: ::core::primitive::u128,
                    },
                    #[codec(index = 1)]
                    force_create {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        owner: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        is_sufficient: ::core::primitive::bool,
                        #[codec(compact)]
                        min_balance: ::core::primitive::u128,
                    },
                    #[codec(index = 2)]
                    destroy {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        witness:
                            runtime_types::pallet_assets::types::DestroyWitness,
                    },
                    #[codec(index = 3)]
                    mint {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        beneficiary: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        #[codec(compact)]
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 4)]
                    burn {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        who: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        #[codec(compact)]
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 5)]
                    transfer {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        target: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        #[codec(compact)]
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 6)]
                    transfer_keep_alive {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        target: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        #[codec(compact)]
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 7)]
                    force_transfer {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        source: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        dest: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        #[codec(compact)]
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 8)]
                    freeze {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        who: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                    },
                    #[codec(index = 9)]
                    thaw {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        who: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                    },
                    #[codec(index = 10)]
                    freeze_asset {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                    },
                    #[codec(index = 11)]
                    thaw_asset {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                    },
                    #[codec(index = 12)]
                    transfer_ownership {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        owner: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                    },
                    #[codec(index = 13)]
                    set_team {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        issuer: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        admin: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        freezer: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                    },
                    #[codec(index = 14)]
                    set_metadata {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        name: ::std::vec::Vec<::core::primitive::u8>,
                        symbol: ::std::vec::Vec<::core::primitive::u8>,
                        decimals: ::core::primitive::u8,
                    },
                    #[codec(index = 15)]
                    clear_metadata {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                    },
                    #[codec(index = 16)]
                    force_set_metadata {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        name: ::std::vec::Vec<::core::primitive::u8>,
                        symbol: ::std::vec::Vec<::core::primitive::u8>,
                        decimals: ::core::primitive::u8,
                        is_frozen: ::core::primitive::bool,
                    },
                    #[codec(index = 17)]
                    force_clear_metadata {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                    },
                    #[codec(index = 18)]
                    force_asset_status {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        owner: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        issuer: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        admin: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        freezer: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        #[codec(compact)]
                        min_balance: ::core::primitive::u128,
                        is_sufficient: ::core::primitive::bool,
                        is_frozen: ::core::primitive::bool,
                    },
                    #[codec(index = 19)]
                    approve_transfer {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        delegate: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        #[codec(compact)]
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 20)]
                    cancel_approval {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        delegate: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                    },
                    #[codec(index = 21)]
                    force_cancel_approval {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        owner: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        delegate: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                    },
                    #[codec(index = 22)]
                    transfer_approved {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        owner: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        destination: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        #[codec(compact)]
                        amount: ::core::primitive::u128,
                    },
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    BalanceLow,
                    #[codec(index = 1)]
                    BalanceZero,
                    #[codec(index = 2)]
                    NoPermission,
                    #[codec(index = 3)]
                    Unknown,
                    #[codec(index = 4)]
                    Frozen,
                    #[codec(index = 5)]
                    InUse,
                    #[codec(index = 6)]
                    BadWitness,
                    #[codec(index = 7)]
                    MinBalanceZero,
                    #[codec(index = 8)]
                    NoProvider,
                    #[codec(index = 9)]
                    BadMetadata,
                    #[codec(index = 10)]
                    Unapproved,
                    #[codec(index = 11)]
                    WouldDie,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Event {
                    #[codec(index = 0)]
                    Created(
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                    ),
                    #[codec(index = 1)]
                    Issued(
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 2)]
                    Transferred(
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 3)]
                    Burned(
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 4)]
                    TeamChanged(
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                    ),
                    #[codec(index = 5)]
                    OwnerChanged(
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                    ),
                    #[codec(index = 6)]
                    Frozen(
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                    ),
                    #[codec(index = 7)]
                    Thawed(
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                    ),
                    #[codec(index = 8)]
                    AssetFrozen(::core::primitive::u32),
                    #[codec(index = 9)]
                    AssetThawed(::core::primitive::u32),
                    #[codec(index = 10)]
                    Destroyed(::core::primitive::u32),
                    #[codec(index = 11)]
                    ForceCreated(
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                    ),
                    #[codec(index = 12)]
                    MetadataSet(
                        ::core::primitive::u32,
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::core::primitive::u8,
                        ::core::primitive::bool,
                    ),
                    #[codec(index = 13)]
                    MetadataCleared(::core::primitive::u32),
                    #[codec(index = 14)]
                    ApprovedTransfer(
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 15)]
                    ApprovalCancelled(
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                    ),
                    #[codec(index = 16)]
                    TransferredApproved(
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 17)]
                    AssetStatusChanged(::core::primitive::u32),
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub struct Approval<_0, _1> {
                    pub amount: _0,
                    pub deposit: _0,
                    #[codec(skip)]
                    pub __subxt_unused_type_params:
                        ::core::marker::PhantomData<_1>,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub struct AssetBalance<_0, _1> {
                    pub balance: _0,
                    pub is_frozen: ::core::primitive::bool,
                    pub sufficient: ::core::primitive::bool,
                    pub extra: _1,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub struct AssetDetails<_0, _1, _2> {
                    pub owner: _1,
                    pub issuer: _1,
                    pub admin: _1,
                    pub freezer: _1,
                    pub supply: _0,
                    pub deposit: _0,
                    pub min_balance: _0,
                    pub is_sufficient: ::core::primitive::bool,
                    pub accounts: ::core::primitive::u32,
                    pub sufficients: ::core::primitive::u32,
                    pub approvals: ::core::primitive::u32,
                    pub is_frozen: ::core::primitive::bool,
                    #[codec(skip)]
                    pub __subxt_unused_type_params:
                        ::core::marker::PhantomData<_2>,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub struct AssetMetadata<_0, _1> {
                    pub deposit: _0,
                    pub name: _1,
                    pub symbol: _1,
                    pub decimals: ::core::primitive::u8,
                    pub is_frozen: ::core::primitive::bool,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub struct DestroyWitness {
                    #[codec(compact)]
                    pub accounts: ::core::primitive::u32,
                    #[codec(compact)]
                    pub sufficients: ::core::primitive::u32,
                    #[codec(compact)]
                    pub approvals: ::core::primitive::u32,
                }
            }
        }
        pub mod pallet_authorship {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Call {
                    #[codec(index = 0)]
                    set_uncles {
                        new_uncles: ::std::vec::Vec<
                            runtime_types::sp_runtime::generic::header::Header<
                                ::core::primitive::u32,
                                runtime_types::sp_runtime::traits::BlakeTwo256,
                            >,
                        >,
                    },
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    InvalidUncleParent,
                    #[codec(index = 1)]
                    UnclesAlreadySet,
                    #[codec(index = 2)]
                    TooManyUncles,
                    #[codec(index = 3)]
                    GenesisUncle,
                    #[codec(index = 4)]
                    TooHighUncle,
                    #[codec(index = 5)]
                    UncleAlreadyIncluded,
                    #[codec(index = 6)]
                    OldUncle,
                }
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub enum UncleEntryItem<_0, _1, _2> {
                #[codec(index = 0)]
                InclusionHeight(_0),
                #[codec(index = 1)]
                Uncle(_1, ::core::option::Option<_2>),
            }
        }
        pub mod pallet_babe {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Call {
                    # [codec (index = 0)] report_equivocation { equivocation_proof : :: std :: boxed :: Box < runtime_types :: sp_consensus_slots :: EquivocationProof < runtime_types :: sp_runtime :: generic :: header :: Header < :: core :: primitive :: u32 , runtime_types :: sp_runtime :: traits :: BlakeTwo256 > , runtime_types :: sp_consensus_babe :: app :: Public > > , key_owner_proof : runtime_types :: sp_session :: MembershipProof , } , # [codec (index = 1)] report_equivocation_unsigned { equivocation_proof : :: std :: boxed :: Box < runtime_types :: sp_consensus_slots :: EquivocationProof < runtime_types :: sp_runtime :: generic :: header :: Header < :: core :: primitive :: u32 , runtime_types :: sp_runtime :: traits :: BlakeTwo256 > , runtime_types :: sp_consensus_babe :: app :: Public > > , key_owner_proof : runtime_types :: sp_session :: MembershipProof , } , # [codec (index = 2)] plan_config_change { config : runtime_types :: sp_consensus_babe :: digests :: NextConfigDescriptor , } , }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    InvalidEquivocationProof,
                    #[codec(index = 1)]
                    InvalidKeyOwnershipProof,
                    #[codec(index = 2)]
                    DuplicateOffenceReport,
                }
            }
        }
        pub mod pallet_bags_list {
            use super::runtime_types;
            pub mod list {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub struct Bag {
                    pub head: ::core::option::Option<
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                    pub tail: ::core::option::Option<
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub struct Node {
                    pub id: ::subxt::sp_core::crypto::AccountId32,
                    pub prev: ::core::option::Option<
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                    pub next: ::core::option::Option<
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                    pub bag_upper: ::core::primitive::u64,
                }
            }
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Call {
                    #[codec(index = 0)]
                    rebag {
                        dislocated: ::subxt::sp_core::crypto::AccountId32,
                    },
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Event {
                    #[codec(index = 0)]
                    Rebagged(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u64,
                        ::core::primitive::u64,
                    ),
                }
            }
        }
        pub mod pallet_balances {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Call {
                    #[codec(index = 0)]
                    transfer {
                        dest: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 1)]
                    set_balance {
                        who: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        #[codec(compact)]
                        new_free: ::core::primitive::u128,
                        #[codec(compact)]
                        new_reserved: ::core::primitive::u128,
                    },
                    #[codec(index = 2)]
                    force_transfer {
                        source: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        dest: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 3)]
                    transfer_keep_alive {
                        dest: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 4)]
                    transfer_all {
                        dest: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        keep_alive: ::core::primitive::bool,
                    },
                    #[codec(index = 5)]
                    force_unreserve {
                        who: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        amount: ::core::primitive::u128,
                    },
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    VestingBalance,
                    #[codec(index = 1)]
                    LiquidityRestrictions,
                    #[codec(index = 2)]
                    InsufficientBalance,
                    #[codec(index = 3)]
                    ExistentialDeposit,
                    #[codec(index = 4)]
                    KeepAlive,
                    #[codec(index = 5)]
                    ExistingVestingSchedule,
                    #[codec(index = 6)]
                    DeadAccount,
                    #[codec(index = 7)]
                    TooManyReserves,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Event {
                    # [codec (index = 0)] Endowed (:: subxt :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u128 ,) , # [codec (index = 1)] DustLost (:: subxt :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u128 ,) , # [codec (index = 2)] Transfer (:: subxt :: sp_core :: crypto :: AccountId32 , :: subxt :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u128 ,) , # [codec (index = 3)] BalanceSet (:: subxt :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u128 , :: core :: primitive :: u128 ,) , # [codec (index = 4)] Reserved (:: subxt :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u128 ,) , # [codec (index = 5)] Unreserved (:: subxt :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u128 ,) , # [codec (index = 6)] ReserveRepatriated (:: subxt :: sp_core :: crypto :: AccountId32 , :: subxt :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u128 , runtime_types :: frame_support :: traits :: tokens :: misc :: BalanceStatus ,) , # [codec (index = 7)] Deposit (:: subxt :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u128 ,) , # [codec (index = 8)] Withdraw (:: subxt :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u128 ,) , # [codec (index = 9)] Slashed (:: subxt :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u128 ,) , }
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct AccountData<_0> {
                pub free: _0,
                pub reserved: _0,
                pub misc_frozen: _0,
                pub fee_frozen: _0,
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct BalanceLock<_0> {
                pub id: [::core::primitive::u8; 8usize],
                pub amount: _0,
                pub reasons: runtime_types::pallet_balances::Reasons,
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub enum Reasons {
                #[codec(index = 0)]
                Fee,
                #[codec(index = 1)]
                Misc,
                #[codec(index = 2)]
                All,
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub enum Releases {
                #[codec(index = 0)]
                V1_0_0,
                #[codec(index = 1)]
                V2_0_0,
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ReserveData<_0, _1> {
                pub id: _0,
                pub amount: _1,
            }
        }
        pub mod pallet_bounties {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Call {
                    #[codec(index = 0)]
                    propose_bounty {
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                        description: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 1)]
                    approve_bounty {
                        #[codec(compact)]
                        bounty_id: ::core::primitive::u32,
                    },
                    #[codec(index = 2)]
                    propose_curator {
                        #[codec(compact)]
                        bounty_id: ::core::primitive::u32,
                        curator: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        #[codec(compact)]
                        fee: ::core::primitive::u128,
                    },
                    #[codec(index = 3)]
                    unassign_curator {
                        #[codec(compact)]
                        bounty_id: ::core::primitive::u32,
                    },
                    #[codec(index = 4)]
                    accept_curator {
                        #[codec(compact)]
                        bounty_id: ::core::primitive::u32,
                    },
                    #[codec(index = 5)]
                    award_bounty {
                        #[codec(compact)]
                        bounty_id: ::core::primitive::u32,
                        beneficiary: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                    },
                    #[codec(index = 6)]
                    claim_bounty {
                        #[codec(compact)]
                        bounty_id: ::core::primitive::u32,
                    },
                    #[codec(index = 7)]
                    close_bounty {
                        #[codec(compact)]
                        bounty_id: ::core::primitive::u32,
                    },
                    #[codec(index = 8)]
                    extend_bounty_expiry {
                        #[codec(compact)]
                        bounty_id: ::core::primitive::u32,
                        remark: ::std::vec::Vec<::core::primitive::u8>,
                    },
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    InsufficientProposersBalance,
                    #[codec(index = 1)]
                    InvalidIndex,
                    #[codec(index = 2)]
                    ReasonTooBig,
                    #[codec(index = 3)]
                    UnexpectedStatus,
                    #[codec(index = 4)]
                    RequireCurator,
                    #[codec(index = 5)]
                    InvalidValue,
                    #[codec(index = 6)]
                    InvalidFee,
                    #[codec(index = 7)]
                    PendingPayout,
                    #[codec(index = 8)]
                    Premature,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Event {
                    #[codec(index = 0)]
                    BountyProposed(::core::primitive::u32),
                    #[codec(index = 1)]
                    BountyRejected(
                        ::core::primitive::u32,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 2)]
                    BountyBecameActive(::core::primitive::u32),
                    #[codec(index = 3)]
                    BountyAwarded(
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                    ),
                    #[codec(index = 4)]
                    BountyClaimed(
                        ::core::primitive::u32,
                        ::core::primitive::u128,
                        ::subxt::sp_core::crypto::AccountId32,
                    ),
                    #[codec(index = 5)]
                    BountyCanceled(::core::primitive::u32),
                    #[codec(index = 6)]
                    BountyExtended(::core::primitive::u32),
                }
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Bounty<_0, _1, _2> {
                pub proposer: _0,
                pub value: _1,
                pub fee: _1,
                pub curator_deposit: _1,
                pub bond: _1,
                pub status:
                    runtime_types::pallet_bounties::BountyStatus<_0, _2>,
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub enum BountyStatus<_0, _1> {
                #[codec(index = 0)]
                Proposed,
                #[codec(index = 1)]
                Approved,
                #[codec(index = 2)]
                Funded,
                #[codec(index = 3)]
                CuratorProposed { curator: _0 },
                #[codec(index = 4)]
                Active { curator: _0, update_due: _1 },
                #[codec(index = 5)]
                PendingPayout {
                    curator: _0,
                    beneficiary: _0,
                    unlock_at: _1,
                },
            }
        }
        pub mod pallet_bridge {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Call {
                    #[codec(index = 0)]
                    set_maintainer {
                        new_maintainer: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 1)]
                    force_set_maintainer {
                        new_maintainer: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 2)]
                    set_threshold { threshold: ::core::primitive::u32 },
                    #[codec(index = 3)]
                    set_resource {
                        id: [::core::primitive::u8; 32usize],
                        method: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 4)]
                    remove_resource {
                        id: [::core::primitive::u8; 32usize],
                    },
                    #[codec(index = 5)]
                    whitelist_chain { id: ::core::primitive::u32 },
                    #[codec(index = 6)]
                    add_relayer {
                        v: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 7)]
                    remove_relayer {
                        v: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 8)]
                    acknowledge_proposal {
                        nonce: ::core::primitive::u64,
                        src_id: ::core::primitive::u32,
                        r_id: [::core::primitive::u8; 32usize],
                        call: ::std::boxed::Box<
                            runtime_types::darkwebb_standalone_runtime::Call,
                        >,
                    },
                    #[codec(index = 9)]
                    reject_proposal {
                        nonce: ::core::primitive::u64,
                        src_id: ::core::primitive::u32,
                        r_id: [::core::primitive::u8; 32usize],
                        call: ::std::boxed::Box<
                            runtime_types::darkwebb_standalone_runtime::Call,
                        >,
                    },
                    #[codec(index = 10)]
                    eval_vote_state {
                        nonce: ::core::primitive::u64,
                        src_id: ::core::primitive::u32,
                        prop: ::std::boxed::Box<
                            runtime_types::darkwebb_standalone_runtime::Call,
                        >,
                    },
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    InvalidPermissions,
                    #[codec(index = 1)]
                    ThresholdNotSet,
                    #[codec(index = 2)]
                    InvalidChainId,
                    #[codec(index = 3)]
                    InvalidThreshold,
                    #[codec(index = 4)]
                    ChainNotWhitelisted,
                    #[codec(index = 5)]
                    ChainAlreadyWhitelisted,
                    #[codec(index = 6)]
                    ResourceDoesNotExist,
                    #[codec(index = 7)]
                    RelayerAlreadyExists,
                    #[codec(index = 8)]
                    RelayerInvalid,
                    #[codec(index = 9)]
                    MustBeRelayer,
                    #[codec(index = 10)]
                    RelayerAlreadyVoted,
                    #[codec(index = 11)]
                    ProposalAlreadyExists,
                    #[codec(index = 12)]
                    ProposalDoesNotExist,
                    #[codec(index = 13)]
                    ProposalNotComplete,
                    #[codec(index = 14)]
                    ProposalAlreadyComplete,
                    #[codec(index = 15)]
                    ProposalExpired,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Event {
                    #[codec(index = 0)]
                    MaintainerSet {
                        old_maintainer: ::subxt::sp_core::crypto::AccountId32,
                        new_maintainer: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 1)]
                    RelayerThresholdChanged {
                        new_threshold: ::core::primitive::u32,
                    },
                    #[codec(index = 2)]
                    ChainWhitelisted { chain_id: ::core::primitive::u32 },
                    #[codec(index = 3)]
                    RelayerAdded {
                        relayer_id: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 4)]
                    RelayerRemoved {
                        relayer_id: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 5)]
                    VoteFor {
                        chain_id: ::core::primitive::u32,
                        deposit_nonce: ::core::primitive::u64,
                        who: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 6)]
                    VoteAgainst {
                        chain_id: ::core::primitive::u32,
                        deposit_nonce: ::core::primitive::u64,
                        who: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 7)]
                    ProposalApproved {
                        chain_id: ::core::primitive::u32,
                        deposit_nonce: ::core::primitive::u64,
                    },
                    #[codec(index = 8)]
                    ProposalRejected {
                        chain_id: ::core::primitive::u32,
                        deposit_nonce: ::core::primitive::u64,
                    },
                    #[codec(index = 9)]
                    ProposalSucceeded {
                        chain_id: ::core::primitive::u32,
                        deposit_nonce: ::core::primitive::u64,
                    },
                    #[codec(index = 10)]
                    ProposalFailed {
                        chain_id: ::core::primitive::u32,
                        deposit_nonce: ::core::primitive::u64,
                    },
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum ProposalStatus {
                    #[codec(index = 0)]
                    Initiated,
                    #[codec(index = 1)]
                    Approved,
                    #[codec(index = 2)]
                    Rejected,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub struct ProposalVotes<_0, _1> {
                    pub votes_for: ::std::vec::Vec<_0>,
                    pub votes_against: ::std::vec::Vec<_0>,
                    pub status:
                        runtime_types::pallet_bridge::types::ProposalStatus,
                    pub expiry: _1,
                }
            }
        }
        pub mod pallet_collective {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Call {
                    #[codec(index = 0)]
                    set_members {
                        new_members: ::std::vec::Vec<
                            ::subxt::sp_core::crypto::AccountId32,
                        >,
                        prime: ::core::option::Option<
                            ::subxt::sp_core::crypto::AccountId32,
                        >,
                        old_count: ::core::primitive::u32,
                    },
                    #[codec(index = 1)]
                    execute {
                        proposal: ::std::boxed::Box<
                            runtime_types::darkwebb_standalone_runtime::Call,
                        >,
                        #[codec(compact)]
                        length_bound: ::core::primitive::u32,
                    },
                    #[codec(index = 2)]
                    propose {
                        #[codec(compact)]
                        threshold: ::core::primitive::u32,
                        proposal: ::std::boxed::Box<
                            runtime_types::darkwebb_standalone_runtime::Call,
                        >,
                        #[codec(compact)]
                        length_bound: ::core::primitive::u32,
                    },
                    #[codec(index = 3)]
                    vote {
                        proposal: ::subxt::sp_core::H256,
                        #[codec(compact)]
                        index: ::core::primitive::u32,
                        approve: ::core::primitive::bool,
                    },
                    #[codec(index = 4)]
                    close {
                        proposal_hash: ::subxt::sp_core::H256,
                        #[codec(compact)]
                        index: ::core::primitive::u32,
                        #[codec(compact)]
                        proposal_weight_bound: ::core::primitive::u64,
                        #[codec(compact)]
                        length_bound: ::core::primitive::u32,
                    },
                    #[codec(index = 5)]
                    disapprove_proposal {
                        proposal_hash: ::subxt::sp_core::H256,
                    },
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    NotMember,
                    #[codec(index = 1)]
                    DuplicateProposal,
                    #[codec(index = 2)]
                    ProposalMissing,
                    #[codec(index = 3)]
                    WrongIndex,
                    #[codec(index = 4)]
                    DuplicateVote,
                    #[codec(index = 5)]
                    AlreadyInitialized,
                    #[codec(index = 6)]
                    TooEarly,
                    #[codec(index = 7)]
                    TooManyProposals,
                    #[codec(index = 8)]
                    WrongProposalWeight,
                    #[codec(index = 9)]
                    WrongProposalLength,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Event {
                    #[codec(index = 0)]
                    Proposed(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                        ::subxt::sp_core::H256,
                        ::core::primitive::u32,
                    ),
                    #[codec(index = 1)]
                    Voted(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::H256,
                        ::core::primitive::bool,
                        ::core::primitive::u32,
                        ::core::primitive::u32,
                    ),
                    #[codec(index = 2)]
                    Approved(::subxt::sp_core::H256),
                    #[codec(index = 3)]
                    Disapproved(::subxt::sp_core::H256),
                    #[codec(index = 4)]
                    Executed(
                        ::subxt::sp_core::H256,
                        ::core::result::Result<
                            (),
                            runtime_types::sp_runtime::DispatchError,
                        >,
                    ),
                    #[codec(index = 5)]
                    MemberExecuted(
                        ::subxt::sp_core::H256,
                        ::core::result::Result<
                            (),
                            runtime_types::sp_runtime::DispatchError,
                        >,
                    ),
                    #[codec(index = 6)]
                    Closed(
                        ::subxt::sp_core::H256,
                        ::core::primitive::u32,
                        ::core::primitive::u32,
                    ),
                }
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub enum RawOrigin<_0> {
                #[codec(index = 0)]
                Members(::core::primitive::u32, ::core::primitive::u32),
                #[codec(index = 1)]
                Member(_0),
                #[codec(index = 2)]
                _Phantom,
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Votes<_0, _1> {
                pub index: _1,
                pub threshold: _1,
                pub ayes: ::std::vec::Vec<_0>,
                pub nays: ::std::vec::Vec<_0>,
                pub end: _1,
            }
        }
        pub mod pallet_democracy {
            use super::runtime_types;
            pub mod conviction {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Conviction {
                    #[codec(index = 0)]
                    None,
                    #[codec(index = 1)]
                    Locked1x,
                    #[codec(index = 2)]
                    Locked2x,
                    #[codec(index = 3)]
                    Locked3x,
                    #[codec(index = 4)]
                    Locked4x,
                    #[codec(index = 5)]
                    Locked5x,
                    #[codec(index = 6)]
                    Locked6x,
                }
            }
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Call {
                    # [codec (index = 0)] propose { proposal_hash : :: subxt :: sp_core :: H256 , # [codec (compact)] value : :: core :: primitive :: u128 , } , # [codec (index = 1)] second { # [codec (compact)] proposal : :: core :: primitive :: u32 , # [codec (compact)] seconds_upper_bound : :: core :: primitive :: u32 , } , # [codec (index = 2)] vote { # [codec (compact)] ref_index : :: core :: primitive :: u32 , vote : runtime_types :: pallet_democracy :: vote :: AccountVote < :: core :: primitive :: u128 > , } , # [codec (index = 3)] emergency_cancel { ref_index : :: core :: primitive :: u32 , } , # [codec (index = 4)] external_propose { proposal_hash : :: subxt :: sp_core :: H256 , } , # [codec (index = 5)] external_propose_majority { proposal_hash : :: subxt :: sp_core :: H256 , } , # [codec (index = 6)] external_propose_default { proposal_hash : :: subxt :: sp_core :: H256 , } , # [codec (index = 7)] fast_track { proposal_hash : :: subxt :: sp_core :: H256 , voting_period : :: core :: primitive :: u32 , delay : :: core :: primitive :: u32 , } , # [codec (index = 8)] veto_external { proposal_hash : :: subxt :: sp_core :: H256 , } , # [codec (index = 9)] cancel_referendum { # [codec (compact)] ref_index : :: core :: primitive :: u32 , } , # [codec (index = 10)] cancel_queued { which : :: core :: primitive :: u32 , } , # [codec (index = 11)] delegate { to : :: subxt :: sp_core :: crypto :: AccountId32 , conviction : runtime_types :: pallet_democracy :: conviction :: Conviction , balance : :: core :: primitive :: u128 , } , # [codec (index = 12)] undelegate , # [codec (index = 13)] clear_public_proposals , # [codec (index = 14)] note_preimage { encoded_proposal : :: std :: vec :: Vec < :: core :: primitive :: u8 > , } , # [codec (index = 15)] note_preimage_operational { encoded_proposal : :: std :: vec :: Vec < :: core :: primitive :: u8 > , } , # [codec (index = 16)] note_imminent_preimage { encoded_proposal : :: std :: vec :: Vec < :: core :: primitive :: u8 > , } , # [codec (index = 17)] note_imminent_preimage_operational { encoded_proposal : :: std :: vec :: Vec < :: core :: primitive :: u8 > , } , # [codec (index = 18)] reap_preimage { proposal_hash : :: subxt :: sp_core :: H256 , # [codec (compact)] proposal_len_upper_bound : :: core :: primitive :: u32 , } , # [codec (index = 19)] unlock { target : :: subxt :: sp_core :: crypto :: AccountId32 , } , # [codec (index = 20)] remove_vote { index : :: core :: primitive :: u32 , } , # [codec (index = 21)] remove_other_vote { target : :: subxt :: sp_core :: crypto :: AccountId32 , index : :: core :: primitive :: u32 , } , # [codec (index = 22)] enact_proposal { proposal_hash : :: subxt :: sp_core :: H256 , index : :: core :: primitive :: u32 , } , # [codec (index = 23)] blacklist { proposal_hash : :: subxt :: sp_core :: H256 , maybe_ref_index : :: core :: option :: Option < :: core :: primitive :: u32 > , } , # [codec (index = 24)] cancel_proposal { # [codec (compact)] prop_index : :: core :: primitive :: u32 , } , }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    ValueLow,
                    #[codec(index = 1)]
                    ProposalMissing,
                    #[codec(index = 2)]
                    AlreadyCanceled,
                    #[codec(index = 3)]
                    DuplicateProposal,
                    #[codec(index = 4)]
                    ProposalBlacklisted,
                    #[codec(index = 5)]
                    NotSimpleMajority,
                    #[codec(index = 6)]
                    InvalidHash,
                    #[codec(index = 7)]
                    NoProposal,
                    #[codec(index = 8)]
                    AlreadyVetoed,
                    #[codec(index = 9)]
                    DuplicatePreimage,
                    #[codec(index = 10)]
                    NotImminent,
                    #[codec(index = 11)]
                    TooEarly,
                    #[codec(index = 12)]
                    Imminent,
                    #[codec(index = 13)]
                    PreimageMissing,
                    #[codec(index = 14)]
                    ReferendumInvalid,
                    #[codec(index = 15)]
                    PreimageInvalid,
                    #[codec(index = 16)]
                    NoneWaiting,
                    #[codec(index = 17)]
                    NotVoter,
                    #[codec(index = 18)]
                    NoPermission,
                    #[codec(index = 19)]
                    AlreadyDelegating,
                    #[codec(index = 20)]
                    InsufficientFunds,
                    #[codec(index = 21)]
                    NotDelegating,
                    #[codec(index = 22)]
                    VotesExist,
                    #[codec(index = 23)]
                    InstantNotAllowed,
                    #[codec(index = 24)]
                    Nonsense,
                    #[codec(index = 25)]
                    WrongUpperBound,
                    #[codec(index = 26)]
                    MaxVotesReached,
                    #[codec(index = 27)]
                    TooManyProposals,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Event {
                    # [codec (index = 0)] Proposed (:: core :: primitive :: u32 , :: core :: primitive :: u128 ,) , # [codec (index = 1)] Tabled (:: core :: primitive :: u32 , :: core :: primitive :: u128 , :: std :: vec :: Vec < :: subxt :: sp_core :: crypto :: AccountId32 > ,) , # [codec (index = 2)] ExternalTabled , # [codec (index = 3)] Started (:: core :: primitive :: u32 , runtime_types :: pallet_democracy :: vote_threshold :: VoteThreshold ,) , # [codec (index = 4)] Passed (:: core :: primitive :: u32 ,) , # [codec (index = 5)] NotPassed (:: core :: primitive :: u32 ,) , # [codec (index = 6)] Cancelled (:: core :: primitive :: u32 ,) , # [codec (index = 7)] Executed (:: core :: primitive :: u32 , :: core :: result :: Result < () , runtime_types :: sp_runtime :: DispatchError > ,) , # [codec (index = 8)] Delegated (:: subxt :: sp_core :: crypto :: AccountId32 , :: subxt :: sp_core :: crypto :: AccountId32 ,) , # [codec (index = 9)] Undelegated (:: subxt :: sp_core :: crypto :: AccountId32 ,) , # [codec (index = 10)] Vetoed (:: subxt :: sp_core :: crypto :: AccountId32 , :: subxt :: sp_core :: H256 , :: core :: primitive :: u32 ,) , # [codec (index = 11)] PreimageNoted (:: subxt :: sp_core :: H256 , :: subxt :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u128 ,) , # [codec (index = 12)] PreimageUsed (:: subxt :: sp_core :: H256 , :: subxt :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u128 ,) , # [codec (index = 13)] PreimageInvalid (:: subxt :: sp_core :: H256 , :: core :: primitive :: u32 ,) , # [codec (index = 14)] PreimageMissing (:: subxt :: sp_core :: H256 , :: core :: primitive :: u32 ,) , # [codec (index = 15)] PreimageReaped (:: subxt :: sp_core :: H256 , :: subxt :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u128 , :: subxt :: sp_core :: crypto :: AccountId32 ,) , # [codec (index = 16)] Blacklisted (:: subxt :: sp_core :: H256 ,) , }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub struct Delegations<_0> {
                    pub votes: _0,
                    pub capital: _0,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum ReferendumInfo<_0, _1, _2> {
                    # [codec (index = 0)] Ongoing (runtime_types :: pallet_democracy :: types :: ReferendumStatus < _0 , _1 , _2 > ,) , # [codec (index = 1)] Finished { approved : :: core :: primitive :: bool , end : _0 , } , }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub struct ReferendumStatus < _0 , _1 , _2 > { pub end : _0 , pub proposal_hash : _1 , pub threshold : runtime_types :: pallet_democracy :: vote_threshold :: VoteThreshold , pub delay : _0 , pub tally : runtime_types :: pallet_democracy :: types :: Tally < _2 > , }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub struct Tally<_0> {
                    pub ayes: _0,
                    pub nays: _0,
                    pub turnout: _0,
                }
            }
            pub mod vote {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum AccountVote<_0> {
                    #[codec(index = 0)]
                    Standard {
                        vote: runtime_types::pallet_democracy::vote::Vote,
                        balance: _0,
                    },
                    #[codec(index = 1)]
                    Split { aye: _0, nay: _0 },
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub struct PriorLock<_0, _1>(pub _0, pub _1);
                #[derive(
                    :: subxt :: codec :: CompactAs,
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                )]
                pub struct Vote(::core::primitive::u8);
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Voting<_0, _1, _2> {
                    # [codec (index = 0)] Direct { votes : :: std :: vec :: Vec < (_2 , runtime_types :: pallet_democracy :: vote :: AccountVote < _0 > ,) > , delegations : runtime_types :: pallet_democracy :: types :: Delegations < _0 > , prior : runtime_types :: pallet_democracy :: vote :: PriorLock < _2 , _0 > , } , # [codec (index = 1)] Delegating { balance : _0 , target : _1 , conviction : runtime_types :: pallet_democracy :: conviction :: Conviction , delegations : runtime_types :: pallet_democracy :: types :: Delegations < _0 > , prior : runtime_types :: pallet_democracy :: vote :: PriorLock < _2 , _0 > , } , }
            }
            pub mod vote_threshold {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum VoteThreshold {
                    #[codec(index = 0)]
                    SuperMajorityApprove,
                    #[codec(index = 1)]
                    SuperMajorityAgainst,
                    #[codec(index = 2)]
                    SimpleMajority,
                }
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub enum PreimageStatus<_0, _1, _2> {
                #[codec(index = 0)]
                Missing(_2),
                #[codec(index = 1)]
                Available {
                    data: ::std::vec::Vec<::core::primitive::u8>,
                    provider: _0,
                    deposit: _1,
                    since: _2,
                    expiry: ::core::option::Option<_2>,
                },
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub enum Releases {
                #[codec(index = 0)]
                V1,
            }
        }
        pub mod pallet_election_provider_multi_phase {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Call {
                    # [codec (index = 0)] submit_unsigned { raw_solution : :: std :: boxed :: Box < runtime_types :: pallet_election_provider_multi_phase :: RawSolution < runtime_types :: darkwebb_standalone_runtime :: NposSolution16 > > , witness : runtime_types :: pallet_election_provider_multi_phase :: SolutionOrSnapshotSize , } , # [codec (index = 1)] set_minimum_untrusted_score { maybe_next_score : :: core :: option :: Option < [:: core :: primitive :: u128 ; 3usize] > , } , # [codec (index = 2)] set_emergency_election_result { supports : :: std :: vec :: Vec < (:: subxt :: sp_core :: crypto :: AccountId32 , runtime_types :: sp_npos_elections :: Support < :: subxt :: sp_core :: crypto :: AccountId32 > ,) > , } , # [codec (index = 3)] submit { raw_solution : :: std :: boxed :: Box < runtime_types :: pallet_election_provider_multi_phase :: RawSolution < runtime_types :: darkwebb_standalone_runtime :: NposSolution16 > > , num_signed_submissions : :: core :: primitive :: u32 , } , }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    PreDispatchEarlySubmission,
                    #[codec(index = 1)]
                    PreDispatchWrongWinnerCount,
                    #[codec(index = 2)]
                    PreDispatchWeakSubmission,
                    #[codec(index = 3)]
                    SignedQueueFull,
                    #[codec(index = 4)]
                    SignedCannotPayDeposit,
                    #[codec(index = 5)]
                    SignedInvalidWitness,
                    #[codec(index = 6)]
                    SignedTooMuchWeight,
                    #[codec(index = 7)]
                    OcwCallWrongEra,
                    #[codec(index = 8)]
                    MissingSnapshotMetadata,
                    #[codec(index = 9)]
                    InvalidSubmissionIndex,
                    #[codec(index = 10)]
                    CallNotAllowed,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Event {
                    # [codec (index = 0)] SolutionStored (runtime_types :: pallet_election_provider_multi_phase :: ElectionCompute , :: core :: primitive :: bool ,) , # [codec (index = 1)] ElectionFinalized (:: core :: option :: Option < runtime_types :: pallet_election_provider_multi_phase :: ElectionCompute > ,) , # [codec (index = 2)] Rewarded (:: subxt :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u128 ,) , # [codec (index = 3)] Slashed (:: subxt :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u128 ,) , # [codec (index = 4)] SignedPhaseStarted (:: core :: primitive :: u32 ,) , # [codec (index = 5)] UnsignedPhaseStarted (:: core :: primitive :: u32 ,) , }
            }
            pub mod signed {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub struct SignedSubmission < _0 , _1 , _2 > { pub who : _0 , pub deposit : _1 , pub raw_solution : runtime_types :: pallet_election_provider_multi_phase :: RawSolution < _2 > , pub reward : _1 , }
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub enum ElectionCompute {
                #[codec(index = 0)]
                OnChain,
                #[codec(index = 1)]
                Signed,
                #[codec(index = 2)]
                Unsigned,
                #[codec(index = 3)]
                Fallback,
                #[codec(index = 4)]
                Emergency,
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub enum Phase<_0> {
                #[codec(index = 0)]
                Off,
                #[codec(index = 1)]
                Signed,
                #[codec(index = 2)]
                Unsigned((::core::primitive::bool, _0)),
                #[codec(index = 3)]
                Emergency,
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct RawSolution<_0> {
                pub solution: _0,
                pub score: [::core::primitive::u128; 3usize],
                pub round: ::core::primitive::u32,
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ReadySolution < _0 > { pub supports : :: std :: vec :: Vec < (_0 , runtime_types :: sp_npos_elections :: Support < _0 > ,) > , pub score : [:: core :: primitive :: u128 ; 3usize] , pub compute : runtime_types :: pallet_election_provider_multi_phase :: ElectionCompute , }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct RoundSnapshot<_0> {
                pub voters: ::std::vec::Vec<(
                    _0,
                    ::core::primitive::u64,
                    ::std::vec::Vec<_0>,
                )>,
                pub targets: ::std::vec::Vec<_0>,
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct SolutionOrSnapshotSize {
                #[codec(compact)]
                pub voters: ::core::primitive::u32,
                #[codec(compact)]
                pub targets: ::core::primitive::u32,
            }
        }
        pub mod pallet_elections_phragmen {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Call {
                    # [codec (index = 0)] vote { votes : :: std :: vec :: Vec < :: subxt :: sp_core :: crypto :: AccountId32 > , # [codec (compact)] value : :: core :: primitive :: u128 , } , # [codec (index = 1)] remove_voter , # [codec (index = 2)] submit_candidacy { # [codec (compact)] candidate_count : :: core :: primitive :: u32 , } , # [codec (index = 3)] renounce_candidacy { renouncing : runtime_types :: pallet_elections_phragmen :: Renouncing , } , # [codec (index = 4)] remove_member { who : :: subxt :: sp_runtime :: MultiAddress < :: subxt :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u32 > , has_replacement : :: core :: primitive :: bool , } , # [codec (index = 5)] clean_defunct_voters { num_voters : :: core :: primitive :: u32 , num_defunct : :: core :: primitive :: u32 , } , }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    UnableToVote,
                    #[codec(index = 1)]
                    NoVotes,
                    #[codec(index = 2)]
                    TooManyVotes,
                    #[codec(index = 3)]
                    MaximumVotesExceeded,
                    #[codec(index = 4)]
                    LowBalance,
                    #[codec(index = 5)]
                    UnableToPayBond,
                    #[codec(index = 6)]
                    MustBeVoter,
                    #[codec(index = 7)]
                    ReportSelf,
                    #[codec(index = 8)]
                    DuplicatedCandidate,
                    #[codec(index = 9)]
                    MemberSubmit,
                    #[codec(index = 10)]
                    RunnerUpSubmit,
                    #[codec(index = 11)]
                    InsufficientCandidateFunds,
                    #[codec(index = 12)]
                    NotMember,
                    #[codec(index = 13)]
                    InvalidWitnessData,
                    #[codec(index = 14)]
                    InvalidVoteCount,
                    #[codec(index = 15)]
                    InvalidRenouncing,
                    #[codec(index = 16)]
                    InvalidReplacement,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Event {
                    #[codec(index = 0)]
                    NewTerm(
                        ::std::vec::Vec<(
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        )>,
                    ),
                    #[codec(index = 1)]
                    EmptyTerm,
                    #[codec(index = 2)]
                    ElectionError,
                    #[codec(index = 3)]
                    MemberKicked(::subxt::sp_core::crypto::AccountId32),
                    #[codec(index = 4)]
                    Renounced(::subxt::sp_core::crypto::AccountId32),
                    #[codec(index = 5)]
                    CandidateSlashed(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 6)]
                    SeatHolderSlashed(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                }
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub enum Renouncing {
                #[codec(index = 0)]
                Member,
                #[codec(index = 1)]
                RunnerUp,
                #[codec(index = 2)]
                Candidate(#[codec(compact)] ::core::primitive::u32),
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct SeatHolder<_0, _1> {
                pub who: _0,
                pub stake: _1,
                pub deposit: _1,
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Voter<_0, _1> {
                pub votes: ::std::vec::Vec<_0>,
                pub stake: _1,
                pub deposit: _1,
            }
        }
        pub mod pallet_grandpa {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Call {
                    # [codec (index = 0)] report_equivocation { equivocation_proof : :: std :: boxed :: Box < runtime_types :: sp_finality_grandpa :: EquivocationProof < :: subxt :: sp_core :: H256 , :: core :: primitive :: u32 > > , key_owner_proof : runtime_types :: sp_session :: MembershipProof , } , # [codec (index = 1)] report_equivocation_unsigned { equivocation_proof : :: std :: boxed :: Box < runtime_types :: sp_finality_grandpa :: EquivocationProof < :: subxt :: sp_core :: H256 , :: core :: primitive :: u32 > > , key_owner_proof : runtime_types :: sp_session :: MembershipProof , } , # [codec (index = 2)] note_stalled { delay : :: core :: primitive :: u32 , best_finalized_block_number : :: core :: primitive :: u32 , } , }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    PauseFailed,
                    #[codec(index = 1)]
                    ResumeFailed,
                    #[codec(index = 2)]
                    ChangePending,
                    #[codec(index = 3)]
                    TooSoon,
                    #[codec(index = 4)]
                    InvalidKeyOwnershipProof,
                    #[codec(index = 5)]
                    InvalidEquivocationProof,
                    #[codec(index = 6)]
                    DuplicateOffenceReport,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Event {
                    #[codec(index = 0)]
                    NewAuthorities(
                        ::std::vec::Vec<(
                            runtime_types::sp_finality_grandpa::app::Public,
                            ::core::primitive::u64,
                        )>,
                    ),
                    #[codec(index = 1)]
                    Paused,
                    #[codec(index = 2)]
                    Resumed,
                }
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct StoredPendingChange < _0 > { pub scheduled_at : _0 , pub delay : _0 , pub next_authorities : runtime_types :: frame_support :: storage :: weak_bounded_vec :: WeakBoundedVec < (runtime_types :: sp_finality_grandpa :: app :: Public , :: core :: primitive :: u64 ,) > , pub forced : :: core :: option :: Option < _0 > , }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub enum StoredState<_0> {
                #[codec(index = 0)]
                Live,
                #[codec(index = 1)]
                PendingPause { scheduled_at: _0, delay: _0 },
                #[codec(index = 2)]
                Paused,
                #[codec(index = 3)]
                PendingResume { scheduled_at: _0, delay: _0 },
            }
        }
        pub mod pallet_hasher {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Call {
                    #[codec(index = 0)]
                    set_parameters {
                        parameters: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 1)]
                    set_maintainer {
                        new_maintainer: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 2)]
                    force_set_parameters {
                        parameters: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 3)]
                    force_set_maintainer {
                        new_maintainer: ::subxt::sp_core::crypto::AccountId32,
                    },
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    ParametersNotInitialized,
                    #[codec(index = 1)]
                    InvalidParameters,
                    #[codec(index = 2)]
                    InvalidPermissions,
                    #[codec(index = 3)]
                    HashError,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Event {
                    #[codec(index = 0)]
                    ParametersSet {
                        who: ::subxt::sp_core::crypto::AccountId32,
                        parameters: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 1)]
                    MaintainerSet {
                        old_maintainer: ::subxt::sp_core::crypto::AccountId32,
                        new_maintainer: ::subxt::sp_core::crypto::AccountId32,
                    },
                }
            }
        }
        pub mod pallet_im_online {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Call {
                    # [codec (index = 0)] heartbeat { heartbeat : runtime_types :: pallet_im_online :: Heartbeat < :: core :: primitive :: u32 > , signature : runtime_types :: pallet_im_online :: sr25519 :: app_sr25519 :: Signature , } , }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    InvalidKey,
                    #[codec(index = 1)]
                    DuplicatedHeartbeat,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Event {
                    # [codec (index = 0)] HeartbeatReceived (runtime_types :: pallet_im_online :: sr25519 :: app_sr25519 :: Public ,) , # [codec (index = 1)] AllGood , # [codec (index = 2)] SomeOffline (:: std :: vec :: Vec < (:: subxt :: sp_core :: crypto :: AccountId32 , runtime_types :: pallet_staking :: Exposure < :: subxt :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u128 > ,) > ,) , }
            }
            pub mod sr25519 {
                use super::runtime_types;
                pub mod app_sr25519 {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                    )]
                    pub struct Public(
                        pub runtime_types::sp_core::sr25519::Public,
                    );
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                    )]
                    pub struct Signature(
                        pub runtime_types::sp_core::sr25519::Signature,
                    );
                }
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct BoundedOpaqueNetworkState { pub peer_id : runtime_types :: frame_support :: storage :: weak_bounded_vec :: WeakBoundedVec < :: core :: primitive :: u8 > , pub external_addresses : runtime_types :: frame_support :: storage :: weak_bounded_vec :: WeakBoundedVec < runtime_types :: frame_support :: storage :: weak_bounded_vec :: WeakBoundedVec < :: core :: primitive :: u8 > > , }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Heartbeat<_0> {
                pub block_number: _0,
                pub network_state:
                    runtime_types::sp_core::offchain::OpaqueNetworkState,
                pub session_index: _0,
                pub authority_index: _0,
                pub validators_len: _0,
            }
        }
        pub mod pallet_indices {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Call {
                    #[codec(index = 0)]
                    claim { index: ::core::primitive::u32 },
                    #[codec(index = 1)]
                    transfer {
                        new: ::subxt::sp_core::crypto::AccountId32,
                        index: ::core::primitive::u32,
                    },
                    #[codec(index = 2)]
                    free { index: ::core::primitive::u32 },
                    #[codec(index = 3)]
                    force_transfer {
                        new: ::subxt::sp_core::crypto::AccountId32,
                        index: ::core::primitive::u32,
                        freeze: ::core::primitive::bool,
                    },
                    #[codec(index = 4)]
                    freeze { index: ::core::primitive::u32 },
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    NotAssigned,
                    #[codec(index = 1)]
                    NotOwner,
                    #[codec(index = 2)]
                    InUse,
                    #[codec(index = 3)]
                    NotTransfer,
                    #[codec(index = 4)]
                    Permanent,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Event {
                    #[codec(index = 0)]
                    IndexAssigned(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    ),
                    #[codec(index = 1)]
                    IndexFreed(::core::primitive::u32),
                    #[codec(index = 2)]
                    IndexFrozen(
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                    ),
                }
            }
        }
        pub mod pallet_linkable_tree {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Call {
                    #[codec(index = 0)]
                    create {
                        max_edges: ::core::primitive::u32,
                        depth: ::core::primitive::u8,
                    },
                    #[codec(index = 1)]
                    set_maintainer {
                        new_maintainer: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 2)]
                    force_set_maintainer {
                        new_maintainer: ::subxt::sp_core::crypto::AccountId32,
                    },
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    UnknownRoot,
                    #[codec(index = 1)]
                    InvalidPermissions,
                    #[codec(index = 2)]
                    InvalidMerkleRoots,
                    #[codec(index = 3)]
                    InvalidNeighborWithdrawRoot,
                    #[codec(index = 4)]
                    TooManyEdges,
                    #[codec(index = 5)]
                    EdgeAlreadyExists,
                    #[codec(index = 6)]
                    EdgeDoesntExists,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Event {
                    #[codec(index = 0)]
                    MaintainerSet {
                        old_maintainer: ::subxt::sp_core::crypto::AccountId32,
                        new_maintainer: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 1)]
                    LinkableTreeCreation { tree_id: ::core::primitive::u32 },
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub struct EdgeMetadata<_0, _1, _2> {
                    pub src_chain_id: _0,
                    pub root: _1,
                    pub latest_leaf_index: _0,
                    #[codec(skip)]
                    pub __subxt_unused_type_params:
                        ::core::marker::PhantomData<_2>,
                }
            }
        }
        pub mod pallet_mixer {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Call {
                    #[codec(index = 0)]
                    create {
                        deposit_size: ::core::primitive::u128,
                        depth: ::core::primitive::u8,
                        asset: ::core::primitive::u32,
                    },
                    #[codec(index = 1)]
                    set_maintainer {
                        new_maintainer: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 2)]
                    force_set_maintainer {
                        new_maintainer: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 3)]
                    deposit {
                        tree_id: ::core::primitive::u32,
                        leaf:
                            runtime_types::darkwebb_standalone_runtime::Element,
                    },
                    #[codec(index = 4)]
                    withdraw {
                        id: ::core::primitive::u32,
                        proof_bytes: ::std::vec::Vec<::core::primitive::u8>,
                        root:
                            runtime_types::darkwebb_standalone_runtime::Element,
                        nullifier_hash:
                            runtime_types::darkwebb_standalone_runtime::Element,
                        recipient: ::subxt::sp_core::crypto::AccountId32,
                        relayer: ::subxt::sp_core::crypto::AccountId32,
                        fee: ::core::primitive::u128,
                        refund: ::core::primitive::u128,
                    },
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    InvalidPermissions,
                    #[codec(index = 1)]
                    InvalidWithdrawProof,
                    #[codec(index = 2)]
                    AlreadyRevealedNullifier,
                    #[codec(index = 3)]
                    UnknownRoot,
                    #[codec(index = 4)]
                    NoMixerFound,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Event {
                    #[codec(index = 0)]
                    MaintainerSet {
                        old_maintainer: ::subxt::sp_core::crypto::AccountId32,
                        new_maintainer: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 1)]
                    MixerCreation { tree_id: ::core::primitive::u32 },
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub struct MixerMetadata<_0, _1, _2> {
                    pub creator: _0,
                    pub deposit_size: _1,
                    pub asset: _2,
                }
            }
        }
        pub mod pallet_mt {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Call {
                    #[codec(index = 0)]
                    create { depth: ::core::primitive::u8 },
                    #[codec(index = 1)]
                    insert {
                        tree_id: ::core::primitive::u32,
                        leaf:
                            runtime_types::darkwebb_standalone_runtime::Element,
                    },
                    #[codec(index = 2)]
                    set_maintainer {
                        new_maintainer: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 3)]
                    force_set_maintainer {
                        new_maintainer: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 4)]
                    force_set_default_hashes {
                        default_hashes: ::std::vec::Vec<
                            runtime_types::darkwebb_standalone_runtime::Element,
                        >,
                    },
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    InvalidPermissions,
                    #[codec(index = 1)]
                    InvalidTreeDepth,
                    #[codec(index = 2)]
                    InvalidLeafIndex,
                    #[codec(index = 3)]
                    ExceedsMaxLeaves,
                    #[codec(index = 4)]
                    TreeDoesntExist,
                    #[codec(index = 5)]
                    ExceedsMaxDefaultHashes,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Event {
                    #[codec(index = 0)]
                    MaintainerSet {
                        old_maintainer: ::subxt::sp_core::crypto::AccountId32,
                        new_maintainer: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 1)]
                    TreeCreation {
                        tree_id: ::core::primitive::u32,
                        who: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 2)]
                    LeafInsertion {
                        tree_id: ::core::primitive::u32,
                        leaf_index: ::core::primitive::u32,
                        leaf:
                            runtime_types::darkwebb_standalone_runtime::Element,
                    },
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub struct TreeMetadata<_0, _1, _2> {
                    pub creator: _0,
                    pub paused: ::core::primitive::bool,
                    pub leaf_count: _1,
                    pub max_leaves: _1,
                    pub depth: ::core::primitive::u8,
                    pub root: _2,
                    pub edge_nodes: ::std::vec::Vec<_2>,
                }
            }
        }
        pub mod pallet_multisig {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Call {
                    #[codec(index = 0)]
                    as_multi_threshold_1 {
                        other_signatories: ::std::vec::Vec<
                            ::subxt::sp_core::crypto::AccountId32,
                        >,
                        call: ::std::boxed::Box<
                            runtime_types::darkwebb_standalone_runtime::Call,
                        >,
                    },
                    #[codec(index = 1)]
                    as_multi {
                        threshold: ::core::primitive::u16,
                        other_signatories: ::std::vec::Vec<
                            ::subxt::sp_core::crypto::AccountId32,
                        >,
                        maybe_timepoint: ::core::option::Option<
                            runtime_types::pallet_multisig::Timepoint<
                                ::core::primitive::u32,
                            >,
                        >,
                        call: ::subxt::WrapperKeepOpaque<
                            runtime_types::darkwebb_standalone_runtime::Call,
                        >,
                        store_call: ::core::primitive::bool,
                        max_weight: ::core::primitive::u64,
                    },
                    #[codec(index = 2)]
                    approve_as_multi {
                        threshold: ::core::primitive::u16,
                        other_signatories: ::std::vec::Vec<
                            ::subxt::sp_core::crypto::AccountId32,
                        >,
                        maybe_timepoint: ::core::option::Option<
                            runtime_types::pallet_multisig::Timepoint<
                                ::core::primitive::u32,
                            >,
                        >,
                        call_hash: [::core::primitive::u8; 32usize],
                        max_weight: ::core::primitive::u64,
                    },
                    #[codec(index = 3)]
                    cancel_as_multi {
                        threshold: ::core::primitive::u16,
                        other_signatories: ::std::vec::Vec<
                            ::subxt::sp_core::crypto::AccountId32,
                        >,
                        timepoint: runtime_types::pallet_multisig::Timepoint<
                            ::core::primitive::u32,
                        >,
                        call_hash: [::core::primitive::u8; 32usize],
                    },
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    MinimumThreshold,
                    #[codec(index = 1)]
                    AlreadyApproved,
                    #[codec(index = 2)]
                    NoApprovalsNeeded,
                    #[codec(index = 3)]
                    TooFewSignatories,
                    #[codec(index = 4)]
                    TooManySignatories,
                    #[codec(index = 5)]
                    SignatoriesOutOfOrder,
                    #[codec(index = 6)]
                    SenderInSignatories,
                    #[codec(index = 7)]
                    NotFound,
                    #[codec(index = 8)]
                    NotOwner,
                    #[codec(index = 9)]
                    NoTimepoint,
                    #[codec(index = 10)]
                    WrongTimepoint,
                    #[codec(index = 11)]
                    UnexpectedTimepoint,
                    #[codec(index = 12)]
                    MaxWeightTooLow,
                    #[codec(index = 13)]
                    AlreadyStored,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Event {
                    #[codec(index = 0)]
                    NewMultisig(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                        [::core::primitive::u8; 32usize],
                    ),
                    #[codec(index = 1)]
                    MultisigApproval(
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::pallet_multisig::Timepoint<
                            ::core::primitive::u32,
                        >,
                        ::subxt::sp_core::crypto::AccountId32,
                        [::core::primitive::u8; 32usize],
                    ),
                    #[codec(index = 2)]
                    MultisigExecuted(
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::pallet_multisig::Timepoint<
                            ::core::primitive::u32,
                        >,
                        ::subxt::sp_core::crypto::AccountId32,
                        [::core::primitive::u8; 32usize],
                        ::core::result::Result<
                            (),
                            runtime_types::sp_runtime::DispatchError,
                        >,
                    ),
                    #[codec(index = 3)]
                    MultisigCancelled(
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::pallet_multisig::Timepoint<
                            ::core::primitive::u32,
                        >,
                        ::subxt::sp_core::crypto::AccountId32,
                        [::core::primitive::u8; 32usize],
                    ),
                }
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Multisig<_0, _1, _2> {
                pub when: runtime_types::pallet_multisig::Timepoint<_0>,
                pub deposit: _1,
                pub depositor: _2,
                pub approvals: ::std::vec::Vec<_2>,
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Timepoint<_0> {
                pub height: _0,
                pub index: _0,
            }
        }
        pub mod pallet_offences {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Event {
                    #[codec(index = 0)]
                    Offence(
                        [::core::primitive::u8; 16usize],
                        ::std::vec::Vec<::core::primitive::u8>,
                    ),
                }
            }
        }
        pub mod pallet_proxy {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Call {
                    # [codec (index = 0)] proxy { real : :: subxt :: sp_core :: crypto :: AccountId32 , force_proxy_type : :: core :: option :: Option < runtime_types :: darkwebb_standalone_runtime :: ProxyType > , call : :: std :: boxed :: Box < runtime_types :: darkwebb_standalone_runtime :: Call > , } , # [codec (index = 1)] add_proxy { delegate : :: subxt :: sp_core :: crypto :: AccountId32 , proxy_type : runtime_types :: darkwebb_standalone_runtime :: ProxyType , delay : :: core :: primitive :: u32 , } , # [codec (index = 2)] remove_proxy { delegate : :: subxt :: sp_core :: crypto :: AccountId32 , proxy_type : runtime_types :: darkwebb_standalone_runtime :: ProxyType , delay : :: core :: primitive :: u32 , } , # [codec (index = 3)] remove_proxies , # [codec (index = 4)] anonymous { proxy_type : runtime_types :: darkwebb_standalone_runtime :: ProxyType , delay : :: core :: primitive :: u32 , index : :: core :: primitive :: u16 , } , # [codec (index = 5)] kill_anonymous { spawner : :: subxt :: sp_core :: crypto :: AccountId32 , proxy_type : runtime_types :: darkwebb_standalone_runtime :: ProxyType , index : :: core :: primitive :: u16 , # [codec (compact)] height : :: core :: primitive :: u32 , # [codec (compact)] ext_index : :: core :: primitive :: u32 , } , # [codec (index = 6)] announce { real : :: subxt :: sp_core :: crypto :: AccountId32 , call_hash : :: subxt :: sp_core :: H256 , } , # [codec (index = 7)] remove_announcement { real : :: subxt :: sp_core :: crypto :: AccountId32 , call_hash : :: subxt :: sp_core :: H256 , } , # [codec (index = 8)] reject_announcement { delegate : :: subxt :: sp_core :: crypto :: AccountId32 , call_hash : :: subxt :: sp_core :: H256 , } , # [codec (index = 9)] proxy_announced { delegate : :: subxt :: sp_core :: crypto :: AccountId32 , real : :: subxt :: sp_core :: crypto :: AccountId32 , force_proxy_type : :: core :: option :: Option < runtime_types :: darkwebb_standalone_runtime :: ProxyType > , call : :: std :: boxed :: Box < runtime_types :: darkwebb_standalone_runtime :: Call > , } , }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    TooMany,
                    #[codec(index = 1)]
                    NotFound,
                    #[codec(index = 2)]
                    NotProxy,
                    #[codec(index = 3)]
                    Unproxyable,
                    #[codec(index = 4)]
                    Duplicate,
                    #[codec(index = 5)]
                    NoPermission,
                    #[codec(index = 6)]
                    Unannounced,
                    #[codec(index = 7)]
                    NoSelfProxy,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Event {
                    #[codec(index = 0)]
                    ProxyExecuted(
                        ::core::result::Result<
                            (),
                            runtime_types::sp_runtime::DispatchError,
                        >,
                    ),
                    #[codec(index = 1)]
                    AnonymousCreated(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::darkwebb_standalone_runtime::ProxyType,
                        ::core::primitive::u16,
                    ),
                    #[codec(index = 2)]
                    Announced(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::H256,
                    ),
                    #[codec(index = 3)]
                    ProxyAdded(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::darkwebb_standalone_runtime::ProxyType,
                        ::core::primitive::u32,
                    ),
                }
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Announcement<_0, _1, _2> {
                pub real: _0,
                pub call_hash: _1,
                pub height: _2,
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ProxyDefinition<_0, _1, _2> {
                pub delegate: _0,
                pub proxy_type: _1,
                pub delay: _2,
            }
        }
        pub mod pallet_scheduler {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Call {
                    #[codec(index = 0)]
                    schedule {
                        when: ::core::primitive::u32,
                        maybe_periodic: ::core::option::Option<(
                            ::core::primitive::u32,
                            ::core::primitive::u32,
                        )>,
                        priority: ::core::primitive::u8,
                        call: ::std::boxed::Box<
                            runtime_types::darkwebb_standalone_runtime::Call,
                        >,
                    },
                    #[codec(index = 1)]
                    cancel {
                        when: ::core::primitive::u32,
                        index: ::core::primitive::u32,
                    },
                    #[codec(index = 2)]
                    schedule_named {
                        id: ::std::vec::Vec<::core::primitive::u8>,
                        when: ::core::primitive::u32,
                        maybe_periodic: ::core::option::Option<(
                            ::core::primitive::u32,
                            ::core::primitive::u32,
                        )>,
                        priority: ::core::primitive::u8,
                        call: ::std::boxed::Box<
                            runtime_types::darkwebb_standalone_runtime::Call,
                        >,
                    },
                    #[codec(index = 3)]
                    cancel_named {
                        id: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 4)]
                    schedule_after {
                        after: ::core::primitive::u32,
                        maybe_periodic: ::core::option::Option<(
                            ::core::primitive::u32,
                            ::core::primitive::u32,
                        )>,
                        priority: ::core::primitive::u8,
                        call: ::std::boxed::Box<
                            runtime_types::darkwebb_standalone_runtime::Call,
                        >,
                    },
                    #[codec(index = 5)]
                    schedule_named_after {
                        id: ::std::vec::Vec<::core::primitive::u8>,
                        after: ::core::primitive::u32,
                        maybe_periodic: ::core::option::Option<(
                            ::core::primitive::u32,
                            ::core::primitive::u32,
                        )>,
                        priority: ::core::primitive::u8,
                        call: ::std::boxed::Box<
                            runtime_types::darkwebb_standalone_runtime::Call,
                        >,
                    },
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    FailedToSchedule,
                    #[codec(index = 1)]
                    NotFound,
                    #[codec(index = 2)]
                    TargetBlockNumberInPast,
                    #[codec(index = 3)]
                    RescheduleNoChange,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Event {
                    #[codec(index = 0)]
                    Scheduled(::core::primitive::u32, ::core::primitive::u32),
                    #[codec(index = 1)]
                    Canceled(::core::primitive::u32, ::core::primitive::u32),
                    #[codec(index = 2)]
                    Dispatched(
                        (::core::primitive::u32, ::core::primitive::u32),
                        ::core::option::Option<
                            ::std::vec::Vec<::core::primitive::u8>,
                        >,
                        ::core::result::Result<
                            (),
                            runtime_types::sp_runtime::DispatchError,
                        >,
                    ),
                }
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub enum Releases {
                #[codec(index = 0)]
                V1,
                #[codec(index = 1)]
                V2,
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ScheduledV2<_0, _1, _2, _3> {
                pub maybe_id: ::core::option::Option<
                    ::std::vec::Vec<::core::primitive::u8>,
                >,
                pub priority: ::core::primitive::u8,
                pub call: _0,
                pub maybe_periodic: ::core::option::Option<(_1, _1)>,
                pub origin: _2,
                #[codec(skip)]
                pub __subxt_unused_type_params: ::core::marker::PhantomData<_3>,
            }
        }
        pub mod pallet_session {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Call {
                    # [codec (index = 0)] set_keys { keys : runtime_types :: darkwebb_standalone_runtime :: SessionKeys , proof : :: std :: vec :: Vec < :: core :: primitive :: u8 > , } , # [codec (index = 1)] purge_keys , }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    InvalidProof,
                    #[codec(index = 1)]
                    NoAssociatedValidatorId,
                    #[codec(index = 2)]
                    DuplicatedKey,
                    #[codec(index = 3)]
                    NoKeys,
                    #[codec(index = 4)]
                    NoAccount,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Event {
                    #[codec(index = 0)]
                    NewSession(::core::primitive::u32),
                }
            }
        }
        pub mod pallet_staking {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                pub mod pallet {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                    )]
                    pub enum Call {
                        # [codec (index = 0)] bond { controller : :: subxt :: sp_runtime :: MultiAddress < :: subxt :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u32 > , # [codec (compact)] value : :: core :: primitive :: u128 , payee : runtime_types :: pallet_staking :: RewardDestination < :: subxt :: sp_core :: crypto :: AccountId32 > , } , # [codec (index = 1)] bond_extra { # [codec (compact)] max_additional : :: core :: primitive :: u128 , } , # [codec (index = 2)] unbond { # [codec (compact)] value : :: core :: primitive :: u128 , } , # [codec (index = 3)] withdraw_unbonded { num_slashing_spans : :: core :: primitive :: u32 , } , # [codec (index = 4)] validate { prefs : runtime_types :: pallet_staking :: ValidatorPrefs , } , # [codec (index = 5)] nominate { targets : :: std :: vec :: Vec < :: subxt :: sp_runtime :: MultiAddress < :: subxt :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u32 > > , } , # [codec (index = 6)] chill , # [codec (index = 7)] set_payee { payee : runtime_types :: pallet_staking :: RewardDestination < :: subxt :: sp_core :: crypto :: AccountId32 > , } , # [codec (index = 8)] set_controller { controller : :: subxt :: sp_runtime :: MultiAddress < :: subxt :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u32 > , } , # [codec (index = 9)] set_validator_count { # [codec (compact)] new : :: core :: primitive :: u32 , } , # [codec (index = 10)] increase_validator_count { # [codec (compact)] additional : :: core :: primitive :: u32 , } , # [codec (index = 11)] scale_validator_count { factor : runtime_types :: sp_arithmetic :: per_things :: Percent , } , # [codec (index = 12)] force_no_eras , # [codec (index = 13)] force_new_era , # [codec (index = 14)] set_invulnerables { invulnerables : :: std :: vec :: Vec < :: subxt :: sp_core :: crypto :: AccountId32 > , } , # [codec (index = 15)] force_unstake { stash : :: subxt :: sp_core :: crypto :: AccountId32 , num_slashing_spans : :: core :: primitive :: u32 , } , # [codec (index = 16)] force_new_era_always , # [codec (index = 17)] cancel_deferred_slash { era : :: core :: primitive :: u32 , slash_indices : :: std :: vec :: Vec < :: core :: primitive :: u32 > , } , # [codec (index = 18)] payout_stakers { validator_stash : :: subxt :: sp_core :: crypto :: AccountId32 , era : :: core :: primitive :: u32 , } , # [codec (index = 19)] rebond { # [codec (compact)] value : :: core :: primitive :: u128 , } , # [codec (index = 20)] set_history_depth { # [codec (compact)] new_history_depth : :: core :: primitive :: u32 , # [codec (compact)] era_items_deleted : :: core :: primitive :: u32 , } , # [codec (index = 21)] reap_stash { stash : :: subxt :: sp_core :: crypto :: AccountId32 , num_slashing_spans : :: core :: primitive :: u32 , } , # [codec (index = 22)] kick { who : :: std :: vec :: Vec < :: subxt :: sp_runtime :: MultiAddress < :: subxt :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u32 > > , } , # [codec (index = 23)] set_staking_limits { min_nominator_bond : :: core :: primitive :: u128 , min_validator_bond : :: core :: primitive :: u128 , max_nominator_count : :: core :: option :: Option < :: core :: primitive :: u32 > , max_validator_count : :: core :: option :: Option < :: core :: primitive :: u32 > , threshold : :: core :: option :: Option < runtime_types :: sp_arithmetic :: per_things :: Percent > , } , # [codec (index = 24)] chill_other { controller : :: subxt :: sp_core :: crypto :: AccountId32 , } , }
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                    )]
                    pub enum Error {
                        #[codec(index = 0)]
                        NotController,
                        #[codec(index = 1)]
                        NotStash,
                        #[codec(index = 2)]
                        AlreadyBonded,
                        #[codec(index = 3)]
                        AlreadyPaired,
                        #[codec(index = 4)]
                        EmptyTargets,
                        #[codec(index = 5)]
                        DuplicateIndex,
                        #[codec(index = 6)]
                        InvalidSlashIndex,
                        #[codec(index = 7)]
                        InsufficientBond,
                        #[codec(index = 8)]
                        NoMoreChunks,
                        #[codec(index = 9)]
                        NoUnlockChunk,
                        #[codec(index = 10)]
                        FundedTarget,
                        #[codec(index = 11)]
                        InvalidEraToReward,
                        #[codec(index = 12)]
                        InvalidNumberOfNominations,
                        #[codec(index = 13)]
                        NotSortedAndUnique,
                        #[codec(index = 14)]
                        AlreadyClaimed,
                        #[codec(index = 15)]
                        IncorrectHistoryDepth,
                        #[codec(index = 16)]
                        IncorrectSlashingSpans,
                        #[codec(index = 17)]
                        BadState,
                        #[codec(index = 18)]
                        TooManyTargets,
                        #[codec(index = 19)]
                        BadTarget,
                        #[codec(index = 20)]
                        CannotChillOther,
                        #[codec(index = 21)]
                        TooManyNominators,
                        #[codec(index = 22)]
                        TooManyValidators,
                    }
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                    )]
                    pub enum Event {
                        #[codec(index = 0)]
                        EraPaid(
                            ::core::primitive::u32,
                            ::core::primitive::u128,
                            ::core::primitive::u128,
                        ),
                        #[codec(index = 1)]
                        Rewarded(
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        ),
                        #[codec(index = 2)]
                        Slashed(
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        ),
                        #[codec(index = 3)]
                        OldSlashingReportDiscarded(::core::primitive::u32),
                        #[codec(index = 4)]
                        StakersElected,
                        #[codec(index = 5)]
                        Bonded(
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        ),
                        #[codec(index = 6)]
                        Unbonded(
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        ),
                        #[codec(index = 7)]
                        Withdrawn(
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        ),
                        #[codec(index = 8)]
                        Kicked(
                            ::subxt::sp_core::crypto::AccountId32,
                            ::subxt::sp_core::crypto::AccountId32,
                        ),
                        #[codec(index = 9)]
                        StakingElectionFailed,
                        #[codec(index = 10)]
                        Chilled(::subxt::sp_core::crypto::AccountId32),
                        #[codec(index = 11)]
                        PayoutStarted(
                            ::core::primitive::u32,
                            ::subxt::sp_core::crypto::AccountId32,
                        ),
                    }
                }
            }
            pub mod slashing {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub struct SlashingSpans {
                    pub span_index: ::core::primitive::u32,
                    pub last_start: ::core::primitive::u32,
                    pub last_nonzero_slash: ::core::primitive::u32,
                    pub prior: ::std::vec::Vec<::core::primitive::u32>,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub struct SpanRecord<_0> {
                    pub slashed: _0,
                    pub paid_out: _0,
                }
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ActiveEraInfo {
                pub index: ::core::primitive::u32,
                pub start: ::core::option::Option<::core::primitive::u64>,
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct EraRewardPoints<_0> {
                pub total: ::core::primitive::u32,
                pub individual:
                    ::std::collections::BTreeMap<_0, ::core::primitive::u32>,
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Exposure<_0, _1> {
                #[codec(compact)]
                pub total: _1,
                #[codec(compact)]
                pub own: _1,
                pub others: ::std::vec::Vec<
                    runtime_types::pallet_staking::IndividualExposure<_0, _1>,
                >,
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub enum Forcing {
                #[codec(index = 0)]
                NotForcing,
                #[codec(index = 1)]
                ForceNew,
                #[codec(index = 2)]
                ForceNone,
                #[codec(index = 3)]
                ForceAlways,
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct IndividualExposure<_0, _1> {
                pub who: _0,
                #[codec(compact)]
                pub value: _1,
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Nominations<_0> {
                pub targets: ::std::vec::Vec<_0>,
                pub submitted_in: ::core::primitive::u32,
                pub suppressed: ::core::primitive::bool,
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub enum Releases {
                #[codec(index = 0)]
                V1_0_0Ancient,
                #[codec(index = 1)]
                V2_0_0,
                #[codec(index = 2)]
                V3_0_0,
                #[codec(index = 3)]
                V4_0_0,
                #[codec(index = 4)]
                V5_0_0,
                #[codec(index = 5)]
                V6_0_0,
                #[codec(index = 6)]
                V7_0_0,
                #[codec(index = 7)]
                V8_0_0,
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub enum RewardDestination<_0> {
                #[codec(index = 0)]
                Staked,
                #[codec(index = 1)]
                Stash,
                #[codec(index = 2)]
                Controller,
                #[codec(index = 3)]
                Account(_0),
                #[codec(index = 4)]
                None,
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct StakingLedger<_0, _1> {
                pub stash: _0,
                #[codec(compact)]
                pub total: _1,
                #[codec(compact)]
                pub active: _1,
                pub unlocking: ::std::vec::Vec<
                    runtime_types::pallet_staking::UnlockChunk<_1>,
                >,
                pub claimed_rewards: ::std::vec::Vec<::core::primitive::u32>,
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct UnappliedSlash<_0, _1> {
                pub validator: _0,
                pub own: _1,
                pub others: ::std::vec::Vec<(_0, _1)>,
                pub reporters: ::std::vec::Vec<_0>,
                pub payout: _1,
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct UnlockChunk<_0> {
                #[codec(compact)]
                pub value: _0,
                #[codec(compact)]
                pub era: ::core::primitive::u32,
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ValidatorPrefs {
                #[codec(compact)]
                pub commission:
                    runtime_types::sp_arithmetic::per_things::Perbill,
                pub blocked: ::core::primitive::bool,
            }
        }
        pub mod pallet_sudo {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Call {
                    #[codec(index = 0)]
                    sudo {
                        call: ::std::boxed::Box<
                            runtime_types::darkwebb_standalone_runtime::Call,
                        >,
                    },
                    #[codec(index = 1)]
                    sudo_unchecked_weight {
                        call: ::std::boxed::Box<
                            runtime_types::darkwebb_standalone_runtime::Call,
                        >,
                        weight: ::core::primitive::u64,
                    },
                    #[codec(index = 2)]
                    set_key {
                        new: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                    },
                    #[codec(index = 3)]
                    sudo_as {
                        who: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        call: ::std::boxed::Box<
                            runtime_types::darkwebb_standalone_runtime::Call,
                        >,
                    },
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    RequireSudo,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Event {
                    #[codec(index = 0)]
                    Sudid(
                        ::core::result::Result<
                            (),
                            runtime_types::sp_runtime::DispatchError,
                        >,
                    ),
                    #[codec(index = 1)]
                    KeyChanged(::subxt::sp_core::crypto::AccountId32),
                    #[codec(index = 2)]
                    SudoAsDone(
                        ::core::result::Result<
                            (),
                            runtime_types::sp_runtime::DispatchError,
                        >,
                    ),
                }
            }
        }
        pub mod pallet_timestamp {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Call {
                    #[codec(index = 0)]
                    set {
                        #[codec(compact)]
                        now: ::core::primitive::u64,
                    },
                }
            }
        }
        pub mod pallet_token_wrapper {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Call {
                    #[codec(index = 0)]
                    set_wrapping_fee { fee: ::core::primitive::u128 },
                    #[codec(index = 1)]
                    wrap {
                        from_asset_id: ::core::primitive::u32,
                        into_pool_share_id: ::core::primitive::u32,
                        amount: ::core::primitive::u128,
                        recipient: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 2)]
                    unwrap {
                        from_pool_share_id: ::core::primitive::u32,
                        into_asset_id: ::core::primitive::u32,
                        amount: ::core::primitive::u128,
                        recipient: ::subxt::sp_core::crypto::AccountId32,
                    },
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    InvalidAmount,
                    #[codec(index = 1)]
                    UnregisteredAssetId,
                    #[codec(index = 2)]
                    NotFoundInPool,
                    #[codec(index = 3)]
                    InsufficientBalance,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Event {
                    #[codec(index = 0)]
                    WrappedToken {
                        pool_share_asset: ::core::primitive::u32,
                        asset_id: ::core::primitive::u32,
                        amount: ::core::primitive::u128,
                        recipient: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 1)]
                    UnwrappedToken {
                        pool_share_asset: ::core::primitive::u32,
                        asset_id: ::core::primitive::u32,
                        amount: ::core::primitive::u128,
                        recipient: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 2)]
                    UpdatedWrappingFeePercent {
                        wrapping_fee_percent: ::core::primitive::u128,
                    },
                }
            }
        }
        pub mod pallet_transaction_payment {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct ChargeTransactionPayment(
                #[codec(compact)] pub ::core::primitive::u128,
            );
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub enum Releases {
                #[codec(index = 0)]
                V1Ancient,
                #[codec(index = 1)]
                V2,
            }
        }
        pub mod pallet_treasury {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Call {
                    #[codec(index = 0)]
                    propose_spend {
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                        beneficiary: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                    },
                    #[codec(index = 1)]
                    reject_proposal {
                        #[codec(compact)]
                        proposal_id: ::core::primitive::u32,
                    },
                    #[codec(index = 2)]
                    approve_proposal {
                        #[codec(compact)]
                        proposal_id: ::core::primitive::u32,
                    },
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    InsufficientProposersBalance,
                    #[codec(index = 1)]
                    InvalidIndex,
                    #[codec(index = 2)]
                    TooManyApprovals,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Event {
                    #[codec(index = 0)]
                    Proposed(::core::primitive::u32),
                    #[codec(index = 1)]
                    Spending(::core::primitive::u128),
                    #[codec(index = 2)]
                    Awarded(
                        ::core::primitive::u32,
                        ::core::primitive::u128,
                        ::subxt::sp_core::crypto::AccountId32,
                    ),
                    #[codec(index = 3)]
                    Rejected(::core::primitive::u32, ::core::primitive::u128),
                    #[codec(index = 4)]
                    Burnt(::core::primitive::u128),
                    #[codec(index = 5)]
                    Rollover(::core::primitive::u128),
                    #[codec(index = 6)]
                    Deposit(::core::primitive::u128),
                }
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Proposal<_0, _1> {
                pub proposer: _0,
                pub value: _1,
                pub beneficiary: _0,
                pub bond: _1,
            }
        }
        pub mod pallet_utility {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Call {
                    # [codec (index = 0)] batch { calls : :: std :: vec :: Vec < runtime_types :: darkwebb_standalone_runtime :: Call > , } , # [codec (index = 1)] as_derivative { index : :: core :: primitive :: u16 , call : :: std :: boxed :: Box < runtime_types :: darkwebb_standalone_runtime :: Call > , } , # [codec (index = 2)] batch_all { calls : :: std :: vec :: Vec < runtime_types :: darkwebb_standalone_runtime :: Call > , } , # [codec (index = 3)] dispatch_as { as_origin : :: std :: boxed :: Box < runtime_types :: darkwebb_standalone_runtime :: OriginCaller > , call : :: std :: boxed :: Box < runtime_types :: darkwebb_standalone_runtime :: Call > , } , }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    TooManyCalls,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Event {
                    #[codec(index = 0)]
                    BatchInterrupted(
                        ::core::primitive::u32,
                        runtime_types::sp_runtime::DispatchError,
                    ),
                    #[codec(index = 1)]
                    BatchCompleted,
                    #[codec(index = 2)]
                    ItemCompleted,
                    #[codec(index = 3)]
                    DispatchedAs(
                        ::core::result::Result<
                            (),
                            runtime_types::sp_runtime::DispatchError,
                        >,
                    ),
                }
            }
        }
        pub mod pallet_verifier {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Call {
                    #[codec(index = 0)]
                    set_parameters {
                        parameters: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 1)]
                    set_maintainer {
                        new_maintainer: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 2)]
                    force_set_parameters {
                        parameters: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 3)]
                    force_set_maintainer {
                        new_maintainer: ::subxt::sp_core::crypto::AccountId32,
                    },
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    ParametersNotInitialized,
                    #[codec(index = 1)]
                    InvalidParameters,
                    #[codec(index = 2)]
                    InvalidPermissions,
                    #[codec(index = 3)]
                    VerifyError,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum Event {
                    #[codec(index = 0)]
                    ParametersSet {
                        who: ::subxt::sp_core::crypto::AccountId32,
                        parameters: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 1)]
                    MaintainerSet {
                        old_maintainer: ::subxt::sp_core::crypto::AccountId32,
                        new_maintainer: ::subxt::sp_core::crypto::AccountId32,
                    },
                }
            }
        }
        pub mod primitive_types {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct H256(pub [::core::primitive::u8; 32usize]);
        }
        pub mod sp_arithmetic {
            use super::runtime_types;
            pub mod fixed_point {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: CompactAs,
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                )]
                pub struct FixedU128(pub ::core::primitive::u128);
            }
            pub mod per_things {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: CompactAs,
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                )]
                pub struct PerU16(pub ::core::primitive::u16);
                #[derive(
                    :: subxt :: codec :: CompactAs,
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                )]
                pub struct Perbill(pub ::core::primitive::u32);
                #[derive(
                    :: subxt :: codec :: CompactAs,
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                )]
                pub struct Percent(pub ::core::primitive::u8);
                #[derive(
                    :: subxt :: codec :: CompactAs,
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                )]
                pub struct Permill(pub ::core::primitive::u32);
            }
        }
        pub mod sp_authority_discovery {
            use super::runtime_types;
            pub mod app {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub struct Public(pub runtime_types::sp_core::sr25519::Public);
            }
        }
        pub mod sp_consensus_babe {
            use super::runtime_types;
            pub mod app {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub struct Public(pub runtime_types::sp_core::sr25519::Public);
            }
            pub mod digests {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum NextConfigDescriptor {
                    #[codec(index = 1)]
                    V1 {
                        c: (::core::primitive::u64, ::core::primitive::u64),
                        allowed_slots:
                            runtime_types::sp_consensus_babe::AllowedSlots,
                    },
                }
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub enum AllowedSlots {
                #[codec(index = 0)]
                PrimarySlots,
                #[codec(index = 1)]
                PrimaryAndSecondaryPlainSlots,
                #[codec(index = 2)]
                PrimaryAndSecondaryVRFSlots,
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct BabeEpochConfiguration {
                pub c: (::core::primitive::u64, ::core::primitive::u64),
                pub allowed_slots:
                    runtime_types::sp_consensus_babe::AllowedSlots,
            }
        }
        pub mod sp_consensus_slots {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct EquivocationProof<_0, _1> {
                pub offender: _1,
                pub slot: runtime_types::sp_consensus_slots::Slot,
                pub first_header: _0,
                pub second_header: _0,
            }
            #[derive(
                :: subxt :: codec :: CompactAs,
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
            )]
            pub struct Slot(pub ::core::primitive::u64);
        }
        pub mod sp_core {
            use super::runtime_types;
            pub mod changes_trie {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub struct ChangesTrieConfiguration {
                    pub digest_interval: ::core::primitive::u32,
                    pub digest_levels: ::core::primitive::u32,
                }
            }
            pub mod crypto {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub struct AccountId32(pub [::core::primitive::u8; 32usize]);
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub struct KeyTypeId(pub [::core::primitive::u8; 4usize]);
            }
            pub mod ecdsa {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub struct Signature(pub [::core::primitive::u8; 65usize]);
            }
            pub mod ed25519 {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub struct Public(pub [::core::primitive::u8; 32usize]);
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub struct Signature(pub [::core::primitive::u8; 64usize]);
            }
            pub mod offchain {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub struct OpaqueMultiaddr(
                    pub ::std::vec::Vec<::core::primitive::u8>,
                );
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub struct OpaqueNetworkState {
                    pub peer_id: runtime_types::sp_core::OpaquePeerId,
                    pub external_addresses: ::std::vec::Vec<
                        runtime_types::sp_core::offchain::OpaqueMultiaddr,
                    >,
                }
            }
            pub mod sr25519 {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub struct Public(pub [::core::primitive::u8; 32usize]);
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub struct Signature(pub [::core::primitive::u8; 64usize]);
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct OpaquePeerId(pub ::std::vec::Vec<::core::primitive::u8>);
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub enum Void {}
        }
        pub mod sp_finality_grandpa {
            use super::runtime_types;
            pub mod app {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub struct Public(pub runtime_types::sp_core::ed25519::Public);
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub struct Signature(
                    pub runtime_types::sp_core::ed25519::Signature,
                );
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub enum Equivocation<_0, _1> {
                #[codec(index = 0)]
                Prevote(
                    runtime_types::finality_grandpa::Equivocation<
                        runtime_types::sp_finality_grandpa::app::Public,
                        runtime_types::finality_grandpa::Prevote<_0, _1>,
                        runtime_types::sp_finality_grandpa::app::Signature,
                    >,
                ),
                #[codec(index = 1)]
                Precommit(
                    runtime_types::finality_grandpa::Equivocation<
                        runtime_types::sp_finality_grandpa::app::Public,
                        runtime_types::finality_grandpa::Precommit<_0, _1>,
                        runtime_types::sp_finality_grandpa::app::Signature,
                    >,
                ),
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct EquivocationProof<_0, _1> {
                pub set_id: ::core::primitive::u64,
                pub equivocation:
                    runtime_types::sp_finality_grandpa::Equivocation<_0, _1>,
            }
        }
        pub mod sp_npos_elections {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct Support<_0> {
                pub total: ::core::primitive::u128,
                pub voters: ::std::vec::Vec<(_0, ::core::primitive::u128)>,
            }
        }
        pub mod sp_runtime {
            use super::runtime_types;
            pub mod generic {
                use super::runtime_types;
                pub mod digest {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                    )]
                    pub enum ChangesTrieSignal {
                        # [codec (index = 0)] NewConfiguration (:: core :: option :: Option < runtime_types :: sp_core :: changes_trie :: ChangesTrieConfiguration > ,) , }
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                    )]
                    pub struct Digest < _0 > { pub logs : :: std :: vec :: Vec < runtime_types :: sp_runtime :: generic :: digest :: DigestItem < _0 > > , }
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                    )]
                    pub enum DigestItem<_0> {
                        # [codec (index = 2)] ChangesTrieRoot (_0 ,) , # [codec (index = 6)] PreRuntime ([:: core :: primitive :: u8 ; 4usize] , :: std :: vec :: Vec < :: core :: primitive :: u8 > ,) , # [codec (index = 4)] Consensus ([:: core :: primitive :: u8 ; 4usize] , :: std :: vec :: Vec < :: core :: primitive :: u8 > ,) , # [codec (index = 5)] Seal ([:: core :: primitive :: u8 ; 4usize] , :: std :: vec :: Vec < :: core :: primitive :: u8 > ,) , # [codec (index = 7)] ChangesTrieSignal (runtime_types :: sp_runtime :: generic :: digest :: ChangesTrieSignal ,) , # [codec (index = 0)] Other (:: std :: vec :: Vec < :: core :: primitive :: u8 > ,) , # [codec (index = 8)] RuntimeEnvironmentUpdated , }
                }
                pub mod era {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                    )]
                    pub enum Era {
                        #[codec(index = 0)]
                        Immortal,
                        #[codec(index = 1)]
                        Mortal1(::core::primitive::u8),
                        #[codec(index = 2)]
                        Mortal2(::core::primitive::u8),
                        #[codec(index = 3)]
                        Mortal3(::core::primitive::u8),
                        #[codec(index = 4)]
                        Mortal4(::core::primitive::u8),
                        #[codec(index = 5)]
                        Mortal5(::core::primitive::u8),
                        #[codec(index = 6)]
                        Mortal6(::core::primitive::u8),
                        #[codec(index = 7)]
                        Mortal7(::core::primitive::u8),
                        #[codec(index = 8)]
                        Mortal8(::core::primitive::u8),
                        #[codec(index = 9)]
                        Mortal9(::core::primitive::u8),
                        #[codec(index = 10)]
                        Mortal10(::core::primitive::u8),
                        #[codec(index = 11)]
                        Mortal11(::core::primitive::u8),
                        #[codec(index = 12)]
                        Mortal12(::core::primitive::u8),
                        #[codec(index = 13)]
                        Mortal13(::core::primitive::u8),
                        #[codec(index = 14)]
                        Mortal14(::core::primitive::u8),
                        #[codec(index = 15)]
                        Mortal15(::core::primitive::u8),
                        #[codec(index = 16)]
                        Mortal16(::core::primitive::u8),
                        #[codec(index = 17)]
                        Mortal17(::core::primitive::u8),
                        #[codec(index = 18)]
                        Mortal18(::core::primitive::u8),
                        #[codec(index = 19)]
                        Mortal19(::core::primitive::u8),
                        #[codec(index = 20)]
                        Mortal20(::core::primitive::u8),
                        #[codec(index = 21)]
                        Mortal21(::core::primitive::u8),
                        #[codec(index = 22)]
                        Mortal22(::core::primitive::u8),
                        #[codec(index = 23)]
                        Mortal23(::core::primitive::u8),
                        #[codec(index = 24)]
                        Mortal24(::core::primitive::u8),
                        #[codec(index = 25)]
                        Mortal25(::core::primitive::u8),
                        #[codec(index = 26)]
                        Mortal26(::core::primitive::u8),
                        #[codec(index = 27)]
                        Mortal27(::core::primitive::u8),
                        #[codec(index = 28)]
                        Mortal28(::core::primitive::u8),
                        #[codec(index = 29)]
                        Mortal29(::core::primitive::u8),
                        #[codec(index = 30)]
                        Mortal30(::core::primitive::u8),
                        #[codec(index = 31)]
                        Mortal31(::core::primitive::u8),
                        #[codec(index = 32)]
                        Mortal32(::core::primitive::u8),
                        #[codec(index = 33)]
                        Mortal33(::core::primitive::u8),
                        #[codec(index = 34)]
                        Mortal34(::core::primitive::u8),
                        #[codec(index = 35)]
                        Mortal35(::core::primitive::u8),
                        #[codec(index = 36)]
                        Mortal36(::core::primitive::u8),
                        #[codec(index = 37)]
                        Mortal37(::core::primitive::u8),
                        #[codec(index = 38)]
                        Mortal38(::core::primitive::u8),
                        #[codec(index = 39)]
                        Mortal39(::core::primitive::u8),
                        #[codec(index = 40)]
                        Mortal40(::core::primitive::u8),
                        #[codec(index = 41)]
                        Mortal41(::core::primitive::u8),
                        #[codec(index = 42)]
                        Mortal42(::core::primitive::u8),
                        #[codec(index = 43)]
                        Mortal43(::core::primitive::u8),
                        #[codec(index = 44)]
                        Mortal44(::core::primitive::u8),
                        #[codec(index = 45)]
                        Mortal45(::core::primitive::u8),
                        #[codec(index = 46)]
                        Mortal46(::core::primitive::u8),
                        #[codec(index = 47)]
                        Mortal47(::core::primitive::u8),
                        #[codec(index = 48)]
                        Mortal48(::core::primitive::u8),
                        #[codec(index = 49)]
                        Mortal49(::core::primitive::u8),
                        #[codec(index = 50)]
                        Mortal50(::core::primitive::u8),
                        #[codec(index = 51)]
                        Mortal51(::core::primitive::u8),
                        #[codec(index = 52)]
                        Mortal52(::core::primitive::u8),
                        #[codec(index = 53)]
                        Mortal53(::core::primitive::u8),
                        #[codec(index = 54)]
                        Mortal54(::core::primitive::u8),
                        #[codec(index = 55)]
                        Mortal55(::core::primitive::u8),
                        #[codec(index = 56)]
                        Mortal56(::core::primitive::u8),
                        #[codec(index = 57)]
                        Mortal57(::core::primitive::u8),
                        #[codec(index = 58)]
                        Mortal58(::core::primitive::u8),
                        #[codec(index = 59)]
                        Mortal59(::core::primitive::u8),
                        #[codec(index = 60)]
                        Mortal60(::core::primitive::u8),
                        #[codec(index = 61)]
                        Mortal61(::core::primitive::u8),
                        #[codec(index = 62)]
                        Mortal62(::core::primitive::u8),
                        #[codec(index = 63)]
                        Mortal63(::core::primitive::u8),
                        #[codec(index = 64)]
                        Mortal64(::core::primitive::u8),
                        #[codec(index = 65)]
                        Mortal65(::core::primitive::u8),
                        #[codec(index = 66)]
                        Mortal66(::core::primitive::u8),
                        #[codec(index = 67)]
                        Mortal67(::core::primitive::u8),
                        #[codec(index = 68)]
                        Mortal68(::core::primitive::u8),
                        #[codec(index = 69)]
                        Mortal69(::core::primitive::u8),
                        #[codec(index = 70)]
                        Mortal70(::core::primitive::u8),
                        #[codec(index = 71)]
                        Mortal71(::core::primitive::u8),
                        #[codec(index = 72)]
                        Mortal72(::core::primitive::u8),
                        #[codec(index = 73)]
                        Mortal73(::core::primitive::u8),
                        #[codec(index = 74)]
                        Mortal74(::core::primitive::u8),
                        #[codec(index = 75)]
                        Mortal75(::core::primitive::u8),
                        #[codec(index = 76)]
                        Mortal76(::core::primitive::u8),
                        #[codec(index = 77)]
                        Mortal77(::core::primitive::u8),
                        #[codec(index = 78)]
                        Mortal78(::core::primitive::u8),
                        #[codec(index = 79)]
                        Mortal79(::core::primitive::u8),
                        #[codec(index = 80)]
                        Mortal80(::core::primitive::u8),
                        #[codec(index = 81)]
                        Mortal81(::core::primitive::u8),
                        #[codec(index = 82)]
                        Mortal82(::core::primitive::u8),
                        #[codec(index = 83)]
                        Mortal83(::core::primitive::u8),
                        #[codec(index = 84)]
                        Mortal84(::core::primitive::u8),
                        #[codec(index = 85)]
                        Mortal85(::core::primitive::u8),
                        #[codec(index = 86)]
                        Mortal86(::core::primitive::u8),
                        #[codec(index = 87)]
                        Mortal87(::core::primitive::u8),
                        #[codec(index = 88)]
                        Mortal88(::core::primitive::u8),
                        #[codec(index = 89)]
                        Mortal89(::core::primitive::u8),
                        #[codec(index = 90)]
                        Mortal90(::core::primitive::u8),
                        #[codec(index = 91)]
                        Mortal91(::core::primitive::u8),
                        #[codec(index = 92)]
                        Mortal92(::core::primitive::u8),
                        #[codec(index = 93)]
                        Mortal93(::core::primitive::u8),
                        #[codec(index = 94)]
                        Mortal94(::core::primitive::u8),
                        #[codec(index = 95)]
                        Mortal95(::core::primitive::u8),
                        #[codec(index = 96)]
                        Mortal96(::core::primitive::u8),
                        #[codec(index = 97)]
                        Mortal97(::core::primitive::u8),
                        #[codec(index = 98)]
                        Mortal98(::core::primitive::u8),
                        #[codec(index = 99)]
                        Mortal99(::core::primitive::u8),
                        #[codec(index = 100)]
                        Mortal100(::core::primitive::u8),
                        #[codec(index = 101)]
                        Mortal101(::core::primitive::u8),
                        #[codec(index = 102)]
                        Mortal102(::core::primitive::u8),
                        #[codec(index = 103)]
                        Mortal103(::core::primitive::u8),
                        #[codec(index = 104)]
                        Mortal104(::core::primitive::u8),
                        #[codec(index = 105)]
                        Mortal105(::core::primitive::u8),
                        #[codec(index = 106)]
                        Mortal106(::core::primitive::u8),
                        #[codec(index = 107)]
                        Mortal107(::core::primitive::u8),
                        #[codec(index = 108)]
                        Mortal108(::core::primitive::u8),
                        #[codec(index = 109)]
                        Mortal109(::core::primitive::u8),
                        #[codec(index = 110)]
                        Mortal110(::core::primitive::u8),
                        #[codec(index = 111)]
                        Mortal111(::core::primitive::u8),
                        #[codec(index = 112)]
                        Mortal112(::core::primitive::u8),
                        #[codec(index = 113)]
                        Mortal113(::core::primitive::u8),
                        #[codec(index = 114)]
                        Mortal114(::core::primitive::u8),
                        #[codec(index = 115)]
                        Mortal115(::core::primitive::u8),
                        #[codec(index = 116)]
                        Mortal116(::core::primitive::u8),
                        #[codec(index = 117)]
                        Mortal117(::core::primitive::u8),
                        #[codec(index = 118)]
                        Mortal118(::core::primitive::u8),
                        #[codec(index = 119)]
                        Mortal119(::core::primitive::u8),
                        #[codec(index = 120)]
                        Mortal120(::core::primitive::u8),
                        #[codec(index = 121)]
                        Mortal121(::core::primitive::u8),
                        #[codec(index = 122)]
                        Mortal122(::core::primitive::u8),
                        #[codec(index = 123)]
                        Mortal123(::core::primitive::u8),
                        #[codec(index = 124)]
                        Mortal124(::core::primitive::u8),
                        #[codec(index = 125)]
                        Mortal125(::core::primitive::u8),
                        #[codec(index = 126)]
                        Mortal126(::core::primitive::u8),
                        #[codec(index = 127)]
                        Mortal127(::core::primitive::u8),
                        #[codec(index = 128)]
                        Mortal128(::core::primitive::u8),
                        #[codec(index = 129)]
                        Mortal129(::core::primitive::u8),
                        #[codec(index = 130)]
                        Mortal130(::core::primitive::u8),
                        #[codec(index = 131)]
                        Mortal131(::core::primitive::u8),
                        #[codec(index = 132)]
                        Mortal132(::core::primitive::u8),
                        #[codec(index = 133)]
                        Mortal133(::core::primitive::u8),
                        #[codec(index = 134)]
                        Mortal134(::core::primitive::u8),
                        #[codec(index = 135)]
                        Mortal135(::core::primitive::u8),
                        #[codec(index = 136)]
                        Mortal136(::core::primitive::u8),
                        #[codec(index = 137)]
                        Mortal137(::core::primitive::u8),
                        #[codec(index = 138)]
                        Mortal138(::core::primitive::u8),
                        #[codec(index = 139)]
                        Mortal139(::core::primitive::u8),
                        #[codec(index = 140)]
                        Mortal140(::core::primitive::u8),
                        #[codec(index = 141)]
                        Mortal141(::core::primitive::u8),
                        #[codec(index = 142)]
                        Mortal142(::core::primitive::u8),
                        #[codec(index = 143)]
                        Mortal143(::core::primitive::u8),
                        #[codec(index = 144)]
                        Mortal144(::core::primitive::u8),
                        #[codec(index = 145)]
                        Mortal145(::core::primitive::u8),
                        #[codec(index = 146)]
                        Mortal146(::core::primitive::u8),
                        #[codec(index = 147)]
                        Mortal147(::core::primitive::u8),
                        #[codec(index = 148)]
                        Mortal148(::core::primitive::u8),
                        #[codec(index = 149)]
                        Mortal149(::core::primitive::u8),
                        #[codec(index = 150)]
                        Mortal150(::core::primitive::u8),
                        #[codec(index = 151)]
                        Mortal151(::core::primitive::u8),
                        #[codec(index = 152)]
                        Mortal152(::core::primitive::u8),
                        #[codec(index = 153)]
                        Mortal153(::core::primitive::u8),
                        #[codec(index = 154)]
                        Mortal154(::core::primitive::u8),
                        #[codec(index = 155)]
                        Mortal155(::core::primitive::u8),
                        #[codec(index = 156)]
                        Mortal156(::core::primitive::u8),
                        #[codec(index = 157)]
                        Mortal157(::core::primitive::u8),
                        #[codec(index = 158)]
                        Mortal158(::core::primitive::u8),
                        #[codec(index = 159)]
                        Mortal159(::core::primitive::u8),
                        #[codec(index = 160)]
                        Mortal160(::core::primitive::u8),
                        #[codec(index = 161)]
                        Mortal161(::core::primitive::u8),
                        #[codec(index = 162)]
                        Mortal162(::core::primitive::u8),
                        #[codec(index = 163)]
                        Mortal163(::core::primitive::u8),
                        #[codec(index = 164)]
                        Mortal164(::core::primitive::u8),
                        #[codec(index = 165)]
                        Mortal165(::core::primitive::u8),
                        #[codec(index = 166)]
                        Mortal166(::core::primitive::u8),
                        #[codec(index = 167)]
                        Mortal167(::core::primitive::u8),
                        #[codec(index = 168)]
                        Mortal168(::core::primitive::u8),
                        #[codec(index = 169)]
                        Mortal169(::core::primitive::u8),
                        #[codec(index = 170)]
                        Mortal170(::core::primitive::u8),
                        #[codec(index = 171)]
                        Mortal171(::core::primitive::u8),
                        #[codec(index = 172)]
                        Mortal172(::core::primitive::u8),
                        #[codec(index = 173)]
                        Mortal173(::core::primitive::u8),
                        #[codec(index = 174)]
                        Mortal174(::core::primitive::u8),
                        #[codec(index = 175)]
                        Mortal175(::core::primitive::u8),
                        #[codec(index = 176)]
                        Mortal176(::core::primitive::u8),
                        #[codec(index = 177)]
                        Mortal177(::core::primitive::u8),
                        #[codec(index = 178)]
                        Mortal178(::core::primitive::u8),
                        #[codec(index = 179)]
                        Mortal179(::core::primitive::u8),
                        #[codec(index = 180)]
                        Mortal180(::core::primitive::u8),
                        #[codec(index = 181)]
                        Mortal181(::core::primitive::u8),
                        #[codec(index = 182)]
                        Mortal182(::core::primitive::u8),
                        #[codec(index = 183)]
                        Mortal183(::core::primitive::u8),
                        #[codec(index = 184)]
                        Mortal184(::core::primitive::u8),
                        #[codec(index = 185)]
                        Mortal185(::core::primitive::u8),
                        #[codec(index = 186)]
                        Mortal186(::core::primitive::u8),
                        #[codec(index = 187)]
                        Mortal187(::core::primitive::u8),
                        #[codec(index = 188)]
                        Mortal188(::core::primitive::u8),
                        #[codec(index = 189)]
                        Mortal189(::core::primitive::u8),
                        #[codec(index = 190)]
                        Mortal190(::core::primitive::u8),
                        #[codec(index = 191)]
                        Mortal191(::core::primitive::u8),
                        #[codec(index = 192)]
                        Mortal192(::core::primitive::u8),
                        #[codec(index = 193)]
                        Mortal193(::core::primitive::u8),
                        #[codec(index = 194)]
                        Mortal194(::core::primitive::u8),
                        #[codec(index = 195)]
                        Mortal195(::core::primitive::u8),
                        #[codec(index = 196)]
                        Mortal196(::core::primitive::u8),
                        #[codec(index = 197)]
                        Mortal197(::core::primitive::u8),
                        #[codec(index = 198)]
                        Mortal198(::core::primitive::u8),
                        #[codec(index = 199)]
                        Mortal199(::core::primitive::u8),
                        #[codec(index = 200)]
                        Mortal200(::core::primitive::u8),
                        #[codec(index = 201)]
                        Mortal201(::core::primitive::u8),
                        #[codec(index = 202)]
                        Mortal202(::core::primitive::u8),
                        #[codec(index = 203)]
                        Mortal203(::core::primitive::u8),
                        #[codec(index = 204)]
                        Mortal204(::core::primitive::u8),
                        #[codec(index = 205)]
                        Mortal205(::core::primitive::u8),
                        #[codec(index = 206)]
                        Mortal206(::core::primitive::u8),
                        #[codec(index = 207)]
                        Mortal207(::core::primitive::u8),
                        #[codec(index = 208)]
                        Mortal208(::core::primitive::u8),
                        #[codec(index = 209)]
                        Mortal209(::core::primitive::u8),
                        #[codec(index = 210)]
                        Mortal210(::core::primitive::u8),
                        #[codec(index = 211)]
                        Mortal211(::core::primitive::u8),
                        #[codec(index = 212)]
                        Mortal212(::core::primitive::u8),
                        #[codec(index = 213)]
                        Mortal213(::core::primitive::u8),
                        #[codec(index = 214)]
                        Mortal214(::core::primitive::u8),
                        #[codec(index = 215)]
                        Mortal215(::core::primitive::u8),
                        #[codec(index = 216)]
                        Mortal216(::core::primitive::u8),
                        #[codec(index = 217)]
                        Mortal217(::core::primitive::u8),
                        #[codec(index = 218)]
                        Mortal218(::core::primitive::u8),
                        #[codec(index = 219)]
                        Mortal219(::core::primitive::u8),
                        #[codec(index = 220)]
                        Mortal220(::core::primitive::u8),
                        #[codec(index = 221)]
                        Mortal221(::core::primitive::u8),
                        #[codec(index = 222)]
                        Mortal222(::core::primitive::u8),
                        #[codec(index = 223)]
                        Mortal223(::core::primitive::u8),
                        #[codec(index = 224)]
                        Mortal224(::core::primitive::u8),
                        #[codec(index = 225)]
                        Mortal225(::core::primitive::u8),
                        #[codec(index = 226)]
                        Mortal226(::core::primitive::u8),
                        #[codec(index = 227)]
                        Mortal227(::core::primitive::u8),
                        #[codec(index = 228)]
                        Mortal228(::core::primitive::u8),
                        #[codec(index = 229)]
                        Mortal229(::core::primitive::u8),
                        #[codec(index = 230)]
                        Mortal230(::core::primitive::u8),
                        #[codec(index = 231)]
                        Mortal231(::core::primitive::u8),
                        #[codec(index = 232)]
                        Mortal232(::core::primitive::u8),
                        #[codec(index = 233)]
                        Mortal233(::core::primitive::u8),
                        #[codec(index = 234)]
                        Mortal234(::core::primitive::u8),
                        #[codec(index = 235)]
                        Mortal235(::core::primitive::u8),
                        #[codec(index = 236)]
                        Mortal236(::core::primitive::u8),
                        #[codec(index = 237)]
                        Mortal237(::core::primitive::u8),
                        #[codec(index = 238)]
                        Mortal238(::core::primitive::u8),
                        #[codec(index = 239)]
                        Mortal239(::core::primitive::u8),
                        #[codec(index = 240)]
                        Mortal240(::core::primitive::u8),
                        #[codec(index = 241)]
                        Mortal241(::core::primitive::u8),
                        #[codec(index = 242)]
                        Mortal242(::core::primitive::u8),
                        #[codec(index = 243)]
                        Mortal243(::core::primitive::u8),
                        #[codec(index = 244)]
                        Mortal244(::core::primitive::u8),
                        #[codec(index = 245)]
                        Mortal245(::core::primitive::u8),
                        #[codec(index = 246)]
                        Mortal246(::core::primitive::u8),
                        #[codec(index = 247)]
                        Mortal247(::core::primitive::u8),
                        #[codec(index = 248)]
                        Mortal248(::core::primitive::u8),
                        #[codec(index = 249)]
                        Mortal249(::core::primitive::u8),
                        #[codec(index = 250)]
                        Mortal250(::core::primitive::u8),
                        #[codec(index = 251)]
                        Mortal251(::core::primitive::u8),
                        #[codec(index = 252)]
                        Mortal252(::core::primitive::u8),
                        #[codec(index = 253)]
                        Mortal253(::core::primitive::u8),
                        #[codec(index = 254)]
                        Mortal254(::core::primitive::u8),
                        #[codec(index = 255)]
                        Mortal255(::core::primitive::u8),
                    }
                }
                pub mod header {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                    )]
                    pub struct Header<_0, _1> {
                        pub parent_hash: ::subxt::sp_core::H256,
                        #[codec(compact)]
                        pub number: _0,
                        pub state_root: ::subxt::sp_core::H256,
                        pub extrinsics_root: ::subxt::sp_core::H256,
                        pub digest:
                            runtime_types::sp_runtime::generic::digest::Digest<
                                ::subxt::sp_core::H256,
                            >,
                        #[codec(skip)]
                        pub __subxt_unused_type_params:
                            ::core::marker::PhantomData<_1>,
                    }
                }
                pub mod unchecked_extrinsic {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                    )]
                    pub struct UncheckedExtrinsic<_0, _1, _2, _3>(
                        ::std::vec::Vec<::core::primitive::u8>,
                        #[codec(skip)]
                        pub  ::core::marker::PhantomData<(_0, _1, _2, _3)>,
                    );
                }
            }
            pub mod multiaddress {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub enum MultiAddress<_0, _1> {
                    #[codec(index = 0)]
                    Id(_0),
                    #[codec(index = 1)]
                    Index(#[codec(compact)] _1),
                    #[codec(index = 2)]
                    Raw(::std::vec::Vec<::core::primitive::u8>),
                    #[codec(index = 3)]
                    Address32([::core::primitive::u8; 32usize]),
                    #[codec(index = 4)]
                    Address20([::core::primitive::u8; 20usize]),
                }
            }
            pub mod traits {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub struct BlakeTwo256 {}
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub enum ArithmeticError {
                #[codec(index = 0)]
                Underflow,
                #[codec(index = 1)]
                Overflow,
                #[codec(index = 2)]
                DivisionByZero,
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub enum DispatchError {
                #[codec(index = 0)]
                Other,
                #[codec(index = 1)]
                CannotLookup,
                #[codec(index = 2)]
                BadOrigin,
                #[codec(index = 3)]
                Module {
                    index: ::core::primitive::u8,
                    error: ::core::primitive::u8,
                },
                #[codec(index = 4)]
                ConsumerRemaining,
                #[codec(index = 5)]
                NoProviders,
                #[codec(index = 6)]
                Token(runtime_types::sp_runtime::TokenError),
                #[codec(index = 7)]
                Arithmetic(runtime_types::sp_runtime::ArithmeticError),
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub enum MultiSignature {
                #[codec(index = 0)]
                Ed25519(runtime_types::sp_core::ed25519::Signature),
                #[codec(index = 1)]
                Sr25519(runtime_types::sp_core::sr25519::Signature),
                #[codec(index = 2)]
                Ecdsa(runtime_types::sp_core::ecdsa::Signature),
            }
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub enum TokenError {
                #[codec(index = 0)]
                NoFunds,
                #[codec(index = 1)]
                WouldDie,
                #[codec(index = 2)]
                BelowMinimum,
                #[codec(index = 3)]
                CannotCreate,
                #[codec(index = 4)]
                UnknownAsset,
                #[codec(index = 5)]
                Frozen,
                #[codec(index = 6)]
                Unsupported,
            }
        }
        pub mod sp_session {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct MembershipProof {
                pub session: ::core::primitive::u32,
                pub trie_nodes:
                    ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                pub validator_count: ::core::primitive::u32,
            }
        }
        pub mod sp_staking {
            use super::runtime_types;
            pub mod offence {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                )]
                pub struct OffenceDetails<_0, _1> {
                    pub offender: _1,
                    pub reporters: ::std::vec::Vec<_0>,
                }
            }
        }
        pub mod sp_version {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
            )]
            pub struct RuntimeVersion {
                pub spec_name: ::std::string::String,
                pub impl_name: ::std::string::String,
                pub authoring_version: ::core::primitive::u32,
                pub spec_version: ::core::primitive::u32,
                pub impl_version: ::core::primitive::u32,
                pub apis: ::std::vec::Vec<(
                    [::core::primitive::u8; 8usize],
                    ::core::primitive::u32,
                )>,
                pub transaction_version: ::core::primitive::u32,
            }
        }
    }
    #[doc = r" Default configuration of common types for a target Substrate runtime."]
    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    pub struct DefaultConfig;
    impl ::subxt::Config for DefaultConfig {
        type Index = u32;
        type BlockNumber = u32;
        type Hash = ::subxt::sp_core::H256;
        type Hashing = ::subxt::sp_runtime::traits::BlakeTwo256;
        type AccountId = ::subxt::sp_runtime::AccountId32;
        type Address = ::subxt::sp_runtime::MultiAddress<Self::AccountId, u32>;
        type Header = ::subxt::sp_runtime::generic::Header<
            Self::BlockNumber,
            ::subxt::sp_runtime::traits::BlakeTwo256,
        >;
        type Signature = ::subxt::sp_runtime::MultiSignature;
        type Extrinsic = ::subxt::sp_runtime::OpaqueExtrinsic;
    }
    impl ::subxt::ExtrinsicExtraData<DefaultConfig> for DefaultConfig {
        type AccountData = AccountData;
        type Extra = ::subxt::DefaultExtra<DefaultConfig>;
    }
    pub type AccountData = self::system::storage::Account;
    impl ::subxt::AccountData<DefaultConfig> for AccountData {
        fn nonce(
            result: &<Self as ::subxt::StorageEntry>::Value,
        ) -> <DefaultConfig as ::subxt::Config>::Index {
            result.nonce
        }
        fn storage_entry(
            account_id: <DefaultConfig as ::subxt::Config>::AccountId,
        ) -> Self {
            Self(account_id)
        }
    }
    pub struct RuntimeApi<T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
        pub client: ::subxt::Client<T>,
    }
    impl<T> ::core::convert::From<::subxt::Client<T>> for RuntimeApi<T>
    where
        T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
    {
        fn from(client: ::subxt::Client<T>) -> Self {
            Self { client }
        }
    }
    impl<'a, T> RuntimeApi<T>
    where
        T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
    {
        pub fn storage(&'a self) -> StorageApi<'a, T> {
            StorageApi {
                client: &self.client,
            }
        }
        pub fn tx(&'a self) -> TransactionApi<'a, T> {
            TransactionApi {
                client: &self.client,
            }
        }
    }
    pub struct StorageApi<'a, T>
    where
        T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
    {
        client: &'a ::subxt::Client<T>,
    }
    impl<'a, T> StorageApi<'a, T>
    where
        T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
    {
        pub fn system(&self) -> system::storage::StorageApi<'a, T> {
            system::storage::StorageApi::new(self.client)
        }
        pub fn randomness_collective_flip(
            &self,
        ) -> randomness_collective_flip::storage::StorageApi<'a, T> {
            randomness_collective_flip::storage::StorageApi::new(self.client)
        }
        pub fn timestamp(&self) -> timestamp::storage::StorageApi<'a, T> {
            timestamp::storage::StorageApi::new(self.client)
        }
        pub fn babe(&self) -> babe::storage::StorageApi<'a, T> {
            babe::storage::StorageApi::new(self.client)
        }
        pub fn authorship(&self) -> authorship::storage::StorageApi<'a, T> {
            authorship::storage::StorageApi::new(self.client)
        }
        pub fn indices(&self) -> indices::storage::StorageApi<'a, T> {
            indices::storage::StorageApi::new(self.client)
        }
        pub fn balances(&self) -> balances::storage::StorageApi<'a, T> {
            balances::storage::StorageApi::new(self.client)
        }
        pub fn transaction_payment(
            &self,
        ) -> transaction_payment::storage::StorageApi<'a, T> {
            transaction_payment::storage::StorageApi::new(self.client)
        }
        pub fn election_provider_multi_phase(
            &self,
        ) -> election_provider_multi_phase::storage::StorageApi<'a, T> {
            election_provider_multi_phase::storage::StorageApi::new(self.client)
        }
        pub fn staking(&self) -> staking::storage::StorageApi<'a, T> {
            staking::storage::StorageApi::new(self.client)
        }
        pub fn session(&self) -> session::storage::StorageApi<'a, T> {
            session::storage::StorageApi::new(self.client)
        }
        pub fn democracy(&self) -> democracy::storage::StorageApi<'a, T> {
            democracy::storage::StorageApi::new(self.client)
        }
        pub fn council(&self) -> council::storage::StorageApi<'a, T> {
            council::storage::StorageApi::new(self.client)
        }
        pub fn elections(&self) -> elections::storage::StorageApi<'a, T> {
            elections::storage::StorageApi::new(self.client)
        }
        pub fn grandpa(&self) -> grandpa::storage::StorageApi<'a, T> {
            grandpa::storage::StorageApi::new(self.client)
        }
        pub fn treasury(&self) -> treasury::storage::StorageApi<'a, T> {
            treasury::storage::StorageApi::new(self.client)
        }
        pub fn multisig(&self) -> multisig::storage::StorageApi<'a, T> {
            multisig::storage::StorageApi::new(self.client)
        }
        pub fn scheduler(&self) -> scheduler::storage::StorageApi<'a, T> {
            scheduler::storage::StorageApi::new(self.client)
        }
        pub fn proxy(&self) -> proxy::storage::StorageApi<'a, T> {
            proxy::storage::StorageApi::new(self.client)
        }
        pub fn assets(&self) -> assets::storage::StorageApi<'a, T> {
            assets::storage::StorageApi::new(self.client)
        }
        pub fn sudo(&self) -> sudo::storage::StorageApi<'a, T> {
            sudo::storage::StorageApi::new(self.client)
        }
        pub fn im_online(&self) -> im_online::storage::StorageApi<'a, T> {
            im_online::storage::StorageApi::new(self.client)
        }
        pub fn offences(&self) -> offences::storage::StorageApi<'a, T> {
            offences::storage::StorageApi::new(self.client)
        }
        pub fn bounties(&self) -> bounties::storage::StorageApi<'a, T> {
            bounties::storage::StorageApi::new(self.client)
        }
        pub fn bags_list(&self) -> bags_list::storage::StorageApi<'a, T> {
            bags_list::storage::StorageApi::new(self.client)
        }
        pub fn hasher_bn254(&self) -> hasher_bn254::storage::StorageApi<'a, T> {
            hasher_bn254::storage::StorageApi::new(self.client)
        }
        pub fn hasher_bls381(
            &self,
        ) -> hasher_bls381::storage::StorageApi<'a, T> {
            hasher_bls381::storage::StorageApi::new(self.client)
        }
        pub fn asset_registry(
            &self,
        ) -> asset_registry::storage::StorageApi<'a, T> {
            asset_registry::storage::StorageApi::new(self.client)
        }
        pub fn tokens(&self) -> tokens::storage::StorageApi<'a, T> {
            tokens::storage::StorageApi::new(self.client)
        }
        pub fn token_wrapper(
            &self,
        ) -> token_wrapper::storage::StorageApi<'a, T> {
            token_wrapper::storage::StorageApi::new(self.client)
        }
        pub fn verifier_bn254(
            &self,
        ) -> verifier_bn254::storage::StorageApi<'a, T> {
            verifier_bn254::storage::StorageApi::new(self.client)
        }
        pub fn verifier_bls381(
            &self,
        ) -> verifier_bls381::storage::StorageApi<'a, T> {
            verifier_bls381::storage::StorageApi::new(self.client)
        }
        pub fn merkle_tree_bn254(
            &self,
        ) -> merkle_tree_bn254::storage::StorageApi<'a, T> {
            merkle_tree_bn254::storage::StorageApi::new(self.client)
        }
        pub fn merkle_tree_bls381(
            &self,
        ) -> merkle_tree_bls381::storage::StorageApi<'a, T> {
            merkle_tree_bls381::storage::StorageApi::new(self.client)
        }
        pub fn linkable_tree_bn254(
            &self,
        ) -> linkable_tree_bn254::storage::StorageApi<'a, T> {
            linkable_tree_bn254::storage::StorageApi::new(self.client)
        }
        pub fn linkable_tree_bls381(
            &self,
        ) -> linkable_tree_bls381::storage::StorageApi<'a, T> {
            linkable_tree_bls381::storage::StorageApi::new(self.client)
        }
        pub fn mixer_bn254(&self) -> mixer_bn254::storage::StorageApi<'a, T> {
            mixer_bn254::storage::StorageApi::new(self.client)
        }
        pub fn mixer_bls381(&self) -> mixer_bls381::storage::StorageApi<'a, T> {
            mixer_bls381::storage::StorageApi::new(self.client)
        }
        pub fn anchor_bn254(&self) -> anchor_bn254::storage::StorageApi<'a, T> {
            anchor_bn254::storage::StorageApi::new(self.client)
        }
        pub fn anchor_bls381(
            &self,
        ) -> anchor_bls381::storage::StorageApi<'a, T> {
            anchor_bls381::storage::StorageApi::new(self.client)
        }
        pub fn anchor_handler_bn254(
            &self,
        ) -> anchor_handler_bn254::storage::StorageApi<'a, T> {
            anchor_handler_bn254::storage::StorageApi::new(self.client)
        }
        pub fn anchor_handler_bls381(
            &self,
        ) -> anchor_handler_bls381::storage::StorageApi<'a, T> {
            anchor_handler_bls381::storage::StorageApi::new(self.client)
        }
        pub fn bridge(&self) -> bridge::storage::StorageApi<'a, T> {
            bridge::storage::StorageApi::new(self.client)
        }
    }
    pub struct TransactionApi<
        'a,
        T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
    > {
        client: &'a ::subxt::Client<T>,
    }
    impl<'a, T> TransactionApi<'a, T>
    where
        T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
    {
        pub fn system(&self) -> system::calls::TransactionApi<'a, T> {
            system::calls::TransactionApi::new(self.client)
        }
        pub fn timestamp(&self) -> timestamp::calls::TransactionApi<'a, T> {
            timestamp::calls::TransactionApi::new(self.client)
        }
        pub fn babe(&self) -> babe::calls::TransactionApi<'a, T> {
            babe::calls::TransactionApi::new(self.client)
        }
        pub fn authorship(&self) -> authorship::calls::TransactionApi<'a, T> {
            authorship::calls::TransactionApi::new(self.client)
        }
        pub fn indices(&self) -> indices::calls::TransactionApi<'a, T> {
            indices::calls::TransactionApi::new(self.client)
        }
        pub fn balances(&self) -> balances::calls::TransactionApi<'a, T> {
            balances::calls::TransactionApi::new(self.client)
        }
        pub fn election_provider_multi_phase(
            &self,
        ) -> election_provider_multi_phase::calls::TransactionApi<'a, T>
        {
            election_provider_multi_phase::calls::TransactionApi::new(
                self.client,
            )
        }
        pub fn staking(&self) -> staking::calls::TransactionApi<'a, T> {
            staking::calls::TransactionApi::new(self.client)
        }
        pub fn session(&self) -> session::calls::TransactionApi<'a, T> {
            session::calls::TransactionApi::new(self.client)
        }
        pub fn democracy(&self) -> democracy::calls::TransactionApi<'a, T> {
            democracy::calls::TransactionApi::new(self.client)
        }
        pub fn council(&self) -> council::calls::TransactionApi<'a, T> {
            council::calls::TransactionApi::new(self.client)
        }
        pub fn elections(&self) -> elections::calls::TransactionApi<'a, T> {
            elections::calls::TransactionApi::new(self.client)
        }
        pub fn grandpa(&self) -> grandpa::calls::TransactionApi<'a, T> {
            grandpa::calls::TransactionApi::new(self.client)
        }
        pub fn treasury(&self) -> treasury::calls::TransactionApi<'a, T> {
            treasury::calls::TransactionApi::new(self.client)
        }
        pub fn utility(&self) -> utility::calls::TransactionApi<'a, T> {
            utility::calls::TransactionApi::new(self.client)
        }
        pub fn multisig(&self) -> multisig::calls::TransactionApi<'a, T> {
            multisig::calls::TransactionApi::new(self.client)
        }
        pub fn scheduler(&self) -> scheduler::calls::TransactionApi<'a, T> {
            scheduler::calls::TransactionApi::new(self.client)
        }
        pub fn proxy(&self) -> proxy::calls::TransactionApi<'a, T> {
            proxy::calls::TransactionApi::new(self.client)
        }
        pub fn assets(&self) -> assets::calls::TransactionApi<'a, T> {
            assets::calls::TransactionApi::new(self.client)
        }
        pub fn sudo(&self) -> sudo::calls::TransactionApi<'a, T> {
            sudo::calls::TransactionApi::new(self.client)
        }
        pub fn im_online(&self) -> im_online::calls::TransactionApi<'a, T> {
            im_online::calls::TransactionApi::new(self.client)
        }
        pub fn bounties(&self) -> bounties::calls::TransactionApi<'a, T> {
            bounties::calls::TransactionApi::new(self.client)
        }
        pub fn bags_list(&self) -> bags_list::calls::TransactionApi<'a, T> {
            bags_list::calls::TransactionApi::new(self.client)
        }
        pub fn hasher_bn254(
            &self,
        ) -> hasher_bn254::calls::TransactionApi<'a, T> {
            hasher_bn254::calls::TransactionApi::new(self.client)
        }
        pub fn hasher_bls381(
            &self,
        ) -> hasher_bls381::calls::TransactionApi<'a, T> {
            hasher_bls381::calls::TransactionApi::new(self.client)
        }
        pub fn asset_registry(
            &self,
        ) -> asset_registry::calls::TransactionApi<'a, T> {
            asset_registry::calls::TransactionApi::new(self.client)
        }
        pub fn currencies(&self) -> currencies::calls::TransactionApi<'a, T> {
            currencies::calls::TransactionApi::new(self.client)
        }
        pub fn tokens(&self) -> tokens::calls::TransactionApi<'a, T> {
            tokens::calls::TransactionApi::new(self.client)
        }
        pub fn token_wrapper(
            &self,
        ) -> token_wrapper::calls::TransactionApi<'a, T> {
            token_wrapper::calls::TransactionApi::new(self.client)
        }
        pub fn verifier_bn254(
            &self,
        ) -> verifier_bn254::calls::TransactionApi<'a, T> {
            verifier_bn254::calls::TransactionApi::new(self.client)
        }
        pub fn verifier_bls381(
            &self,
        ) -> verifier_bls381::calls::TransactionApi<'a, T> {
            verifier_bls381::calls::TransactionApi::new(self.client)
        }
        pub fn merkle_tree_bn254(
            &self,
        ) -> merkle_tree_bn254::calls::TransactionApi<'a, T> {
            merkle_tree_bn254::calls::TransactionApi::new(self.client)
        }
        pub fn merkle_tree_bls381(
            &self,
        ) -> merkle_tree_bls381::calls::TransactionApi<'a, T> {
            merkle_tree_bls381::calls::TransactionApi::new(self.client)
        }
        pub fn linkable_tree_bn254(
            &self,
        ) -> linkable_tree_bn254::calls::TransactionApi<'a, T> {
            linkable_tree_bn254::calls::TransactionApi::new(self.client)
        }
        pub fn linkable_tree_bls381(
            &self,
        ) -> linkable_tree_bls381::calls::TransactionApi<'a, T> {
            linkable_tree_bls381::calls::TransactionApi::new(self.client)
        }
        pub fn mixer_bn254(&self) -> mixer_bn254::calls::TransactionApi<'a, T> {
            mixer_bn254::calls::TransactionApi::new(self.client)
        }
        pub fn mixer_bls381(
            &self,
        ) -> mixer_bls381::calls::TransactionApi<'a, T> {
            mixer_bls381::calls::TransactionApi::new(self.client)
        }
        pub fn anchor_bn254(
            &self,
        ) -> anchor_bn254::calls::TransactionApi<'a, T> {
            anchor_bn254::calls::TransactionApi::new(self.client)
        }
        pub fn anchor_bls381(
            &self,
        ) -> anchor_bls381::calls::TransactionApi<'a, T> {
            anchor_bls381::calls::TransactionApi::new(self.client)
        }
        pub fn anchor_handler_bn254(
            &self,
        ) -> anchor_handler_bn254::calls::TransactionApi<'a, T> {
            anchor_handler_bn254::calls::TransactionApi::new(self.client)
        }
        pub fn anchor_handler_bls381(
            &self,
        ) -> anchor_handler_bls381::calls::TransactionApi<'a, T> {
            anchor_handler_bls381::calls::TransactionApi::new(self.client)
        }
        pub fn bridge(&self) -> bridge::calls::TransactionApi<'a, T> {
            bridge::calls::TransactionApi::new(self.client)
        }
    }
}
