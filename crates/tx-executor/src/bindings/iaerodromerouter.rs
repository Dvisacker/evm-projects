///Module containing a contract's types and functions.
/**

```solidity
library IRouter {
    struct Route { address from; address to; bool stable; address factory; }
    struct Zap { address tokenA; address tokenB; bool stable; address factory; uint256 amountOutMinA; uint256 amountOutMinB; uint256 amountAMin; uint256 amountBMin; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod IRouter {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
struct Route { address from; address to; bool stable; address factory; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Route {
        #[allow(missing_docs)]
        pub from: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub to: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub stable: bool,
        #[allow(missing_docs)]
        pub factory: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Bool,
            alloy::sol_types::sol_data::Address,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Address,
            bool,
            alloy::sol_types::private::Address,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<Route> for UnderlyingRustTuple<'_> {
            fn from(value: Route) -> Self {
                (value.from, value.to, value.stable, value.factory)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Route {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    from: tuple.0,
                    to: tuple.1,
                    stable: tuple.2,
                    factory: tuple.3,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for Route {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for Route {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.from,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.to,
                    ),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.stable,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.factory,
                    ),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for Route {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for Route {
            const NAME: &'static str = "Route";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "Route(address from,address to,bool stable,address factory)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.from,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.to,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::eip712_data_word(
                            &self.stable,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.factory,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Route {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.from,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.to,
                    )
                    + <alloy::sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.stable,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.factory,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.from,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.to,
                    out,
                );
                <alloy::sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.stable,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.factory,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    /**```solidity
struct Zap { address tokenA; address tokenB; bool stable; address factory; uint256 amountOutMinA; uint256 amountOutMinB; uint256 amountAMin; uint256 amountBMin; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Zap {
        #[allow(missing_docs)]
        pub tokenA: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub tokenB: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub stable: bool,
        #[allow(missing_docs)]
        pub factory: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub amountOutMinA: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub amountOutMinB: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub amountAMin: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub amountBMin: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Bool,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Address,
            bool,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<Zap> for UnderlyingRustTuple<'_> {
            fn from(value: Zap) -> Self {
                (
                    value.tokenA,
                    value.tokenB,
                    value.stable,
                    value.factory,
                    value.amountOutMinA,
                    value.amountOutMinB,
                    value.amountAMin,
                    value.amountBMin,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Zap {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    tokenA: tuple.0,
                    tokenB: tuple.1,
                    stable: tuple.2,
                    factory: tuple.3,
                    amountOutMinA: tuple.4,
                    amountOutMinB: tuple.5,
                    amountAMin: tuple.6,
                    amountBMin: tuple.7,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for Zap {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for Zap {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.tokenA,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.tokenB,
                    ),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.stable,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.factory,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amountOutMinA),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amountOutMinB),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amountAMin),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amountBMin),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for Zap {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for Zap {
            const NAME: &'static str = "Zap";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "Zap(address tokenA,address tokenB,bool stable,address factory,uint256 amountOutMinA,uint256 amountOutMinB,uint256 amountAMin,uint256 amountBMin)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.tokenA,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.tokenB,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::eip712_data_word(
                            &self.stable,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.factory,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.amountOutMinA)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.amountOutMinB)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.amountAMin)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.amountBMin)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Zap {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.tokenA,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.tokenB,
                    )
                    + <alloy::sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.stable,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.factory,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.amountOutMinA,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.amountOutMinB,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.amountAMin,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.amountBMin,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.tokenA,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.tokenB,
                    out,
                );
                <alloy::sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.stable,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.factory,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.amountOutMinA,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.amountOutMinB,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.amountAMin,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.amountBMin,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`IRouter`](self) contract instance.

See the [wrapper's documentation](`IRouterInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IRouterInstance<T, P, N> {
        IRouterInstance::<T, P, N>::new(address, provider)
    }
    /**A [`IRouter`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`IRouter`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IRouterInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for IRouterInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IRouterInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IRouterInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`IRouter`](self) contract instance.

See the [wrapper's documentation](`IRouterInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            provider: P,
        ) -> Self {
            Self {
                address,
                provider,
                _network_transport: ::core::marker::PhantomData,
            }
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<T, P: ::core::clone::Clone, N> IRouterInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IRouterInstance<T, P, N> {
            IRouterInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network_transport: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IRouterInstance<T, P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<T, &P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IRouterInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
/**

Generated by the following Solidity interface...
```solidity
library IRouter {
    struct Route {
        address from;
        address to;
        bool stable;
        address factory;
    }
    struct Zap {
        address tokenA;
        address tokenB;
        bool stable;
        address factory;
        uint256 amountOutMinA;
        uint256 amountOutMinB;
        uint256 amountAMin;
        uint256 amountBMin;
    }
}

interface IAerodromeRouter {
    error ETHTransferFailed();
    error Expired();
    error InsufficientAmount();
    error InsufficientAmountA();
    error InsufficientAmountADesired();
    error InsufficientAmountAOptimal();
    error InsufficientAmountB();
    error InsufficientAmountBDesired();
    error InsufficientLiquidity();
    error InsufficientOutputAmount();
    error InvalidAmountInForETHDeposit();
    error InvalidPath();
    error InvalidRouteA();
    error InvalidRouteB();
    error InvalidTokenInForETHDeposit();
    error OnlyWETH();
    error PoolDoesNotExist();
    error PoolFactoryDoesNotExist();
    error SameAddresses();
    error ZeroAddress();

    receive() external payable;

    function ETHER() external view returns (address);
    function UNSAFE_swapExactTokensForTokens(uint256[] memory amounts, IRouter.Route[] memory routes, address to, uint256 deadline) external returns (uint256[] memory);
    function addLiquidity(address tokenA, address tokenB, bool stable, uint256 amountADesired, uint256 amountBDesired, uint256 amountAMin, uint256 amountBMin, address to, uint256 deadline) external returns (uint256 amountA, uint256 amountB, uint256 liquidity);
    function addLiquidityETH(address token, bool stable, uint256 amountTokenDesired, uint256 amountTokenMin, uint256 amountETHMin, address to, uint256 deadline) external payable returns (uint256 amountToken, uint256 amountETH, uint256 liquidity);
    function defaultFactory() external view returns (address);
    function factoryRegistry() external view returns (address);
    function generateZapInParams(address tokenA, address tokenB, bool stable, address _factory, uint256 amountInA, uint256 amountInB, IRouter.Route[] memory routesA, IRouter.Route[] memory routesB) external view returns (uint256 amountOutMinA, uint256 amountOutMinB, uint256 amountAMin, uint256 amountBMin);
    function generateZapOutParams(address tokenA, address tokenB, bool stable, address _factory, uint256 liquidity, IRouter.Route[] memory routesA, IRouter.Route[] memory routesB) external view returns (uint256 amountOutMinA, uint256 amountOutMinB, uint256 amountAMin, uint256 amountBMin);
    function getAmountsOut(uint256 amountIn, IRouter.Route[] memory routes) external view returns (uint256[] memory amounts);
    function getReserves(address tokenA, address tokenB, bool stable, address _factory) external view returns (uint256 reserveA, uint256 reserveB);
    function isTrustedForwarder(address forwarder) external view returns (bool);
    function poolFor(address tokenA, address tokenB, bool stable, address _factory) external view returns (address pool);
    function quoteAddLiquidity(address tokenA, address tokenB, bool stable, address _factory, uint256 amountADesired, uint256 amountBDesired) external view returns (uint256 amountA, uint256 amountB, uint256 liquidity);
    function quoteRemoveLiquidity(address tokenA, address tokenB, bool stable, address _factory, uint256 liquidity) external view returns (uint256 amountA, uint256 amountB);
    function quoteStableLiquidityRatio(address tokenA, address tokenB, address _factory) external view returns (uint256 ratio);
    function removeLiquidity(address tokenA, address tokenB, bool stable, uint256 liquidity, uint256 amountAMin, uint256 amountBMin, address to, uint256 deadline) external returns (uint256 amountA, uint256 amountB);
    function removeLiquidityETH(address token, bool stable, uint256 liquidity, uint256 amountTokenMin, uint256 amountETHMin, address to, uint256 deadline) external returns (uint256 amountToken, uint256 amountETH);
    function removeLiquidityETHSupportingFeeOnTransferTokens(address token, bool stable, uint256 liquidity, uint256 amountTokenMin, uint256 amountETHMin, address to, uint256 deadline) external returns (uint256 amountETH);
    function sortTokens(address tokenA, address tokenB) external pure returns (address token0, address token1);
    function swapExactETHForTokens(uint256 amountOutMin, IRouter.Route[] memory routes, address to, uint256 deadline) external payable returns (uint256[] memory amounts);
    function swapExactETHForTokensSupportingFeeOnTransferTokens(uint256 amountOutMin, IRouter.Route[] memory routes, address to, uint256 deadline) external payable;
    function swapExactTokensForETH(uint256 amountIn, uint256 amountOutMin, IRouter.Route[] memory routes, address to, uint256 deadline) external returns (uint256[] memory amounts);
    function swapExactTokensForETHSupportingFeeOnTransferTokens(uint256 amountIn, uint256 amountOutMin, IRouter.Route[] memory routes, address to, uint256 deadline) external;
    function swapExactTokensForTokens(uint256 amountIn, uint256 amountOutMin, IRouter.Route[] memory routes, address to, uint256 deadline) external returns (uint256[] memory amounts);
    function swapExactTokensForTokensSupportingFeeOnTransferTokens(uint256 amountIn, uint256 amountOutMin, IRouter.Route[] memory routes, address to, uint256 deadline) external;
    function voter() external view returns (address);
    function weth() external view returns (address);
    function zapIn(address tokenIn, uint256 amountInA, uint256 amountInB, IRouter.Zap memory zapInPool, IRouter.Route[] memory routesA, IRouter.Route[] memory routesB, address to, bool stake) external payable returns (uint256 liquidity);
    function zapOut(address tokenOut, uint256 liquidity, IRouter.Zap memory zapOutPool, IRouter.Route[] memory routesA, IRouter.Route[] memory routesB) external;
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "receive",
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "ETHER",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "UNSAFE_swapExactTokensForTokens",
    "inputs": [
      {
        "name": "amounts",
        "type": "uint256[]",
        "internalType": "uint256[]"
      },
      {
        "name": "routes",
        "type": "tuple[]",
        "internalType": "struct IRouter.Route[]",
        "components": [
          {
            "name": "from",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "to",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "stable",
            "type": "bool",
            "internalType": "bool"
          },
          {
            "name": "factory",
            "type": "address",
            "internalType": "address"
          }
        ]
      },
      {
        "name": "to",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "deadline",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256[]",
        "internalType": "uint256[]"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "addLiquidity",
    "inputs": [
      {
        "name": "tokenA",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "tokenB",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "stable",
        "type": "bool",
        "internalType": "bool"
      },
      {
        "name": "amountADesired",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "amountBDesired",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "amountAMin",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "amountBMin",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "to",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "deadline",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "amountA",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "amountB",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "liquidity",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "addLiquidityETH",
    "inputs": [
      {
        "name": "token",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "stable",
        "type": "bool",
        "internalType": "bool"
      },
      {
        "name": "amountTokenDesired",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "amountTokenMin",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "amountETHMin",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "to",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "deadline",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "amountToken",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "amountETH",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "liquidity",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "defaultFactory",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "factoryRegistry",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "generateZapInParams",
    "inputs": [
      {
        "name": "tokenA",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "tokenB",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "stable",
        "type": "bool",
        "internalType": "bool"
      },
      {
        "name": "_factory",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "amountInA",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "amountInB",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "routesA",
        "type": "tuple[]",
        "internalType": "struct IRouter.Route[]",
        "components": [
          {
            "name": "from",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "to",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "stable",
            "type": "bool",
            "internalType": "bool"
          },
          {
            "name": "factory",
            "type": "address",
            "internalType": "address"
          }
        ]
      },
      {
        "name": "routesB",
        "type": "tuple[]",
        "internalType": "struct IRouter.Route[]",
        "components": [
          {
            "name": "from",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "to",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "stable",
            "type": "bool",
            "internalType": "bool"
          },
          {
            "name": "factory",
            "type": "address",
            "internalType": "address"
          }
        ]
      }
    ],
    "outputs": [
      {
        "name": "amountOutMinA",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "amountOutMinB",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "amountAMin",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "amountBMin",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "generateZapOutParams",
    "inputs": [
      {
        "name": "tokenA",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "tokenB",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "stable",
        "type": "bool",
        "internalType": "bool"
      },
      {
        "name": "_factory",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "liquidity",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "routesA",
        "type": "tuple[]",
        "internalType": "struct IRouter.Route[]",
        "components": [
          {
            "name": "from",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "to",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "stable",
            "type": "bool",
            "internalType": "bool"
          },
          {
            "name": "factory",
            "type": "address",
            "internalType": "address"
          }
        ]
      },
      {
        "name": "routesB",
        "type": "tuple[]",
        "internalType": "struct IRouter.Route[]",
        "components": [
          {
            "name": "from",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "to",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "stable",
            "type": "bool",
            "internalType": "bool"
          },
          {
            "name": "factory",
            "type": "address",
            "internalType": "address"
          }
        ]
      }
    ],
    "outputs": [
      {
        "name": "amountOutMinA",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "amountOutMinB",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "amountAMin",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "amountBMin",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getAmountsOut",
    "inputs": [
      {
        "name": "amountIn",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "routes",
        "type": "tuple[]",
        "internalType": "struct IRouter.Route[]",
        "components": [
          {
            "name": "from",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "to",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "stable",
            "type": "bool",
            "internalType": "bool"
          },
          {
            "name": "factory",
            "type": "address",
            "internalType": "address"
          }
        ]
      }
    ],
    "outputs": [
      {
        "name": "amounts",
        "type": "uint256[]",
        "internalType": "uint256[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getReserves",
    "inputs": [
      {
        "name": "tokenA",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "tokenB",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "stable",
        "type": "bool",
        "internalType": "bool"
      },
      {
        "name": "_factory",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "reserveA",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "reserveB",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "isTrustedForwarder",
    "inputs": [
      {
        "name": "forwarder",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "poolFor",
    "inputs": [
      {
        "name": "tokenA",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "tokenB",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "stable",
        "type": "bool",
        "internalType": "bool"
      },
      {
        "name": "_factory",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "pool",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "quoteAddLiquidity",
    "inputs": [
      {
        "name": "tokenA",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "tokenB",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "stable",
        "type": "bool",
        "internalType": "bool"
      },
      {
        "name": "_factory",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "amountADesired",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "amountBDesired",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "amountA",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "amountB",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "liquidity",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "quoteRemoveLiquidity",
    "inputs": [
      {
        "name": "tokenA",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "tokenB",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "stable",
        "type": "bool",
        "internalType": "bool"
      },
      {
        "name": "_factory",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "liquidity",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "amountA",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "amountB",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "quoteStableLiquidityRatio",
    "inputs": [
      {
        "name": "tokenA",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "tokenB",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "_factory",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "ratio",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "removeLiquidity",
    "inputs": [
      {
        "name": "tokenA",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "tokenB",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "stable",
        "type": "bool",
        "internalType": "bool"
      },
      {
        "name": "liquidity",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "amountAMin",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "amountBMin",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "to",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "deadline",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "amountA",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "amountB",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "removeLiquidityETH",
    "inputs": [
      {
        "name": "token",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "stable",
        "type": "bool",
        "internalType": "bool"
      },
      {
        "name": "liquidity",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "amountTokenMin",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "amountETHMin",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "to",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "deadline",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "amountToken",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "amountETH",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "removeLiquidityETHSupportingFeeOnTransferTokens",
    "inputs": [
      {
        "name": "token",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "stable",
        "type": "bool",
        "internalType": "bool"
      },
      {
        "name": "liquidity",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "amountTokenMin",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "amountETHMin",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "to",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "deadline",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "amountETH",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "sortTokens",
    "inputs": [
      {
        "name": "tokenA",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "tokenB",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "token0",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "token1",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "pure"
  },
  {
    "type": "function",
    "name": "swapExactETHForTokens",
    "inputs": [
      {
        "name": "amountOutMin",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "routes",
        "type": "tuple[]",
        "internalType": "struct IRouter.Route[]",
        "components": [
          {
            "name": "from",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "to",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "stable",
            "type": "bool",
            "internalType": "bool"
          },
          {
            "name": "factory",
            "type": "address",
            "internalType": "address"
          }
        ]
      },
      {
        "name": "to",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "deadline",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "amounts",
        "type": "uint256[]",
        "internalType": "uint256[]"
      }
    ],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "swapExactETHForTokensSupportingFeeOnTransferTokens",
    "inputs": [
      {
        "name": "amountOutMin",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "routes",
        "type": "tuple[]",
        "internalType": "struct IRouter.Route[]",
        "components": [
          {
            "name": "from",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "to",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "stable",
            "type": "bool",
            "internalType": "bool"
          },
          {
            "name": "factory",
            "type": "address",
            "internalType": "address"
          }
        ]
      },
      {
        "name": "to",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "deadline",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "swapExactTokensForETH",
    "inputs": [
      {
        "name": "amountIn",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "amountOutMin",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "routes",
        "type": "tuple[]",
        "internalType": "struct IRouter.Route[]",
        "components": [
          {
            "name": "from",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "to",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "stable",
            "type": "bool",
            "internalType": "bool"
          },
          {
            "name": "factory",
            "type": "address",
            "internalType": "address"
          }
        ]
      },
      {
        "name": "to",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "deadline",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "amounts",
        "type": "uint256[]",
        "internalType": "uint256[]"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "swapExactTokensForETHSupportingFeeOnTransferTokens",
    "inputs": [
      {
        "name": "amountIn",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "amountOutMin",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "routes",
        "type": "tuple[]",
        "internalType": "struct IRouter.Route[]",
        "components": [
          {
            "name": "from",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "to",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "stable",
            "type": "bool",
            "internalType": "bool"
          },
          {
            "name": "factory",
            "type": "address",
            "internalType": "address"
          }
        ]
      },
      {
        "name": "to",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "deadline",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "swapExactTokensForTokens",
    "inputs": [
      {
        "name": "amountIn",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "amountOutMin",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "routes",
        "type": "tuple[]",
        "internalType": "struct IRouter.Route[]",
        "components": [
          {
            "name": "from",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "to",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "stable",
            "type": "bool",
            "internalType": "bool"
          },
          {
            "name": "factory",
            "type": "address",
            "internalType": "address"
          }
        ]
      },
      {
        "name": "to",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "deadline",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "amounts",
        "type": "uint256[]",
        "internalType": "uint256[]"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "swapExactTokensForTokensSupportingFeeOnTransferTokens",
    "inputs": [
      {
        "name": "amountIn",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "amountOutMin",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "routes",
        "type": "tuple[]",
        "internalType": "struct IRouter.Route[]",
        "components": [
          {
            "name": "from",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "to",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "stable",
            "type": "bool",
            "internalType": "bool"
          },
          {
            "name": "factory",
            "type": "address",
            "internalType": "address"
          }
        ]
      },
      {
        "name": "to",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "deadline",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "voter",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "weth",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "zapIn",
    "inputs": [
      {
        "name": "tokenIn",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "amountInA",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "amountInB",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "zapInPool",
        "type": "tuple",
        "internalType": "struct IRouter.Zap",
        "components": [
          {
            "name": "tokenA",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "tokenB",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "stable",
            "type": "bool",
            "internalType": "bool"
          },
          {
            "name": "factory",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "amountOutMinA",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "amountOutMinB",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "amountAMin",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "amountBMin",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      },
      {
        "name": "routesA",
        "type": "tuple[]",
        "internalType": "struct IRouter.Route[]",
        "components": [
          {
            "name": "from",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "to",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "stable",
            "type": "bool",
            "internalType": "bool"
          },
          {
            "name": "factory",
            "type": "address",
            "internalType": "address"
          }
        ]
      },
      {
        "name": "routesB",
        "type": "tuple[]",
        "internalType": "struct IRouter.Route[]",
        "components": [
          {
            "name": "from",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "to",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "stable",
            "type": "bool",
            "internalType": "bool"
          },
          {
            "name": "factory",
            "type": "address",
            "internalType": "address"
          }
        ]
      },
      {
        "name": "to",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "stake",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "outputs": [
      {
        "name": "liquidity",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "zapOut",
    "inputs": [
      {
        "name": "tokenOut",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "liquidity",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "zapOutPool",
        "type": "tuple",
        "internalType": "struct IRouter.Zap",
        "components": [
          {
            "name": "tokenA",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "tokenB",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "stable",
            "type": "bool",
            "internalType": "bool"
          },
          {
            "name": "factory",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "amountOutMinA",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "amountOutMinB",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "amountAMin",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "amountBMin",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      },
      {
        "name": "routesA",
        "type": "tuple[]",
        "internalType": "struct IRouter.Route[]",
        "components": [
          {
            "name": "from",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "to",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "stable",
            "type": "bool",
            "internalType": "bool"
          },
          {
            "name": "factory",
            "type": "address",
            "internalType": "address"
          }
        ]
      },
      {
        "name": "routesB",
        "type": "tuple[]",
        "internalType": "struct IRouter.Route[]",
        "components": [
          {
            "name": "from",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "to",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "stable",
            "type": "bool",
            "internalType": "bool"
          },
          {
            "name": "factory",
            "type": "address",
            "internalType": "address"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "error",
    "name": "ETHTransferFailed",
    "inputs": []
  },
  {
    "type": "error",
    "name": "Expired",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InsufficientAmount",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InsufficientAmountA",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InsufficientAmountADesired",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InsufficientAmountAOptimal",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InsufficientAmountB",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InsufficientAmountBDesired",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InsufficientLiquidity",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InsufficientOutputAmount",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidAmountInForETHDeposit",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidPath",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidRouteA",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidRouteB",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidTokenInForETHDeposit",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OnlyWETH",
    "inputs": []
  },
  {
    "type": "error",
    "name": "PoolDoesNotExist",
    "inputs": []
  },
  {
    "type": "error",
    "name": "PoolFactoryDoesNotExist",
    "inputs": []
  },
  {
    "type": "error",
    "name": "SameAddresses",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ZeroAddress",
    "inputs": []
  }
]
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod IAerodromeRouter {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"",
    );
    /**Custom error with signature `ETHTransferFailed()` and selector `0xb12d13eb`.
```solidity
error ETHTransferFailed();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ETHTransferFailed {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ETHTransferFailed> for UnderlyingRustTuple<'_> {
            fn from(value: ETHTransferFailed) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ETHTransferFailed {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ETHTransferFailed {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ETHTransferFailed()";
            const SELECTOR: [u8; 4] = [177u8, 45u8, 19u8, 235u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `Expired()` and selector `0x203d82d8`.
```solidity
error Expired();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Expired {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<Expired> for UnderlyingRustTuple<'_> {
            fn from(value: Expired) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Expired {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for Expired {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "Expired()";
            const SELECTOR: [u8; 4] = [32u8, 61u8, 130u8, 216u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `InsufficientAmount()` and selector `0x5945ea56`.
```solidity
error InsufficientAmount();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InsufficientAmount {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InsufficientAmount> for UnderlyingRustTuple<'_> {
            fn from(value: InsufficientAmount) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InsufficientAmount {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InsufficientAmount {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InsufficientAmount()";
            const SELECTOR: [u8; 4] = [89u8, 69u8, 234u8, 86u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `InsufficientAmountA()` and selector `0x8f66ec14`.
```solidity
error InsufficientAmountA();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InsufficientAmountA {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InsufficientAmountA> for UnderlyingRustTuple<'_> {
            fn from(value: InsufficientAmountA) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InsufficientAmountA {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InsufficientAmountA {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InsufficientAmountA()";
            const SELECTOR: [u8; 4] = [143u8, 102u8, 236u8, 20u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `InsufficientAmountADesired()` and selector `0xdc6b2ef2`.
```solidity
error InsufficientAmountADesired();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InsufficientAmountADesired {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InsufficientAmountADesired>
        for UnderlyingRustTuple<'_> {
            fn from(value: InsufficientAmountADesired) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for InsufficientAmountADesired {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InsufficientAmountADesired {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InsufficientAmountADesired()";
            const SELECTOR: [u8; 4] = [220u8, 107u8, 46u8, 242u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `InsufficientAmountAOptimal()` and selector `0xfe496df6`.
```solidity
error InsufficientAmountAOptimal();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InsufficientAmountAOptimal {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InsufficientAmountAOptimal>
        for UnderlyingRustTuple<'_> {
            fn from(value: InsufficientAmountAOptimal) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for InsufficientAmountAOptimal {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InsufficientAmountAOptimal {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InsufficientAmountAOptimal()";
            const SELECTOR: [u8; 4] = [254u8, 73u8, 109u8, 246u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `InsufficientAmountB()` and selector `0x34c90624`.
```solidity
error InsufficientAmountB();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InsufficientAmountB {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InsufficientAmountB> for UnderlyingRustTuple<'_> {
            fn from(value: InsufficientAmountB) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InsufficientAmountB {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InsufficientAmountB {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InsufficientAmountB()";
            const SELECTOR: [u8; 4] = [52u8, 201u8, 6u8, 36u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `InsufficientAmountBDesired()` and selector `0xacee0513`.
```solidity
error InsufficientAmountBDesired();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InsufficientAmountBDesired {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InsufficientAmountBDesired>
        for UnderlyingRustTuple<'_> {
            fn from(value: InsufficientAmountBDesired) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for InsufficientAmountBDesired {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InsufficientAmountBDesired {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InsufficientAmountBDesired()";
            const SELECTOR: [u8; 4] = [172u8, 238u8, 5u8, 19u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `InsufficientLiquidity()` and selector `0xbb55fd27`.
```solidity
error InsufficientLiquidity();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InsufficientLiquidity {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InsufficientLiquidity> for UnderlyingRustTuple<'_> {
            fn from(value: InsufficientLiquidity) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InsufficientLiquidity {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InsufficientLiquidity {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InsufficientLiquidity()";
            const SELECTOR: [u8; 4] = [187u8, 85u8, 253u8, 39u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `InsufficientOutputAmount()` and selector `0x42301c23`.
```solidity
error InsufficientOutputAmount();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InsufficientOutputAmount {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InsufficientOutputAmount>
        for UnderlyingRustTuple<'_> {
            fn from(value: InsufficientOutputAmount) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for InsufficientOutputAmount {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InsufficientOutputAmount {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InsufficientOutputAmount()";
            const SELECTOR: [u8; 4] = [66u8, 48u8, 28u8, 35u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `InvalidAmountInForETHDeposit()` and selector `0x70a3fb92`.
```solidity
error InvalidAmountInForETHDeposit();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidAmountInForETHDeposit {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidAmountInForETHDeposit>
        for UnderlyingRustTuple<'_> {
            fn from(value: InvalidAmountInForETHDeposit) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for InvalidAmountInForETHDeposit {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidAmountInForETHDeposit {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidAmountInForETHDeposit()";
            const SELECTOR: [u8; 4] = [112u8, 163u8, 251u8, 146u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `InvalidPath()` and selector `0x20db8267`.
```solidity
error InvalidPath();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidPath {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidPath> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidPath) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidPath {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidPath {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidPath()";
            const SELECTOR: [u8; 4] = [32u8, 219u8, 130u8, 103u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `InvalidRouteA()` and selector `0x4ea0e338`.
```solidity
error InvalidRouteA();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidRouteA {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidRouteA> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidRouteA) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidRouteA {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidRouteA {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidRouteA()";
            const SELECTOR: [u8; 4] = [78u8, 160u8, 227u8, 56u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `InvalidRouteB()` and selector `0xcac9040c`.
```solidity
error InvalidRouteB();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidRouteB {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidRouteB> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidRouteB) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidRouteB {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidRouteB {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidRouteB()";
            const SELECTOR: [u8; 4] = [202u8, 201u8, 4u8, 12u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `InvalidTokenInForETHDeposit()` and selector `0xae6d566f`.
```solidity
error InvalidTokenInForETHDeposit();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidTokenInForETHDeposit {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidTokenInForETHDeposit>
        for UnderlyingRustTuple<'_> {
            fn from(value: InvalidTokenInForETHDeposit) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for InvalidTokenInForETHDeposit {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidTokenInForETHDeposit {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidTokenInForETHDeposit()";
            const SELECTOR: [u8; 4] = [174u8, 109u8, 86u8, 111u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `OnlyWETH()` and selector `0x01f180c9`.
```solidity
error OnlyWETH();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OnlyWETH {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<OnlyWETH> for UnderlyingRustTuple<'_> {
            fn from(value: OnlyWETH) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OnlyWETH {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OnlyWETH {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OnlyWETH()";
            const SELECTOR: [u8; 4] = [1u8, 241u8, 128u8, 201u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `PoolDoesNotExist()` and selector `0x9c8787c0`.
```solidity
error PoolDoesNotExist();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct PoolDoesNotExist {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<PoolDoesNotExist> for UnderlyingRustTuple<'_> {
            fn from(value: PoolDoesNotExist) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for PoolDoesNotExist {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for PoolDoesNotExist {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "PoolDoesNotExist()";
            const SELECTOR: [u8; 4] = [156u8, 135u8, 135u8, 192u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `PoolFactoryDoesNotExist()` and selector `0x9a73ab46`.
```solidity
error PoolFactoryDoesNotExist();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct PoolFactoryDoesNotExist {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<PoolFactoryDoesNotExist> for UnderlyingRustTuple<'_> {
            fn from(value: PoolFactoryDoesNotExist) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for PoolFactoryDoesNotExist {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for PoolFactoryDoesNotExist {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "PoolFactoryDoesNotExist()";
            const SELECTOR: [u8; 4] = [154u8, 115u8, 171u8, 70u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `SameAddresses()` and selector `0xca57cff4`.
```solidity
error SameAddresses();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SameAddresses {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<SameAddresses> for UnderlyingRustTuple<'_> {
            fn from(value: SameAddresses) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for SameAddresses {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for SameAddresses {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "SameAddresses()";
            const SELECTOR: [u8; 4] = [202u8, 87u8, 207u8, 244u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `ZeroAddress()` and selector `0xd92e233d`.
```solidity
error ZeroAddress();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ZeroAddress {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ZeroAddress> for UnderlyingRustTuple<'_> {
            fn from(value: ZeroAddress) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ZeroAddress {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ZeroAddress {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ZeroAddress()";
            const SELECTOR: [u8; 4] = [217u8, 46u8, 35u8, 61u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Function with signature `ETHER()` and selector `0x42cb1fbc`.
```solidity
function ETHER() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ETHERCall {}
    ///Container type for the return parameters of the [`ETHER()`](ETHERCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ETHERReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<ETHERCall> for UnderlyingRustTuple<'_> {
                fn from(value: ETHERCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ETHERCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<ETHERReturn> for UnderlyingRustTuple<'_> {
                fn from(value: ETHERReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ETHERReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for ETHERCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = ETHERReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ETHER()";
            const SELECTOR: [u8; 4] = [66u8, 203u8, 31u8, 188u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `UNSAFE_swapExactTokensForTokens(uint256[],(address,address,bool,address)[],address,uint256)` and selector `0x4111d597`.
```solidity
function UNSAFE_swapExactTokensForTokens(uint256[] memory amounts, IRouter.Route[] memory routes, address to, uint256 deadline) external returns (uint256[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UNSAFE_swapExactTokensForTokensCall {
        #[allow(missing_docs)]
        pub amounts: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
        #[allow(missing_docs)]
        pub routes: alloy::sol_types::private::Vec<
            <IRouter::Route as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub to: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub deadline: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`UNSAFE_swapExactTokensForTokens(uint256[],(address,address,bool,address)[],address,uint256)`](UNSAFE_swapExactTokensForTokensCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UNSAFE_swapExactTokensForTokensReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
                alloy::sol_types::sol_data::Array<IRouter::Route>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U256,
                >,
                alloy::sol_types::private::Vec<
                    <IRouter::Route as alloy::sol_types::SolType>::RustType,
                >,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UNSAFE_swapExactTokensForTokensCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: UNSAFE_swapExactTokensForTokensCall) -> Self {
                    (value.amounts, value.routes, value.to, value.deadline)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for UNSAFE_swapExactTokensForTokensCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        amounts: tuple.0,
                        routes: tuple.1,
                        to: tuple.2,
                        deadline: tuple.3,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U256,
                >,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UNSAFE_swapExactTokensForTokensReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: UNSAFE_swapExactTokensForTokensReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for UNSAFE_swapExactTokensForTokensReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for UNSAFE_swapExactTokensForTokensCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
                alloy::sol_types::sol_data::Array<IRouter::Route>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = UNSAFE_swapExactTokensForTokensReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "UNSAFE_swapExactTokensForTokens(uint256[],(address,address,bool,address)[],address,uint256)";
            const SELECTOR: [u8; 4] = [65u8, 17u8, 213u8, 151u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.amounts),
                    <alloy::sol_types::sol_data::Array<
                        IRouter::Route,
                    > as alloy_sol_types::SolType>::tokenize(&self.routes),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.to,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.deadline),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `addLiquidity(address,address,bool,uint256,uint256,uint256,uint256,address,uint256)` and selector `0x5a47ddc3`.
```solidity
function addLiquidity(address tokenA, address tokenB, bool stable, uint256 amountADesired, uint256 amountBDesired, uint256 amountAMin, uint256 amountBMin, address to, uint256 deadline) external returns (uint256 amountA, uint256 amountB, uint256 liquidity);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addLiquidityCall {
        #[allow(missing_docs)]
        pub tokenA: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub tokenB: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub stable: bool,
        #[allow(missing_docs)]
        pub amountADesired: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub amountBDesired: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub amountAMin: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub amountBMin: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub to: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub deadline: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`addLiquidity(address,address,bool,uint256,uint256,uint256,uint256,address,uint256)`](addLiquidityCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addLiquidityReturn {
        #[allow(missing_docs)]
        pub amountA: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub amountB: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub liquidity: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                bool,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<addLiquidityCall> for UnderlyingRustTuple<'_> {
                fn from(value: addLiquidityCall) -> Self {
                    (
                        value.tokenA,
                        value.tokenB,
                        value.stable,
                        value.amountADesired,
                        value.amountBDesired,
                        value.amountAMin,
                        value.amountBMin,
                        value.to,
                        value.deadline,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for addLiquidityCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        tokenA: tuple.0,
                        tokenB: tuple.1,
                        stable: tuple.2,
                        amountADesired: tuple.3,
                        amountBDesired: tuple.4,
                        amountAMin: tuple.5,
                        amountBMin: tuple.6,
                        to: tuple.7,
                        deadline: tuple.8,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<addLiquidityReturn> for UnderlyingRustTuple<'_> {
                fn from(value: addLiquidityReturn) -> Self {
                    (value.amountA, value.amountB, value.liquidity)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for addLiquidityReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        amountA: tuple.0,
                        amountB: tuple.1,
                        liquidity: tuple.2,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for addLiquidityCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = addLiquidityReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "addLiquidity(address,address,bool,uint256,uint256,uint256,uint256,address,uint256)";
            const SELECTOR: [u8; 4] = [90u8, 71u8, 221u8, 195u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.tokenA,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.tokenB,
                    ),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.stable,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amountADesired),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amountBDesired),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amountAMin),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amountBMin),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.to,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.deadline),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `addLiquidityETH(address,bool,uint256,uint256,uint256,address,uint256)` and selector `0xb7e0d4c0`.
```solidity
function addLiquidityETH(address token, bool stable, uint256 amountTokenDesired, uint256 amountTokenMin, uint256 amountETHMin, address to, uint256 deadline) external payable returns (uint256 amountToken, uint256 amountETH, uint256 liquidity);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addLiquidityETHCall {
        #[allow(missing_docs)]
        pub token: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub stable: bool,
        #[allow(missing_docs)]
        pub amountTokenDesired: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub amountTokenMin: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub amountETHMin: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub to: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub deadline: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`addLiquidityETH(address,bool,uint256,uint256,uint256,address,uint256)`](addLiquidityETHCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addLiquidityETHReturn {
        #[allow(missing_docs)]
        pub amountToken: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub amountETH: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub liquidity: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                bool,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<addLiquidityETHCall> for UnderlyingRustTuple<'_> {
                fn from(value: addLiquidityETHCall) -> Self {
                    (
                        value.token,
                        value.stable,
                        value.amountTokenDesired,
                        value.amountTokenMin,
                        value.amountETHMin,
                        value.to,
                        value.deadline,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for addLiquidityETHCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        token: tuple.0,
                        stable: tuple.1,
                        amountTokenDesired: tuple.2,
                        amountTokenMin: tuple.3,
                        amountETHMin: tuple.4,
                        to: tuple.5,
                        deadline: tuple.6,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<addLiquidityETHReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: addLiquidityETHReturn) -> Self {
                    (value.amountToken, value.amountETH, value.liquidity)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for addLiquidityETHReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        amountToken: tuple.0,
                        amountETH: tuple.1,
                        liquidity: tuple.2,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for addLiquidityETHCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = addLiquidityETHReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "addLiquidityETH(address,bool,uint256,uint256,uint256,address,uint256)";
            const SELECTOR: [u8; 4] = [183u8, 224u8, 212u8, 192u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.token,
                    ),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.stable,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amountTokenDesired),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amountTokenMin),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amountETHMin),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.to,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.deadline),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `defaultFactory()` and selector `0xd4b6846d`.
```solidity
function defaultFactory() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct defaultFactoryCall {}
    ///Container type for the return parameters of the [`defaultFactory()`](defaultFactoryCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct defaultFactoryReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<defaultFactoryCall> for UnderlyingRustTuple<'_> {
                fn from(value: defaultFactoryCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for defaultFactoryCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<defaultFactoryReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: defaultFactoryReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for defaultFactoryReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for defaultFactoryCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = defaultFactoryReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "defaultFactory()";
            const SELECTOR: [u8; 4] = [212u8, 182u8, 132u8, 109u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `factoryRegistry()` and selector `0x3bf0c9fb`.
```solidity
function factoryRegistry() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct factoryRegistryCall {}
    ///Container type for the return parameters of the [`factoryRegistry()`](factoryRegistryCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct factoryRegistryReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<factoryRegistryCall> for UnderlyingRustTuple<'_> {
                fn from(value: factoryRegistryCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for factoryRegistryCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<factoryRegistryReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: factoryRegistryReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for factoryRegistryReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for factoryRegistryCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = factoryRegistryReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "factoryRegistry()";
            const SELECTOR: [u8; 4] = [59u8, 240u8, 201u8, 251u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `generateZapInParams(address,address,bool,address,uint256,uint256,(address,address,bool,address)[],(address,address,bool,address)[])` and selector `0x07db50fa`.
```solidity
function generateZapInParams(address tokenA, address tokenB, bool stable, address _factory, uint256 amountInA, uint256 amountInB, IRouter.Route[] memory routesA, IRouter.Route[] memory routesB) external view returns (uint256 amountOutMinA, uint256 amountOutMinB, uint256 amountAMin, uint256 amountBMin);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct generateZapInParamsCall {
        #[allow(missing_docs)]
        pub tokenA: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub tokenB: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub stable: bool,
        #[allow(missing_docs)]
        pub _factory: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub amountInA: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub amountInB: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub routesA: alloy::sol_types::private::Vec<
            <IRouter::Route as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub routesB: alloy::sol_types::private::Vec<
            <IRouter::Route as alloy::sol_types::SolType>::RustType,
        >,
    }
    ///Container type for the return parameters of the [`generateZapInParams(address,address,bool,address,uint256,uint256,(address,address,bool,address)[],(address,address,bool,address)[])`](generateZapInParamsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct generateZapInParamsReturn {
        #[allow(missing_docs)]
        pub amountOutMinA: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub amountOutMinB: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub amountAMin: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub amountBMin: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Array<IRouter::Route>,
                alloy::sol_types::sol_data::Array<IRouter::Route>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                bool,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Vec<
                    <IRouter::Route as alloy::sol_types::SolType>::RustType,
                >,
                alloy::sol_types::private::Vec<
                    <IRouter::Route as alloy::sol_types::SolType>::RustType,
                >,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<generateZapInParamsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: generateZapInParamsCall) -> Self {
                    (
                        value.tokenA,
                        value.tokenB,
                        value.stable,
                        value._factory,
                        value.amountInA,
                        value.amountInB,
                        value.routesA,
                        value.routesB,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for generateZapInParamsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        tokenA: tuple.0,
                        tokenB: tuple.1,
                        stable: tuple.2,
                        _factory: tuple.3,
                        amountInA: tuple.4,
                        amountInB: tuple.5,
                        routesA: tuple.6,
                        routesB: tuple.7,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<generateZapInParamsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: generateZapInParamsReturn) -> Self {
                    (
                        value.amountOutMinA,
                        value.amountOutMinB,
                        value.amountAMin,
                        value.amountBMin,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for generateZapInParamsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        amountOutMinA: tuple.0,
                        amountOutMinB: tuple.1,
                        amountAMin: tuple.2,
                        amountBMin: tuple.3,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for generateZapInParamsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Array<IRouter::Route>,
                alloy::sol_types::sol_data::Array<IRouter::Route>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = generateZapInParamsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "generateZapInParams(address,address,bool,address,uint256,uint256,(address,address,bool,address)[],(address,address,bool,address)[])";
            const SELECTOR: [u8; 4] = [7u8, 219u8, 80u8, 250u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.tokenA,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.tokenB,
                    ),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.stable,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._factory,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amountInA),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amountInB),
                    <alloy::sol_types::sol_data::Array<
                        IRouter::Route,
                    > as alloy_sol_types::SolType>::tokenize(&self.routesA),
                    <alloy::sol_types::sol_data::Array<
                        IRouter::Route,
                    > as alloy_sol_types::SolType>::tokenize(&self.routesB),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `generateZapOutParams(address,address,bool,address,uint256,(address,address,bool,address)[],(address,address,bool,address)[])` and selector `0x7539d413`.
```solidity
function generateZapOutParams(address tokenA, address tokenB, bool stable, address _factory, uint256 liquidity, IRouter.Route[] memory routesA, IRouter.Route[] memory routesB) external view returns (uint256 amountOutMinA, uint256 amountOutMinB, uint256 amountAMin, uint256 amountBMin);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct generateZapOutParamsCall {
        #[allow(missing_docs)]
        pub tokenA: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub tokenB: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub stable: bool,
        #[allow(missing_docs)]
        pub _factory: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub liquidity: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub routesA: alloy::sol_types::private::Vec<
            <IRouter::Route as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub routesB: alloy::sol_types::private::Vec<
            <IRouter::Route as alloy::sol_types::SolType>::RustType,
        >,
    }
    ///Container type for the return parameters of the [`generateZapOutParams(address,address,bool,address,uint256,(address,address,bool,address)[],(address,address,bool,address)[])`](generateZapOutParamsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct generateZapOutParamsReturn {
        #[allow(missing_docs)]
        pub amountOutMinA: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub amountOutMinB: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub amountAMin: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub amountBMin: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Array<IRouter::Route>,
                alloy::sol_types::sol_data::Array<IRouter::Route>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                bool,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Vec<
                    <IRouter::Route as alloy::sol_types::SolType>::RustType,
                >,
                alloy::sol_types::private::Vec<
                    <IRouter::Route as alloy::sol_types::SolType>::RustType,
                >,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<generateZapOutParamsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: generateZapOutParamsCall) -> Self {
                    (
                        value.tokenA,
                        value.tokenB,
                        value.stable,
                        value._factory,
                        value.liquidity,
                        value.routesA,
                        value.routesB,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for generateZapOutParamsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        tokenA: tuple.0,
                        tokenB: tuple.1,
                        stable: tuple.2,
                        _factory: tuple.3,
                        liquidity: tuple.4,
                        routesA: tuple.5,
                        routesB: tuple.6,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<generateZapOutParamsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: generateZapOutParamsReturn) -> Self {
                    (
                        value.amountOutMinA,
                        value.amountOutMinB,
                        value.amountAMin,
                        value.amountBMin,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for generateZapOutParamsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        amountOutMinA: tuple.0,
                        amountOutMinB: tuple.1,
                        amountAMin: tuple.2,
                        amountBMin: tuple.3,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for generateZapOutParamsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Array<IRouter::Route>,
                alloy::sol_types::sol_data::Array<IRouter::Route>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = generateZapOutParamsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "generateZapOutParams(address,address,bool,address,uint256,(address,address,bool,address)[],(address,address,bool,address)[])";
            const SELECTOR: [u8; 4] = [117u8, 57u8, 212u8, 19u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.tokenA,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.tokenB,
                    ),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.stable,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._factory,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.liquidity),
                    <alloy::sol_types::sol_data::Array<
                        IRouter::Route,
                    > as alloy_sol_types::SolType>::tokenize(&self.routesA),
                    <alloy::sol_types::sol_data::Array<
                        IRouter::Route,
                    > as alloy_sol_types::SolType>::tokenize(&self.routesB),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `getAmountsOut(uint256,(address,address,bool,address)[])` and selector `0x5509a1ac`.
```solidity
function getAmountsOut(uint256 amountIn, IRouter.Route[] memory routes) external view returns (uint256[] memory amounts);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getAmountsOutCall {
        #[allow(missing_docs)]
        pub amountIn: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub routes: alloy::sol_types::private::Vec<
            <IRouter::Route as alloy::sol_types::SolType>::RustType,
        >,
    }
    ///Container type for the return parameters of the [`getAmountsOut(uint256,(address,address,bool,address)[])`](getAmountsOutCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getAmountsOutReturn {
        #[allow(missing_docs)]
        pub amounts: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Array<IRouter::Route>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Vec<
                    <IRouter::Route as alloy::sol_types::SolType>::RustType,
                >,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getAmountsOutCall> for UnderlyingRustTuple<'_> {
                fn from(value: getAmountsOutCall) -> Self {
                    (value.amountIn, value.routes)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getAmountsOutCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        amountIn: tuple.0,
                        routes: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U256,
                >,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getAmountsOutReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getAmountsOutReturn) -> Self {
                    (value.amounts,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getAmountsOutReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { amounts: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getAmountsOutCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Array<IRouter::Route>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getAmountsOutReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getAmountsOut(uint256,(address,address,bool,address)[])";
            const SELECTOR: [u8; 4] = [85u8, 9u8, 161u8, 172u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amountIn),
                    <alloy::sol_types::sol_data::Array<
                        IRouter::Route,
                    > as alloy_sol_types::SolType>::tokenize(&self.routes),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `getReserves(address,address,bool,address)` and selector `0x8c0037dc`.
```solidity
function getReserves(address tokenA, address tokenB, bool stable, address _factory) external view returns (uint256 reserveA, uint256 reserveB);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getReservesCall {
        #[allow(missing_docs)]
        pub tokenA: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub tokenB: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub stable: bool,
        #[allow(missing_docs)]
        pub _factory: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getReserves(address,address,bool,address)`](getReservesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getReservesReturn {
        #[allow(missing_docs)]
        pub reserveA: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub reserveB: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                bool,
                alloy::sol_types::private::Address,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getReservesCall> for UnderlyingRustTuple<'_> {
                fn from(value: getReservesCall) -> Self {
                    (value.tokenA, value.tokenB, value.stable, value._factory)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getReservesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        tokenA: tuple.0,
                        tokenB: tuple.1,
                        stable: tuple.2,
                        _factory: tuple.3,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getReservesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getReservesReturn) -> Self {
                    (value.reserveA, value.reserveB)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getReservesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        reserveA: tuple.0,
                        reserveB: tuple.1,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getReservesCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getReservesReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getReserves(address,address,bool,address)";
            const SELECTOR: [u8; 4] = [140u8, 0u8, 55u8, 220u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.tokenA,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.tokenB,
                    ),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.stable,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._factory,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `isTrustedForwarder(address)` and selector `0x572b6c05`.
```solidity
function isTrustedForwarder(address forwarder) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isTrustedForwarderCall {
        #[allow(missing_docs)]
        pub forwarder: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`isTrustedForwarder(address)`](isTrustedForwarderCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isTrustedForwarderReturn {
        #[allow(missing_docs)]
        pub _0: bool,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<isTrustedForwarderCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: isTrustedForwarderCall) -> Self {
                    (value.forwarder,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isTrustedForwarderCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { forwarder: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<isTrustedForwarderReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: isTrustedForwarderReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isTrustedForwarderReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isTrustedForwarderCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = isTrustedForwarderReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isTrustedForwarder(address)";
            const SELECTOR: [u8; 4] = [87u8, 43u8, 108u8, 5u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.forwarder,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `poolFor(address,address,bool,address)` and selector `0x874029d9`.
```solidity
function poolFor(address tokenA, address tokenB, bool stable, address _factory) external view returns (address pool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct poolForCall {
        #[allow(missing_docs)]
        pub tokenA: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub tokenB: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub stable: bool,
        #[allow(missing_docs)]
        pub _factory: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`poolFor(address,address,bool,address)`](poolForCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct poolForReturn {
        #[allow(missing_docs)]
        pub pool: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                bool,
                alloy::sol_types::private::Address,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<poolForCall> for UnderlyingRustTuple<'_> {
                fn from(value: poolForCall) -> Self {
                    (value.tokenA, value.tokenB, value.stable, value._factory)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for poolForCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        tokenA: tuple.0,
                        tokenB: tuple.1,
                        stable: tuple.2,
                        _factory: tuple.3,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<poolForReturn> for UnderlyingRustTuple<'_> {
                fn from(value: poolForReturn) -> Self {
                    (value.pool,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for poolForReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { pool: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for poolForCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = poolForReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "poolFor(address,address,bool,address)";
            const SELECTOR: [u8; 4] = [135u8, 64u8, 41u8, 217u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.tokenA,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.tokenB,
                    ),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.stable,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._factory,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `quoteAddLiquidity(address,address,bool,address,uint256,uint256)` and selector `0xce700c29`.
```solidity
function quoteAddLiquidity(address tokenA, address tokenB, bool stable, address _factory, uint256 amountADesired, uint256 amountBDesired) external view returns (uint256 amountA, uint256 amountB, uint256 liquidity);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct quoteAddLiquidityCall {
        #[allow(missing_docs)]
        pub tokenA: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub tokenB: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub stable: bool,
        #[allow(missing_docs)]
        pub _factory: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub amountADesired: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub amountBDesired: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`quoteAddLiquidity(address,address,bool,address,uint256,uint256)`](quoteAddLiquidityCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct quoteAddLiquidityReturn {
        #[allow(missing_docs)]
        pub amountA: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub amountB: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub liquidity: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                bool,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<quoteAddLiquidityCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: quoteAddLiquidityCall) -> Self {
                    (
                        value.tokenA,
                        value.tokenB,
                        value.stable,
                        value._factory,
                        value.amountADesired,
                        value.amountBDesired,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for quoteAddLiquidityCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        tokenA: tuple.0,
                        tokenB: tuple.1,
                        stable: tuple.2,
                        _factory: tuple.3,
                        amountADesired: tuple.4,
                        amountBDesired: tuple.5,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<quoteAddLiquidityReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: quoteAddLiquidityReturn) -> Self {
                    (value.amountA, value.amountB, value.liquidity)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for quoteAddLiquidityReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        amountA: tuple.0,
                        amountB: tuple.1,
                        liquidity: tuple.2,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for quoteAddLiquidityCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = quoteAddLiquidityReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "quoteAddLiquidity(address,address,bool,address,uint256,uint256)";
            const SELECTOR: [u8; 4] = [206u8, 112u8, 12u8, 41u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.tokenA,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.tokenB,
                    ),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.stable,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._factory,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amountADesired),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amountBDesired),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `quoteRemoveLiquidity(address,address,bool,address,uint256)` and selector `0xc92de3ec`.
```solidity
function quoteRemoveLiquidity(address tokenA, address tokenB, bool stable, address _factory, uint256 liquidity) external view returns (uint256 amountA, uint256 amountB);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct quoteRemoveLiquidityCall {
        #[allow(missing_docs)]
        pub tokenA: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub tokenB: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub stable: bool,
        #[allow(missing_docs)]
        pub _factory: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub liquidity: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`quoteRemoveLiquidity(address,address,bool,address,uint256)`](quoteRemoveLiquidityCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct quoteRemoveLiquidityReturn {
        #[allow(missing_docs)]
        pub amountA: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub amountB: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                bool,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<quoteRemoveLiquidityCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: quoteRemoveLiquidityCall) -> Self {
                    (
                        value.tokenA,
                        value.tokenB,
                        value.stable,
                        value._factory,
                        value.liquidity,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for quoteRemoveLiquidityCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        tokenA: tuple.0,
                        tokenB: tuple.1,
                        stable: tuple.2,
                        _factory: tuple.3,
                        liquidity: tuple.4,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<quoteRemoveLiquidityReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: quoteRemoveLiquidityReturn) -> Self {
                    (value.amountA, value.amountB)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for quoteRemoveLiquidityReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        amountA: tuple.0,
                        amountB: tuple.1,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for quoteRemoveLiquidityCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = quoteRemoveLiquidityReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "quoteRemoveLiquidity(address,address,bool,address,uint256)";
            const SELECTOR: [u8; 4] = [201u8, 45u8, 227u8, 236u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.tokenA,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.tokenB,
                    ),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.stable,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._factory,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.liquidity),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `quoteStableLiquidityRatio(address,address,address)` and selector `0xf5ba53c7`.
```solidity
function quoteStableLiquidityRatio(address tokenA, address tokenB, address _factory) external view returns (uint256 ratio);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct quoteStableLiquidityRatioCall {
        #[allow(missing_docs)]
        pub tokenA: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub tokenB: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _factory: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`quoteStableLiquidityRatio(address,address,address)`](quoteStableLiquidityRatioCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct quoteStableLiquidityRatioReturn {
        #[allow(missing_docs)]
        pub ratio: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<quoteStableLiquidityRatioCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: quoteStableLiquidityRatioCall) -> Self {
                    (value.tokenA, value.tokenB, value._factory)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for quoteStableLiquidityRatioCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        tokenA: tuple.0,
                        tokenB: tuple.1,
                        _factory: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<quoteStableLiquidityRatioReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: quoteStableLiquidityRatioReturn) -> Self {
                    (value.ratio,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for quoteStableLiquidityRatioReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { ratio: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for quoteStableLiquidityRatioCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = quoteStableLiquidityRatioReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "quoteStableLiquidityRatio(address,address,address)";
            const SELECTOR: [u8; 4] = [245u8, 186u8, 83u8, 199u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.tokenA,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.tokenB,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._factory,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `removeLiquidity(address,address,bool,uint256,uint256,uint256,address,uint256)` and selector `0x0dede6c4`.
```solidity
function removeLiquidity(address tokenA, address tokenB, bool stable, uint256 liquidity, uint256 amountAMin, uint256 amountBMin, address to, uint256 deadline) external returns (uint256 amountA, uint256 amountB);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct removeLiquidityCall {
        #[allow(missing_docs)]
        pub tokenA: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub tokenB: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub stable: bool,
        #[allow(missing_docs)]
        pub liquidity: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub amountAMin: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub amountBMin: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub to: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub deadline: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`removeLiquidity(address,address,bool,uint256,uint256,uint256,address,uint256)`](removeLiquidityCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct removeLiquidityReturn {
        #[allow(missing_docs)]
        pub amountA: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub amountB: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                bool,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<removeLiquidityCall> for UnderlyingRustTuple<'_> {
                fn from(value: removeLiquidityCall) -> Self {
                    (
                        value.tokenA,
                        value.tokenB,
                        value.stable,
                        value.liquidity,
                        value.amountAMin,
                        value.amountBMin,
                        value.to,
                        value.deadline,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for removeLiquidityCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        tokenA: tuple.0,
                        tokenB: tuple.1,
                        stable: tuple.2,
                        liquidity: tuple.3,
                        amountAMin: tuple.4,
                        amountBMin: tuple.5,
                        to: tuple.6,
                        deadline: tuple.7,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<removeLiquidityReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: removeLiquidityReturn) -> Self {
                    (value.amountA, value.amountB)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for removeLiquidityReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        amountA: tuple.0,
                        amountB: tuple.1,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for removeLiquidityCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = removeLiquidityReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "removeLiquidity(address,address,bool,uint256,uint256,uint256,address,uint256)";
            const SELECTOR: [u8; 4] = [13u8, 237u8, 230u8, 196u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.tokenA,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.tokenB,
                    ),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.stable,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.liquidity),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amountAMin),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amountBMin),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.to,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.deadline),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `removeLiquidityETH(address,bool,uint256,uint256,uint256,address,uint256)` and selector `0xd7b0e0a5`.
```solidity
function removeLiquidityETH(address token, bool stable, uint256 liquidity, uint256 amountTokenMin, uint256 amountETHMin, address to, uint256 deadline) external returns (uint256 amountToken, uint256 amountETH);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct removeLiquidityETHCall {
        #[allow(missing_docs)]
        pub token: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub stable: bool,
        #[allow(missing_docs)]
        pub liquidity: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub amountTokenMin: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub amountETHMin: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub to: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub deadline: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`removeLiquidityETH(address,bool,uint256,uint256,uint256,address,uint256)`](removeLiquidityETHCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct removeLiquidityETHReturn {
        #[allow(missing_docs)]
        pub amountToken: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub amountETH: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                bool,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<removeLiquidityETHCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: removeLiquidityETHCall) -> Self {
                    (
                        value.token,
                        value.stable,
                        value.liquidity,
                        value.amountTokenMin,
                        value.amountETHMin,
                        value.to,
                        value.deadline,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for removeLiquidityETHCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        token: tuple.0,
                        stable: tuple.1,
                        liquidity: tuple.2,
                        amountTokenMin: tuple.3,
                        amountETHMin: tuple.4,
                        to: tuple.5,
                        deadline: tuple.6,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<removeLiquidityETHReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: removeLiquidityETHReturn) -> Self {
                    (value.amountToken, value.amountETH)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for removeLiquidityETHReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        amountToken: tuple.0,
                        amountETH: tuple.1,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for removeLiquidityETHCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = removeLiquidityETHReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "removeLiquidityETH(address,bool,uint256,uint256,uint256,address,uint256)";
            const SELECTOR: [u8; 4] = [215u8, 176u8, 224u8, 165u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.token,
                    ),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.stable,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.liquidity),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amountTokenMin),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amountETHMin),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.to,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.deadline),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `removeLiquidityETHSupportingFeeOnTransferTokens(address,bool,uint256,uint256,uint256,address,uint256)` and selector `0xfe411f14`.
```solidity
function removeLiquidityETHSupportingFeeOnTransferTokens(address token, bool stable, uint256 liquidity, uint256 amountTokenMin, uint256 amountETHMin, address to, uint256 deadline) external returns (uint256 amountETH);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct removeLiquidityETHSupportingFeeOnTransferTokensCall {
        #[allow(missing_docs)]
        pub token: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub stable: bool,
        #[allow(missing_docs)]
        pub liquidity: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub amountTokenMin: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub amountETHMin: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub to: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub deadline: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`removeLiquidityETHSupportingFeeOnTransferTokens(address,bool,uint256,uint256,uint256,address,uint256)`](removeLiquidityETHSupportingFeeOnTransferTokensCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct removeLiquidityETHSupportingFeeOnTransferTokensReturn {
        #[allow(missing_docs)]
        pub amountETH: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                bool,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<
                removeLiquidityETHSupportingFeeOnTransferTokensCall,
            > for UnderlyingRustTuple<'_> {
                fn from(
                    value: removeLiquidityETHSupportingFeeOnTransferTokensCall,
                ) -> Self {
                    (
                        value.token,
                        value.stable,
                        value.liquidity,
                        value.amountTokenMin,
                        value.amountETHMin,
                        value.to,
                        value.deadline,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for removeLiquidityETHSupportingFeeOnTransferTokensCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        token: tuple.0,
                        stable: tuple.1,
                        liquidity: tuple.2,
                        amountTokenMin: tuple.3,
                        amountETHMin: tuple.4,
                        to: tuple.5,
                        deadline: tuple.6,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<
                removeLiquidityETHSupportingFeeOnTransferTokensReturn,
            > for UnderlyingRustTuple<'_> {
                fn from(
                    value: removeLiquidityETHSupportingFeeOnTransferTokensReturn,
                ) -> Self {
                    (value.amountETH,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for removeLiquidityETHSupportingFeeOnTransferTokensReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { amountETH: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall
        for removeLiquidityETHSupportingFeeOnTransferTokensCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = removeLiquidityETHSupportingFeeOnTransferTokensReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "removeLiquidityETHSupportingFeeOnTransferTokens(address,bool,uint256,uint256,uint256,address,uint256)";
            const SELECTOR: [u8; 4] = [254u8, 65u8, 31u8, 20u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.token,
                    ),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.stable,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.liquidity),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amountTokenMin),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amountETHMin),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.to,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.deadline),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `sortTokens(address,address)` and selector `0x544caa56`.
```solidity
function sortTokens(address tokenA, address tokenB) external pure returns (address token0, address token1);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct sortTokensCall {
        #[allow(missing_docs)]
        pub tokenA: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub tokenB: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`sortTokens(address,address)`](sortTokensCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct sortTokensReturn {
        #[allow(missing_docs)]
        pub token0: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub token1: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<sortTokensCall> for UnderlyingRustTuple<'_> {
                fn from(value: sortTokensCall) -> Self {
                    (value.tokenA, value.tokenB)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for sortTokensCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        tokenA: tuple.0,
                        tokenB: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<sortTokensReturn> for UnderlyingRustTuple<'_> {
                fn from(value: sortTokensReturn) -> Self {
                    (value.token0, value.token1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for sortTokensReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        token0: tuple.0,
                        token1: tuple.1,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for sortTokensCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = sortTokensReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "sortTokens(address,address)";
            const SELECTOR: [u8; 4] = [84u8, 76u8, 170u8, 86u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.tokenA,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.tokenB,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `swapExactETHForTokens(uint256,(address,address,bool,address)[],address,uint256)` and selector `0x903638a4`.
```solidity
function swapExactETHForTokens(uint256 amountOutMin, IRouter.Route[] memory routes, address to, uint256 deadline) external payable returns (uint256[] memory amounts);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct swapExactETHForTokensCall {
        #[allow(missing_docs)]
        pub amountOutMin: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub routes: alloy::sol_types::private::Vec<
            <IRouter::Route as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub to: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub deadline: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`swapExactETHForTokens(uint256,(address,address,bool,address)[],address,uint256)`](swapExactETHForTokensCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct swapExactETHForTokensReturn {
        #[allow(missing_docs)]
        pub amounts: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Array<IRouter::Route>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Vec<
                    <IRouter::Route as alloy::sol_types::SolType>::RustType,
                >,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<swapExactETHForTokensCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: swapExactETHForTokensCall) -> Self {
                    (value.amountOutMin, value.routes, value.to, value.deadline)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for swapExactETHForTokensCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        amountOutMin: tuple.0,
                        routes: tuple.1,
                        to: tuple.2,
                        deadline: tuple.3,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U256,
                >,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<swapExactETHForTokensReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: swapExactETHForTokensReturn) -> Self {
                    (value.amounts,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for swapExactETHForTokensReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { amounts: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for swapExactETHForTokensCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Array<IRouter::Route>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = swapExactETHForTokensReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "swapExactETHForTokens(uint256,(address,address,bool,address)[],address,uint256)";
            const SELECTOR: [u8; 4] = [144u8, 54u8, 56u8, 164u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amountOutMin),
                    <alloy::sol_types::sol_data::Array<
                        IRouter::Route,
                    > as alloy_sol_types::SolType>::tokenize(&self.routes),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.to,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.deadline),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `swapExactETHForTokensSupportingFeeOnTransferTokens(uint256,(address,address,bool,address)[],address,uint256)` and selector `0x3da5acba`.
```solidity
function swapExactETHForTokensSupportingFeeOnTransferTokens(uint256 amountOutMin, IRouter.Route[] memory routes, address to, uint256 deadline) external payable;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct swapExactETHForTokensSupportingFeeOnTransferTokensCall {
        #[allow(missing_docs)]
        pub amountOutMin: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub routes: alloy::sol_types::private::Vec<
            <IRouter::Route as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub to: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub deadline: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`swapExactETHForTokensSupportingFeeOnTransferTokens(uint256,(address,address,bool,address)[],address,uint256)`](swapExactETHForTokensSupportingFeeOnTransferTokensCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct swapExactETHForTokensSupportingFeeOnTransferTokensReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Array<IRouter::Route>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Vec<
                    <IRouter::Route as alloy::sol_types::SolType>::RustType,
                >,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<
                swapExactETHForTokensSupportingFeeOnTransferTokensCall,
            > for UnderlyingRustTuple<'_> {
                fn from(
                    value: swapExactETHForTokensSupportingFeeOnTransferTokensCall,
                ) -> Self {
                    (value.amountOutMin, value.routes, value.to, value.deadline)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for swapExactETHForTokensSupportingFeeOnTransferTokensCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        amountOutMin: tuple.0,
                        routes: tuple.1,
                        to: tuple.2,
                        deadline: tuple.3,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<
                swapExactETHForTokensSupportingFeeOnTransferTokensReturn,
            > for UnderlyingRustTuple<'_> {
                fn from(
                    value: swapExactETHForTokensSupportingFeeOnTransferTokensReturn,
                ) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for swapExactETHForTokensSupportingFeeOnTransferTokensReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall
        for swapExactETHForTokensSupportingFeeOnTransferTokensCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Array<IRouter::Route>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = swapExactETHForTokensSupportingFeeOnTransferTokensReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "swapExactETHForTokensSupportingFeeOnTransferTokens(uint256,(address,address,bool,address)[],address,uint256)";
            const SELECTOR: [u8; 4] = [61u8, 165u8, 172u8, 186u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amountOutMin),
                    <alloy::sol_types::sol_data::Array<
                        IRouter::Route,
                    > as alloy_sol_types::SolType>::tokenize(&self.routes),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.to,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.deadline),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `swapExactTokensForETH(uint256,uint256,(address,address,bool,address)[],address,uint256)` and selector `0xc6b7f1b6`.
```solidity
function swapExactTokensForETH(uint256 amountIn, uint256 amountOutMin, IRouter.Route[] memory routes, address to, uint256 deadline) external returns (uint256[] memory amounts);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct swapExactTokensForETHCall {
        #[allow(missing_docs)]
        pub amountIn: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub amountOutMin: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub routes: alloy::sol_types::private::Vec<
            <IRouter::Route as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub to: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub deadline: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`swapExactTokensForETH(uint256,uint256,(address,address,bool,address)[],address,uint256)`](swapExactTokensForETHCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct swapExactTokensForETHReturn {
        #[allow(missing_docs)]
        pub amounts: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Array<IRouter::Route>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Vec<
                    <IRouter::Route as alloy::sol_types::SolType>::RustType,
                >,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<swapExactTokensForETHCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: swapExactTokensForETHCall) -> Self {
                    (
                        value.amountIn,
                        value.amountOutMin,
                        value.routes,
                        value.to,
                        value.deadline,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for swapExactTokensForETHCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        amountIn: tuple.0,
                        amountOutMin: tuple.1,
                        routes: tuple.2,
                        to: tuple.3,
                        deadline: tuple.4,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U256,
                >,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<swapExactTokensForETHReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: swapExactTokensForETHReturn) -> Self {
                    (value.amounts,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for swapExactTokensForETHReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { amounts: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for swapExactTokensForETHCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Array<IRouter::Route>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = swapExactTokensForETHReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "swapExactTokensForETH(uint256,uint256,(address,address,bool,address)[],address,uint256)";
            const SELECTOR: [u8; 4] = [198u8, 183u8, 241u8, 182u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amountIn),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amountOutMin),
                    <alloy::sol_types::sol_data::Array<
                        IRouter::Route,
                    > as alloy_sol_types::SolType>::tokenize(&self.routes),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.to,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.deadline),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `swapExactTokensForETHSupportingFeeOnTransferTokens(uint256,uint256,(address,address,bool,address)[],address,uint256)` and selector `0x12bc3aca`.
```solidity
function swapExactTokensForETHSupportingFeeOnTransferTokens(uint256 amountIn, uint256 amountOutMin, IRouter.Route[] memory routes, address to, uint256 deadline) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct swapExactTokensForETHSupportingFeeOnTransferTokensCall {
        #[allow(missing_docs)]
        pub amountIn: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub amountOutMin: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub routes: alloy::sol_types::private::Vec<
            <IRouter::Route as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub to: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub deadline: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`swapExactTokensForETHSupportingFeeOnTransferTokens(uint256,uint256,(address,address,bool,address)[],address,uint256)`](swapExactTokensForETHSupportingFeeOnTransferTokensCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct swapExactTokensForETHSupportingFeeOnTransferTokensReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Array<IRouter::Route>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Vec<
                    <IRouter::Route as alloy::sol_types::SolType>::RustType,
                >,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<
                swapExactTokensForETHSupportingFeeOnTransferTokensCall,
            > for UnderlyingRustTuple<'_> {
                fn from(
                    value: swapExactTokensForETHSupportingFeeOnTransferTokensCall,
                ) -> Self {
                    (
                        value.amountIn,
                        value.amountOutMin,
                        value.routes,
                        value.to,
                        value.deadline,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for swapExactTokensForETHSupportingFeeOnTransferTokensCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        amountIn: tuple.0,
                        amountOutMin: tuple.1,
                        routes: tuple.2,
                        to: tuple.3,
                        deadline: tuple.4,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<
                swapExactTokensForETHSupportingFeeOnTransferTokensReturn,
            > for UnderlyingRustTuple<'_> {
                fn from(
                    value: swapExactTokensForETHSupportingFeeOnTransferTokensReturn,
                ) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for swapExactTokensForETHSupportingFeeOnTransferTokensReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall
        for swapExactTokensForETHSupportingFeeOnTransferTokensCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Array<IRouter::Route>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = swapExactTokensForETHSupportingFeeOnTransferTokensReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "swapExactTokensForETHSupportingFeeOnTransferTokens(uint256,uint256,(address,address,bool,address)[],address,uint256)";
            const SELECTOR: [u8; 4] = [18u8, 188u8, 58u8, 202u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amountIn),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amountOutMin),
                    <alloy::sol_types::sol_data::Array<
                        IRouter::Route,
                    > as alloy_sol_types::SolType>::tokenize(&self.routes),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.to,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.deadline),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `swapExactTokensForTokens(uint256,uint256,(address,address,bool,address)[],address,uint256)` and selector `0xcac88ea9`.
```solidity
function swapExactTokensForTokens(uint256 amountIn, uint256 amountOutMin, IRouter.Route[] memory routes, address to, uint256 deadline) external returns (uint256[] memory amounts);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct swapExactTokensForTokensCall {
        #[allow(missing_docs)]
        pub amountIn: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub amountOutMin: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub routes: alloy::sol_types::private::Vec<
            <IRouter::Route as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub to: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub deadline: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`swapExactTokensForTokens(uint256,uint256,(address,address,bool,address)[],address,uint256)`](swapExactTokensForTokensCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct swapExactTokensForTokensReturn {
        #[allow(missing_docs)]
        pub amounts: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Array<IRouter::Route>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Vec<
                    <IRouter::Route as alloy::sol_types::SolType>::RustType,
                >,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<swapExactTokensForTokensCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: swapExactTokensForTokensCall) -> Self {
                    (
                        value.amountIn,
                        value.amountOutMin,
                        value.routes,
                        value.to,
                        value.deadline,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for swapExactTokensForTokensCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        amountIn: tuple.0,
                        amountOutMin: tuple.1,
                        routes: tuple.2,
                        to: tuple.3,
                        deadline: tuple.4,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U256,
                >,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<swapExactTokensForTokensReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: swapExactTokensForTokensReturn) -> Self {
                    (value.amounts,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for swapExactTokensForTokensReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { amounts: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for swapExactTokensForTokensCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Array<IRouter::Route>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = swapExactTokensForTokensReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "swapExactTokensForTokens(uint256,uint256,(address,address,bool,address)[],address,uint256)";
            const SELECTOR: [u8; 4] = [202u8, 200u8, 142u8, 169u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amountIn),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amountOutMin),
                    <alloy::sol_types::sol_data::Array<
                        IRouter::Route,
                    > as alloy_sol_types::SolType>::tokenize(&self.routes),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.to,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.deadline),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `swapExactTokensForTokensSupportingFeeOnTransferTokens(uint256,uint256,(address,address,bool,address)[],address,uint256)` and selector `0x88cd821e`.
```solidity
function swapExactTokensForTokensSupportingFeeOnTransferTokens(uint256 amountIn, uint256 amountOutMin, IRouter.Route[] memory routes, address to, uint256 deadline) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct swapExactTokensForTokensSupportingFeeOnTransferTokensCall {
        #[allow(missing_docs)]
        pub amountIn: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub amountOutMin: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub routes: alloy::sol_types::private::Vec<
            <IRouter::Route as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub to: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub deadline: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`swapExactTokensForTokensSupportingFeeOnTransferTokens(uint256,uint256,(address,address,bool,address)[],address,uint256)`](swapExactTokensForTokensSupportingFeeOnTransferTokensCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct swapExactTokensForTokensSupportingFeeOnTransferTokensReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Array<IRouter::Route>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Vec<
                    <IRouter::Route as alloy::sol_types::SolType>::RustType,
                >,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<
                swapExactTokensForTokensSupportingFeeOnTransferTokensCall,
            > for UnderlyingRustTuple<'_> {
                fn from(
                    value: swapExactTokensForTokensSupportingFeeOnTransferTokensCall,
                ) -> Self {
                    (
                        value.amountIn,
                        value.amountOutMin,
                        value.routes,
                        value.to,
                        value.deadline,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for swapExactTokensForTokensSupportingFeeOnTransferTokensCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        amountIn: tuple.0,
                        amountOutMin: tuple.1,
                        routes: tuple.2,
                        to: tuple.3,
                        deadline: tuple.4,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<
                swapExactTokensForTokensSupportingFeeOnTransferTokensReturn,
            > for UnderlyingRustTuple<'_> {
                fn from(
                    value: swapExactTokensForTokensSupportingFeeOnTransferTokensReturn,
                ) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for swapExactTokensForTokensSupportingFeeOnTransferTokensReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall
        for swapExactTokensForTokensSupportingFeeOnTransferTokensCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Array<IRouter::Route>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = swapExactTokensForTokensSupportingFeeOnTransferTokensReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "swapExactTokensForTokensSupportingFeeOnTransferTokens(uint256,uint256,(address,address,bool,address)[],address,uint256)";
            const SELECTOR: [u8; 4] = [136u8, 205u8, 130u8, 30u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amountIn),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amountOutMin),
                    <alloy::sol_types::sol_data::Array<
                        IRouter::Route,
                    > as alloy_sol_types::SolType>::tokenize(&self.routes),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.to,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.deadline),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `voter()` and selector `0x46c96aac`.
```solidity
function voter() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct voterCall {}
    ///Container type for the return parameters of the [`voter()`](voterCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct voterReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<voterCall> for UnderlyingRustTuple<'_> {
                fn from(value: voterCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for voterCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<voterReturn> for UnderlyingRustTuple<'_> {
                fn from(value: voterReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for voterReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for voterCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = voterReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "voter()";
            const SELECTOR: [u8; 4] = [70u8, 201u8, 106u8, 172u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `weth()` and selector `0x3fc8cef3`.
```solidity
function weth() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct wethCall {}
    ///Container type for the return parameters of the [`weth()`](wethCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct wethReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<wethCall> for UnderlyingRustTuple<'_> {
                fn from(value: wethCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for wethCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<wethReturn> for UnderlyingRustTuple<'_> {
                fn from(value: wethReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for wethReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for wethCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = wethReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "weth()";
            const SELECTOR: [u8; 4] = [63u8, 200u8, 206u8, 243u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `zapIn(address,uint256,uint256,(address,address,bool,address,uint256,uint256,uint256,uint256),(address,address,bool,address)[],(address,address,bool,address)[],address,bool)` and selector `0xfb49bafd`.
```solidity
function zapIn(address tokenIn, uint256 amountInA, uint256 amountInB, IRouter.Zap memory zapInPool, IRouter.Route[] memory routesA, IRouter.Route[] memory routesB, address to, bool stake) external payable returns (uint256 liquidity);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct zapInCall {
        #[allow(missing_docs)]
        pub tokenIn: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub amountInA: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub amountInB: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub zapInPool: <IRouter::Zap as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub routesA: alloy::sol_types::private::Vec<
            <IRouter::Route as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub routesB: alloy::sol_types::private::Vec<
            <IRouter::Route as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub to: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub stake: bool,
    }
    ///Container type for the return parameters of the [`zapIn(address,uint256,uint256,(address,address,bool,address,uint256,uint256,uint256,uint256),(address,address,bool,address)[],(address,address,bool,address)[],address,bool)`](zapInCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct zapInReturn {
        #[allow(missing_docs)]
        pub liquidity: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                IRouter::Zap,
                alloy::sol_types::sol_data::Array<IRouter::Route>,
                alloy::sol_types::sol_data::Array<IRouter::Route>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bool,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                <IRouter::Zap as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::Vec<
                    <IRouter::Route as alloy::sol_types::SolType>::RustType,
                >,
                alloy::sol_types::private::Vec<
                    <IRouter::Route as alloy::sol_types::SolType>::RustType,
                >,
                alloy::sol_types::private::Address,
                bool,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<zapInCall> for UnderlyingRustTuple<'_> {
                fn from(value: zapInCall) -> Self {
                    (
                        value.tokenIn,
                        value.amountInA,
                        value.amountInB,
                        value.zapInPool,
                        value.routesA,
                        value.routesB,
                        value.to,
                        value.stake,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for zapInCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        tokenIn: tuple.0,
                        amountInA: tuple.1,
                        amountInB: tuple.2,
                        zapInPool: tuple.3,
                        routesA: tuple.4,
                        routesB: tuple.5,
                        to: tuple.6,
                        stake: tuple.7,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<zapInReturn> for UnderlyingRustTuple<'_> {
                fn from(value: zapInReturn) -> Self {
                    (value.liquidity,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for zapInReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { liquidity: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for zapInCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                IRouter::Zap,
                alloy::sol_types::sol_data::Array<IRouter::Route>,
                alloy::sol_types::sol_data::Array<IRouter::Route>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bool,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = zapInReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "zapIn(address,uint256,uint256,(address,address,bool,address,uint256,uint256,uint256,uint256),(address,address,bool,address)[],(address,address,bool,address)[],address,bool)";
            const SELECTOR: [u8; 4] = [251u8, 73u8, 186u8, 253u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.tokenIn,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amountInA),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amountInB),
                    <IRouter::Zap as alloy_sol_types::SolType>::tokenize(
                        &self.zapInPool,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        IRouter::Route,
                    > as alloy_sol_types::SolType>::tokenize(&self.routesA),
                    <alloy::sol_types::sol_data::Array<
                        IRouter::Route,
                    > as alloy_sol_types::SolType>::tokenize(&self.routesB),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.to,
                    ),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.stake,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `zapOut(address,uint256,(address,address,bool,address,uint256,uint256,uint256,uint256),(address,address,bool,address)[],(address,address,bool,address)[])` and selector `0xa81b9159`.
```solidity
function zapOut(address tokenOut, uint256 liquidity, IRouter.Zap memory zapOutPool, IRouter.Route[] memory routesA, IRouter.Route[] memory routesB) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct zapOutCall {
        #[allow(missing_docs)]
        pub tokenOut: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub liquidity: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub zapOutPool: <IRouter::Zap as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub routesA: alloy::sol_types::private::Vec<
            <IRouter::Route as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub routesB: alloy::sol_types::private::Vec<
            <IRouter::Route as alloy::sol_types::SolType>::RustType,
        >,
    }
    ///Container type for the return parameters of the [`zapOut(address,uint256,(address,address,bool,address,uint256,uint256,uint256,uint256),(address,address,bool,address)[],(address,address,bool,address)[])`](zapOutCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct zapOutReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                IRouter::Zap,
                alloy::sol_types::sol_data::Array<IRouter::Route>,
                alloy::sol_types::sol_data::Array<IRouter::Route>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                <IRouter::Zap as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::Vec<
                    <IRouter::Route as alloy::sol_types::SolType>::RustType,
                >,
                alloy::sol_types::private::Vec<
                    <IRouter::Route as alloy::sol_types::SolType>::RustType,
                >,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<zapOutCall> for UnderlyingRustTuple<'_> {
                fn from(value: zapOutCall) -> Self {
                    (
                        value.tokenOut,
                        value.liquidity,
                        value.zapOutPool,
                        value.routesA,
                        value.routesB,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for zapOutCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        tokenOut: tuple.0,
                        liquidity: tuple.1,
                        zapOutPool: tuple.2,
                        routesA: tuple.3,
                        routesB: tuple.4,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<zapOutReturn> for UnderlyingRustTuple<'_> {
                fn from(value: zapOutReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for zapOutReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for zapOutCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                IRouter::Zap,
                alloy::sol_types::sol_data::Array<IRouter::Route>,
                alloy::sol_types::sol_data::Array<IRouter::Route>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = zapOutReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "zapOut(address,uint256,(address,address,bool,address,uint256,uint256,uint256,uint256),(address,address,bool,address)[],(address,address,bool,address)[])";
            const SELECTOR: [u8; 4] = [168u8, 27u8, 145u8, 89u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.tokenOut,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.liquidity),
                    <IRouter::Zap as alloy_sol_types::SolType>::tokenize(
                        &self.zapOutPool,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        IRouter::Route,
                    > as alloy_sol_types::SolType>::tokenize(&self.routesA),
                    <alloy::sol_types::sol_data::Array<
                        IRouter::Route,
                    > as alloy_sol_types::SolType>::tokenize(&self.routesB),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    ///Container for all the [`IAerodromeRouter`](self) function calls.
    pub enum IAerodromeRouterCalls {
        #[allow(missing_docs)]
        ETHER(ETHERCall),
        #[allow(missing_docs)]
        UNSAFE_swapExactTokensForTokens(UNSAFE_swapExactTokensForTokensCall),
        #[allow(missing_docs)]
        addLiquidity(addLiquidityCall),
        #[allow(missing_docs)]
        addLiquidityETH(addLiquidityETHCall),
        #[allow(missing_docs)]
        defaultFactory(defaultFactoryCall),
        #[allow(missing_docs)]
        factoryRegistry(factoryRegistryCall),
        #[allow(missing_docs)]
        generateZapInParams(generateZapInParamsCall),
        #[allow(missing_docs)]
        generateZapOutParams(generateZapOutParamsCall),
        #[allow(missing_docs)]
        getAmountsOut(getAmountsOutCall),
        #[allow(missing_docs)]
        getReserves(getReservesCall),
        #[allow(missing_docs)]
        isTrustedForwarder(isTrustedForwarderCall),
        #[allow(missing_docs)]
        poolFor(poolForCall),
        #[allow(missing_docs)]
        quoteAddLiquidity(quoteAddLiquidityCall),
        #[allow(missing_docs)]
        quoteRemoveLiquidity(quoteRemoveLiquidityCall),
        #[allow(missing_docs)]
        quoteStableLiquidityRatio(quoteStableLiquidityRatioCall),
        #[allow(missing_docs)]
        removeLiquidity(removeLiquidityCall),
        #[allow(missing_docs)]
        removeLiquidityETH(removeLiquidityETHCall),
        #[allow(missing_docs)]
        removeLiquidityETHSupportingFeeOnTransferTokens(
            removeLiquidityETHSupportingFeeOnTransferTokensCall,
        ),
        #[allow(missing_docs)]
        sortTokens(sortTokensCall),
        #[allow(missing_docs)]
        swapExactETHForTokens(swapExactETHForTokensCall),
        #[allow(missing_docs)]
        swapExactETHForTokensSupportingFeeOnTransferTokens(
            swapExactETHForTokensSupportingFeeOnTransferTokensCall,
        ),
        #[allow(missing_docs)]
        swapExactTokensForETH(swapExactTokensForETHCall),
        #[allow(missing_docs)]
        swapExactTokensForETHSupportingFeeOnTransferTokens(
            swapExactTokensForETHSupportingFeeOnTransferTokensCall,
        ),
        #[allow(missing_docs)]
        swapExactTokensForTokens(swapExactTokensForTokensCall),
        #[allow(missing_docs)]
        swapExactTokensForTokensSupportingFeeOnTransferTokens(
            swapExactTokensForTokensSupportingFeeOnTransferTokensCall,
        ),
        #[allow(missing_docs)]
        voter(voterCall),
        #[allow(missing_docs)]
        weth(wethCall),
        #[allow(missing_docs)]
        zapIn(zapInCall),
        #[allow(missing_docs)]
        zapOut(zapOutCall),
    }
    #[automatically_derived]
    impl IAerodromeRouterCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [7u8, 219u8, 80u8, 250u8],
            [13u8, 237u8, 230u8, 196u8],
            [18u8, 188u8, 58u8, 202u8],
            [59u8, 240u8, 201u8, 251u8],
            [61u8, 165u8, 172u8, 186u8],
            [63u8, 200u8, 206u8, 243u8],
            [65u8, 17u8, 213u8, 151u8],
            [66u8, 203u8, 31u8, 188u8],
            [70u8, 201u8, 106u8, 172u8],
            [84u8, 76u8, 170u8, 86u8],
            [85u8, 9u8, 161u8, 172u8],
            [87u8, 43u8, 108u8, 5u8],
            [90u8, 71u8, 221u8, 195u8],
            [117u8, 57u8, 212u8, 19u8],
            [135u8, 64u8, 41u8, 217u8],
            [136u8, 205u8, 130u8, 30u8],
            [140u8, 0u8, 55u8, 220u8],
            [144u8, 54u8, 56u8, 164u8],
            [168u8, 27u8, 145u8, 89u8],
            [183u8, 224u8, 212u8, 192u8],
            [198u8, 183u8, 241u8, 182u8],
            [201u8, 45u8, 227u8, 236u8],
            [202u8, 200u8, 142u8, 169u8],
            [206u8, 112u8, 12u8, 41u8],
            [212u8, 182u8, 132u8, 109u8],
            [215u8, 176u8, 224u8, 165u8],
            [245u8, 186u8, 83u8, 199u8],
            [251u8, 73u8, 186u8, 253u8],
            [254u8, 65u8, 31u8, 20u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for IAerodromeRouterCalls {
        const NAME: &'static str = "IAerodromeRouterCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 29usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::ETHER(_) => <ETHERCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::UNSAFE_swapExactTokensForTokens(_) => {
                    <UNSAFE_swapExactTokensForTokensCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::addLiquidity(_) => {
                    <addLiquidityCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::addLiquidityETH(_) => {
                    <addLiquidityETHCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::defaultFactory(_) => {
                    <defaultFactoryCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::factoryRegistry(_) => {
                    <factoryRegistryCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::generateZapInParams(_) => {
                    <generateZapInParamsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::generateZapOutParams(_) => {
                    <generateZapOutParamsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getAmountsOut(_) => {
                    <getAmountsOutCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getReserves(_) => {
                    <getReservesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isTrustedForwarder(_) => {
                    <isTrustedForwarderCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::poolFor(_) => <poolForCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::quoteAddLiquidity(_) => {
                    <quoteAddLiquidityCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::quoteRemoveLiquidity(_) => {
                    <quoteRemoveLiquidityCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::quoteStableLiquidityRatio(_) => {
                    <quoteStableLiquidityRatioCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::removeLiquidity(_) => {
                    <removeLiquidityCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::removeLiquidityETH(_) => {
                    <removeLiquidityETHCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::removeLiquidityETHSupportingFeeOnTransferTokens(_) => {
                    <removeLiquidityETHSupportingFeeOnTransferTokensCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::sortTokens(_) => {
                    <sortTokensCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::swapExactETHForTokens(_) => {
                    <swapExactETHForTokensCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::swapExactETHForTokensSupportingFeeOnTransferTokens(_) => {
                    <swapExactETHForTokensSupportingFeeOnTransferTokensCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::swapExactTokensForETH(_) => {
                    <swapExactTokensForETHCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::swapExactTokensForETHSupportingFeeOnTransferTokens(_) => {
                    <swapExactTokensForETHSupportingFeeOnTransferTokensCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::swapExactTokensForTokens(_) => {
                    <swapExactTokensForTokensCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::swapExactTokensForTokensSupportingFeeOnTransferTokens(_) => {
                    <swapExactTokensForTokensSupportingFeeOnTransferTokensCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::voter(_) => <voterCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::weth(_) => <wethCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::zapIn(_) => <zapInCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::zapOut(_) => <zapOutCall as alloy_sol_types::SolCall>::SELECTOR,
            }
        }
        #[inline]
        fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
            Self::SELECTORS.get(i).copied()
        }
        #[inline]
        fn valid_selector(selector: [u8; 4]) -> bool {
            Self::SELECTORS.binary_search(&selector).is_ok()
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
                bool,
            ) -> alloy_sol_types::Result<IAerodromeRouterCalls>] = &[
                {
                    fn generateZapInParams(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IAerodromeRouterCalls> {
                        <generateZapInParamsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IAerodromeRouterCalls::generateZapInParams)
                    }
                    generateZapInParams
                },
                {
                    fn removeLiquidity(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IAerodromeRouterCalls> {
                        <removeLiquidityCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IAerodromeRouterCalls::removeLiquidity)
                    }
                    removeLiquidity
                },
                {
                    fn swapExactTokensForETHSupportingFeeOnTransferTokens(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IAerodromeRouterCalls> {
                        <swapExactTokensForETHSupportingFeeOnTransferTokensCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                IAerodromeRouterCalls::swapExactTokensForETHSupportingFeeOnTransferTokens,
                            )
                    }
                    swapExactTokensForETHSupportingFeeOnTransferTokens
                },
                {
                    fn factoryRegistry(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IAerodromeRouterCalls> {
                        <factoryRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IAerodromeRouterCalls::factoryRegistry)
                    }
                    factoryRegistry
                },
                {
                    fn swapExactETHForTokensSupportingFeeOnTransferTokens(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IAerodromeRouterCalls> {
                        <swapExactETHForTokensSupportingFeeOnTransferTokensCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                IAerodromeRouterCalls::swapExactETHForTokensSupportingFeeOnTransferTokens,
                            )
                    }
                    swapExactETHForTokensSupportingFeeOnTransferTokens
                },
                {
                    fn weth(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IAerodromeRouterCalls> {
                        <wethCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IAerodromeRouterCalls::weth)
                    }
                    weth
                },
                {
                    fn UNSAFE_swapExactTokensForTokens(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IAerodromeRouterCalls> {
                        <UNSAFE_swapExactTokensForTokensCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IAerodromeRouterCalls::UNSAFE_swapExactTokensForTokens)
                    }
                    UNSAFE_swapExactTokensForTokens
                },
                {
                    fn ETHER(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IAerodromeRouterCalls> {
                        <ETHERCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IAerodromeRouterCalls::ETHER)
                    }
                    ETHER
                },
                {
                    fn voter(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IAerodromeRouterCalls> {
                        <voterCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IAerodromeRouterCalls::voter)
                    }
                    voter
                },
                {
                    fn sortTokens(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IAerodromeRouterCalls> {
                        <sortTokensCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IAerodromeRouterCalls::sortTokens)
                    }
                    sortTokens
                },
                {
                    fn getAmountsOut(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IAerodromeRouterCalls> {
                        <getAmountsOutCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IAerodromeRouterCalls::getAmountsOut)
                    }
                    getAmountsOut
                },
                {
                    fn isTrustedForwarder(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IAerodromeRouterCalls> {
                        <isTrustedForwarderCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IAerodromeRouterCalls::isTrustedForwarder)
                    }
                    isTrustedForwarder
                },
                {
                    fn addLiquidity(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IAerodromeRouterCalls> {
                        <addLiquidityCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IAerodromeRouterCalls::addLiquidity)
                    }
                    addLiquidity
                },
                {
                    fn generateZapOutParams(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IAerodromeRouterCalls> {
                        <generateZapOutParamsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IAerodromeRouterCalls::generateZapOutParams)
                    }
                    generateZapOutParams
                },
                {
                    fn poolFor(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IAerodromeRouterCalls> {
                        <poolForCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IAerodromeRouterCalls::poolFor)
                    }
                    poolFor
                },
                {
                    fn swapExactTokensForTokensSupportingFeeOnTransferTokens(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IAerodromeRouterCalls> {
                        <swapExactTokensForTokensSupportingFeeOnTransferTokensCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                IAerodromeRouterCalls::swapExactTokensForTokensSupportingFeeOnTransferTokens,
                            )
                    }
                    swapExactTokensForTokensSupportingFeeOnTransferTokens
                },
                {
                    fn getReserves(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IAerodromeRouterCalls> {
                        <getReservesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IAerodromeRouterCalls::getReserves)
                    }
                    getReserves
                },
                {
                    fn swapExactETHForTokens(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IAerodromeRouterCalls> {
                        <swapExactETHForTokensCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IAerodromeRouterCalls::swapExactETHForTokens)
                    }
                    swapExactETHForTokens
                },
                {
                    fn zapOut(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IAerodromeRouterCalls> {
                        <zapOutCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IAerodromeRouterCalls::zapOut)
                    }
                    zapOut
                },
                {
                    fn addLiquidityETH(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IAerodromeRouterCalls> {
                        <addLiquidityETHCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IAerodromeRouterCalls::addLiquidityETH)
                    }
                    addLiquidityETH
                },
                {
                    fn swapExactTokensForETH(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IAerodromeRouterCalls> {
                        <swapExactTokensForETHCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IAerodromeRouterCalls::swapExactTokensForETH)
                    }
                    swapExactTokensForETH
                },
                {
                    fn quoteRemoveLiquidity(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IAerodromeRouterCalls> {
                        <quoteRemoveLiquidityCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IAerodromeRouterCalls::quoteRemoveLiquidity)
                    }
                    quoteRemoveLiquidity
                },
                {
                    fn swapExactTokensForTokens(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IAerodromeRouterCalls> {
                        <swapExactTokensForTokensCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IAerodromeRouterCalls::swapExactTokensForTokens)
                    }
                    swapExactTokensForTokens
                },
                {
                    fn quoteAddLiquidity(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IAerodromeRouterCalls> {
                        <quoteAddLiquidityCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IAerodromeRouterCalls::quoteAddLiquidity)
                    }
                    quoteAddLiquidity
                },
                {
                    fn defaultFactory(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IAerodromeRouterCalls> {
                        <defaultFactoryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IAerodromeRouterCalls::defaultFactory)
                    }
                    defaultFactory
                },
                {
                    fn removeLiquidityETH(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IAerodromeRouterCalls> {
                        <removeLiquidityETHCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IAerodromeRouterCalls::removeLiquidityETH)
                    }
                    removeLiquidityETH
                },
                {
                    fn quoteStableLiquidityRatio(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IAerodromeRouterCalls> {
                        <quoteStableLiquidityRatioCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IAerodromeRouterCalls::quoteStableLiquidityRatio)
                    }
                    quoteStableLiquidityRatio
                },
                {
                    fn zapIn(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IAerodromeRouterCalls> {
                        <zapInCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IAerodromeRouterCalls::zapIn)
                    }
                    zapIn
                },
                {
                    fn removeLiquidityETHSupportingFeeOnTransferTokens(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IAerodromeRouterCalls> {
                        <removeLiquidityETHSupportingFeeOnTransferTokensCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                IAerodromeRouterCalls::removeLiquidityETHSupportingFeeOnTransferTokens,
                            )
                    }
                    removeLiquidityETHSupportingFeeOnTransferTokens
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            DECODE_SHIMS[idx](data, validate)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::ETHER(inner) => {
                    <ETHERCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::UNSAFE_swapExactTokensForTokens(inner) => {
                    <UNSAFE_swapExactTokensForTokensCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::addLiquidity(inner) => {
                    <addLiquidityCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::addLiquidityETH(inner) => {
                    <addLiquidityETHCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::defaultFactory(inner) => {
                    <defaultFactoryCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::factoryRegistry(inner) => {
                    <factoryRegistryCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::generateZapInParams(inner) => {
                    <generateZapInParamsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::generateZapOutParams(inner) => {
                    <generateZapOutParamsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getAmountsOut(inner) => {
                    <getAmountsOutCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getReserves(inner) => {
                    <getReservesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::isTrustedForwarder(inner) => {
                    <isTrustedForwarderCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::poolFor(inner) => {
                    <poolForCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::quoteAddLiquidity(inner) => {
                    <quoteAddLiquidityCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::quoteRemoveLiquidity(inner) => {
                    <quoteRemoveLiquidityCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::quoteStableLiquidityRatio(inner) => {
                    <quoteStableLiquidityRatioCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::removeLiquidity(inner) => {
                    <removeLiquidityCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::removeLiquidityETH(inner) => {
                    <removeLiquidityETHCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::removeLiquidityETHSupportingFeeOnTransferTokens(inner) => {
                    <removeLiquidityETHSupportingFeeOnTransferTokensCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::sortTokens(inner) => {
                    <sortTokensCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::swapExactETHForTokens(inner) => {
                    <swapExactETHForTokensCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::swapExactETHForTokensSupportingFeeOnTransferTokens(inner) => {
                    <swapExactETHForTokensSupportingFeeOnTransferTokensCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::swapExactTokensForETH(inner) => {
                    <swapExactTokensForETHCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::swapExactTokensForETHSupportingFeeOnTransferTokens(inner) => {
                    <swapExactTokensForETHSupportingFeeOnTransferTokensCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::swapExactTokensForTokens(inner) => {
                    <swapExactTokensForTokensCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::swapExactTokensForTokensSupportingFeeOnTransferTokens(inner) => {
                    <swapExactTokensForTokensSupportingFeeOnTransferTokensCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::voter(inner) => {
                    <voterCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::weth(inner) => {
                    <wethCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::zapIn(inner) => {
                    <zapInCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::zapOut(inner) => {
                    <zapOutCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::ETHER(inner) => {
                    <ETHERCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::UNSAFE_swapExactTokensForTokens(inner) => {
                    <UNSAFE_swapExactTokensForTokensCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::addLiquidity(inner) => {
                    <addLiquidityCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::addLiquidityETH(inner) => {
                    <addLiquidityETHCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::defaultFactory(inner) => {
                    <defaultFactoryCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::factoryRegistry(inner) => {
                    <factoryRegistryCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::generateZapInParams(inner) => {
                    <generateZapInParamsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::generateZapOutParams(inner) => {
                    <generateZapOutParamsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getAmountsOut(inner) => {
                    <getAmountsOutCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getReserves(inner) => {
                    <getReservesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isTrustedForwarder(inner) => {
                    <isTrustedForwarderCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::poolFor(inner) => {
                    <poolForCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::quoteAddLiquidity(inner) => {
                    <quoteAddLiquidityCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::quoteRemoveLiquidity(inner) => {
                    <quoteRemoveLiquidityCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::quoteStableLiquidityRatio(inner) => {
                    <quoteStableLiquidityRatioCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::removeLiquidity(inner) => {
                    <removeLiquidityCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::removeLiquidityETH(inner) => {
                    <removeLiquidityETHCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::removeLiquidityETHSupportingFeeOnTransferTokens(inner) => {
                    <removeLiquidityETHSupportingFeeOnTransferTokensCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::sortTokens(inner) => {
                    <sortTokensCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::swapExactETHForTokens(inner) => {
                    <swapExactETHForTokensCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::swapExactETHForTokensSupportingFeeOnTransferTokens(inner) => {
                    <swapExactETHForTokensSupportingFeeOnTransferTokensCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::swapExactTokensForETH(inner) => {
                    <swapExactTokensForETHCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::swapExactTokensForETHSupportingFeeOnTransferTokens(inner) => {
                    <swapExactTokensForETHSupportingFeeOnTransferTokensCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::swapExactTokensForTokens(inner) => {
                    <swapExactTokensForTokensCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::swapExactTokensForTokensSupportingFeeOnTransferTokens(inner) => {
                    <swapExactTokensForTokensSupportingFeeOnTransferTokensCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::voter(inner) => {
                    <voterCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::weth(inner) => {
                    <wethCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::zapIn(inner) => {
                    <zapInCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::zapOut(inner) => {
                    <zapOutCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
            }
        }
    }
    ///Container for all the [`IAerodromeRouter`](self) custom errors.
    pub enum IAerodromeRouterErrors {
        #[allow(missing_docs)]
        ETHTransferFailed(ETHTransferFailed),
        #[allow(missing_docs)]
        Expired(Expired),
        #[allow(missing_docs)]
        InsufficientAmount(InsufficientAmount),
        #[allow(missing_docs)]
        InsufficientAmountA(InsufficientAmountA),
        #[allow(missing_docs)]
        InsufficientAmountADesired(InsufficientAmountADesired),
        #[allow(missing_docs)]
        InsufficientAmountAOptimal(InsufficientAmountAOptimal),
        #[allow(missing_docs)]
        InsufficientAmountB(InsufficientAmountB),
        #[allow(missing_docs)]
        InsufficientAmountBDesired(InsufficientAmountBDesired),
        #[allow(missing_docs)]
        InsufficientLiquidity(InsufficientLiquidity),
        #[allow(missing_docs)]
        InsufficientOutputAmount(InsufficientOutputAmount),
        #[allow(missing_docs)]
        InvalidAmountInForETHDeposit(InvalidAmountInForETHDeposit),
        #[allow(missing_docs)]
        InvalidPath(InvalidPath),
        #[allow(missing_docs)]
        InvalidRouteA(InvalidRouteA),
        #[allow(missing_docs)]
        InvalidRouteB(InvalidRouteB),
        #[allow(missing_docs)]
        InvalidTokenInForETHDeposit(InvalidTokenInForETHDeposit),
        #[allow(missing_docs)]
        OnlyWETH(OnlyWETH),
        #[allow(missing_docs)]
        PoolDoesNotExist(PoolDoesNotExist),
        #[allow(missing_docs)]
        PoolFactoryDoesNotExist(PoolFactoryDoesNotExist),
        #[allow(missing_docs)]
        SameAddresses(SameAddresses),
        #[allow(missing_docs)]
        ZeroAddress(ZeroAddress),
    }
    #[automatically_derived]
    impl IAerodromeRouterErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [1u8, 241u8, 128u8, 201u8],
            [32u8, 61u8, 130u8, 216u8],
            [32u8, 219u8, 130u8, 103u8],
            [52u8, 201u8, 6u8, 36u8],
            [66u8, 48u8, 28u8, 35u8],
            [78u8, 160u8, 227u8, 56u8],
            [89u8, 69u8, 234u8, 86u8],
            [112u8, 163u8, 251u8, 146u8],
            [143u8, 102u8, 236u8, 20u8],
            [154u8, 115u8, 171u8, 70u8],
            [156u8, 135u8, 135u8, 192u8],
            [172u8, 238u8, 5u8, 19u8],
            [174u8, 109u8, 86u8, 111u8],
            [177u8, 45u8, 19u8, 235u8],
            [187u8, 85u8, 253u8, 39u8],
            [202u8, 87u8, 207u8, 244u8],
            [202u8, 201u8, 4u8, 12u8],
            [217u8, 46u8, 35u8, 61u8],
            [220u8, 107u8, 46u8, 242u8],
            [254u8, 73u8, 109u8, 246u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for IAerodromeRouterErrors {
        const NAME: &'static str = "IAerodromeRouterErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 20usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::ETHTransferFailed(_) => {
                    <ETHTransferFailed as alloy_sol_types::SolError>::SELECTOR
                }
                Self::Expired(_) => <Expired as alloy_sol_types::SolError>::SELECTOR,
                Self::InsufficientAmount(_) => {
                    <InsufficientAmount as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InsufficientAmountA(_) => {
                    <InsufficientAmountA as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InsufficientAmountADesired(_) => {
                    <InsufficientAmountADesired as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InsufficientAmountAOptimal(_) => {
                    <InsufficientAmountAOptimal as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InsufficientAmountB(_) => {
                    <InsufficientAmountB as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InsufficientAmountBDesired(_) => {
                    <InsufficientAmountBDesired as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InsufficientLiquidity(_) => {
                    <InsufficientLiquidity as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InsufficientOutputAmount(_) => {
                    <InsufficientOutputAmount as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidAmountInForETHDeposit(_) => {
                    <InvalidAmountInForETHDeposit as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidPath(_) => {
                    <InvalidPath as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidRouteA(_) => {
                    <InvalidRouteA as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidRouteB(_) => {
                    <InvalidRouteB as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidTokenInForETHDeposit(_) => {
                    <InvalidTokenInForETHDeposit as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OnlyWETH(_) => <OnlyWETH as alloy_sol_types::SolError>::SELECTOR,
                Self::PoolDoesNotExist(_) => {
                    <PoolDoesNotExist as alloy_sol_types::SolError>::SELECTOR
                }
                Self::PoolFactoryDoesNotExist(_) => {
                    <PoolFactoryDoesNotExist as alloy_sol_types::SolError>::SELECTOR
                }
                Self::SameAddresses(_) => {
                    <SameAddresses as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ZeroAddress(_) => {
                    <ZeroAddress as alloy_sol_types::SolError>::SELECTOR
                }
            }
        }
        #[inline]
        fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
            Self::SELECTORS.get(i).copied()
        }
        #[inline]
        fn valid_selector(selector: [u8; 4]) -> bool {
            Self::SELECTORS.binary_search(&selector).is_ok()
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
                bool,
            ) -> alloy_sol_types::Result<IAerodromeRouterErrors>] = &[
                {
                    fn OnlyWETH(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IAerodromeRouterErrors> {
                        <OnlyWETH as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IAerodromeRouterErrors::OnlyWETH)
                    }
                    OnlyWETH
                },
                {
                    fn Expired(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IAerodromeRouterErrors> {
                        <Expired as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IAerodromeRouterErrors::Expired)
                    }
                    Expired
                },
                {
                    fn InvalidPath(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IAerodromeRouterErrors> {
                        <InvalidPath as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IAerodromeRouterErrors::InvalidPath)
                    }
                    InvalidPath
                },
                {
                    fn InsufficientAmountB(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IAerodromeRouterErrors> {
                        <InsufficientAmountB as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IAerodromeRouterErrors::InsufficientAmountB)
                    }
                    InsufficientAmountB
                },
                {
                    fn InsufficientOutputAmount(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IAerodromeRouterErrors> {
                        <InsufficientOutputAmount as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IAerodromeRouterErrors::InsufficientOutputAmount)
                    }
                    InsufficientOutputAmount
                },
                {
                    fn InvalidRouteA(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IAerodromeRouterErrors> {
                        <InvalidRouteA as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IAerodromeRouterErrors::InvalidRouteA)
                    }
                    InvalidRouteA
                },
                {
                    fn InsufficientAmount(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IAerodromeRouterErrors> {
                        <InsufficientAmount as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IAerodromeRouterErrors::InsufficientAmount)
                    }
                    InsufficientAmount
                },
                {
                    fn InvalidAmountInForETHDeposit(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IAerodromeRouterErrors> {
                        <InvalidAmountInForETHDeposit as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IAerodromeRouterErrors::InvalidAmountInForETHDeposit)
                    }
                    InvalidAmountInForETHDeposit
                },
                {
                    fn InsufficientAmountA(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IAerodromeRouterErrors> {
                        <InsufficientAmountA as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IAerodromeRouterErrors::InsufficientAmountA)
                    }
                    InsufficientAmountA
                },
                {
                    fn PoolFactoryDoesNotExist(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IAerodromeRouterErrors> {
                        <PoolFactoryDoesNotExist as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IAerodromeRouterErrors::PoolFactoryDoesNotExist)
                    }
                    PoolFactoryDoesNotExist
                },
                {
                    fn PoolDoesNotExist(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IAerodromeRouterErrors> {
                        <PoolDoesNotExist as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IAerodromeRouterErrors::PoolDoesNotExist)
                    }
                    PoolDoesNotExist
                },
                {
                    fn InsufficientAmountBDesired(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IAerodromeRouterErrors> {
                        <InsufficientAmountBDesired as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IAerodromeRouterErrors::InsufficientAmountBDesired)
                    }
                    InsufficientAmountBDesired
                },
                {
                    fn InvalidTokenInForETHDeposit(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IAerodromeRouterErrors> {
                        <InvalidTokenInForETHDeposit as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IAerodromeRouterErrors::InvalidTokenInForETHDeposit)
                    }
                    InvalidTokenInForETHDeposit
                },
                {
                    fn ETHTransferFailed(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IAerodromeRouterErrors> {
                        <ETHTransferFailed as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IAerodromeRouterErrors::ETHTransferFailed)
                    }
                    ETHTransferFailed
                },
                {
                    fn InsufficientLiquidity(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IAerodromeRouterErrors> {
                        <InsufficientLiquidity as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IAerodromeRouterErrors::InsufficientLiquidity)
                    }
                    InsufficientLiquidity
                },
                {
                    fn SameAddresses(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IAerodromeRouterErrors> {
                        <SameAddresses as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IAerodromeRouterErrors::SameAddresses)
                    }
                    SameAddresses
                },
                {
                    fn InvalidRouteB(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IAerodromeRouterErrors> {
                        <InvalidRouteB as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IAerodromeRouterErrors::InvalidRouteB)
                    }
                    InvalidRouteB
                },
                {
                    fn ZeroAddress(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IAerodromeRouterErrors> {
                        <ZeroAddress as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IAerodromeRouterErrors::ZeroAddress)
                    }
                    ZeroAddress
                },
                {
                    fn InsufficientAmountADesired(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IAerodromeRouterErrors> {
                        <InsufficientAmountADesired as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IAerodromeRouterErrors::InsufficientAmountADesired)
                    }
                    InsufficientAmountADesired
                },
                {
                    fn InsufficientAmountAOptimal(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IAerodromeRouterErrors> {
                        <InsufficientAmountAOptimal as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IAerodromeRouterErrors::InsufficientAmountAOptimal)
                    }
                    InsufficientAmountAOptimal
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            DECODE_SHIMS[idx](data, validate)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::ETHTransferFailed(inner) => {
                    <ETHTransferFailed as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::Expired(inner) => {
                    <Expired as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InsufficientAmount(inner) => {
                    <InsufficientAmount as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InsufficientAmountA(inner) => {
                    <InsufficientAmountA as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InsufficientAmountADesired(inner) => {
                    <InsufficientAmountADesired as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InsufficientAmountAOptimal(inner) => {
                    <InsufficientAmountAOptimal as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InsufficientAmountB(inner) => {
                    <InsufficientAmountB as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InsufficientAmountBDesired(inner) => {
                    <InsufficientAmountBDesired as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InsufficientLiquidity(inner) => {
                    <InsufficientLiquidity as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InsufficientOutputAmount(inner) => {
                    <InsufficientOutputAmount as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidAmountInForETHDeposit(inner) => {
                    <InvalidAmountInForETHDeposit as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidPath(inner) => {
                    <InvalidPath as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidRouteA(inner) => {
                    <InvalidRouteA as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidRouteB(inner) => {
                    <InvalidRouteB as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidTokenInForETHDeposit(inner) => {
                    <InvalidTokenInForETHDeposit as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OnlyWETH(inner) => {
                    <OnlyWETH as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::PoolDoesNotExist(inner) => {
                    <PoolDoesNotExist as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::PoolFactoryDoesNotExist(inner) => {
                    <PoolFactoryDoesNotExist as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::SameAddresses(inner) => {
                    <SameAddresses as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::ZeroAddress(inner) => {
                    <ZeroAddress as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::ETHTransferFailed(inner) => {
                    <ETHTransferFailed as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::Expired(inner) => {
                    <Expired as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InsufficientAmount(inner) => {
                    <InsufficientAmount as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InsufficientAmountA(inner) => {
                    <InsufficientAmountA as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InsufficientAmountADesired(inner) => {
                    <InsufficientAmountADesired as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InsufficientAmountAOptimal(inner) => {
                    <InsufficientAmountAOptimal as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InsufficientAmountB(inner) => {
                    <InsufficientAmountB as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InsufficientAmountBDesired(inner) => {
                    <InsufficientAmountBDesired as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InsufficientLiquidity(inner) => {
                    <InsufficientLiquidity as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InsufficientOutputAmount(inner) => {
                    <InsufficientOutputAmount as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidAmountInForETHDeposit(inner) => {
                    <InvalidAmountInForETHDeposit as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidPath(inner) => {
                    <InvalidPath as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidRouteA(inner) => {
                    <InvalidRouteA as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidRouteB(inner) => {
                    <InvalidRouteB as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidTokenInForETHDeposit(inner) => {
                    <InvalidTokenInForETHDeposit as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OnlyWETH(inner) => {
                    <OnlyWETH as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::PoolDoesNotExist(inner) => {
                    <PoolDoesNotExist as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::PoolFactoryDoesNotExist(inner) => {
                    <PoolFactoryDoesNotExist as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::SameAddresses(inner) => {
                    <SameAddresses as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ZeroAddress(inner) => {
                    <ZeroAddress as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`IAerodromeRouter`](self) contract instance.

See the [wrapper's documentation](`IAerodromeRouterInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IAerodromeRouterInstance<T, P, N> {
        IAerodromeRouterInstance::<T, P, N>::new(address, provider)
    }
    /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
    #[inline]
    pub fn deploy<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        provider: P,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<IAerodromeRouterInstance<T, P, N>>,
    > {
        IAerodromeRouterInstance::<T, P, N>::deploy(provider)
    }
    /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
    #[inline]
    pub fn deploy_builder<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(provider: P) -> alloy_contract::RawCallBuilder<T, P, N> {
        IAerodromeRouterInstance::<T, P, N>::deploy_builder(provider)
    }
    /**A [`IAerodromeRouter`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`IAerodromeRouter`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IAerodromeRouterInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for IAerodromeRouterInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IAerodromeRouterInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IAerodromeRouterInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`IAerodromeRouter`](self) contract instance.

See the [wrapper's documentation](`IAerodromeRouterInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            provider: P,
        ) -> Self {
            Self {
                address,
                provider,
                _network_transport: ::core::marker::PhantomData,
            }
        }
        /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
        #[inline]
        pub async fn deploy(
            provider: P,
        ) -> alloy_contract::Result<IAerodromeRouterInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(provider);
            let contract_address = call_builder.deploy().await?;
            Ok(Self::new(contract_address, call_builder.provider))
        }
        /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
        #[inline]
        pub fn deploy_builder(provider: P) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                ::core::clone::Clone::clone(&BYTECODE),
            )
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<T, P: ::core::clone::Clone, N> IAerodromeRouterInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IAerodromeRouterInstance<T, P, N> {
            IAerodromeRouterInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network_transport: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IAerodromeRouterInstance<T, P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<T, &P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
        ///Creates a new call builder for the [`ETHER`] function.
        pub fn ETHER(&self) -> alloy_contract::SolCallBuilder<T, &P, ETHERCall, N> {
            self.call_builder(&ETHERCall {})
        }
        ///Creates a new call builder for the [`UNSAFE_swapExactTokensForTokens`] function.
        pub fn UNSAFE_swapExactTokensForTokens(
            &self,
            amounts: alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U256,
            >,
            routes: alloy::sol_types::private::Vec<
                <IRouter::Route as alloy::sol_types::SolType>::RustType,
            >,
            to: alloy::sol_types::private::Address,
            deadline: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            UNSAFE_swapExactTokensForTokensCall,
            N,
        > {
            self.call_builder(
                &UNSAFE_swapExactTokensForTokensCall {
                    amounts,
                    routes,
                    to,
                    deadline,
                },
            )
        }
        ///Creates a new call builder for the [`addLiquidity`] function.
        pub fn addLiquidity(
            &self,
            tokenA: alloy::sol_types::private::Address,
            tokenB: alloy::sol_types::private::Address,
            stable: bool,
            amountADesired: alloy::sol_types::private::primitives::aliases::U256,
            amountBDesired: alloy::sol_types::private::primitives::aliases::U256,
            amountAMin: alloy::sol_types::private::primitives::aliases::U256,
            amountBMin: alloy::sol_types::private::primitives::aliases::U256,
            to: alloy::sol_types::private::Address,
            deadline: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, addLiquidityCall, N> {
            self.call_builder(
                &addLiquidityCall {
                    tokenA,
                    tokenB,
                    stable,
                    amountADesired,
                    amountBDesired,
                    amountAMin,
                    amountBMin,
                    to,
                    deadline,
                },
            )
        }
        ///Creates a new call builder for the [`addLiquidityETH`] function.
        pub fn addLiquidityETH(
            &self,
            token: alloy::sol_types::private::Address,
            stable: bool,
            amountTokenDesired: alloy::sol_types::private::primitives::aliases::U256,
            amountTokenMin: alloy::sol_types::private::primitives::aliases::U256,
            amountETHMin: alloy::sol_types::private::primitives::aliases::U256,
            to: alloy::sol_types::private::Address,
            deadline: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, addLiquidityETHCall, N> {
            self.call_builder(
                &addLiquidityETHCall {
                    token,
                    stable,
                    amountTokenDesired,
                    amountTokenMin,
                    amountETHMin,
                    to,
                    deadline,
                },
            )
        }
        ///Creates a new call builder for the [`defaultFactory`] function.
        pub fn defaultFactory(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, defaultFactoryCall, N> {
            self.call_builder(&defaultFactoryCall {})
        }
        ///Creates a new call builder for the [`factoryRegistry`] function.
        pub fn factoryRegistry(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, factoryRegistryCall, N> {
            self.call_builder(&factoryRegistryCall {})
        }
        ///Creates a new call builder for the [`generateZapInParams`] function.
        pub fn generateZapInParams(
            &self,
            tokenA: alloy::sol_types::private::Address,
            tokenB: alloy::sol_types::private::Address,
            stable: bool,
            _factory: alloy::sol_types::private::Address,
            amountInA: alloy::sol_types::private::primitives::aliases::U256,
            amountInB: alloy::sol_types::private::primitives::aliases::U256,
            routesA: alloy::sol_types::private::Vec<
                <IRouter::Route as alloy::sol_types::SolType>::RustType,
            >,
            routesB: alloy::sol_types::private::Vec<
                <IRouter::Route as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, generateZapInParamsCall, N> {
            self.call_builder(
                &generateZapInParamsCall {
                    tokenA,
                    tokenB,
                    stable,
                    _factory,
                    amountInA,
                    amountInB,
                    routesA,
                    routesB,
                },
            )
        }
        ///Creates a new call builder for the [`generateZapOutParams`] function.
        pub fn generateZapOutParams(
            &self,
            tokenA: alloy::sol_types::private::Address,
            tokenB: alloy::sol_types::private::Address,
            stable: bool,
            _factory: alloy::sol_types::private::Address,
            liquidity: alloy::sol_types::private::primitives::aliases::U256,
            routesA: alloy::sol_types::private::Vec<
                <IRouter::Route as alloy::sol_types::SolType>::RustType,
            >,
            routesB: alloy::sol_types::private::Vec<
                <IRouter::Route as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, generateZapOutParamsCall, N> {
            self.call_builder(
                &generateZapOutParamsCall {
                    tokenA,
                    tokenB,
                    stable,
                    _factory,
                    liquidity,
                    routesA,
                    routesB,
                },
            )
        }
        ///Creates a new call builder for the [`getAmountsOut`] function.
        pub fn getAmountsOut(
            &self,
            amountIn: alloy::sol_types::private::primitives::aliases::U256,
            routes: alloy::sol_types::private::Vec<
                <IRouter::Route as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, getAmountsOutCall, N> {
            self.call_builder(
                &getAmountsOutCall {
                    amountIn,
                    routes,
                },
            )
        }
        ///Creates a new call builder for the [`getReserves`] function.
        pub fn getReserves(
            &self,
            tokenA: alloy::sol_types::private::Address,
            tokenB: alloy::sol_types::private::Address,
            stable: bool,
            _factory: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getReservesCall, N> {
            self.call_builder(
                &getReservesCall {
                    tokenA,
                    tokenB,
                    stable,
                    _factory,
                },
            )
        }
        ///Creates a new call builder for the [`isTrustedForwarder`] function.
        pub fn isTrustedForwarder(
            &self,
            forwarder: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, isTrustedForwarderCall, N> {
            self.call_builder(
                &isTrustedForwarderCall {
                    forwarder,
                },
            )
        }
        ///Creates a new call builder for the [`poolFor`] function.
        pub fn poolFor(
            &self,
            tokenA: alloy::sol_types::private::Address,
            tokenB: alloy::sol_types::private::Address,
            stable: bool,
            _factory: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, poolForCall, N> {
            self.call_builder(
                &poolForCall {
                    tokenA,
                    tokenB,
                    stable,
                    _factory,
                },
            )
        }
        ///Creates a new call builder for the [`quoteAddLiquidity`] function.
        pub fn quoteAddLiquidity(
            &self,
            tokenA: alloy::sol_types::private::Address,
            tokenB: alloy::sol_types::private::Address,
            stable: bool,
            _factory: alloy::sol_types::private::Address,
            amountADesired: alloy::sol_types::private::primitives::aliases::U256,
            amountBDesired: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, quoteAddLiquidityCall, N> {
            self.call_builder(
                &quoteAddLiquidityCall {
                    tokenA,
                    tokenB,
                    stable,
                    _factory,
                    amountADesired,
                    amountBDesired,
                },
            )
        }
        ///Creates a new call builder for the [`quoteRemoveLiquidity`] function.
        pub fn quoteRemoveLiquidity(
            &self,
            tokenA: alloy::sol_types::private::Address,
            tokenB: alloy::sol_types::private::Address,
            stable: bool,
            _factory: alloy::sol_types::private::Address,
            liquidity: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, quoteRemoveLiquidityCall, N> {
            self.call_builder(
                &quoteRemoveLiquidityCall {
                    tokenA,
                    tokenB,
                    stable,
                    _factory,
                    liquidity,
                },
            )
        }
        ///Creates a new call builder for the [`quoteStableLiquidityRatio`] function.
        pub fn quoteStableLiquidityRatio(
            &self,
            tokenA: alloy::sol_types::private::Address,
            tokenB: alloy::sol_types::private::Address,
            _factory: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, quoteStableLiquidityRatioCall, N> {
            self.call_builder(
                &quoteStableLiquidityRatioCall {
                    tokenA,
                    tokenB,
                    _factory,
                },
            )
        }
        ///Creates a new call builder for the [`removeLiquidity`] function.
        pub fn removeLiquidity(
            &self,
            tokenA: alloy::sol_types::private::Address,
            tokenB: alloy::sol_types::private::Address,
            stable: bool,
            liquidity: alloy::sol_types::private::primitives::aliases::U256,
            amountAMin: alloy::sol_types::private::primitives::aliases::U256,
            amountBMin: alloy::sol_types::private::primitives::aliases::U256,
            to: alloy::sol_types::private::Address,
            deadline: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, removeLiquidityCall, N> {
            self.call_builder(
                &removeLiquidityCall {
                    tokenA,
                    tokenB,
                    stable,
                    liquidity,
                    amountAMin,
                    amountBMin,
                    to,
                    deadline,
                },
            )
        }
        ///Creates a new call builder for the [`removeLiquidityETH`] function.
        pub fn removeLiquidityETH(
            &self,
            token: alloy::sol_types::private::Address,
            stable: bool,
            liquidity: alloy::sol_types::private::primitives::aliases::U256,
            amountTokenMin: alloy::sol_types::private::primitives::aliases::U256,
            amountETHMin: alloy::sol_types::private::primitives::aliases::U256,
            to: alloy::sol_types::private::Address,
            deadline: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, removeLiquidityETHCall, N> {
            self.call_builder(
                &removeLiquidityETHCall {
                    token,
                    stable,
                    liquidity,
                    amountTokenMin,
                    amountETHMin,
                    to,
                    deadline,
                },
            )
        }
        ///Creates a new call builder for the [`removeLiquidityETHSupportingFeeOnTransferTokens`] function.
        pub fn removeLiquidityETHSupportingFeeOnTransferTokens(
            &self,
            token: alloy::sol_types::private::Address,
            stable: bool,
            liquidity: alloy::sol_types::private::primitives::aliases::U256,
            amountTokenMin: alloy::sol_types::private::primitives::aliases::U256,
            amountETHMin: alloy::sol_types::private::primitives::aliases::U256,
            to: alloy::sol_types::private::Address,
            deadline: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            removeLiquidityETHSupportingFeeOnTransferTokensCall,
            N,
        > {
            self.call_builder(
                &removeLiquidityETHSupportingFeeOnTransferTokensCall {
                    token,
                    stable,
                    liquidity,
                    amountTokenMin,
                    amountETHMin,
                    to,
                    deadline,
                },
            )
        }
        ///Creates a new call builder for the [`sortTokens`] function.
        pub fn sortTokens(
            &self,
            tokenA: alloy::sol_types::private::Address,
            tokenB: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, sortTokensCall, N> {
            self.call_builder(&sortTokensCall { tokenA, tokenB })
        }
        ///Creates a new call builder for the [`swapExactETHForTokens`] function.
        pub fn swapExactETHForTokens(
            &self,
            amountOutMin: alloy::sol_types::private::primitives::aliases::U256,
            routes: alloy::sol_types::private::Vec<
                <IRouter::Route as alloy::sol_types::SolType>::RustType,
            >,
            to: alloy::sol_types::private::Address,
            deadline: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, swapExactETHForTokensCall, N> {
            self.call_builder(
                &swapExactETHForTokensCall {
                    amountOutMin,
                    routes,
                    to,
                    deadline,
                },
            )
        }
        ///Creates a new call builder for the [`swapExactETHForTokensSupportingFeeOnTransferTokens`] function.
        pub fn swapExactETHForTokensSupportingFeeOnTransferTokens(
            &self,
            amountOutMin: alloy::sol_types::private::primitives::aliases::U256,
            routes: alloy::sol_types::private::Vec<
                <IRouter::Route as alloy::sol_types::SolType>::RustType,
            >,
            to: alloy::sol_types::private::Address,
            deadline: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            swapExactETHForTokensSupportingFeeOnTransferTokensCall,
            N,
        > {
            self.call_builder(
                &swapExactETHForTokensSupportingFeeOnTransferTokensCall {
                    amountOutMin,
                    routes,
                    to,
                    deadline,
                },
            )
        }
        ///Creates a new call builder for the [`swapExactTokensForETH`] function.
        pub fn swapExactTokensForETH(
            &self,
            amountIn: alloy::sol_types::private::primitives::aliases::U256,
            amountOutMin: alloy::sol_types::private::primitives::aliases::U256,
            routes: alloy::sol_types::private::Vec<
                <IRouter::Route as alloy::sol_types::SolType>::RustType,
            >,
            to: alloy::sol_types::private::Address,
            deadline: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, swapExactTokensForETHCall, N> {
            self.call_builder(
                &swapExactTokensForETHCall {
                    amountIn,
                    amountOutMin,
                    routes,
                    to,
                    deadline,
                },
            )
        }
        ///Creates a new call builder for the [`swapExactTokensForETHSupportingFeeOnTransferTokens`] function.
        pub fn swapExactTokensForETHSupportingFeeOnTransferTokens(
            &self,
            amountIn: alloy::sol_types::private::primitives::aliases::U256,
            amountOutMin: alloy::sol_types::private::primitives::aliases::U256,
            routes: alloy::sol_types::private::Vec<
                <IRouter::Route as alloy::sol_types::SolType>::RustType,
            >,
            to: alloy::sol_types::private::Address,
            deadline: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            swapExactTokensForETHSupportingFeeOnTransferTokensCall,
            N,
        > {
            self.call_builder(
                &swapExactTokensForETHSupportingFeeOnTransferTokensCall {
                    amountIn,
                    amountOutMin,
                    routes,
                    to,
                    deadline,
                },
            )
        }
        ///Creates a new call builder for the [`swapExactTokensForTokens`] function.
        pub fn swapExactTokensForTokens(
            &self,
            amountIn: alloy::sol_types::private::primitives::aliases::U256,
            amountOutMin: alloy::sol_types::private::primitives::aliases::U256,
            routes: alloy::sol_types::private::Vec<
                <IRouter::Route as alloy::sol_types::SolType>::RustType,
            >,
            to: alloy::sol_types::private::Address,
            deadline: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, swapExactTokensForTokensCall, N> {
            self.call_builder(
                &swapExactTokensForTokensCall {
                    amountIn,
                    amountOutMin,
                    routes,
                    to,
                    deadline,
                },
            )
        }
        ///Creates a new call builder for the [`swapExactTokensForTokensSupportingFeeOnTransferTokens`] function.
        pub fn swapExactTokensForTokensSupportingFeeOnTransferTokens(
            &self,
            amountIn: alloy::sol_types::private::primitives::aliases::U256,
            amountOutMin: alloy::sol_types::private::primitives::aliases::U256,
            routes: alloy::sol_types::private::Vec<
                <IRouter::Route as alloy::sol_types::SolType>::RustType,
            >,
            to: alloy::sol_types::private::Address,
            deadline: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            swapExactTokensForTokensSupportingFeeOnTransferTokensCall,
            N,
        > {
            self.call_builder(
                &swapExactTokensForTokensSupportingFeeOnTransferTokensCall {
                    amountIn,
                    amountOutMin,
                    routes,
                    to,
                    deadline,
                },
            )
        }
        ///Creates a new call builder for the [`voter`] function.
        pub fn voter(&self) -> alloy_contract::SolCallBuilder<T, &P, voterCall, N> {
            self.call_builder(&voterCall {})
        }
        ///Creates a new call builder for the [`weth`] function.
        pub fn weth(&self) -> alloy_contract::SolCallBuilder<T, &P, wethCall, N> {
            self.call_builder(&wethCall {})
        }
        ///Creates a new call builder for the [`zapIn`] function.
        pub fn zapIn(
            &self,
            tokenIn: alloy::sol_types::private::Address,
            amountInA: alloy::sol_types::private::primitives::aliases::U256,
            amountInB: alloy::sol_types::private::primitives::aliases::U256,
            zapInPool: <IRouter::Zap as alloy::sol_types::SolType>::RustType,
            routesA: alloy::sol_types::private::Vec<
                <IRouter::Route as alloy::sol_types::SolType>::RustType,
            >,
            routesB: alloy::sol_types::private::Vec<
                <IRouter::Route as alloy::sol_types::SolType>::RustType,
            >,
            to: alloy::sol_types::private::Address,
            stake: bool,
        ) -> alloy_contract::SolCallBuilder<T, &P, zapInCall, N> {
            self.call_builder(
                &zapInCall {
                    tokenIn,
                    amountInA,
                    amountInB,
                    zapInPool,
                    routesA,
                    routesB,
                    to,
                    stake,
                },
            )
        }
        ///Creates a new call builder for the [`zapOut`] function.
        pub fn zapOut(
            &self,
            tokenOut: alloy::sol_types::private::Address,
            liquidity: alloy::sol_types::private::primitives::aliases::U256,
            zapOutPool: <IRouter::Zap as alloy::sol_types::SolType>::RustType,
            routesA: alloy::sol_types::private::Vec<
                <IRouter::Route as alloy::sol_types::SolType>::RustType,
            >,
            routesB: alloy::sol_types::private::Vec<
                <IRouter::Route as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, zapOutCall, N> {
            self.call_builder(
                &zapOutCall {
                    tokenOut,
                    liquidity,
                    zapOutPool,
                    routesA,
                    routesB,
                },
            )
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IAerodromeRouterInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
