    const INTERNAL_ERR: &'static str = "`ethabi_derive` internal error";
    /// Contract's functions.
    #[allow(dead_code)]
    #[allow(unused_variables)]
    #[allow(unused_imports)]
    pub mod functions {
        use substreams_ethereum::scalar::EthBigInt;
        use super::INTERNAL_ERR;
        #[derive(Debug, Clone, PartialEq)]
        pub struct AddUpgradeQueueLength {
            pub realm_id: ethabi::Uint,
        }
        impl AddUpgradeQueueLength {
            const METHOD_ID: [u8; 4] = [87u8, 159u8, 164u8, 175u8];
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
                    realm_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(&[ethabi::Token::Uint(self.realm_id)]);
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
        impl substreams_ethereum::Function for AddUpgradeQueueLength {
            const NAME: &'static str = "addUpgradeQueueLength";
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
        pub struct BalanceOf {
            pub owner: Vec<u8>,
        }
        impl BalanceOf {
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
        impl substreams_ethereum::Function for BalanceOf {
            const NAME: &'static str = "balanceOf";
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
        impl substreams_ethereum::rpc::RPCDecodable<ethabi::Uint> for BalanceOf {
            fn output(data: &[u8]) -> Result<ethabi::Uint, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct BatchGetDistrictParcels {
            pub owner: Vec<u8>,
            pub district: ethabi::Uint,
        }
        impl BatchGetDistrictParcels {
            const METHOD_ID: [u8; 4] = [89u8, 74u8, 10u8, 112u8];
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
                    district: values
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
                        ethabi::Token::Uint(self.district),
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
        impl substreams_ethereum::Function for BatchGetDistrictParcels {
            const NAME: &'static str = "batchGetDistrictParcels";
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
        for BatchGetDistrictParcels {
            fn output(data: &[u8]) -> Result<Vec<ethabi::Uint>, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct BatchTransferAlchemica {
            pub targets: Vec<Vec<u8>>,
            pub amounts: Vec<[ethabi::Uint; 4usize]>,
        }
        impl BatchTransferAlchemica {
            const METHOD_ID: [u8; 4] = [112u8, 61u8, 193u8, 139u8];
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
                                Box::new(
                                    ethabi::ParamType::FixedArray(
                                        Box::new(ethabi::ParamType::Uint(256usize)),
                                        4usize,
                                    ),
                                ),
                            ),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    targets: values
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
                        .map(|inner| {
                            let mut iter = inner
                                .into_fixed_array()
                                .expect(INTERNAL_ERR)
                                .into_iter()
                                .map(|inner| inner.into_uint().expect(INTERNAL_ERR));
                            [
                                iter.next().expect(INTERNAL_ERR),
                                iter.next().expect(INTERNAL_ERR),
                                iter.next().expect(INTERNAL_ERR),
                                iter.next().expect(INTERNAL_ERR),
                            ]
                        })
                        .collect(),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        {
                            let v = self
                                .targets
                                .iter()
                                .map(|inner| ethabi::Token::Address(
                                    ethabi::Address::from_slice(&inner),
                                ))
                                .collect();
                            ethabi::Token::Array(v)
                        },
                        {
                            let v = self
                                .amounts
                                .iter()
                                .map(|inner| {
                                    let v = inner
                                        .iter()
                                        .map(|inner| ethabi::Token::Uint(inner.clone()))
                                        .collect();
                                    ethabi::Token::FixedArray(v)
                                })
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
        impl substreams_ethereum::Function for BatchTransferAlchemica {
            const NAME: &'static str = "batchTransferAlchemica";
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
        pub struct BatchTransferTokens {
            pub tokens: Vec<Vec<Vec<u8>>>,
            pub amounts: Vec<Vec<ethabi::Uint>>,
            pub to: Vec<Vec<u8>>,
        }
        impl BatchTransferTokens {
            const METHOD_ID: [u8; 4] = [29u8, 232u8, 214u8, 195u8];
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
                                Box::new(
                                    ethabi::ParamType::Array(
                                        Box::new(ethabi::ParamType::Address),
                                    ),
                                ),
                            ),
                            ethabi::ParamType::Array(
                                Box::new(
                                    ethabi::ParamType::Array(
                                        Box::new(ethabi::ParamType::Uint(256usize)),
                                    ),
                                ),
                            ),
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Address),
                            ),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    tokens: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| {
                            inner
                                .into_array()
                                .expect(INTERNAL_ERR)
                                .into_iter()
                                .map(|inner| {
                                    inner
                                        .into_address()
                                        .expect(INTERNAL_ERR)
                                        .as_bytes()
                                        .to_vec()
                                })
                                .collect()
                        })
                        .collect(),
                    amounts: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| {
                            inner
                                .into_array()
                                .expect(INTERNAL_ERR)
                                .into_iter()
                                .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                                .collect()
                        })
                        .collect(),
                    to: values
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
                                .tokens
                                .iter()
                                .map(|inner| {
                                    let v = inner
                                        .iter()
                                        .map(|inner| ethabi::Token::Address(
                                            ethabi::Address::from_slice(&inner),
                                        ))
                                        .collect();
                                    ethabi::Token::Array(v)
                                })
                                .collect();
                            ethabi::Token::Array(v)
                        },
                        {
                            let v = self
                                .amounts
                                .iter()
                                .map(|inner| {
                                    let v = inner
                                        .iter()
                                        .map(|inner| ethabi::Token::Uint(inner.clone()))
                                        .collect();
                                    ethabi::Token::Array(v)
                                })
                                .collect();
                            ethabi::Token::Array(v)
                        },
                        {
                            let v = self
                                .to
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
        impl substreams_ethereum::Function for BatchTransferTokens {
            const NAME: &'static str = "batchTransferTokens";
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
        pub struct BatchTransferTokensToGotchis {
            pub gotchi_ids: Vec<ethabi::Uint>,
            pub token_addresses: Vec<Vec<u8>>,
            pub amounts: Vec<Vec<ethabi::Uint>>,
        }
        impl BatchTransferTokensToGotchis {
            const METHOD_ID: [u8; 4] = [95u8, 219u8, 136u8, 252u8];
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
                                Box::new(
                                    ethabi::ParamType::Array(
                                        Box::new(ethabi::ParamType::Uint(256usize)),
                                    ),
                                ),
                            ),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    gotchi_ids: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
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
                        .map(|inner| {
                            inner
                                .into_array()
                                .expect(INTERNAL_ERR)
                                .into_iter()
                                .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                                .collect()
                        })
                        .collect(),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        {
                            let v = self
                                .gotchi_ids
                                .iter()
                                .map(|inner| ethabi::Token::Uint(inner.clone()))
                                .collect();
                            ethabi::Token::Array(v)
                        },
                        {
                            let v = self
                                .token_addresses
                                .iter()
                                .map(|inner| ethabi::Token::Address(
                                    ethabi::Address::from_slice(&inner),
                                ))
                                .collect();
                            ethabi::Token::Array(v)
                        },
                        {
                            let v = self
                                .amounts
                                .iter()
                                .map(|inner| {
                                    let v = inner
                                        .iter()
                                        .map(|inner| ethabi::Token::Uint(inner.clone()))
                                        .collect();
                                    ethabi::Token::Array(v)
                                })
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
        impl substreams_ethereum::Function for BatchTransferTokensToGotchis {
            const NAME: &'static str = "batchTransferTokensToGotchis";
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
        pub struct BuildingFrozen {}
        impl BuildingFrozen {
            const METHOD_ID: [u8; 4] = [129u8, 72u8, 247u8, 84u8];
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
        impl substreams_ethereum::Function for BuildingFrozen {
            const NAME: &'static str = "buildingFrozen";
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
        impl substreams_ethereum::rpc::RPCDecodable<bool> for BuildingFrozen {
            fn output(data: &[u8]) -> Result<bool, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct CancelEvent {
            pub realm_id: ethabi::Uint,
        }
        impl CancelEvent {
            const METHOD_ID: [u8; 4] = [63u8, 105u8, 186u8, 189u8];
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
                    realm_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(&[ethabi::Token::Uint(self.realm_id)]);
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
        impl substreams_ethereum::Function for CancelEvent {
            const NAME: &'static str = "cancelEvent";
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
        pub struct ChannelAlchemica {
            pub realm_id: ethabi::Uint,
            pub gotchi_id: ethabi::Uint,
            pub last_channeled: ethabi::Uint,
            pub signature: Vec<u8>,
        }
        impl ChannelAlchemica {
            const METHOD_ID: [u8; 4] = [128u8, 39u8, 135u8, 14u8];
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
                            ethabi::ParamType::Bytes,
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    realm_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    gotchi_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    last_channeled: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    signature: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_bytes()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Uint(self.realm_id),
                        ethabi::Token::Uint(self.gotchi_id),
                        ethabi::Token::Uint(self.last_channeled),
                        ethabi::Token::Bytes(self.signature.clone()),
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
        impl substreams_ethereum::Function for ChannelAlchemica {
            const NAME: &'static str = "channelAlchemica";
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
        pub struct CheckCoordinates {
            pub realm_id: ethabi::Uint,
            pub coordinate_x: ethabi::Uint,
            pub coordinate_y: ethabi::Uint,
            pub installation_id: ethabi::Uint,
        }
        impl CheckCoordinates {
            const METHOD_ID: [u8; 4] = [52u8, 115u8, 115u8, 199u8];
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
                            ethabi::ParamType::Uint(256usize),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    realm_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    coordinate_x: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    coordinate_y: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    installation_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Uint(self.realm_id),
                        ethabi::Token::Uint(self.coordinate_x),
                        ethabi::Token::Uint(self.coordinate_y),
                        ethabi::Token::Uint(self.installation_id),
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
        impl substreams_ethereum::Function for CheckCoordinates {
            const NAME: &'static str = "checkCoordinates";
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
        pub struct ClaimAvailableAlchemica {
            pub realm_id: ethabi::Uint,
            pub gotchi_id: ethabi::Uint,
            pub signature: Vec<u8>,
        }
        impl ClaimAvailableAlchemica {
            const METHOD_ID: [u8; 4] = [159u8, 239u8, 229u8, 71u8];
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
                            ethabi::ParamType::Bytes,
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    realm_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    gotchi_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    signature: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_bytes()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Uint(self.realm_id),
                        ethabi::Token::Uint(self.gotchi_id),
                        ethabi::Token::Bytes(self.signature.clone()),
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
        impl substreams_ethereum::Function for ClaimAvailableAlchemica {
            const NAME: &'static str = "claimAvailableAlchemica";
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
        pub struct CreateEvent {
            pub title: String,
            pub start_time: ethabi::Uint,
            pub duration_in_minutes: ethabi::Uint,
            pub alchemica_spent: [ethabi::Uint; 4usize],
            pub realm_id: ethabi::Uint,
        }
        impl CreateEvent {
            const METHOD_ID: [u8; 4] = [184u8, 3u8, 110u8, 250u8];
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
                            ethabi::ParamType::Uint(64usize),
                            ethabi::ParamType::Uint(64usize),
                            ethabi::ParamType::FixedArray(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                                4usize,
                            ),
                            ethabi::ParamType::Uint(256usize),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    title: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_string()
                        .expect(INTERNAL_ERR),
                    start_time: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    duration_in_minutes: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    alchemica_spent: {
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
                        ]
                    },
                    realm_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::String(self.title.clone()),
                        ethabi::Token::Uint(self.start_time),
                        ethabi::Token::Uint(self.duration_in_minutes),
                        {
                            let v = self
                                .alchemica_spent
                                .iter()
                                .map(|inner| ethabi::Token::Uint(inner.clone()))
                                .collect();
                            ethabi::Token::FixedArray(v)
                        },
                        ethabi::Token::Uint(self.realm_id),
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
        impl substreams_ethereum::Function for CreateEvent {
            const NAME: &'static str = "createEvent";
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
        pub struct EquipInstallation {
            pub realm_id: ethabi::Uint,
            pub gotchi_id: ethabi::Uint,
            pub installation_id: ethabi::Uint,
            pub x: ethabi::Uint,
            pub y: ethabi::Uint,
            pub signature: Vec<u8>,
        }
        impl EquipInstallation {
            const METHOD_ID: [u8; 4] = [90u8, 40u8, 74u8, 167u8];
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
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Bytes,
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    realm_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    gotchi_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    installation_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    x: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    y: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    signature: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_bytes()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Uint(self.realm_id),
                        ethabi::Token::Uint(self.gotchi_id),
                        ethabi::Token::Uint(self.installation_id),
                        ethabi::Token::Uint(self.x),
                        ethabi::Token::Uint(self.y),
                        ethabi::Token::Bytes(self.signature.clone()),
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
        impl substreams_ethereum::Function for EquipInstallation {
            const NAME: &'static str = "equipInstallation";
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
        pub struct EquipTile {
            pub realm_id: ethabi::Uint,
            pub gotchi_id: ethabi::Uint,
            pub tile_id: ethabi::Uint,
            pub x: ethabi::Uint,
            pub y: ethabi::Uint,
            pub signature: Vec<u8>,
        }
        impl EquipTile {
            const METHOD_ID: [u8; 4] = [216u8, 6u8, 66u8, 112u8];
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
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Bytes,
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    realm_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    gotchi_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    tile_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    x: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    y: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    signature: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_bytes()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Uint(self.realm_id),
                        ethabi::Token::Uint(self.gotchi_id),
                        ethabi::Token::Uint(self.tile_id),
                        ethabi::Token::Uint(self.x),
                        ethabi::Token::Uint(self.y),
                        ethabi::Token::Bytes(self.signature.clone()),
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
        impl substreams_ethereum::Function for EquipTile {
            const NAME: &'static str = "equipTile";
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
        pub struct FixGridStartPositions {
            pub parcel_ids: Vec<ethabi::Uint>,
            pub x: Vec<ethabi::Uint>,
            pub y: Vec<ethabi::Uint>,
            pub is_tile: bool,
            pub ids: Vec<ethabi::Uint>,
        }
        impl FixGridStartPositions {
            const METHOD_ID: [u8; 4] = [77u8, 52u8, 225u8, 219u8];
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
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                            ),
                            ethabi::ParamType::Bool,
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                            ),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    parcel_ids: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
                    x: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
                    y: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
                    is_tile: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_bool()
                        .expect(INTERNAL_ERR),
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
                                .parcel_ids
                                .iter()
                                .map(|inner| ethabi::Token::Uint(inner.clone()))
                                .collect();
                            ethabi::Token::Array(v)
                        },
                        {
                            let v = self
                                .x
                                .iter()
                                .map(|inner| ethabi::Token::Uint(inner.clone()))
                                .collect();
                            ethabi::Token::Array(v)
                        },
                        {
                            let v = self
                                .y
                                .iter()
                                .map(|inner| ethabi::Token::Uint(inner.clone()))
                                .collect();
                            ethabi::Token::Array(v)
                        },
                        ethabi::Token::Bool(self.is_tile),
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
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for FixGridStartPositions {
            const NAME: &'static str = "fixGridStartPositions";
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
        pub struct GetAlchemicaAddresses {}
        impl GetAlchemicaAddresses {
            const METHOD_ID: [u8; 4] = [85u8, 233u8, 46u8, 83u8];
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
            ) -> Result<[Vec<u8>; 4usize], String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<[Vec<u8>; 4usize], String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::FixedArray(
                                Box::new(ethabi::ParamType::Address),
                                4usize,
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
                            inner.into_address().expect(INTERNAL_ERR).as_bytes().to_vec()
                        });
                    [
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
            pub fn call(&self, address: Vec<u8>) -> Option<[Vec<u8>; 4usize]> {
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
        impl substreams_ethereum::Function for GetAlchemicaAddresses {
            const NAME: &'static str = "getAlchemicaAddresses";
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
        impl substreams_ethereum::rpc::RPCDecodable<[Vec<u8>; 4usize]>
        for GetAlchemicaAddresses {
            fn output(data: &[u8]) -> Result<[Vec<u8>; 4usize], String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct GetAltarId {
            pub parcel_id: ethabi::Uint,
        }
        impl GetAltarId {
            const METHOD_ID: [u8; 4] = [229u8, 125u8, 14u8, 53u8];
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
                    parcel_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(&[ethabi::Token::Uint(self.parcel_id)]);
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
        impl substreams_ethereum::Function for GetAltarId {
            const NAME: &'static str = "getAltarId";
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
        impl substreams_ethereum::rpc::RPCDecodable<ethabi::Uint> for GetAltarId {
            fn output(data: &[u8]) -> Result<ethabi::Uint, String> {
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
        pub struct GetAvailableAlchemica {
            pub realm_id: ethabi::Uint,
        }
        impl GetAvailableAlchemica {
            const METHOD_ID: [u8; 4] = [216u8, 132u8, 117u8, 173u8];
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
                    realm_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(&[ethabi::Token::Uint(self.realm_id)]);
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<[ethabi::Uint; 4usize], String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<[ethabi::Uint; 4usize], String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::FixedArray(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                                4usize,
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
                    ]
                })
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<[ethabi::Uint; 4usize]> {
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
        impl substreams_ethereum::Function for GetAvailableAlchemica {
            const NAME: &'static str = "getAvailableAlchemica";
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
        impl substreams_ethereum::rpc::RPCDecodable<[ethabi::Uint; 4usize]>
        for GetAvailableAlchemica {
            fn output(data: &[u8]) -> Result<[ethabi::Uint; 4usize], String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct GetCapacities {
            pub realm_id: ethabi::Uint,
        }
        impl GetCapacities {
            const METHOD_ID: [u8; 4] = [50u8, 252u8, 109u8, 124u8];
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
                    realm_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(&[ethabi::Token::Uint(self.realm_id)]);
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
        impl substreams_ethereum::Function for GetCapacities {
            const NAME: &'static str = "getCapacities";
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
        for GetCapacities {
            fn output(data: &[u8]) -> Result<Vec<ethabi::Uint>, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct GetHarvestRates {
            pub realm_id: ethabi::Uint,
        }
        impl GetHarvestRates {
            const METHOD_ID: [u8; 4] = [100u8, 185u8, 228u8, 57u8];
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
                    realm_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(&[ethabi::Token::Uint(self.realm_id)]);
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
        impl substreams_ethereum::Function for GetHarvestRates {
            const NAME: &'static str = "getHarvestRates";
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
        for GetHarvestRates {
            fn output(data: &[u8]) -> Result<Vec<ethabi::Uint>, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct GetHumbleGrid {
            pub parcel_id: ethabi::Uint,
            pub grid_type: ethabi::Uint,
        }
        impl GetHumbleGrid {
            const METHOD_ID: [u8; 4] = [118u8, 144u8, 166u8, 179u8];
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
                    parcel_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    grid_type: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Uint(self.parcel_id),
                        ethabi::Token::Uint(self.grid_type),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<[[ethabi::Uint; 8usize]; 8usize], String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(
                data: &[u8],
            ) -> Result<[[ethabi::Uint; 8usize]; 8usize], String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::FixedArray(
                                Box::new(
                                    ethabi::ParamType::FixedArray(
                                        Box::new(ethabi::ParamType::Uint(256usize)),
                                        8usize,
                                    ),
                                ),
                                8usize,
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
                            let mut iter = inner
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
                            ]
                        });
                    [
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
            pub fn call(
                &self,
                address: Vec<u8>,
            ) -> Option<[[ethabi::Uint; 8usize]; 8usize]> {
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
        impl substreams_ethereum::Function for GetHumbleGrid {
            const NAME: &'static str = "getHumbleGrid";
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
        impl substreams_ethereum::rpc::RPCDecodable<[[ethabi::Uint; 8usize]; 8usize]>
        for GetHumbleGrid {
            fn output(data: &[u8]) -> Result<[[ethabi::Uint; 8usize]; 8usize], String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct GetLastChanneled {
            pub gotchi_id: ethabi::Uint,
        }
        impl GetLastChanneled {
            const METHOD_ID: [u8; 4] = [0u8, 219u8, 160u8, 176u8];
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
                    gotchi_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(&[ethabi::Token::Uint(self.gotchi_id)]);
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
        impl substreams_ethereum::Function for GetLastChanneled {
            const NAME: &'static str = "getLastChanneled";
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
        impl substreams_ethereum::rpc::RPCDecodable<ethabi::Uint> for GetLastChanneled {
            fn output(data: &[u8]) -> Result<ethabi::Uint, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct GetPaartnerGrid {
            pub parcel_id: ethabi::Uint,
            pub grid_type: ethabi::Uint,
        }
        impl GetPaartnerGrid {
            const METHOD_ID: [u8; 4] = [30u8, 242u8, 95u8, 148u8];
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
                    parcel_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    grid_type: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Uint(self.parcel_id),
                        ethabi::Token::Uint(self.grid_type),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<[[ethabi::Uint; 64usize]; 64usize], String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(
                data: &[u8],
            ) -> Result<[[ethabi::Uint; 64usize]; 64usize], String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::FixedArray(
                                Box::new(
                                    ethabi::ParamType::FixedArray(
                                        Box::new(ethabi::ParamType::Uint(256usize)),
                                        64usize,
                                    ),
                                ),
                                64usize,
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
                            let mut iter = inner
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
                        });
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
            pub fn call(
                &self,
                address: Vec<u8>,
            ) -> Option<[[ethabi::Uint; 64usize]; 64usize]> {
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
        impl substreams_ethereum::Function for GetPaartnerGrid {
            const NAME: &'static str = "getPaartnerGrid";
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
        impl substreams_ethereum::rpc::RPCDecodable<[[ethabi::Uint; 64usize]; 64usize]>
        for GetPaartnerGrid {
            fn output(
                data: &[u8],
            ) -> Result<[[ethabi::Uint; 64usize]; 64usize], String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct GetParcelLastChanneled {
            pub parcel_id: ethabi::Uint,
        }
        impl GetParcelLastChanneled {
            const METHOD_ID: [u8; 4] = [203u8, 53u8, 7u8, 185u8];
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
                    parcel_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(&[ethabi::Token::Uint(self.parcel_id)]);
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
        impl substreams_ethereum::Function for GetParcelLastChanneled {
            const NAME: &'static str = "getParcelLastChanneled";
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
        for GetParcelLastChanneled {
            fn output(data: &[u8]) -> Result<ethabi::Uint, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct GetParcelUpgradeQueueCapacity {
            pub parcel_id: ethabi::Uint,
        }
        impl GetParcelUpgradeQueueCapacity {
            const METHOD_ID: [u8; 4] = [218u8, 128u8, 199u8, 110u8];
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
                    parcel_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(&[ethabi::Token::Uint(self.parcel_id)]);
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
        impl substreams_ethereum::Function for GetParcelUpgradeQueueCapacity {
            const NAME: &'static str = "getParcelUpgradeQueueCapacity";
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
        for GetParcelUpgradeQueueCapacity {
            fn output(data: &[u8]) -> Result<ethabi::Uint, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct GetParcelUpgradeQueueLength {
            pub parcel_id: ethabi::Uint,
        }
        impl GetParcelUpgradeQueueLength {
            const METHOD_ID: [u8; 4] = [228u8, 92u8, 125u8, 226u8];
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
                    parcel_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(&[ethabi::Token::Uint(self.parcel_id)]);
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
        impl substreams_ethereum::Function for GetParcelUpgradeQueueLength {
            const NAME: &'static str = "getParcelUpgradeQueueLength";
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
        for GetParcelUpgradeQueueLength {
            fn output(data: &[u8]) -> Result<ethabi::Uint, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct GetParcelsAccessRights {
            pub parcel_ids: Vec<ethabi::Uint>,
            pub action_rights: Vec<ethabi::Uint>,
        }
        impl GetParcelsAccessRights {
            const METHOD_ID: [u8; 4] = [173u8, 246u8, 136u8, 8u8];
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
                    parcel_ids: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
                    action_rights: values
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
                                .parcel_ids
                                .iter()
                                .map(|inner| ethabi::Token::Uint(inner.clone()))
                                .collect();
                            ethabi::Token::Array(v)
                        },
                        {
                            let v = self
                                .action_rights
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
        impl substreams_ethereum::Function for GetParcelsAccessRights {
            const NAME: &'static str = "getParcelsAccessRights";
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
        for GetParcelsAccessRights {
            fn output(data: &[u8]) -> Result<Vec<ethabi::Uint>, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct GetRealmAlchemica {
            pub realm_id: ethabi::Uint,
        }
        impl GetRealmAlchemica {
            const METHOD_ID: [u8; 4] = [94u8, 48u8, 103u8, 197u8];
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
                    realm_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(&[ethabi::Token::Uint(self.realm_id)]);
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<[ethabi::Uint; 4usize], String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<[ethabi::Uint; 4usize], String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::FixedArray(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                                4usize,
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
                    ]
                })
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<[ethabi::Uint; 4usize]> {
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
        impl substreams_ethereum::Function for GetRealmAlchemica {
            const NAME: &'static str = "getRealmAlchemica";
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
        impl substreams_ethereum::rpc::RPCDecodable<[ethabi::Uint; 4usize]>
        for GetRealmAlchemica {
            fn output(data: &[u8]) -> Result<[ethabi::Uint; 4usize], String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct GetReasonableGrid {
            pub parcel_id: ethabi::Uint,
            pub grid_type: ethabi::Uint,
        }
        impl GetReasonableGrid {
            const METHOD_ID: [u8; 4] = [195u8, 103u8, 18u8, 22u8];
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
                    parcel_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    grid_type: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Uint(self.parcel_id),
                        ethabi::Token::Uint(self.grid_type),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<[[ethabi::Uint; 16usize]; 16usize], String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(
                data: &[u8],
            ) -> Result<[[ethabi::Uint; 16usize]; 16usize], String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::FixedArray(
                                Box::new(
                                    ethabi::ParamType::FixedArray(
                                        Box::new(ethabi::ParamType::Uint(256usize)),
                                        16usize,
                                    ),
                                ),
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
                        .map(|inner| {
                            let mut iter = inner
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
                        });
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
            pub fn call(
                &self,
                address: Vec<u8>,
            ) -> Option<[[ethabi::Uint; 16usize]; 16usize]> {
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
        impl substreams_ethereum::Function for GetReasonableGrid {
            const NAME: &'static str = "getReasonableGrid";
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
        impl substreams_ethereum::rpc::RPCDecodable<[[ethabi::Uint; 16usize]; 16usize]>
        for GetReasonableGrid {
            fn output(
                data: &[u8],
            ) -> Result<[[ethabi::Uint; 16usize]; 16usize], String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct GetRoundAlchemica {
            pub realm_id: ethabi::Uint,
            pub round_id: ethabi::Uint,
        }
        impl GetRoundAlchemica {
            const METHOD_ID: [u8; 4] = [117u8, 39u8, 22u8, 234u8];
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
                    realm_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    round_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Uint(self.realm_id),
                        ethabi::Token::Uint(self.round_id),
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
        impl substreams_ethereum::Function for GetRoundAlchemica {
            const NAME: &'static str = "getRoundAlchemica";
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
        for GetRoundAlchemica {
            fn output(data: &[u8]) -> Result<Vec<ethabi::Uint>, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct GetRoundBaseAlchemica {
            pub realm_id: ethabi::Uint,
            pub round_id: ethabi::Uint,
        }
        impl GetRoundBaseAlchemica {
            const METHOD_ID: [u8; 4] = [21u8, 10u8, 51u8, 252u8];
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
                    realm_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    round_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Uint(self.realm_id),
                        ethabi::Token::Uint(self.round_id),
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
        impl substreams_ethereum::Function for GetRoundBaseAlchemica {
            const NAME: &'static str = "getRoundBaseAlchemica";
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
        for GetRoundBaseAlchemica {
            fn output(data: &[u8]) -> Result<Vec<ethabi::Uint>, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct GetSpaciousHorizontalGrid {
            pub parcel_id: ethabi::Uint,
            pub grid_type: ethabi::Uint,
        }
        impl GetSpaciousHorizontalGrid {
            const METHOD_ID: [u8; 4] = [173u8, 61u8, 15u8, 101u8];
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
                    parcel_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    grid_type: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Uint(self.parcel_id),
                        ethabi::Token::Uint(self.grid_type),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<[[ethabi::Uint; 64usize]; 32usize], String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(
                data: &[u8],
            ) -> Result<[[ethabi::Uint; 64usize]; 32usize], String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::FixedArray(
                                Box::new(
                                    ethabi::ParamType::FixedArray(
                                        Box::new(ethabi::ParamType::Uint(256usize)),
                                        64usize,
                                    ),
                                ),
                                32usize,
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
                            let mut iter = inner
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
                        });
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
            pub fn call(
                &self,
                address: Vec<u8>,
            ) -> Option<[[ethabi::Uint; 64usize]; 32usize]> {
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
        impl substreams_ethereum::Function for GetSpaciousHorizontalGrid {
            const NAME: &'static str = "getSpaciousHorizontalGrid";
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
        impl substreams_ethereum::rpc::RPCDecodable<[[ethabi::Uint; 64usize]; 32usize]>
        for GetSpaciousHorizontalGrid {
            fn output(
                data: &[u8],
            ) -> Result<[[ethabi::Uint; 64usize]; 32usize], String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct GetSpaciousVerticalGrid {
            pub parcel_id: ethabi::Uint,
            pub grid_type: ethabi::Uint,
        }
        impl GetSpaciousVerticalGrid {
            const METHOD_ID: [u8; 4] = [45u8, 126u8, 19u8, 1u8];
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
                    parcel_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    grid_type: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Uint(self.parcel_id),
                        ethabi::Token::Uint(self.grid_type),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<[[ethabi::Uint; 32usize]; 64usize], String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(
                data: &[u8],
            ) -> Result<[[ethabi::Uint; 32usize]; 64usize], String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::FixedArray(
                                Box::new(
                                    ethabi::ParamType::FixedArray(
                                        Box::new(ethabi::ParamType::Uint(256usize)),
                                        32usize,
                                    ),
                                ),
                                64usize,
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
                            let mut iter = inner
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
                        });
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
            pub fn call(
                &self,
                address: Vec<u8>,
            ) -> Option<[[ethabi::Uint; 32usize]; 64usize]> {
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
        impl substreams_ethereum::Function for GetSpaciousVerticalGrid {
            const NAME: &'static str = "getSpaciousVerticalGrid";
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
        impl substreams_ethereum::rpc::RPCDecodable<[[ethabi::Uint; 32usize]; 64usize]>
        for GetSpaciousVerticalGrid {
            fn output(
                data: &[u8],
            ) -> Result<[[ethabi::Uint; 32usize]; 64usize], String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct GetTotalAlchemicas {}
        impl GetTotalAlchemicas {
            const METHOD_ID: [u8; 4] = [63u8, 208u8, 88u8, 44u8];
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
            ) -> Result<[[ethabi::Uint; 4usize]; 5usize], String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(
                data: &[u8],
            ) -> Result<[[ethabi::Uint; 4usize]; 5usize], String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::FixedArray(
                                Box::new(
                                    ethabi::ParamType::FixedArray(
                                        Box::new(ethabi::ParamType::Uint(256usize)),
                                        4usize,
                                    ),
                                ),
                                5usize,
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
                            let mut iter = inner
                                .into_fixed_array()
                                .expect(INTERNAL_ERR)
                                .into_iter()
                                .map(|inner| inner.into_uint().expect(INTERNAL_ERR));
                            [
                                iter.next().expect(INTERNAL_ERR),
                                iter.next().expect(INTERNAL_ERR),
                                iter.next().expect(INTERNAL_ERR),
                                iter.next().expect(INTERNAL_ERR),
                            ]
                        });
                    [
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
            pub fn call(
                &self,
                address: Vec<u8>,
            ) -> Option<[[ethabi::Uint; 4usize]; 5usize]> {
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
        impl substreams_ethereum::Function for GetTotalAlchemicas {
            const NAME: &'static str = "getTotalAlchemicas";
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
        impl substreams_ethereum::rpc::RPCDecodable<[[ethabi::Uint; 4usize]; 5usize]>
        for GetTotalAlchemicas {
            fn output(data: &[u8]) -> Result<[[ethabi::Uint; 4usize]; 5usize], String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct GetTotalClaimed {
            pub realm_id: ethabi::Uint,
        }
        impl GetTotalClaimed {
            const METHOD_ID: [u8; 4] = [48u8, 221u8, 213u8, 29u8];
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
                    realm_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(&[ethabi::Token::Uint(self.realm_id)]);
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
        impl substreams_ethereum::Function for GetTotalClaimed {
            const NAME: &'static str = "getTotalClaimed";
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
        for GetTotalClaimed {
            fn output(data: &[u8]) -> Result<Vec<ethabi::Uint>, String> {
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
        pub struct IsGridStartPosition {
            pub parcel_id: ethabi::Uint,
            pub x: ethabi::Uint,
            pub y: ethabi::Uint,
            pub is_tile: bool,
            pub id: ethabi::Uint,
        }
        impl IsGridStartPosition {
            const METHOD_ID: [u8; 4] = [146u8, 2u8, 168u8, 26u8];
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
                            ethabi::ParamType::Bool,
                            ethabi::ParamType::Uint(256usize),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    parcel_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    x: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    y: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    is_tile: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_bool()
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
                        ethabi::Token::Uint(self.parcel_id),
                        ethabi::Token::Uint(self.x),
                        ethabi::Token::Uint(self.y),
                        ethabi::Token::Bool(self.is_tile),
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
        impl substreams_ethereum::Function for IsGridStartPosition {
            const NAME: &'static str = "isGridStartPosition";
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
        impl substreams_ethereum::rpc::RPCDecodable<bool> for IsGridStartPosition {
            fn output(data: &[u8]) -> Result<bool, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct IsSurveying {
            pub realm_id: ethabi::Uint,
        }
        impl IsSurveying {
            const METHOD_ID: [u8; 4] = [18u8, 138u8, 176u8, 87u8];
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
                    realm_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(&[ethabi::Token::Uint(self.realm_id)]);
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
        impl substreams_ethereum::Function for IsSurveying {
            const NAME: &'static str = "isSurveying";
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
        impl substreams_ethereum::rpc::RPCDecodable<bool> for IsSurveying {
            fn output(data: &[u8]) -> Result<bool, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct LastClaimedAlchemica {
            pub realm_id: ethabi::Uint,
        }
        impl LastClaimedAlchemica {
            const METHOD_ID: [u8; 4] = [36u8, 19u8, 63u8, 241u8];
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
                    realm_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(&[ethabi::Token::Uint(self.realm_id)]);
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
        impl substreams_ethereum::Function for LastClaimedAlchemica {
            const NAME: &'static str = "lastClaimedAlchemica";
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
        for LastClaimedAlchemica {
            fn output(data: &[u8]) -> Result<ethabi::Uint, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct MaxSupply {}
        impl MaxSupply {
            const METHOD_ID: [u8; 4] = [213u8, 171u8, 235u8, 1u8];
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
        impl substreams_ethereum::Function for MaxSupply {
            const NAME: &'static str = "maxSupply";
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
        impl substreams_ethereum::rpc::RPCDecodable<ethabi::Uint> for MaxSupply {
            fn output(data: &[u8]) -> Result<ethabi::Uint, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct MoveInstallation {
            pub realm_id: ethabi::Uint,
            pub installation_id: ethabi::Uint,
            pub x0: ethabi::Uint,
            pub y0: ethabi::Uint,
            pub x1: ethabi::Uint,
            pub y1: ethabi::Uint,
        }
        impl MoveInstallation {
            const METHOD_ID: [u8; 4] = [117u8, 112u8, 111u8, 239u8];
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
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    realm_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    installation_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    x0: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    y0: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    x1: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    y1: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Uint(self.realm_id),
                        ethabi::Token::Uint(self.installation_id),
                        ethabi::Token::Uint(self.x0),
                        ethabi::Token::Uint(self.y0),
                        ethabi::Token::Uint(self.x1),
                        ethabi::Token::Uint(self.y1),
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
        impl substreams_ethereum::Function for MoveInstallation {
            const NAME: &'static str = "moveInstallation";
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
        pub struct MoveTile {
            pub realm_id: ethabi::Uint,
            pub tile_id: ethabi::Uint,
            pub x0: ethabi::Uint,
            pub y0: ethabi::Uint,
            pub x1: ethabi::Uint,
            pub y1: ethabi::Uint,
        }
        impl MoveTile {
            const METHOD_ID: [u8; 4] = [82u8, 79u8, 17u8, 213u8];
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
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    realm_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    tile_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    x0: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    y0: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    x1: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    y1: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Uint(self.realm_id),
                        ethabi::Token::Uint(self.tile_id),
                        ethabi::Token::Uint(self.x0),
                        ethabi::Token::Uint(self.y0),
                        ethabi::Token::Uint(self.x1),
                        ethabi::Token::Uint(self.y1),
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
        impl substreams_ethereum::Function for MoveTile {
            const NAME: &'static str = "moveTile";
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
        pub struct NftDisplayAllowed {
            pub token: Vec<u8>,
            pub chain_id: ethabi::Uint,
        }
        impl NftDisplayAllowed {
            const METHOD_ID: [u8; 4] = [229u8, 17u8, 36u8, 69u8];
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
                    token: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    chain_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Address(ethabi::Address::from_slice(&self.token)),
                        ethabi::Token::Uint(self.chain_id),
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
        impl substreams_ethereum::Function for NftDisplayAllowed {
            const NAME: &'static str = "nftDisplayAllowed";
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
        impl substreams_ethereum::rpc::RPCDecodable<bool> for NftDisplayAllowed {
            fn output(data: &[u8]) -> Result<bool, String> {
                Self::output(data)
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
        pub struct ProgressSurveyingRound {}
        impl ProgressSurveyingRound {
            const METHOD_ID: [u8; 4] = [196u8, 120u8, 36u8, 58u8];
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
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for ProgressSurveyingRound {
            const NAME: &'static str = "progressSurveyingRound";
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
        pub struct RawFulfillRandomWords {
            pub request_id: ethabi::Uint,
            pub random_words: Vec<ethabi::Uint>,
        }
        impl RawFulfillRandomWords {
            const METHOD_ID: [u8; 4] = [31u8, 229u8, 67u8, 227u8];
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
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    request_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    random_words: values
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
                        ethabi::Token::Uint(self.request_id),
                        {
                            let v = self
                                .random_words
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
        impl substreams_ethereum::Function for RawFulfillRandomWords {
            const NAME: &'static str = "rawFulfillRandomWords";
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
        pub struct RecreateEvent {
            pub realm_id: ethabi::Uint,
            pub start_time: ethabi::Uint,
            pub duration_in_minutes: ethabi::Uint,
            pub alchemica_spent: [ethabi::Uint; 4usize],
        }
        impl RecreateEvent {
            const METHOD_ID: [u8; 4] = [206u8, 14u8, 16u8, 174u8];
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
                            ethabi::ParamType::Uint(64usize),
                            ethabi::ParamType::Uint(64usize),
                            ethabi::ParamType::FixedArray(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                                4usize,
                            ),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    realm_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    start_time: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    duration_in_minutes: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    alchemica_spent: {
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
                        ]
                    },
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Uint(self.realm_id),
                        ethabi::Token::Uint(self.start_time),
                        ethabi::Token::Uint(self.duration_in_minutes),
                        {
                            let v = self
                                .alchemica_spent
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
        impl substreams_ethereum::Function for RecreateEvent {
            const NAME: &'static str = "recreateEvent";
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
        pub struct ResyncParcel {
            pub token_ids: Vec<ethabi::Uint>,
        }
        impl ResyncParcel {
            const METHOD_ID: [u8; 4] = [188u8, 108u8, 13u8, 51u8];
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
        impl substreams_ethereum::Function for ResyncParcel {
            const NAME: &'static str = "resyncParcel";
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
        pub struct SafeBatchTransfer {
            pub from: Vec<u8>,
            pub to: Vec<u8>,
            pub token_ids: Vec<ethabi::Uint>,
            pub data: Vec<u8>,
        }
        impl SafeBatchTransfer {
            const METHOD_ID: [u8; 4] = [129u8, 238u8, 186u8, 219u8];
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
        impl substreams_ethereum::Function for SafeBatchTransfer {
            const NAME: &'static str = "safeBatchTransfer";
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
        pub struct SetAltarId {
            pub parcel_id: ethabi::Uint,
            pub altar_id: ethabi::Uint,
        }
        impl SetAltarId {
            const METHOD_ID: [u8; 4] = [158u8, 114u8, 194u8, 125u8];
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
                    parcel_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    altar_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Uint(self.parcel_id),
                        ethabi::Token::Uint(self.altar_id),
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
        impl substreams_ethereum::Function for SetAltarId {
            const NAME: &'static str = "setAltarId";
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
        pub struct SetChannelingLimits {
            pub altar_level: Vec<ethabi::Uint>,
            pub limits: Vec<ethabi::Uint>,
        }
        impl SetChannelingLimits {
            const METHOD_ID: [u8; 4] = [230u8, 144u8, 235u8, 172u8];
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
                    altar_level: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
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
                                .altar_level
                                .iter()
                                .map(|inner| ethabi::Token::Uint(inner.clone()))
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
        impl substreams_ethereum::Function for SetChannelingLimits {
            const NAME: &'static str = "setChannelingLimits";
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
        pub struct SetFreezeBuilding {
            pub freeze_building: bool,
        }
        impl SetFreezeBuilding {
            const METHOD_ID: [u8; 4] = [156u8, 33u8, 71u8, 43u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Bool],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    freeze_building: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_bool()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(&[ethabi::Token::Bool(self.freeze_building)]);
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
        impl substreams_ethereum::Function for SetFreezeBuilding {
            const NAME: &'static str = "setFreezeBuilding";
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
        pub struct SetGameActive {
            pub game_active: bool,
        }
        impl SetGameActive {
            const METHOD_ID: [u8; 4] = [204u8, 192u8, 137u8, 116u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Bool],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    game_active: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_bool()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(&[ethabi::Token::Bool(self.game_active)]);
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
        impl substreams_ethereum::Function for SetGameActive {
            const NAME: &'static str = "setGameActive";
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
        pub struct SetIndex {
            pub token_id: ethabi::Uint,
        }
        impl SetIndex {
            const METHOD_ID: [u8; 4] = [64u8, 165u8, 115u8, 127u8];
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
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for SetIndex {
            const NAME: &'static str = "setIndex";
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
        pub struct SetParcelsAccessRightWithWhitelists {
            pub realm_ids: Vec<ethabi::Uint>,
            pub action_rights: Vec<ethabi::Uint>,
            pub access_rights: Vec<ethabi::Uint>,
            pub whitelist_ids: Vec<ethabi::Uint>,
        }
        impl SetParcelsAccessRightWithWhitelists {
            const METHOD_ID: [u8; 4] = [153u8, 250u8, 88u8, 235u8];
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
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                            ),
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Uint(32usize)),
                            ),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    realm_ids: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
                    action_rights: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
                    access_rights: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
                    whitelist_ids: values
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
                                .realm_ids
                                .iter()
                                .map(|inner| ethabi::Token::Uint(inner.clone()))
                                .collect();
                            ethabi::Token::Array(v)
                        },
                        {
                            let v = self
                                .action_rights
                                .iter()
                                .map(|inner| ethabi::Token::Uint(inner.clone()))
                                .collect();
                            ethabi::Token::Array(v)
                        },
                        {
                            let v = self
                                .access_rights
                                .iter()
                                .map(|inner| ethabi::Token::Uint(inner.clone()))
                                .collect();
                            ethabi::Token::Array(v)
                        },
                        {
                            let v = self
                                .whitelist_ids
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
        impl substreams_ethereum::Function for SetParcelsAccessRightWithWhitelists {
            const NAME: &'static str = "setParcelsAccessRightWithWhitelists";
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
        pub struct SetParcelsAccessRights {
            pub realm_ids: Vec<ethabi::Uint>,
            pub action_rights: Vec<ethabi::Uint>,
            pub access_rights: Vec<ethabi::Uint>,
        }
        impl SetParcelsAccessRights {
            const METHOD_ID: [u8; 4] = [54u8, 126u8, 61u8, 172u8];
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
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                            ),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    realm_ids: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
                    action_rights: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
                    access_rights: values
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
                                .realm_ids
                                .iter()
                                .map(|inner| ethabi::Token::Uint(inner.clone()))
                                .collect();
                            ethabi::Token::Array(v)
                        },
                        {
                            let v = self
                                .action_rights
                                .iter()
                                .map(|inner| ethabi::Token::Uint(inner.clone()))
                                .collect();
                            ethabi::Token::Array(v)
                        },
                        {
                            let v = self
                                .access_rights
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
        impl substreams_ethereum::Function for SetParcelsAccessRights {
            const NAME: &'static str = "setParcelsAccessRights";
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
        pub struct SetTotalAlchemicas {
            pub total_alchemicas: [[ethabi::Uint; 4usize]; 5usize],
        }
        impl SetTotalAlchemicas {
            const METHOD_ID: [u8; 4] = [249u8, 250u8, 203u8, 102u8];
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
                                Box::new(
                                    ethabi::ParamType::FixedArray(
                                        Box::new(ethabi::ParamType::Uint(256usize)),
                                        4usize,
                                    ),
                                ),
                                5usize,
                            ),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    total_alchemicas: {
                        let mut iter = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_array()
                            .expect(INTERNAL_ERR)
                            .into_iter()
                            .map(|inner| {
                                let mut iter = inner
                                    .into_fixed_array()
                                    .expect(INTERNAL_ERR)
                                    .into_iter()
                                    .map(|inner| inner.into_uint().expect(INTERNAL_ERR));
                                [
                                    iter.next().expect(INTERNAL_ERR),
                                    iter.next().expect(INTERNAL_ERR),
                                    iter.next().expect(INTERNAL_ERR),
                                    iter.next().expect(INTERNAL_ERR),
                                ]
                            });
                        [
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
                                .total_alchemicas
                                .iter()
                                .map(|inner| {
                                    let v = inner
                                        .iter()
                                        .map(|inner| ethabi::Token::Uint(inner.clone()))
                                        .collect();
                                    ethabi::Token::FixedArray(v)
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
        impl substreams_ethereum::Function for SetTotalAlchemicas {
            const NAME: &'static str = "setTotalAlchemicas";
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
        pub struct SetVars {
            pub alchemicas: [[ethabi::Uint; 4usize]; 5usize],
            pub boost_multipliers: [ethabi::Uint; 4usize],
            pub great_portal_capacity: [ethabi::Uint; 4usize],
            pub installations_diamond: Vec<u8>,
            pub vrf_coordinator: Vec<u8>,
            pub link_address: Vec<u8>,
            pub alchemica_addresses: [Vec<u8>; 4usize],
            pub gltr_address: Vec<u8>,
            pub backend_pub_key: Vec<u8>,
            pub game_manager: Vec<u8>,
            pub tile_diamond: Vec<u8>,
            pub aavegotchi_diamond: Vec<u8>,
        }
        impl SetVars {
            const METHOD_ID: [u8; 4] = [129u8, 157u8, 75u8, 253u8];
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
                                Box::new(
                                    ethabi::ParamType::FixedArray(
                                        Box::new(ethabi::ParamType::Uint(256usize)),
                                        4usize,
                                    ),
                                ),
                                5usize,
                            ),
                            ethabi::ParamType::FixedArray(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                                4usize,
                            ),
                            ethabi::ParamType::FixedArray(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                                4usize,
                            ),
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Address,
                            ethabi::ParamType::FixedArray(
                                Box::new(ethabi::ParamType::Address),
                                4usize,
                            ),
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Bytes,
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Address,
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    alchemicas: {
                        let mut iter = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_array()
                            .expect(INTERNAL_ERR)
                            .into_iter()
                            .map(|inner| {
                                let mut iter = inner
                                    .into_fixed_array()
                                    .expect(INTERNAL_ERR)
                                    .into_iter()
                                    .map(|inner| inner.into_uint().expect(INTERNAL_ERR));
                                [
                                    iter.next().expect(INTERNAL_ERR),
                                    iter.next().expect(INTERNAL_ERR),
                                    iter.next().expect(INTERNAL_ERR),
                                    iter.next().expect(INTERNAL_ERR),
                                ]
                            });
                        [
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                        ]
                    },
                    boost_multipliers: {
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
                        ]
                    },
                    great_portal_capacity: {
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
                        ]
                    },
                    installations_diamond: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    vrf_coordinator: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    link_address: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    alchemica_addresses: {
                        let mut iter = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_array()
                            .expect(INTERNAL_ERR)
                            .into_iter()
                            .map(|inner| {
                                inner
                                    .into_address()
                                    .expect(INTERNAL_ERR)
                                    .as_bytes()
                                    .to_vec()
                            });
                        [
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                        ]
                    },
                    gltr_address: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    backend_pub_key: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_bytes()
                        .expect(INTERNAL_ERR),
                    game_manager: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    tile_diamond: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    aavegotchi_diamond: values
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
                        {
                            let v = self
                                .alchemicas
                                .iter()
                                .map(|inner| {
                                    let v = inner
                                        .iter()
                                        .map(|inner| ethabi::Token::Uint(inner.clone()))
                                        .collect();
                                    ethabi::Token::FixedArray(v)
                                })
                                .collect();
                            ethabi::Token::FixedArray(v)
                        },
                        {
                            let v = self
                                .boost_multipliers
                                .iter()
                                .map(|inner| ethabi::Token::Uint(inner.clone()))
                                .collect();
                            ethabi::Token::FixedArray(v)
                        },
                        {
                            let v = self
                                .great_portal_capacity
                                .iter()
                                .map(|inner| ethabi::Token::Uint(inner.clone()))
                                .collect();
                            ethabi::Token::FixedArray(v)
                        },
                        ethabi::Token::Address(
                            ethabi::Address::from_slice(&self.installations_diamond),
                        ),
                        ethabi::Token::Address(
                            ethabi::Address::from_slice(&self.vrf_coordinator),
                        ),
                        ethabi::Token::Address(
                            ethabi::Address::from_slice(&self.link_address),
                        ),
                        {
                            let v = self
                                .alchemica_addresses
                                .iter()
                                .map(|inner| ethabi::Token::Address(
                                    ethabi::Address::from_slice(&inner),
                                ))
                                .collect();
                            ethabi::Token::FixedArray(v)
                        },
                        ethabi::Token::Address(
                            ethabi::Address::from_slice(&self.gltr_address),
                        ),
                        ethabi::Token::Bytes(self.backend_pub_key.clone()),
                        ethabi::Token::Address(
                            ethabi::Address::from_slice(&self.game_manager),
                        ),
                        ethabi::Token::Address(
                            ethabi::Address::from_slice(&self.tile_diamond),
                        ),
                        ethabi::Token::Address(
                            ethabi::Address::from_slice(&self.aavegotchi_diamond),
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
        impl substreams_ethereum::Function for SetVars {
            const NAME: &'static str = "setVars";
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
        pub struct StartSurveying {
            pub realm_id: ethabi::Uint,
        }
        impl StartSurveying {
            const METHOD_ID: [u8; 4] = [131u8, 116u8, 68u8, 57u8];
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
                    realm_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(&[ethabi::Token::Uint(self.realm_id)]);
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
        impl substreams_ethereum::Function for StartSurveying {
            const NAME: &'static str = "startSurveying";
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
        pub struct SubUpgradeQueueLength {
            pub realm_id: ethabi::Uint,
        }
        impl SubUpgradeQueueLength {
            const METHOD_ID: [u8; 4] = [117u8, 67u8, 140u8, 212u8];
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
                    realm_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(&[ethabi::Token::Uint(self.realm_id)]);
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
        impl substreams_ethereum::Function for SubUpgradeQueueLength {
            const NAME: &'static str = "subUpgradeQueueLength";
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
        pub struct Subscribe {}
        impl Subscribe {
            const METHOD_ID: [u8; 4] = [143u8, 68u8, 154u8, 5u8];
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
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for Subscribe {
            const NAME: &'static str = "subscribe";
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
        pub struct ToggleNftDisplayAllowed {
            pub tokens: Vec<Vec<u8>>,
            pub chain_ids: Vec<ethabi::Uint>,
            pub allow: Vec<bool>,
        }
        impl ToggleNftDisplayAllowed {
            const METHOD_ID: [u8; 4] = [194u8, 111u8, 187u8, 167u8];
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
                            ethabi::ParamType::Array(Box::new(ethabi::ParamType::Bool)),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    tokens: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| {
                            inner.into_address().expect(INTERNAL_ERR).as_bytes().to_vec()
                        })
                        .collect(),
                    chain_ids: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
                    allow: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_bool().expect(INTERNAL_ERR))
                        .collect(),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        {
                            let v = self
                                .tokens
                                .iter()
                                .map(|inner| ethabi::Token::Address(
                                    ethabi::Address::from_slice(&inner),
                                ))
                                .collect();
                            ethabi::Token::Array(v)
                        },
                        {
                            let v = self
                                .chain_ids
                                .iter()
                                .map(|inner| ethabi::Token::Uint(inner.clone()))
                                .collect();
                            ethabi::Token::Array(v)
                        },
                        {
                            let v = self
                                .allow
                                .iter()
                                .map(|inner| ethabi::Token::Bool(inner))
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
        impl substreams_ethereum::Function for ToggleNftDisplayAllowed {
            const NAME: &'static str = "toggleNftDisplayAllowed";
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
        pub struct TopUpSubscription {
            pub amount: ethabi::Uint,
        }
        impl TopUpSubscription {
            const METHOD_ID: [u8; 4] = [134u8, 133u8, 14u8, 147u8];
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
                    amount: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(&[ethabi::Token::Uint(self.amount)]);
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
        impl substreams_ethereum::Function for TopUpSubscription {
            const NAME: &'static str = "topUpSubscription";
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
        pub struct UnequipInstallation {
            pub realm_id: ethabi::Uint,
            pub gotchi_id: ethabi::Uint,
            pub installation_id: ethabi::Uint,
            pub x: ethabi::Uint,
            pub y: ethabi::Uint,
            pub signature: Vec<u8>,
        }
        impl UnequipInstallation {
            const METHOD_ID: [u8; 4] = [113u8, 211u8, 172u8, 245u8];
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
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Bytes,
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    realm_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    gotchi_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    installation_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    x: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    y: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    signature: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_bytes()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Uint(self.realm_id),
                        ethabi::Token::Uint(self.gotchi_id),
                        ethabi::Token::Uint(self.installation_id),
                        ethabi::Token::Uint(self.x),
                        ethabi::Token::Uint(self.y),
                        ethabi::Token::Bytes(self.signature.clone()),
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
        impl substreams_ethereum::Function for UnequipInstallation {
            const NAME: &'static str = "unequipInstallation";
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
        pub struct UnequipTile {
            pub realm_id: ethabi::Uint,
            pub gotchi_id: ethabi::Uint,
            pub tile_id: ethabi::Uint,
            pub x: ethabi::Uint,
            pub y: ethabi::Uint,
            pub signature: Vec<u8>,
        }
        impl UnequipTile {
            const METHOD_ID: [u8; 4] = [6u8, 246u8, 40u8, 18u8];
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
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Bytes,
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    realm_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    gotchi_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    tile_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    x: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    y: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    signature: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_bytes()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Uint(self.realm_id),
                        ethabi::Token::Uint(self.gotchi_id),
                        ethabi::Token::Uint(self.tile_id),
                        ethabi::Token::Uint(self.x),
                        ethabi::Token::Uint(self.y),
                        ethabi::Token::Bytes(self.signature.clone()),
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
        impl substreams_ethereum::Function for UnequipTile {
            const NAME: &'static str = "unequipTile";
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
        pub struct UpdateEvent {
            pub realm_id: ethabi::Uint,
            pub alchemica_spent: [ethabi::Uint; 4usize],
            pub duration_extension_in_minutes: ethabi::Uint,
        }
        impl UpdateEvent {
            const METHOD_ID: [u8; 4] = [183u8, 160u8, 107u8, 48u8];
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
                                Box::new(ethabi::ParamType::Uint(256usize)),
                                4usize,
                            ),
                            ethabi::ParamType::Uint(40usize),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    realm_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    alchemica_spent: {
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
                        ]
                    },
                    duration_extension_in_minutes: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Uint(self.realm_id),
                        {
                            let v = self
                                .alchemica_spent
                                .iter()
                                .map(|inner| ethabi::Token::Uint(inner.clone()))
                                .collect();
                            ethabi::Token::FixedArray(v)
                        },
                        ethabi::Token::Uint(self.duration_extension_in_minutes),
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
        impl substreams_ethereum::Function for UpdateEvent {
            const NAME: &'static str = "updateEvent";
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
        pub struct UpgradeInstallation {
            pub realm_id: ethabi::Uint,
            pub prev_installation_id: ethabi::Uint,
            pub next_installation_id: ethabi::Uint,
            pub coordinate_x: ethabi::Uint,
            pub coordinate_y: ethabi::Uint,
        }
        impl UpgradeInstallation {
            const METHOD_ID: [u8; 4] = [85u8, 5u8, 66u8, 27u8];
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
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    realm_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    prev_installation_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    next_installation_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    coordinate_x: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    coordinate_y: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Uint(self.realm_id),
                        ethabi::Token::Uint(self.prev_installation_id),
                        ethabi::Token::Uint(self.next_installation_id),
                        ethabi::Token::Uint(self.coordinate_x),
                        ethabi::Token::Uint(self.coordinate_y),
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
        impl substreams_ethereum::Function for UpgradeInstallation {
            const NAME: &'static str = "upgradeInstallation";
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
        pub struct VerifyAccessRight {
            pub realm_id: ethabi::Uint,
            pub gotchi_id: ethabi::Uint,
            pub action_right: ethabi::Uint,
            pub sender: Vec<u8>,
        }
        impl VerifyAccessRight {
            const METHOD_ID: [u8; 4] = [228u8, 250u8, 171u8, 91u8];
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
                            ethabi::ParamType::Address,
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    realm_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    gotchi_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    action_right: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    sender: values
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
                        ethabi::Token::Uint(self.realm_id),
                        ethabi::Token::Uint(self.gotchi_id),
                        ethabi::Token::Uint(self.action_right),
                        ethabi::Token::Address(ethabi::Address::from_slice(&self.sender)),
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
        impl substreams_ethereum::Function for VerifyAccessRight {
            const NAME: &'static str = "verifyAccessRight";
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
    }
    /// Contract's events.
    #[allow(dead_code)]
    pub mod events {
        use substreams_ethereum::scalar::EthBigInt;
        use super::INTERNAL_ERR;
        #[derive(Debug, Clone, PartialEq)]
        pub struct AavegotchiDiamondUpdated {
            pub aavegotchi_diamond: Vec<u8>,
        }
        impl AavegotchiDiamondUpdated {
            const TOPIC_ID: [u8; 32] = [
                128u8,
                58u8,
                56u8,
                155u8,
                149u8,
                39u8,
                105u8,
                215u8,
                36u8,
                62u8,
                225u8,
                91u8,
                86u8,
                182u8,
                66u8,
                181u8,
                93u8,
                176u8,
                196u8,
                130u8,
                34u8,
                168u8,
                119u8,
                124u8,
                83u8,
                35u8,
                180u8,
                252u8,
                104u8,
                121u8,
                241u8,
                221u8,
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
                        &[ethabi::ParamType::Address],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    aavegotchi_diamond: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                })
            }
        }
        impl substreams_ethereum::Event for AavegotchiDiamondUpdated {
            const NAME: &'static str = "AavegotchiDiamondUpdated";
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
        pub struct AlchemicaClaimed1 {
            pub realm_id: ethabi::Uint,
            pub gotchi_id: ethabi::Uint,
            pub alchemica_type: ethabi::Uint,
            pub amount: ethabi::Uint,
            pub spillover_rate: ethabi::Uint,
            pub spillover_radius: ethabi::Uint,
        }
        impl AlchemicaClaimed1 {
            const TOPIC_ID: [u8; 32] = [
                205u8,
                55u8,
                222u8,
                191u8,
                140u8,
                21u8,
                175u8,
                204u8,
                243u8,
                177u8,
                150u8,
                169u8,
                112u8,
                242u8,
                8u8,
                99u8,
                19u8,
                158u8,
                36u8,
                42u8,
                45u8,
                120u8,
                228u8,
                54u8,
                56u8,
                165u8,
                54u8,
                211u8,
                117u8,
                207u8,
                74u8,
                219u8,
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
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    realm_id: ethabi::decode(
                            &[ethabi::ParamType::Uint(256usize)],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'realm_id' from topic of type 'uint256': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    gotchi_id: ethabi::decode(
                            &[ethabi::ParamType::Uint(256usize)],
                            log.topics[2usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'gotchi_id' from topic of type 'uint256': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    alchemica_type: ethabi::decode(
                            &[ethabi::ParamType::Uint(256usize)],
                            log.topics[3usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'alchemica_type' from topic of type 'uint256': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    amount: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    spillover_rate: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    spillover_radius: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for AlchemicaClaimed1 {
            const NAME: &'static str = "AlchemicaClaimed1";
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
        pub struct AlchemicaClaimed2 {
            pub realm_id: ethabi::Uint,
            pub gotchi_id: ethabi::Uint,
            pub alchemica_type: ethabi::Uint,
            pub amount: ethabi::Uint,
            pub spillover_rate: ethabi::Uint,
            pub spillover_radius: ethabi::Uint,
        }
        impl AlchemicaClaimed2 {
            const TOPIC_ID: [u8; 32] = [
                205u8,
                55u8,
                222u8,
                191u8,
                140u8,
                21u8,
                175u8,
                204u8,
                243u8,
                177u8,
                150u8,
                169u8,
                112u8,
                242u8,
                8u8,
                99u8,
                19u8,
                158u8,
                36u8,
                42u8,
                45u8,
                120u8,
                228u8,
                54u8,
                56u8,
                165u8,
                54u8,
                211u8,
                117u8,
                207u8,
                74u8,
                219u8,
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
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    realm_id: ethabi::decode(
                            &[ethabi::ParamType::Uint(256usize)],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'realm_id' from topic of type 'uint256': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    gotchi_id: ethabi::decode(
                            &[ethabi::ParamType::Uint(256usize)],
                            log.topics[2usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'gotchi_id' from topic of type 'uint256': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    alchemica_type: ethabi::decode(
                            &[ethabi::ParamType::Uint(256usize)],
                            log.topics[3usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'alchemica_type' from topic of type 'uint256': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    amount: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    spillover_rate: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    spillover_radius: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for AlchemicaClaimed2 {
            const NAME: &'static str = "AlchemicaClaimed2";
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
        pub struct Approval1 {
            pub owner: Vec<u8>,
            pub approved: Vec<u8>,
            pub token_id: ethabi::Uint,
        }
        impl Approval1 {
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
        impl substreams_ethereum::Event for Approval1 {
            const NAME: &'static str = "Approval1";
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
        pub struct Approval2 {
            pub owner: Vec<u8>,
            pub approved: Vec<u8>,
            pub token_id: ethabi::Uint,
        }
        impl Approval2 {
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
        impl substreams_ethereum::Event for Approval2 {
            const NAME: &'static str = "Approval2";
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
        pub struct ApprovalForAll3 {
            pub owner: Vec<u8>,
            pub operator: Vec<u8>,
            pub approved: bool,
        }
        impl ApprovalForAll3 {
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
        impl substreams_ethereum::Event for ApprovalForAll3 {
            const NAME: &'static str = "ApprovalForAll3";
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
        pub struct ApprovalForAll4 {
            pub owner: Vec<u8>,
            pub operator: Vec<u8>,
            pub approved: bool,
        }
        impl ApprovalForAll4 {
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
        impl substreams_ethereum::Event for ApprovalForAll4 {
            const NAME: &'static str = "ApprovalForAll4";
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
        pub struct ApprovalForAll5 {
            pub owner: Vec<u8>,
            pub operator: Vec<u8>,
            pub approved: bool,
        }
        impl ApprovalForAll5 {
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
        impl substreams_ethereum::Event for ApprovalForAll5 {
            const NAME: &'static str = "ApprovalForAll5";
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
        pub struct ApprovalForAll6 {
            pub owner: Vec<u8>,
            pub operator: Vec<u8>,
            pub approved: bool,
        }
        impl ApprovalForAll6 {
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
        impl substreams_ethereum::Event for ApprovalForAll6 {
            const NAME: &'static str = "ApprovalForAll6";
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
        pub struct ChannelAlchemica {
            pub realm_id: ethabi::Uint,
            pub gotchi_id: ethabi::Uint,
            pub alchemica: [ethabi::Uint; 4usize],
            pub spillover_rate: ethabi::Uint,
            pub spillover_radius: ethabi::Uint,
        }
        impl ChannelAlchemica {
            const TOPIC_ID: [u8; 32] = [
                144u8,
                174u8,
                176u8,
                25u8,
                145u8,
                74u8,
                157u8,
                132u8,
                238u8,
                20u8,
                41u8,
                237u8,
                155u8,
                71u8,
                166u8,
                121u8,
                239u8,
                187u8,
                140u8,
                243u8,
                136u8,
                191u8,
                161u8,
                92u8,
                219u8,
                133u8,
                187u8,
                116u8,
                80u8,
                248u8,
                19u8,
                206u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 3usize {
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
                            ethabi::ParamType::FixedArray(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                                4usize,
                            ),
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    realm_id: ethabi::decode(
                            &[ethabi::ParamType::Uint(256usize)],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'realm_id' from topic of type 'uint256': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    gotchi_id: ethabi::decode(
                            &[ethabi::ParamType::Uint(256usize)],
                            log.topics[2usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'gotchi_id' from topic of type 'uint256': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    alchemica: {
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
                        ]
                    },
                    spillover_rate: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    spillover_radius: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for ChannelAlchemica {
            const NAME: &'static str = "ChannelAlchemica";
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
        pub struct EquipInstallation {
            pub realm_id: ethabi::Uint,
            pub installation_id: ethabi::Uint,
            pub x: ethabi::Uint,
            pub y: ethabi::Uint,
        }
        impl EquipInstallation {
            const TOPIC_ID: [u8; 32] = [
                40u8,
                238u8,
                143u8,
                135u8,
                15u8,
                191u8,
                47u8,
                200u8,
                239u8,
                65u8,
                65u8,
                152u8,
                203u8,
                38u8,
                62u8,
                111u8,
                84u8,
                140u8,
                233u8,
                161u8,
                0u8,
                136u8,
                199u8,
                108u8,
                40u8,
                33u8,
                182u8,
                108u8,
                246u8,
                29u8,
                221u8,
                196u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 1usize {
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
                    realm_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    installation_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    x: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    y: values.pop().expect(INTERNAL_ERR).into_uint().expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for EquipInstallation {
            const NAME: &'static str = "EquipInstallation";
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
        pub struct EquipTile {
            pub realm_id: ethabi::Uint,
            pub tile_id: ethabi::Uint,
            pub x: ethabi::Uint,
            pub y: ethabi::Uint,
        }
        impl EquipTile {
            const TOPIC_ID: [u8; 32] = [
                252u8,
                183u8,
                215u8,
                46u8,
                148u8,
                238u8,
                228u8,
                124u8,
                254u8,
                247u8,
                132u8,
                245u8,
                213u8,
                78u8,
                31u8,
                7u8,
                204u8,
                32u8,
                223u8,
                242u8,
                249u8,
                98u8,
                125u8,
                224u8,
                111u8,
                31u8,
                6u8,
                20u8,
                40u8,
                199u8,
                201u8,
                30u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 1usize {
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
                    realm_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    tile_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    x: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    y: values.pop().expect(INTERNAL_ERR).into_uint().expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for EquipTile {
            const NAME: &'static str = "EquipTile";
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
        pub struct EventCancelled1 {
            pub event_id: ethabi::Uint,
        }
        impl EventCancelled1 {
            const TOPIC_ID: [u8; 32] = [
                220u8,
                60u8,
                181u8,
                105u8,
                176u8,
                102u8,
                70u8,
                15u8,
                74u8,
                34u8,
                223u8,
                250u8,
                5u8,
                49u8,
                123u8,
                220u8,
                206u8,
                93u8,
                56u8,
                38u8,
                56u8,
                46u8,
                212u8,
                154u8,
                33u8,
                241u8,
                62u8,
                95u8,
                162u8,
                6u8,
                61u8,
                57u8,
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
                    event_id: ethabi::decode(
                            &[ethabi::ParamType::Uint(256usize)],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'event_id' from topic of type 'uint256': {:?}",
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
        impl substreams_ethereum::Event for EventCancelled1 {
            const NAME: &'static str = "EventCancelled1";
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
        pub struct EventCancelled2 {
            pub event_id: ethabi::Uint,
        }
        impl EventCancelled2 {
            const TOPIC_ID: [u8; 32] = [
                220u8,
                60u8,
                181u8,
                105u8,
                176u8,
                102u8,
                70u8,
                15u8,
                74u8,
                34u8,
                223u8,
                250u8,
                5u8,
                49u8,
                123u8,
                220u8,
                206u8,
                93u8,
                56u8,
                38u8,
                56u8,
                46u8,
                212u8,
                154u8,
                33u8,
                241u8,
                62u8,
                95u8,
                162u8,
                6u8,
                61u8,
                57u8,
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
                    event_id: ethabi::decode(
                            &[ethabi::ParamType::Uint(256usize)],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'event_id' from topic of type 'uint256': {:?}",
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
        impl substreams_ethereum::Event for EventCancelled2 {
            const NAME: &'static str = "EventCancelled2";
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
        pub struct EventPriorityAndDurationUpdated1 {
            pub event_id: ethabi::Uint,
            pub new_priority: ethabi::Uint,
            pub new_end_time: ethabi::Uint,
        }
        impl EventPriorityAndDurationUpdated1 {
            const TOPIC_ID: [u8; 32] = [
                132u8,
                249u8,
                228u8,
                118u8,
                251u8,
                167u8,
                12u8,
                14u8,
                13u8,
                130u8,
                38u8,
                171u8,
                82u8,
                4u8,
                157u8,
                218u8,
                199u8,
                179u8,
                142u8,
                161u8,
                136u8,
                231u8,
                133u8,
                196u8,
                88u8,
                49u8,
                185u8,
                24u8,
                50u8,
                135u8,
                122u8,
                248u8,
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
                            ethabi::ParamType::Uint(120usize),
                            ethabi::ParamType::Uint(64usize),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    event_id: ethabi::decode(
                            &[ethabi::ParamType::Uint(256usize)],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'event_id' from topic of type 'uint256': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    new_priority: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    new_end_time: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for EventPriorityAndDurationUpdated1 {
            const NAME: &'static str = "EventPriorityAndDurationUpdated1";
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
        pub struct EventPriorityAndDurationUpdated2 {
            pub event_id: ethabi::Uint,
            pub new_priority: ethabi::Uint,
            pub new_end_time: ethabi::Uint,
        }
        impl EventPriorityAndDurationUpdated2 {
            const TOPIC_ID: [u8; 32] = [
                132u8,
                249u8,
                228u8,
                118u8,
                251u8,
                167u8,
                12u8,
                14u8,
                13u8,
                130u8,
                38u8,
                171u8,
                82u8,
                4u8,
                157u8,
                218u8,
                199u8,
                179u8,
                142u8,
                161u8,
                136u8,
                231u8,
                133u8,
                196u8,
                88u8,
                49u8,
                185u8,
                24u8,
                50u8,
                135u8,
                122u8,
                248u8,
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
                            ethabi::ParamType::Uint(120usize),
                            ethabi::ParamType::Uint(64usize),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    event_id: ethabi::decode(
                            &[ethabi::ParamType::Uint(256usize)],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'event_id' from topic of type 'uint256': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    new_priority: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    new_end_time: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for EventPriorityAndDurationUpdated2 {
            const NAME: &'static str = "EventPriorityAndDurationUpdated2";
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
        pub struct ExitAlchemica {
            pub gotchi_id: ethabi::Uint,
            pub alchemica: Vec<ethabi::Uint>,
        }
        impl ExitAlchemica {
            const TOPIC_ID: [u8; 32] = [
                242u8,
                3u8,
                190u8,
                225u8,
                173u8,
                223u8,
                30u8,
                85u8,
                47u8,
                171u8,
                238u8,
                18u8,
                23u8,
                7u8,
                25u8,
                191u8,
                205u8,
                234u8,
                119u8,
                245u8,
                108u8,
                6u8,
                66u8,
                136u8,
                6u8,
                206u8,
                20u8,
                97u8,
                68u8,
                63u8,
                230u8,
                221u8,
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
                    gotchi_id: ethabi::decode(
                            &[ethabi::ParamType::Uint(256usize)],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'gotchi_id' from topic of type 'uint256': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    alchemica: values
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
        impl substreams_ethereum::Event for ExitAlchemica {
            const NAME: &'static str = "ExitAlchemica";
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
        pub struct InstallationUpgraded {
            pub realm_id: ethabi::Uint,
            pub prev_installation_id: ethabi::Uint,
            pub next_installation_id: ethabi::Uint,
            pub coordinate_x: ethabi::Uint,
            pub coordinate_y: ethabi::Uint,
        }
        impl InstallationUpgraded {
            const TOPIC_ID: [u8; 32] = [
                116u8,
                182u8,
                84u8,
                97u8,
                78u8,
                162u8,
                68u8,
                75u8,
                135u8,
                106u8,
                20u8,
                222u8,
                193u8,
                28u8,
                112u8,
                152u8,
                7u8,
                205u8,
                168u8,
                144u8,
                96u8,
                144u8,
                108u8,
                75u8,
                126u8,
                68u8,
                62u8,
                79u8,
                143u8,
                249u8,
                237u8,
                83u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 1usize {
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
                            ethabi::ParamType::Uint(256usize),
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
                    realm_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    prev_installation_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    next_installation_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    coordinate_x: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    coordinate_y: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for InstallationUpgraded {
            const NAME: &'static str = "InstallationUpgraded";
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
        pub struct MintInstallation1 {
            pub owner: Vec<u8>,
            pub installation_type: ethabi::Uint,
            pub installation_id: ethabi::Uint,
        }
        impl MintInstallation1 {
            const TOPIC_ID: [u8; 32] = [
                242u8,
                189u8,
                122u8,
                10u8,
                69u8,
                135u8,
                180u8,
                192u8,
                0u8,
                238u8,
                54u8,
                173u8,
                86u8,
                129u8,
                198u8,
                124u8,
                127u8,
                202u8,
                217u8,
                208u8,
                10u8,
                128u8,
                191u8,
                151u8,
                90u8,
                194u8,
                227u8,
                21u8,
                248u8,
                87u8,
                124u8,
                77u8,
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
                    installation_type: ethabi::decode(
                            &[ethabi::ParamType::Uint(256usize)],
                            log.topics[2usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'installation_type' from topic of type 'uint256': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    installation_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for MintInstallation1 {
            const NAME: &'static str = "MintInstallation1";
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
        pub struct MintInstallation2 {
            pub owner: Vec<u8>,
            pub installation_type: ethabi::Uint,
            pub installation_id: ethabi::Uint,
        }
        impl MintInstallation2 {
            const TOPIC_ID: [u8; 32] = [
                242u8,
                189u8,
                122u8,
                10u8,
                69u8,
                135u8,
                180u8,
                192u8,
                0u8,
                238u8,
                54u8,
                173u8,
                86u8,
                129u8,
                198u8,
                124u8,
                127u8,
                202u8,
                217u8,
                208u8,
                10u8,
                128u8,
                191u8,
                151u8,
                90u8,
                194u8,
                227u8,
                21u8,
                248u8,
                87u8,
                124u8,
                77u8,
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
                    installation_type: ethabi::decode(
                            &[ethabi::ParamType::Uint(256usize)],
                            log.topics[2usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'installation_type' from topic of type 'uint256': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    installation_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for MintInstallation2 {
            const NAME: &'static str = "MintInstallation2";
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
        pub struct MintInstallations1 {
            pub owner: Vec<u8>,
            pub installation_id: ethabi::Uint,
            pub amount: ethabi::Uint,
        }
        impl MintInstallations1 {
            const TOPIC_ID: [u8; 32] = [
                7u8,
                214u8,
                223u8,
                222u8,
                3u8,
                138u8,
                204u8,
                77u8,
                178u8,
                120u8,
                188u8,
                67u8,
                51u8,
                165u8,
                147u8,
                117u8,
                184u8,
                6u8,
                222u8,
                236u8,
                102u8,
                78u8,
                70u8,
                156u8,
                107u8,
                9u8,
                241u8,
                122u8,
                115u8,
                186u8,
                250u8,
                222u8,
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
                        &[ethabi::ParamType::Uint(16usize)],
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
                    installation_id: ethabi::decode(
                            &[ethabi::ParamType::Uint(256usize)],
                            log.topics[2usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'installation_id' from topic of type 'uint256': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    amount: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for MintInstallations1 {
            const NAME: &'static str = "MintInstallations1";
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
        pub struct MintInstallations2 {
            pub owner: Vec<u8>,
            pub installation_id: ethabi::Uint,
            pub amount: ethabi::Uint,
        }
        impl MintInstallations2 {
            const TOPIC_ID: [u8; 32] = [
                7u8,
                214u8,
                223u8,
                222u8,
                3u8,
                138u8,
                204u8,
                77u8,
                178u8,
                120u8,
                188u8,
                67u8,
                51u8,
                165u8,
                147u8,
                117u8,
                184u8,
                6u8,
                222u8,
                236u8,
                102u8,
                78u8,
                70u8,
                156u8,
                107u8,
                9u8,
                241u8,
                122u8,
                115u8,
                186u8,
                250u8,
                222u8,
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
                        &[ethabi::ParamType::Uint(16usize)],
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
                    installation_id: ethabi::decode(
                            &[ethabi::ParamType::Uint(256usize)],
                            log.topics[2usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'installation_id' from topic of type 'uint256': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    amount: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for MintInstallations2 {
            const NAME: &'static str = "MintInstallations2";
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
        pub struct MintParcel1 {
            pub owner: Vec<u8>,
            pub token_id: ethabi::Uint,
        }
        impl MintParcel1 {
            const TOPIC_ID: [u8; 32] = [
                22u8,
                97u8,
                233u8,
                235u8,
                112u8,
                195u8,
                81u8,
                66u8,
                251u8,
                180u8,
                25u8,
                15u8,
                160u8,
                109u8,
                150u8,
                38u8,
                54u8,
                207u8,
                156u8,
                205u8,
                136u8,
                221u8,
                145u8,
                63u8,
                144u8,
                192u8,
                139u8,
                203u8,
                240u8,
                212u8,
                62u8,
                145u8,
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
                    token_id: ethabi::decode(
                            &[ethabi::ParamType::Uint(256usize)],
                            log.topics[2usize].as_ref(),
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
        impl substreams_ethereum::Event for MintParcel1 {
            const NAME: &'static str = "MintParcel1";
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
        pub struct MintParcel2 {
            pub owner: Vec<u8>,
            pub token_id: ethabi::Uint,
        }
        impl MintParcel2 {
            const TOPIC_ID: [u8; 32] = [
                22u8,
                97u8,
                233u8,
                235u8,
                112u8,
                195u8,
                81u8,
                66u8,
                251u8,
                180u8,
                25u8,
                15u8,
                160u8,
                109u8,
                150u8,
                38u8,
                54u8,
                207u8,
                156u8,
                205u8,
                136u8,
                221u8,
                145u8,
                63u8,
                144u8,
                192u8,
                139u8,
                203u8,
                240u8,
                212u8,
                62u8,
                145u8,
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
                    token_id: ethabi::decode(
                            &[ethabi::ParamType::Uint(256usize)],
                            log.topics[2usize].as_ref(),
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
        impl substreams_ethereum::Event for MintParcel2 {
            const NAME: &'static str = "MintParcel2";
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
        pub struct MintTile1 {
            pub owner: Vec<u8>,
            pub tile_type: ethabi::Uint,
            pub tile_id: ethabi::Uint,
        }
        impl MintTile1 {
            const TOPIC_ID: [u8; 32] = [
                81u8,
                101u8,
                126u8,
                85u8,
                40u8,
                226u8,
                101u8,
                97u8,
                99u8,
                111u8,
                242u8,
                225u8,
                80u8,
                162u8,
                5u8,
                172u8,
                197u8,
                134u8,
                148u8,
                55u8,
                254u8,
                127u8,
                104u8,
                31u8,
                144u8,
                106u8,
                149u8,
                136u8,
                126u8,
                63u8,
                14u8,
                172u8,
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
                    tile_type: ethabi::decode(
                            &[ethabi::ParamType::Uint(256usize)],
                            log.topics[2usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'tile_type' from topic of type 'uint256': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    tile_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for MintTile1 {
            const NAME: &'static str = "MintTile1";
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
        pub struct MintTile2 {
            pub owner: Vec<u8>,
            pub tile_type: ethabi::Uint,
            pub tile_id: ethabi::Uint,
        }
        impl MintTile2 {
            const TOPIC_ID: [u8; 32] = [
                81u8,
                101u8,
                126u8,
                85u8,
                40u8,
                226u8,
                101u8,
                97u8,
                99u8,
                111u8,
                242u8,
                225u8,
                80u8,
                162u8,
                5u8,
                172u8,
                197u8,
                134u8,
                148u8,
                55u8,
                254u8,
                127u8,
                104u8,
                31u8,
                144u8,
                106u8,
                149u8,
                136u8,
                126u8,
                63u8,
                14u8,
                172u8,
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
                    tile_type: ethabi::decode(
                            &[ethabi::ParamType::Uint(256usize)],
                            log.topics[2usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'tile_type' from topic of type 'uint256': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    tile_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for MintTile2 {
            const NAME: &'static str = "MintTile2";
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
        pub struct MintTiles1 {
            pub owner: Vec<u8>,
            pub tile_id: ethabi::Uint,
            pub amount: ethabi::Uint,
        }
        impl MintTiles1 {
            const TOPIC_ID: [u8; 32] = [
                207u8,
                78u8,
                194u8,
                154u8,
                33u8,
                223u8,
                74u8,
                131u8,
                62u8,
                9u8,
                0u8,
                131u8,
                132u8,
                193u8,
                0u8,
                243u8,
                189u8,
                231u8,
                16u8,
                23u8,
                195u8,
                134u8,
                141u8,
                211u8,
                184u8,
                158u8,
                45u8,
                75u8,
                134u8,
                220u8,
                95u8,
                252u8,
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
                        &[ethabi::ParamType::Uint(16usize)],
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
                    tile_id: ethabi::decode(
                            &[ethabi::ParamType::Uint(256usize)],
                            log.topics[2usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'tile_id' from topic of type 'uint256': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    amount: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for MintTiles1 {
            const NAME: &'static str = "MintTiles1";
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
        pub struct MintTiles2 {
            pub owner: Vec<u8>,
            pub tile_id: ethabi::Uint,
            pub amount: ethabi::Uint,
        }
        impl MintTiles2 {
            const TOPIC_ID: [u8; 32] = [
                207u8,
                78u8,
                194u8,
                154u8,
                33u8,
                223u8,
                74u8,
                131u8,
                62u8,
                9u8,
                0u8,
                131u8,
                132u8,
                193u8,
                0u8,
                243u8,
                189u8,
                231u8,
                16u8,
                23u8,
                195u8,
                134u8,
                141u8,
                211u8,
                184u8,
                158u8,
                45u8,
                75u8,
                134u8,
                220u8,
                95u8,
                252u8,
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
                        &[ethabi::ParamType::Uint(16usize)],
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
                    tile_id: ethabi::decode(
                            &[ethabi::ParamType::Uint(256usize)],
                            log.topics[2usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'tile_id' from topic of type 'uint256': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    amount: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for MintTiles2 {
            const NAME: &'static str = "MintTiles2";
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
        pub struct NftDisplayStatusUpdated {
            pub token: Vec<u8>,
            pub chain_id: ethabi::Uint,
            pub allowed: bool,
        }
        impl NftDisplayStatusUpdated {
            const TOPIC_ID: [u8; 32] = [
                128u8,
                6u8,
                41u8,
                56u8,
                250u8,
                62u8,
                76u8,
                188u8,
                195u8,
                1u8,
                110u8,
                214u8,
                202u8,
                30u8,
                201u8,
                235u8,
                194u8,
                80u8,
                70u8,
                118u8,
                152u8,
                129u8,
                28u8,
                76u8,
                15u8,
                202u8,
                139u8,
                187u8,
                156u8,
                249u8,
                150u8,
                241u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 1usize {
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
                            ethabi::ParamType::Bool,
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    token: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    chain_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    allowed: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_bool()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for NftDisplayStatusUpdated {
            const NAME: &'static str = "NFTDisplayStatusUpdated";
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
        pub struct OwnershipTransferred1 {
            pub previous_owner: Vec<u8>,
            pub new_owner: Vec<u8>,
        }
        impl OwnershipTransferred1 {
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
        impl substreams_ethereum::Event for OwnershipTransferred1 {
            const NAME: &'static str = "OwnershipTransferred1";
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
        pub struct OwnershipTransferred2 {
            pub previous_owner: Vec<u8>,
            pub new_owner: Vec<u8>,
        }
        impl OwnershipTransferred2 {
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
        impl substreams_ethereum::Event for OwnershipTransferred2 {
            const NAME: &'static str = "OwnershipTransferred2";
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
        pub struct ParcelAccessRightSet1 {
            pub realm_id: ethabi::Uint,
            pub action_right: ethabi::Uint,
            pub access_right: ethabi::Uint,
        }
        impl ParcelAccessRightSet1 {
            const TOPIC_ID: [u8; 32] = [
                234u8,
                123u8,
                168u8,
                239u8,
                117u8,
                203u8,
                53u8,
                156u8,
                18u8,
                90u8,
                222u8,
                206u8,
                99u8,
                89u8,
                95u8,
                127u8,
                238u8,
                6u8,
                250u8,
                41u8,
                90u8,
                203u8,
                148u8,
                180u8,
                86u8,
                11u8,
                249u8,
                251u8,
                248u8,
                102u8,
                192u8,
                95u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 1usize {
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
                    realm_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    action_right: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    access_right: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for ParcelAccessRightSet1 {
            const NAME: &'static str = "ParcelAccessRightSet1";
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
        pub struct ParcelAccessRightSet2 {
            pub realm_id: ethabi::Uint,
            pub action_right: ethabi::Uint,
            pub access_right: ethabi::Uint,
        }
        impl ParcelAccessRightSet2 {
            const TOPIC_ID: [u8; 32] = [
                234u8,
                123u8,
                168u8,
                239u8,
                117u8,
                203u8,
                53u8,
                156u8,
                18u8,
                90u8,
                222u8,
                206u8,
                99u8,
                89u8,
                95u8,
                127u8,
                238u8,
                6u8,
                250u8,
                41u8,
                90u8,
                203u8,
                148u8,
                180u8,
                86u8,
                11u8,
                249u8,
                251u8,
                248u8,
                102u8,
                192u8,
                95u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 1usize {
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
                    realm_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    action_right: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    access_right: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for ParcelAccessRightSet2 {
            const NAME: &'static str = "ParcelAccessRightSet2";
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
        pub struct ParcelAccessRightSet3 {
            pub realm_id: ethabi::Uint,
            pub action_right: ethabi::Uint,
            pub access_right: ethabi::Uint,
        }
        impl ParcelAccessRightSet3 {
            const TOPIC_ID: [u8; 32] = [
                234u8,
                123u8,
                168u8,
                239u8,
                117u8,
                203u8,
                53u8,
                156u8,
                18u8,
                90u8,
                222u8,
                206u8,
                99u8,
                89u8,
                95u8,
                127u8,
                238u8,
                6u8,
                250u8,
                41u8,
                90u8,
                203u8,
                148u8,
                180u8,
                86u8,
                11u8,
                249u8,
                251u8,
                248u8,
                102u8,
                192u8,
                95u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 1usize {
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
                    realm_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    action_right: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    access_right: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for ParcelAccessRightSet3 {
            const NAME: &'static str = "ParcelAccessRightSet3";
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
        pub struct ParcelWhitelistSet {
            pub realm_id: ethabi::Uint,
            pub action_right: ethabi::Uint,
            pub whitelist_id: ethabi::Uint,
        }
        impl ParcelWhitelistSet {
            const TOPIC_ID: [u8; 32] = [
                73u8,
                252u8,
                233u8,
                227u8,
                63u8,
                173u8,
                71u8,
                130u8,
                25u8,
                221u8,
                212u8,
                2u8,
                236u8,
                227u8,
                144u8,
                75u8,
                106u8,
                141u8,
                186u8,
                255u8,
                206u8,
                254u8,
                8u8,
                158u8,
                44u8,
                14u8,
                22u8,
                133u8,
                66u8,
                42u8,
                76u8,
                5u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 1usize {
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
                    realm_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    action_right: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    whitelist_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for ParcelWhitelistSet {
            const NAME: &'static str = "ParcelWhitelistSet";
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
        pub struct ResyncParcel {
            pub realm_id: ethabi::Uint,
        }
        impl ResyncParcel {
            const TOPIC_ID: [u8; 32] = [
                242u8,
                15u8,
                173u8,
                74u8,
                154u8,
                164u8,
                124u8,
                238u8,
                178u8,
                154u8,
                127u8,
                105u8,
                59u8,
                137u8,
                135u8,
                29u8,
                228u8,
                179u8,
                185u8,
                237u8,
                47u8,
                193u8,
                253u8,
                243u8,
                39u8,
                120u8,
                84u8,
                118u8,
                249u8,
                147u8,
                140u8,
                151u8,
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
                    realm_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for ResyncParcel {
            const NAME: &'static str = "ResyncParcel";
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
        pub struct SetAltarId {
            pub realm_id: ethabi::Uint,
            pub altar_id: ethabi::Uint,
        }
        impl SetAltarId {
            const TOPIC_ID: [u8; 32] = [
                150u8,
                47u8,
                31u8,
                20u8,
                131u8,
                75u8,
                148u8,
                96u8,
                191u8,
                252u8,
                221u8,
                68u8,
                160u8,
                254u8,
                240u8,
                216u8,
                102u8,
                211u8,
                194u8,
                210u8,
                194u8,
                116u8,
                58u8,
                239u8,
                129u8,
                7u8,
                12u8,
                88u8,
                57u8,
                106u8,
                192u8,
                49u8,
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
                    realm_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    altar_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for SetAltarId {
            const NAME: &'static str = "SetAltarId";
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
        pub struct StartSurveying {
            pub realm_id: ethabi::Uint,
            pub round: ethabi::Uint,
        }
        impl StartSurveying {
            const TOPIC_ID: [u8; 32] = [
                149u8,
                3u8,
                88u8,
                158u8,
                184u8,
                160u8,
                16u8,
                50u8,
                158u8,
                39u8,
                166u8,
                236u8,
                166u8,
                193u8,
                149u8,
                114u8,
                157u8,
                111u8,
                141u8,
                144u8,
                40u8,
                10u8,
                199u8,
                28u8,
                234u8,
                93u8,
                172u8,
                65u8,
                235u8,
                166u8,
                162u8,
                164u8,
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
                    realm_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    round: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for StartSurveying {
            const NAME: &'static str = "StartSurveying";
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
        pub struct SurveyParcel1 {
            pub token_id: ethabi::Uint,
            pub round: ethabi::Uint,
            pub alchemicas: Vec<ethabi::Uint>,
        }
        impl SurveyParcel1 {
            const TOPIC_ID: [u8; 32] = [
                97u8,
                251u8,
                103u8,
                12u8,
                177u8,
                12u8,
                100u8,
                36u8,
                63u8,
                184u8,
                160u8,
                155u8,
                61u8,
                30u8,
                34u8,
                205u8,
                55u8,
                201u8,
                198u8,
                228u8,
                11u8,
                94u8,
                28u8,
                119u8,
                247u8,
                134u8,
                194u8,
                49u8,
                43u8,
                118u8,
                99u8,
                86u8,
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
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                            ),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    token_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    round: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    alchemicas: values
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
        impl substreams_ethereum::Event for SurveyParcel1 {
            const NAME: &'static str = "SurveyParcel1";
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
        pub struct SurveyParcel2 {
            pub token_id: ethabi::Uint,
            pub round: ethabi::Uint,
            pub alchemicas: Vec<ethabi::Uint>,
        }
        impl SurveyParcel2 {
            const TOPIC_ID: [u8; 32] = [
                97u8,
                251u8,
                103u8,
                12u8,
                177u8,
                12u8,
                100u8,
                36u8,
                63u8,
                184u8,
                160u8,
                155u8,
                61u8,
                30u8,
                34u8,
                205u8,
                55u8,
                201u8,
                198u8,
                228u8,
                11u8,
                94u8,
                28u8,
                119u8,
                247u8,
                134u8,
                194u8,
                49u8,
                43u8,
                118u8,
                99u8,
                86u8,
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
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                            ),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    token_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    round: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    alchemicas: values
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
        impl substreams_ethereum::Event for SurveyParcel2 {
            const NAME: &'static str = "SurveyParcel2";
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
        pub struct SurveyingRoundProgressed {
            pub new_round: ethabi::Uint,
        }
        impl SurveyingRoundProgressed {
            const TOPIC_ID: [u8; 32] = [
                27u8,
                234u8,
                178u8,
                238u8,
                132u8,
                216u8,
                223u8,
                229u8,
                124u8,
                42u8,
                95u8,
                104u8,
                185u8,
                164u8,
                187u8,
                20u8,
                187u8,
                115u8,
                108u8,
                5u8,
                99u8,
                191u8,
                47u8,
                49u8,
                173u8,
                193u8,
                223u8,
                207u8,
                186u8,
                23u8,
                234u8,
                115u8,
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
                    new_round: ethabi::decode(
                            &[ethabi::ParamType::Uint(256usize)],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'new_round' from topic of type 'uint256': {:?}",
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
        impl substreams_ethereum::Event for SurveyingRoundProgressed {
            const NAME: &'static str = "SurveyingRoundProgressed";
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
        pub struct Transfer1 {
            pub from: Vec<u8>,
            pub to: Vec<u8>,
            pub token_id: ethabi::Uint,
        }
        impl Transfer1 {
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
        impl substreams_ethereum::Event for Transfer1 {
            const NAME: &'static str = "Transfer1";
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
        pub struct Transfer2 {
            pub from: Vec<u8>,
            pub to: Vec<u8>,
            pub token_id: ethabi::Uint,
        }
        impl Transfer2 {
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
        impl substreams_ethereum::Event for Transfer2 {
            const NAME: &'static str = "Transfer2";
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
        pub struct TransferBatch1 {
            pub operator: Vec<u8>,
            pub from: Vec<u8>,
            pub to: Vec<u8>,
            pub ids: Vec<ethabi::Uint>,
            pub values: Vec<ethabi::Uint>,
        }
        impl TransferBatch1 {
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
        impl substreams_ethereum::Event for TransferBatch1 {
            const NAME: &'static str = "TransferBatch1";
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
        pub struct TransferBatch2 {
            pub operator: Vec<u8>,
            pub from: Vec<u8>,
            pub to: Vec<u8>,
            pub ids: Vec<ethabi::Uint>,
            pub values: Vec<ethabi::Uint>,
        }
        impl TransferBatch2 {
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
        impl substreams_ethereum::Event for TransferBatch2 {
            const NAME: &'static str = "TransferBatch2";
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
        pub struct TransferBatch3 {
            pub operator: Vec<u8>,
            pub from: Vec<u8>,
            pub to: Vec<u8>,
            pub ids: Vec<ethabi::Uint>,
            pub values: Vec<ethabi::Uint>,
        }
        impl TransferBatch3 {
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
        impl substreams_ethereum::Event for TransferBatch3 {
            const NAME: &'static str = "TransferBatch3";
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
        pub struct TransferBatch4 {
            pub operator: Vec<u8>,
            pub from: Vec<u8>,
            pub to: Vec<u8>,
            pub ids: Vec<ethabi::Uint>,
            pub values: Vec<ethabi::Uint>,
        }
        impl TransferBatch4 {
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
        impl substreams_ethereum::Event for TransferBatch4 {
            const NAME: &'static str = "TransferBatch4";
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
        pub struct TransferFromParent1 {
            pub from_contract: Vec<u8>,
            pub from_token_id: ethabi::Uint,
            pub token_type_id: ethabi::Uint,
            pub value: ethabi::Uint,
        }
        impl TransferFromParent1 {
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
        impl substreams_ethereum::Event for TransferFromParent1 {
            const NAME: &'static str = "TransferFromParent1";
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
        pub struct TransferFromParent2 {
            pub from_contract: Vec<u8>,
            pub from_token_id: ethabi::Uint,
            pub token_type_id: ethabi::Uint,
            pub value: ethabi::Uint,
        }
        impl TransferFromParent2 {
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
        impl substreams_ethereum::Event for TransferFromParent2 {
            const NAME: &'static str = "TransferFromParent2";
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
        pub struct TransferFromParent3 {
            pub from_contract: Vec<u8>,
            pub from_token_id: ethabi::Uint,
            pub token_type_id: ethabi::Uint,
            pub value: ethabi::Uint,
        }
        impl TransferFromParent3 {
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
        impl substreams_ethereum::Event for TransferFromParent3 {
            const NAME: &'static str = "TransferFromParent3";
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
        pub struct TransferFromParent4 {
            pub from_contract: Vec<u8>,
            pub from_token_id: ethabi::Uint,
            pub token_type_id: ethabi::Uint,
            pub value: ethabi::Uint,
        }
        impl TransferFromParent4 {
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
        impl substreams_ethereum::Event for TransferFromParent4 {
            const NAME: &'static str = "TransferFromParent4";
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
        pub struct TransferSingle1 {
            pub operator: Vec<u8>,
            pub from: Vec<u8>,
            pub to: Vec<u8>,
            pub id: ethabi::Uint,
            pub value: ethabi::Uint,
        }
        impl TransferSingle1 {
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
        impl substreams_ethereum::Event for TransferSingle1 {
            const NAME: &'static str = "TransferSingle1";
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
        pub struct TransferSingle2 {
            pub operator: Vec<u8>,
            pub from: Vec<u8>,
            pub to: Vec<u8>,
            pub id: ethabi::Uint,
            pub value: ethabi::Uint,
        }
        impl TransferSingle2 {
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
        impl substreams_ethereum::Event for TransferSingle2 {
            const NAME: &'static str = "TransferSingle2";
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
        pub struct TransferSingle3 {
            pub operator: Vec<u8>,
            pub from: Vec<u8>,
            pub to: Vec<u8>,
            pub id: ethabi::Uint,
            pub value: ethabi::Uint,
        }
        impl TransferSingle3 {
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
        impl substreams_ethereum::Event for TransferSingle3 {
            const NAME: &'static str = "TransferSingle3";
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
        pub struct TransferSingle4 {
            pub operator: Vec<u8>,
            pub from: Vec<u8>,
            pub to: Vec<u8>,
            pub id: ethabi::Uint,
            pub value: ethabi::Uint,
        }
        impl TransferSingle4 {
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
        impl substreams_ethereum::Event for TransferSingle4 {
            const NAME: &'static str = "TransferSingle4";
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
        pub struct TransferToParent3 {
            pub to_contract: Vec<u8>,
            pub to_token_id: ethabi::Uint,
            pub token_type_id: ethabi::Uint,
            pub value: ethabi::Uint,
        }
        impl TransferToParent3 {
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
        impl substreams_ethereum::Event for TransferToParent3 {
            const NAME: &'static str = "TransferToParent3";
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
        pub struct TransferToParent4 {
            pub to_contract: Vec<u8>,
            pub to_token_id: ethabi::Uint,
            pub token_type_id: ethabi::Uint,
            pub value: ethabi::Uint,
        }
        impl TransferToParent4 {
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
        impl substreams_ethereum::Event for TransferToParent4 {
            const NAME: &'static str = "TransferToParent4";
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
        pub struct Uri1 {
            pub value: String,
            pub token_id: ethabi::Uint,
        }
        impl Uri1 {
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
                    value: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_string()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for Uri1 {
            const NAME: &'static str = "URI1";
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
        pub struct Uri2 {
            pub value: String,
            pub token_id: ethabi::Uint,
        }
        impl Uri2 {
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
                    value: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_string()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for Uri2 {
            const NAME: &'static str = "URI2";
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
        pub struct Uri3 {
            pub value: String,
            pub token_id: ethabi::Uint,
        }
        impl Uri3 {
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
                    value: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_string()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for Uri3 {
            const NAME: &'static str = "URI3";
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
        pub struct Uri4 {
            pub value: String,
            pub token_id: ethabi::Uint,
        }
        impl Uri4 {
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
                    value: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_string()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for Uri4 {
            const NAME: &'static str = "URI4";
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
        pub struct UnequipInstallation {
            pub realm_id: ethabi::Uint,
            pub installation_id: ethabi::Uint,
            pub x: ethabi::Uint,
            pub y: ethabi::Uint,
        }
        impl UnequipInstallation {
            const TOPIC_ID: [u8; 32] = [
                235u8,
                141u8,
                195u8,
                117u8,
                52u8,
                183u8,
                45u8,
                229u8,
                33u8,
                209u8,
                104u8,
                132u8,
                195u8,
                33u8,
                72u8,
                224u8,
                90u8,
                190u8,
                85u8,
                12u8,
                86u8,
                39u8,
                238u8,
                75u8,
                165u8,
                39u8,
                221u8,
                250u8,
                26u8,
                119u8,
                23u8,
                255u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 1usize {
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
                    realm_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    installation_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    x: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    y: values.pop().expect(INTERNAL_ERR).into_uint().expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for UnequipInstallation {
            const NAME: &'static str = "UnequipInstallation";
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
        pub struct UnequipTile {
            pub realm_id: ethabi::Uint,
            pub tile_id: ethabi::Uint,
            pub x: ethabi::Uint,
            pub y: ethabi::Uint,
        }
        impl UnequipTile {
            const TOPIC_ID: [u8; 32] = [
                177u8,
                233u8,
                67u8,
                194u8,
                128u8,
                208u8,
                85u8,
                191u8,
                225u8,
                104u8,
                107u8,
                161u8,
                25u8,
                89u8,
                156u8,
                210u8,
                7u8,
                31u8,
                24u8,
                225u8,
                243u8,
                188u8,
                225u8,
                42u8,
                211u8,
                195u8,
                196u8,
                240u8,
                213u8,
                180u8,
                84u8,
                31u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 1usize {
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
                    realm_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    tile_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    x: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    y: values.pop().expect(INTERNAL_ERR).into_uint().expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for UnequipTile {
            const NAME: &'static str = "UnequipTile";
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
        pub struct UpgradeFinalized1 {
            pub realm_id: ethabi::Uint,
            pub coordinate_x: ethabi::Uint,
            pub coordinate_y: ethabi::Uint,
            pub new_installation_id: ethabi::Uint,
        }
        impl UpgradeFinalized1 {
            const TOPIC_ID: [u8; 32] = [
                226u8,
                228u8,
                179u8,
                188u8,
                186u8,
                27u8,
                141u8,
                34u8,
                122u8,
                224u8,
                10u8,
                17u8,
                68u8,
                104u8,
                21u8,
                58u8,
                96u8,
                250u8,
                209u8,
                84u8,
                48u8,
                97u8,
                84u8,
                178u8,
                255u8,
                30u8,
                103u8,
                50u8,
                92u8,
                196u8,
                131u8,
                21u8,
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
                    realm_id: ethabi::decode(
                            &[ethabi::ParamType::Uint(256usize)],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'realm_id' from topic of type 'uint256': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    coordinate_x: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    coordinate_y: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    new_installation_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for UpgradeFinalized1 {
            const NAME: &'static str = "UpgradeFinalized1";
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
        pub struct UpgradeFinalized2 {
            pub realm_id: ethabi::Uint,
            pub coordinate_x: ethabi::Uint,
            pub coordinate_y: ethabi::Uint,
            pub new_installation_id: ethabi::Uint,
        }
        impl UpgradeFinalized2 {
            const TOPIC_ID: [u8; 32] = [
                226u8,
                228u8,
                179u8,
                188u8,
                186u8,
                27u8,
                141u8,
                34u8,
                122u8,
                224u8,
                10u8,
                17u8,
                68u8,
                104u8,
                21u8,
                58u8,
                96u8,
                250u8,
                209u8,
                84u8,
                48u8,
                97u8,
                84u8,
                178u8,
                255u8,
                30u8,
                103u8,
                50u8,
                92u8,
                196u8,
                131u8,
                21u8,
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
                    realm_id: ethabi::decode(
                            &[ethabi::ParamType::Uint(256usize)],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'realm_id' from topic of type 'uint256': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    coordinate_x: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    coordinate_y: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    new_installation_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for UpgradeFinalized2 {
            const NAME: &'static str = "UpgradeFinalized2";
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
        pub struct UpgradeInitiated1 {
            pub realm_id: ethabi::Uint,
            pub coordinate_x: ethabi::Uint,
            pub coordinate_y: ethabi::Uint,
            pub block_initiated: ethabi::Uint,
            pub ready_block: ethabi::Uint,
            pub installation_id: ethabi::Uint,
        }
        impl UpgradeInitiated1 {
            const TOPIC_ID: [u8; 32] = [
                255u8,
                151u8,
                122u8,
                72u8,
                230u8,
                237u8,
                6u8,
                14u8,
                19u8,
                35u8,
                243u8,
                49u8,
                181u8,
                141u8,
                117u8,
                254u8,
                209u8,
                234u8,
                219u8,
                77u8,
                172u8,
                62u8,
                59u8,
                219u8,
                134u8,
                102u8,
                78u8,
                166u8,
                169u8,
                244u8,
                53u8,
                78u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 2usize {
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
                            ethabi::ParamType::Uint(256usize),
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
                    realm_id: ethabi::decode(
                            &[ethabi::ParamType::Uint(256usize)],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'realm_id' from topic of type 'uint256': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    coordinate_x: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    coordinate_y: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    block_initiated: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    ready_block: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    installation_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for UpgradeInitiated1 {
            const NAME: &'static str = "UpgradeInitiated1";
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
        pub struct UpgradeInitiated2 {
            pub realm_id: ethabi::Uint,
            pub coordinate_x: ethabi::Uint,
            pub coordinate_y: ethabi::Uint,
            pub block_initiated: ethabi::Uint,
            pub ready_block: ethabi::Uint,
            pub installation_id: ethabi::Uint,
        }
        impl UpgradeInitiated2 {
            const TOPIC_ID: [u8; 32] = [
                255u8,
                151u8,
                122u8,
                72u8,
                230u8,
                237u8,
                6u8,
                14u8,
                19u8,
                35u8,
                243u8,
                49u8,
                181u8,
                141u8,
                117u8,
                254u8,
                209u8,
                234u8,
                219u8,
                77u8,
                172u8,
                62u8,
                59u8,
                219u8,
                134u8,
                102u8,
                78u8,
                166u8,
                169u8,
                244u8,
                53u8,
                78u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 2usize {
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
                            ethabi::ParamType::Uint(256usize),
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
                    realm_id: ethabi::decode(
                            &[ethabi::ParamType::Uint(256usize)],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'realm_id' from topic of type 'uint256': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    coordinate_x: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    coordinate_y: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    block_initiated: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    ready_block: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    installation_id: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for UpgradeInitiated2 {
            const NAME: &'static str = "UpgradeInitiated2";
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
        pub struct UpgradeQueueFinalized1 {
            pub owner: Vec<u8>,
            pub realm_id: ethabi::Uint,
            pub queue_index: ethabi::Uint,
        }
        impl UpgradeQueueFinalized1 {
            const TOPIC_ID: [u8; 32] = [
                182u8,
                138u8,
                183u8,
                191u8,
                154u8,
                98u8,
                236u8,
                166u8,
                50u8,
                228u8,
                175u8,
                82u8,
                226u8,
                171u8,
                113u8,
                219u8,
                50u8,
                124u8,
                154u8,
                187u8,
                107u8,
                92u8,
                238u8,
                145u8,
                238u8,
                35u8,
                175u8,
                179u8,
                44u8,
                244u8,
                14u8,
                39u8,
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
                    realm_id: ethabi::decode(
                            &[ethabi::ParamType::Uint(256usize)],
                            log.topics[2usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'realm_id' from topic of type 'uint256': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    queue_index: ethabi::decode(
                            &[ethabi::ParamType::Uint(256usize)],
                            log.topics[3usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'queue_index' from topic of type 'uint256': {:?}",
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
        impl substreams_ethereum::Event for UpgradeQueueFinalized1 {
            const NAME: &'static str = "UpgradeQueueFinalized1";
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
        pub struct UpgradeQueueFinalized2 {
            pub owner: Vec<u8>,
            pub realm_id: ethabi::Uint,
            pub queue_index: ethabi::Uint,
        }
        impl UpgradeQueueFinalized2 {
            const TOPIC_ID: [u8; 32] = [
                182u8,
                138u8,
                183u8,
                191u8,
                154u8,
                98u8,
                236u8,
                166u8,
                50u8,
                228u8,
                175u8,
                82u8,
                226u8,
                171u8,
                113u8,
                219u8,
                50u8,
                124u8,
                154u8,
                187u8,
                107u8,
                92u8,
                238u8,
                145u8,
                238u8,
                35u8,
                175u8,
                179u8,
                44u8,
                244u8,
                14u8,
                39u8,
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
                    realm_id: ethabi::decode(
                            &[ethabi::ParamType::Uint(256usize)],
                            log.topics[2usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'realm_id' from topic of type 'uint256': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    queue_index: ethabi::decode(
                            &[ethabi::ParamType::Uint(256usize)],
                            log.topics[3usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'queue_index' from topic of type 'uint256': {:?}",
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
        impl substreams_ethereum::Event for UpgradeQueueFinalized2 {
            const NAME: &'static str = "UpgradeQueueFinalized2";
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
        pub struct UpgradeQueued1 {
            pub owner: Vec<u8>,
            pub realm_id: ethabi::Uint,
            pub queue_index: ethabi::Uint,
        }
        impl UpgradeQueued1 {
            const TOPIC_ID: [u8; 32] = [
                241u8,
                231u8,
                171u8,
                111u8,
                64u8,
                58u8,
                176u8,
                39u8,
                50u8,
                155u8,
                229u8,
                92u8,
                26u8,
                168u8,
                44u8,
                140u8,
                146u8,
                197u8,
                19u8,
                170u8,
                226u8,
                162u8,
                127u8,
                192u8,
                187u8,
                170u8,
                160u8,
                59u8,
                13u8,
                216u8,
                241u8,
                42u8,
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
                    realm_id: ethabi::decode(
                            &[ethabi::ParamType::Uint(256usize)],
                            log.topics[2usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'realm_id' from topic of type 'uint256': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    queue_index: ethabi::decode(
                            &[ethabi::ParamType::Uint(256usize)],
                            log.topics[3usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'queue_index' from topic of type 'uint256': {:?}",
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
        impl substreams_ethereum::Event for UpgradeQueued1 {
            const NAME: &'static str = "UpgradeQueued1";
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
        pub struct UpgradeQueued2 {
            pub owner: Vec<u8>,
            pub realm_id: ethabi::Uint,
            pub queue_index: ethabi::Uint,
        }
        impl UpgradeQueued2 {
            const TOPIC_ID: [u8; 32] = [
                241u8,
                231u8,
                171u8,
                111u8,
                64u8,
                58u8,
                176u8,
                39u8,
                50u8,
                155u8,
                229u8,
                92u8,
                26u8,
                168u8,
                44u8,
                140u8,
                146u8,
                197u8,
                19u8,
                170u8,
                226u8,
                162u8,
                127u8,
                192u8,
                187u8,
                170u8,
                160u8,
                59u8,
                13u8,
                216u8,
                241u8,
                42u8,
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
                    realm_id: ethabi::decode(
                            &[ethabi::ParamType::Uint(256usize)],
                            log.topics[2usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'realm_id' from topic of type 'uint256': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    queue_index: ethabi::decode(
                            &[ethabi::ParamType::Uint(256usize)],
                            log.topics[3usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'queue_index' from topic of type 'uint256': {:?}",
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
        impl substreams_ethereum::Event for UpgradeQueued2 {
            const NAME: &'static str = "UpgradeQueued2";
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