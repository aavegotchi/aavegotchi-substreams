    const INTERNAL_ERR: &'static str = "`ethabi_derive` internal error";
    /// Contract's functions.
    #[allow(dead_code)]
    #[allow(unused_variables)]
    #[allow(unused_imports)]
    pub mod functions {
        use substreams_ethereum::scalar::EthBigInt;
        use super::INTERNAL_ERR;
        #[derive(Debug, Clone, PartialEq)]
        pub struct AavegotchiLevel {
            pub experience: ethabi::Uint,
        }
        impl AavegotchiLevel {
            const METHOD_ID: [u8; 4] = [249u8, 141u8, 193u8, 224u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    experience: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(&[ethabi::Token::Uint(self.experience)]);
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<ethabi::Uint, String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<ethabi::Uint, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok(
                    values
                        .pop()
                        .expect("one output data should have existed")
                        .into_uint()
                        .expect(INTERNAL_ERR),
                )
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<ethabi::Uint> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for AavegotchiLevel {
            const NAME: &'static str = "aavegotchiLevel";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<ethabi::Uint> for AavegotchiLevel {
            fn output(data: &[u8]) -> Result<ethabi::Uint, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct AavegotchiNameAvailable {
            pub name: String,
        }
        impl AavegotchiNameAvailable {
            const METHOD_ID: [u8; 4] = [241u8, 86u8, 126u8, 36u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::String],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    name: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_string()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(&[ethabi::Token::String(self.name.clone())]);
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<bool, String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<bool, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Bool],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok(
                    values
                        .pop()
                        .expect("one output data should have existed")
                        .into_bool()
                        .expect(INTERNAL_ERR),
                )
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<bool> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for AavegotchiNameAvailable {
            const NAME: &'static str = "aavegotchiNameAvailable";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<bool> for AavegotchiNameAvailable {
            fn output(data: &[u8]) -> Result<bool, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct AddErc721Listing {
            pub erc721_token_address: Vec<u8>,
            pub erc721_token_id: ethabi::Uint,
            pub price_in_wei: ethabi::Uint,
        }
        impl AddErc721Listing {
            const METHOD_ID: [u8; 4] = [49u8, 176u8, 181u8, 20u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    erc721_token_address: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    erc721_token_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    price_in_wei: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Address(
                            ethabi::Address::from_slice(&self.erc721_token_address),
                        ),
                        ethabi::Token::Uint(self.erc721_token_id),
                        ethabi::Token::Uint(self.price_in_wei),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for AddErc721Listing {
            const NAME: &'static str = "addERC721Listing";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct AddGameManagers {
            pub new_game_managers: Vec<Vec<u8>>,
            pub limits: Vec<ethabi::Uint>,
        }
        impl AddGameManagers {
            const METHOD_ID: [u8; 4] = [81u8, 233u8, 223u8, 180u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Address),
                            ),
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                            ),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    new_game_managers: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| {
                            inner.into_address().expect(INTERNAL_ERR).as_bytes().to_vec()
                        })
                        .collect(),
                    limits: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        {
                            let v = self
                                .new_game_managers
                                .iter()
                                .map(|inner| ethabi::Token::Address(
                                    ethabi::Address::from_slice(&inner),
                                ))
                                .collect();
                            ethabi::Token::Array(v)
                        },
                        {
                            let v = self
                                .limits
                                .iter()
                                .map(|inner| ethabi::Token::Uint(inner.clone()))
                                .collect();
                            ethabi::Token::Array(v)
                        },
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for AddGameManagers {
            const NAME: &'static str = "addGameManagers";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct AddGotchiLending {
            pub erc721_token_id: ethabi::Uint,
            pub initial_cost: ethabi::Uint,
            pub period: ethabi::Uint,
            pub revenue_split: [ethabi::Uint; 3usize],
            pub original_owner: Vec<u8>,
            pub third_party: Vec<u8>,
            pub whitelist_id: ethabi::Uint,
            pub revenue_tokens: Vec<Vec<u8>>,
        }
        impl AddGotchiLending {
            const METHOD_ID: [u8; 4] = [214u8, 121u8, 90u8, 39u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Uint(32usize),
                            ethabi::ParamType::Uint(96usize),
                            ethabi::ParamType::Uint(32usize),
                            ethabi::ParamType::FixedArray(
                                Box::new(ethabi::ParamType::Uint(8usize)),
                                3usize,
                            ),
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Uint(32usize),
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Address),
                            ),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    erc721_token_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    initial_cost: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    period: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    revenue_split: {
                        let mut iter = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_array()
                            .expect(INTERNAL_ERR)
                            .into_iter()
                            .map(|inner| inner.into_uint().expect(INTERNAL_ERR));
                        [
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                        ]
                    },
                    original_owner: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    third_party: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    whitelist_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    revenue_tokens: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| {
                            inner.into_address().expect(INTERNAL_ERR).as_bytes().to_vec()
                        })
                        .collect(),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Uint(self.erc721_token_id),
                        ethabi::Token::Uint(self.initial_cost),
                        ethabi::Token::Uint(self.period),
                        {
                            let v = self
                                .revenue_split
                                .iter()
                                .map(|inner| ethabi::Token::Uint(inner.clone()))
                                .collect();
                            ethabi::Token::FixedArray(v)
                        },
                        ethabi::Token::Address(
                            ethabi::Address::from_slice(&self.original_owner),
                        ),
                        ethabi::Token::Address(
                            ethabi::Address::from_slice(&self.third_party),
                        ),
                        ethabi::Token::Uint(self.whitelist_id),
                        {
                            let v = self
                                .revenue_tokens
                                .iter()
                                .map(|inner| ethabi::Token::Address(
                                    ethabi::Address::from_slice(&inner),
                                ))
                                .collect();
                            ethabi::Token::Array(v)
                        },
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for AddGotchiLending {
            const NAME: &'static str = "addGotchiLending";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct AddItemManagers {
            pub new_item_managers: Vec<Vec<u8>>,
        }
        impl AddItemManagers {
            const METHOD_ID: [u8; 4] = [21u8, 194u8, 197u8, 188u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Address),
                            ),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    new_item_managers: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| {
                            inner.into_address().expect(INTERNAL_ERR).as_bytes().to_vec()
                        })
                        .collect(),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        {
                            let v = self
                                .new_item_managers
                                .iter()
                                .map(|inner| ethabi::Token::Address(
                                    ethabi::Address::from_slice(&inner),
                                ))
                                .collect();
                            ethabi::Token::Array(v)
                        },
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for AddItemManagers {
            const NAME: &'static str = "addItemManagers";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct AgreeGotchiLending {
            pub listing_id: ethabi::Uint,
            pub erc721_token_id: ethabi::Uint,
            pub initial_cost: ethabi::Uint,
            pub period: ethabi::Uint,
            pub revenue_split: [ethabi::Uint; 3usize],
        }
        impl AgreeGotchiLending {
            const METHOD_ID: [u8; 4] = [133u8, 195u8, 195u8, 207u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Uint(32usize),
                            ethabi::ParamType::Uint(32usize),
                            ethabi::ParamType::Uint(96usize),
                            ethabi::ParamType::Uint(32usize),
                            ethabi::ParamType::FixedArray(
                                Box::new(ethabi::ParamType::Uint(8usize)),
                                3usize,
                            ),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    listing_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    erc721_token_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    initial_cost: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    period: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    revenue_split: {
                        let mut iter = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_array()
                            .expect(INTERNAL_ERR)
                            .into_iter()
                            .map(|inner| inner.into_uint().expect(INTERNAL_ERR));
                        [
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                        ]
                    },
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Uint(self.listing_id),
                        ethabi::Token::Uint(self.erc721_token_id),
                        ethabi::Token::Uint(self.initial_cost),
                        ethabi::Token::Uint(self.period),
                        {
                            let v = self
                                .revenue_split
                                .iter()
                                .map(|inner| ethabi::Token::Uint(inner.clone()))
                                .collect();
                            ethabi::Token::FixedArray(v)
                        },
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for AgreeGotchiLending {
            const NAME: &'static str = "agreeGotchiLending";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct Approve {
            pub approved: Vec<u8>,
            pub token_id: ethabi::Uint,
        }
        impl Approve {
            const METHOD_ID: [u8; 4] = [9u8, 94u8, 167u8, 179u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Address, ethabi::ParamType::Uint(256usize)],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    approved: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    token_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Address(
                            ethabi::Address::from_slice(&self.approved),
                        ),
                        ethabi::Token::Uint(self.token_id),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for Approve {
            const NAME: &'static str = "approve";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct AvailableSkillPoints {
            pub token_id: ethabi::Uint,
        }
        impl AvailableSkillPoints {
            const METHOD_ID: [u8; 4] = [164u8, 195u8, 102u8, 254u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    token_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(&[ethabi::Token::Uint(self.token_id)]);
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<ethabi::Uint, String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<ethabi::Uint, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok(
                    values
                        .pop()
                        .expect("one output data should have existed")
                        .into_uint()
                        .expect(INTERNAL_ERR),
                )
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<ethabi::Uint> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for AvailableSkillPoints {
            const NAME: &'static str = "availableSkillPoints";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<ethabi::Uint>
        for AvailableSkillPoints {
            fn output(data: &[u8]) -> Result<ethabi::Uint, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct BalanceOf1 {
            pub owner: Vec<u8>,
        }
        impl BalanceOf1 {
            const METHOD_ID: [u8; 4] = [112u8, 160u8, 130u8, 49u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Address],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    owner: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[ethabi::Token::Address(ethabi::Address::from_slice(&self.owner))],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<ethabi::Uint, String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<ethabi::Uint, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok(
                    values
                        .pop()
                        .expect("one output data should have existed")
                        .into_uint()
                        .expect(INTERNAL_ERR),
                )
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<ethabi::Uint> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for BalanceOf1 {
            const NAME: &'static str = "balanceOf1";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<ethabi::Uint> for BalanceOf1 {
            fn output(data: &[u8]) -> Result<ethabi::Uint, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct BalanceOf2 {
            pub owner: Vec<u8>,
            pub id: ethabi::Uint,
        }
        impl BalanceOf2 {
            const METHOD_ID: [u8; 4] = [0u8, 253u8, 213u8, 142u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Address, ethabi::ParamType::Uint(256usize)],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    owner: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Address(ethabi::Address::from_slice(&self.owner)),
                        ethabi::Token::Uint(self.id),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<ethabi::Uint, String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<ethabi::Uint, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok(
                    values
                        .pop()
                        .expect("one output data should have existed")
                        .into_uint()
                        .expect(INTERNAL_ERR),
                )
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<ethabi::Uint> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for BalanceOf2 {
            const NAME: &'static str = "balanceOf2";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<ethabi::Uint> for BalanceOf2 {
            fn output(data: &[u8]) -> Result<ethabi::Uint, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct BalanceOfBatch {
            pub owners: Vec<Vec<u8>>,
            pub ids: Vec<ethabi::Uint>,
        }
        impl BalanceOfBatch {
            const METHOD_ID: [u8; 4] = [78u8, 18u8, 115u8, 244u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Address),
                            ),
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                            ),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    owners: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| {
                            inner.into_address().expect(INTERNAL_ERR).as_bytes().to_vec()
                        })
                        .collect(),
                    ids: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        {
                            let v = self
                                .owners
                                .iter()
                                .map(|inner| ethabi::Token::Address(
                                    ethabi::Address::from_slice(&inner),
                                ))
                                .collect();
                            ethabi::Token::Array(v)
                        },
                        {
                            let v = self
                                .ids
                                .iter()
                                .map(|inner| ethabi::Token::Uint(inner.clone()))
                                .collect();
                            ethabi::Token::Array(v)
                        },
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Vec<ethabi::Uint>, String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<Vec<ethabi::Uint>, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                            ),
                        ],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok(
                    values
                        .pop()
                        .expect("one output data should have existed")
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
                )
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<Vec<ethabi::Uint>> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for BalanceOfBatch {
            const NAME: &'static str = "balanceOfBatch";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<Vec<ethabi::Uint>>
        for BalanceOfBatch {
            fn output(data: &[u8]) -> Result<Vec<ethabi::Uint>, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct BalanceOfToken {
            pub token_contract: Vec<u8>,
            pub token_id: ethabi::Uint,
            pub id: ethabi::Uint,
        }
        impl BalanceOfToken {
            const METHOD_ID: [u8; 4] = [57u8, 162u8, 9u8, 223u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    token_contract: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    token_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Address(
                            ethabi::Address::from_slice(&self.token_contract),
                        ),
                        ethabi::Token::Uint(self.token_id),
                        ethabi::Token::Uint(self.id),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<ethabi::Uint, String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<ethabi::Uint, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok(
                    values
                        .pop()
                        .expect("one output data should have existed")
                        .into_uint()
                        .expect(INTERNAL_ERR),
                )
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<ethabi::Uint> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for BalanceOfToken {
            const NAME: &'static str = "balanceOfToken";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<ethabi::Uint> for BalanceOfToken {
            fn output(data: &[u8]) -> Result<ethabi::Uint, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct BaseRarityScore {
            pub numeric_traits: [EthBigInt; 6usize],
        }
        impl BaseRarityScore {
            const METHOD_ID: [u8; 4] = [154u8, 191u8, 108u8, 81u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::FixedArray(
                                Box::new(ethabi::ParamType::Int(16usize)),
                                6usize,
                            ),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    numeric_traits: {
                        let mut iter = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_array()
                            .expect(INTERNAL_ERR)
                            .into_iter()
                            .map(|inner| {
                                let mut v = [0 as u8; 32];
                                inner
                                    .into_int()
                                    .expect(INTERNAL_ERR)
                                    .to_big_endian(v.as_mut_slice());
                                EthBigInt::new(v.into())
                            });
                        [
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                        ]
                    },
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        {
                            let v = self
                                .numeric_traits
                                .iter()
                                .map(|inner| {
                                    let non_full_signed_bytes = inner
                                        .get_big_int()
                                        .to_signed_bytes_be();
                                    let mut full_signed_bytes = [0xff as u8; 32];
                                    non_full_signed_bytes
                                        .into_iter()
                                        .rev()
                                        .enumerate()
                                        .for_each(|(i, byte)| full_signed_bytes[31 - i] = byte);
                                    ethabi::Token::Int(
                                        ethabi::Int::from_big_endian(full_signed_bytes.as_ref()),
                                    )
                                })
                                .collect();
                            ethabi::Token::FixedArray(v)
                        },
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<ethabi::Uint, String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<ethabi::Uint, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok(
                    values
                        .pop()
                        .expect("one output data should have existed")
                        .into_uint()
                        .expect(INTERNAL_ERR),
                )
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<ethabi::Uint> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for BaseRarityScore {
            const NAME: &'static str = "baseRarityScore";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<ethabi::Uint> for BaseRarityScore {
            fn output(data: &[u8]) -> Result<ethabi::Uint, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct BatchDepositErc20 {
            pub token_ids: Vec<ethabi::Uint>,
            pub erc20_contracts: Vec<Vec<u8>>,
            pub values: Vec<ethabi::Uint>,
        }
        impl BatchDepositErc20 {
            const METHOD_ID: [u8; 4] = [70u8, 122u8, 181u8, 207u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                            ),
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Address),
                            ),
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                            ),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    token_ids: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
                    erc20_contracts: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| {
                            inner.into_address().expect(INTERNAL_ERR).as_bytes().to_vec()
                        })
                        .collect(),
                    values: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        {
                            let v = self
                                .token_ids
                                .iter()
                                .map(|inner| ethabi::Token::Uint(inner.clone()))
                                .collect();
                            ethabi::Token::Array(v)
                        },
                        {
                            let v = self
                                .erc20_contracts
                                .iter()
                                .map(|inner| ethabi::Token::Address(
                                    ethabi::Address::from_slice(&inner),
                                ))
                                .collect();
                            ethabi::Token::Array(v)
                        },
                        {
                            let v = self
                                .values
                                .iter()
                                .map(|inner| ethabi::Token::Uint(inner.clone()))
                                .collect();
                            ethabi::Token::Array(v)
                        },
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for BatchDepositErc20 {
            const NAME: &'static str = "batchDepositERC20";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct BatchDepositGhst {
            pub token_ids: Vec<ethabi::Uint>,
            pub values: Vec<ethabi::Uint>,
        }
        impl BatchDepositGhst {
            const METHOD_ID: [u8; 4] = [234u8, 32u8, 195u8, 198u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                            ),
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                            ),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    token_ids: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
                    values: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        {
                            let v = self
                                .token_ids
                                .iter()
                                .map(|inner| ethabi::Token::Uint(inner.clone()))
                                .collect();
                            ethabi::Token::Array(v)
                        },
                        {
                            let v = self
                                .values
                                .iter()
                                .map(|inner| ethabi::Token::Uint(inner.clone()))
                                .collect();
                            ethabi::Token::Array(v)
                        },
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for BatchDepositGhst {
            const NAME: &'static str = "batchDepositGHST";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct BatchTransferAsChild {
            pub from_contract: Vec<u8>,
            pub from_token_id: ethabi::Uint,
            pub to_contract: Vec<u8>,
            pub to_token_id: ethabi::Uint,
            pub ids: Vec<ethabi::Uint>,
            pub values: Vec<ethabi::Uint>,
        }
        impl BatchTransferAsChild {
            const METHOD_ID: [u8; 4] = [106u8, 137u8, 38u8, 232u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                            ),
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                            ),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    from_contract: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    from_token_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    to_contract: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    to_token_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    ids: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
                    values: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Address(
                            ethabi::Address::from_slice(&self.from_contract),
                        ),
                        ethabi::Token::Uint(self.from_token_id),
                        ethabi::Token::Address(
                            ethabi::Address::from_slice(&self.to_contract),
                        ),
                        ethabi::Token::Uint(self.to_token_id),
                        {
                            let v = self
                                .ids
                                .iter()
                                .map(|inner| ethabi::Token::Uint(inner.clone()))
                                .collect();
                            ethabi::Token::Array(v)
                        },
                        {
                            let v = self
                                .values
                                .iter()
                                .map(|inner| ethabi::Token::Uint(inner.clone()))
                                .collect();
                            ethabi::Token::Array(v)
                        },
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for BatchTransferAsChild {
            const NAME: &'static str = "batchTransferAsChild";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct BatchTransferFromParent {
            pub from_contract: Vec<u8>,
            pub from_token_id: ethabi::Uint,
            pub to: Vec<u8>,
            pub ids: Vec<ethabi::Uint>,
            pub values: Vec<ethabi::Uint>,
        }
        impl BatchTransferFromParent {
            const METHOD_ID: [u8; 4] = [108u8, 113u8, 196u8, 11u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                            ),
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                            ),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    from_contract: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    from_token_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    to: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    ids: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
                    values: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Address(
                            ethabi::Address::from_slice(&self.from_contract),
                        ),
                        ethabi::Token::Uint(self.from_token_id),
                        ethabi::Token::Address(ethabi::Address::from_slice(&self.to)),
                        {
                            let v = self
                                .ids
                                .iter()
                                .map(|inner| ethabi::Token::Uint(inner.clone()))
                                .collect();
                            ethabi::Token::Array(v)
                        },
                        {
                            let v = self
                                .values
                                .iter()
                                .map(|inner| ethabi::Token::Uint(inner.clone()))
                                .collect();
                            ethabi::Token::Array(v)
                        },
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for BatchTransferFromParent {
            const NAME: &'static str = "batchTransferFromParent";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct BatchTransferToParent {
            pub from: Vec<u8>,
            pub to_contract: Vec<u8>,
            pub to_token_id: ethabi::Uint,
            pub ids: Vec<ethabi::Uint>,
            pub values: Vec<ethabi::Uint>,
        }
        impl BatchTransferToParent {
            const METHOD_ID: [u8; 4] = [234u8, 105u8, 170u8, 226u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                            ),
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                            ),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    from: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    to_contract: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    to_token_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    ids: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
                    values: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Address(ethabi::Address::from_slice(&self.from)),
                        ethabi::Token::Address(
                            ethabi::Address::from_slice(&self.to_contract),
                        ),
                        ethabi::Token::Uint(self.to_token_id),
                        {
                            let v = self
                                .ids
                                .iter()
                                .map(|inner| ethabi::Token::Uint(inner.clone()))
                                .collect();
                            ethabi::Token::Array(v)
                        },
                        {
                            let v = self
                                .values
                                .iter()
                                .map(|inner| ethabi::Token::Uint(inner.clone()))
                                .collect();
                            ethabi::Token::Array(v)
                        },
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for BatchTransferToParent {
            const NAME: &'static str = "batchTransferToParent";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct BuyPortals {
            pub to: Vec<u8>,
            pub ghst: ethabi::Uint,
        }
        impl BuyPortals {
            const METHOD_ID: [u8; 4] = [224u8, 167u8, 105u8, 71u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Address, ethabi::ParamType::Uint(256usize)],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    to: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    ghst: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Address(ethabi::Address::from_slice(&self.to)),
                        ethabi::Token::Uint(self.ghst),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for BuyPortals {
            const NAME: &'static str = "buyPortals";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct CancelErc1155Listing {
            pub listing_id: ethabi::Uint,
        }
        impl CancelErc1155Listing {
            const METHOD_ID: [u8; 4] = [8u8, 100u8, 46u8, 36u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    listing_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(&[ethabi::Token::Uint(self.listing_id)]);
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for CancelErc1155Listing {
            const NAME: &'static str = "cancelERC1155Listing";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct CancelErc1155Listings {
            pub listing_ids: Vec<ethabi::Uint>,
        }
        impl CancelErc1155Listings {
            const METHOD_ID: [u8; 4] = [23u8, 202u8, 54u8, 129u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                            ),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    listing_ids: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        {
                            let v = self
                                .listing_ids
                                .iter()
                                .map(|inner| ethabi::Token::Uint(inner.clone()))
                                .collect();
                            ethabi::Token::Array(v)
                        },
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for CancelErc1155Listings {
            const NAME: &'static str = "cancelERC1155Listings";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct CancelErc721Listing {
            pub listing_id: ethabi::Uint,
        }
        impl CancelErc721Listing {
            const METHOD_ID: [u8; 4] = [223u8, 58u8, 214u8, 209u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    listing_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(&[ethabi::Token::Uint(self.listing_id)]);
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for CancelErc721Listing {
            const NAME: &'static str = "cancelERC721Listing";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct CancelErc721ListingByToken {
            pub erc721_token_address: Vec<u8>,
            pub erc721_token_id: ethabi::Uint,
        }
        impl CancelErc721ListingByToken {
            const METHOD_ID: [u8; 4] = [186u8, 114u8, 107u8, 228u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Address, ethabi::ParamType::Uint(256usize)],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    erc721_token_address: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    erc721_token_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Address(
                            ethabi::Address::from_slice(&self.erc721_token_address),
                        ),
                        ethabi::Token::Uint(self.erc721_token_id),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for CancelErc721ListingByToken {
            const NAME: &'static str = "cancelERC721ListingByToken";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct CancelErc721Listings {
            pub listing_ids: Vec<ethabi::Uint>,
        }
        impl CancelErc721Listings {
            const METHOD_ID: [u8; 4] = [10u8, 223u8, 171u8, 23u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                            ),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    listing_ids: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        {
                            let v = self
                                .listing_ids
                                .iter()
                                .map(|inner| ethabi::Token::Uint(inner.clone()))
                                .collect();
                            ethabi::Token::Array(v)
                        },
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for CancelErc721Listings {
            const NAME: &'static str = "cancelERC721Listings";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct CancelGotchiLending {
            pub listing_id: ethabi::Uint,
        }
        impl CancelGotchiLending {
            const METHOD_ID: [u8; 4] = [240u8, 176u8, 158u8, 59u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(32usize)],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    listing_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(&[ethabi::Token::Uint(self.listing_id)]);
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for CancelGotchiLending {
            const NAME: &'static str = "cancelGotchiLending";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct CancelGotchiLendingByToken {
            pub erc721_token_id: ethabi::Uint,
        }
        impl CancelGotchiLendingByToken {
            const METHOD_ID: [u8; 4] = [247u8, 180u8, 237u8, 218u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(32usize)],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    erc721_token_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(&[ethabi::Token::Uint(self.erc721_token_id)]);
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for CancelGotchiLendingByToken {
            const NAME: &'static str = "cancelGotchiLendingByToken";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct ChangeVrf {
            pub new_fee: ethabi::Uint,
            pub key_hash: [u8; 32usize],
            pub vrf_coordinator: Vec<u8>,
            pub link: Vec<u8>,
        }
        impl ChangeVrf {
            const METHOD_ID: [u8; 4] = [229u8, 156u8, 47u8, 12u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::FixedBytes(32usize),
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Address,
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    new_fee: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    key_hash: {
                        let mut result = [0u8; 32];
                        let v = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_bytes()
                            .expect(INTERNAL_ERR);
                        result.copy_from_slice(&v);
                        result
                    },
                    vrf_coordinator: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    link: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Uint(self.new_fee),
                        ethabi::Token::FixedBytes(self.key_hash.as_ref().to_vec()),
                        ethabi::Token::Address(
                            ethabi::Address::from_slice(&self.vrf_coordinator),
                        ),
                        ethabi::Token::Address(ethabi::Address::from_slice(&self.link)),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for ChangeVrf {
            const NAME: &'static str = "changeVrf";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct ChildChainManager {}
        impl ChildChainManager {
            const METHOD_ID: [u8; 4] = [200u8, 41u8, 29u8, 132u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Ok(Self {})
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(&[]);
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Vec<u8>, String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<Vec<u8>, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Address],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok(
                    values
                        .pop()
                        .expect("one output data should have existed")
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                )
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<Vec<u8>> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for ChildChainManager {
            const NAME: &'static str = "childChainManager";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<Vec<u8>> for ChildChainManager {
            fn output(data: &[u8]) -> Result<Vec<u8>, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct ClaimAavegotchi {
            pub token_id: ethabi::Uint,
            pub option: ethabi::Uint,
            pub stake_amount: ethabi::Uint,
        }
        impl ClaimAavegotchi {
            const METHOD_ID: [u8; 4] = [73u8, 132u8, 176u8, 28u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    token_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    option: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    stake_amount: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Uint(self.token_id),
                        ethabi::Token::Uint(self.option),
                        ethabi::Token::Uint(self.stake_amount),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for ClaimAavegotchi {
            const NAME: &'static str = "claimAavegotchi";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct ClaimAndEndGotchiLending {
            pub token_id: ethabi::Uint,
        }
        impl ClaimAndEndGotchiLending {
            const METHOD_ID: [u8; 4] = [174u8, 12u8, 139u8, 216u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(32usize)],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    token_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(&[ethabi::Token::Uint(self.token_id)]);
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for ClaimAndEndGotchiLending {
            const NAME: &'static str = "claimAndEndGotchiLending";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct ClaimGotchiLending {
            pub token_id: ethabi::Uint,
        }
        impl ClaimGotchiLending {
            const METHOD_ID: [u8; 4] = [255u8, 167u8, 128u8, 45u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(32usize)],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    token_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(&[ethabi::Token::Uint(self.token_id)]);
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for ClaimGotchiLending {
            const NAME: &'static str = "claimGotchiLending";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct CollateralBalance {
            pub token_id: ethabi::Uint,
        }
        impl CollateralBalance {
            const METHOD_ID: [u8; 4] = [76u8, 213u8, 209u8, 247u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    token_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(&[ethabi::Token::Uint(self.token_id)]);
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<(Vec<u8>, Vec<u8>, ethabi::Uint), String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(
                data: &[u8],
            ) -> Result<(Vec<u8>, Vec<u8>, ethabi::Uint), String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Uint(256usize),
                        ],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                values.reverse();
                Ok((
                    values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    values.pop().expect(INTERNAL_ERR).into_uint().expect(INTERNAL_ERR),
                ))
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(
                &self,
                address: Vec<u8>,
            ) -> Option<(Vec<u8>, Vec<u8>, ethabi::Uint)> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for CollateralBalance {
            const NAME: &'static str = "collateralBalance";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<(Vec<u8>, Vec<u8>, ethabi::Uint)>
        for CollateralBalance {
            fn output(data: &[u8]) -> Result<(Vec<u8>, Vec<u8>, ethabi::Uint), String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct Collaterals {
            pub haunt_id: ethabi::Uint,
        }
        impl Collaterals {
            const METHOD_ID: [u8; 4] = [36u8, 193u8, 23u8, 59u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    haunt_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(&[ethabi::Token::Uint(self.haunt_id)]);
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Vec<Vec<u8>>, String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<Vec<Vec<u8>>, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Address),
                            ),
                        ],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok(
                    values
                        .pop()
                        .expect("one output data should have existed")
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| {
                            inner.into_address().expect(INTERNAL_ERR).as_bytes().to_vec()
                        })
                        .collect(),
                )
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<Vec<Vec<u8>>> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for Collaterals {
            const NAME: &'static str = "collaterals";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<Vec<Vec<u8>>> for Collaterals {
            fn output(data: &[u8]) -> Result<Vec<Vec<u8>>, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct CreateHaunt {
            pub haunt_max_size: ethabi::Uint,
            pub portal_price: ethabi::Uint,
            pub body_color: [u8; 3usize],
        }
        impl CreateHaunt {
            const METHOD_ID: [u8; 4] = [18u8, 109u8, 21u8, 3u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Uint(24usize),
                            ethabi::ParamType::Uint(96usize),
                            ethabi::ParamType::FixedBytes(3usize),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    haunt_max_size: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    portal_price: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    body_color: {
                        let mut result = [0u8; 3];
                        let v = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_bytes()
                            .expect(INTERNAL_ERR);
                        result.copy_from_slice(&v);
                        result
                    },
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Uint(self.haunt_max_size),
                        ethabi::Token::Uint(self.portal_price),
                        ethabi::Token::FixedBytes(self.body_color.as_ref().to_vec()),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<ethabi::Uint, String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<ethabi::Uint, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok(
                    values
                        .pop()
                        .expect("one output data should have existed")
                        .into_uint()
                        .expect(INTERNAL_ERR),
                )
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<ethabi::Uint> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for CreateHaunt {
            const NAME: &'static str = "createHaunt";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<ethabi::Uint> for CreateHaunt {
            fn output(data: &[u8]) -> Result<ethabi::Uint, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct CreateWhitelist {
            pub name: String,
            pub whitelist_addresses: Vec<Vec<u8>>,
        }
        impl CreateWhitelist {
            const METHOD_ID: [u8; 4] = [213u8, 8u8, 217u8, 168u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::String,
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Address),
                            ),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    name: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_string()
                        .expect(INTERNAL_ERR),
                    whitelist_addresses: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| {
                            inner.into_address().expect(INTERNAL_ERR).as_bytes().to_vec()
                        })
                        .collect(),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::String(self.name.clone()),
                        {
                            let v = self
                                .whitelist_addresses
                                .iter()
                                .map(|inner| ethabi::Token::Address(
                                    ethabi::Address::from_slice(&inner),
                                ))
                                .collect();
                            ethabi::Token::Array(v)
                        },
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for CreateWhitelist {
            const NAME: &'static str = "createWhitelist";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct DecreaseAndDestroy {
            pub token_id: ethabi::Uint,
            pub to_id: ethabi::Uint,
        }
        impl DecreaseAndDestroy {
            const METHOD_ID: [u8; 4] = [73u8, 170u8, 31u8, 39u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    token_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    to_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Uint(self.token_id),
                        ethabi::Token::Uint(self.to_id),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for DecreaseAndDestroy {
            const NAME: &'static str = "decreaseAndDestroy";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct DecreaseStake {
            pub token_id: ethabi::Uint,
            pub reduce_amount: ethabi::Uint,
        }
        impl DecreaseStake {
            const METHOD_ID: [u8; 4] = [36u8, 216u8, 111u8, 0u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    token_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    reduce_amount: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Uint(self.token_id),
                        ethabi::Token::Uint(self.reduce_amount),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for DecreaseStake {
            const NAME: &'static str = "decreaseStake";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct DeleteLastSvgLayers {
            pub svg_type: [u8; 32usize],
            pub num_layers: ethabi::Uint,
        }
        impl DeleteLastSvgLayers {
            const METHOD_ID: [u8; 4] = [253u8, 145u8, 131u8, 143u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::FixedBytes(32usize),
                            ethabi::ParamType::Uint(256usize),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    svg_type: {
                        let mut result = [0u8; 32];
                        let v = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_bytes()
                            .expect(INTERNAL_ERR);
                        result.copy_from_slice(&v);
                        result
                    },
                    num_layers: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::FixedBytes(self.svg_type.as_ref().to_vec()),
                        ethabi::Token::Uint(self.num_layers),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for DeleteLastSvgLayers {
            const NAME: &'static str = "deleteLastSvgLayers";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct Deposit {
            pub user: Vec<u8>,
            pub deposit_data: Vec<u8>,
        }
        impl Deposit {
            const METHOD_ID: [u8; 4] = [207u8, 44u8, 82u8, 203u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Address, ethabi::ParamType::Bytes],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    user: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    deposit_data: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_bytes()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Address(ethabi::Address::from_slice(&self.user)),
                        ethabi::Token::Bytes(self.deposit_data.clone()),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for Deposit {
            const NAME: &'static str = "deposit";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct DepositErc20 {
            pub token_id: ethabi::Uint,
            pub erc20_contract: Vec<u8>,
            pub value: ethabi::Uint,
        }
        impl DepositErc20 {
            const METHOD_ID: [u8; 4] = [176u8, 250u8, 202u8, 179u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Uint(256usize),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    token_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    erc20_contract: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    value: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Uint(self.token_id),
                        ethabi::Token::Address(
                            ethabi::Address::from_slice(&self.erc20_contract),
                        ),
                        ethabi::Token::Uint(self.value),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for DepositErc20 {
            const NAME: &'static str = "depositERC20";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct EquipWearables {
            pub token_id: ethabi::Uint,
            pub wearables_to_equip: [ethabi::Uint; 16usize],
        }
        impl EquipWearables {
            const METHOD_ID: [u8; 4] = [159u8, 123u8, 145u8, 198u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::FixedArray(
                                Box::new(ethabi::ParamType::Uint(16usize)),
                                16usize,
                            ),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    token_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    wearables_to_equip: {
                        let mut iter = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_array()
                            .expect(INTERNAL_ERR)
                            .into_iter()
                            .map(|inner| inner.into_uint().expect(INTERNAL_ERR));
                        [
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                        ]
                    },
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Uint(self.token_id),
                        {
                            let v = self
                                .wearables_to_equip
                                .iter()
                                .map(|inner| ethabi::Token::Uint(inner.clone()))
                                .collect();
                            ethabi::Token::FixedArray(v)
                        },
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for EquipWearables {
            const NAME: &'static str = "equipWearables";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct EquippedWearables {
            pub token_id: ethabi::Uint,
        }
        impl EquippedWearables {
            const METHOD_ID: [u8; 4] = [71u8, 241u8, 140u8, 4u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    token_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(&[ethabi::Token::Uint(self.token_id)]);
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<[ethabi::Uint; 16usize], String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<[ethabi::Uint; 16usize], String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::FixedArray(
                                Box::new(ethabi::ParamType::Uint(16usize)),
                                16usize,
                            ),
                        ],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok({
                    let mut iter = values
                        .pop()
                        .expect("one output data should have existed")
                        .into_fixed_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR));
                    [
                        iter.next().expect(INTERNAL_ERR),
                        iter.next().expect(INTERNAL_ERR),
                        iter.next().expect(INTERNAL_ERR),
                        iter.next().expect(INTERNAL_ERR),
                        iter.next().expect(INTERNAL_ERR),
                        iter.next().expect(INTERNAL_ERR),
                        iter.next().expect(INTERNAL_ERR),
                        iter.next().expect(INTERNAL_ERR),
                        iter.next().expect(INTERNAL_ERR),
                        iter.next().expect(INTERNAL_ERR),
                        iter.next().expect(INTERNAL_ERR),
                        iter.next().expect(INTERNAL_ERR),
                        iter.next().expect(INTERNAL_ERR),
                        iter.next().expect(INTERNAL_ERR),
                        iter.next().expect(INTERNAL_ERR),
                        iter.next().expect(INTERNAL_ERR),
                    ]
                })
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<[ethabi::Uint; 16usize]> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for EquippedWearables {
            const NAME: &'static str = "equippedWearables";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<[ethabi::Uint; 16usize]>
        for EquippedWearables {
            fn output(data: &[u8]) -> Result<[ethabi::Uint; 16usize], String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct EscrowBalance {
            pub token_id: ethabi::Uint,
            pub erc20_contract: Vec<u8>,
        }
        impl EscrowBalance {
            const METHOD_ID: [u8; 4] = [219u8, 135u8, 219u8, 82u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize), ethabi::ParamType::Address],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    token_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    erc20_contract: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Uint(self.token_id),
                        ethabi::Token::Address(
                            ethabi::Address::from_slice(&self.erc20_contract),
                        ),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<ethabi::Uint, String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<ethabi::Uint, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok(
                    values
                        .pop()
                        .expect("one output data should have existed")
                        .into_uint()
                        .expect(INTERNAL_ERR),
                )
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<ethabi::Uint> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for EscrowBalance {
            const NAME: &'static str = "escrowBalance";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<ethabi::Uint> for EscrowBalance {
            fn output(data: &[u8]) -> Result<ethabi::Uint, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct ExecuteErc1155Listing {
            pub listing_id: ethabi::Uint,
            pub quantity: ethabi::Uint,
            pub price_in_wei: ethabi::Uint,
        }
        impl ExecuteErc1155Listing {
            const METHOD_ID: [u8; 4] = [87u8, 90u8, 232u8, 118u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    listing_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    quantity: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    price_in_wei: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Uint(self.listing_id),
                        ethabi::Token::Uint(self.quantity),
                        ethabi::Token::Uint(self.price_in_wei),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for ExecuteErc1155Listing {
            const NAME: &'static str = "executeERC1155Listing";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct ExecuteErc721Listing {
            pub listing_id: ethabi::Uint,
        }
        impl ExecuteErc721Listing {
            const METHOD_ID: [u8; 4] = [241u8, 122u8, 92u8, 108u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    listing_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(&[ethabi::Token::Uint(self.listing_id)]);
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for ExecuteErc721Listing {
            const NAME: &'static str = "executeERC721Listing";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct ExecuteMetaTransaction {
            pub user_address: Vec<u8>,
            pub function_signature: Vec<u8>,
            pub sig_r: [u8; 32usize],
            pub sig_s: [u8; 32usize],
            pub sig_v: ethabi::Uint,
        }
        impl ExecuteMetaTransaction {
            const METHOD_ID: [u8; 4] = [12u8, 83u8, 197u8, 28u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Bytes,
                            ethabi::ParamType::FixedBytes(32usize),
                            ethabi::ParamType::FixedBytes(32usize),
                            ethabi::ParamType::Uint(8usize),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    user_address: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    function_signature: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_bytes()
                        .expect(INTERNAL_ERR),
                    sig_r: {
                        let mut result = [0u8; 32];
                        let v = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_bytes()
                            .expect(INTERNAL_ERR);
                        result.copy_from_slice(&v);
                        result
                    },
                    sig_s: {
                        let mut result = [0u8; 32];
                        let v = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_bytes()
                            .expect(INTERNAL_ERR);
                        result.copy_from_slice(&v);
                        result
                    },
                    sig_v: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Address(
                            ethabi::Address::from_slice(&self.user_address),
                        ),
                        ethabi::Token::Bytes(self.function_signature.clone()),
                        ethabi::Token::FixedBytes(self.sig_r.as_ref().to_vec()),
                        ethabi::Token::FixedBytes(self.sig_s.as_ref().to_vec()),
                        ethabi::Token::Uint(self.sig_v),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Vec<u8>, String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<Vec<u8>, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Bytes],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok(
                    values
                        .pop()
                        .expect("one output data should have existed")
                        .into_bytes()
                        .expect(INTERNAL_ERR),
                )
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<Vec<u8>> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for ExecuteMetaTransaction {
            const NAME: &'static str = "executeMetaTransaction";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<Vec<u8>> for ExecuteMetaTransaction {
            fn output(data: &[u8]) -> Result<Vec<u8>, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct ExtractItemsFromDiamond {
            pub to: Vec<u8>,
            pub item_ids: Vec<ethabi::Uint>,
            pub values: Vec<ethabi::Uint>,
        }
        impl ExtractItemsFromDiamond {
            const METHOD_ID: [u8; 4] = [111u8, 95u8, 208u8, 20u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                            ),
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                            ),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    to: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    item_ids: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
                    values: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Address(ethabi::Address::from_slice(&self.to)),
                        {
                            let v = self
                                .item_ids
                                .iter()
                                .map(|inner| ethabi::Token::Uint(inner.clone()))
                                .collect();
                            ethabi::Token::Array(v)
                        },
                        {
                            let v = self
                                .values
                                .iter()
                                .map(|inner| ethabi::Token::Uint(inner.clone()))
                                .collect();
                            ethabi::Token::Array(v)
                        },
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for ExtractItemsFromDiamond {
            const NAME: &'static str = "extractItemsFromDiamond";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct ExtractItemsFromSacrificedGotchi {
            pub to: Vec<u8>,
            pub token_id: ethabi::Uint,
            pub item_ids: Vec<ethabi::Uint>,
            pub values: Vec<ethabi::Uint>,
        }
        impl ExtractItemsFromSacrificedGotchi {
            const METHOD_ID: [u8; 4] = [30u8, 146u8, 226u8, 244u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                            ),
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                            ),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    to: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    token_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    item_ids: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
                    values: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Address(ethabi::Address::from_slice(&self.to)),
                        ethabi::Token::Uint(self.token_id),
                        {
                            let v = self
                                .item_ids
                                .iter()
                                .map(|inner| ethabi::Token::Uint(inner.clone()))
                                .collect();
                            ethabi::Token::Array(v)
                        },
                        {
                            let v = self
                                .values
                                .iter()
                                .map(|inner| ethabi::Token::Uint(inner.clone()))
                                .collect();
                            ethabi::Token::Array(v)
                        },
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for ExtractItemsFromSacrificedGotchi {
            const NAME: &'static str = "extractItemsFromSacrificedGotchi";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct FindWearableSets {
            pub wearable_ids: Vec<ethabi::Uint>,
        }
        impl FindWearableSets {
            const METHOD_ID: [u8; 4] = [171u8, 111u8, 68u8, 109u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                            ),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    wearable_ids: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        {
                            let v = self
                                .wearable_ids
                                .iter()
                                .map(|inner| ethabi::Token::Uint(inner.clone()))
                                .collect();
                            ethabi::Token::Array(v)
                        },
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Vec<ethabi::Uint>, String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<Vec<ethabi::Uint>, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                            ),
                        ],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok(
                    values
                        .pop()
                        .expect("one output data should have existed")
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
                )
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<Vec<ethabi::Uint>> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for FindWearableSets {
            const NAME: &'static str = "findWearableSets";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<Vec<ethabi::Uint>>
        for FindWearableSets {
            fn output(data: &[u8]) -> Result<Vec<ethabi::Uint>, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct GameManagerBalance {
            pub manager: Vec<u8>,
        }
        impl GameManagerBalance {
            const METHOD_ID: [u8; 4] = [154u8, 73u8, 77u8, 160u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Address],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    manager: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[ethabi::Token::Address(ethabi::Address::from_slice(&self.manager))],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<ethabi::Uint, String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<ethabi::Uint, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok(
                    values
                        .pop()
                        .expect("one output data should have existed")
                        .into_uint()
                        .expect(INTERNAL_ERR),
                )
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<ethabi::Uint> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for GameManagerBalance {
            const NAME: &'static str = "gameManagerBalance";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<ethabi::Uint>
        for GameManagerBalance {
            fn output(data: &[u8]) -> Result<ethabi::Uint, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct GameManagerRefreshTime {
            pub manager: Vec<u8>,
        }
        impl GameManagerRefreshTime {
            const METHOD_ID: [u8; 4] = [250u8, 26u8, 11u8, 33u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Address],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    manager: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[ethabi::Token::Address(ethabi::Address::from_slice(&self.manager))],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<ethabi::Uint, String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<ethabi::Uint, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok(
                    values
                        .pop()
                        .expect("one output data should have existed")
                        .into_uint()
                        .expect(INTERNAL_ERR),
                )
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<ethabi::Uint> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for GameManagerRefreshTime {
            const NAME: &'static str = "gameManagerRefreshTime";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<ethabi::Uint>
        for GameManagerRefreshTime {
            fn output(data: &[u8]) -> Result<ethabi::Uint, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct GetAavegotchiSideSvgs {
            pub token_id: ethabi::Uint,
        }
        impl GetAavegotchiSideSvgs {
            const METHOD_ID: [u8; 4] = [110u8, 149u8, 7u8, 234u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    token_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(&[ethabi::Token::Uint(self.token_id)]);
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Vec<String>, String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<Vec<String>, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Array(Box::new(ethabi::ParamType::String))],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok(
                    values
                        .pop()
                        .expect("one output data should have existed")
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_string().expect(INTERNAL_ERR))
                        .collect(),
                )
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<Vec<String>> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for GetAavegotchiSideSvgs {
            const NAME: &'static str = "getAavegotchiSideSvgs";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<Vec<String>>
        for GetAavegotchiSideSvgs {
            fn output(data: &[u8]) -> Result<Vec<String>, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct GetAavegotchiSvg {
            pub token_id: ethabi::Uint,
        }
        impl GetAavegotchiSvg {
            const METHOD_ID: [u8; 4] = [148u8, 95u8, 78u8, 36u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    token_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(&[ethabi::Token::Uint(self.token_id)]);
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<String, String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<String, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::String],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok(
                    values
                        .pop()
                        .expect("one output data should have existed")
                        .into_string()
                        .expect(INTERNAL_ERR),
                )
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<String> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for GetAavegotchiSvg {
            const NAME: &'static str = "getAavegotchiSvg";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<String> for GetAavegotchiSvg {
            fn output(data: &[u8]) -> Result<String, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct GetAllCollateralTypes {}
        impl GetAllCollateralTypes {
            const METHOD_ID: [u8; 4] = [41u8, 77u8, 73u8, 158u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Ok(Self {})
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(&[]);
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Vec<Vec<u8>>, String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<Vec<Vec<u8>>, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Address),
                            ),
                        ],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok(
                    values
                        .pop()
                        .expect("one output data should have existed")
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| {
                            inner.into_address().expect(INTERNAL_ERR).as_bytes().to_vec()
                        })
                        .collect(),
                )
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<Vec<Vec<u8>>> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for GetAllCollateralTypes {
            const NAME: &'static str = "getAllCollateralTypes";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<Vec<Vec<u8>>>
        for GetAllCollateralTypes {
            fn output(data: &[u8]) -> Result<Vec<Vec<u8>>, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct GetApproved {
            pub token_id: ethabi::Uint,
        }
        impl GetApproved {
            const METHOD_ID: [u8; 4] = [8u8, 24u8, 18u8, 252u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    token_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(&[ethabi::Token::Uint(self.token_id)]);
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Vec<u8>, String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<Vec<u8>, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Address],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok(
                    values
                        .pop()
                        .expect("one output data should have existed")
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                )
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<Vec<u8>> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for GetApproved {
            const NAME: &'static str = "getApproved";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<Vec<u8>> for GetApproved {
            fn output(data: &[u8]) -> Result<Vec<u8>, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct GetErc1155Category {
            pub erc1155_token_address: Vec<u8>,
            pub erc1155_type_id: ethabi::Uint,
        }
        impl GetErc1155Category {
            const METHOD_ID: [u8; 4] = [68u8, 105u8, 120u8, 240u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Address, ethabi::ParamType::Uint(256usize)],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    erc1155_token_address: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    erc1155_type_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Address(
                            ethabi::Address::from_slice(&self.erc1155_token_address),
                        ),
                        ethabi::Token::Uint(self.erc1155_type_id),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<ethabi::Uint, String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<ethabi::Uint, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok(
                    values
                        .pop()
                        .expect("one output data should have existed")
                        .into_uint()
                        .expect(INTERNAL_ERR),
                )
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<ethabi::Uint> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for GetErc1155Category {
            const NAME: &'static str = "getERC1155Category";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<ethabi::Uint>
        for GetErc1155Category {
            fn output(data: &[u8]) -> Result<ethabi::Uint, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct GetErc721Category {
            pub erc721_token_address: Vec<u8>,
            pub erc721_token_id: ethabi::Uint,
        }
        impl GetErc721Category {
            const METHOD_ID: [u8; 4] = [166u8, 118u8, 186u8, 251u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Address, ethabi::ParamType::Uint(256usize)],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    erc721_token_address: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    erc721_token_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Address(
                            ethabi::Address::from_slice(&self.erc721_token_address),
                        ),
                        ethabi::Token::Uint(self.erc721_token_id),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<ethabi::Uint, String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<ethabi::Uint, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok(
                    values
                        .pop()
                        .expect("one output data should have existed")
                        .into_uint()
                        .expect(INTERNAL_ERR),
                )
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<ethabi::Uint> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for GetErc721Category {
            const NAME: &'static str = "getERC721Category";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<ethabi::Uint> for GetErc721Category {
            fn output(data: &[u8]) -> Result<ethabi::Uint, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGotchiLendingIdByToken {
            pub erc721_token_id: ethabi::Uint,
        }
        impl GetGotchiLendingIdByToken {
            const METHOD_ID: [u8; 4] = [252u8, 2u8, 228u8, 238u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(32usize)],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    erc721_token_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(&[ethabi::Token::Uint(self.erc721_token_id)]);
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<ethabi::Uint, String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<ethabi::Uint, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(32usize)],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok(
                    values
                        .pop()
                        .expect("one output data should have existed")
                        .into_uint()
                        .expect(INTERNAL_ERR),
                )
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<ethabi::Uint> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for GetGotchiLendingIdByToken {
            const NAME: &'static str = "getGotchiLendingIdByToken";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<ethabi::Uint>
        for GetGotchiLendingIdByToken {
            fn output(data: &[u8]) -> Result<ethabi::Uint, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct GetItemSvg {
            pub item_id: ethabi::Uint,
        }
        impl GetItemSvg {
            const METHOD_ID: [u8; 4] = [23u8, 197u8, 13u8, 44u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    item_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(&[ethabi::Token::Uint(self.item_id)]);
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<String, String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<String, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::String],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok(
                    values
                        .pop()
                        .expect("one output data should have existed")
                        .into_string()
                        .expect(INTERNAL_ERR),
                )
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<String> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for GetItemSvg {
            const NAME: &'static str = "getItemSvg";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<String> for GetItemSvg {
            fn output(data: &[u8]) -> Result<String, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct GetItemSvgs {
            pub item_id: ethabi::Uint,
        }
        impl GetItemSvgs {
            const METHOD_ID: [u8; 4] = [253u8, 78u8, 33u8, 162u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    item_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(&[ethabi::Token::Uint(self.item_id)]);
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Vec<String>, String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<Vec<String>, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Array(Box::new(ethabi::ParamType::String))],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok(
                    values
                        .pop()
                        .expect("one output data should have existed")
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_string().expect(INTERNAL_ERR))
                        .collect(),
                )
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<Vec<String>> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for GetItemSvgs {
            const NAME: &'static str = "getItemSvgs";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<Vec<String>> for GetItemSvgs {
            fn output(data: &[u8]) -> Result<Vec<String>, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct GetListingFeeInWei {}
        impl GetListingFeeInWei {
            const METHOD_ID: [u8; 4] = [150u8, 32u8, 149u8, 161u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Ok(Self {})
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(&[]);
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<ethabi::Uint, String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<ethabi::Uint, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok(
                    values
                        .pop()
                        .expect("one output data should have existed")
                        .into_uint()
                        .expect(INTERNAL_ERR),
                )
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<ethabi::Uint> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for GetListingFeeInWei {
            const NAME: &'static str = "getListingFeeInWei";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<ethabi::Uint>
        for GetListingFeeInWei {
            fn output(data: &[u8]) -> Result<ethabi::Uint, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct GetNonce {
            pub user: Vec<u8>,
        }
        impl GetNonce {
            const METHOD_ID: [u8; 4] = [45u8, 3u8, 53u8, 171u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Address],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    user: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[ethabi::Token::Address(ethabi::Address::from_slice(&self.user))],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<ethabi::Uint, String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<ethabi::Uint, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok(
                    values
                        .pop()
                        .expect("one output data should have existed")
                        .into_uint()
                        .expect(INTERNAL_ERR),
                )
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<ethabi::Uint> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for GetNonce {
            const NAME: &'static str = "getNonce";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<ethabi::Uint> for GetNonce {
            fn output(data: &[u8]) -> Result<ethabi::Uint, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct GetNumericTraits {
            pub token_id: ethabi::Uint,
        }
        impl GetNumericTraits {
            const METHOD_ID: [u8; 4] = [250u8, 95u8, 82u8, 149u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    token_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(&[ethabi::Token::Uint(self.token_id)]);
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<[EthBigInt; 6usize], String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<[EthBigInt; 6usize], String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::FixedArray(
                                Box::new(ethabi::ParamType::Int(16usize)),
                                6usize,
                            ),
                        ],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok({
                    let mut iter = values
                        .pop()
                        .expect("one output data should have existed")
                        .into_fixed_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| {
                            let mut v = [0 as u8; 32];
                            inner
                                .into_int()
                                .expect(INTERNAL_ERR)
                                .to_big_endian(v.as_mut_slice());
                            EthBigInt::new(v.into())
                        });
                    [
                        iter.next().expect(INTERNAL_ERR),
                        iter.next().expect(INTERNAL_ERR),
                        iter.next().expect(INTERNAL_ERR),
                        iter.next().expect(INTERNAL_ERR),
                        iter.next().expect(INTERNAL_ERR),
                        iter.next().expect(INTERNAL_ERR),
                    ]
                })
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<[EthBigInt; 6usize]> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for GetNumericTraits {
            const NAME: &'static str = "getNumericTraits";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<[EthBigInt; 6usize]>
        for GetNumericTraits {
            fn output(data: &[u8]) -> Result<[EthBigInt; 6usize], String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct GetSvg {
            pub svg_type: [u8; 32usize],
            pub item_id: ethabi::Uint,
        }
        impl GetSvg {
            const METHOD_ID: [u8; 4] = [243u8, 0u8, 99u8, 61u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::FixedBytes(32usize),
                            ethabi::ParamType::Uint(256usize),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    svg_type: {
                        let mut result = [0u8; 32];
                        let v = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_bytes()
                            .expect(INTERNAL_ERR);
                        result.copy_from_slice(&v);
                        result
                    },
                    item_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::FixedBytes(self.svg_type.as_ref().to_vec()),
                        ethabi::Token::Uint(self.item_id),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<String, String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<String, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::String],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok(
                    values
                        .pop()
                        .expect("one output data should have existed")
                        .into_string()
                        .expect(INTERNAL_ERR),
                )
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<String> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for GetSvg {
            const NAME: &'static str = "getSvg";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<String> for GetSvg {
            fn output(data: &[u8]) -> Result<String, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct GetSvgs {
            pub svg_type: [u8; 32usize],
            pub item_ids: Vec<ethabi::Uint>,
        }
        impl GetSvgs {
            const METHOD_ID: [u8; 4] = [136u8, 37u8, 76u8, 124u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::FixedBytes(32usize),
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                            ),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    svg_type: {
                        let mut result = [0u8; 32];
                        let v = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_bytes()
                            .expect(INTERNAL_ERR);
                        result.copy_from_slice(&v);
                        result
                    },
                    item_ids: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::FixedBytes(self.svg_type.as_ref().to_vec()),
                        {
                            let v = self
                                .item_ids
                                .iter()
                                .map(|inner| ethabi::Token::Uint(inner.clone()))
                                .collect();
                            ethabi::Token::Array(v)
                        },
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Vec<String>, String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<Vec<String>, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Array(Box::new(ethabi::ParamType::String))],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok(
                    values
                        .pop()
                        .expect("one output data should have existed")
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_string().expect(INTERNAL_ERR))
                        .collect(),
                )
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<Vec<String>> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for GetSvgs {
            const NAME: &'static str = "getSvgs";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<Vec<String>> for GetSvgs {
            fn output(data: &[u8]) -> Result<Vec<String>, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct GhstAddress {}
        impl GhstAddress {
            const METHOD_ID: [u8; 4] = [167u8, 1u8, 53u8, 194u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Ok(Self {})
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(&[]);
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Vec<u8>, String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<Vec<u8>, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Address],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok(
                    values
                        .pop()
                        .expect("one output data should have existed")
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                )
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<Vec<u8>> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for GhstAddress {
            const NAME: &'static str = "ghstAddress";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<Vec<u8>> for GhstAddress {
            fn output(data: &[u8]) -> Result<Vec<u8>, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct GrantExperience {
            pub token_ids: Vec<ethabi::Uint>,
            pub xp_values: Vec<ethabi::Uint>,
        }
        impl GrantExperience {
            const METHOD_ID: [u8; 4] = [242u8, 65u8, 62u8, 244u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                            ),
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                            ),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    token_ids: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
                    xp_values: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        {
                            let v = self
                                .token_ids
                                .iter()
                                .map(|inner| ethabi::Token::Uint(inner.clone()))
                                .collect();
                            ethabi::Token::Array(v)
                        },
                        {
                            let v = self
                                .xp_values
                                .iter()
                                .map(|inner| ethabi::Token::Uint(inner.clone()))
                                .collect();
                            ethabi::Token::Array(v)
                        },
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for GrantExperience {
            const NAME: &'static str = "grantExperience";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct IncreaseStake {
            pub token_id: ethabi::Uint,
            pub stake_amount: ethabi::Uint,
        }
        impl IncreaseStake {
            const METHOD_ID: [u8; 4] = [190u8, 193u8, 12u8, 222u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    token_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    stake_amount: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Uint(self.token_id),
                        ethabi::Token::Uint(self.stake_amount),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for IncreaseStake {
            const NAME: &'static str = "increaseStake";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct Interact {
            pub token_ids: Vec<ethabi::Uint>,
        }
        impl Interact {
            const METHOD_ID: [u8; 4] = [34u8, 198u8, 117u8, 25u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                            ),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    token_ids: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        {
                            let v = self
                                .token_ids
                                .iter()
                                .map(|inner| ethabi::Token::Uint(inner.clone()))
                                .collect();
                            ethabi::Token::Array(v)
                        },
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for Interact {
            const NAME: &'static str = "interact";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct IsAavegotchiLent {
            pub erc721_token_id: ethabi::Uint,
        }
        impl IsAavegotchiLent {
            const METHOD_ID: [u8; 4] = [215u8, 49u8, 212u8, 78u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(32usize)],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    erc721_token_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(&[ethabi::Token::Uint(self.erc721_token_id)]);
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<bool, String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<bool, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Bool],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok(
                    values
                        .pop()
                        .expect("one output data should have existed")
                        .into_bool()
                        .expect(INTERNAL_ERR),
                )
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<bool> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for IsAavegotchiLent {
            const NAME: &'static str = "isAavegotchiLent";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<bool> for IsAavegotchiLent {
            fn output(data: &[u8]) -> Result<bool, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct IsApprovedForAll {
            pub owner: Vec<u8>,
            pub operator: Vec<u8>,
        }
        impl IsApprovedForAll {
            const METHOD_ID: [u8; 4] = [233u8, 133u8, 233u8, 197u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Address, ethabi::ParamType::Address],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    owner: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    operator: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Address(ethabi::Address::from_slice(&self.owner)),
                        ethabi::Token::Address(
                            ethabi::Address::from_slice(&self.operator),
                        ),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<bool, String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<bool, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Bool],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok(
                    values
                        .pop()
                        .expect("one output data should have existed")
                        .into_bool()
                        .expect(INTERNAL_ERR),
                )
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<bool> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for IsApprovedForAll {
            const NAME: &'static str = "isApprovedForAll";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<bool> for IsApprovedForAll {
            fn output(data: &[u8]) -> Result<bool, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct IsGameManager {
            pub manager: Vec<u8>,
        }
        impl IsGameManager {
            const METHOD_ID: [u8; 4] = [224u8, 69u8, 253u8, 4u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Address],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    manager: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[ethabi::Token::Address(ethabi::Address::from_slice(&self.manager))],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<bool, String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<bool, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Bool],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok(
                    values
                        .pop()
                        .expect("one output data should have existed")
                        .into_bool()
                        .expect(INTERNAL_ERR),
                )
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<bool> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for IsGameManager {
            const NAME: &'static str = "isGameManager";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<bool> for IsGameManager {
            fn output(data: &[u8]) -> Result<bool, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct IsPetOperatorForAll {
            pub owner: Vec<u8>,
            pub operator: Vec<u8>,
        }
        impl IsPetOperatorForAll {
            const METHOD_ID: [u8; 4] = [215u8, 53u8, 143u8, 234u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Address, ethabi::ParamType::Address],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    owner: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    operator: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Address(ethabi::Address::from_slice(&self.owner)),
                        ethabi::Token::Address(
                            ethabi::Address::from_slice(&self.operator),
                        ),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<bool, String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<bool, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Bool],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok(
                    values
                        .pop()
                        .expect("one output data should have existed")
                        .into_bool()
                        .expect(INTERNAL_ERR),
                )
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<bool> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for IsPetOperatorForAll {
            const NAME: &'static str = "isPetOperatorForAll";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<bool> for IsPetOperatorForAll {
            fn output(data: &[u8]) -> Result<bool, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct KeyHash {}
        impl KeyHash {
            const METHOD_ID: [u8; 4] = [97u8, 114u8, 143u8, 57u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Ok(Self {})
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(&[]);
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<[u8; 32usize], String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<[u8; 32usize], String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::FixedBytes(32usize)],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok({
                    let mut result = [0u8; 32];
                    let v = values
                        .pop()
                        .expect("one output data should have existed")
                        .into_fixed_bytes()
                        .expect(INTERNAL_ERR);
                    result.copy_from_slice(&v);
                    result
                })
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<[u8; 32usize]> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for KeyHash {
            const NAME: &'static str = "keyHash";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<[u8; 32usize]> for KeyHash {
            fn output(data: &[u8]) -> Result<[u8; 32usize], String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct Kinship {
            pub token_id: ethabi::Uint,
        }
        impl Kinship {
            const METHOD_ID: [u8; 4] = [245u8, 185u8, 24u8, 82u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    token_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(&[ethabi::Token::Uint(self.token_id)]);
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<ethabi::Uint, String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<ethabi::Uint, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok(
                    values
                        .pop()
                        .expect("one output data should have existed")
                        .into_uint()
                        .expect(INTERNAL_ERR),
                )
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<ethabi::Uint> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for Kinship {
            const NAME: &'static str = "kinship";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<ethabi::Uint> for Kinship {
            fn output(data: &[u8]) -> Result<ethabi::Uint, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct Link {}
        impl Link {
            const METHOD_ID: [u8; 4] = [28u8, 70u8, 149u8, 244u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Ok(Self {})
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(&[]);
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Vec<u8>, String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<Vec<u8>, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Address],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok(
                    values
                        .pop()
                        .expect("one output data should have existed")
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                )
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<Vec<u8>> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for Link {
            const NAME: &'static str = "link";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<Vec<u8>> for Link {
            fn output(data: &[u8]) -> Result<Vec<u8>, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct LinkBalance {}
        impl LinkBalance {
            const METHOD_ID: [u8; 4] = [194u8, 110u8, 206u8, 250u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Ok(Self {})
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(&[]);
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<ethabi::Uint, String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<ethabi::Uint, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok(
                    values
                        .pop()
                        .expect("one output data should have existed")
                        .into_uint()
                        .expect(INTERNAL_ERR),
                )
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<ethabi::Uint> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for LinkBalance {
            const NAME: &'static str = "linkBalance";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<ethabi::Uint> for LinkBalance {
            fn output(data: &[u8]) -> Result<ethabi::Uint, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct MintItems {
            pub to: Vec<u8>,
            pub item_ids: Vec<ethabi::Uint>,
            pub quantities: Vec<ethabi::Uint>,
        }
        impl MintItems {
            const METHOD_ID: [u8; 4] = [141u8, 133u8, 201u8, 111u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                            ),
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                            ),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    to: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    item_ids: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
                    quantities: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Address(ethabi::Address::from_slice(&self.to)),
                        {
                            let v = self
                                .item_ids
                                .iter()
                                .map(|inner| ethabi::Token::Uint(inner.clone()))
                                .collect();
                            ethabi::Token::Array(v)
                        },
                        {
                            let v = self
                                .quantities
                                .iter()
                                .map(|inner| ethabi::Token::Uint(inner.clone()))
                                .collect();
                            ethabi::Token::Array(v)
                        },
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for MintItems {
            const NAME: &'static str = "mintItems";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct MintPortals {
            pub to: Vec<u8>,
            pub amount: ethabi::Uint,
        }
        impl MintPortals {
            const METHOD_ID: [u8; 4] = [67u8, 134u8, 111u8, 24u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Address, ethabi::ParamType::Uint(256usize)],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    to: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    amount: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Address(ethabi::Address::from_slice(&self.to)),
                        ethabi::Token::Uint(self.amount),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for MintPortals {
            const NAME: &'static str = "mintPortals";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct ModifiedTraitsAndRarityScore {
            pub token_id: ethabi::Uint,
        }
        impl ModifiedTraitsAndRarityScore {
            const METHOD_ID: [u8; 4] = [255u8, 254u8, 187u8, 65u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    token_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(&[ethabi::Token::Uint(self.token_id)]);
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<([EthBigInt; 6usize], ethabi::Uint), String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(
                data: &[u8],
            ) -> Result<([EthBigInt; 6usize], ethabi::Uint), String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::FixedArray(
                                Box::new(ethabi::ParamType::Int(16usize)),
                                6usize,
                            ),
                            ethabi::ParamType::Uint(256usize),
                        ],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                values.reverse();
                Ok((
                    {
                        let mut iter = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_array()
                            .expect(INTERNAL_ERR)
                            .into_iter()
                            .map(|inner| {
                                let mut v = [0 as u8; 32];
                                inner
                                    .into_int()
                                    .expect(INTERNAL_ERR)
                                    .to_big_endian(v.as_mut_slice());
                                EthBigInt::new(v.into())
                            });
                        [
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                        ]
                    },
                    values.pop().expect(INTERNAL_ERR).into_uint().expect(INTERNAL_ERR),
                ))
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(
                &self,
                address: Vec<u8>,
            ) -> Option<([EthBigInt; 6usize], ethabi::Uint)> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for ModifiedTraitsAndRarityScore {
            const NAME: &'static str = "modifiedTraitsAndRarityScore";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<([EthBigInt; 6usize], ethabi::Uint)>
        for ModifiedTraitsAndRarityScore {
            fn output(
                data: &[u8],
            ) -> Result<([EthBigInt; 6usize], ethabi::Uint), String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct Name {}
        impl Name {
            const METHOD_ID: [u8; 4] = [6u8, 253u8, 222u8, 3u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Ok(Self {})
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(&[]);
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<String, String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<String, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::String],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok(
                    values
                        .pop()
                        .expect("one output data should have existed")
                        .into_string()
                        .expect(INTERNAL_ERR),
                )
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<String> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for Name {
            const NAME: &'static str = "name";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<String> for Name {
            fn output(data: &[u8]) -> Result<String, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct OnErc1155BatchReceived {
            pub param0: Vec<u8>,
            pub param1: Vec<u8>,
            pub param2: Vec<ethabi::Uint>,
            pub param3: Vec<ethabi::Uint>,
            pub param4: Vec<u8>,
        }
        impl OnErc1155BatchReceived {
            const METHOD_ID: [u8; 4] = [188u8, 25u8, 124u8, 129u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                            ),
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                            ),
                            ethabi::ParamType::Bytes,
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    param0: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    param1: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    param2: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
                    param3: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
                    param4: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_bytes()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Address(
                            ethabi::Address::from_slice(&self.param0),
                        ),
                        ethabi::Token::Address(
                            ethabi::Address::from_slice(&self.param1),
                        ),
                        {
                            let v = self
                                .param2
                                .iter()
                                .map(|inner| ethabi::Token::Uint(inner.clone()))
                                .collect();
                            ethabi::Token::Array(v)
                        },
                        {
                            let v = self
                                .param3
                                .iter()
                                .map(|inner| ethabi::Token::Uint(inner.clone()))
                                .collect();
                            ethabi::Token::Array(v)
                        },
                        ethabi::Token::Bytes(self.param4.clone()),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<[u8; 4usize], String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<[u8; 4usize], String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::FixedBytes(4usize)],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok({
                    let mut result = [0u8; 4];
                    let v = values
                        .pop()
                        .expect("one output data should have existed")
                        .into_fixed_bytes()
                        .expect(INTERNAL_ERR);
                    result.copy_from_slice(&v);
                    result
                })
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<[u8; 4usize]> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for OnErc1155BatchReceived {
            const NAME: &'static str = "onERC1155BatchReceived";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<[u8; 4usize]>
        for OnErc1155BatchReceived {
            fn output(data: &[u8]) -> Result<[u8; 4usize], String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct OnErc1155Received {
            pub param0: Vec<u8>,
            pub param1: Vec<u8>,
            pub param2: ethabi::Uint,
            pub param3: ethabi::Uint,
            pub param4: Vec<u8>,
        }
        impl OnErc1155Received {
            const METHOD_ID: [u8; 4] = [242u8, 58u8, 110u8, 97u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Bytes,
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    param0: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    param1: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    param2: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    param3: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    param4: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_bytes()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Address(
                            ethabi::Address::from_slice(&self.param0),
                        ),
                        ethabi::Token::Address(
                            ethabi::Address::from_slice(&self.param1),
                        ),
                        ethabi::Token::Uint(self.param2),
                        ethabi::Token::Uint(self.param3),
                        ethabi::Token::Bytes(self.param4.clone()),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<[u8; 4usize], String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<[u8; 4usize], String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::FixedBytes(4usize)],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok({
                    let mut result = [0u8; 4];
                    let v = values
                        .pop()
                        .expect("one output data should have existed")
                        .into_fixed_bytes()
                        .expect(INTERNAL_ERR);
                    result.copy_from_slice(&v);
                    result
                })
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<[u8; 4usize]> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for OnErc1155Received {
            const NAME: &'static str = "onERC1155Received";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<[u8; 4usize]> for OnErc1155Received {
            fn output(data: &[u8]) -> Result<[u8; 4usize], String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct OpenPortals {
            pub token_ids: Vec<ethabi::Uint>,
        }
        impl OpenPortals {
            const METHOD_ID: [u8; 4] = [36u8, 123u8, 48u8, 251u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                            ),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    token_ids: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        {
                            let v = self
                                .token_ids
                                .iter()
                                .map(|inner| ethabi::Token::Uint(inner.clone()))
                                .collect();
                            ethabi::Token::Array(v)
                        },
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for OpenPortals {
            const NAME: &'static str = "openPortals";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct OwnerOf {
            pub token_id: ethabi::Uint,
        }
        impl OwnerOf {
            const METHOD_ID: [u8; 4] = [99u8, 82u8, 33u8, 30u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    token_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(&[ethabi::Token::Uint(self.token_id)]);
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Vec<u8>, String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<Vec<u8>, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Address],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok(
                    values
                        .pop()
                        .expect("one output data should have existed")
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                )
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<Vec<u8>> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for OwnerOf {
            const NAME: &'static str = "ownerOf";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<Vec<u8>> for OwnerOf {
            fn output(data: &[u8]) -> Result<Vec<u8>, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct PortalAavegotchisSvg {
            pub token_id: ethabi::Uint,
        }
        impl PortalAavegotchisSvg {
            const METHOD_ID: [u8; 4] = [106u8, 100u8, 111u8, 52u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    token_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(&[ethabi::Token::Uint(self.token_id)]);
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<[String; 10usize], String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<[String; 10usize], String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::FixedArray(
                                Box::new(ethabi::ParamType::String),
                                10usize,
                            ),
                        ],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok({
                    let mut iter = values
                        .pop()
                        .expect("one output data should have existed")
                        .into_fixed_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_string().expect(INTERNAL_ERR));
                    [
                        iter.next().expect(INTERNAL_ERR),
                        iter.next().expect(INTERNAL_ERR),
                        iter.next().expect(INTERNAL_ERR),
                        iter.next().expect(INTERNAL_ERR),
                        iter.next().expect(INTERNAL_ERR),
                        iter.next().expect(INTERNAL_ERR),
                        iter.next().expect(INTERNAL_ERR),
                        iter.next().expect(INTERNAL_ERR),
                        iter.next().expect(INTERNAL_ERR),
                        iter.next().expect(INTERNAL_ERR),
                    ]
                })
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<[String; 10usize]> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for PortalAavegotchisSvg {
            const NAME: &'static str = "portalAavegotchisSvg";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<[String; 10usize]>
        for PortalAavegotchisSvg {
            fn output(data: &[u8]) -> Result<[String; 10usize], String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct PreviewAavegotchi {
            pub haunt_id: ethabi::Uint,
            pub collateral_type: Vec<u8>,
            pub numeric_traits: [EthBigInt; 6usize],
            pub equipped_wearables: [ethabi::Uint; 16usize],
        }
        impl PreviewAavegotchi {
            const METHOD_ID: [u8; 4] = [41u8, 80u8, 65u8, 240u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Address,
                            ethabi::ParamType::FixedArray(
                                Box::new(ethabi::ParamType::Int(16usize)),
                                6usize,
                            ),
                            ethabi::ParamType::FixedArray(
                                Box::new(ethabi::ParamType::Uint(16usize)),
                                16usize,
                            ),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    haunt_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    collateral_type: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    numeric_traits: {
                        let mut iter = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_array()
                            .expect(INTERNAL_ERR)
                            .into_iter()
                            .map(|inner| {
                                let mut v = [0 as u8; 32];
                                inner
                                    .into_int()
                                    .expect(INTERNAL_ERR)
                                    .to_big_endian(v.as_mut_slice());
                                EthBigInt::new(v.into())
                            });
                        [
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                        ]
                    },
                    equipped_wearables: {
                        let mut iter = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_array()
                            .expect(INTERNAL_ERR)
                            .into_iter()
                            .map(|inner| inner.into_uint().expect(INTERNAL_ERR));
                        [
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                        ]
                    },
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Uint(self.haunt_id),
                        ethabi::Token::Address(
                            ethabi::Address::from_slice(&self.collateral_type),
                        ),
                        {
                            let v = self
                                .numeric_traits
                                .iter()
                                .map(|inner| {
                                    let non_full_signed_bytes = inner
                                        .get_big_int()
                                        .to_signed_bytes_be();
                                    let mut full_signed_bytes = [0xff as u8; 32];
                                    non_full_signed_bytes
                                        .into_iter()
                                        .rev()
                                        .enumerate()
                                        .for_each(|(i, byte)| full_signed_bytes[31 - i] = byte);
                                    ethabi::Token::Int(
                                        ethabi::Int::from_big_endian(full_signed_bytes.as_ref()),
                                    )
                                })
                                .collect();
                            ethabi::Token::FixedArray(v)
                        },
                        {
                            let v = self
                                .equipped_wearables
                                .iter()
                                .map(|inner| ethabi::Token::Uint(inner.clone()))
                                .collect();
                            ethabi::Token::FixedArray(v)
                        },
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<String, String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<String, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::String],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok(
                    values
                        .pop()
                        .expect("one output data should have existed")
                        .into_string()
                        .expect(INTERNAL_ERR),
                )
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<String> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for PreviewAavegotchi {
            const NAME: &'static str = "previewAavegotchi";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<String> for PreviewAavegotchi {
            fn output(data: &[u8]) -> Result<String, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct PreviewSideAavegotchi {
            pub haunt_id: ethabi::Uint,
            pub collateral_type: Vec<u8>,
            pub numeric_traits: [EthBigInt; 6usize],
            pub equipped_wearables: [ethabi::Uint; 16usize],
        }
        impl PreviewSideAavegotchi {
            const METHOD_ID: [u8; 4] = [116u8, 117u8, 13u8, 24u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Address,
                            ethabi::ParamType::FixedArray(
                                Box::new(ethabi::ParamType::Int(16usize)),
                                6usize,
                            ),
                            ethabi::ParamType::FixedArray(
                                Box::new(ethabi::ParamType::Uint(16usize)),
                                16usize,
                            ),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    haunt_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    collateral_type: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    numeric_traits: {
                        let mut iter = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_array()
                            .expect(INTERNAL_ERR)
                            .into_iter()
                            .map(|inner| {
                                let mut v = [0 as u8; 32];
                                inner
                                    .into_int()
                                    .expect(INTERNAL_ERR)
                                    .to_big_endian(v.as_mut_slice());
                                EthBigInt::new(v.into())
                            });
                        [
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                        ]
                    },
                    equipped_wearables: {
                        let mut iter = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_array()
                            .expect(INTERNAL_ERR)
                            .into_iter()
                            .map(|inner| inner.into_uint().expect(INTERNAL_ERR));
                        [
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                        ]
                    },
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Uint(self.haunt_id),
                        ethabi::Token::Address(
                            ethabi::Address::from_slice(&self.collateral_type),
                        ),
                        {
                            let v = self
                                .numeric_traits
                                .iter()
                                .map(|inner| {
                                    let non_full_signed_bytes = inner
                                        .get_big_int()
                                        .to_signed_bytes_be();
                                    let mut full_signed_bytes = [0xff as u8; 32];
                                    non_full_signed_bytes
                                        .into_iter()
                                        .rev()
                                        .enumerate()
                                        .for_each(|(i, byte)| full_signed_bytes[31 - i] = byte);
                                    ethabi::Token::Int(
                                        ethabi::Int::from_big_endian(full_signed_bytes.as_ref()),
                                    )
                                })
                                .collect();
                            ethabi::Token::FixedArray(v)
                        },
                        {
                            let v = self
                                .equipped_wearables
                                .iter()
                                .map(|inner| ethabi::Token::Uint(inner.clone()))
                                .collect();
                            ethabi::Token::FixedArray(v)
                        },
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Vec<String>, String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<Vec<String>, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Array(Box::new(ethabi::ParamType::String))],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok(
                    values
                        .pop()
                        .expect("one output data should have existed")
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_string().expect(INTERNAL_ERR))
                        .collect(),
                )
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<Vec<String>> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for PreviewSideAavegotchi {
            const NAME: &'static str = "previewSideAavegotchi";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<Vec<String>>
        for PreviewSideAavegotchi {
            fn output(data: &[u8]) -> Result<Vec<String>, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct PurchaseItemsWithGhst {
            pub to: Vec<u8>,
            pub item_ids: Vec<ethabi::Uint>,
            pub quantities: Vec<ethabi::Uint>,
        }
        impl PurchaseItemsWithGhst {
            const METHOD_ID: [u8; 4] = [235u8, 220u8, 59u8, 88u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                            ),
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                            ),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    to: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    item_ids: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
                    quantities: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Address(ethabi::Address::from_slice(&self.to)),
                        {
                            let v = self
                                .item_ids
                                .iter()
                                .map(|inner| ethabi::Token::Uint(inner.clone()))
                                .collect();
                            ethabi::Token::Array(v)
                        },
                        {
                            let v = self
                                .quantities
                                .iter()
                                .map(|inner| ethabi::Token::Uint(inner.clone()))
                                .collect();
                            ethabi::Token::Array(v)
                        },
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for PurchaseItemsWithGhst {
            const NAME: &'static str = "purchaseItemsWithGhst";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct PurchaseTransferItemsWithGhst {
            pub to: Vec<u8>,
            pub item_ids: Vec<ethabi::Uint>,
            pub quantities: Vec<ethabi::Uint>,
        }
        impl PurchaseTransferItemsWithGhst {
            const METHOD_ID: [u8; 4] = [67u8, 39u8, 136u8, 236u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                            ),
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                            ),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    to: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    item_ids: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
                    quantities: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Address(ethabi::Address::from_slice(&self.to)),
                        {
                            let v = self
                                .item_ids
                                .iter()
                                .map(|inner| ethabi::Token::Uint(inner.clone()))
                                .collect();
                            ethabi::Token::Array(v)
                        },
                        {
                            let v = self
                                .quantities
                                .iter()
                                .map(|inner| ethabi::Token::Uint(inner.clone()))
                                .collect();
                            ethabi::Token::Array(v)
                        },
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for PurchaseTransferItemsWithGhst {
            const NAME: &'static str = "purchaseTransferItemsWithGhst";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct RarityMultiplier {
            pub numeric_traits: [EthBigInt; 6usize],
        }
        impl RarityMultiplier {
            const METHOD_ID: [u8; 4] = [9u8, 203u8, 175u8, 6u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::FixedArray(
                                Box::new(ethabi::ParamType::Int(16usize)),
                                6usize,
                            ),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    numeric_traits: {
                        let mut iter = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_array()
                            .expect(INTERNAL_ERR)
                            .into_iter()
                            .map(|inner| {
                                let mut v = [0 as u8; 32];
                                inner
                                    .into_int()
                                    .expect(INTERNAL_ERR)
                                    .to_big_endian(v.as_mut_slice());
                                EthBigInt::new(v.into())
                            });
                        [
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                        ]
                    },
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        {
                            let v = self
                                .numeric_traits
                                .iter()
                                .map(|inner| {
                                    let non_full_signed_bytes = inner
                                        .get_big_int()
                                        .to_signed_bytes_be();
                                    let mut full_signed_bytes = [0xff as u8; 32];
                                    non_full_signed_bytes
                                        .into_iter()
                                        .rev()
                                        .enumerate()
                                        .for_each(|(i, byte)| full_signed_bytes[31 - i] = byte);
                                    ethabi::Token::Int(
                                        ethabi::Int::from_big_endian(full_signed_bytes.as_ref()),
                                    )
                                })
                                .collect();
                            ethabi::Token::FixedArray(v)
                        },
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<ethabi::Uint, String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<ethabi::Uint, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok(
                    values
                        .pop()
                        .expect("one output data should have existed")
                        .into_uint()
                        .expect(INTERNAL_ERR),
                )
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<ethabi::Uint> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for RarityMultiplier {
            const NAME: &'static str = "rarityMultiplier";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<ethabi::Uint> for RarityMultiplier {
            fn output(data: &[u8]) -> Result<ethabi::Uint, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct RawFulfillRandomness {
            pub request_id: [u8; 32usize],
            pub random_number: ethabi::Uint,
        }
        impl RawFulfillRandomness {
            const METHOD_ID: [u8; 4] = [148u8, 152u8, 93u8, 221u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::FixedBytes(32usize),
                            ethabi::ParamType::Uint(256usize),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    request_id: {
                        let mut result = [0u8; 32];
                        let v = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_bytes()
                            .expect(INTERNAL_ERR);
                        result.copy_from_slice(&v);
                        result
                    },
                    random_number: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::FixedBytes(self.request_id.as_ref().to_vec()),
                        ethabi::Token::Uint(self.random_number),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for RawFulfillRandomness {
            const NAME: &'static str = "rawFulfillRandomness";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct RemoveExperience {
            pub token_ids: Vec<ethabi::Uint>,
            pub xp_values: Vec<ethabi::Uint>,
        }
        impl RemoveExperience {
            const METHOD_ID: [u8; 4] = [80u8, 131u8, 53u8, 165u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                            ),
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                            ),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    token_ids: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
                    xp_values: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        {
                            let v = self
                                .token_ids
                                .iter()
                                .map(|inner| ethabi::Token::Uint(inner.clone()))
                                .collect();
                            ethabi::Token::Array(v)
                        },
                        {
                            let v = self
                                .xp_values
                                .iter()
                                .map(|inner| ethabi::Token::Uint(inner.clone()))
                                .collect();
                            ethabi::Token::Array(v)
                        },
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for RemoveExperience {
            const NAME: &'static str = "removeExperience";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct RemoveGameManagers {
            pub game_managers: Vec<Vec<u8>>,
        }
        impl RemoveGameManagers {
            const METHOD_ID: [u8; 4] = [31u8, 61u8, 132u8, 47u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Address),
                            ),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    game_managers: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| {
                            inner.into_address().expect(INTERNAL_ERR).as_bytes().to_vec()
                        })
                        .collect(),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        {
                            let v = self
                                .game_managers
                                .iter()
                                .map(|inner| ethabi::Token::Address(
                                    ethabi::Address::from_slice(&inner),
                                ))
                                .collect();
                            ethabi::Token::Array(v)
                        },
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for RemoveGameManagers {
            const NAME: &'static str = "removeGameManagers";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct RemoveItemManagers {
            pub item_managers: Vec<Vec<u8>>,
        }
        impl RemoveItemManagers {
            const METHOD_ID: [u8; 4] = [104u8, 38u8, 164u8, 21u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Address),
                            ),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    item_managers: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| {
                            inner.into_address().expect(INTERNAL_ERR).as_bytes().to_vec()
                        })
                        .collect(),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        {
                            let v = self
                                .item_managers
                                .iter()
                                .map(|inner| ethabi::Token::Address(
                                    ethabi::Address::from_slice(&inner),
                                ))
                                .collect();
                            ethabi::Token::Array(v)
                        },
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for RemoveItemManagers {
            const NAME: &'static str = "removeItemManagers";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct RemoveLinkTokens {
            pub to: Vec<u8>,
            pub value: ethabi::Uint,
        }
        impl RemoveLinkTokens {
            const METHOD_ID: [u8; 4] = [118u8, 30u8, 153u8, 240u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Address, ethabi::ParamType::Uint(256usize)],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    to: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    value: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Address(ethabi::Address::from_slice(&self.to)),
                        ethabi::Token::Uint(self.value),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for RemoveLinkTokens {
            const NAME: &'static str = "removeLinkTokens";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct SafeBatchTransferFrom1 {
            pub from: Vec<u8>,
            pub to: Vec<u8>,
            pub token_ids: Vec<ethabi::Uint>,
            pub data: Vec<u8>,
        }
        impl SafeBatchTransferFrom1 {
            const METHOD_ID: [u8; 4] = [40u8, 207u8, 189u8, 70u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                            ),
                            ethabi::ParamType::Bytes,
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    from: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    to: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    token_ids: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
                    data: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_bytes()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Address(ethabi::Address::from_slice(&self.from)),
                        ethabi::Token::Address(ethabi::Address::from_slice(&self.to)),
                        {
                            let v = self
                                .token_ids
                                .iter()
                                .map(|inner| ethabi::Token::Uint(inner.clone()))
                                .collect();
                            ethabi::Token::Array(v)
                        },
                        ethabi::Token::Bytes(self.data.clone()),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for SafeBatchTransferFrom1 {
            const NAME: &'static str = "safeBatchTransferFrom1";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct SafeBatchTransferFrom2 {
            pub from: Vec<u8>,
            pub to: Vec<u8>,
            pub ids: Vec<ethabi::Uint>,
            pub values: Vec<ethabi::Uint>,
            pub data: Vec<u8>,
        }
        impl SafeBatchTransferFrom2 {
            const METHOD_ID: [u8; 4] = [46u8, 178u8, 194u8, 214u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                            ),
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                            ),
                            ethabi::ParamType::Bytes,
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    from: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    to: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    ids: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
                    values: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
                    data: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_bytes()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Address(ethabi::Address::from_slice(&self.from)),
                        ethabi::Token::Address(ethabi::Address::from_slice(&self.to)),
                        {
                            let v = self
                                .ids
                                .iter()
                                .map(|inner| ethabi::Token::Uint(inner.clone()))
                                .collect();
                            ethabi::Token::Array(v)
                        },
                        {
                            let v = self
                                .values
                                .iter()
                                .map(|inner| ethabi::Token::Uint(inner.clone()))
                                .collect();
                            ethabi::Token::Array(v)
                        },
                        ethabi::Token::Bytes(self.data.clone()),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for SafeBatchTransferFrom2 {
            const NAME: &'static str = "safeBatchTransferFrom2";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct SafeTransferFrom1 {
            pub from: Vec<u8>,
            pub to: Vec<u8>,
            pub token_id: ethabi::Uint,
        }
        impl SafeTransferFrom1 {
            const METHOD_ID: [u8; 4] = [66u8, 132u8, 46u8, 14u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Uint(256usize),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    from: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    to: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    token_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Address(ethabi::Address::from_slice(&self.from)),
                        ethabi::Token::Address(ethabi::Address::from_slice(&self.to)),
                        ethabi::Token::Uint(self.token_id),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for SafeTransferFrom1 {
            const NAME: &'static str = "safeTransferFrom1";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct SafeTransferFrom2 {
            pub from: Vec<u8>,
            pub to: Vec<u8>,
            pub token_id: ethabi::Uint,
            pub data: Vec<u8>,
        }
        impl SafeTransferFrom2 {
            const METHOD_ID: [u8; 4] = [184u8, 141u8, 79u8, 222u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Bytes,
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    from: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    to: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    token_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    data: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_bytes()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Address(ethabi::Address::from_slice(&self.from)),
                        ethabi::Token::Address(ethabi::Address::from_slice(&self.to)),
                        ethabi::Token::Uint(self.token_id),
                        ethabi::Token::Bytes(self.data.clone()),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for SafeTransferFrom2 {
            const NAME: &'static str = "safeTransferFrom2";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct SafeTransferFrom3 {
            pub from: Vec<u8>,
            pub to: Vec<u8>,
            pub id: ethabi::Uint,
            pub value: ethabi::Uint,
            pub data: Vec<u8>,
        }
        impl SafeTransferFrom3 {
            const METHOD_ID: [u8; 4] = [242u8, 66u8, 67u8, 42u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Bytes,
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    from: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    to: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    value: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    data: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_bytes()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Address(ethabi::Address::from_slice(&self.from)),
                        ethabi::Token::Address(ethabi::Address::from_slice(&self.to)),
                        ethabi::Token::Uint(self.id),
                        ethabi::Token::Uint(self.value),
                        ethabi::Token::Bytes(self.data.clone()),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for SafeTransferFrom3 {
            const NAME: &'static str = "safeTransferFrom3";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct SetAavegotchiName {
            pub token_id: ethabi::Uint,
            pub name: String,
        }
        impl SetAavegotchiName {
            const METHOD_ID: [u8; 4] = [234u8, 125u8, 172u8, 127u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize), ethabi::ParamType::String],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    token_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    name: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_string()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Uint(self.token_id),
                        ethabi::Token::String(self.name.clone()),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for SetAavegotchiName {
            const NAME: &'static str = "setAavegotchiName";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct SetApprovalForAll {
            pub operator: Vec<u8>,
            pub approved: bool,
        }
        impl SetApprovalForAll {
            const METHOD_ID: [u8; 4] = [162u8, 44u8, 180u8, 101u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Address, ethabi::ParamType::Bool],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    operator: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    approved: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_bool()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Address(
                            ethabi::Address::from_slice(&self.operator),
                        ),
                        ethabi::Token::Bool(self.approved),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for SetApprovalForAll {
            const NAME: &'static str = "setApprovalForAll";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct SetBaseUri {
            pub value: String,
        }
        impl SetBaseUri {
            const METHOD_ID: [u8; 4] = [85u8, 248u8, 4u8, 179u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::String],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    value: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_string()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(&[ethabi::Token::String(self.value.clone())]);
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for SetBaseUri {
            const NAME: &'static str = "setBaseURI";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct SetChildChainManager {
            pub new_child_chain_manager: Vec<u8>,
        }
        impl SetChildChainManager {
            const METHOD_ID: [u8; 4] = [118u8, 74u8, 129u8, 187u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Address],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    new_child_chain_manager: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Address(
                            ethabi::Address::from_slice(&self.new_child_chain_manager),
                        ),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for SetChildChainManager {
            const NAME: &'static str = "setChildChainManager";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct SetCollateralEyeShapeSvgId {
            pub collateral_token: Vec<u8>,
            pub svg_id: ethabi::Uint,
        }
        impl SetCollateralEyeShapeSvgId {
            const METHOD_ID: [u8; 4] = [163u8, 234u8, 0u8, 241u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Address, ethabi::ParamType::Uint(8usize)],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    collateral_token: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    svg_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Address(
                            ethabi::Address::from_slice(&self.collateral_token),
                        ),
                        ethabi::Token::Uint(self.svg_id),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for SetCollateralEyeShapeSvgId {
            const NAME: &'static str = "setCollateralEyeShapeSvgId";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct SetDao {
            pub new_dao: Vec<u8>,
            pub new_dao_treasury: Vec<u8>,
        }
        impl SetDao {
            const METHOD_ID: [u8; 4] = [119u8, 60u8, 23u8, 170u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Address, ethabi::ParamType::Address],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    new_dao: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    new_dao_treasury: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Address(
                            ethabi::Address::from_slice(&self.new_dao),
                        ),
                        ethabi::Token::Address(
                            ethabi::Address::from_slice(&self.new_dao_treasury),
                        ),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for SetDao {
            const NAME: &'static str = "setDao";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct SetErc1155Listing {
            pub erc1155_token_address: Vec<u8>,
            pub erc1155_type_id: ethabi::Uint,
            pub quantity: ethabi::Uint,
            pub price_in_wei: ethabi::Uint,
        }
        impl SetErc1155Listing {
            const METHOD_ID: [u8; 4] = [232u8, 218u8, 43u8, 250u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    erc1155_token_address: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    erc1155_type_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    quantity: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    price_in_wei: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Address(
                            ethabi::Address::from_slice(&self.erc1155_token_address),
                        ),
                        ethabi::Token::Uint(self.erc1155_type_id),
                        ethabi::Token::Uint(self.quantity),
                        ethabi::Token::Uint(self.price_in_wei),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for SetErc1155Listing {
            const NAME: &'static str = "setERC1155Listing";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct SetItemTraitModifiersAndRarityModifier {
            pub wearable_id: ethabi::Uint,
            pub trait_modifiers: [EthBigInt; 6usize],
            pub rarity_score_modifier: ethabi::Uint,
        }
        impl SetItemTraitModifiersAndRarityModifier {
            const METHOD_ID: [u8; 4] = [220u8, 47u8, 192u8, 141u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::FixedArray(
                                Box::new(ethabi::ParamType::Int(8usize)),
                                6usize,
                            ),
                            ethabi::ParamType::Uint(8usize),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    wearable_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    trait_modifiers: {
                        let mut iter = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_array()
                            .expect(INTERNAL_ERR)
                            .into_iter()
                            .map(|inner| {
                                let mut v = [0 as u8; 32];
                                inner
                                    .into_int()
                                    .expect(INTERNAL_ERR)
                                    .to_big_endian(v.as_mut_slice());
                                EthBigInt::new(v.into())
                            });
                        [
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                        ]
                    },
                    rarity_score_modifier: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Uint(self.wearable_id),
                        {
                            let v = self
                                .trait_modifiers
                                .iter()
                                .map(|inner| {
                                    let non_full_signed_bytes = inner
                                        .get_big_int()
                                        .to_signed_bytes_be();
                                    let mut full_signed_bytes = [0xff as u8; 32];
                                    non_full_signed_bytes
                                        .into_iter()
                                        .rev()
                                        .enumerate()
                                        .for_each(|(i, byte)| full_signed_bytes[31 - i] = byte);
                                    ethabi::Token::Int(
                                        ethabi::Int::from_big_endian(full_signed_bytes.as_ref()),
                                    )
                                })
                                .collect();
                            ethabi::Token::FixedArray(v)
                        },
                        ethabi::Token::Uint(self.rarity_score_modifier),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for SetItemTraitModifiersAndRarityModifier {
            const NAME: &'static str = "setItemTraitModifiersAndRarityModifier";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct SetListingFee {
            pub listing_fee_in_wei: ethabi::Uint,
        }
        impl SetListingFee {
            const METHOD_ID: [u8; 4] = [19u8, 29u8, 189u8, 9u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    listing_fee_in_wei: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[ethabi::Token::Uint(self.listing_fee_in_wei)],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for SetListingFee {
            const NAME: &'static str = "setListingFee";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct SetPetOperatorForAll {
            pub operator: Vec<u8>,
            pub approved: bool,
        }
        impl SetPetOperatorForAll {
            const METHOD_ID: [u8; 4] = [205u8, 103u8, 93u8, 87u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Address, ethabi::ParamType::Bool],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    operator: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    approved: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_bool()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Address(
                            ethabi::Address::from_slice(&self.operator),
                        ),
                        ethabi::Token::Bool(self.approved),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for SetPetOperatorForAll {
            const NAME: &'static str = "setPetOperatorForAll";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct SetWearableSlotPositions {
            pub wearable_id: ethabi::Uint,
            pub slot_positions: [bool; 16usize],
        }
        impl SetWearableSlotPositions {
            const METHOD_ID: [u8; 4] = [197u8, 71u8, 248u8, 226u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::FixedArray(
                                Box::new(ethabi::ParamType::Bool),
                                16usize,
                            ),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    wearable_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    slot_positions: {
                        let mut iter = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_array()
                            .expect(INTERNAL_ERR)
                            .into_iter()
                            .map(|inner| inner.into_bool().expect(INTERNAL_ERR));
                        [
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                        ]
                    },
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Uint(self.wearable_id),
                        {
                            let v = self
                                .slot_positions
                                .iter()
                                .map(|inner| ethabi::Token::Bool(inner))
                                .collect();
                            ethabi::Token::FixedArray(v)
                        },
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for SetWearableSlotPositions {
            const NAME: &'static str = "setWearableSlotPositions";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct SpendSkillPoints {
            pub token_id: ethabi::Uint,
            pub values: [EthBigInt; 4usize],
        }
        impl SpendSkillPoints {
            const METHOD_ID: [u8; 4] = [200u8, 137u8, 223u8, 196u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::FixedArray(
                                Box::new(ethabi::ParamType::Int(16usize)),
                                4usize,
                            ),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    token_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    values: {
                        let mut iter = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_array()
                            .expect(INTERNAL_ERR)
                            .into_iter()
                            .map(|inner| {
                                let mut v = [0 as u8; 32];
                                inner
                                    .into_int()
                                    .expect(INTERNAL_ERR)
                                    .to_big_endian(v.as_mut_slice());
                                EthBigInt::new(v.into())
                            });
                        [
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                        ]
                    },
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Uint(self.token_id),
                        {
                            let v = self
                                .values
                                .iter()
                                .map(|inner| {
                                    let non_full_signed_bytes = inner
                                        .get_big_int()
                                        .to_signed_bytes_be();
                                    let mut full_signed_bytes = [0xff as u8; 32];
                                    non_full_signed_bytes
                                        .into_iter()
                                        .rev()
                                        .enumerate()
                                        .for_each(|(i, byte)| full_signed_bytes[31 - i] = byte);
                                    ethabi::Token::Int(
                                        ethabi::Int::from_big_endian(full_signed_bytes.as_ref()),
                                    )
                                })
                                .collect();
                            ethabi::Token::FixedArray(v)
                        },
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for SpendSkillPoints {
            const NAME: &'static str = "spendSkillPoints";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct Symbol {}
        impl Symbol {
            const METHOD_ID: [u8; 4] = [149u8, 216u8, 155u8, 65u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Ok(Self {})
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(&[]);
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<String, String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<String, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::String],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok(
                    values
                        .pop()
                        .expect("one output data should have existed")
                        .into_string()
                        .expect(INTERNAL_ERR),
                )
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<String> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for Symbol {
            const NAME: &'static str = "symbol";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<String> for Symbol {
            fn output(data: &[u8]) -> Result<String, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct TokenByIndex {
            pub index: ethabi::Uint,
        }
        impl TokenByIndex {
            const METHOD_ID: [u8; 4] = [79u8, 108u8, 204u8, 231u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    index: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(&[ethabi::Token::Uint(self.index)]);
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<ethabi::Uint, String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<ethabi::Uint, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok(
                    values
                        .pop()
                        .expect("one output data should have existed")
                        .into_uint()
                        .expect(INTERNAL_ERR),
                )
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<ethabi::Uint> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for TokenByIndex {
            const NAME: &'static str = "tokenByIndex";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<ethabi::Uint> for TokenByIndex {
            fn output(data: &[u8]) -> Result<ethabi::Uint, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct TokenIdsOfOwner {
            pub owner: Vec<u8>,
        }
        impl TokenIdsOfOwner {
            const METHOD_ID: [u8; 4] = [158u8, 89u8, 229u8, 152u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Address],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    owner: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[ethabi::Token::Address(ethabi::Address::from_slice(&self.owner))],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Vec<ethabi::Uint>, String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<Vec<ethabi::Uint>, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Uint(32usize)),
                            ),
                        ],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok(
                    values
                        .pop()
                        .expect("one output data should have existed")
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
                )
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<Vec<ethabi::Uint>> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for TokenIdsOfOwner {
            const NAME: &'static str = "tokenIdsOfOwner";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<Vec<ethabi::Uint>>
        for TokenIdsOfOwner {
            fn output(data: &[u8]) -> Result<Vec<ethabi::Uint>, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct TokenOfOwnerByIndex {
            pub owner: Vec<u8>,
            pub index: ethabi::Uint,
        }
        impl TokenOfOwnerByIndex {
            const METHOD_ID: [u8; 4] = [47u8, 116u8, 92u8, 89u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Address, ethabi::ParamType::Uint(256usize)],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    owner: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    index: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Address(ethabi::Address::from_slice(&self.owner)),
                        ethabi::Token::Uint(self.index),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<ethabi::Uint, String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<ethabi::Uint, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok(
                    values
                        .pop()
                        .expect("one output data should have existed")
                        .into_uint()
                        .expect(INTERNAL_ERR),
                )
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<ethabi::Uint> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for TokenOfOwnerByIndex {
            const NAME: &'static str = "tokenOfOwnerByIndex";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<ethabi::Uint>
        for TokenOfOwnerByIndex {
            fn output(data: &[u8]) -> Result<ethabi::Uint, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct TokenUri {
            pub token_id: ethabi::Uint,
        }
        impl TokenUri {
            const METHOD_ID: [u8; 4] = [200u8, 123u8, 86u8, 221u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    token_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(&[ethabi::Token::Uint(self.token_id)]);
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<String, String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<String, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::String],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok(
                    values
                        .pop()
                        .expect("one output data should have existed")
                        .into_string()
                        .expect(INTERNAL_ERR),
                )
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<String> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for TokenUri {
            const NAME: &'static str = "tokenURI";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<String> for TokenUri {
            fn output(data: &[u8]) -> Result<String, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct TotalSupply {}
        impl TotalSupply {
            const METHOD_ID: [u8; 4] = [24u8, 22u8, 13u8, 221u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Ok(Self {})
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(&[]);
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<ethabi::Uint, String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<ethabi::Uint, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok(
                    values
                        .pop()
                        .expect("one output data should have existed")
                        .into_uint()
                        .expect(INTERNAL_ERR),
                )
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<ethabi::Uint> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for TotalSupply {
            const NAME: &'static str = "totalSupply";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<ethabi::Uint> for TotalSupply {
            fn output(data: &[u8]) -> Result<ethabi::Uint, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct TotalWearableSets {}
        impl TotalWearableSets {
            const METHOD_ID: [u8; 4] = [73u8, 247u8, 237u8, 210u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Ok(Self {})
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(&[]);
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<ethabi::Uint, String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<ethabi::Uint, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok(
                    values
                        .pop()
                        .expect("one output data should have existed")
                        .into_uint()
                        .expect(INTERNAL_ERR),
                )
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<ethabi::Uint> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for TotalWearableSets {
            const NAME: &'static str = "totalWearableSets";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<ethabi::Uint> for TotalWearableSets {
            fn output(data: &[u8]) -> Result<ethabi::Uint, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct TransferAsChild {
            pub from_contract: Vec<u8>,
            pub from_token_id: ethabi::Uint,
            pub to_contract: Vec<u8>,
            pub to_token_id: ethabi::Uint,
            pub id: ethabi::Uint,
            pub value: ethabi::Uint,
        }
        impl TransferAsChild {
            const METHOD_ID: [u8; 4] = [77u8, 17u8, 240u8, 46u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    from_contract: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    from_token_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    to_contract: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    to_token_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    value: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Address(
                            ethabi::Address::from_slice(&self.from_contract),
                        ),
                        ethabi::Token::Uint(self.from_token_id),
                        ethabi::Token::Address(
                            ethabi::Address::from_slice(&self.to_contract),
                        ),
                        ethabi::Token::Uint(self.to_token_id),
                        ethabi::Token::Uint(self.id),
                        ethabi::Token::Uint(self.value),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for TransferAsChild {
            const NAME: &'static str = "transferAsChild";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct TransferEscrow {
            pub token_id: ethabi::Uint,
            pub erc20_contract: Vec<u8>,
            pub recipient: Vec<u8>,
            pub transfer_amount: ethabi::Uint,
        }
        impl TransferEscrow {
            const METHOD_ID: [u8; 4] = [171u8, 15u8, 202u8, 191u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Uint(256usize),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    token_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    erc20_contract: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    recipient: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    transfer_amount: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Uint(self.token_id),
                        ethabi::Token::Address(
                            ethabi::Address::from_slice(&self.erc20_contract),
                        ),
                        ethabi::Token::Address(
                            ethabi::Address::from_slice(&self.recipient),
                        ),
                        ethabi::Token::Uint(self.transfer_amount),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for TransferEscrow {
            const NAME: &'static str = "transferEscrow";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct TransferFrom {
            pub from: Vec<u8>,
            pub to: Vec<u8>,
            pub token_id: ethabi::Uint,
        }
        impl TransferFrom {
            const METHOD_ID: [u8; 4] = [35u8, 184u8, 114u8, 221u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Uint(256usize),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    from: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    to: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    token_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Address(ethabi::Address::from_slice(&self.from)),
                        ethabi::Token::Address(ethabi::Address::from_slice(&self.to)),
                        ethabi::Token::Uint(self.token_id),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for TransferFrom {
            const NAME: &'static str = "transferFrom";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct TransferFromParent {
            pub from_contract: Vec<u8>,
            pub from_token_id: ethabi::Uint,
            pub to: Vec<u8>,
            pub id: ethabi::Uint,
            pub value: ethabi::Uint,
        }
        impl TransferFromParent {
            const METHOD_ID: [u8; 4] = [39u8, 26u8, 108u8, 89u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    from_contract: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    from_token_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    to: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    value: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Address(
                            ethabi::Address::from_slice(&self.from_contract),
                        ),
                        ethabi::Token::Uint(self.from_token_id),
                        ethabi::Token::Address(ethabi::Address::from_slice(&self.to)),
                        ethabi::Token::Uint(self.id),
                        ethabi::Token::Uint(self.value),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for TransferFromParent {
            const NAME: &'static str = "transferFromParent";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct TransferToParent {
            pub from: Vec<u8>,
            pub to_contract: Vec<u8>,
            pub to_token_id: ethabi::Uint,
            pub id: ethabi::Uint,
            pub value: ethabi::Uint,
        }
        impl TransferToParent {
            const METHOD_ID: [u8; 4] = [57u8, 54u8, 184u8, 210u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    from: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    to_contract: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    to_token_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    value: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Address(ethabi::Address::from_slice(&self.from)),
                        ethabi::Token::Address(
                            ethabi::Address::from_slice(&self.to_contract),
                        ),
                        ethabi::Token::Uint(self.to_token_id),
                        ethabi::Token::Uint(self.id),
                        ethabi::Token::Uint(self.value),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for TransferToParent {
            const NAME: &'static str = "transferToParent";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct UpdateBatchErc1155Listing {
            pub erc1155_token_address: Vec<u8>,
            pub erc1155_type_ids: Vec<ethabi::Uint>,
            pub owner: Vec<u8>,
        }
        impl UpdateBatchErc1155Listing {
            const METHOD_ID: [u8; 4] = [250u8, 196u8, 60u8, 133u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                            ),
                            ethabi::ParamType::Address,
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    erc1155_token_address: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    erc1155_type_ids: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
                    owner: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Address(
                            ethabi::Address::from_slice(&self.erc1155_token_address),
                        ),
                        {
                            let v = self
                                .erc1155_type_ids
                                .iter()
                                .map(|inner| ethabi::Token::Uint(inner.clone()))
                                .collect();
                            ethabi::Token::Array(v)
                        },
                        ethabi::Token::Address(ethabi::Address::from_slice(&self.owner)),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for UpdateBatchErc1155Listing {
            const NAME: &'static str = "updateBatchERC1155Listing";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct UpdateCollateralModifiers {
            pub collateral_type: Vec<u8>,
            pub modifiers: [EthBigInt; 6usize],
        }
        impl UpdateCollateralModifiers {
            const METHOD_ID: [u8; 4] = [156u8, 201u8, 183u8, 169u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Address,
                            ethabi::ParamType::FixedArray(
                                Box::new(ethabi::ParamType::Int(16usize)),
                                6usize,
                            ),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    collateral_type: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    modifiers: {
                        let mut iter = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_array()
                            .expect(INTERNAL_ERR)
                            .into_iter()
                            .map(|inner| {
                                let mut v = [0 as u8; 32];
                                inner
                                    .into_int()
                                    .expect(INTERNAL_ERR)
                                    .to_big_endian(v.as_mut_slice());
                                EthBigInt::new(v.into())
                            });
                        [
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                        ]
                    },
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Address(
                            ethabi::Address::from_slice(&self.collateral_type),
                        ),
                        {
                            let v = self
                                .modifiers
                                .iter()
                                .map(|inner| {
                                    let non_full_signed_bytes = inner
                                        .get_big_int()
                                        .to_signed_bytes_be();
                                    let mut full_signed_bytes = [0xff as u8; 32];
                                    non_full_signed_bytes
                                        .into_iter()
                                        .rev()
                                        .enumerate()
                                        .for_each(|(i, byte)| full_signed_bytes[31 - i] = byte);
                                    ethabi::Token::Int(
                                        ethabi::Int::from_big_endian(full_signed_bytes.as_ref()),
                                    )
                                })
                                .collect();
                            ethabi::Token::FixedArray(v)
                        },
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for UpdateCollateralModifiers {
            const NAME: &'static str = "updateCollateralModifiers";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct UpdateErc1155Listing {
            pub erc1155_token_address: Vec<u8>,
            pub erc1155_type_id: ethabi::Uint,
            pub owner: Vec<u8>,
        }
        impl UpdateErc1155Listing {
            const METHOD_ID: [u8; 4] = [233u8, 168u8, 136u8, 127u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Address,
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    erc1155_token_address: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    erc1155_type_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    owner: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Address(
                            ethabi::Address::from_slice(&self.erc1155_token_address),
                        ),
                        ethabi::Token::Uint(self.erc1155_type_id),
                        ethabi::Token::Address(ethabi::Address::from_slice(&self.owner)),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for UpdateErc1155Listing {
            const NAME: &'static str = "updateERC1155Listing";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct UpdateErc721Listing {
            pub erc721_token_address: Vec<u8>,
            pub erc721_token_id: ethabi::Uint,
            pub owner: Vec<u8>,
        }
        impl UpdateErc721Listing {
            const METHOD_ID: [u8; 4] = [62u8, 237u8, 99u8, 90u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Address,
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    erc721_token_address: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    erc721_token_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    owner: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Address(
                            ethabi::Address::from_slice(&self.erc721_token_address),
                        ),
                        ethabi::Token::Uint(self.erc721_token_id),
                        ethabi::Token::Address(ethabi::Address::from_slice(&self.owner)),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for UpdateErc721Listing {
            const NAME: &'static str = "updateERC721Listing";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct UpdateItemTypeMaxQuantity {
            pub item_ids: Vec<ethabi::Uint>,
            pub max_quantities: Vec<ethabi::Uint>,
        }
        impl UpdateItemTypeMaxQuantity {
            const METHOD_ID: [u8; 4] = [99u8, 155u8, 175u8, 171u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                            ),
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                            ),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    item_ids: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
                    max_quantities: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        {
                            let v = self
                                .item_ids
                                .iter()
                                .map(|inner| ethabi::Token::Uint(inner.clone()))
                                .collect();
                            ethabi::Token::Array(v)
                        },
                        {
                            let v = self
                                .max_quantities
                                .iter()
                                .map(|inner| ethabi::Token::Uint(inner.clone()))
                                .collect();
                            ethabi::Token::Array(v)
                        },
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for UpdateItemTypeMaxQuantity {
            const NAME: &'static str = "updateItemTypeMaxQuantity";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct UpdateWhitelist {
            pub whitelist_id: ethabi::Uint,
            pub whitelist_addresses: Vec<Vec<u8>>,
        }
        impl UpdateWhitelist {
            const METHOD_ID: [u8; 4] = [236u8, 245u8, 249u8, 145u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Uint(32usize),
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Address),
                            ),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    whitelist_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    whitelist_addresses: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| {
                            inner.into_address().expect(INTERNAL_ERR).as_bytes().to_vec()
                        })
                        .collect(),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Uint(self.whitelist_id),
                        {
                            let v = self
                                .whitelist_addresses
                                .iter()
                                .map(|inner| ethabi::Token::Address(
                                    ethabi::Address::from_slice(&inner),
                                ))
                                .collect();
                            ethabi::Token::Array(v)
                        },
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for UpdateWhitelist {
            const NAME: &'static str = "updateWhitelist";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct Uri {
            pub id: ethabi::Uint,
        }
        impl Uri {
            const METHOD_ID: [u8; 4] = [14u8, 137u8, 52u8, 28u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(&[ethabi::Token::Uint(self.id)]);
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<String, String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<String, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::String],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok(
                    values
                        .pop()
                        .expect("one output data should have existed")
                        .into_string()
                        .expect(INTERNAL_ERR),
                )
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<String> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for Uri {
            const NAME: &'static str = "uri";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<String> for Uri {
            fn output(data: &[u8]) -> Result<String, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct UseConsumables {
            pub token_id: ethabi::Uint,
            pub item_ids: Vec<ethabi::Uint>,
            pub quantities: Vec<ethabi::Uint>,
        }
        impl UseConsumables {
            const METHOD_ID: [u8; 4] = [237u8, 68u8, 102u8, 1u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                            ),
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                            ),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    token_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    item_ids: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
                    quantities: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Uint(self.token_id),
                        {
                            let v = self
                                .item_ids
                                .iter()
                                .map(|inner| ethabi::Token::Uint(inner.clone()))
                                .collect();
                            ethabi::Token::Array(v)
                        },
                        {
                            let v = self
                                .quantities
                                .iter()
                                .map(|inner| ethabi::Token::Uint(inner.clone()))
                                .collect();
                            ethabi::Token::Array(v)
                        },
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for UseConsumables {
            const NAME: &'static str = "useConsumables";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct VrfCoordinator {}
        impl VrfCoordinator {
            const METHOD_ID: [u8; 4] = [163u8, 229u8, 111u8, 168u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Ok(Self {})
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(&[]);
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Vec<u8>, String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<Vec<u8>, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Address],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok(
                    values
                        .pop()
                        .expect("one output data should have existed")
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                )
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<Vec<u8>> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for VrfCoordinator {
            const NAME: &'static str = "vrfCoordinator";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<Vec<u8>> for VrfCoordinator {
            fn output(data: &[u8]) -> Result<Vec<u8>, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct WithdrawAavegotchiBatch {
            pub token_ids: Vec<ethabi::Uint>,
        }
        impl WithdrawAavegotchiBatch {
            const METHOD_ID: [u8; 4] = [60u8, 166u8, 93u8, 49u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                            ),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    token_ids: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        {
                            let v = self
                                .token_ids
                                .iter()
                                .map(|inner| ethabi::Token::Uint(inner.clone()))
                                .collect();
                            ethabi::Token::Array(v)
                        },
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for WithdrawAavegotchiBatch {
            const NAME: &'static str = "withdrawAavegotchiBatch";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct WithdrawItemsBatch {
            pub ids: Vec<ethabi::Uint>,
            pub values: Vec<ethabi::Uint>,
        }
        impl WithdrawItemsBatch {
            const METHOD_ID: [u8; 4] = [108u8, 128u8, 193u8, 108u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                            ),
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                            ),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    ids: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
                    values: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        {
                            let v = self
                                .ids
                                .iter()
                                .map(|inner| ethabi::Token::Uint(inner.clone()))
                                .collect();
                            ethabi::Token::Array(v)
                        },
                        {
                            let v = self
                                .values
                                .iter()
                                .map(|inner| ethabi::Token::Uint(inner.clone()))
                                .collect();
                            ethabi::Token::Array(v)
                        },
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for WithdrawItemsBatch {
            const NAME: &'static str = "withdrawItemsBatch";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct XpUntilNextLevel {
            pub experience: ethabi::Uint,
        }
        impl XpUntilNextLevel {
            const METHOD_ID: [u8; 4] = [130u8, 64u8, 28u8, 205u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    experience: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(&[ethabi::Token::Uint(self.experience)]);
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<ethabi::Uint, String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<ethabi::Uint, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok(
                    values
                        .pop()
                        .expect("one output data should have existed")
                        .into_uint()
                        .expect(INTERNAL_ERR),
                )
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<ethabi::Uint> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for XpUntilNextLevel {
            const NAME: &'static str = "xpUntilNextLevel";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<ethabi::Uint> for XpUntilNextLevel {
            fn output(data: &[u8]) -> Result<ethabi::Uint, String> {
                Self::output(data)
            }
        }
    }
    /// Contract's events.
    #[allow(dead_code)]
    pub mod events {
        use substreams_ethereum::scalar::EthBigInt;
        use super::INTERNAL_ERR;
        #[derive(Debug, Clone, PartialEq)]
        pub struct AavegotchiInteract {
            pub token_id: ethabi::Uint,
            pub kinship: ethabi::Uint,
        }
        impl AavegotchiInteract {
            const TOPIC_ID: [u8; 32] = [
                241u8,
                217u8,
                5u8,
                38u8,
                22u8,
                36u8,
                149u8,
                136u8,
                147u8,
                27u8,
                82u8,
                132u8,
                103u8,
                162u8,
                50u8,
                152u8,
                208u8,
                213u8,
                165u8,
                17u8,
                54u8,
                163u8,
                100u8,
                187u8,
                39u8,
                58u8,
                136u8,
                58u8,
                92u8,
                73u8,
                205u8,
                165u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 2usize {
                    return false;
                }
                if log.data.len() != 32usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    token_id: ethabi::decode(
                            &[ethabi::ParamType::Uint(256usize)],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'token_id' from topic of type 'uint256': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    kinship: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for AavegotchiInteract {
            const NAME: &'static str = "AavegotchiInteract";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct AddedAavegotchiBatch {
            pub owner: Vec<u8>,
            pub token_ids: Vec<ethabi::Uint>,
        }
        impl AddedAavegotchiBatch {
            const TOPIC_ID: [u8; 32] = [
                43u8,
                225u8,
                213u8,
                144u8,
                95u8,
                221u8,
                50u8,
                127u8,
                57u8,
                172u8,
                242u8,
                42u8,
                97u8,
                80u8,
                142u8,
                175u8,
                246u8,
                137u8,
                115u8,
                10u8,
                17u8,
                190u8,
                8u8,
                34u8,
                71u8,
                71u8,
                33u8,
                237u8,
                215u8,
                108u8,
                203u8,
                213u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 2usize {
                    return false;
                }
                if log.data.len() < 64usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                            ),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    owner: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'owner' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    token_ids: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
                })
            }
        }
        impl substreams_ethereum::Event for AddedAavegotchiBatch {
            const NAME: &'static str = "AddedAavegotchiBatch";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct AddedItemsBatch {
            pub owner: Vec<u8>,
            pub ids: Vec<ethabi::Uint>,
            pub values: Vec<ethabi::Uint>,
        }
        impl AddedItemsBatch {
            const TOPIC_ID: [u8; 32] = [
                31u8,
                143u8,
                126u8,
                6u8,
                196u8,
                105u8,
                101u8,
                243u8,
                69u8,
                69u8,
                232u8,
                254u8,
                107u8,
                81u8,
                22u8,
                249u8,
                133u8,
                88u8,
                221u8,
                246u8,
                193u8,
                25u8,
                228u8,
                203u8,
                18u8,
                19u8,
                86u8,
                102u8,
                68u8,
                79u8,
                103u8,
                1u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 2usize {
                    return false;
                }
                if log.data.len() < 128usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                            ),
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                            ),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    owner: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'owner' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    ids: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
                    values: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
                })
            }
        }
        impl substreams_ethereum::Event for AddedItemsBatch {
            const NAME: &'static str = "AddedItemsBatch";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct Approval {
            pub owner: Vec<u8>,
            pub approved: Vec<u8>,
            pub token_id: ethabi::Uint,
        }
        impl Approval {
            const TOPIC_ID: [u8; 32] = [
                140u8,
                91u8,
                225u8,
                229u8,
                235u8,
                236u8,
                125u8,
                91u8,
                209u8,
                79u8,
                113u8,
                66u8,
                125u8,
                30u8,
                132u8,
                243u8,
                221u8,
                3u8,
                20u8,
                192u8,
                247u8,
                178u8,
                41u8,
                30u8,
                91u8,
                32u8,
                10u8,
                200u8,
                199u8,
                195u8,
                185u8,
                37u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 4usize {
                    return false;
                }
                if log.data.len() != 0usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Ok(Self {
                    owner: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'owner' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    approved: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[2usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'approved' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    token_id: ethabi::decode(
                            &[ethabi::ParamType::Uint(256usize)],
                            log.topics[3usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'token_id' from topic of type 'uint256': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for Approval {
            const NAME: &'static str = "Approval";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct ApprovalForAll1 {
            pub owner: Vec<u8>,
            pub operator: Vec<u8>,
            pub approved: bool,
        }
        impl ApprovalForAll1 {
            const TOPIC_ID: [u8; 32] = [
                23u8,
                48u8,
                126u8,
                171u8,
                57u8,
                171u8,
                97u8,
                7u8,
                232u8,
                137u8,
                152u8,
                69u8,
                173u8,
                61u8,
                89u8,
                189u8,
                150u8,
                83u8,
                242u8,
                0u8,
                242u8,
                32u8,
                146u8,
                4u8,
                137u8,
                202u8,
                43u8,
                89u8,
                55u8,
                105u8,
                108u8,
                49u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 3usize {
                    return false;
                }
                if log.data.len() != 32usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Bool],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    owner: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'owner' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    operator: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[2usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'operator' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    approved: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_bool()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for ApprovalForAll1 {
            const NAME: &'static str = "ApprovalForAll1";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct ApprovalForAll2 {
            pub owner: Vec<u8>,
            pub operator: Vec<u8>,
            pub approved: bool,
        }
        impl ApprovalForAll2 {
            const TOPIC_ID: [u8; 32] = [
                23u8,
                48u8,
                126u8,
                171u8,
                57u8,
                171u8,
                97u8,
                7u8,
                232u8,
                137u8,
                152u8,
                69u8,
                173u8,
                61u8,
                89u8,
                189u8,
                150u8,
                83u8,
                242u8,
                0u8,
                242u8,
                32u8,
                146u8,
                4u8,
                137u8,
                202u8,
                43u8,
                89u8,
                55u8,
                105u8,
                108u8,
                49u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 3usize {
                    return false;
                }
                if log.data.len() != 32usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Bool],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    owner: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'owner' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    operator: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[2usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'operator' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    approved: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_bool()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for ApprovalForAll2 {
            const NAME: &'static str = "ApprovalForAll2";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct BuyPortals {
            pub from: Vec<u8>,
            pub to: Vec<u8>,
            pub token_id: ethabi::Uint,
            pub num_aavegotchis_to_purchase: ethabi::Uint,
            pub total_price: ethabi::Uint,
        }
        impl BuyPortals {
            const TOPIC_ID: [u8; 32] = [
                209u8,
                198u8,
                193u8,
                6u8,
                114u8,
                99u8,
                241u8,
                1u8,
                207u8,
                24u8,
                3u8,
                242u8,
                125u8,
                86u8,
                45u8,
                85u8,
                20u8,
                220u8,
                187u8,
                146u8,
                26u8,
                17u8,
                142u8,
                30u8,
                79u8,
                116u8,
                183u8,
                58u8,
                135u8,
                242u8,
                109u8,
                35u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 3usize {
                    return false;
                }
                if log.data.len() != 96usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    from: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'from' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    to: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[2usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'to' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    token_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    num_aavegotchis_to_purchase: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    total_price: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for BuyPortals {
            const NAME: &'static str = "BuyPortals";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct ChangedListingFee {
            pub listing_fee_in_wei: ethabi::Uint,
        }
        impl ChangedListingFee {
            const TOPIC_ID: [u8; 32] = [
                71u8,
                17u8,
                10u8,
                166u8,
                55u8,
                153u8,
                3u8,
                161u8,
                201u8,
                48u8,
                83u8,
                183u8,
                216u8,
                182u8,
                232u8,
                189u8,
                187u8,
                173u8,
                216u8,
                147u8,
                228u8,
                87u8,
                161u8,
                225u8,
                5u8,
                136u8,
                196u8,
                226u8,
                169u8,
                87u8,
                93u8,
                199u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 1usize {
                    return false;
                }
                if log.data.len() != 32usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    listing_fee_in_wei: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for ChangedListingFee {
            const NAME: &'static str = "ChangedListingFee";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct ClaimAavegotchi {
            pub token_id: ethabi::Uint,
        }
        impl ClaimAavegotchi {
            const TOPIC_ID: [u8; 32] = [
                173u8,
                191u8,
                120u8,
                239u8,
                50u8,
                44u8,
                15u8,
                40u8,
                108u8,
                159u8,
                13u8,
                8u8,
                18u8,
                245u8,
                63u8,
                109u8,
                181u8,
                159u8,
                206u8,
                252u8,
                78u8,
                65u8,
                21u8,
                7u8,
                88u8,
                244u8,
                61u8,
                169u8,
                126u8,
                42u8,
                128u8,
                227u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 2usize {
                    return false;
                }
                if log.data.len() != 0usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Ok(Self {
                    token_id: ethabi::decode(
                            &[ethabi::ParamType::Uint(256usize)],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'token_id' from topic of type 'uint256': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for ClaimAavegotchi {
            const NAME: &'static str = "ClaimAavegotchi";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct CreateHaunt {
            pub haunt_id: ethabi::Uint,
            pub haunt_max_size: ethabi::Uint,
            pub portal_price: ethabi::Uint,
            pub body_color: [u8; 32usize],
        }
        impl CreateHaunt {
            const TOPIC_ID: [u8; 32] = [
                247u8,
                208u8,
                227u8,
                194u8,
                241u8,
                195u8,
                247u8,
                202u8,
                12u8,
                201u8,
                79u8,
                138u8,
                127u8,
                184u8,
                237u8,
                99u8,
                171u8,
                78u8,
                37u8,
                37u8,
                43u8,
                226u8,
                224u8,
                109u8,
                229u8,
                188u8,
                117u8,
                228u8,
                211u8,
                14u8,
                149u8,
                53u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 2usize {
                    return false;
                }
                if log.data.len() != 96usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::FixedBytes(32usize),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    haunt_id: ethabi::decode(
                            &[ethabi::ParamType::Uint(256usize)],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'haunt_id' from topic of type 'uint256': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    haunt_max_size: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    portal_price: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    body_color: {
                        let mut result = [0u8; 32];
                        let v = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_bytes()
                            .expect(INTERNAL_ERR);
                        result.copy_from_slice(&v);
                        result
                    },
                })
            }
        }
        impl substreams_ethereum::Event for CreateHaunt {
            const NAME: &'static str = "CreateHaunt";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct DaoTransferred {
            pub previous_dao: Vec<u8>,
            pub new_dao: Vec<u8>,
        }
        impl DaoTransferred {
            const TOPIC_ID: [u8; 32] = [
                128u8,
                227u8,
                114u8,
                149u8,
                25u8,
                150u8,
                212u8,
                234u8,
                245u8,
                91u8,
                172u8,
                105u8,
                209u8,
                137u8,
                173u8,
                60u8,
                95u8,
                57u8,
                146u8,
                220u8,
                143u8,
                72u8,
                109u8,
                133u8,
                194u8,
                199u8,
                171u8,
                255u8,
                254u8,
                9u8,
                249u8,
                157u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 3usize {
                    return false;
                }
                if log.data.len() != 0usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Ok(Self {
                    previous_dao: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'previous_dao' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    new_dao: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[2usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'new_dao' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                })
            }
        }
        impl substreams_ethereum::Event for DaoTransferred {
            const NAME: &'static str = "DaoTransferred";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct DaoTreasuryTransferred {
            pub previous_dao_treasury: Vec<u8>,
            pub new_dao_treasury: Vec<u8>,
        }
        impl DaoTreasuryTransferred {
            const TOPIC_ID: [u8; 32] = [
                141u8,
                218u8,
                153u8,
                244u8,
                192u8,
                151u8,
                103u8,
                50u8,
                197u8,
                169u8,
                127u8,
                80u8,
                253u8,
                188u8,
                148u8,
                123u8,
                240u8,
                123u8,
                154u8,
                239u8,
                26u8,
                71u8,
                69u8,
                188u8,
                200u8,
                171u8,
                89u8,
                181u8,
                48u8,
                47u8,
                107u8,
                134u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 3usize {
                    return false;
                }
                if log.data.len() != 0usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Ok(Self {
                    previous_dao_treasury: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'previous_dao_treasury' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    new_dao_treasury: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[2usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'new_dao_treasury' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                })
            }
        }
        impl substreams_ethereum::Event for DaoTreasuryTransferred {
            const NAME: &'static str = "DaoTreasuryTransferred";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct DecreaseStake {
            pub token_id: ethabi::Uint,
            pub reduce_amount: ethabi::Uint,
        }
        impl DecreaseStake {
            const TOPIC_ID: [u8; 32] = [
                67u8,
                194u8,
                245u8,
                27u8,
                9u8,
                29u8,
                121u8,
                136u8,
                169u8,
                178u8,
                107u8,
                122u8,
                15u8,
                48u8,
                77u8,
                246u8,
                202u8,
                49u8,
                206u8,
                5u8,
                200u8,
                106u8,
                200u8,
                255u8,
                108u8,
                181u8,
                129u8,
                82u8,
                149u8,
                152u8,
                182u8,
                54u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 2usize {
                    return false;
                }
                if log.data.len() != 32usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    token_id: ethabi::decode(
                            &[ethabi::ParamType::Uint(256usize)],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'token_id' from topic of type 'uint256': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    reduce_amount: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for DecreaseStake {
            const NAME: &'static str = "DecreaseStake";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct Erc1155ExecutedListing {
            pub listing_id: ethabi::Uint,
            pub seller: Vec<u8>,
            pub buyer: Vec<u8>,
            pub erc1155_token_address: Vec<u8>,
            pub erc1155_type_id: ethabi::Uint,
            pub category: ethabi::Uint,
            pub quantity: ethabi::Uint,
            pub price_in_wei: ethabi::Uint,
            pub time: ethabi::Uint,
        }
        impl Erc1155ExecutedListing {
            const TOPIC_ID: [u8; 32] = [
                64u8,
                4u8,
                24u8,
                95u8,
                69u8,
                207u8,
                95u8,
                51u8,
                31u8,
                230u8,
                58u8,
                252u8,
                154u8,
                165u8,
                170u8,
                10u8,
                13u8,
                140u8,
                250u8,
                29u8,
                107u8,
                197u8,
                216u8,
                198u8,
                203u8,
                3u8,
                4u8,
                19u8,
                108u8,
                73u8,
                81u8,
                92u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 4usize {
                    return false;
                }
                if log.data.len() != 192usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    listing_id: ethabi::decode(
                            &[ethabi::ParamType::Uint(256usize)],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'listing_id' from topic of type 'uint256': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    seller: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[2usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'seller' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    category: ethabi::decode(
                            &[ethabi::ParamType::Uint(256usize)],
                            log.topics[3usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'category' from topic of type 'uint256': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    buyer: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    erc1155_token_address: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    erc1155_type_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    quantity: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    price_in_wei: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    time: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for Erc1155ExecutedListing {
            const NAME: &'static str = "ERC1155ExecutedListing";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct Erc1155ExecutedToRecipient {
            pub listing_id: ethabi::Uint,
            pub buyer: Vec<u8>,
            pub recipient: Vec<u8>,
        }
        impl Erc1155ExecutedToRecipient {
            const TOPIC_ID: [u8; 32] = [
                109u8,
                16u8,
                89u8,
                126u8,
                174u8,
                163u8,
                182u8,
                241u8,
                7u8,
                57u8,
                137u8,
                108u8,
                147u8,
                162u8,
                205u8,
                235u8,
                248u8,
                120u8,
                16u8,
                211u8,
                114u8,
                38u8,
                180u8,
                11u8,
                195u8,
                228u8,
                51u8,
                116u8,
                95u8,
                85u8,
                206u8,
                156u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 4usize {
                    return false;
                }
                if log.data.len() != 0usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Ok(Self {
                    listing_id: ethabi::decode(
                            &[ethabi::ParamType::Uint(256usize)],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'listing_id' from topic of type 'uint256': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    buyer: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[2usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'buyer' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    recipient: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[3usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'recipient' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                })
            }
        }
        impl substreams_ethereum::Event for Erc1155ExecutedToRecipient {
            const NAME: &'static str = "ERC1155ExecutedToRecipient";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct Erc1155ListingAdd {
            pub listing_id: ethabi::Uint,
            pub seller: Vec<u8>,
            pub erc1155_token_address: Vec<u8>,
            pub erc1155_type_id: ethabi::Uint,
            pub category: ethabi::Uint,
            pub quantity: ethabi::Uint,
            pub price_in_wei: ethabi::Uint,
            pub time: ethabi::Uint,
        }
        impl Erc1155ListingAdd {
            const TOPIC_ID: [u8; 32] = [
                249u8,
                154u8,
                7u8,
                237u8,
                194u8,
                44u8,
                136u8,
                247u8,
                59u8,
                25u8,
                223u8,
                9u8,
                255u8,
                250u8,
                155u8,
                17u8,
                201u8,
                171u8,
                92u8,
                93u8,
                214u8,
                223u8,
                194u8,
                246u8,
                225u8,
                3u8,
                169u8,
                95u8,
                91u8,
                64u8,
                226u8,
                252u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 4usize {
                    return false;
                }
                if log.data.len() != 160usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    listing_id: ethabi::decode(
                            &[ethabi::ParamType::Uint(256usize)],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'listing_id' from topic of type 'uint256': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    seller: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[2usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'seller' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    category: ethabi::decode(
                            &[ethabi::ParamType::Uint(256usize)],
                            log.topics[3usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'category' from topic of type 'uint256': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    erc1155_token_address: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    erc1155_type_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    quantity: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    price_in_wei: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    time: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for Erc1155ListingAdd {
            const NAME: &'static str = "ERC1155ListingAdd";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct Erc1155ListingCancelled1 {
            pub listing_id: ethabi::Uint,
        }
        impl Erc1155ListingCancelled1 {
            const TOPIC_ID: [u8; 32] = [
                252u8,
                175u8,
                109u8,
                210u8,
                189u8,
                58u8,
                15u8,
                78u8,
                150u8,
                114u8,
                112u8,
                136u8,
                204u8,
                224u8,
                200u8,
                71u8,
                115u8,
                204u8,
                68u8,
                69u8,
                175u8,
                126u8,
                102u8,
                37u8,
                251u8,
                116u8,
                219u8,
                16u8,
                253u8,
                191u8,
                86u8,
                191u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 2usize {
                    return false;
                }
                if log.data.len() != 0usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Ok(Self {
                    listing_id: ethabi::decode(
                            &[ethabi::ParamType::Uint(256usize)],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'listing_id' from topic of type 'uint256': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for Erc1155ListingCancelled1 {
            const NAME: &'static str = "ERC1155ListingCancelled1";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct Erc1155ListingCancelled2 {
            pub listing_id: ethabi::Uint,
            pub category: ethabi::Uint,
            pub time: ethabi::Uint,
        }
        impl Erc1155ListingCancelled2 {
            const TOPIC_ID: [u8; 32] = [
                193u8,
                238u8,
                112u8,
                212u8,
                218u8,
                220u8,
                14u8,
                168u8,
                4u8,
                31u8,
                49u8,
                248u8,
                27u8,
                7u8,
                208u8,
                172u8,
                147u8,
                116u8,
                182u8,
                191u8,
                65u8,
                149u8,
                109u8,
                156u8,
                174u8,
                86u8,
                46u8,
                103u8,
                47u8,
                63u8,
                138u8,
                145u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 2usize {
                    return false;
                }
                if log.data.len() != 64usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    listing_id: ethabi::decode(
                            &[ethabi::ParamType::Uint(256usize)],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'listing_id' from topic of type 'uint256': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    category: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    time: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for Erc1155ListingCancelled2 {
            const NAME: &'static str = "ERC1155ListingCancelled2";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct Erc1155ListingRemoved {
            pub listing_id: ethabi::Uint,
            pub category: ethabi::Uint,
            pub time: ethabi::Uint,
        }
        impl Erc1155ListingRemoved {
            const TOPIC_ID: [u8; 32] = [
                39u8,
                106u8,
                195u8,
                155u8,
                15u8,
                152u8,
                33u8,
                85u8,
                146u8,
                180u8,
                72u8,
                116u8,
                52u8,
                193u8,
                97u8,
                142u8,
                62u8,
                82u8,
                124u8,
                87u8,
                202u8,
                47u8,
                71u8,
                26u8,
                201u8,
                62u8,
                180u8,
                234u8,
                177u8,
                105u8,
                64u8,
                145u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 2usize {
                    return false;
                }
                if log.data.len() != 64usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    listing_id: ethabi::decode(
                            &[ethabi::ParamType::Uint(256usize)],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'listing_id' from topic of type 'uint256': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    category: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    time: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for Erc1155ListingRemoved {
            const NAME: &'static str = "ERC1155ListingRemoved";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct Erc721ExecutedListing {
            pub listing_id: ethabi::Uint,
            pub seller: Vec<u8>,
            pub buyer: Vec<u8>,
            pub erc721_token_address: Vec<u8>,
            pub erc721_token_id: ethabi::Uint,
            pub category: ethabi::Uint,
            pub price_in_wei: ethabi::Uint,
            pub time: ethabi::Uint,
        }
        impl Erc721ExecutedListing {
            const TOPIC_ID: [u8; 32] = [
                138u8,
                187u8,
                39u8,
                229u8,
                63u8,
                196u8,
                171u8,
                81u8,
                78u8,
                126u8,
                85u8,
                33u8,
                145u8,
                11u8,
                223u8,
                45u8,
                30u8,
                72u8,
                166u8,
                159u8,
                21u8,
                13u8,
                187u8,
                103u8,
                30u8,
                250u8,
                124u8,
                204u8,
                143u8,
                68u8,
                187u8,
                40u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 4usize {
                    return false;
                }
                if log.data.len() != 160usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    listing_id: ethabi::decode(
                            &[ethabi::ParamType::Uint(256usize)],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'listing_id' from topic of type 'uint256': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    seller: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[2usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'seller' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    category: ethabi::decode(
                            &[ethabi::ParamType::Uint(256usize)],
                            log.topics[3usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'category' from topic of type 'uint256': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    buyer: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    erc721_token_address: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    erc721_token_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    price_in_wei: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    time: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for Erc721ExecutedListing {
            const NAME: &'static str = "ERC721ExecutedListing";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct Erc721ExecutedToRecipient {
            pub listing_id: ethabi::Uint,
            pub buyer: Vec<u8>,
            pub recipient: Vec<u8>,
        }
        impl Erc721ExecutedToRecipient {
            const TOPIC_ID: [u8; 32] = [
                15u8,
                82u8,
                253u8,
                230u8,
                23u8,
                29u8,
                69u8,
                192u8,
                37u8,
                121u8,
                54u8,
                69u8,
                151u8,
                64u8,
                102u8,
                82u8,
                105u8,
                11u8,
                242u8,
                38u8,
                78u8,
                86u8,
                75u8,
                161u8,
                165u8,
                184u8,
                143u8,
                250u8,
                152u8,
                216u8,
                129u8,
                91u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 4usize {
                    return false;
                }
                if log.data.len() != 0usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Ok(Self {
                    listing_id: ethabi::decode(
                            &[ethabi::ParamType::Uint(256usize)],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'listing_id' from topic of type 'uint256': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    buyer: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[2usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'buyer' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    recipient: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[3usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'recipient' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                })
            }
        }
        impl substreams_ethereum::Event for Erc721ExecutedToRecipient {
            const NAME: &'static str = "ERC721ExecutedToRecipient";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct Erc721ListingAdd {
            pub listing_id: ethabi::Uint,
            pub seller: Vec<u8>,
            pub erc721_token_address: Vec<u8>,
            pub erc721_token_id: ethabi::Uint,
            pub category: ethabi::Uint,
            pub time: ethabi::Uint,
        }
        impl Erc721ListingAdd {
            const TOPIC_ID: [u8; 32] = [
                217u8,
                181u8,
                65u8,
                205u8,
                29u8,
                150u8,
                80u8,
                219u8,
                188u8,
                185u8,
                178u8,
                68u8,
                239u8,
                228u8,
                170u8,
                148u8,
                85u8,
                169u8,
                231u8,
                187u8,
                50u8,
                159u8,
                128u8,
                205u8,
                48u8,
                171u8,
                37u8,
                39u8,
                85u8,
                52u8,
                199u8,
                45u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 4usize {
                    return false;
                }
                if log.data.len() != 96usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    listing_id: ethabi::decode(
                            &[ethabi::ParamType::Uint(256usize)],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'listing_id' from topic of type 'uint256': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    seller: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[2usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'seller' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    category: ethabi::decode(
                            &[ethabi::ParamType::Uint(256usize)],
                            log.topics[3usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'category' from topic of type 'uint256': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    erc721_token_address: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    erc721_token_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    time: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for Erc721ListingAdd {
            const NAME: &'static str = "ERC721ListingAdd";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct Erc721ListingCancelled {
            pub listing_id: ethabi::Uint,
            pub category: ethabi::Uint,
            pub time: ethabi::Uint,
        }
        impl Erc721ListingCancelled {
            const TOPIC_ID: [u8; 32] = [
                16u8,
                229u8,
                59u8,
                202u8,
                98u8,
                3u8,
                20u8,
                231u8,
                148u8,
                214u8,
                30u8,
                55u8,
                147u8,
                32u8,
                98u8,
                232u8,
                28u8,
                72u8,
                232u8,
                130u8,
                51u8,
                118u8,
                93u8,
                9u8,
                218u8,
                66u8,
                174u8,
                176u8,
                10u8,
                228u8,
                219u8,
                160u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 2usize {
                    return false;
                }
                if log.data.len() != 64usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    listing_id: ethabi::decode(
                            &[ethabi::ParamType::Uint(256usize)],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'listing_id' from topic of type 'uint256': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    category: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    time: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for Erc721ListingCancelled {
            const NAME: &'static str = "ERC721ListingCancelled";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct Erc721ListingRemoved {
            pub listing_id: ethabi::Uint,
            pub category: ethabi::Uint,
            pub time: ethabi::Uint,
        }
        impl Erc721ListingRemoved {
            const TOPIC_ID: [u8; 32] = [
                211u8,
                9u8,
                187u8,
                160u8,
                115u8,
                221u8,
                70u8,
                61u8,
                157u8,
                120u8,
                233u8,
                39u8,
                102u8,
                208u8,
                109u8,
                167u8,
                205u8,
                231u8,
                208u8,
                230u8,
                232u8,
                49u8,
                73u8,
                22u8,
                138u8,
                123u8,
                201u8,
                19u8,
                201u8,
                236u8,
                196u8,
                66u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 2usize {
                    return false;
                }
                if log.data.len() != 64usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    listing_id: ethabi::decode(
                            &[ethabi::ParamType::Uint(256usize)],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'listing_id' from topic of type 'uint256': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    category: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    time: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for Erc721ListingRemoved {
            const NAME: &'static str = "ERC721ListingRemoved";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct EquipWearables {
            pub token_id: ethabi::Uint,
            pub old_wearables: [ethabi::Uint; 16usize],
            pub new_wearables: [ethabi::Uint; 16usize],
        }
        impl EquipWearables {
            const TOPIC_ID: [u8; 32] = [
                211u8,
                125u8,
                206u8,
                200u8,
                71u8,
                79u8,
                24u8,
                17u8,
                204u8,
                135u8,
                154u8,
                192u8,
                103u8,
                36u8,
                171u8,
                217u8,
                201u8,
                175u8,
                58u8,
                193u8,
                130u8,
                78u8,
                188u8,
                188u8,
                134u8,
                76u8,
                72u8,
                84u8,
                219u8,
                214u8,
                2u8,
                234u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 2usize {
                    return false;
                }
                if log.data.len() != 1024usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::FixedArray(
                                Box::new(ethabi::ParamType::Uint(16usize)),
                                16usize,
                            ),
                            ethabi::ParamType::FixedArray(
                                Box::new(ethabi::ParamType::Uint(16usize)),
                                16usize,
                            ),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    token_id: ethabi::decode(
                            &[ethabi::ParamType::Uint(256usize)],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'token_id' from topic of type 'uint256': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    old_wearables: {
                        let mut iter = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_array()
                            .expect(INTERNAL_ERR)
                            .into_iter()
                            .map(|inner| inner.into_uint().expect(INTERNAL_ERR));
                        [
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                        ]
                    },
                    new_wearables: {
                        let mut iter = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_array()
                            .expect(INTERNAL_ERR)
                            .into_iter()
                            .map(|inner| inner.into_uint().expect(INTERNAL_ERR));
                        [
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                        ]
                    },
                })
            }
        }
        impl substreams_ethereum::Event for EquipWearables {
            const NAME: &'static str = "EquipWearables";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct Erc20Deposited {
            pub token_id: ethabi::Uint,
            pub erc20_contract: Vec<u8>,
            pub from: Vec<u8>,
            pub to: Vec<u8>,
            pub deposit_amount: ethabi::Uint,
        }
        impl Erc20Deposited {
            const TOPIC_ID: [u8; 32] = [
                108u8,
                210u8,
                176u8,
                188u8,
                98u8,
                255u8,
                131u8,
                40u8,
                169u8,
                81u8,
                210u8,
                143u8,
                172u8,
                74u8,
                243u8,
                112u8,
                196u8,
                83u8,
                199u8,
                38u8,
                99u8,
                8u8,
                150u8,
                188u8,
                235u8,
                135u8,
                70u8,
                109u8,
                207u8,
                110u8,
                213u8,
                131u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 4usize {
                    return false;
                }
                if log.data.len() != 64usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Address, ethabi::ParamType::Uint(256usize)],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    token_id: ethabi::decode(
                            &[ethabi::ParamType::Uint(256usize)],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'token_id' from topic of type 'uint256': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    erc20_contract: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[2usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'erc20_contract' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    from: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[3usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'from' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    to: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    deposit_amount: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for Erc20Deposited {
            const NAME: &'static str = "Erc20Deposited";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct ExperienceTransfer {
            pub from_token_id: ethabi::Uint,
            pub to_token_id: ethabi::Uint,
            pub experience: ethabi::Uint,
        }
        impl ExperienceTransfer {
            const TOPIC_ID: [u8; 32] = [
                223u8,
                106u8,
                59u8,
                4u8,
                67u8,
                175u8,
                206u8,
                109u8,
                73u8,
                196u8,
                227u8,
                152u8,
                60u8,
                0u8,
                189u8,
                95u8,
                151u8,
                217u8,
                219u8,
                206u8,
                135u8,
                143u8,
                232u8,
                121u8,
                68u8,
                3u8,
                112u8,
                38u8,
                101u8,
                247u8,
                180u8,
                194u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 3usize {
                    return false;
                }
                if log.data.len() != 32usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    from_token_id: ethabi::decode(
                            &[ethabi::ParamType::Uint(256usize)],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'from_token_id' from topic of type 'uint256': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    to_token_id: ethabi::decode(
                            &[ethabi::ParamType::Uint(256usize)],
                            log.topics[2usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'to_token_id' from topic of type 'uint256': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    experience: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for ExperienceTransfer {
            const NAME: &'static str = "ExperienceTransfer";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct GameManagerAdded {
            pub game_manager: Vec<u8>,
            pub limit: ethabi::Uint,
            pub refresh_time: ethabi::Uint,
        }
        impl GameManagerAdded {
            const TOPIC_ID: [u8; 32] = [
                25u8,
                56u8,
                252u8,
                59u8,
                81u8,
                239u8,
                184u8,
                230u8,
                27u8,
                122u8,
                122u8,
                37u8,
                111u8,
                138u8,
                21u8,
                26u8,
                198u8,
                3u8,
                2u8,
                126u8,
                165u8,
                20u8,
                12u8,
                70u8,
                165u8,
                134u8,
                248u8,
                154u8,
                221u8,
                235u8,
                135u8,
                138u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 3usize {
                    return false;
                }
                if log.data.len() != 32usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    game_manager: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'game_manager' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    limit: ethabi::decode(
                            &[ethabi::ParamType::Uint(256usize)],
                            log.topics[2usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'limit' from topic of type 'uint256': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    refresh_time: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for GameManagerAdded {
            const NAME: &'static str = "GameManagerAdded";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct GameManagerRemoved {
            pub game_manager: Vec<u8>,
        }
        impl GameManagerRemoved {
            const TOPIC_ID: [u8; 32] = [
                109u8,
                232u8,
                47u8,
                130u8,
                116u8,
                113u8,
                22u8,
                10u8,
                242u8,
                115u8,
                204u8,
                220u8,
                3u8,
                206u8,
                139u8,
                227u8,
                164u8,
                15u8,
                47u8,
                159u8,
                231u8,
                250u8,
                65u8,
                16u8,
                205u8,
                2u8,
                121u8,
                151u8,
                23u8,
                187u8,
                61u8,
                145u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 2usize {
                    return false;
                }
                if log.data.len() != 0usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Ok(Self {
                    game_manager: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'game_manager' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                })
            }
        }
        impl substreams_ethereum::Event for GameManagerRemoved {
            const NAME: &'static str = "GameManagerRemoved";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct GotchiLendingAdd {
            pub listing_id: ethabi::Uint,
        }
        impl GotchiLendingAdd {
            const TOPIC_ID: [u8; 32] = [
                210u8,
                35u8,
                91u8,
                47u8,
                93u8,
                170u8,
                42u8,
                199u8,
                234u8,
                133u8,
                16u8,
                88u8,
                168u8,
                184u8,
                107u8,
                30u8,
                68u8,
                207u8,
                166u8,
                196u8,
                144u8,
                65u8,
                103u8,
                245u8,
                145u8,
                20u8,
                112u8,
                123u8,
                37u8,
                116u8,
                168u8,
                125u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 2usize {
                    return false;
                }
                if log.data.len() != 0usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Ok(Self {
                    listing_id: ethabi::decode(
                            &[ethabi::ParamType::Uint(32usize)],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'listing_id' from topic of type 'uint32': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for GotchiLendingAdd {
            const NAME: &'static str = "GotchiLendingAdd";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct GotchiLendingAdded {
            pub listing_id: ethabi::Uint,
            pub lender: Vec<u8>,
            pub token_id: ethabi::Uint,
            pub initial_cost: ethabi::Uint,
            pub period: ethabi::Uint,
            pub revenue_split: [ethabi::Uint; 3usize],
            pub original_owner: Vec<u8>,
            pub third_party: Vec<u8>,
            pub whitelist_id: ethabi::Uint,
            pub revenue_tokens: Vec<Vec<u8>>,
            pub time_created: ethabi::Uint,
        }
        impl GotchiLendingAdded {
            const TOPIC_ID: [u8; 32] = [
                64u8,
                27u8,
                67u8,
                35u8,
                58u8,
                39u8,
                40u8,
                12u8,
                212u8,
                127u8,
                42u8,
                185u8,
                103u8,
                32u8,
                217u8,
                125u8,
                93u8,
                120u8,
                40u8,
                170u8,
                70u8,
                204u8,
                38u8,
                214u8,
                91u8,
                117u8,
                147u8,
                1u8,
                92u8,
                2u8,
                172u8,
                179u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 4usize {
                    return false;
                }
                if log.data.len() < 352usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Uint(96usize),
                            ethabi::ParamType::Uint(32usize),
                            ethabi::ParamType::FixedArray(
                                Box::new(ethabi::ParamType::Uint(8usize)),
                                3usize,
                            ),
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Uint(32usize),
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Address),
                            ),
                            ethabi::ParamType::Uint(256usize),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    listing_id: ethabi::decode(
                            &[ethabi::ParamType::Uint(32usize)],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'listing_id' from topic of type 'uint32': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    lender: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[2usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'lender' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    token_id: ethabi::decode(
                            &[ethabi::ParamType::Uint(32usize)],
                            log.topics[3usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'token_id' from topic of type 'uint32': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    initial_cost: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    period: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    revenue_split: {
                        let mut iter = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_array()
                            .expect(INTERNAL_ERR)
                            .into_iter()
                            .map(|inner| inner.into_uint().expect(INTERNAL_ERR));
                        [
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                        ]
                    },
                    original_owner: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    third_party: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    whitelist_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    revenue_tokens: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| {
                            inner.into_address().expect(INTERNAL_ERR).as_bytes().to_vec()
                        })
                        .collect(),
                    time_created: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for GotchiLendingAdded {
            const NAME: &'static str = "GotchiLendingAdded";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct GotchiLendingCancel {
            pub listing_id: ethabi::Uint,
            pub time: ethabi::Uint,
        }
        impl GotchiLendingCancel {
            const TOPIC_ID: [u8; 32] = [
                88u8,
                191u8,
                134u8,
                215u8,
                50u8,
                28u8,
                169u8,
                111u8,
                104u8,
                115u8,
                251u8,
                210u8,
                70u8,
                130u8,
                192u8,
                106u8,
                20u8,
                27u8,
                39u8,
                187u8,
                60u8,
                125u8,
                170u8,
                66u8,
                57u8,
                62u8,
                177u8,
                119u8,
                214u8,
                218u8,
                141u8,
                65u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 2usize {
                    return false;
                }
                if log.data.len() != 32usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    listing_id: ethabi::decode(
                            &[ethabi::ParamType::Uint(32usize)],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'listing_id' from topic of type 'uint32': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    time: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for GotchiLendingCancel {
            const NAME: &'static str = "GotchiLendingCancel";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct GotchiLendingCanceled {
            pub listing_id: ethabi::Uint,
            pub lender: Vec<u8>,
            pub token_id: ethabi::Uint,
            pub initial_cost: ethabi::Uint,
            pub period: ethabi::Uint,
            pub revenue_split: [ethabi::Uint; 3usize],
            pub original_owner: Vec<u8>,
            pub third_party: Vec<u8>,
            pub whitelist_id: ethabi::Uint,
            pub revenue_tokens: Vec<Vec<u8>>,
            pub time_canceled: ethabi::Uint,
        }
        impl GotchiLendingCanceled {
            const TOPIC_ID: [u8; 32] = [
                247u8,
                236u8,
                13u8,
                222u8,
                168u8,
                186u8,
                197u8,
                170u8,
                89u8,
                187u8,
                241u8,
                40u8,
                147u8,
                114u8,
                23u8,
                12u8,
                26u8,
                95u8,
                234u8,
                66u8,
                156u8,
                137u8,
                215u8,
                250u8,
                42u8,
                65u8,
                189u8,
                33u8,
                116u8,
                20u8,
                35u8,
                128u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 4usize {
                    return false;
                }
                if log.data.len() < 352usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Uint(96usize),
                            ethabi::ParamType::Uint(32usize),
                            ethabi::ParamType::FixedArray(
                                Box::new(ethabi::ParamType::Uint(8usize)),
                                3usize,
                            ),
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Uint(32usize),
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Address),
                            ),
                            ethabi::ParamType::Uint(256usize),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    listing_id: ethabi::decode(
                            &[ethabi::ParamType::Uint(32usize)],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'listing_id' from topic of type 'uint32': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    lender: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[2usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'lender' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    token_id: ethabi::decode(
                            &[ethabi::ParamType::Uint(32usize)],
                            log.topics[3usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'token_id' from topic of type 'uint32': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    initial_cost: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    period: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    revenue_split: {
                        let mut iter = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_array()
                            .expect(INTERNAL_ERR)
                            .into_iter()
                            .map(|inner| inner.into_uint().expect(INTERNAL_ERR));
                        [
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                        ]
                    },
                    original_owner: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    third_party: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    whitelist_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    revenue_tokens: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| {
                            inner.into_address().expect(INTERNAL_ERR).as_bytes().to_vec()
                        })
                        .collect(),
                    time_canceled: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for GotchiLendingCanceled {
            const NAME: &'static str = "GotchiLendingCanceled";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct GotchiLendingClaim {
            pub listing_id: ethabi::Uint,
            pub token_addresses: Vec<Vec<u8>>,
            pub amounts: Vec<ethabi::Uint>,
        }
        impl GotchiLendingClaim {
            const TOPIC_ID: [u8; 32] = [
                27u8,
                229u8,
                130u8,
                20u8,
                150u8,
                42u8,
                140u8,
                189u8,
                126u8,
                248u8,
                94u8,
                247u8,
                183u8,
                141u8,
                234u8,
                2u8,
                219u8,
                37u8,
                80u8,
                203u8,
                152u8,
                141u8,
                184u8,
                83u8,
                49u8,
                164u8,
                93u8,
                99u8,
                3u8,
                46u8,
                23u8,
                114u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 2usize {
                    return false;
                }
                if log.data.len() < 128usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Address),
                            ),
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                            ),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    listing_id: ethabi::decode(
                            &[ethabi::ParamType::Uint(32usize)],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'listing_id' from topic of type 'uint32': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    token_addresses: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| {
                            inner.into_address().expect(INTERNAL_ERR).as_bytes().to_vec()
                        })
                        .collect(),
                    amounts: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
                })
            }
        }
        impl substreams_ethereum::Event for GotchiLendingClaim {
            const NAME: &'static str = "GotchiLendingClaim";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct GotchiLendingClaimed {
            pub listing_id: ethabi::Uint,
            pub lender: Vec<u8>,
            pub borrower: Vec<u8>,
            pub token_id: ethabi::Uint,
            pub initial_cost: ethabi::Uint,
            pub period: ethabi::Uint,
            pub revenue_split: [ethabi::Uint; 3usize],
            pub original_owner: Vec<u8>,
            pub third_party: Vec<u8>,
            pub whitelist_id: ethabi::Uint,
            pub revenue_tokens: Vec<Vec<u8>>,
            pub amounts: Vec<ethabi::Uint>,
            pub time_claimed: ethabi::Uint,
        }
        impl GotchiLendingClaimed {
            const TOPIC_ID: [u8; 32] = [
                238u8,
                49u8,
                199u8,
                184u8,
                1u8,
                122u8,
                57u8,
                44u8,
                227u8,
                105u8,
                7u8,
                21u8,
                250u8,
                235u8,
                92u8,
                69u8,
                236u8,
                70u8,
                220u8,
                52u8,
                131u8,
                234u8,
                37u8,
                212u8,
                182u8,
                46u8,
                111u8,
                104u8,
                12u8,
                216u8,
                120u8,
                179u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 4usize {
                    return false;
                }
                if log.data.len() < 448usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Uint(32usize),
                            ethabi::ParamType::Uint(96usize),
                            ethabi::ParamType::Uint(32usize),
                            ethabi::ParamType::FixedArray(
                                Box::new(ethabi::ParamType::Uint(8usize)),
                                3usize,
                            ),
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Uint(32usize),
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Address),
                            ),
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                            ),
                            ethabi::ParamType::Uint(256usize),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    listing_id: ethabi::decode(
                            &[ethabi::ParamType::Uint(32usize)],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'listing_id' from topic of type 'uint32': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    lender: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[2usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'lender' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    borrower: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[3usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'borrower' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    token_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    initial_cost: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    period: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    revenue_split: {
                        let mut iter = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_array()
                            .expect(INTERNAL_ERR)
                            .into_iter()
                            .map(|inner| inner.into_uint().expect(INTERNAL_ERR));
                        [
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                        ]
                    },
                    original_owner: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    third_party: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    whitelist_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    revenue_tokens: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| {
                            inner.into_address().expect(INTERNAL_ERR).as_bytes().to_vec()
                        })
                        .collect(),
                    amounts: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
                    time_claimed: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for GotchiLendingClaimed {
            const NAME: &'static str = "GotchiLendingClaimed";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct GotchiLendingEnd {
            pub listing_id: ethabi::Uint,
        }
        impl GotchiLendingEnd {
            const TOPIC_ID: [u8; 32] = [
                213u8,
                5u8,
                46u8,
                199u8,
                240u8,
                25u8,
                121u8,
                110u8,
                94u8,
                39u8,
                230u8,
                72u8,
                206u8,
                13u8,
                204u8,
                85u8,
                20u8,
                189u8,
                177u8,
                79u8,
                180u8,
                131u8,
                202u8,
                248u8,
                56u8,
                48u8,
                182u8,
                31u8,
                9u8,
                249u8,
                165u8,
                188u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 2usize {
                    return false;
                }
                if log.data.len() != 0usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Ok(Self {
                    listing_id: ethabi::decode(
                            &[ethabi::ParamType::Uint(32usize)],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'listing_id' from topic of type 'uint32': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for GotchiLendingEnd {
            const NAME: &'static str = "GotchiLendingEnd";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct GotchiLendingEnded {
            pub listing_id: ethabi::Uint,
            pub lender: Vec<u8>,
            pub borrower: Vec<u8>,
            pub token_id: ethabi::Uint,
            pub initial_cost: ethabi::Uint,
            pub period: ethabi::Uint,
            pub revenue_split: [ethabi::Uint; 3usize],
            pub original_owner: Vec<u8>,
            pub third_party: Vec<u8>,
            pub whitelist_id: ethabi::Uint,
            pub revenue_tokens: Vec<Vec<u8>>,
            pub time_ended: ethabi::Uint,
        }
        impl GotchiLendingEnded {
            const TOPIC_ID: [u8; 32] = [
                34u8,
                64u8,
                217u8,
                79u8,
                146u8,
                54u8,
                156u8,
                119u8,
                34u8,
                2u8,
                26u8,
                159u8,
                97u8,
                88u8,
                250u8,
                85u8,
                119u8,
                238u8,
                222u8,
                174u8,
                81u8,
                145u8,
                197u8,
                206u8,
                159u8,
                188u8,
                201u8,
                198u8,
                111u8,
                228u8,
                167u8,
                206u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 4usize {
                    return false;
                }
                if log.data.len() < 384usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Uint(32usize),
                            ethabi::ParamType::Uint(96usize),
                            ethabi::ParamType::Uint(32usize),
                            ethabi::ParamType::FixedArray(
                                Box::new(ethabi::ParamType::Uint(8usize)),
                                3usize,
                            ),
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Uint(32usize),
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Address),
                            ),
                            ethabi::ParamType::Uint(256usize),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    listing_id: ethabi::decode(
                            &[ethabi::ParamType::Uint(32usize)],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'listing_id' from topic of type 'uint32': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    lender: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[2usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'lender' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    borrower: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[3usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'borrower' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    token_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    initial_cost: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    period: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    revenue_split: {
                        let mut iter = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_array()
                            .expect(INTERNAL_ERR)
                            .into_iter()
                            .map(|inner| inner.into_uint().expect(INTERNAL_ERR));
                        [
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                        ]
                    },
                    original_owner: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    third_party: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    whitelist_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    revenue_tokens: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| {
                            inner.into_address().expect(INTERNAL_ERR).as_bytes().to_vec()
                        })
                        .collect(),
                    time_ended: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for GotchiLendingEnded {
            const NAME: &'static str = "GotchiLendingEnded";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct GotchiLendingExecute {
            pub listing_id: ethabi::Uint,
        }
        impl GotchiLendingExecute {
            const TOPIC_ID: [u8; 32] = [
                40u8,
                215u8,
                184u8,
                73u8,
                181u8,
                177u8,
                175u8,
                22u8,
                181u8,
                241u8,
                224u8,
                122u8,
                65u8,
                103u8,
                58u8,
                105u8,
                148u8,
                1u8,
                130u8,
                166u8,
                89u8,
                83u8,
                135u8,
                64u8,
                50u8,
                220u8,
                94u8,
                86u8,
                152u8,
                23u8,
                15u8,
                150u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 2usize {
                    return false;
                }
                if log.data.len() != 0usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Ok(Self {
                    listing_id: ethabi::decode(
                            &[ethabi::ParamType::Uint(32usize)],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'listing_id' from topic of type 'uint32': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for GotchiLendingExecute {
            const NAME: &'static str = "GotchiLendingExecute";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct GotchiLendingExecuted {
            pub listing_id: ethabi::Uint,
            pub lender: Vec<u8>,
            pub borrower: Vec<u8>,
            pub token_id: ethabi::Uint,
            pub initial_cost: ethabi::Uint,
            pub period: ethabi::Uint,
            pub revenue_split: [ethabi::Uint; 3usize],
            pub original_owner: Vec<u8>,
            pub third_party: Vec<u8>,
            pub whitelist_id: ethabi::Uint,
            pub revenue_tokens: Vec<Vec<u8>>,
            pub time_agreed: ethabi::Uint,
        }
        impl GotchiLendingExecuted {
            const TOPIC_ID: [u8; 32] = [
                103u8,
                114u8,
                91u8,
                59u8,
                88u8,
                54u8,
                53u8,
                72u8,
                103u8,
                229u8,
                126u8,
                245u8,
                151u8,
                132u8,
                135u8,
                11u8,
                82u8,
                160u8,
                176u8,
                127u8,
                83u8,
                199u8,
                240u8,
                180u8,
                31u8,
                239u8,
                199u8,
                38u8,
                34u8,
                95u8,
                91u8,
                251u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 4usize {
                    return false;
                }
                if log.data.len() < 384usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Uint(32usize),
                            ethabi::ParamType::Uint(96usize),
                            ethabi::ParamType::Uint(32usize),
                            ethabi::ParamType::FixedArray(
                                Box::new(ethabi::ParamType::Uint(8usize)),
                                3usize,
                            ),
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Uint(32usize),
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Address),
                            ),
                            ethabi::ParamType::Uint(256usize),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    listing_id: ethabi::decode(
                            &[ethabi::ParamType::Uint(32usize)],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'listing_id' from topic of type 'uint32': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    lender: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[2usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'lender' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    borrower: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[3usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'borrower' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    token_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    initial_cost: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    period: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    revenue_split: {
                        let mut iter = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_array()
                            .expect(INTERNAL_ERR)
                            .into_iter()
                            .map(|inner| inner.into_uint().expect(INTERNAL_ERR));
                        [
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                        ]
                    },
                    original_owner: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    third_party: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    whitelist_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    revenue_tokens: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| {
                            inner.into_address().expect(INTERNAL_ERR).as_bytes().to_vec()
                        })
                        .collect(),
                    time_agreed: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for GotchiLendingExecuted {
            const NAME: &'static str = "GotchiLendingExecuted";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct GrantExperience {
            pub token_ids: Vec<ethabi::Uint>,
            pub xp_values: Vec<ethabi::Uint>,
        }
        impl GrantExperience {
            const TOPIC_ID: [u8; 32] = [
                147u8,
                115u8,
                250u8,
                218u8,
                128u8,
                187u8,
                79u8,
                226u8,
                83u8,
                210u8,
                243u8,
                182u8,
                110u8,
                18u8,
                148u8,
                196u8,
                2u8,
                112u8,
                134u8,
                69u8,
                92u8,
                126u8,
                191u8,
                128u8,
                196u8,
                88u8,
                235u8,
                69u8,
                212u8,
                123u8,
                203u8,
                96u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 1usize {
                    return false;
                }
                if log.data.len() < 128usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                            ),
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                            ),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    token_ids: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
                    xp_values: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
                })
            }
        }
        impl substreams_ethereum::Event for GrantExperience {
            const NAME: &'static str = "GrantExperience";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct IncreaseStake {
            pub token_id: ethabi::Uint,
            pub stake_amount: ethabi::Uint,
        }
        impl IncreaseStake {
            const TOPIC_ID: [u8; 32] = [
                200u8,
                63u8,
                74u8,
                79u8,
                47u8,
                134u8,
                236u8,
                186u8,
                60u8,
                96u8,
                6u8,
                62u8,
                141u8,
                37u8,
                44u8,
                72u8,
                128u8,
                145u8,
                196u8,
                13u8,
                139u8,
                72u8,
                74u8,
                48u8,
                18u8,
                35u8,
                224u8,
                253u8,
                28u8,
                159u8,
                106u8,
                243u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 2usize {
                    return false;
                }
                if log.data.len() != 32usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    token_id: ethabi::decode(
                            &[ethabi::ParamType::Uint(256usize)],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'token_id' from topic of type 'uint256': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    stake_amount: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for IncreaseStake {
            const NAME: &'static str = "IncreaseStake";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct ItemManagerAdded {
            pub new_item_manager: Vec<u8>,
        }
        impl ItemManagerAdded {
            const TOPIC_ID: [u8; 32] = [
                242u8,
                193u8,
                59u8,
                143u8,
                211u8,
                196u8,
                0u8,
                80u8,
                239u8,
                177u8,
                137u8,
                2u8,
                183u8,
                176u8,
                172u8,
                55u8,
                25u8,
                65u8,
                119u8,
                85u8,
                150u8,
                50u8,
                128u8,
                127u8,
                62u8,
                132u8,
                141u8,
                48u8,
                47u8,
                117u8,
                35u8,
                136u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 2usize {
                    return false;
                }
                if log.data.len() != 0usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Ok(Self {
                    new_item_manager: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'new_item_manager' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                })
            }
        }
        impl substreams_ethereum::Event for ItemManagerAdded {
            const NAME: &'static str = "ItemManagerAdded";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct ItemManagerRemoved {
            pub item_manager: Vec<u8>,
        }
        impl ItemManagerRemoved {
            const TOPIC_ID: [u8; 32] = [
                204u8,
                53u8,
                169u8,
                86u8,
                80u8,
                25u8,
                155u8,
                51u8,
                145u8,
                210u8,
                139u8,
                1u8,
                28u8,
                67u8,
                130u8,
                217u8,
                76u8,
                102u8,
                169u8,
                182u8,
                78u8,
                55u8,
                182u8,
                26u8,
                55u8,
                119u8,
                79u8,
                104u8,
                49u8,
                30u8,
                239u8,
                46u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 2usize {
                    return false;
                }
                if log.data.len() != 0usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Ok(Self {
                    item_manager: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'item_manager' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                })
            }
        }
        impl substreams_ethereum::Event for ItemManagerRemoved {
            const NAME: &'static str = "ItemManagerRemoved";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct ItemModifiersSet {
            pub wearable_id: ethabi::Uint,
            pub trait_modifiers: [EthBigInt; 6usize],
            pub rarity_score_modifier: ethabi::Uint,
        }
        impl ItemModifiersSet {
            const TOPIC_ID: [u8; 32] = [
                197u8,
                185u8,
                162u8,
                153u8,
                243u8,
                213u8,
                255u8,
                49u8,
                186u8,
                69u8,
                11u8,
                37u8,
                122u8,
                196u8,
                160u8,
                166u8,
                51u8,
                182u8,
                64u8,
                199u8,
                17u8,
                236u8,
                209u8,
                158u8,
                126u8,
                118u8,
                215u8,
                216u8,
                44u8,
                80u8,
                111u8,
                218u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 1usize {
                    return false;
                }
                if log.data.len() != 256usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::FixedArray(
                                Box::new(ethabi::ParamType::Int(8usize)),
                                6usize,
                            ),
                            ethabi::ParamType::Uint(8usize),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    wearable_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    trait_modifiers: {
                        let mut iter = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_array()
                            .expect(INTERNAL_ERR)
                            .into_iter()
                            .map(|inner| {
                                let mut v = [0 as u8; 32];
                                inner
                                    .into_int()
                                    .expect(INTERNAL_ERR)
                                    .to_big_endian(v.as_mut_slice());
                                EthBigInt::new(v.into())
                            });
                        [
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                        ]
                    },
                    rarity_score_modifier: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for ItemModifiersSet {
            const NAME: &'static str = "ItemModifiersSet";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct ItemTypeMaxQuantity {
            pub item_ids: Vec<ethabi::Uint>,
            pub max_quanities: Vec<ethabi::Uint>,
        }
        impl ItemTypeMaxQuantity {
            const TOPIC_ID: [u8; 32] = [
                237u8,
                13u8,
                126u8,
                0u8,
                158u8,
                148u8,
                189u8,
                46u8,
                32u8,
                247u8,
                102u8,
                40u8,
                52u8,
                190u8,
                158u8,
                111u8,
                131u8,
                106u8,
                235u8,
                165u8,
                47u8,
                39u8,
                224u8,
                114u8,
                238u8,
                119u8,
                223u8,
                40u8,
                18u8,
                85u8,
                81u8,
                160u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 1usize {
                    return false;
                }
                if log.data.len() < 128usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                            ),
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                            ),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    item_ids: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
                    max_quanities: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
                })
            }
        }
        impl substreams_ethereum::Event for ItemTypeMaxQuantity {
            const NAME: &'static str = "ItemTypeMaxQuantity";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct LockAavegotchi {
            pub token_id: ethabi::Uint,
            pub time: ethabi::Uint,
        }
        impl LockAavegotchi {
            const TOPIC_ID: [u8; 32] = [
                76u8,
                254u8,
                156u8,
                119u8,
                221u8,
                184u8,
                137u8,
                152u8,
                117u8,
                2u8,
                86u8,
                39u8,
                154u8,
                74u8,
                105u8,
                75u8,
                18u8,
                93u8,
                74u8,
                168u8,
                132u8,
                73u8,
                244u8,
                101u8,
                72u8,
                9u8,
                230u8,
                10u8,
                230u8,
                192u8,
                27u8,
                172u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 2usize {
                    return false;
                }
                if log.data.len() != 32usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    token_id: ethabi::decode(
                            &[ethabi::ParamType::Uint(256usize)],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'token_id' from topic of type 'uint256': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    time: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for LockAavegotchi {
            const NAME: &'static str = "LockAavegotchi";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct MetaTransactionExecuted {
            pub user_address: Vec<u8>,
            pub relayer_address: Vec<u8>,
            pub function_signature: Vec<u8>,
        }
        impl MetaTransactionExecuted {
            const TOPIC_ID: [u8; 32] = [
                88u8,
                69u8,
                137u8,
                33u8,
                50u8,
                148u8,
                104u8,
                80u8,
                70u8,
                11u8,
                255u8,
                90u8,
                0u8,
                131u8,
                247u8,
                16u8,
                49u8,
                188u8,
                91u8,
                249u8,
                170u8,
                220u8,
                212u8,
                15u8,
                29u8,
                231u8,
                148u8,
                35u8,
                234u8,
                201u8,
                177u8,
                11u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 1usize {
                    return false;
                }
                if log.data.len() < 128usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Bytes,
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    user_address: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    relayer_address: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    function_signature: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_bytes()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for MetaTransactionExecuted {
            const NAME: &'static str = "MetaTransactionExecuted";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct MigrateVouchers {
            pub owner: Vec<u8>,
            pub ids: Vec<ethabi::Uint>,
            pub values: Vec<ethabi::Uint>,
        }
        impl MigrateVouchers {
            const TOPIC_ID: [u8; 32] = [
                166u8,
                205u8,
                219u8,
                80u8,
                165u8,
                19u8,
                127u8,
                69u8,
                0u8,
                144u8,
                153u8,
                39u8,
                56u8,
                151u8,
                82u8,
                217u8,
                88u8,
                197u8,
                194u8,
                215u8,
                122u8,
                57u8,
                85u8,
                32u8,
                9u8,
                142u8,
                113u8,
                222u8,
                71u8,
                36u8,
                179u8,
                22u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 2usize {
                    return false;
                }
                if log.data.len() < 128usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                            ),
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                            ),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    owner: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'owner' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    ids: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
                    values: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
                })
            }
        }
        impl substreams_ethereum::Event for MigrateVouchers {
            const NAME: &'static str = "MigrateVouchers";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct MintPortals {
            pub from: Vec<u8>,
            pub to: Vec<u8>,
            pub token_id: ethabi::Uint,
            pub num_aavegotchis_to_purchase: ethabi::Uint,
            pub haunt_id: ethabi::Uint,
        }
        impl MintPortals {
            const TOPIC_ID: [u8; 32] = [
                55u8,
                237u8,
                2u8,
                46u8,
                106u8,
                158u8,
                150u8,
                180u8,
                142u8,
                142u8,
                103u8,
                74u8,
                86u8,
                2u8,
                75u8,
                171u8,
                123u8,
                168u8,
                83u8,
                216u8,
                78u8,
                238u8,
                255u8,
                108u8,
                29u8,
                127u8,
                205u8,
                186u8,
                52u8,
                90u8,
                157u8,
                46u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 3usize {
                    return false;
                }
                if log.data.len() != 96usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    from: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'from' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    to: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[2usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'to' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    token_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    num_aavegotchis_to_purchase: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    haunt_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for MintPortals {
            const NAME: &'static str = "MintPortals";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct OpenPortals {
            pub token_ids: Vec<ethabi::Uint>,
        }
        impl OpenPortals {
            const TOPIC_ID: [u8; 32] = [
                174u8,
                102u8,
                239u8,
                200u8,
                248u8,
                224u8,
                240u8,
                144u8,
                205u8,
                167u8,
                2u8,
                243u8,
                145u8,
                203u8,
                120u8,
                217u8,
                142u8,
                219u8,
                201u8,
                61u8,
                102u8,
                46u8,
                40u8,
                118u8,
                165u8,
                198u8,
                214u8,
                242u8,
                87u8,
                217u8,
                89u8,
                185u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 1usize {
                    return false;
                }
                if log.data.len() < 64usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                            ),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    token_ids: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
                })
            }
        }
        impl substreams_ethereum::Event for OpenPortals {
            const NAME: &'static str = "OpenPortals";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct OwnershipTransferred {
            pub previous_owner: Vec<u8>,
            pub new_owner: Vec<u8>,
        }
        impl OwnershipTransferred {
            const TOPIC_ID: [u8; 32] = [
                139u8,
                224u8,
                7u8,
                156u8,
                83u8,
                22u8,
                89u8,
                20u8,
                19u8,
                68u8,
                205u8,
                31u8,
                208u8,
                164u8,
                242u8,
                132u8,
                25u8,
                73u8,
                127u8,
                151u8,
                34u8,
                163u8,
                218u8,
                175u8,
                227u8,
                180u8,
                24u8,
                111u8,
                107u8,
                100u8,
                87u8,
                224u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 3usize {
                    return false;
                }
                if log.data.len() != 0usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Ok(Self {
                    previous_owner: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'previous_owner' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    new_owner: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[2usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'new_owner' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                })
            }
        }
        impl substreams_ethereum::Event for OwnershipTransferred {
            const NAME: &'static str = "OwnershipTransferred";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct PetOperatorApprovalForAll {
            pub owner: Vec<u8>,
            pub operator: Vec<u8>,
            pub approved: bool,
        }
        impl PetOperatorApprovalForAll {
            const TOPIC_ID: [u8; 32] = [
                25u8,
                3u8,
                180u8,
                22u8,
                125u8,
                9u8,
                11u8,
                76u8,
                80u8,
                111u8,
                177u8,
                64u8,
                58u8,
                90u8,
                233u8,
                249u8,
                180u8,
                3u8,
                189u8,
                62u8,
                138u8,
                63u8,
                235u8,
                51u8,
                200u8,
                8u8,
                45u8,
                26u8,
                10u8,
                144u8,
                83u8,
                3u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 3usize {
                    return false;
                }
                if log.data.len() != 32usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Bool],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    owner: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'owner' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    operator: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[2usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'operator' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    approved: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_bool()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for PetOperatorApprovalForAll {
            const NAME: &'static str = "PetOperatorApprovalForAll";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct PortalOpened {
            pub token_id: ethabi::Uint,
        }
        impl PortalOpened {
            const TOPIC_ID: [u8; 32] = [
                181u8,
                130u8,
                74u8,
                255u8,
                109u8,
                206u8,
                15u8,
                141u8,
                25u8,
                146u8,
                99u8,
                80u8,
                6u8,
                252u8,
                160u8,
                158u8,
                217u8,
                0u8,
                154u8,
                122u8,
                77u8,
                208u8,
                170u8,
                46u8,
                134u8,
                120u8,
                89u8,
                77u8,
                135u8,
                234u8,
                222u8,
                126u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 2usize {
                    return false;
                }
                if log.data.len() != 0usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Ok(Self {
                    token_id: ethabi::decode(
                            &[ethabi::ParamType::Uint(256usize)],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'token_id' from topic of type 'uint256': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for PortalOpened {
            const NAME: &'static str = "PortalOpened";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct PurchaseItemsWithGhst {
            pub buyer: Vec<u8>,
            pub to: Vec<u8>,
            pub item_ids: Vec<ethabi::Uint>,
            pub quantities: Vec<ethabi::Uint>,
            pub total_price: ethabi::Uint,
        }
        impl PurchaseItemsWithGhst {
            const TOPIC_ID: [u8; 32] = [
                127u8,
                31u8,
                177u8,
                149u8,
                153u8,
                221u8,
                20u8,
                201u8,
                204u8,
                218u8,
                9u8,
                31u8,
                142u8,
                14u8,
                115u8,
                159u8,
                45u8,
                8u8,
                196u8,
                102u8,
                61u8,
                222u8,
                85u8,
                222u8,
                114u8,
                251u8,
                34u8,
                230u8,
                178u8,
                97u8,
                185u8,
                241u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 3usize {
                    return false;
                }
                if log.data.len() < 160usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                            ),
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                            ),
                            ethabi::ParamType::Uint(256usize),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    buyer: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'buyer' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    to: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[2usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'to' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    item_ids: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
                    quantities: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
                    total_price: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for PurchaseItemsWithGhst {
            const NAME: &'static str = "PurchaseItemsWithGhst";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct PurchaseItemsWithVouchers {
            pub buyer: Vec<u8>,
            pub to: Vec<u8>,
            pub item_ids: Vec<ethabi::Uint>,
            pub quantities: Vec<ethabi::Uint>,
        }
        impl PurchaseItemsWithVouchers {
            const TOPIC_ID: [u8; 32] = [
                175u8,
                189u8,
                247u8,
                213u8,
                131u8,
                6u8,
                220u8,
                128u8,
                45u8,
                247u8,
                213u8,
                192u8,
                222u8,
                117u8,
                46u8,
                37u8,
                34u8,
                178u8,
                186u8,
                203u8,
                21u8,
                11u8,
                125u8,
                128u8,
                37u8,
                166u8,
                240u8,
                71u8,
                155u8,
                248u8,
                89u8,
                198u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 3usize {
                    return false;
                }
                if log.data.len() < 128usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                            ),
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                            ),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    buyer: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'buyer' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    to: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[2usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'to' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    item_ids: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
                    quantities: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
                })
            }
        }
        impl substreams_ethereum::Event for PurchaseItemsWithVouchers {
            const NAME: &'static str = "PurchaseItemsWithVouchers";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct PurchaseTransferItemsWithGhst {
            pub buyer: Vec<u8>,
            pub to: Vec<u8>,
            pub item_ids: Vec<ethabi::Uint>,
            pub quantities: Vec<ethabi::Uint>,
            pub total_price: ethabi::Uint,
        }
        impl PurchaseTransferItemsWithGhst {
            const TOPIC_ID: [u8; 32] = [
                133u8,
                139u8,
                56u8,
                38u8,
                231u8,
                93u8,
                18u8,
                201u8,
                159u8,
                162u8,
                114u8,
                71u8,
                81u8,
                155u8,
                222u8,
                164u8,
                96u8,
                31u8,
                48u8,
                122u8,
                140u8,
                33u8,
                12u8,
                17u8,
                101u8,
                67u8,
                229u8,
                116u8,
                146u8,
                209u8,
                216u8,
                179u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 3usize {
                    return false;
                }
                if log.data.len() < 160usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                            ),
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                            ),
                            ethabi::ParamType::Uint(256usize),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    buyer: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'buyer' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    to: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[2usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'to' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    item_ids: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
                    quantities: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
                    total_price: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for PurchaseTransferItemsWithGhst {
            const NAME: &'static str = "PurchaseTransferItemsWithGhst";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct RemoveExperience {
            pub token_ids: Vec<ethabi::Uint>,
            pub xp_values: Vec<ethabi::Uint>,
        }
        impl RemoveExperience {
            const TOPIC_ID: [u8; 32] = [
                229u8,
                203u8,
                16u8,
                142u8,
                169u8,
                41u8,
                112u8,
                139u8,
                162u8,
                9u8,
                82u8,
                24u8,
                44u8,
                215u8,
                223u8,
                161u8,
                187u8,
                173u8,
                248u8,
                82u8,
                186u8,
                38u8,
                227u8,
                36u8,
                51u8,
                98u8,
                67u8,
                153u8,
                163u8,
                114u8,
                230u8,
                239u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 1usize {
                    return false;
                }
                if log.data.len() < 128usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                            ),
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                            ),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    token_ids: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
                    xp_values: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
                })
            }
        }
        impl substreams_ethereum::Event for RemoveExperience {
            const NAME: &'static str = "RemoveExperience";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct SetAavegotchiName {
            pub token_id: ethabi::Uint,
            pub old_name: String,
            pub new_name: String,
        }
        impl SetAavegotchiName {
            const TOPIC_ID: [u8; 32] = [
                158u8,
                148u8,
                174u8,
                90u8,
                93u8,
                140u8,
                77u8,
                195u8,
                96u8,
                13u8,
                116u8,
                158u8,
                161u8,
                119u8,
                14u8,
                228u8,
                104u8,
                209u8,
                218u8,
                53u8,
                231u8,
                165u8,
                151u8,
                30u8,
                179u8,
                168u8,
                137u8,
                176u8,
                208u8,
                36u8,
                37u8,
                4u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 2usize {
                    return false;
                }
                if log.data.len() < 128usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::String, ethabi::ParamType::String],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    token_id: ethabi::decode(
                            &[ethabi::ParamType::Uint(256usize)],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'token_id' from topic of type 'uint256': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    old_name: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_string()
                        .expect(INTERNAL_ERR),
                    new_name: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_string()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for SetAavegotchiName {
            const NAME: &'static str = "SetAavegotchiName";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct SetBatchId {
            pub batch_id: ethabi::Uint,
            pub token_ids: Vec<ethabi::Uint>,
        }
        impl SetBatchId {
            const TOPIC_ID: [u8; 32] = [
                177u8,
                229u8,
                219u8,
                44u8,
                234u8,
                148u8,
                90u8,
                60u8,
                186u8,
                60u8,
                175u8,
                35u8,
                218u8,
                65u8,
                64u8,
                151u8,
                111u8,
                31u8,
                52u8,
                72u8,
                181u8,
                159u8,
                160u8,
                192u8,
                9u8,
                196u8,
                121u8,
                45u8,
                108u8,
                23u8,
                101u8,
                36u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 2usize {
                    return false;
                }
                if log.data.len() < 64usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                            ),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    batch_id: ethabi::decode(
                            &[ethabi::ParamType::Uint(256usize)],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'batch_id' from topic of type 'uint256': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    token_ids: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
                })
            }
        }
        impl substreams_ethereum::Event for SetBatchId {
            const NAME: &'static str = "SetBatchId";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct SpendSkillpoints {
            pub token_id: ethabi::Uint,
            pub values: [EthBigInt; 4usize],
        }
        impl SpendSkillpoints {
            const TOPIC_ID: [u8; 32] = [
                75u8,
                146u8,
                237u8,
                13u8,
                176u8,
                117u8,
                142u8,
                126u8,
                215u8,
                54u8,
                116u8,
                214u8,
                145u8,
                91u8,
                213u8,
                50u8,
                165u8,
                83u8,
                34u8,
                170u8,
                27u8,
                19u8,
                171u8,
                234u8,
                173u8,
                20u8,
                172u8,
                243u8,
                149u8,
                75u8,
                26u8,
                140u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 2usize {
                    return false;
                }
                if log.data.len() != 128usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::FixedArray(
                                Box::new(ethabi::ParamType::Int(16usize)),
                                4usize,
                            ),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    token_id: ethabi::decode(
                            &[ethabi::ParamType::Uint(256usize)],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'token_id' from topic of type 'uint256': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    values: {
                        let mut iter = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_array()
                            .expect(INTERNAL_ERR)
                            .into_iter()
                            .map(|inner| {
                                let mut v = [0 as u8; 32];
                                inner
                                    .into_int()
                                    .expect(INTERNAL_ERR)
                                    .to_big_endian(v.as_mut_slice());
                                EthBigInt::new(v.into())
                            });
                        [
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                        ]
                    },
                })
            }
        }
        impl substreams_ethereum::Event for SpendSkillpoints {
            const NAME: &'static str = "SpendSkillpoints";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct Transfer {
            pub from: Vec<u8>,
            pub to: Vec<u8>,
            pub token_id: ethabi::Uint,
        }
        impl Transfer {
            const TOPIC_ID: [u8; 32] = [
                221u8,
                242u8,
                82u8,
                173u8,
                27u8,
                226u8,
                200u8,
                155u8,
                105u8,
                194u8,
                176u8,
                104u8,
                252u8,
                55u8,
                141u8,
                170u8,
                149u8,
                43u8,
                167u8,
                241u8,
                99u8,
                196u8,
                161u8,
                22u8,
                40u8,
                245u8,
                90u8,
                77u8,
                245u8,
                35u8,
                179u8,
                239u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 4usize {
                    return false;
                }
                if log.data.len() != 0usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Ok(Self {
                    from: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'from' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    to: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[2usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'to' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    token_id: ethabi::decode(
                            &[ethabi::ParamType::Uint(256usize)],
                            log.topics[3usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'token_id' from topic of type 'uint256': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for Transfer {
            const NAME: &'static str = "Transfer";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct TransferBatch {
            pub operator: Vec<u8>,
            pub from: Vec<u8>,
            pub to: Vec<u8>,
            pub ids: Vec<ethabi::Uint>,
            pub values: Vec<ethabi::Uint>,
        }
        impl TransferBatch {
            const TOPIC_ID: [u8; 32] = [
                74u8,
                57u8,
                220u8,
                6u8,
                212u8,
                192u8,
                219u8,
                198u8,
                75u8,
                112u8,
                175u8,
                144u8,
                253u8,
                105u8,
                138u8,
                35u8,
                58u8,
                81u8,
                138u8,
                165u8,
                208u8,
                126u8,
                89u8,
                93u8,
                152u8,
                59u8,
                140u8,
                5u8,
                38u8,
                200u8,
                247u8,
                251u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 4usize {
                    return false;
                }
                if log.data.len() < 128usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                            ),
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                            ),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    operator: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'operator' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    from: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[2usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'from' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    to: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[3usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'to' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    ids: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
                    values: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
                })
            }
        }
        impl substreams_ethereum::Event for TransferBatch {
            const NAME: &'static str = "TransferBatch";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct TransferEscrow {
            pub token_id: ethabi::Uint,
            pub erc20_contract: Vec<u8>,
            pub from: Vec<u8>,
            pub to: Vec<u8>,
            pub transfer_amount: ethabi::Uint,
        }
        impl TransferEscrow {
            const TOPIC_ID: [u8; 32] = [
                58u8,
                221u8,
                73u8,
                119u8,
                19u8,
                202u8,
                189u8,
                220u8,
                155u8,
                63u8,
                61u8,
                153u8,
                99u8,
                104u8,
                138u8,
                98u8,
                153u8,
                21u8,
                167u8,
                6u8,
                1u8,
                107u8,
                11u8,
                56u8,
                192u8,
                138u8,
                126u8,
                103u8,
                91u8,
                141u8,
                167u8,
                174u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 4usize {
                    return false;
                }
                if log.data.len() != 64usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Address, ethabi::ParamType::Uint(256usize)],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    token_id: ethabi::decode(
                            &[ethabi::ParamType::Uint(256usize)],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'token_id' from topic of type 'uint256': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    erc20_contract: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[2usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'erc20_contract' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    to: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[3usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'to' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    from: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    transfer_amount: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for TransferEscrow {
            const NAME: &'static str = "TransferEscrow";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct TransferFromParent {
            pub from_contract: Vec<u8>,
            pub from_token_id: ethabi::Uint,
            pub token_type_id: ethabi::Uint,
            pub value: ethabi::Uint,
        }
        impl TransferFromParent {
            const TOPIC_ID: [u8; 32] = [
                75u8,
                40u8,
                126u8,
                54u8,
                160u8,
                83u8,
                29u8,
                168u8,
                204u8,
                238u8,
                48u8,
                61u8,
                225u8,
                39u8,
                199u8,
                120u8,
                48u8,
                47u8,
                72u8,
                216u8,
                252u8,
                149u8,
                105u8,
                177u8,
                254u8,
                29u8,
                203u8,
                157u8,
                100u8,
                70u8,
                92u8,
                109u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 4usize {
                    return false;
                }
                if log.data.len() != 32usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    from_contract: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'from_contract' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    from_token_id: ethabi::decode(
                            &[ethabi::ParamType::Uint(256usize)],
                            log.topics[2usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'from_token_id' from topic of type 'uint256': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    token_type_id: ethabi::decode(
                            &[ethabi::ParamType::Uint(256usize)],
                            log.topics[3usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'token_type_id' from topic of type 'uint256': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    value: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for TransferFromParent {
            const NAME: &'static str = "TransferFromParent";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct TransferSingle {
            pub operator: Vec<u8>,
            pub from: Vec<u8>,
            pub to: Vec<u8>,
            pub id: ethabi::Uint,
            pub value: ethabi::Uint,
        }
        impl TransferSingle {
            const TOPIC_ID: [u8; 32] = [
                195u8,
                213u8,
                129u8,
                104u8,
                197u8,
                174u8,
                115u8,
                151u8,
                115u8,
                29u8,
                6u8,
                61u8,
                91u8,
                191u8,
                61u8,
                101u8,
                120u8,
                84u8,
                66u8,
                115u8,
                67u8,
                244u8,
                192u8,
                131u8,
                36u8,
                15u8,
                122u8,
                172u8,
                170u8,
                45u8,
                15u8,
                98u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 4usize {
                    return false;
                }
                if log.data.len() != 64usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    operator: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'operator' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    from: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[2usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'from' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    to: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[3usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'to' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    value: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for TransferSingle {
            const NAME: &'static str = "TransferSingle";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct TransferToParent1 {
            pub to_contract: Vec<u8>,
            pub to_token_id: ethabi::Uint,
            pub token_type_id: ethabi::Uint,
            pub value: ethabi::Uint,
        }
        impl TransferToParent1 {
            const TOPIC_ID: [u8; 32] = [
                232u8,
                40u8,
                16u8,
                9u8,
                157u8,
                24u8,
                43u8,
                37u8,
                226u8,
                62u8,
                137u8,
                87u8,
                21u8,
                149u8,
                77u8,
                110u8,
                123u8,
                65u8,
                134u8,
                122u8,
                2u8,
                226u8,
                198u8,
                147u8,
                204u8,
                230u8,
                138u8,
                143u8,
                37u8,
                14u8,
                67u8,
                211u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 4usize {
                    return false;
                }
                if log.data.len() != 32usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    to_contract: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'to_contract' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    to_token_id: ethabi::decode(
                            &[ethabi::ParamType::Uint(256usize)],
                            log.topics[2usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'to_token_id' from topic of type 'uint256': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    token_type_id: ethabi::decode(
                            &[ethabi::ParamType::Uint(256usize)],
                            log.topics[3usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'token_type_id' from topic of type 'uint256': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    value: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for TransferToParent1 {
            const NAME: &'static str = "TransferToParent1";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct TransferToParent2 {
            pub to_contract: Vec<u8>,
            pub to_token_id: ethabi::Uint,
            pub token_type_id: ethabi::Uint,
            pub value: ethabi::Uint,
        }
        impl TransferToParent2 {
            const TOPIC_ID: [u8; 32] = [
                232u8,
                40u8,
                16u8,
                9u8,
                157u8,
                24u8,
                43u8,
                37u8,
                226u8,
                62u8,
                137u8,
                87u8,
                21u8,
                149u8,
                77u8,
                110u8,
                123u8,
                65u8,
                134u8,
                122u8,
                2u8,
                226u8,
                198u8,
                147u8,
                204u8,
                230u8,
                138u8,
                143u8,
                37u8,
                14u8,
                67u8,
                211u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 4usize {
                    return false;
                }
                if log.data.len() != 32usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    to_contract: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'to_contract' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    to_token_id: ethabi::decode(
                            &[ethabi::ParamType::Uint(256usize)],
                            log.topics[2usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'to_token_id' from topic of type 'uint256': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    token_type_id: ethabi::decode(
                            &[ethabi::ParamType::Uint(256usize)],
                            log.topics[3usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'token_type_id' from topic of type 'uint256': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    value: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for TransferToParent2 {
            const NAME: &'static str = "TransferToParent2";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct Uri {
            pub value: String,
            pub id: ethabi::Uint,
        }
        impl Uri {
            const TOPIC_ID: [u8; 32] = [
                107u8,
                183u8,
                255u8,
                112u8,
                134u8,
                25u8,
                186u8,
                6u8,
                16u8,
                203u8,
                162u8,
                149u8,
                165u8,
                133u8,
                146u8,
                224u8,
                69u8,
                29u8,
                238u8,
                38u8,
                34u8,
                147u8,
                140u8,
                135u8,
                85u8,
                102u8,
                118u8,
                136u8,
                218u8,
                243u8,
                82u8,
                155u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 2usize {
                    return false;
                }
                if log.data.len() < 64usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::String],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    id: ethabi::decode(
                            &[ethabi::ParamType::Uint(256usize)],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'id' from topic of type 'uint256': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    value: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_string()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for Uri {
            const NAME: &'static str = "URI";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct UnLockAavegotchi {
            pub token_id: ethabi::Uint,
            pub time: ethabi::Uint,
        }
        impl UnLockAavegotchi {
            const TOPIC_ID: [u8; 32] = [
                94u8,
                113u8,
                202u8,
                197u8,
                166u8,
                64u8,
                94u8,
                48u8,
                8u8,
                172u8,
                67u8,
                73u8,
                175u8,
                55u8,
                217u8,
                8u8,
                191u8,
                216u8,
                78u8,
                18u8,
                231u8,
                111u8,
                113u8,
                232u8,
                234u8,
                39u8,
                2u8,
                16u8,
                254u8,
                38u8,
                53u8,
                166u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 2usize {
                    return false;
                }
                if log.data.len() != 32usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    token_id: ethabi::decode(
                            &[ethabi::ParamType::Uint(256usize)],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'token_id' from topic of type 'uint256': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    time: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for UnLockAavegotchi {
            const NAME: &'static str = "UnLockAavegotchi";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct UpdateCollateralModifiers {
            pub old_modifiers: [EthBigInt; 6usize],
            pub new_modifiers: [EthBigInt; 6usize],
        }
        impl UpdateCollateralModifiers {
            const TOPIC_ID: [u8; 32] = [
                90u8,
                211u8,
                231u8,
                218u8,
                172u8,
                82u8,
                110u8,
                220u8,
                203u8,
                59u8,
                190u8,
                83u8,
                234u8,
                109u8,
                203u8,
                198u8,
                163u8,
                211u8,
                2u8,
                129u8,
                232u8,
                187u8,
                245u8,
                99u8,
                142u8,
                244u8,
                76u8,
                59u8,
                166u8,
                104u8,
                41u8,
                185u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 1usize {
                    return false;
                }
                if log.data.len() != 384usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::FixedArray(
                                Box::new(ethabi::ParamType::Int(16usize)),
                                6usize,
                            ),
                            ethabi::ParamType::FixedArray(
                                Box::new(ethabi::ParamType::Int(16usize)),
                                6usize,
                            ),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    old_modifiers: {
                        let mut iter = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_array()
                            .expect(INTERNAL_ERR)
                            .into_iter()
                            .map(|inner| {
                                let mut v = [0 as u8; 32];
                                inner
                                    .into_int()
                                    .expect(INTERNAL_ERR)
                                    .to_big_endian(v.as_mut_slice());
                                EthBigInt::new(v.into())
                            });
                        [
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                        ]
                    },
                    new_modifiers: {
                        let mut iter = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_array()
                            .expect(INTERNAL_ERR)
                            .into_iter()
                            .map(|inner| {
                                let mut v = [0 as u8; 32];
                                inner
                                    .into_int()
                                    .expect(INTERNAL_ERR)
                                    .to_big_endian(v.as_mut_slice());
                                EthBigInt::new(v.into())
                            });
                        [
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                        ]
                    },
                })
            }
        }
        impl substreams_ethereum::Event for UpdateCollateralModifiers {
            const NAME: &'static str = "UpdateCollateralModifiers";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct UpdateErc1155Listing {
            pub listing_id: ethabi::Uint,
            pub quantity: ethabi::Uint,
            pub price_in_wei: ethabi::Uint,
            pub time: ethabi::Uint,
        }
        impl UpdateErc1155Listing {
            const TOPIC_ID: [u8; 32] = [
                154u8,
                30u8,
                5u8,
                61u8,
                40u8,
                194u8,
                239u8,
                198u8,
                173u8,
                209u8,
                228u8,
                165u8,
                209u8,
                100u8,
                122u8,
                153u8,
                232u8,
                131u8,
                178u8,
                86u8,
                165u8,
                79u8,
                158u8,
                244u8,
                160u8,
                171u8,
                248u8,
                252u8,
                186u8,
                100u8,
                58u8,
                85u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 2usize {
                    return false;
                }
                if log.data.len() != 96usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    listing_id: ethabi::decode(
                            &[ethabi::ParamType::Uint(256usize)],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'listing_id' from topic of type 'uint256': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    quantity: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    price_in_wei: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    time: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for UpdateErc1155Listing {
            const NAME: &'static str = "UpdateERC1155Listing";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct UpdateItemPrice {
            pub item_id: ethabi::Uint,
            pub price_in_wei: ethabi::Uint,
        }
        impl UpdateItemPrice {
            const TOPIC_ID: [u8; 32] = [
                213u8,
                253u8,
                211u8,
                106u8,
                223u8,
                236u8,
                118u8,
                211u8,
                145u8,
                221u8,
                129u8,
                103u8,
                32u8,
                189u8,
                88u8,
                183u8,
                78u8,
                100u8,
                212u8,
                158u8,
                93u8,
                53u8,
                225u8,
                228u8,
                17u8,
                14u8,
                32u8,
                4u8,
                83u8,
                118u8,
                224u8,
                35u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 1usize {
                    return false;
                }
                if log.data.len() != 64usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    item_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    price_in_wei: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for UpdateItemPrice {
            const NAME: &'static str = "UpdateItemPrice";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct UseConsumables {
            pub token_id: ethabi::Uint,
            pub item_ids: Vec<ethabi::Uint>,
            pub quantities: Vec<ethabi::Uint>,
        }
        impl UseConsumables {
            const TOPIC_ID: [u8; 32] = [
                202u8,
                110u8,
                141u8,
                5u8,
                219u8,
                129u8,
                212u8,
                117u8,
                65u8,
                14u8,
                145u8,
                120u8,
                56u8,
                146u8,
                57u8,
                124u8,
                21u8,
                19u8,
                217u8,
                226u8,
                43u8,
                27u8,
                77u8,
                201u8,
                5u8,
                239u8,
                209u8,
                133u8,
                106u8,
                141u8,
                28u8,
                37u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 2usize {
                    return false;
                }
                if log.data.len() < 128usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                            ),
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                            ),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    token_id: ethabi::decode(
                            &[ethabi::ParamType::Uint(256usize)],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'token_id' from topic of type 'uint256': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    item_ids: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
                    quantities: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
                })
            }
        }
        impl substreams_ethereum::Event for UseConsumables {
            const NAME: &'static str = "UseConsumables";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct VrfRandomNumber {
            pub token_id: ethabi::Uint,
            pub random_number: ethabi::Uint,
            pub vrf_time_set: ethabi::Uint,
        }
        impl VrfRandomNumber {
            const TOPIC_ID: [u8; 32] = [
                144u8,
                44u8,
                63u8,
                72u8,
                35u8,
                3u8,
                87u8,
                29u8,
                245u8,
                75u8,
                84u8,
                140u8,
                245u8,
                204u8,
                250u8,
                182u8,
                164u8,
                40u8,
                209u8,
                29u8,
                135u8,
                177u8,
                66u8,
                36u8,
                58u8,
                240u8,
                100u8,
                185u8,
                254u8,
                100u8,
                226u8,
                212u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 2usize {
                    return false;
                }
                if log.data.len() != 64usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    token_id: ethabi::decode(
                            &[ethabi::ParamType::Uint(256usize)],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'token_id' from topic of type 'uint256': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    random_number: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    vrf_time_set: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for VrfRandomNumber {
            const NAME: &'static str = "VrfRandomNumber";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct WearableSlotPositionsSet {
            pub wearable_id: ethabi::Uint,
            pub slot_positions: [bool; 16usize],
        }
        impl WearableSlotPositionsSet {
            const TOPIC_ID: [u8; 32] = [
                156u8,
                177u8,
                170u8,
                172u8,
                145u8,
                111u8,
                254u8,
                217u8,
                227u8,
                60u8,
                14u8,
                239u8,
                152u8,
                138u8,
                223u8,
                49u8,
                112u8,
                101u8,
                80u8,
                52u8,
                104u8,
                245u8,
                67u8,
                127u8,
                25u8,
                59u8,
                134u8,
                195u8,
                250u8,
                238u8,
                13u8,
                246u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 1usize {
                    return false;
                }
                if log.data.len() != 544usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::FixedArray(
                                Box::new(ethabi::ParamType::Bool),
                                16usize,
                            ),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    wearable_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    slot_positions: {
                        let mut iter = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_array()
                            .expect(INTERNAL_ERR)
                            .into_iter()
                            .map(|inner| inner.into_bool().expect(INTERNAL_ERR));
                        [
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                        ]
                    },
                })
            }
        }
        impl substreams_ethereum::Event for WearableSlotPositionsSet {
            const NAME: &'static str = "WearableSlotPositionsSet";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct WhitelistAccessRightSet {
            pub whitelist_id: ethabi::Uint,
            pub action_right: ethabi::Uint,
            pub access_right: ethabi::Uint,
        }
        impl WhitelistAccessRightSet {
            const TOPIC_ID: [u8; 32] = [
                195u8,
                180u8,
                181u8,
                250u8,
                250u8,
                139u8,
                54u8,
                132u8,
                157u8,
                39u8,
                207u8,
                245u8,
                243u8,
                119u8,
                236u8,
                130u8,
                144u8,
                97u8,
                0u8,
                39u8,
                185u8,
                247u8,
                101u8,
                20u8,
                61u8,
                155u8,
                50u8,
                68u8,
                98u8,
                163u8,
                47u8,
                87u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 4usize {
                    return false;
                }
                if log.data.len() != 0usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Ok(Self {
                    whitelist_id: ethabi::decode(
                            &[ethabi::ParamType::Uint(32usize)],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'whitelist_id' from topic of type 'uint32': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    action_right: ethabi::decode(
                            &[ethabi::ParamType::Uint(256usize)],
                            log.topics[2usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'action_right' from topic of type 'uint256': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    access_right: ethabi::decode(
                            &[ethabi::ParamType::Uint(256usize)],
                            log.topics[3usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'access_right' from topic of type 'uint256': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for WhitelistAccessRightSet {
            const NAME: &'static str = "WhitelistAccessRightSet";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct WhitelistCreated {
            pub whitelist_id: ethabi::Uint,
        }
        impl WhitelistCreated {
            const TOPIC_ID: [u8; 32] = [
                51u8,
                62u8,
                251u8,
                193u8,
                4u8,
                245u8,
                52u8,
                133u8,
                161u8,
                186u8,
                101u8,
                25u8,
                8u8,
                135u8,
                125u8,
                112u8,
                23u8,
                76u8,
                244u8,
                124u8,
                191u8,
                230u8,
                18u8,
                96u8,
                53u8,
                139u8,
                252u8,
                171u8,
                252u8,
                251u8,
                236u8,
                53u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 2usize {
                    return false;
                }
                if log.data.len() != 0usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Ok(Self {
                    whitelist_id: ethabi::decode(
                            &[ethabi::ParamType::Uint(32usize)],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'whitelist_id' from topic of type 'uint32': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for WhitelistCreated {
            const NAME: &'static str = "WhitelistCreated";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct WhitelistOwnershipTransferred {
            pub whitelist_id: ethabi::Uint,
            pub new_owner: Vec<u8>,
        }
        impl WhitelistOwnershipTransferred {
            const TOPIC_ID: [u8; 32] = [
                37u8,
                12u8,
                154u8,
                44u8,
                128u8,
                180u8,
                166u8,
                83u8,
                154u8,
                68u8,
                10u8,
                3u8,
                34u8,
                3u8,
                54u8,
                244u8,
                39u8,
                164u8,
                198u8,
                189u8,
                226u8,
                148u8,
                83u8,
                77u8,
                151u8,
                216u8,
                159u8,
                186u8,
                116u8,
                52u8,
                73u8,
                196u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 3usize {
                    return false;
                }
                if log.data.len() != 0usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Ok(Self {
                    whitelist_id: ethabi::decode(
                            &[ethabi::ParamType::Uint(32usize)],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'whitelist_id' from topic of type 'uint32': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    new_owner: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[2usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'new_owner' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                })
            }
        }
        impl substreams_ethereum::Event for WhitelistOwnershipTransferred {
            const NAME: &'static str = "WhitelistOwnershipTransferred";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct WhitelistUpdated {
            pub whitelist_id: ethabi::Uint,
        }
        impl WhitelistUpdated {
            const TOPIC_ID: [u8; 32] = [
                232u8,
                216u8,
                71u8,
                210u8,
                238u8,
                54u8,
                149u8,
                5u8,
                19u8,
                203u8,
                164u8,
                157u8,
                106u8,
                192u8,
                224u8,
                1u8,
                43u8,
                179u8,
                67u8,
                109u8,
                90u8,
                181u8,
                83u8,
                133u8,
                226u8,
                215u8,
                162u8,
                131u8,
                77u8,
                150u8,
                217u8,
                183u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 2usize {
                    return false;
                }
                if log.data.len() != 0usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Ok(Self {
                    whitelist_id: ethabi::decode(
                            &[ethabi::ParamType::Uint(32usize)],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'whitelist_id' from topic of type 'uint32': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for WhitelistUpdated {
            const NAME: &'static str = "WhitelistUpdated";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct WithdrawnBatch {
            pub owner: Vec<u8>,
            pub token_ids: Vec<ethabi::Uint>,
        }
        impl WithdrawnBatch {
            const TOPIC_ID: [u8; 32] = [
                248u8,
                113u8,
                137u8,
                107u8,
                23u8,
                233u8,
                203u8,
                122u8,
                100u8,
                148u8,
                28u8,
                98u8,
                193u8,
                136u8,
                164u8,
                245u8,
                198u8,
                33u8,
                184u8,
                104u8,
                0u8,
                227u8,
                209u8,
                84u8,
                82u8,
                236u8,
                224u8,
                28u8,
                229u8,
                96u8,
                115u8,
                223u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 2usize {
                    return false;
                }
                if log.data.len() < 64usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                            ),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    owner: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'owner' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    token_ids: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
                })
            }
        }
        impl substreams_ethereum::Event for WithdrawnBatch {
            const NAME: &'static str = "WithdrawnBatch";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct WithdrawnItems {
            pub owner: Vec<u8>,
            pub ids: Vec<ethabi::Uint>,
            pub values: Vec<ethabi::Uint>,
        }
        impl WithdrawnItems {
            const TOPIC_ID: [u8; 32] = [
                94u8,
                147u8,
                39u8,
                118u8,
                106u8,
                75u8,
                165u8,
                117u8,
                28u8,
                231u8,
                177u8,
                42u8,
                59u8,
                155u8,
                181u8,
                3u8,
                218u8,
                19u8,
                210u8,
                14u8,
                124u8,
                90u8,
                214u8,
                238u8,
                208u8,
                34u8,
                142u8,
                135u8,
                179u8,
                139u8,
                214u8,
                240u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 2usize {
                    return false;
                }
                if log.data.len() < 128usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                            ),
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                            ),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    owner: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'owner' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    ids: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
                    values: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
                })
            }
        }
        impl substreams_ethereum::Event for WithdrawnItems {
            const NAME: &'static str = "WithdrawnItems";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct Xingyun {
            pub from: Vec<u8>,
            pub to: Vec<u8>,
            pub token_id: ethabi::Uint,
            pub num_aavegotchis_to_purchase: ethabi::Uint,
            pub total_price: ethabi::Uint,
        }
        impl Xingyun {
            const TOPIC_ID: [u8; 32] = [
                227u8,
                136u8,
                159u8,
                78u8,
                84u8,
                16u8,
                213u8,
                196u8,
                174u8,
                128u8,
                7u8,
                139u8,
                110u8,
                193u8,
                73u8,
                222u8,
                70u8,
                115u8,
                205u8,
                178u8,
                251u8,
                46u8,
                54u8,
                174u8,
                26u8,
                20u8,
                228u8,
                187u8,
                177u8,
                206u8,
                105u8,
                248u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 3usize {
                    return false;
                }
                if log.data.len() != 96usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    from: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'from' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    to: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[2usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'to' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    token_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    num_aavegotchis_to_purchase: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    total_price: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for Xingyun {
            const NAME: &'static str = "Xingyun";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
    }