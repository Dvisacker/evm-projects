pub use uniswap_v2_router_02::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod uniswap_v2_router_02 {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_factory"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_WETH"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("WETH"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("WETH"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("addLiquidity"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("addLiquidity"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenA"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenB"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountADesired"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountBDesired"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountAMin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountBMin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("deadline"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountA"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountB"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("liquidity"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("addLiquidityETH"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("addLiquidityETH"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "amountTokenDesired",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountTokenMin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountETHMin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("deadline"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountETH"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("liquidity"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("factory"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("factory"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getAmountIn"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getAmountIn"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountOut"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("reserveIn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("reserveOut"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountIn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getAmountOut"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getAmountOut"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountIn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("reserveIn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("reserveOut"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountOut"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getAmountsIn"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getAmountsIn"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountOut"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("path"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amounts"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getAmountsOut"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getAmountsOut"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountIn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("path"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amounts"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("quote"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("quote"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountA"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("reserveA"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("reserveB"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountB"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("removeLiquidity"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("removeLiquidity"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenA"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenB"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("liquidity"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountAMin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountBMin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("deadline"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountA"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountB"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("removeLiquidityETH"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("removeLiquidityETH"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("liquidity"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountTokenMin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountETHMin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("deadline"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountETH"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "removeLiquidityETHSupportingFeeOnTransferTokens",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "removeLiquidityETHSupportingFeeOnTransferTokens",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("liquidity"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountTokenMin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountETHMin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("deadline"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountETH"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("removeLiquidityETHWithPermit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "removeLiquidityETHWithPermit",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("liquidity"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountTokenMin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountETHMin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("deadline"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("approveMax"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("v"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("r"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("s"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountETH"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "removeLiquidityETHWithPermitSupportingFeeOnTransferTokens",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "removeLiquidityETHWithPermitSupportingFeeOnTransferTokens",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("liquidity"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountTokenMin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountETHMin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("deadline"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("approveMax"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("v"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("r"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("s"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountETH"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("removeLiquidityWithPermit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "removeLiquidityWithPermit",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenA"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenB"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("liquidity"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountAMin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountBMin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("deadline"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("approveMax"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("v"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("r"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("s"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountA"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountB"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("swapETHForExactTokens"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "swapETHForExactTokens",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountOut"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("path"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("deadline"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amounts"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("swapExactETHForTokens"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "swapExactETHForTokens",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountOutMin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("path"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("deadline"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amounts"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "swapExactETHForTokensSupportingFeeOnTransferTokens",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "swapExactETHForTokensSupportingFeeOnTransferTokens",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountOutMin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("path"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("deadline"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("swapExactTokensForETH"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "swapExactTokensForETH",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountIn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountOutMin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("path"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("deadline"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amounts"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "swapExactTokensForETHSupportingFeeOnTransferTokens",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "swapExactTokensForETHSupportingFeeOnTransferTokens",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountIn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountOutMin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("path"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("deadline"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("swapExactTokensForTokens"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "swapExactTokensForTokens",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountIn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountOutMin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("path"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("deadline"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amounts"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "swapExactTokensForTokensSupportingFeeOnTransferTokens",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "swapExactTokensForTokensSupportingFeeOnTransferTokens",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountIn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountOutMin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("path"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("deadline"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("swapTokensForExactETH"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "swapTokensForExactETH",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountOut"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountInMax"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("path"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("deadline"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amounts"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("swapTokensForExactTokens"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "swapTokensForExactTokens",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountOut"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountInMax"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("path"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("deadline"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amounts"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: true,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static UNISWAPV2ROUTER02_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xC0`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qb\0Gh8\x03\x80b\0Gh\x839\x81\x81\x01`@R`@\x81\x10\x15a\x005W`\0\x80\xFD[P\x80Q` \x90\x91\x01Q`\x01`\x01``\x1B\x03\x19``\x92\x83\x1B\x81\x16`\x80R\x91\x1B\x16`\xA0R`\x80Q``\x1C`\xA0Q``\x1CaE\xE3b\0\x01\x85`\09\x80a\x01_R\x80a\x0C\xE4R\x80a\r\x1FR\x80a\x0E\x16R\x80a\x104R\x80a\x13\xBER\x80a\x15$R\x80a\x18\xEBR\x80a\x19\xE5R\x80a\x1A\x9BR\x80a\x1BiR\x80a\x1C\xAFR\x80a\x1D7R\x80a\x1F|R\x80a\x1F\xF7R\x80a \xA6R\x80a!rR\x80a\"\x07R\x80a\"{R\x80a'yR\x80a)\xECR\x80a*BR\x80a*vR\x80a*\xEAR\x80a,\x8AR\x80a-\xCDR\x80a.URP\x80a\x0E\xA4R\x80a\x0F{R\x80a\x10\xFAR\x80a\x113R\x80a\x12nR\x80a\x14LR\x80a\x15\x02R\x80a\x16rR\x80a\x1B\xFCR\x80a\x1DiR\x80a\x1E\xCCR\x80a\"\xADR\x80a%\x06R\x80a&\xFER\x80a''R\x80a'WR\x80a(\xC4R\x80a* R\x80a-\x1DR\x80a.\x87R\x80a7.R\x80a7qR\x80a:TR\x80a;\xD3R\x80a@\x03R\x80a@\xB1R\x80aA1RPaE\xE3`\0\xF3\xFE`\x80`@R`\x046\x10a\x01OW`\x005`\xE0\x1C\x80c\x88\x03\xDB\xEE\x11a\0\xB6W\x80c\xC4Z\x01U\x11a\0oW\x80c\xC4Z\x01U\x14a\n\x10W\x80c\xD0l\xA6\x1F\x14a\n%W\x80c\xDE\xD98*\x14a\n\xDAW\x80c\xE8\xE37\0\x14a\x0BMW\x80c\xF3\x05\xD7\x19\x14a\x0B\xCDW\x80c\xFB;\xDBA\x14a\x0C\x13Wa\x01\x88V[\x80c\x88\x03\xDB\xEE\x14a\x07\xDFW\x80c\xAD\\FH\x14a\x08uW\x80c\xADa]\xEC\x14a\x08\xA6W\x80c\xAF)y\xEB\x14a\x08\xDCW\x80c\xB6\xF9\xDE\x95\x14a\t/W\x80c\xBA\xA2\xAB\xDE\x14a\t\xB3Wa\x01\x88V[\x80cJ%\xD9J\x11a\x01\x08W\x80cJ%\xD9J\x14a\x04\xF0W\x80c[\rY\x84\x14a\x05\x86W\x80c\\\x11\xD7\x95\x14a\x05\xF9W\x80cy\x1A\xC9G\x14a\x06\x8FW\x80c\x7F\xF3j\xB5\x14a\x07%W\x80c\x85\xF8\xC2Y\x14a\x07\xA9Wa\x01\x88V[\x80c\x02u\x1C\xEC\x14a\x01\x8DW\x80c\x05MP\xD4\x14a\x01\xF9W\x80c\x18\xCB\xAF\xE5\x14a\x02AW\x80c\x1F\0\xCAt\x14a\x03'W\x80c!\x95\x99\\\x14a\x03\xDCW\x80c8\xED\x179\x14a\x04ZWa\x01\x88V[6a\x01\x88W3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x01\x86W\xFE[\0[`\0\x80\xFD[4\x80\x15a\x01\x99W`\0\x80\xFD[Pa\x01\xE0`\x04\x806\x03`\xC0\x81\x10\x15a\x01\xB0W`\0\x80\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x81\x16\x91` \x81\x015\x91`@\x82\x015\x91``\x81\x015\x91`\x80\x82\x015\x16\x90`\xA0\x015a\x0C\x97V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x80Q\x91\x82\x90\x03\x01\x90\xF3[4\x80\x15a\x02\x05W`\0\x80\xFD[Pa\x02/`\x04\x806\x03``\x81\x10\x15a\x02\x1CW`\0\x80\xFD[P\x805\x90` \x81\x015\x90`@\x015a\r\xB1V[`@\x80Q\x91\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[4\x80\x15a\x02MW`\0\x80\xFD[Pa\x02\xD7`\x04\x806\x03`\xA0\x81\x10\x15a\x02dW`\0\x80\xFD[\x815\x91` \x81\x015\x91\x81\x01\x90``\x81\x01`@\x82\x015`\x01` \x1B\x81\x11\x15a\x02\x8AW`\0\x80\xFD[\x82\x01\x83` \x82\x01\x11\x15a\x02\x9CW`\0\x80\xFD[\x805\x90` \x01\x91\x84` \x83\x02\x84\x01\x11`\x01` \x1B\x83\x11\x17\x15a\x02\xBDW`\0\x80\xFD[\x91\x93P\x91P`\x01`\x01`\xA0\x1B\x03\x815\x16\x90` \x015a\r\xC6V[`@\x80Q` \x80\x82R\x83Q\x81\x83\x01R\x83Q\x91\x92\x83\x92\x90\x83\x01\x91\x85\x81\x01\x91\x02\x80\x83\x83`\0[\x83\x81\x10\x15a\x03\x13W\x81\x81\x01Q\x83\x82\x01R` \x01a\x02\xFBV[PPPP\x90P\x01\x92PPP`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x033W`\0\x80\xFD[Pa\x02\xD7`\x04\x806\x03`@\x81\x10\x15a\x03JW`\0\x80\xFD[\x815\x91\x90\x81\x01\x90`@\x81\x01` \x82\x015`\x01` \x1B\x81\x11\x15a\x03kW`\0\x80\xFD[\x82\x01\x83` \x82\x01\x11\x15a\x03}W`\0\x80\xFD[\x805\x90` \x01\x91\x84` \x83\x02\x84\x01\x11`\x01` \x1B\x83\x11\x17\x15a\x03\x9EW`\0\x80\xFD[\x91\x90\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x92\x95Pa\x10\xF3\x94PPPPPV[4\x80\x15a\x03\xE8W`\0\x80\xFD[Pa\x01\xE0`\x04\x806\x03a\x01`\x81\x10\x15a\x04\0W`\0\x80\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x81\x16\x91` \x81\x015\x82\x16\x91`@\x82\x015\x91``\x81\x015\x91`\x80\x82\x015\x91`\xA0\x81\x015\x90\x91\x16\x90`\xC0\x81\x015\x90`\xE0\x81\x015\x15\x15\x90`\xFFa\x01\0\x82\x015\x16\x90a\x01 \x81\x015\x90a\x01@\x015a\x11)V[4\x80\x15a\x04fW`\0\x80\xFD[Pa\x02\xD7`\x04\x806\x03`\xA0\x81\x10\x15a\x04}W`\0\x80\xFD[\x815\x91` \x81\x015\x91\x81\x01\x90``\x81\x01`@\x82\x015`\x01` \x1B\x81\x11\x15a\x04\xA3W`\0\x80\xFD[\x82\x01\x83` \x82\x01\x11\x15a\x04\xB5W`\0\x80\xFD[\x805\x90` \x01\x91\x84` \x83\x02\x84\x01\x11`\x01` \x1B\x83\x11\x17\x15a\x04\xD6W`\0\x80\xFD[\x91\x93P\x91P`\x01`\x01`\xA0\x1B\x03\x815\x16\x90` \x015a\x12#V[4\x80\x15a\x04\xFCW`\0\x80\xFD[Pa\x02\xD7`\x04\x806\x03`\xA0\x81\x10\x15a\x05\x13W`\0\x80\xFD[\x815\x91` \x81\x015\x91\x81\x01\x90``\x81\x01`@\x82\x015`\x01` \x1B\x81\x11\x15a\x059W`\0\x80\xFD[\x82\x01\x83` \x82\x01\x11\x15a\x05KW`\0\x80\xFD[\x805\x90` \x01\x91\x84` \x83\x02\x84\x01\x11`\x01` \x1B\x83\x11\x17\x15a\x05lW`\0\x80\xFD[\x91\x93P\x91P`\x01`\x01`\xA0\x1B\x03\x815\x16\x90` \x015a\x13nV[4\x80\x15a\x05\x92W`\0\x80\xFD[Pa\x02/`\x04\x806\x03a\x01@\x81\x10\x15a\x05\xAAW`\0\x80\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x81\x16\x91` \x81\x015\x91`@\x82\x015\x91``\x81\x015\x91`\x80\x82\x015\x16\x90`\xA0\x81\x015\x90`\xC0\x81\x015\x15\x15\x90`\xFF`\xE0\x82\x015\x16\x90a\x01\0\x81\x015\x90a\x01 \x015a\x14\xFAV[4\x80\x15a\x06\x05W`\0\x80\xFD[Pa\x01\x86`\x04\x806\x03`\xA0\x81\x10\x15a\x06\x1CW`\0\x80\xFD[\x815\x91` \x81\x015\x91\x81\x01\x90``\x81\x01`@\x82\x015`\x01` \x1B\x81\x11\x15a\x06BW`\0\x80\xFD[\x82\x01\x83` \x82\x01\x11\x15a\x06TW`\0\x80\xFD[\x805\x90` \x01\x91\x84` \x83\x02\x84\x01\x11`\x01` \x1B\x83\x11\x17\x15a\x06uW`\0\x80\xFD[\x91\x93P\x91P`\x01`\x01`\xA0\x1B\x03\x815\x16\x90` \x015a\x16\x08V[4\x80\x15a\x06\x9BW`\0\x80\xFD[Pa\x01\x86`\x04\x806\x03`\xA0\x81\x10\x15a\x06\xB2W`\0\x80\xFD[\x815\x91` \x81\x015\x91\x81\x01\x90``\x81\x01`@\x82\x015`\x01` \x1B\x81\x11\x15a\x06\xD8W`\0\x80\xFD[\x82\x01\x83` \x82\x01\x11\x15a\x06\xEAW`\0\x80\xFD[\x805\x90` \x01\x91\x84` \x83\x02\x84\x01\x11`\x01` \x1B\x83\x11\x17\x15a\x07\x0BW`\0\x80\xFD[\x91\x93P\x91P`\x01`\x01`\xA0\x1B\x03\x815\x16\x90` \x015a\x18\x9DV[a\x02\xD7`\x04\x806\x03`\x80\x81\x10\x15a\x07;W`\0\x80\xFD[\x815\x91\x90\x81\x01\x90`@\x81\x01` \x82\x015`\x01` \x1B\x81\x11\x15a\x07\\W`\0\x80\xFD[\x82\x01\x83` \x82\x01\x11\x15a\x07nW`\0\x80\xFD[\x805\x90` \x01\x91\x84` \x83\x02\x84\x01\x11`\x01` \x1B\x83\x11\x17\x15a\x07\x8FW`\0\x80\xFD[\x91\x93P\x91P`\x01`\x01`\xA0\x1B\x03\x815\x16\x90` \x015a\x1B!V[4\x80\x15a\x07\xB5W`\0\x80\xFD[Pa\x02/`\x04\x806\x03``\x81\x10\x15a\x07\xCCW`\0\x80\xFD[P\x805\x90` \x81\x015\x90`@\x015a\x1EtV[4\x80\x15a\x07\xEBW`\0\x80\xFD[Pa\x02\xD7`\x04\x806\x03`\xA0\x81\x10\x15a\x08\x02W`\0\x80\xFD[\x815\x91` \x81\x015\x91\x81\x01\x90``\x81\x01`@\x82\x015`\x01` \x1B\x81\x11\x15a\x08(W`\0\x80\xFD[\x82\x01\x83` \x82\x01\x11\x15a\x08:W`\0\x80\xFD[\x805\x90` \x01\x91\x84` \x83\x02\x84\x01\x11`\x01` \x1B\x83\x11\x17\x15a\x08[W`\0\x80\xFD[\x91\x93P\x91P`\x01`\x01`\xA0\x1B\x03\x815\x16\x90` \x015a\x1E\x81V[4\x80\x15a\x08\x81W`\0\x80\xFD[Pa\x08\x8Aa\x1FzV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[4\x80\x15a\x08\xB2W`\0\x80\xFD[Pa\x02/`\x04\x806\x03``\x81\x10\x15a\x08\xC9W`\0\x80\xFD[P\x805\x90` \x81\x015\x90`@\x015a\x1F\x9EV[4\x80\x15a\x08\xE8W`\0\x80\xFD[Pa\x02/`\x04\x806\x03`\xC0\x81\x10\x15a\x08\xFFW`\0\x80\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x81\x16\x91` \x81\x015\x91`@\x82\x015\x91``\x81\x015\x91`\x80\x82\x015\x16\x90`\xA0\x015a\x1F\xABV[a\x01\x86`\x04\x806\x03`\x80\x81\x10\x15a\tEW`\0\x80\xFD[\x815\x91\x90\x81\x01\x90`@\x81\x01` \x82\x015`\x01` \x1B\x81\x11\x15a\tfW`\0\x80\xFD[\x82\x01\x83` \x82\x01\x11\x15a\txW`\0\x80\xFD[\x805\x90` \x01\x91\x84` \x83\x02\x84\x01\x11`\x01` \x1B\x83\x11\x17\x15a\t\x99W`\0\x80\xFD[\x91\x93P\x91P`\x01`\x01`\xA0\x1B\x03\x815\x16\x90` \x015a!,V[4\x80\x15a\t\xBFW`\0\x80\xFD[Pa\x01\xE0`\x04\x806\x03`\xE0\x81\x10\x15a\t\xD6W`\0\x80\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x81\x16\x91` \x81\x015\x82\x16\x91`@\x82\x015\x91``\x81\x015\x91`\x80\x82\x015\x91`\xA0\x81\x015\x90\x91\x16\x90`\xC0\x015a$\xB8V[4\x80\x15a\n\x1CW`\0\x80\xFD[Pa\x08\x8Aa&\xFCV[4\x80\x15a\n1W`\0\x80\xFD[Pa\x02\xD7`\x04\x806\x03`@\x81\x10\x15a\nHW`\0\x80\xFD[\x815\x91\x90\x81\x01\x90`@\x81\x01` \x82\x015`\x01` \x1B\x81\x11\x15a\niW`\0\x80\xFD[\x82\x01\x83` \x82\x01\x11\x15a\n{W`\0\x80\xFD[\x805\x90` \x01\x91\x84` \x83\x02\x84\x01\x11`\x01` \x1B\x83\x11\x17\x15a\n\x9CW`\0\x80\xFD[\x91\x90\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x92\x95Pa' \x94PPPPPV[4\x80\x15a\n\xE6W`\0\x80\xFD[Pa\x01\xE0`\x04\x806\x03a\x01@\x81\x10\x15a\n\xFEW`\0\x80\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x81\x16\x91` \x81\x015\x91`@\x82\x015\x91``\x81\x015\x91`\x80\x82\x015\x16\x90`\xA0\x81\x015\x90`\xC0\x81\x015\x15\x15\x90`\xFF`\xE0\x82\x015\x16\x90a\x01\0\x81\x015\x90a\x01 \x015a'MV[4\x80\x15a\x0BYW`\0\x80\xFD[Pa\x0B\xAF`\x04\x806\x03a\x01\0\x81\x10\x15a\x0BqW`\0\x80\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x81\x16\x91` \x81\x015\x82\x16\x91`@\x82\x015\x91``\x81\x015\x91`\x80\x82\x015\x91`\xA0\x81\x015\x91`\xC0\x82\x015\x16\x90`\xE0\x015a(aV[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x82\x82\x01RQ\x90\x81\x90\x03``\x01\x90\xF3[a\x0B\xAF`\x04\x806\x03`\xC0\x81\x10\x15a\x0B\xE3W`\0\x80\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x81\x16\x91` \x81\x015\x91`@\x82\x015\x91``\x81\x015\x91`\x80\x82\x015\x16\x90`\xA0\x015a)\x9DV[a\x02\xD7`\x04\x806\x03`\x80\x81\x10\x15a\x0C)W`\0\x80\xFD[\x815\x91\x90\x81\x01\x90`@\x81\x01` \x82\x015`\x01` \x1B\x81\x11\x15a\x0CJW`\0\x80\xFD[\x82\x01\x83` \x82\x01\x11\x15a\x0C\\W`\0\x80\xFD[\x805\x90` \x01\x91\x84` \x83\x02\x84\x01\x11`\x01` \x1B\x83\x11\x17\x15a\x0C}W`\0\x80\xFD[\x91\x93P\x91P`\x01`\x01`\xA0\x1B\x03\x815\x16\x90` \x015a,BV[`\0\x80\x82B\x81\x10\x15a\x0C\xDEW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R`\0\x80Q` aE\x8E\x839\x81Q\x91R`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[a\r\r\x89\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x8A\x8A\x8A0\x8Aa$\xB8V[\x90\x93P\x91Pa\r\x1D\x89\x86\x85a/\xC4V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c.\x1A}M\x83`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x82\x81R` \x01\x91PP`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\r\x83W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\r\x97W=`\0\x80>=`\0\xFD[PPPPa\r\xA5\x85\x83a1.V[P\x96P\x96\x94PPPPPV[`\0a\r\xBE\x84\x84\x84a2&V[\x94\x93PPPPV[``\x81B\x81\x10\x15a\x0E\x0CW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R`\0\x80Q` aE\x8E\x839\x81Q\x91R`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x86\x86`\0\x19\x81\x01\x81\x81\x10a\x0EFW\xFE[\x90P` \x02\x015`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0E\x9FW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R`\0\x80Q` aD\xAB\x839\x81Q\x91R`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[a\x0E\xFD\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x89\x88\x88\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa3\x16\x92PPPV[\x91P\x86\x82`\x01\x84Q\x03\x81Q\x81\x10a\x0F\x10W\xFE[` \x02` \x01\x01Q\x10\x15a\x0FUW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`+\x81R` \x01\x80aE\x14`+\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[a\x0F\xF3\x86\x86`\0\x81\x81\x10a\x0FeW\xFE[\x90P` \x02\x015`\x01`\x01`\xA0\x1B\x03\x163a\x0F\xD9\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x8A\x8A`\0\x81\x81\x10a\x0F\xA7W\xFE[\x90P` \x02\x015`\x01`\x01`\xA0\x1B\x03\x16\x8B\x8B`\x01\x81\x81\x10a\x0F\xC4W\xFE[\x90P` \x02\x015`\x01`\x01`\xA0\x1B\x03\x16a4bV[\x85`\0\x81Q\x81\x10a\x0F\xE6W\xFE[` \x02` \x01\x01Qa5\"V[a\x102\x82\x87\x87\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RP0\x92Pa6\x7F\x91PPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c.\x1A}M\x83`\x01\x85Q\x03\x81Q\x81\x10a\x10qW\xFE[` \x02` \x01\x01Q`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x82\x81R` \x01\x91PP`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x10\xAFW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x10\xC3W=`\0\x80>=`\0\xFD[PPPPa\x10\xE8\x84\x83`\x01\x85Q\x03\x81Q\x81\x10a\x10\xDBW\xFE[` \x02` \x01\x01Qa1.V[P\x96\x95PPPPPPV[``a\x11 \x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84\x84a8\xC5V[\x90P[\x92\x91PPV[`\0\x80`\0a\x11Y\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x8F\x8Fa4bV[\x90P`\0\x87a\x11hW\x8Ca\x11lV[`\0\x19[`@\x80Qc\xD5\x05\xAC\xCF`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x83\x90R`d\x81\x01\x8C\x90R`\xFF\x8A\x16`\x84\x82\x01R`\xA4\x81\x01\x89\x90R`\xC4\x81\x01\x88\x90R\x90Q\x91\x92P`\x01`\x01`\xA0\x1B\x03\x84\x16\x91c\xD5\x05\xAC\xCF\x91`\xE4\x80\x82\x01\x92`\0\x92\x90\x91\x90\x82\x90\x03\x01\x81\x83\x87\x80;\x15\x80\x15a\x11\xE2W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x11\xF6W=`\0\x80>=`\0\xFD[PPPPa\x12\t\x8F\x8F\x8F\x8F\x8F\x8F\x8Fa$\xB8V[\x80\x94P\x81\x95PPPPP\x9BP\x9B\x99PPPPPPPPPPV[``\x81B\x81\x10\x15a\x12iW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R`\0\x80Q` aE\x8E\x839\x81Q\x91R`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[a\x12\xC7\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x89\x88\x88\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa3\x16\x92PPPV[\x91P\x86\x82`\x01\x84Q\x03\x81Q\x81\x10a\x12\xDAW\xFE[` \x02` \x01\x01Q\x10\x15a\x13\x1FW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`+\x81R` \x01\x80aE\x14`+\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[a\x13/\x86\x86`\0\x81\x81\x10a\x0FeW\xFE[a\x10\xE8\x82\x87\x87\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x89\x92Pa6\x7F\x91PPV[``\x81B\x81\x10\x15a\x13\xB4W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R`\0\x80Q` aE\x8E\x839\x81Q\x91R`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x86\x86`\0\x19\x81\x01\x81\x81\x10a\x13\xEEW\xFE[\x90P` \x02\x015`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x14a\x14GW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R`\0\x80Q` aD\xAB\x839\x81Q\x91R`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[a\x14\xA5\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x89\x88\x88\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa8\xC5\x92PPPV[\x91P\x86\x82`\0\x81Q\x81\x10a\x14\xB5W\xFE[` \x02` \x01\x01Q\x11\x15a\x0FUW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`'\x81R` \x01\x80aD\x84`'\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[`\0\x80a\x15H\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x8D\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a4bV[\x90P`\0\x86a\x15WW\x8Ba\x15[V[`\0\x19[`@\x80Qc\xD5\x05\xAC\xCF`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x83\x90R`d\x81\x01\x8B\x90R`\xFF\x89\x16`\x84\x82\x01R`\xA4\x81\x01\x88\x90R`\xC4\x81\x01\x87\x90R\x90Q\x91\x92P`\x01`\x01`\xA0\x1B\x03\x84\x16\x91c\xD5\x05\xAC\xCF\x91`\xE4\x80\x82\x01\x92`\0\x92\x90\x91\x90\x82\x90\x03\x01\x81\x83\x87\x80;\x15\x80\x15a\x15\xD1W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x15\xE5W=`\0\x80>=`\0\xFD[PPPPa\x15\xF7\x8D\x8D\x8D\x8D\x8D\x8Da\x1F\xABV[\x9D\x9CPPPPPPPPPPPPPV[\x80B\x81\x10\x15a\x16LW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R`\0\x80Q` aE\x8E\x839\x81Q\x91R`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[a\x16\xC1\x85\x85`\0\x81\x81\x10a\x16\\W\xFE[\x90P` \x02\x015`\x01`\x01`\xA0\x1B\x03\x163a\x16\xBB\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x89\x89`\0\x81\x81\x10a\x16\x9EW\xFE[\x90P` \x02\x015`\x01`\x01`\xA0\x1B\x03\x16\x8A\x8A`\x01\x81\x81\x10a\x0F\xC4W\xFE[\x8Aa5\"V[`\0\x85\x85`\0\x19\x81\x01\x81\x81\x10a\x16\xD3W\xFE[\x90P` \x02\x015`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cp\xA0\x821\x85`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x82`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x91PP` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x178W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x17LW=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x17bW`\0\x80\xFD[PQ`@\x80Q` \x88\x81\x02\x82\x81\x01\x82\x01\x90\x93R\x88\x82R\x92\x93Pa\x17\xA4\x92\x90\x91\x89\x91\x89\x91\x82\x91\x85\x01\x90\x84\x90\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x88\x92Pa9\xFD\x91PPV[\x86a\x18V\x82\x88\x88`\0\x19\x81\x01\x81\x81\x10a\x17\xB9W\xFE[\x90P` \x02\x015`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cp\xA0\x821\x88`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x82`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x91PP` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x18\x1EW`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x182W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x18HW`\0\x80\xFD[PQ\x90c\xFF\xFF\xFF\xFFa=\x08\x16V[\x10\x15a\x18\x93W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`+\x81R` \x01\x80aE\x14`+\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[PPPPPPPPV[\x80B\x81\x10\x15a\x18\xE1W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R`\0\x80Q` aE\x8E\x839\x81Q\x91R`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x85\x85`\0\x19\x81\x01\x81\x81\x10a\x19\x1BW\xFE[\x90P` \x02\x015`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x14a\x19tW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R`\0\x80Q` aD\xAB\x839\x81Q\x91R`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[a\x19\x84\x85\x85`\0\x81\x81\x10a\x16\\W\xFE[a\x19\xC2\x85\x85\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RP0\x92Pa9\xFD\x91PPV[`@\x80Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90Q`\0\x91`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91cp\xA0\x821\x91`$\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\x1A,W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x1A@W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x1AVW`\0\x80\xFD[PQ\x90P\x86\x81\x10\x15a\x1A\x99W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`+\x81R` \x01\x80aE\x14`+\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c.\x1A}M\x82`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x82\x81R` \x01\x91PP`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1A\xFFW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1B\x13W=`\0\x80>=`\0\xFD[PPPPa\x18\x93\x84\x82a1.V[``\x81B\x81\x10\x15a\x1BgW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R`\0\x80Q` aE\x8E\x839\x81Q\x91R`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x86\x86`\0\x81\x81\x10a\x1B\x9EW\xFE[\x90P` \x02\x015`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1B\xF7W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R`\0\x80Q` aD\xAB\x839\x81Q\x91R`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[a\x1CU\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x004\x88\x88\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa3\x16\x92PPPV[\x91P\x86\x82`\x01\x84Q\x03\x81Q\x81\x10a\x1ChW\xFE[` \x02` \x01\x01Q\x10\x15a\x1C\xADW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`+\x81R` \x01\x80aE\x14`+\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xD0\xE3\r\xB0\x83`\0\x81Q\x81\x10a\x1C\xE9W\xFE[` \x02` \x01\x01Q`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\x1D\x1CW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1D0W=`\0\x80>=`\0\xFD[PPPPP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xA9\x05\x9C\xBBa\x1D\x95\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x89\x89`\0\x81\x81\x10a\x16\x9EW\xFE[\x84`\0\x81Q\x81\x10a\x1D\xA2W\xFE[` \x02` \x01\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x83`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x82\x81R` \x01\x92PPP` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1D\xF9W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1E\rW=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x1E#W`\0\x80\xFD[PQa\x1E+W\xFE[a\x1Ej\x82\x87\x87\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x89\x92Pa6\x7F\x91PPV[P\x95\x94PPPPPV[`\0a\r\xBE\x84\x84\x84a=XV[``\x81B\x81\x10\x15a\x1E\xC7W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R`\0\x80Q` aE\x8E\x839\x81Q\x91R`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[a\x1F%\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x89\x88\x88\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa8\xC5\x92PPPV[\x91P\x86\x82`\0\x81Q\x81\x10a\x1F5W\xFE[` \x02` \x01\x01Q\x11\x15a\x13\x1FW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`'\x81R` \x01\x80aD\x84`'\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0a\r\xBE\x84\x84\x84a>HV[`\0\x81B\x81\x10\x15a\x1F\xF1W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R`\0\x80Q` aE\x8E\x839\x81Q\x91R`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[a  \x88\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x89\x89\x890\x89a$\xB8V[`@\x80Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90Q\x91\x94Pa \xA4\x92P\x8A\x91\x87\x91`\x01`\x01`\xA0\x1B\x03\x84\x16\x91cp\xA0\x821\x91`$\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a sW`\0\x80\xFD[PZ\xFA\x15\x80\x15a \x87W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a \x9DW`\0\x80\xFD[PQa/\xC4V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c.\x1A}M\x83`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x82\x81R` \x01\x91PP`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a!\nW`\0\x80\xFD[PZ\xF1\x15\x80\x15a!\x1EW=`\0\x80>=`\0\xFD[PPPPa\x10\xE8\x84\x83a1.V[\x80B\x81\x10\x15a!pW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R`\0\x80Q` aE\x8E\x839\x81Q\x91R`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x85\x85`\0\x81\x81\x10a!\xA7W\xFE[\x90P` \x02\x015`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x14a\"\0W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R`\0\x80Q` aD\xAB\x839\x81Q\x91R`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\x004\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xD0\xE3\r\xB0\x82`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\"`W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\"tW=`\0\x80>=`\0\xFD[PPPPP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xA9\x05\x9C\xBBa\"\xD9\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x89\x89`\0\x81\x81\x10a\x16\x9EW\xFE[\x83`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x83`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x82\x81R` \x01\x92PPP` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a#)W`\0\x80\xFD[PZ\xF1\x15\x80\x15a#=W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a#SW`\0\x80\xFD[PQa#[W\xFE[`\0\x86\x86`\0\x19\x81\x01\x81\x81\x10a#mW\xFE[\x90P` \x02\x015`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cp\xA0\x821\x86`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x82`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x91PP` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a#\xD2W`\0\x80\xFD[PZ\xFA\x15\x80\x15a#\xE6W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a#\xFCW`\0\x80\xFD[PQ`@\x80Q` \x89\x81\x02\x82\x81\x01\x82\x01\x90\x93R\x89\x82R\x92\x93Pa$>\x92\x90\x91\x8A\x91\x8A\x91\x82\x91\x85\x01\x90\x84\x90\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x89\x92Pa9\xFD\x91PPV[\x87a\x18V\x82\x89\x89`\0\x19\x81\x01\x81\x81\x10a$SW\xFE[\x90P` \x02\x015`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cp\xA0\x821\x89`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x82`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x91PP` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x18\x1EW`\0\x80\xFD[`\0\x80\x82B\x81\x10\x15a$\xFFW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R`\0\x80Q` aE\x8E\x839\x81Q\x91R`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\0a%,\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x8C\x8Ca4bV[`@\x80Qc#\xB8r\xDD`\xE0\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16`$\x82\x01\x81\x90R`D\x82\x01\x8D\x90R\x91Q\x92\x93P\x90\x91c#\xB8r\xDD\x91`d\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81`\0\x87\x80;\x15\x80\x15a%\x87W`\0\x80\xFD[PZ\xF1\x15\x80\x15a%\x9BW=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a%\xB1W`\0\x80\xFD[PP`@\x80Qc\"k\xF2\xD1`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x88\x81\x16`\x04\x83\x01R\x82Q`\0\x93\x84\x93\x92\x86\x16\x92c\x89\xAF\xCBD\x92`$\x80\x83\x01\x93\x92\x82\x90\x03\x01\x81\x87\x87\x80;\x15\x80\x15a%\xFEW`\0\x80\xFD[PZ\xF1\x15\x80\x15a&\x12W=`\0\x80>=`\0\xFD[PPPP`@Q=`@\x81\x10\x15a&(W`\0\x80\xFD[P\x80Q` \x90\x91\x01Q\x90\x92P\x90P`\0a&B\x8E\x8Ea>\xF4V[P\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16\x8E`\x01`\x01`\xA0\x1B\x03\x16\x14a&eW\x81\x83a&hV[\x82\x82[\x90\x97P\x95P\x8A\x87\x10\x15a&\xACW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`&\x81R` \x01\x80aD\xCB`&\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[\x89\x86\x10\x15a&\xEBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`&\x81R` \x01\x80aD\x11`&\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[PPPPP\x97P\x97\x95PPPPPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[``a\x11 \x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84\x84a3\x16V[`\0\x80`\0a'\x9D\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x8E\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a4bV[\x90P`\0\x87a'\xACW\x8Ca'\xB0V[`\0\x19[`@\x80Qc\xD5\x05\xAC\xCF`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x83\x90R`d\x81\x01\x8C\x90R`\xFF\x8A\x16`\x84\x82\x01R`\xA4\x81\x01\x89\x90R`\xC4\x81\x01\x88\x90R\x90Q\x91\x92P`\x01`\x01`\xA0\x1B\x03\x84\x16\x91c\xD5\x05\xAC\xCF\x91`\xE4\x80\x82\x01\x92`\0\x92\x90\x91\x90\x82\x90\x03\x01\x81\x83\x87\x80;\x15\x80\x15a(&W`\0\x80\xFD[PZ\xF1\x15\x80\x15a(:W=`\0\x80>=`\0\xFD[PPPPa(L\x8E\x8E\x8E\x8E\x8E\x8Ea\x0C\x97V[\x90\x9F\x90\x9EP\x9CPPPPPPPPPPPPPV[`\0\x80`\0\x83B\x81\x10\x15a(\xAAW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R`\0\x80Q` aE\x8E\x839\x81Q\x91R`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[a(\xB8\x8C\x8C\x8C\x8C\x8C\x8Ca?\xD2V[\x90\x94P\x92P`\0a(\xEA\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x8E\x8Ea4bV[\x90Pa(\xF8\x8D3\x83\x88a5\"V[a)\x04\x8C3\x83\x87a5\"V[\x80`\x01`\x01`\xA0\x1B\x03\x16cjbxB\x88`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x82`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x91PP` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a)\\W`\0\x80\xFD[PZ\xF1\x15\x80\x15a)pW=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a)\x86W`\0\x80\xFD[PQ\x94\x9D\x93\x9CP\x93\x9AP\x91\x98PPPPPPPPPV[`\0\x80`\0\x83B\x81\x10\x15a)\xE6W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R`\0\x80Q` aE\x8E\x839\x81Q\x91R`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[a*\x14\x8A\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x8B4\x8C\x8Ca?\xD2V[\x90\x94P\x92P`\0a*f\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x8C\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a4bV[\x90Pa*t\x8B3\x83\x88a5\"V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xD0\xE3\r\xB0\x85`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a*\xCFW`\0\x80\xFD[PZ\xF1\x15\x80\x15a*\xE3W=`\0\x80>=`\0\xFD[PPPPP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xA9\x05\x9C\xBB\x82\x86`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x83`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x82\x81R` \x01\x92PPP` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a+hW`\0\x80\xFD[PZ\xF1\x15\x80\x15a+|W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a+\x92W`\0\x80\xFD[PQa+\x9AW\xFE[\x80`\x01`\x01`\xA0\x1B\x03\x16cjbxB\x88`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x82`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x91PP` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a+\xF2W`\0\x80\xFD[PZ\xF1\x15\x80\x15a,\x06W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a,\x1CW`\0\x80\xFD[PQ\x92P4\x84\x10\x15a,4Wa,43\x854\x03a1.V[PP\x96P\x96P\x96\x93PPPPV[``\x81B\x81\x10\x15a,\x88W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R`\0\x80Q` aE\x8E\x839\x81Q\x91R`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x86\x86`\0\x81\x81\x10a,\xBFW\xFE[\x90P` \x02\x015`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x14a-\x18W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R`\0\x80Q` aD\xAB\x839\x81Q\x91R`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[a-v\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x88\x88\x88\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa8\xC5\x92PPPV[\x91P4\x82`\0\x81Q\x81\x10a-\x86W\xFE[` \x02` \x01\x01Q\x11\x15a-\xCBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`'\x81R` \x01\x80aD\x84`'\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xD0\xE3\r\xB0\x83`\0\x81Q\x81\x10a.\x07W\xFE[` \x02` \x01\x01Q`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a.:W`\0\x80\xFD[PZ\xF1\x15\x80\x15a.NW=`\0\x80>=`\0\xFD[PPPPP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xA9\x05\x9C\xBBa.\xB3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x89\x89`\0\x81\x81\x10a\x16\x9EW\xFE[\x84`\0\x81Q\x81\x10a.\xC0W\xFE[` \x02` \x01\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x83`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x82\x81R` \x01\x92PPP` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a/\x17W`\0\x80\xFD[PZ\xF1\x15\x80\x15a/+W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a/AW`\0\x80\xFD[PQa/IW\xFE[a/\x88\x82\x87\x87\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x89\x92Pa6\x7F\x91PPV[\x81`\0\x81Q\x81\x10a/\x95W\xFE[` \x02` \x01\x01Q4\x11\x15a\x1EjWa\x1Ej3\x83`\0\x81Q\x81\x10a/\xB5W\xFE[` \x02` \x01\x01Q4\x03a1.V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`$\x83\x01R`D\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`d\x90\x92\x01\x83R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\xA9\x05\x9C\xBB`\xE0\x1B\x17\x81R\x92Q\x82Q`\0\x94``\x94\x93\x89\x16\x93\x92\x91\x82\x91\x90\x80\x83\x83[` \x83\x10a0AW\x80Q\x82R`\x1F\x19\x90\x92\x01\x91` \x91\x82\x01\x91\x01a0\"V[`\x01\x83` \x03a\x01\0\n\x03\x80\x19\x82Q\x16\x81\x84Q\x16\x80\x82\x17\x85RPPPPPP\x90P\x01\x91PP`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a0\xA3W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a0\xA8V[``\x91P[P\x91P\x91P\x81\x80\x15a0\xD6WP\x80Q\x15\x80a0\xD6WP\x80\x80` \x01\x90Q` \x81\x10\x15a0\xD3W`\0\x80\xFD[PQ[a1'W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FTransferHelper: TRANSFER_FAILED\0`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[PPPPPV[`@\x80Q`\0\x80\x82R` \x82\x01\x90\x92R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x83\x90`@Q\x80\x82\x80Q\x90` \x01\x90\x80\x83\x83[` \x83\x10a1zW\x80Q\x82R`\x1F\x19\x90\x92\x01\x91` \x91\x82\x01\x91\x01a1[V[`\x01\x83` \x03a\x01\0\n\x03\x80\x19\x82Q\x16\x81\x84Q\x16\x80\x82\x17\x85RPPPPPP\x90P\x01\x91PP`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a1\xDCW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a1\xE1V[``\x91P[PP\x90P\x80a2!W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`#\x81R` \x01\x80aD\xF1`#\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[PPPV[`\0\x80\x84\x11a2fW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`+\x81R` \x01\x80aEc`+\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[`\0\x83\x11\x80\x15a2vWP`\0\x82\x11[a2\xB1W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`(\x81R` \x01\x80aD7`(\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[`\0a2\xC5\x85a\x03\xE5c\xFF\xFF\xFF\xFFaBF\x16V[\x90P`\0a2\xD9\x82\x85c\xFF\xFF\xFF\xFFaBF\x16V[\x90P`\0a2\xFF\x83a2\xF3\x88a\x03\xE8c\xFF\xFF\xFF\xFFaBF\x16V[\x90c\xFF\xFF\xFF\xFFaB\xA9\x16V[\x90P\x80\x82\x81a3\nW\xFE[\x04\x97\x96PPPPPPPV[```\x02\x82Q\x10\x15a3oW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7FUniswapV2Library: INVALID_PATH\0\0`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x80\x15a3\x87W`\0\x80\xFD[P`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a3\xB1W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P\x82\x81`\0\x81Q\x81\x10a3\xC2W\xFE[` \x02` \x01\x01\x81\x81RPP`\0[`\x01\x83Q\x03\x81\x10\x15a4ZW`\0\x80a4\x14\x87\x86\x85\x81Q\x81\x10a3\xF0W\xFE[` \x02` \x01\x01Q\x87\x86`\x01\x01\x81Q\x81\x10a4\x07W\xFE[` \x02` \x01\x01QaB\xF8V[\x91P\x91Pa46\x84\x84\x81Q\x81\x10a4'W\xFE[` \x02` \x01\x01Q\x83\x83a2&V[\x84\x84`\x01\x01\x81Q\x81\x10a4EW\xFE[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a3\xD1V[P\x93\x92PPPV[`\0\x80`\0a4q\x85\x85a>\xF4V[`@\x80Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19``\x94\x85\x1B\x81\x16` \x80\x84\x01\x91\x90\x91R\x93\x85\x1B\x81\x16`4\x83\x01R\x82Q`(\x81\x84\x03\x01\x81R`H\x83\x01\x84R\x80Q\x90\x85\x01 `\x01`\x01`\xF8\x1B\x03\x19`h\x84\x01R\x9A\x90\x94\x1B\x90\x93\x16`i\x84\x01R`}\x83\x01\x98\x90\x98R\x7F\x96\xE8\xACBw\x19\x8F\xF8\xB6\xF7\x85G\x8A\xA9\xA3\x9F@<\xB7h\xDD\x02\xCB\xEE2l>}\xA3H\x84_`\x9D\x80\x84\x01\x91\x90\x91R\x88Q\x80\x84\x03\x90\x91\x01\x81R`\xBD\x90\x92\x01\x90\x97R\x80Q\x96\x01\x95\x90\x95 \x95\x94PPPPPV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`$\x83\x01R\x84\x81\x16`D\x83\x01R`d\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`\x84\x90\x92\x01\x83R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c#\xB8r\xDD`\xE0\x1B\x17\x81R\x92Q\x82Q`\0\x94``\x94\x93\x8A\x16\x93\x92\x91\x82\x91\x90\x80\x83\x83[` \x83\x10a5\xA7W\x80Q\x82R`\x1F\x19\x90\x92\x01\x91` \x91\x82\x01\x91\x01a5\x88V[`\x01\x83` \x03a\x01\0\n\x03\x80\x19\x82Q\x16\x81\x84Q\x16\x80\x82\x17\x85RPPPPPP\x90P\x01\x91PP`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a6\tW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a6\x0EV[``\x91P[P\x91P\x91P\x81\x80\x15a6<WP\x80Q\x15\x80a6<WP\x80\x80` \x01\x90Q` \x81\x10\x15a69W`\0\x80\xFD[PQ[a6wW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`$\x81R` \x01\x80aE?`$\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[PPPPPPV[`\0[`\x01\x83Q\x03\x81\x10\x15a8\xBFW`\0\x80\x84\x83\x81Q\x81\x10a6\x9DW\xFE[` \x02` \x01\x01Q\x85\x84`\x01\x01\x81Q\x81\x10a6\xB4W\xFE[` \x02` \x01\x01Q\x91P\x91P`\0a6\xCC\x83\x83a>\xF4V[P\x90P`\0\x87\x85`\x01\x01\x81Q\x81\x10a6\xE0W\xFE[` \x02` \x01\x01Q\x90P`\0\x80\x83`\x01`\x01`\xA0\x1B\x03\x16\x86`\x01`\x01`\xA0\x1B\x03\x16\x14a7\x0EW\x82`\0a7\x12V[`\0\x83[\x91P\x91P`\0`\x02\x8AQ\x03\x88\x10a7)W\x88a7jV[a7j\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x87\x8C\x8B`\x02\x01\x81Q\x81\x10a7]W\xFE[` \x02` \x01\x01Qa4bV[\x90Pa7\x97\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x88\x88a4bV[`\x01`\x01`\xA0\x1B\x03\x16c\x02,\r\x9F\x84\x84\x84`\0`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a7\xD4W` \x82\x01\x81\x806\x837\x01\x90P[P`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x85\x81R` \x01\x84\x81R` \x01\x83`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x80` \x01\x82\x81\x03\x82R\x83\x81\x81Q\x81R` \x01\x91P\x80Q\x90` \x01\x90\x80\x83\x83`\0[\x83\x81\x10\x15a8EW\x81\x81\x01Q\x83\x82\x01R` \x01a8-V[PPPP\x90P\x90\x81\x01\x90`\x1F\x16\x80\x15a8rW\x80\x82\x03\x80Q`\x01\x83` \x03a\x01\0\n\x03\x19\x16\x81R` \x01\x91P[P\x95PPPPPP`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a8\x94W`\0\x80\xFD[PZ\xF1\x15\x80\x15a8\xA8W=`\0\x80>=`\0\xFD[PP`\x01\x90\x99\x01\x98Pa6\x82\x97PPPPPPPPV[PPPPV[```\x02\x82Q\x10\x15a9\x1EW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7FUniswapV2Library: INVALID_PATH\0\0`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x80\x15a96W`\0\x80\xFD[P`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a9`W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P\x82\x81`\x01\x83Q\x03\x81Q\x81\x10a9tW\xFE[` \x90\x81\x02\x91\x90\x91\x01\x01R\x81Q`\0\x19\x01[\x80\x15a4ZW`\0\x80a9\xB6\x87\x86`\x01\x86\x03\x81Q\x81\x10a9\xA2W\xFE[` \x02` \x01\x01Q\x87\x86\x81Q\x81\x10a4\x07W\xFE[\x91P\x91Pa9\xD8\x84\x84\x81Q\x81\x10a9\xC9W\xFE[` \x02` \x01\x01Q\x83\x83a=XV[\x84`\x01\x85\x03\x81Q\x81\x10a9\xE7W\xFE[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\0\x19\x01a9\x86V[`\0[`\x01\x83Q\x03\x81\x10\x15a2!W`\0\x80\x84\x83\x81Q\x81\x10a:\x1BW\xFE[` \x02` \x01\x01Q\x85\x84`\x01\x01\x81Q\x81\x10a:2W\xFE[` \x02` \x01\x01Q\x91P\x91P`\0a:J\x83\x83a>\xF4V[P\x90P`\0a:z\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85\x85a4bV[\x90P`\0\x80`\0\x80\x84`\x01`\x01`\xA0\x1B\x03\x16c\t\x02\xF1\xAC`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a:\xBBW`\0\x80\xFD[PZ\xFA\x15\x80\x15a:\xCFW=`\0\x80>=`\0\xFD[PPPP`@Q=``\x81\x10\x15a:\xE5W`\0\x80\xFD[P\x80Q` \x90\x91\x01Q`\x01`\x01`p\x1B\x03\x91\x82\x16\x93P\x16\x90P`\0\x80`\x01`\x01`\xA0\x1B\x03\x8A\x81\x16\x90\x89\x16\x14a;\x1BW\x82\x84a;\x1EV[\x83\x83[\x91P\x91Pa;|\x82\x8B`\x01`\x01`\xA0\x1B\x03\x16cp\xA0\x821\x8A`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x82`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x91PP` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x18\x1EW`\0\x80\xFD[\x95Pa;\x89\x86\x83\x83a2&V[\x94PPPPP`\0\x80\x85`\x01`\x01`\xA0\x1B\x03\x16\x88`\x01`\x01`\xA0\x1B\x03\x16\x14a;\xB3W\x82`\0a;\xB7V[`\0\x83[\x91P\x91P`\0`\x02\x8CQ\x03\x8A\x10a;\xCEW\x8Aa<\x02V[a<\x02\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x89\x8E\x8D`\x02\x01\x81Q\x81\x10a7]W\xFE[`@\x80Q`\0\x80\x82R` \x82\x01\x92\x83\x90Rc\x02,\r\x9F`\xE0\x1B\x83R`$\x82\x01\x87\x81R`D\x83\x01\x87\x90R`\x01`\x01`\xA0\x1B\x03\x80\x86\x16`d\x85\x01R`\x80`\x84\x85\x01\x90\x81R\x84Q`\xA4\x86\x01\x81\x90R\x96\x97P\x90\x8C\x16\x95c\x02,\r\x9F\x95\x8A\x95\x8A\x95\x8A\x95\x91\x94\x91\x93\x91\x92`\xC4\x86\x01\x92\x90\x91\x81\x90\x84\x90\x84\x90[\x83\x81\x10\x15a<\x8CW\x81\x81\x01Q\x83\x82\x01R` \x01a<tV[PPPP\x90P\x90\x81\x01\x90`\x1F\x16\x80\x15a<\xB9W\x80\x82\x03\x80Q`\x01\x83` \x03a\x01\0\n\x03\x19\x16\x81R` \x01\x91P[P\x95PPPPPP`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a<\xDBW`\0\x80\xFD[PZ\xF1\x15\x80\x15a<\xEFW=`\0\x80>=`\0\xFD[PP`\x01\x90\x9B\x01\x9APa:\0\x99PPPPPPPPPPV[\x80\x82\x03\x82\x81\x11\x15a\x11#W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rtds-math-sub-underflow`X\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\0\x80\x84\x11a=\x98W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`,\x81R` \x01\x80aC\xC0`,\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[`\0\x83\x11\x80\x15a=\xA8WP`\0\x82\x11[a=\xE3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`(\x81R` \x01\x80aD7`(\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[`\0a>\x07a\x03\xE8a=\xFB\x86\x88c\xFF\xFF\xFF\xFFaBF\x16V[\x90c\xFF\xFF\xFF\xFFaBF\x16V[\x90P`\0a>!a\x03\xE5a=\xFB\x86\x89c\xFF\xFF\xFF\xFFa=\x08\x16V[\x90Pa>>`\x01\x82\x84\x81a>1W\xFE[\x04\x90c\xFF\xFF\xFF\xFFaB\xA9\x16V[\x96\x95PPPPPPV[`\0\x80\x84\x11a>\x88W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`%\x81R` \x01\x80aD_`%\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[`\0\x83\x11\x80\x15a>\x98WP`\0\x82\x11[a>\xD3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`(\x81R` \x01\x80aD7`(\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[\x82a>\xE4\x85\x84c\xFF\xFF\xFF\xFFaBF\x16V[\x81a>\xEBW\xFE[\x04\x94\x93PPPPV[`\0\x80\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a?HW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`%\x81R` \x01\x80aC\xEC`%\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x10a?hW\x82\x84a?kV[\x83\x83[\x90\x92P\x90P`\x01`\x01`\xA0\x1B\x03\x82\x16a?\xCBW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7FUniswapV2Library: ZERO_ADDRESS\0\0`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x92P\x92\x90PV[`@\x80Qc\xE6\xA49\x05`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x88\x81\x16`\x04\x83\x01R\x87\x81\x16`$\x83\x01R\x91Q`\0\x92\x83\x92\x83\x92\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x92\x16\x91c\xE6\xA49\x05\x91`D\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a@LW`\0\x80\xFD[PZ\xFA\x15\x80\x15a@`W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a@vW`\0\x80\xFD[PQ`\x01`\x01`\xA0\x1B\x03\x16\x14\x15aA)W`@\x80Qcd\xE3)\xCB`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8A\x81\x16`\x04\x83\x01R\x89\x81\x16`$\x83\x01R\x91Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x92\x16\x91c\xC9\xC6S\x96\x91`D\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81`\0\x87\x80;\x15\x80\x15a@\xFCW`\0\x80\xFD[PZ\xF1\x15\x80\x15aA\x10W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15aA&W`\0\x80\xFD[PP[`\0\x80aAW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x8B\x8BaB\xF8V[\x91P\x91P\x81`\0\x14\x80\x15aAiWP\x80\x15[\x15aAyW\x87\x93P\x86\x92PaB9V[`\0aA\x86\x89\x84\x84a>HV[\x90P\x87\x81\x11aA\xD9W\x85\x81\x10\x15aA\xCEW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`&\x81R` \x01\x80aD\x11`&\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[\x88\x94P\x92P\x82aB7V[`\0aA\xE6\x89\x84\x86a>HV[\x90P\x89\x81\x11\x15aA\xF2W\xFE[\x87\x81\x10\x15aB1W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`&\x81R` \x01\x80aD\xCB`&\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[\x94P\x87\x93P[P[PP\x96P\x96\x94PPPPPV[`\0\x81\x15\x80aBaWPP\x80\x82\x02\x82\x82\x82\x81aB^W\xFE[\x04\x14[a\x11#W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rsds-math-mul-overflow``\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x80\x82\x01\x82\x81\x10\x15a\x11#W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rsds-math-add-overflow``\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\0\x80`\0aC\x07\x85\x85a>\xF4V[P\x90P`\0\x80aC\x18\x88\x88\x88a4bV[`\x01`\x01`\xA0\x1B\x03\x16c\t\x02\xF1\xAC`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15aCPW`\0\x80\xFD[PZ\xFA\x15\x80\x15aCdW=`\0\x80>=`\0\xFD[PPPP`@Q=``\x81\x10\x15aCzW`\0\x80\xFD[P\x80Q` \x90\x91\x01Q`\x01`\x01`p\x1B\x03\x91\x82\x16\x93P\x16\x90P`\x01`\x01`\xA0\x1B\x03\x87\x81\x16\x90\x84\x16\x14aC\xADW\x80\x82aC\xB0V[\x81\x81[\x90\x99\x90\x98P\x96PPPPPPPV\xFEUniswapV2Library: INSUFFICIENT_OUTPUT_AMOUNTUniswapV2Library: IDENTICAL_ADDRESSESUniswapV2Router: INSUFFICIENT_B_AMOUNTUniswapV2Library: INSUFFICIENT_LIQUIDITYUniswapV2Library: INSUFFICIENT_AMOUNTUniswapV2Router: EXCESSIVE_INPUT_AMOUNTUniswapV2Router: INVALID_PATH\0\0\0UniswapV2Router: INSUFFICIENT_A_AMOUNTTransferHelper: ETH_TRANSFER_FAILEDUniswapV2Router: INSUFFICIENT_OUTPUT_AMOUNTTransferHelper: TRANSFER_FROM_FAILEDUniswapV2Library: INSUFFICIENT_INPUT_AMOUNTUniswapV2Router: EXPIRED\0\0\0\0\0\0\0\0\xA2dipfsX\"\x12 \xEA\x9B\x1C\x7F\x08z\x07;\xD5\xBA\x03\xFC2\x0B]\"\xFA\x87B\x1B.\x16\xBC\x86\xA5\xEA\n\xF0\x07\x8C\n8dsolcC\0\x06\x06\x003";
    /// The bytecode of the contract.
    pub static UNISWAPV2ROUTER02_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\x01OW`\x005`\xE0\x1C\x80c\x88\x03\xDB\xEE\x11a\0\xB6W\x80c\xC4Z\x01U\x11a\0oW\x80c\xC4Z\x01U\x14a\n\x10W\x80c\xD0l\xA6\x1F\x14a\n%W\x80c\xDE\xD98*\x14a\n\xDAW\x80c\xE8\xE37\0\x14a\x0BMW\x80c\xF3\x05\xD7\x19\x14a\x0B\xCDW\x80c\xFB;\xDBA\x14a\x0C\x13Wa\x01\x88V[\x80c\x88\x03\xDB\xEE\x14a\x07\xDFW\x80c\xAD\\FH\x14a\x08uW\x80c\xADa]\xEC\x14a\x08\xA6W\x80c\xAF)y\xEB\x14a\x08\xDCW\x80c\xB6\xF9\xDE\x95\x14a\t/W\x80c\xBA\xA2\xAB\xDE\x14a\t\xB3Wa\x01\x88V[\x80cJ%\xD9J\x11a\x01\x08W\x80cJ%\xD9J\x14a\x04\xF0W\x80c[\rY\x84\x14a\x05\x86W\x80c\\\x11\xD7\x95\x14a\x05\xF9W\x80cy\x1A\xC9G\x14a\x06\x8FW\x80c\x7F\xF3j\xB5\x14a\x07%W\x80c\x85\xF8\xC2Y\x14a\x07\xA9Wa\x01\x88V[\x80c\x02u\x1C\xEC\x14a\x01\x8DW\x80c\x05MP\xD4\x14a\x01\xF9W\x80c\x18\xCB\xAF\xE5\x14a\x02AW\x80c\x1F\0\xCAt\x14a\x03'W\x80c!\x95\x99\\\x14a\x03\xDCW\x80c8\xED\x179\x14a\x04ZWa\x01\x88V[6a\x01\x88W3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x01\x86W\xFE[\0[`\0\x80\xFD[4\x80\x15a\x01\x99W`\0\x80\xFD[Pa\x01\xE0`\x04\x806\x03`\xC0\x81\x10\x15a\x01\xB0W`\0\x80\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x81\x16\x91` \x81\x015\x91`@\x82\x015\x91``\x81\x015\x91`\x80\x82\x015\x16\x90`\xA0\x015a\x0C\x97V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x80Q\x91\x82\x90\x03\x01\x90\xF3[4\x80\x15a\x02\x05W`\0\x80\xFD[Pa\x02/`\x04\x806\x03``\x81\x10\x15a\x02\x1CW`\0\x80\xFD[P\x805\x90` \x81\x015\x90`@\x015a\r\xB1V[`@\x80Q\x91\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[4\x80\x15a\x02MW`\0\x80\xFD[Pa\x02\xD7`\x04\x806\x03`\xA0\x81\x10\x15a\x02dW`\0\x80\xFD[\x815\x91` \x81\x015\x91\x81\x01\x90``\x81\x01`@\x82\x015`\x01` \x1B\x81\x11\x15a\x02\x8AW`\0\x80\xFD[\x82\x01\x83` \x82\x01\x11\x15a\x02\x9CW`\0\x80\xFD[\x805\x90` \x01\x91\x84` \x83\x02\x84\x01\x11`\x01` \x1B\x83\x11\x17\x15a\x02\xBDW`\0\x80\xFD[\x91\x93P\x91P`\x01`\x01`\xA0\x1B\x03\x815\x16\x90` \x015a\r\xC6V[`@\x80Q` \x80\x82R\x83Q\x81\x83\x01R\x83Q\x91\x92\x83\x92\x90\x83\x01\x91\x85\x81\x01\x91\x02\x80\x83\x83`\0[\x83\x81\x10\x15a\x03\x13W\x81\x81\x01Q\x83\x82\x01R` \x01a\x02\xFBV[PPPP\x90P\x01\x92PPP`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x033W`\0\x80\xFD[Pa\x02\xD7`\x04\x806\x03`@\x81\x10\x15a\x03JW`\0\x80\xFD[\x815\x91\x90\x81\x01\x90`@\x81\x01` \x82\x015`\x01` \x1B\x81\x11\x15a\x03kW`\0\x80\xFD[\x82\x01\x83` \x82\x01\x11\x15a\x03}W`\0\x80\xFD[\x805\x90` \x01\x91\x84` \x83\x02\x84\x01\x11`\x01` \x1B\x83\x11\x17\x15a\x03\x9EW`\0\x80\xFD[\x91\x90\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x92\x95Pa\x10\xF3\x94PPPPPV[4\x80\x15a\x03\xE8W`\0\x80\xFD[Pa\x01\xE0`\x04\x806\x03a\x01`\x81\x10\x15a\x04\0W`\0\x80\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x81\x16\x91` \x81\x015\x82\x16\x91`@\x82\x015\x91``\x81\x015\x91`\x80\x82\x015\x91`\xA0\x81\x015\x90\x91\x16\x90`\xC0\x81\x015\x90`\xE0\x81\x015\x15\x15\x90`\xFFa\x01\0\x82\x015\x16\x90a\x01 \x81\x015\x90a\x01@\x015a\x11)V[4\x80\x15a\x04fW`\0\x80\xFD[Pa\x02\xD7`\x04\x806\x03`\xA0\x81\x10\x15a\x04}W`\0\x80\xFD[\x815\x91` \x81\x015\x91\x81\x01\x90``\x81\x01`@\x82\x015`\x01` \x1B\x81\x11\x15a\x04\xA3W`\0\x80\xFD[\x82\x01\x83` \x82\x01\x11\x15a\x04\xB5W`\0\x80\xFD[\x805\x90` \x01\x91\x84` \x83\x02\x84\x01\x11`\x01` \x1B\x83\x11\x17\x15a\x04\xD6W`\0\x80\xFD[\x91\x93P\x91P`\x01`\x01`\xA0\x1B\x03\x815\x16\x90` \x015a\x12#V[4\x80\x15a\x04\xFCW`\0\x80\xFD[Pa\x02\xD7`\x04\x806\x03`\xA0\x81\x10\x15a\x05\x13W`\0\x80\xFD[\x815\x91` \x81\x015\x91\x81\x01\x90``\x81\x01`@\x82\x015`\x01` \x1B\x81\x11\x15a\x059W`\0\x80\xFD[\x82\x01\x83` \x82\x01\x11\x15a\x05KW`\0\x80\xFD[\x805\x90` \x01\x91\x84` \x83\x02\x84\x01\x11`\x01` \x1B\x83\x11\x17\x15a\x05lW`\0\x80\xFD[\x91\x93P\x91P`\x01`\x01`\xA0\x1B\x03\x815\x16\x90` \x015a\x13nV[4\x80\x15a\x05\x92W`\0\x80\xFD[Pa\x02/`\x04\x806\x03a\x01@\x81\x10\x15a\x05\xAAW`\0\x80\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x81\x16\x91` \x81\x015\x91`@\x82\x015\x91``\x81\x015\x91`\x80\x82\x015\x16\x90`\xA0\x81\x015\x90`\xC0\x81\x015\x15\x15\x90`\xFF`\xE0\x82\x015\x16\x90a\x01\0\x81\x015\x90a\x01 \x015a\x14\xFAV[4\x80\x15a\x06\x05W`\0\x80\xFD[Pa\x01\x86`\x04\x806\x03`\xA0\x81\x10\x15a\x06\x1CW`\0\x80\xFD[\x815\x91` \x81\x015\x91\x81\x01\x90``\x81\x01`@\x82\x015`\x01` \x1B\x81\x11\x15a\x06BW`\0\x80\xFD[\x82\x01\x83` \x82\x01\x11\x15a\x06TW`\0\x80\xFD[\x805\x90` \x01\x91\x84` \x83\x02\x84\x01\x11`\x01` \x1B\x83\x11\x17\x15a\x06uW`\0\x80\xFD[\x91\x93P\x91P`\x01`\x01`\xA0\x1B\x03\x815\x16\x90` \x015a\x16\x08V[4\x80\x15a\x06\x9BW`\0\x80\xFD[Pa\x01\x86`\x04\x806\x03`\xA0\x81\x10\x15a\x06\xB2W`\0\x80\xFD[\x815\x91` \x81\x015\x91\x81\x01\x90``\x81\x01`@\x82\x015`\x01` \x1B\x81\x11\x15a\x06\xD8W`\0\x80\xFD[\x82\x01\x83` \x82\x01\x11\x15a\x06\xEAW`\0\x80\xFD[\x805\x90` \x01\x91\x84` \x83\x02\x84\x01\x11`\x01` \x1B\x83\x11\x17\x15a\x07\x0BW`\0\x80\xFD[\x91\x93P\x91P`\x01`\x01`\xA0\x1B\x03\x815\x16\x90` \x015a\x18\x9DV[a\x02\xD7`\x04\x806\x03`\x80\x81\x10\x15a\x07;W`\0\x80\xFD[\x815\x91\x90\x81\x01\x90`@\x81\x01` \x82\x015`\x01` \x1B\x81\x11\x15a\x07\\W`\0\x80\xFD[\x82\x01\x83` \x82\x01\x11\x15a\x07nW`\0\x80\xFD[\x805\x90` \x01\x91\x84` \x83\x02\x84\x01\x11`\x01` \x1B\x83\x11\x17\x15a\x07\x8FW`\0\x80\xFD[\x91\x93P\x91P`\x01`\x01`\xA0\x1B\x03\x815\x16\x90` \x015a\x1B!V[4\x80\x15a\x07\xB5W`\0\x80\xFD[Pa\x02/`\x04\x806\x03``\x81\x10\x15a\x07\xCCW`\0\x80\xFD[P\x805\x90` \x81\x015\x90`@\x015a\x1EtV[4\x80\x15a\x07\xEBW`\0\x80\xFD[Pa\x02\xD7`\x04\x806\x03`\xA0\x81\x10\x15a\x08\x02W`\0\x80\xFD[\x815\x91` \x81\x015\x91\x81\x01\x90``\x81\x01`@\x82\x015`\x01` \x1B\x81\x11\x15a\x08(W`\0\x80\xFD[\x82\x01\x83` \x82\x01\x11\x15a\x08:W`\0\x80\xFD[\x805\x90` \x01\x91\x84` \x83\x02\x84\x01\x11`\x01` \x1B\x83\x11\x17\x15a\x08[W`\0\x80\xFD[\x91\x93P\x91P`\x01`\x01`\xA0\x1B\x03\x815\x16\x90` \x015a\x1E\x81V[4\x80\x15a\x08\x81W`\0\x80\xFD[Pa\x08\x8Aa\x1FzV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[4\x80\x15a\x08\xB2W`\0\x80\xFD[Pa\x02/`\x04\x806\x03``\x81\x10\x15a\x08\xC9W`\0\x80\xFD[P\x805\x90` \x81\x015\x90`@\x015a\x1F\x9EV[4\x80\x15a\x08\xE8W`\0\x80\xFD[Pa\x02/`\x04\x806\x03`\xC0\x81\x10\x15a\x08\xFFW`\0\x80\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x81\x16\x91` \x81\x015\x91`@\x82\x015\x91``\x81\x015\x91`\x80\x82\x015\x16\x90`\xA0\x015a\x1F\xABV[a\x01\x86`\x04\x806\x03`\x80\x81\x10\x15a\tEW`\0\x80\xFD[\x815\x91\x90\x81\x01\x90`@\x81\x01` \x82\x015`\x01` \x1B\x81\x11\x15a\tfW`\0\x80\xFD[\x82\x01\x83` \x82\x01\x11\x15a\txW`\0\x80\xFD[\x805\x90` \x01\x91\x84` \x83\x02\x84\x01\x11`\x01` \x1B\x83\x11\x17\x15a\t\x99W`\0\x80\xFD[\x91\x93P\x91P`\x01`\x01`\xA0\x1B\x03\x815\x16\x90` \x015a!,V[4\x80\x15a\t\xBFW`\0\x80\xFD[Pa\x01\xE0`\x04\x806\x03`\xE0\x81\x10\x15a\t\xD6W`\0\x80\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x81\x16\x91` \x81\x015\x82\x16\x91`@\x82\x015\x91``\x81\x015\x91`\x80\x82\x015\x91`\xA0\x81\x015\x90\x91\x16\x90`\xC0\x015a$\xB8V[4\x80\x15a\n\x1CW`\0\x80\xFD[Pa\x08\x8Aa&\xFCV[4\x80\x15a\n1W`\0\x80\xFD[Pa\x02\xD7`\x04\x806\x03`@\x81\x10\x15a\nHW`\0\x80\xFD[\x815\x91\x90\x81\x01\x90`@\x81\x01` \x82\x015`\x01` \x1B\x81\x11\x15a\niW`\0\x80\xFD[\x82\x01\x83` \x82\x01\x11\x15a\n{W`\0\x80\xFD[\x805\x90` \x01\x91\x84` \x83\x02\x84\x01\x11`\x01` \x1B\x83\x11\x17\x15a\n\x9CW`\0\x80\xFD[\x91\x90\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x92\x95Pa' \x94PPPPPV[4\x80\x15a\n\xE6W`\0\x80\xFD[Pa\x01\xE0`\x04\x806\x03a\x01@\x81\x10\x15a\n\xFEW`\0\x80\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x81\x16\x91` \x81\x015\x91`@\x82\x015\x91``\x81\x015\x91`\x80\x82\x015\x16\x90`\xA0\x81\x015\x90`\xC0\x81\x015\x15\x15\x90`\xFF`\xE0\x82\x015\x16\x90a\x01\0\x81\x015\x90a\x01 \x015a'MV[4\x80\x15a\x0BYW`\0\x80\xFD[Pa\x0B\xAF`\x04\x806\x03a\x01\0\x81\x10\x15a\x0BqW`\0\x80\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x81\x16\x91` \x81\x015\x82\x16\x91`@\x82\x015\x91``\x81\x015\x91`\x80\x82\x015\x91`\xA0\x81\x015\x91`\xC0\x82\x015\x16\x90`\xE0\x015a(aV[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x82\x82\x01RQ\x90\x81\x90\x03``\x01\x90\xF3[a\x0B\xAF`\x04\x806\x03`\xC0\x81\x10\x15a\x0B\xE3W`\0\x80\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x81\x16\x91` \x81\x015\x91`@\x82\x015\x91``\x81\x015\x91`\x80\x82\x015\x16\x90`\xA0\x015a)\x9DV[a\x02\xD7`\x04\x806\x03`\x80\x81\x10\x15a\x0C)W`\0\x80\xFD[\x815\x91\x90\x81\x01\x90`@\x81\x01` \x82\x015`\x01` \x1B\x81\x11\x15a\x0CJW`\0\x80\xFD[\x82\x01\x83` \x82\x01\x11\x15a\x0C\\W`\0\x80\xFD[\x805\x90` \x01\x91\x84` \x83\x02\x84\x01\x11`\x01` \x1B\x83\x11\x17\x15a\x0C}W`\0\x80\xFD[\x91\x93P\x91P`\x01`\x01`\xA0\x1B\x03\x815\x16\x90` \x015a,BV[`\0\x80\x82B\x81\x10\x15a\x0C\xDEW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R`\0\x80Q` aE\x8E\x839\x81Q\x91R`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[a\r\r\x89\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x8A\x8A\x8A0\x8Aa$\xB8V[\x90\x93P\x91Pa\r\x1D\x89\x86\x85a/\xC4V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c.\x1A}M\x83`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x82\x81R` \x01\x91PP`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\r\x83W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\r\x97W=`\0\x80>=`\0\xFD[PPPPa\r\xA5\x85\x83a1.V[P\x96P\x96\x94PPPPPV[`\0a\r\xBE\x84\x84\x84a2&V[\x94\x93PPPPV[``\x81B\x81\x10\x15a\x0E\x0CW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R`\0\x80Q` aE\x8E\x839\x81Q\x91R`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x86\x86`\0\x19\x81\x01\x81\x81\x10a\x0EFW\xFE[\x90P` \x02\x015`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0E\x9FW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R`\0\x80Q` aD\xAB\x839\x81Q\x91R`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[a\x0E\xFD\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x89\x88\x88\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa3\x16\x92PPPV[\x91P\x86\x82`\x01\x84Q\x03\x81Q\x81\x10a\x0F\x10W\xFE[` \x02` \x01\x01Q\x10\x15a\x0FUW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`+\x81R` \x01\x80aE\x14`+\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[a\x0F\xF3\x86\x86`\0\x81\x81\x10a\x0FeW\xFE[\x90P` \x02\x015`\x01`\x01`\xA0\x1B\x03\x163a\x0F\xD9\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x8A\x8A`\0\x81\x81\x10a\x0F\xA7W\xFE[\x90P` \x02\x015`\x01`\x01`\xA0\x1B\x03\x16\x8B\x8B`\x01\x81\x81\x10a\x0F\xC4W\xFE[\x90P` \x02\x015`\x01`\x01`\xA0\x1B\x03\x16a4bV[\x85`\0\x81Q\x81\x10a\x0F\xE6W\xFE[` \x02` \x01\x01Qa5\"V[a\x102\x82\x87\x87\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RP0\x92Pa6\x7F\x91PPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c.\x1A}M\x83`\x01\x85Q\x03\x81Q\x81\x10a\x10qW\xFE[` \x02` \x01\x01Q`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x82\x81R` \x01\x91PP`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x10\xAFW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x10\xC3W=`\0\x80>=`\0\xFD[PPPPa\x10\xE8\x84\x83`\x01\x85Q\x03\x81Q\x81\x10a\x10\xDBW\xFE[` \x02` \x01\x01Qa1.V[P\x96\x95PPPPPPV[``a\x11 \x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84\x84a8\xC5V[\x90P[\x92\x91PPV[`\0\x80`\0a\x11Y\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x8F\x8Fa4bV[\x90P`\0\x87a\x11hW\x8Ca\x11lV[`\0\x19[`@\x80Qc\xD5\x05\xAC\xCF`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x83\x90R`d\x81\x01\x8C\x90R`\xFF\x8A\x16`\x84\x82\x01R`\xA4\x81\x01\x89\x90R`\xC4\x81\x01\x88\x90R\x90Q\x91\x92P`\x01`\x01`\xA0\x1B\x03\x84\x16\x91c\xD5\x05\xAC\xCF\x91`\xE4\x80\x82\x01\x92`\0\x92\x90\x91\x90\x82\x90\x03\x01\x81\x83\x87\x80;\x15\x80\x15a\x11\xE2W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x11\xF6W=`\0\x80>=`\0\xFD[PPPPa\x12\t\x8F\x8F\x8F\x8F\x8F\x8F\x8Fa$\xB8V[\x80\x94P\x81\x95PPPPP\x9BP\x9B\x99PPPPPPPPPPV[``\x81B\x81\x10\x15a\x12iW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R`\0\x80Q` aE\x8E\x839\x81Q\x91R`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[a\x12\xC7\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x89\x88\x88\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa3\x16\x92PPPV[\x91P\x86\x82`\x01\x84Q\x03\x81Q\x81\x10a\x12\xDAW\xFE[` \x02` \x01\x01Q\x10\x15a\x13\x1FW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`+\x81R` \x01\x80aE\x14`+\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[a\x13/\x86\x86`\0\x81\x81\x10a\x0FeW\xFE[a\x10\xE8\x82\x87\x87\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x89\x92Pa6\x7F\x91PPV[``\x81B\x81\x10\x15a\x13\xB4W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R`\0\x80Q` aE\x8E\x839\x81Q\x91R`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x86\x86`\0\x19\x81\x01\x81\x81\x10a\x13\xEEW\xFE[\x90P` \x02\x015`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x14a\x14GW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R`\0\x80Q` aD\xAB\x839\x81Q\x91R`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[a\x14\xA5\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x89\x88\x88\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa8\xC5\x92PPPV[\x91P\x86\x82`\0\x81Q\x81\x10a\x14\xB5W\xFE[` \x02` \x01\x01Q\x11\x15a\x0FUW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`'\x81R` \x01\x80aD\x84`'\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[`\0\x80a\x15H\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x8D\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a4bV[\x90P`\0\x86a\x15WW\x8Ba\x15[V[`\0\x19[`@\x80Qc\xD5\x05\xAC\xCF`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x83\x90R`d\x81\x01\x8B\x90R`\xFF\x89\x16`\x84\x82\x01R`\xA4\x81\x01\x88\x90R`\xC4\x81\x01\x87\x90R\x90Q\x91\x92P`\x01`\x01`\xA0\x1B\x03\x84\x16\x91c\xD5\x05\xAC\xCF\x91`\xE4\x80\x82\x01\x92`\0\x92\x90\x91\x90\x82\x90\x03\x01\x81\x83\x87\x80;\x15\x80\x15a\x15\xD1W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x15\xE5W=`\0\x80>=`\0\xFD[PPPPa\x15\xF7\x8D\x8D\x8D\x8D\x8D\x8Da\x1F\xABV[\x9D\x9CPPPPPPPPPPPPPV[\x80B\x81\x10\x15a\x16LW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R`\0\x80Q` aE\x8E\x839\x81Q\x91R`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[a\x16\xC1\x85\x85`\0\x81\x81\x10a\x16\\W\xFE[\x90P` \x02\x015`\x01`\x01`\xA0\x1B\x03\x163a\x16\xBB\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x89\x89`\0\x81\x81\x10a\x16\x9EW\xFE[\x90P` \x02\x015`\x01`\x01`\xA0\x1B\x03\x16\x8A\x8A`\x01\x81\x81\x10a\x0F\xC4W\xFE[\x8Aa5\"V[`\0\x85\x85`\0\x19\x81\x01\x81\x81\x10a\x16\xD3W\xFE[\x90P` \x02\x015`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cp\xA0\x821\x85`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x82`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x91PP` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x178W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x17LW=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x17bW`\0\x80\xFD[PQ`@\x80Q` \x88\x81\x02\x82\x81\x01\x82\x01\x90\x93R\x88\x82R\x92\x93Pa\x17\xA4\x92\x90\x91\x89\x91\x89\x91\x82\x91\x85\x01\x90\x84\x90\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x88\x92Pa9\xFD\x91PPV[\x86a\x18V\x82\x88\x88`\0\x19\x81\x01\x81\x81\x10a\x17\xB9W\xFE[\x90P` \x02\x015`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cp\xA0\x821\x88`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x82`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x91PP` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x18\x1EW`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x182W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x18HW`\0\x80\xFD[PQ\x90c\xFF\xFF\xFF\xFFa=\x08\x16V[\x10\x15a\x18\x93W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`+\x81R` \x01\x80aE\x14`+\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[PPPPPPPPV[\x80B\x81\x10\x15a\x18\xE1W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R`\0\x80Q` aE\x8E\x839\x81Q\x91R`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x85\x85`\0\x19\x81\x01\x81\x81\x10a\x19\x1BW\xFE[\x90P` \x02\x015`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x14a\x19tW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R`\0\x80Q` aD\xAB\x839\x81Q\x91R`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[a\x19\x84\x85\x85`\0\x81\x81\x10a\x16\\W\xFE[a\x19\xC2\x85\x85\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RP0\x92Pa9\xFD\x91PPV[`@\x80Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90Q`\0\x91`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91cp\xA0\x821\x91`$\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\x1A,W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x1A@W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x1AVW`\0\x80\xFD[PQ\x90P\x86\x81\x10\x15a\x1A\x99W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`+\x81R` \x01\x80aE\x14`+\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c.\x1A}M\x82`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x82\x81R` \x01\x91PP`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1A\xFFW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1B\x13W=`\0\x80>=`\0\xFD[PPPPa\x18\x93\x84\x82a1.V[``\x81B\x81\x10\x15a\x1BgW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R`\0\x80Q` aE\x8E\x839\x81Q\x91R`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x86\x86`\0\x81\x81\x10a\x1B\x9EW\xFE[\x90P` \x02\x015`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1B\xF7W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R`\0\x80Q` aD\xAB\x839\x81Q\x91R`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[a\x1CU\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x004\x88\x88\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa3\x16\x92PPPV[\x91P\x86\x82`\x01\x84Q\x03\x81Q\x81\x10a\x1ChW\xFE[` \x02` \x01\x01Q\x10\x15a\x1C\xADW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`+\x81R` \x01\x80aE\x14`+\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xD0\xE3\r\xB0\x83`\0\x81Q\x81\x10a\x1C\xE9W\xFE[` \x02` \x01\x01Q`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\x1D\x1CW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1D0W=`\0\x80>=`\0\xFD[PPPPP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xA9\x05\x9C\xBBa\x1D\x95\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x89\x89`\0\x81\x81\x10a\x16\x9EW\xFE[\x84`\0\x81Q\x81\x10a\x1D\xA2W\xFE[` \x02` \x01\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x83`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x82\x81R` \x01\x92PPP` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1D\xF9W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1E\rW=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x1E#W`\0\x80\xFD[PQa\x1E+W\xFE[a\x1Ej\x82\x87\x87\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x89\x92Pa6\x7F\x91PPV[P\x95\x94PPPPPV[`\0a\r\xBE\x84\x84\x84a=XV[``\x81B\x81\x10\x15a\x1E\xC7W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R`\0\x80Q` aE\x8E\x839\x81Q\x91R`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[a\x1F%\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x89\x88\x88\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa8\xC5\x92PPPV[\x91P\x86\x82`\0\x81Q\x81\x10a\x1F5W\xFE[` \x02` \x01\x01Q\x11\x15a\x13\x1FW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`'\x81R` \x01\x80aD\x84`'\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0a\r\xBE\x84\x84\x84a>HV[`\0\x81B\x81\x10\x15a\x1F\xF1W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R`\0\x80Q` aE\x8E\x839\x81Q\x91R`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[a  \x88\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x89\x89\x890\x89a$\xB8V[`@\x80Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90Q\x91\x94Pa \xA4\x92P\x8A\x91\x87\x91`\x01`\x01`\xA0\x1B\x03\x84\x16\x91cp\xA0\x821\x91`$\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a sW`\0\x80\xFD[PZ\xFA\x15\x80\x15a \x87W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a \x9DW`\0\x80\xFD[PQa/\xC4V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c.\x1A}M\x83`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x82\x81R` \x01\x91PP`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a!\nW`\0\x80\xFD[PZ\xF1\x15\x80\x15a!\x1EW=`\0\x80>=`\0\xFD[PPPPa\x10\xE8\x84\x83a1.V[\x80B\x81\x10\x15a!pW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R`\0\x80Q` aE\x8E\x839\x81Q\x91R`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x85\x85`\0\x81\x81\x10a!\xA7W\xFE[\x90P` \x02\x015`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x14a\"\0W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R`\0\x80Q` aD\xAB\x839\x81Q\x91R`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\x004\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xD0\xE3\r\xB0\x82`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\"`W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\"tW=`\0\x80>=`\0\xFD[PPPPP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xA9\x05\x9C\xBBa\"\xD9\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x89\x89`\0\x81\x81\x10a\x16\x9EW\xFE[\x83`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x83`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x82\x81R` \x01\x92PPP` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a#)W`\0\x80\xFD[PZ\xF1\x15\x80\x15a#=W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a#SW`\0\x80\xFD[PQa#[W\xFE[`\0\x86\x86`\0\x19\x81\x01\x81\x81\x10a#mW\xFE[\x90P` \x02\x015`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cp\xA0\x821\x86`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x82`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x91PP` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a#\xD2W`\0\x80\xFD[PZ\xFA\x15\x80\x15a#\xE6W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a#\xFCW`\0\x80\xFD[PQ`@\x80Q` \x89\x81\x02\x82\x81\x01\x82\x01\x90\x93R\x89\x82R\x92\x93Pa$>\x92\x90\x91\x8A\x91\x8A\x91\x82\x91\x85\x01\x90\x84\x90\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x89\x92Pa9\xFD\x91PPV[\x87a\x18V\x82\x89\x89`\0\x19\x81\x01\x81\x81\x10a$SW\xFE[\x90P` \x02\x015`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cp\xA0\x821\x89`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x82`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x91PP` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x18\x1EW`\0\x80\xFD[`\0\x80\x82B\x81\x10\x15a$\xFFW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R`\0\x80Q` aE\x8E\x839\x81Q\x91R`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\0a%,\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x8C\x8Ca4bV[`@\x80Qc#\xB8r\xDD`\xE0\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16`$\x82\x01\x81\x90R`D\x82\x01\x8D\x90R\x91Q\x92\x93P\x90\x91c#\xB8r\xDD\x91`d\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81`\0\x87\x80;\x15\x80\x15a%\x87W`\0\x80\xFD[PZ\xF1\x15\x80\x15a%\x9BW=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a%\xB1W`\0\x80\xFD[PP`@\x80Qc\"k\xF2\xD1`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x88\x81\x16`\x04\x83\x01R\x82Q`\0\x93\x84\x93\x92\x86\x16\x92c\x89\xAF\xCBD\x92`$\x80\x83\x01\x93\x92\x82\x90\x03\x01\x81\x87\x87\x80;\x15\x80\x15a%\xFEW`\0\x80\xFD[PZ\xF1\x15\x80\x15a&\x12W=`\0\x80>=`\0\xFD[PPPP`@Q=`@\x81\x10\x15a&(W`\0\x80\xFD[P\x80Q` \x90\x91\x01Q\x90\x92P\x90P`\0a&B\x8E\x8Ea>\xF4V[P\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16\x8E`\x01`\x01`\xA0\x1B\x03\x16\x14a&eW\x81\x83a&hV[\x82\x82[\x90\x97P\x95P\x8A\x87\x10\x15a&\xACW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`&\x81R` \x01\x80aD\xCB`&\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[\x89\x86\x10\x15a&\xEBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`&\x81R` \x01\x80aD\x11`&\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[PPPPP\x97P\x97\x95PPPPPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[``a\x11 \x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84\x84a3\x16V[`\0\x80`\0a'\x9D\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x8E\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a4bV[\x90P`\0\x87a'\xACW\x8Ca'\xB0V[`\0\x19[`@\x80Qc\xD5\x05\xAC\xCF`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x83\x90R`d\x81\x01\x8C\x90R`\xFF\x8A\x16`\x84\x82\x01R`\xA4\x81\x01\x89\x90R`\xC4\x81\x01\x88\x90R\x90Q\x91\x92P`\x01`\x01`\xA0\x1B\x03\x84\x16\x91c\xD5\x05\xAC\xCF\x91`\xE4\x80\x82\x01\x92`\0\x92\x90\x91\x90\x82\x90\x03\x01\x81\x83\x87\x80;\x15\x80\x15a(&W`\0\x80\xFD[PZ\xF1\x15\x80\x15a(:W=`\0\x80>=`\0\xFD[PPPPa(L\x8E\x8E\x8E\x8E\x8E\x8Ea\x0C\x97V[\x90\x9F\x90\x9EP\x9CPPPPPPPPPPPPPV[`\0\x80`\0\x83B\x81\x10\x15a(\xAAW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R`\0\x80Q` aE\x8E\x839\x81Q\x91R`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[a(\xB8\x8C\x8C\x8C\x8C\x8C\x8Ca?\xD2V[\x90\x94P\x92P`\0a(\xEA\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x8E\x8Ea4bV[\x90Pa(\xF8\x8D3\x83\x88a5\"V[a)\x04\x8C3\x83\x87a5\"V[\x80`\x01`\x01`\xA0\x1B\x03\x16cjbxB\x88`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x82`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x91PP` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a)\\W`\0\x80\xFD[PZ\xF1\x15\x80\x15a)pW=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a)\x86W`\0\x80\xFD[PQ\x94\x9D\x93\x9CP\x93\x9AP\x91\x98PPPPPPPPPV[`\0\x80`\0\x83B\x81\x10\x15a)\xE6W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R`\0\x80Q` aE\x8E\x839\x81Q\x91R`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[a*\x14\x8A\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x8B4\x8C\x8Ca?\xD2V[\x90\x94P\x92P`\0a*f\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x8C\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a4bV[\x90Pa*t\x8B3\x83\x88a5\"V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xD0\xE3\r\xB0\x85`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a*\xCFW`\0\x80\xFD[PZ\xF1\x15\x80\x15a*\xE3W=`\0\x80>=`\0\xFD[PPPPP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xA9\x05\x9C\xBB\x82\x86`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x83`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x82\x81R` \x01\x92PPP` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a+hW`\0\x80\xFD[PZ\xF1\x15\x80\x15a+|W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a+\x92W`\0\x80\xFD[PQa+\x9AW\xFE[\x80`\x01`\x01`\xA0\x1B\x03\x16cjbxB\x88`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x82`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x91PP` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a+\xF2W`\0\x80\xFD[PZ\xF1\x15\x80\x15a,\x06W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a,\x1CW`\0\x80\xFD[PQ\x92P4\x84\x10\x15a,4Wa,43\x854\x03a1.V[PP\x96P\x96P\x96\x93PPPPV[``\x81B\x81\x10\x15a,\x88W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R`\0\x80Q` aE\x8E\x839\x81Q\x91R`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x86\x86`\0\x81\x81\x10a,\xBFW\xFE[\x90P` \x02\x015`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x14a-\x18W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R`\0\x80Q` aD\xAB\x839\x81Q\x91R`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[a-v\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x88\x88\x88\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa8\xC5\x92PPPV[\x91P4\x82`\0\x81Q\x81\x10a-\x86W\xFE[` \x02` \x01\x01Q\x11\x15a-\xCBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`'\x81R` \x01\x80aD\x84`'\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xD0\xE3\r\xB0\x83`\0\x81Q\x81\x10a.\x07W\xFE[` \x02` \x01\x01Q`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a.:W`\0\x80\xFD[PZ\xF1\x15\x80\x15a.NW=`\0\x80>=`\0\xFD[PPPPP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xA9\x05\x9C\xBBa.\xB3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x89\x89`\0\x81\x81\x10a\x16\x9EW\xFE[\x84`\0\x81Q\x81\x10a.\xC0W\xFE[` \x02` \x01\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x83`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x82\x81R` \x01\x92PPP` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a/\x17W`\0\x80\xFD[PZ\xF1\x15\x80\x15a/+W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a/AW`\0\x80\xFD[PQa/IW\xFE[a/\x88\x82\x87\x87\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x89\x92Pa6\x7F\x91PPV[\x81`\0\x81Q\x81\x10a/\x95W\xFE[` \x02` \x01\x01Q4\x11\x15a\x1EjWa\x1Ej3\x83`\0\x81Q\x81\x10a/\xB5W\xFE[` \x02` \x01\x01Q4\x03a1.V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`$\x83\x01R`D\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`d\x90\x92\x01\x83R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\xA9\x05\x9C\xBB`\xE0\x1B\x17\x81R\x92Q\x82Q`\0\x94``\x94\x93\x89\x16\x93\x92\x91\x82\x91\x90\x80\x83\x83[` \x83\x10a0AW\x80Q\x82R`\x1F\x19\x90\x92\x01\x91` \x91\x82\x01\x91\x01a0\"V[`\x01\x83` \x03a\x01\0\n\x03\x80\x19\x82Q\x16\x81\x84Q\x16\x80\x82\x17\x85RPPPPPP\x90P\x01\x91PP`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a0\xA3W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a0\xA8V[``\x91P[P\x91P\x91P\x81\x80\x15a0\xD6WP\x80Q\x15\x80a0\xD6WP\x80\x80` \x01\x90Q` \x81\x10\x15a0\xD3W`\0\x80\xFD[PQ[a1'W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FTransferHelper: TRANSFER_FAILED\0`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[PPPPPV[`@\x80Q`\0\x80\x82R` \x82\x01\x90\x92R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x83\x90`@Q\x80\x82\x80Q\x90` \x01\x90\x80\x83\x83[` \x83\x10a1zW\x80Q\x82R`\x1F\x19\x90\x92\x01\x91` \x91\x82\x01\x91\x01a1[V[`\x01\x83` \x03a\x01\0\n\x03\x80\x19\x82Q\x16\x81\x84Q\x16\x80\x82\x17\x85RPPPPPP\x90P\x01\x91PP`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a1\xDCW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a1\xE1V[``\x91P[PP\x90P\x80a2!W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`#\x81R` \x01\x80aD\xF1`#\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[PPPV[`\0\x80\x84\x11a2fW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`+\x81R` \x01\x80aEc`+\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[`\0\x83\x11\x80\x15a2vWP`\0\x82\x11[a2\xB1W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`(\x81R` \x01\x80aD7`(\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[`\0a2\xC5\x85a\x03\xE5c\xFF\xFF\xFF\xFFaBF\x16V[\x90P`\0a2\xD9\x82\x85c\xFF\xFF\xFF\xFFaBF\x16V[\x90P`\0a2\xFF\x83a2\xF3\x88a\x03\xE8c\xFF\xFF\xFF\xFFaBF\x16V[\x90c\xFF\xFF\xFF\xFFaB\xA9\x16V[\x90P\x80\x82\x81a3\nW\xFE[\x04\x97\x96PPPPPPPV[```\x02\x82Q\x10\x15a3oW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7FUniswapV2Library: INVALID_PATH\0\0`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x80\x15a3\x87W`\0\x80\xFD[P`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a3\xB1W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P\x82\x81`\0\x81Q\x81\x10a3\xC2W\xFE[` \x02` \x01\x01\x81\x81RPP`\0[`\x01\x83Q\x03\x81\x10\x15a4ZW`\0\x80a4\x14\x87\x86\x85\x81Q\x81\x10a3\xF0W\xFE[` \x02` \x01\x01Q\x87\x86`\x01\x01\x81Q\x81\x10a4\x07W\xFE[` \x02` \x01\x01QaB\xF8V[\x91P\x91Pa46\x84\x84\x81Q\x81\x10a4'W\xFE[` \x02` \x01\x01Q\x83\x83a2&V[\x84\x84`\x01\x01\x81Q\x81\x10a4EW\xFE[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a3\xD1V[P\x93\x92PPPV[`\0\x80`\0a4q\x85\x85a>\xF4V[`@\x80Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19``\x94\x85\x1B\x81\x16` \x80\x84\x01\x91\x90\x91R\x93\x85\x1B\x81\x16`4\x83\x01R\x82Q`(\x81\x84\x03\x01\x81R`H\x83\x01\x84R\x80Q\x90\x85\x01 `\x01`\x01`\xF8\x1B\x03\x19`h\x84\x01R\x9A\x90\x94\x1B\x90\x93\x16`i\x84\x01R`}\x83\x01\x98\x90\x98R\x7F\x96\xE8\xACBw\x19\x8F\xF8\xB6\xF7\x85G\x8A\xA9\xA3\x9F@<\xB7h\xDD\x02\xCB\xEE2l>}\xA3H\x84_`\x9D\x80\x84\x01\x91\x90\x91R\x88Q\x80\x84\x03\x90\x91\x01\x81R`\xBD\x90\x92\x01\x90\x97R\x80Q\x96\x01\x95\x90\x95 \x95\x94PPPPPV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`$\x83\x01R\x84\x81\x16`D\x83\x01R`d\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`\x84\x90\x92\x01\x83R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c#\xB8r\xDD`\xE0\x1B\x17\x81R\x92Q\x82Q`\0\x94``\x94\x93\x8A\x16\x93\x92\x91\x82\x91\x90\x80\x83\x83[` \x83\x10a5\xA7W\x80Q\x82R`\x1F\x19\x90\x92\x01\x91` \x91\x82\x01\x91\x01a5\x88V[`\x01\x83` \x03a\x01\0\n\x03\x80\x19\x82Q\x16\x81\x84Q\x16\x80\x82\x17\x85RPPPPPP\x90P\x01\x91PP`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a6\tW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a6\x0EV[``\x91P[P\x91P\x91P\x81\x80\x15a6<WP\x80Q\x15\x80a6<WP\x80\x80` \x01\x90Q` \x81\x10\x15a69W`\0\x80\xFD[PQ[a6wW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`$\x81R` \x01\x80aE?`$\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[PPPPPPV[`\0[`\x01\x83Q\x03\x81\x10\x15a8\xBFW`\0\x80\x84\x83\x81Q\x81\x10a6\x9DW\xFE[` \x02` \x01\x01Q\x85\x84`\x01\x01\x81Q\x81\x10a6\xB4W\xFE[` \x02` \x01\x01Q\x91P\x91P`\0a6\xCC\x83\x83a>\xF4V[P\x90P`\0\x87\x85`\x01\x01\x81Q\x81\x10a6\xE0W\xFE[` \x02` \x01\x01Q\x90P`\0\x80\x83`\x01`\x01`\xA0\x1B\x03\x16\x86`\x01`\x01`\xA0\x1B\x03\x16\x14a7\x0EW\x82`\0a7\x12V[`\0\x83[\x91P\x91P`\0`\x02\x8AQ\x03\x88\x10a7)W\x88a7jV[a7j\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x87\x8C\x8B`\x02\x01\x81Q\x81\x10a7]W\xFE[` \x02` \x01\x01Qa4bV[\x90Pa7\x97\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x88\x88a4bV[`\x01`\x01`\xA0\x1B\x03\x16c\x02,\r\x9F\x84\x84\x84`\0`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a7\xD4W` \x82\x01\x81\x806\x837\x01\x90P[P`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x85\x81R` \x01\x84\x81R` \x01\x83`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x80` \x01\x82\x81\x03\x82R\x83\x81\x81Q\x81R` \x01\x91P\x80Q\x90` \x01\x90\x80\x83\x83`\0[\x83\x81\x10\x15a8EW\x81\x81\x01Q\x83\x82\x01R` \x01a8-V[PPPP\x90P\x90\x81\x01\x90`\x1F\x16\x80\x15a8rW\x80\x82\x03\x80Q`\x01\x83` \x03a\x01\0\n\x03\x19\x16\x81R` \x01\x91P[P\x95PPPPPP`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a8\x94W`\0\x80\xFD[PZ\xF1\x15\x80\x15a8\xA8W=`\0\x80>=`\0\xFD[PP`\x01\x90\x99\x01\x98Pa6\x82\x97PPPPPPPPV[PPPPV[```\x02\x82Q\x10\x15a9\x1EW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7FUniswapV2Library: INVALID_PATH\0\0`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x80\x15a96W`\0\x80\xFD[P`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a9`W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P\x82\x81`\x01\x83Q\x03\x81Q\x81\x10a9tW\xFE[` \x90\x81\x02\x91\x90\x91\x01\x01R\x81Q`\0\x19\x01[\x80\x15a4ZW`\0\x80a9\xB6\x87\x86`\x01\x86\x03\x81Q\x81\x10a9\xA2W\xFE[` \x02` \x01\x01Q\x87\x86\x81Q\x81\x10a4\x07W\xFE[\x91P\x91Pa9\xD8\x84\x84\x81Q\x81\x10a9\xC9W\xFE[` \x02` \x01\x01Q\x83\x83a=XV[\x84`\x01\x85\x03\x81Q\x81\x10a9\xE7W\xFE[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\0\x19\x01a9\x86V[`\0[`\x01\x83Q\x03\x81\x10\x15a2!W`\0\x80\x84\x83\x81Q\x81\x10a:\x1BW\xFE[` \x02` \x01\x01Q\x85\x84`\x01\x01\x81Q\x81\x10a:2W\xFE[` \x02` \x01\x01Q\x91P\x91P`\0a:J\x83\x83a>\xF4V[P\x90P`\0a:z\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85\x85a4bV[\x90P`\0\x80`\0\x80\x84`\x01`\x01`\xA0\x1B\x03\x16c\t\x02\xF1\xAC`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a:\xBBW`\0\x80\xFD[PZ\xFA\x15\x80\x15a:\xCFW=`\0\x80>=`\0\xFD[PPPP`@Q=``\x81\x10\x15a:\xE5W`\0\x80\xFD[P\x80Q` \x90\x91\x01Q`\x01`\x01`p\x1B\x03\x91\x82\x16\x93P\x16\x90P`\0\x80`\x01`\x01`\xA0\x1B\x03\x8A\x81\x16\x90\x89\x16\x14a;\x1BW\x82\x84a;\x1EV[\x83\x83[\x91P\x91Pa;|\x82\x8B`\x01`\x01`\xA0\x1B\x03\x16cp\xA0\x821\x8A`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x82`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x91PP` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x18\x1EW`\0\x80\xFD[\x95Pa;\x89\x86\x83\x83a2&V[\x94PPPPP`\0\x80\x85`\x01`\x01`\xA0\x1B\x03\x16\x88`\x01`\x01`\xA0\x1B\x03\x16\x14a;\xB3W\x82`\0a;\xB7V[`\0\x83[\x91P\x91P`\0`\x02\x8CQ\x03\x8A\x10a;\xCEW\x8Aa<\x02V[a<\x02\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x89\x8E\x8D`\x02\x01\x81Q\x81\x10a7]W\xFE[`@\x80Q`\0\x80\x82R` \x82\x01\x92\x83\x90Rc\x02,\r\x9F`\xE0\x1B\x83R`$\x82\x01\x87\x81R`D\x83\x01\x87\x90R`\x01`\x01`\xA0\x1B\x03\x80\x86\x16`d\x85\x01R`\x80`\x84\x85\x01\x90\x81R\x84Q`\xA4\x86\x01\x81\x90R\x96\x97P\x90\x8C\x16\x95c\x02,\r\x9F\x95\x8A\x95\x8A\x95\x8A\x95\x91\x94\x91\x93\x91\x92`\xC4\x86\x01\x92\x90\x91\x81\x90\x84\x90\x84\x90[\x83\x81\x10\x15a<\x8CW\x81\x81\x01Q\x83\x82\x01R` \x01a<tV[PPPP\x90P\x90\x81\x01\x90`\x1F\x16\x80\x15a<\xB9W\x80\x82\x03\x80Q`\x01\x83` \x03a\x01\0\n\x03\x19\x16\x81R` \x01\x91P[P\x95PPPPPP`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a<\xDBW`\0\x80\xFD[PZ\xF1\x15\x80\x15a<\xEFW=`\0\x80>=`\0\xFD[PP`\x01\x90\x9B\x01\x9APa:\0\x99PPPPPPPPPPV[\x80\x82\x03\x82\x81\x11\x15a\x11#W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rtds-math-sub-underflow`X\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\0\x80\x84\x11a=\x98W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`,\x81R` \x01\x80aC\xC0`,\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[`\0\x83\x11\x80\x15a=\xA8WP`\0\x82\x11[a=\xE3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`(\x81R` \x01\x80aD7`(\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[`\0a>\x07a\x03\xE8a=\xFB\x86\x88c\xFF\xFF\xFF\xFFaBF\x16V[\x90c\xFF\xFF\xFF\xFFaBF\x16V[\x90P`\0a>!a\x03\xE5a=\xFB\x86\x89c\xFF\xFF\xFF\xFFa=\x08\x16V[\x90Pa>>`\x01\x82\x84\x81a>1W\xFE[\x04\x90c\xFF\xFF\xFF\xFFaB\xA9\x16V[\x96\x95PPPPPPV[`\0\x80\x84\x11a>\x88W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`%\x81R` \x01\x80aD_`%\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[`\0\x83\x11\x80\x15a>\x98WP`\0\x82\x11[a>\xD3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`(\x81R` \x01\x80aD7`(\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[\x82a>\xE4\x85\x84c\xFF\xFF\xFF\xFFaBF\x16V[\x81a>\xEBW\xFE[\x04\x94\x93PPPPV[`\0\x80\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a?HW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`%\x81R` \x01\x80aC\xEC`%\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x10a?hW\x82\x84a?kV[\x83\x83[\x90\x92P\x90P`\x01`\x01`\xA0\x1B\x03\x82\x16a?\xCBW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7FUniswapV2Library: ZERO_ADDRESS\0\0`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x92P\x92\x90PV[`@\x80Qc\xE6\xA49\x05`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x88\x81\x16`\x04\x83\x01R\x87\x81\x16`$\x83\x01R\x91Q`\0\x92\x83\x92\x83\x92\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x92\x16\x91c\xE6\xA49\x05\x91`D\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a@LW`\0\x80\xFD[PZ\xFA\x15\x80\x15a@`W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a@vW`\0\x80\xFD[PQ`\x01`\x01`\xA0\x1B\x03\x16\x14\x15aA)W`@\x80Qcd\xE3)\xCB`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8A\x81\x16`\x04\x83\x01R\x89\x81\x16`$\x83\x01R\x91Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x92\x16\x91c\xC9\xC6S\x96\x91`D\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81`\0\x87\x80;\x15\x80\x15a@\xFCW`\0\x80\xFD[PZ\xF1\x15\x80\x15aA\x10W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15aA&W`\0\x80\xFD[PP[`\0\x80aAW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x8B\x8BaB\xF8V[\x91P\x91P\x81`\0\x14\x80\x15aAiWP\x80\x15[\x15aAyW\x87\x93P\x86\x92PaB9V[`\0aA\x86\x89\x84\x84a>HV[\x90P\x87\x81\x11aA\xD9W\x85\x81\x10\x15aA\xCEW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`&\x81R` \x01\x80aD\x11`&\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[\x88\x94P\x92P\x82aB7V[`\0aA\xE6\x89\x84\x86a>HV[\x90P\x89\x81\x11\x15aA\xF2W\xFE[\x87\x81\x10\x15aB1W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`&\x81R` \x01\x80aD\xCB`&\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[\x94P\x87\x93P[P[PP\x96P\x96\x94PPPPPV[`\0\x81\x15\x80aBaWPP\x80\x82\x02\x82\x82\x82\x81aB^W\xFE[\x04\x14[a\x11#W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rsds-math-mul-overflow``\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x80\x82\x01\x82\x81\x10\x15a\x11#W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rsds-math-add-overflow``\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\0\x80`\0aC\x07\x85\x85a>\xF4V[P\x90P`\0\x80aC\x18\x88\x88\x88a4bV[`\x01`\x01`\xA0\x1B\x03\x16c\t\x02\xF1\xAC`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15aCPW`\0\x80\xFD[PZ\xFA\x15\x80\x15aCdW=`\0\x80>=`\0\xFD[PPPP`@Q=``\x81\x10\x15aCzW`\0\x80\xFD[P\x80Q` \x90\x91\x01Q`\x01`\x01`p\x1B\x03\x91\x82\x16\x93P\x16\x90P`\x01`\x01`\xA0\x1B\x03\x87\x81\x16\x90\x84\x16\x14aC\xADW\x80\x82aC\xB0V[\x81\x81[\x90\x99\x90\x98P\x96PPPPPPPV\xFEUniswapV2Library: INSUFFICIENT_OUTPUT_AMOUNTUniswapV2Library: IDENTICAL_ADDRESSESUniswapV2Router: INSUFFICIENT_B_AMOUNTUniswapV2Library: INSUFFICIENT_LIQUIDITYUniswapV2Library: INSUFFICIENT_AMOUNTUniswapV2Router: EXCESSIVE_INPUT_AMOUNTUniswapV2Router: INVALID_PATH\0\0\0UniswapV2Router: INSUFFICIENT_A_AMOUNTTransferHelper: ETH_TRANSFER_FAILEDUniswapV2Router: INSUFFICIENT_OUTPUT_AMOUNTTransferHelper: TRANSFER_FROM_FAILEDUniswapV2Library: INSUFFICIENT_INPUT_AMOUNTUniswapV2Router: EXPIRED\0\0\0\0\0\0\0\0\xA2dipfsX\"\x12 \xEA\x9B\x1C\x7F\x08z\x07;\xD5\xBA\x03\xFC2\x0B]\"\xFA\x87B\x1B.\x16\xBC\x86\xA5\xEA\n\xF0\x07\x8C\n8dsolcC\0\x06\x06\x003";
    /// The deployed bytecode of the contract.
    pub static UNISWAPV2ROUTER02_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct UniswapV2Router02<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for UniswapV2Router02<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for UniswapV2Router02<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for UniswapV2Router02<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for UniswapV2Router02<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(UniswapV2Router02))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> UniswapV2Router02<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    UNISWAPV2ROUTER02_ABI.clone(),
                    client,
                ),
            )
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                UNISWAPV2ROUTER02_ABI.clone(),
                UNISWAPV2ROUTER02_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `WETH` (0xad5c4648) function
        pub fn weth(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([173, 92, 70, 72], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addLiquidity` (0xe8e33700) function
        pub fn add_liquidity(
            &self,
            token_a: ::ethers::core::types::Address,
            token_b: ::ethers::core::types::Address,
            amount_a_desired: ::ethers::core::types::U256,
            amount_b_desired: ::ethers::core::types::U256,
            amount_a_min: ::ethers::core::types::U256,
            amount_b_min: ::ethers::core::types::U256,
            to: ::ethers::core::types::Address,
            deadline: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash(
                    [232, 227, 55, 0],
                    (
                        token_a,
                        token_b,
                        amount_a_desired,
                        amount_b_desired,
                        amount_a_min,
                        amount_b_min,
                        to,
                        deadline,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addLiquidityETH` (0xf305d719) function
        pub fn add_liquidity_eth(
            &self,
            token: ::ethers::core::types::Address,
            amount_token_desired: ::ethers::core::types::U256,
            amount_token_min: ::ethers::core::types::U256,
            amount_eth_min: ::ethers::core::types::U256,
            to: ::ethers::core::types::Address,
            deadline: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash(
                    [243, 5, 215, 25],
                    (
                        token,
                        amount_token_desired,
                        amount_token_min,
                        amount_eth_min,
                        to,
                        deadline,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `factory` (0xc45a0155) function
        pub fn factory(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([196, 90, 1, 85], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAmountIn` (0x85f8c259) function
        pub fn get_amount_in(
            &self,
            amount_out: ::ethers::core::types::U256,
            reserve_in: ::ethers::core::types::U256,
            reserve_out: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([133, 248, 194, 89], (amount_out, reserve_in, reserve_out))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAmountOut` (0x054d50d4) function
        pub fn get_amount_out(
            &self,
            amount_in: ::ethers::core::types::U256,
            reserve_in: ::ethers::core::types::U256,
            reserve_out: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([5, 77, 80, 212], (amount_in, reserve_in, reserve_out))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAmountsIn` (0x1f00ca74) function
        pub fn get_amounts_in(
            &self,
            amount_out: ::ethers::core::types::U256,
            path: ::std::vec::Vec<::ethers::core::types::Address>,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::U256>,
        > {
            self.0
                .method_hash([31, 0, 202, 116], (amount_out, path))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAmountsOut` (0xd06ca61f) function
        pub fn get_amounts_out(
            &self,
            amount_in: ::ethers::core::types::U256,
            path: ::std::vec::Vec<::ethers::core::types::Address>,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::U256>,
        > {
            self.0
                .method_hash([208, 108, 166, 31], (amount_in, path))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `quote` (0xad615dec) function
        pub fn quote(
            &self,
            amount_a: ::ethers::core::types::U256,
            reserve_a: ::ethers::core::types::U256,
            reserve_b: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([173, 97, 93, 236], (amount_a, reserve_a, reserve_b))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeLiquidity` (0xbaa2abde) function
        pub fn remove_liquidity(
            &self,
            token_a: ::ethers::core::types::Address,
            token_b: ::ethers::core::types::Address,
            liquidity: ::ethers::core::types::U256,
            amount_a_min: ::ethers::core::types::U256,
            amount_b_min: ::ethers::core::types::U256,
            to: ::ethers::core::types::Address,
            deadline: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash(
                    [186, 162, 171, 222],
                    (
                        token_a,
                        token_b,
                        liquidity,
                        amount_a_min,
                        amount_b_min,
                        to,
                        deadline,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeLiquidityETH` (0x02751cec) function
        pub fn remove_liquidity_eth(
            &self,
            token: ::ethers::core::types::Address,
            liquidity: ::ethers::core::types::U256,
            amount_token_min: ::ethers::core::types::U256,
            amount_eth_min: ::ethers::core::types::U256,
            to: ::ethers::core::types::Address,
            deadline: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash(
                    [2, 117, 28, 236],
                    (token, liquidity, amount_token_min, amount_eth_min, to, deadline),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeLiquidityETHSupportingFeeOnTransferTokens` (0xaf2979eb) function
        pub fn remove_liquidity_eth_supporting_fee_on_transfer_tokens(
            &self,
            token: ::ethers::core::types::Address,
            liquidity: ::ethers::core::types::U256,
            amount_token_min: ::ethers::core::types::U256,
            amount_eth_min: ::ethers::core::types::U256,
            to: ::ethers::core::types::Address,
            deadline: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [175, 41, 121, 235],
                    (token, liquidity, amount_token_min, amount_eth_min, to, deadline),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeLiquidityETHWithPermit` (0xded9382a) function
        pub fn remove_liquidity_eth_with_permit(
            &self,
            token: ::ethers::core::types::Address,
            liquidity: ::ethers::core::types::U256,
            amount_token_min: ::ethers::core::types::U256,
            amount_eth_min: ::ethers::core::types::U256,
            to: ::ethers::core::types::Address,
            deadline: ::ethers::core::types::U256,
            approve_max: bool,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash(
                    [222, 217, 56, 42],
                    (
                        token,
                        liquidity,
                        amount_token_min,
                        amount_eth_min,
                        to,
                        deadline,
                        approve_max,
                        v,
                        r,
                        s,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeLiquidityETHWithPermitSupportingFeeOnTransferTokens` (0x5b0d5984) function
        pub fn remove_liquidity_eth_with_permit_supporting_fee_on_transfer_tokens(
            &self,
            token: ::ethers::core::types::Address,
            liquidity: ::ethers::core::types::U256,
            amount_token_min: ::ethers::core::types::U256,
            amount_eth_min: ::ethers::core::types::U256,
            to: ::ethers::core::types::Address,
            deadline: ::ethers::core::types::U256,
            approve_max: bool,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [91, 13, 89, 132],
                    (
                        token,
                        liquidity,
                        amount_token_min,
                        amount_eth_min,
                        to,
                        deadline,
                        approve_max,
                        v,
                        r,
                        s,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeLiquidityWithPermit` (0x2195995c) function
        pub fn remove_liquidity_with_permit(
            &self,
            token_a: ::ethers::core::types::Address,
            token_b: ::ethers::core::types::Address,
            liquidity: ::ethers::core::types::U256,
            amount_a_min: ::ethers::core::types::U256,
            amount_b_min: ::ethers::core::types::U256,
            to: ::ethers::core::types::Address,
            deadline: ::ethers::core::types::U256,
            approve_max: bool,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash(
                    [33, 149, 153, 92],
                    (
                        token_a,
                        token_b,
                        liquidity,
                        amount_a_min,
                        amount_b_min,
                        to,
                        deadline,
                        approve_max,
                        v,
                        r,
                        s,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `swapETHForExactTokens` (0xfb3bdb41) function
        pub fn swap_eth_for_exact_tokens(
            &self,
            amount_out: ::ethers::core::types::U256,
            path: ::std::vec::Vec<::ethers::core::types::Address>,
            to: ::ethers::core::types::Address,
            deadline: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::U256>,
        > {
            self.0
                .method_hash([251, 59, 219, 65], (amount_out, path, to, deadline))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `swapExactETHForTokens` (0x7ff36ab5) function
        pub fn swap_exact_eth_for_tokens(
            &self,
            amount_out_min: ::ethers::core::types::U256,
            path: ::std::vec::Vec<::ethers::core::types::Address>,
            to: ::ethers::core::types::Address,
            deadline: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::U256>,
        > {
            self.0
                .method_hash([127, 243, 106, 181], (amount_out_min, path, to, deadline))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `swapExactETHForTokensSupportingFeeOnTransferTokens` (0xb6f9de95) function
        pub fn swap_exact_eth_for_tokens_supporting_fee_on_transfer_tokens(
            &self,
            amount_out_min: ::ethers::core::types::U256,
            path: ::std::vec::Vec<::ethers::core::types::Address>,
            to: ::ethers::core::types::Address,
            deadline: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([182, 249, 222, 149], (amount_out_min, path, to, deadline))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `swapExactTokensForETH` (0x18cbafe5) function
        pub fn swap_exact_tokens_for_eth(
            &self,
            amount_in: ::ethers::core::types::U256,
            amount_out_min: ::ethers::core::types::U256,
            path: ::std::vec::Vec<::ethers::core::types::Address>,
            to: ::ethers::core::types::Address,
            deadline: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::U256>,
        > {
            self.0
                .method_hash(
                    [24, 203, 175, 229],
                    (amount_in, amount_out_min, path, to, deadline),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `swapExactTokensForETHSupportingFeeOnTransferTokens` (0x791ac947) function
        pub fn swap_exact_tokens_for_eth_supporting_fee_on_transfer_tokens(
            &self,
            amount_in: ::ethers::core::types::U256,
            amount_out_min: ::ethers::core::types::U256,
            path: ::std::vec::Vec<::ethers::core::types::Address>,
            to: ::ethers::core::types::Address,
            deadline: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [121, 26, 201, 71],
                    (amount_in, amount_out_min, path, to, deadline),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `swapExactTokensForTokens` (0x38ed1739) function
        pub fn swap_exact_tokens_for_tokens(
            &self,
            amount_in: ::ethers::core::types::U256,
            amount_out_min: ::ethers::core::types::U256,
            path: ::std::vec::Vec<::ethers::core::types::Address>,
            to: ::ethers::core::types::Address,
            deadline: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::U256>,
        > {
            self.0
                .method_hash(
                    [56, 237, 23, 57],
                    (amount_in, amount_out_min, path, to, deadline),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `swapExactTokensForTokensSupportingFeeOnTransferTokens` (0x5c11d795) function
        pub fn swap_exact_tokens_for_tokens_supporting_fee_on_transfer_tokens(
            &self,
            amount_in: ::ethers::core::types::U256,
            amount_out_min: ::ethers::core::types::U256,
            path: ::std::vec::Vec<::ethers::core::types::Address>,
            to: ::ethers::core::types::Address,
            deadline: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [92, 17, 215, 149],
                    (amount_in, amount_out_min, path, to, deadline),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `swapTokensForExactETH` (0x4a25d94a) function
        pub fn swap_tokens_for_exact_eth(
            &self,
            amount_out: ::ethers::core::types::U256,
            amount_in_max: ::ethers::core::types::U256,
            path: ::std::vec::Vec<::ethers::core::types::Address>,
            to: ::ethers::core::types::Address,
            deadline: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::U256>,
        > {
            self.0
                .method_hash(
                    [74, 37, 217, 74],
                    (amount_out, amount_in_max, path, to, deadline),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `swapTokensForExactTokens` (0x8803dbee) function
        pub fn swap_tokens_for_exact_tokens(
            &self,
            amount_out: ::ethers::core::types::U256,
            amount_in_max: ::ethers::core::types::U256,
            path: ::std::vec::Vec<::ethers::core::types::Address>,
            to: ::ethers::core::types::Address,
            deadline: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::U256>,
        > {
            self.0
                .method_hash(
                    [136, 3, 219, 238],
                    (amount_out, amount_in_max, path, to, deadline),
                )
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for UniswapV2Router02<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `WETH` function with signature `WETH()` and selector `0xad5c4648`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "WETH", abi = "WETH()")]
    pub struct WethCall;
    ///Container type for all input parameters for the `addLiquidity` function with signature `addLiquidity(address,address,uint256,uint256,uint256,uint256,address,uint256)` and selector `0xe8e33700`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "addLiquidity",
        abi = "addLiquidity(address,address,uint256,uint256,uint256,uint256,address,uint256)"
    )]
    pub struct AddLiquidityCall {
        pub token_a: ::ethers::core::types::Address,
        pub token_b: ::ethers::core::types::Address,
        pub amount_a_desired: ::ethers::core::types::U256,
        pub amount_b_desired: ::ethers::core::types::U256,
        pub amount_a_min: ::ethers::core::types::U256,
        pub amount_b_min: ::ethers::core::types::U256,
        pub to: ::ethers::core::types::Address,
        pub deadline: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `addLiquidityETH` function with signature `addLiquidityETH(address,uint256,uint256,uint256,address,uint256)` and selector `0xf305d719`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "addLiquidityETH",
        abi = "addLiquidityETH(address,uint256,uint256,uint256,address,uint256)"
    )]
    pub struct AddLiquidityETHCall {
        pub token: ::ethers::core::types::Address,
        pub amount_token_desired: ::ethers::core::types::U256,
        pub amount_token_min: ::ethers::core::types::U256,
        pub amount_eth_min: ::ethers::core::types::U256,
        pub to: ::ethers::core::types::Address,
        pub deadline: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `factory` function with signature `factory()` and selector `0xc45a0155`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "factory", abi = "factory()")]
    pub struct FactoryCall;
    ///Container type for all input parameters for the `getAmountIn` function with signature `getAmountIn(uint256,uint256,uint256)` and selector `0x85f8c259`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getAmountIn", abi = "getAmountIn(uint256,uint256,uint256)")]
    pub struct GetAmountInCall {
        pub amount_out: ::ethers::core::types::U256,
        pub reserve_in: ::ethers::core::types::U256,
        pub reserve_out: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getAmountOut` function with signature `getAmountOut(uint256,uint256,uint256)` and selector `0x054d50d4`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getAmountOut", abi = "getAmountOut(uint256,uint256,uint256)")]
    pub struct GetAmountOutCall {
        pub amount_in: ::ethers::core::types::U256,
        pub reserve_in: ::ethers::core::types::U256,
        pub reserve_out: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getAmountsIn` function with signature `getAmountsIn(uint256,address[])` and selector `0x1f00ca74`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getAmountsIn", abi = "getAmountsIn(uint256,address[])")]
    pub struct GetAmountsInCall {
        pub amount_out: ::ethers::core::types::U256,
        pub path: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all input parameters for the `getAmountsOut` function with signature `getAmountsOut(uint256,address[])` and selector `0xd06ca61f`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getAmountsOut", abi = "getAmountsOut(uint256,address[])")]
    pub struct GetAmountsOutCall {
        pub amount_in: ::ethers::core::types::U256,
        pub path: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all input parameters for the `quote` function with signature `quote(uint256,uint256,uint256)` and selector `0xad615dec`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "quote", abi = "quote(uint256,uint256,uint256)")]
    pub struct QuoteCall {
        pub amount_a: ::ethers::core::types::U256,
        pub reserve_a: ::ethers::core::types::U256,
        pub reserve_b: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `removeLiquidity` function with signature `removeLiquidity(address,address,uint256,uint256,uint256,address,uint256)` and selector `0xbaa2abde`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "removeLiquidity",
        abi = "removeLiquidity(address,address,uint256,uint256,uint256,address,uint256)"
    )]
    pub struct RemoveLiquidityCall {
        pub token_a: ::ethers::core::types::Address,
        pub token_b: ::ethers::core::types::Address,
        pub liquidity: ::ethers::core::types::U256,
        pub amount_a_min: ::ethers::core::types::U256,
        pub amount_b_min: ::ethers::core::types::U256,
        pub to: ::ethers::core::types::Address,
        pub deadline: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `removeLiquidityETH` function with signature `removeLiquidityETH(address,uint256,uint256,uint256,address,uint256)` and selector `0x02751cec`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "removeLiquidityETH",
        abi = "removeLiquidityETH(address,uint256,uint256,uint256,address,uint256)"
    )]
    pub struct RemoveLiquidityETHCall {
        pub token: ::ethers::core::types::Address,
        pub liquidity: ::ethers::core::types::U256,
        pub amount_token_min: ::ethers::core::types::U256,
        pub amount_eth_min: ::ethers::core::types::U256,
        pub to: ::ethers::core::types::Address,
        pub deadline: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `removeLiquidityETHSupportingFeeOnTransferTokens` function with signature `removeLiquidityETHSupportingFeeOnTransferTokens(address,uint256,uint256,uint256,address,uint256)` and selector `0xaf2979eb`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "removeLiquidityETHSupportingFeeOnTransferTokens",
        abi = "removeLiquidityETHSupportingFeeOnTransferTokens(address,uint256,uint256,uint256,address,uint256)"
    )]
    pub struct RemoveLiquidityETHSupportingFeeOnTransferTokensCall {
        pub token: ::ethers::core::types::Address,
        pub liquidity: ::ethers::core::types::U256,
        pub amount_token_min: ::ethers::core::types::U256,
        pub amount_eth_min: ::ethers::core::types::U256,
        pub to: ::ethers::core::types::Address,
        pub deadline: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `removeLiquidityETHWithPermit` function with signature `removeLiquidityETHWithPermit(address,uint256,uint256,uint256,address,uint256,bool,uint8,bytes32,bytes32)` and selector `0xded9382a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "removeLiquidityETHWithPermit",
        abi = "removeLiquidityETHWithPermit(address,uint256,uint256,uint256,address,uint256,bool,uint8,bytes32,bytes32)"
    )]
    pub struct RemoveLiquidityETHWithPermitCall {
        pub token: ::ethers::core::types::Address,
        pub liquidity: ::ethers::core::types::U256,
        pub amount_token_min: ::ethers::core::types::U256,
        pub amount_eth_min: ::ethers::core::types::U256,
        pub to: ::ethers::core::types::Address,
        pub deadline: ::ethers::core::types::U256,
        pub approve_max: bool,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    ///Container type for all input parameters for the `removeLiquidityETHWithPermitSupportingFeeOnTransferTokens` function with signature `removeLiquidityETHWithPermitSupportingFeeOnTransferTokens(address,uint256,uint256,uint256,address,uint256,bool,uint8,bytes32,bytes32)` and selector `0x5b0d5984`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "removeLiquidityETHWithPermitSupportingFeeOnTransferTokens",
        abi = "removeLiquidityETHWithPermitSupportingFeeOnTransferTokens(address,uint256,uint256,uint256,address,uint256,bool,uint8,bytes32,bytes32)"
    )]
    pub struct RemoveLiquidityETHWithPermitSupportingFeeOnTransferTokensCall {
        pub token: ::ethers::core::types::Address,
        pub liquidity: ::ethers::core::types::U256,
        pub amount_token_min: ::ethers::core::types::U256,
        pub amount_eth_min: ::ethers::core::types::U256,
        pub to: ::ethers::core::types::Address,
        pub deadline: ::ethers::core::types::U256,
        pub approve_max: bool,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    ///Container type for all input parameters for the `removeLiquidityWithPermit` function with signature `removeLiquidityWithPermit(address,address,uint256,uint256,uint256,address,uint256,bool,uint8,bytes32,bytes32)` and selector `0x2195995c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "removeLiquidityWithPermit",
        abi = "removeLiquidityWithPermit(address,address,uint256,uint256,uint256,address,uint256,bool,uint8,bytes32,bytes32)"
    )]
    pub struct RemoveLiquidityWithPermitCall {
        pub token_a: ::ethers::core::types::Address,
        pub token_b: ::ethers::core::types::Address,
        pub liquidity: ::ethers::core::types::U256,
        pub amount_a_min: ::ethers::core::types::U256,
        pub amount_b_min: ::ethers::core::types::U256,
        pub to: ::ethers::core::types::Address,
        pub deadline: ::ethers::core::types::U256,
        pub approve_max: bool,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    ///Container type for all input parameters for the `swapETHForExactTokens` function with signature `swapETHForExactTokens(uint256,address[],address,uint256)` and selector `0xfb3bdb41`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "swapETHForExactTokens",
        abi = "swapETHForExactTokens(uint256,address[],address,uint256)"
    )]
    pub struct SwapETHForExactTokensCall {
        pub amount_out: ::ethers::core::types::U256,
        pub path: ::std::vec::Vec<::ethers::core::types::Address>,
        pub to: ::ethers::core::types::Address,
        pub deadline: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `swapExactETHForTokens` function with signature `swapExactETHForTokens(uint256,address[],address,uint256)` and selector `0x7ff36ab5`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "swapExactETHForTokens",
        abi = "swapExactETHForTokens(uint256,address[],address,uint256)"
    )]
    pub struct SwapExactETHForTokensCall {
        pub amount_out_min: ::ethers::core::types::U256,
        pub path: ::std::vec::Vec<::ethers::core::types::Address>,
        pub to: ::ethers::core::types::Address,
        pub deadline: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `swapExactETHForTokensSupportingFeeOnTransferTokens` function with signature `swapExactETHForTokensSupportingFeeOnTransferTokens(uint256,address[],address,uint256)` and selector `0xb6f9de95`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "swapExactETHForTokensSupportingFeeOnTransferTokens",
        abi = "swapExactETHForTokensSupportingFeeOnTransferTokens(uint256,address[],address,uint256)"
    )]
    pub struct SwapExactETHForTokensSupportingFeeOnTransferTokensCall {
        pub amount_out_min: ::ethers::core::types::U256,
        pub path: ::std::vec::Vec<::ethers::core::types::Address>,
        pub to: ::ethers::core::types::Address,
        pub deadline: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `swapExactTokensForETH` function with signature `swapExactTokensForETH(uint256,uint256,address[],address,uint256)` and selector `0x18cbafe5`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "swapExactTokensForETH",
        abi = "swapExactTokensForETH(uint256,uint256,address[],address,uint256)"
    )]
    pub struct SwapExactTokensForETHCall {
        pub amount_in: ::ethers::core::types::U256,
        pub amount_out_min: ::ethers::core::types::U256,
        pub path: ::std::vec::Vec<::ethers::core::types::Address>,
        pub to: ::ethers::core::types::Address,
        pub deadline: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `swapExactTokensForETHSupportingFeeOnTransferTokens` function with signature `swapExactTokensForETHSupportingFeeOnTransferTokens(uint256,uint256,address[],address,uint256)` and selector `0x791ac947`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "swapExactTokensForETHSupportingFeeOnTransferTokens",
        abi = "swapExactTokensForETHSupportingFeeOnTransferTokens(uint256,uint256,address[],address,uint256)"
    )]
    pub struct SwapExactTokensForETHSupportingFeeOnTransferTokensCall {
        pub amount_in: ::ethers::core::types::U256,
        pub amount_out_min: ::ethers::core::types::U256,
        pub path: ::std::vec::Vec<::ethers::core::types::Address>,
        pub to: ::ethers::core::types::Address,
        pub deadline: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `swapExactTokensForTokens` function with signature `swapExactTokensForTokens(uint256,uint256,address[],address,uint256)` and selector `0x38ed1739`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "swapExactTokensForTokens",
        abi = "swapExactTokensForTokens(uint256,uint256,address[],address,uint256)"
    )]
    pub struct SwapExactTokensForTokensCall {
        pub amount_in: ::ethers::core::types::U256,
        pub amount_out_min: ::ethers::core::types::U256,
        pub path: ::std::vec::Vec<::ethers::core::types::Address>,
        pub to: ::ethers::core::types::Address,
        pub deadline: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `swapExactTokensForTokensSupportingFeeOnTransferTokens` function with signature `swapExactTokensForTokensSupportingFeeOnTransferTokens(uint256,uint256,address[],address,uint256)` and selector `0x5c11d795`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "swapExactTokensForTokensSupportingFeeOnTransferTokens",
        abi = "swapExactTokensForTokensSupportingFeeOnTransferTokens(uint256,uint256,address[],address,uint256)"
    )]
    pub struct SwapExactTokensForTokensSupportingFeeOnTransferTokensCall {
        pub amount_in: ::ethers::core::types::U256,
        pub amount_out_min: ::ethers::core::types::U256,
        pub path: ::std::vec::Vec<::ethers::core::types::Address>,
        pub to: ::ethers::core::types::Address,
        pub deadline: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `swapTokensForExactETH` function with signature `swapTokensForExactETH(uint256,uint256,address[],address,uint256)` and selector `0x4a25d94a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "swapTokensForExactETH",
        abi = "swapTokensForExactETH(uint256,uint256,address[],address,uint256)"
    )]
    pub struct SwapTokensForExactETHCall {
        pub amount_out: ::ethers::core::types::U256,
        pub amount_in_max: ::ethers::core::types::U256,
        pub path: ::std::vec::Vec<::ethers::core::types::Address>,
        pub to: ::ethers::core::types::Address,
        pub deadline: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `swapTokensForExactTokens` function with signature `swapTokensForExactTokens(uint256,uint256,address[],address,uint256)` and selector `0x8803dbee`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "swapTokensForExactTokens",
        abi = "swapTokensForExactTokens(uint256,uint256,address[],address,uint256)"
    )]
    pub struct SwapTokensForExactTokensCall {
        pub amount_out: ::ethers::core::types::U256,
        pub amount_in_max: ::ethers::core::types::U256,
        pub path: ::std::vec::Vec<::ethers::core::types::Address>,
        pub to: ::ethers::core::types::Address,
        pub deadline: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's call
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub enum UniswapV2Router02Calls {
        Weth(WethCall),
        AddLiquidity(AddLiquidityCall),
        AddLiquidityETH(AddLiquidityETHCall),
        Factory(FactoryCall),
        GetAmountIn(GetAmountInCall),
        GetAmountOut(GetAmountOutCall),
        GetAmountsIn(GetAmountsInCall),
        GetAmountsOut(GetAmountsOutCall),
        Quote(QuoteCall),
        RemoveLiquidity(RemoveLiquidityCall),
        RemoveLiquidityETH(RemoveLiquidityETHCall),
        RemoveLiquidityETHSupportingFeeOnTransferTokens(
            RemoveLiquidityETHSupportingFeeOnTransferTokensCall,
        ),
        RemoveLiquidityETHWithPermit(RemoveLiquidityETHWithPermitCall),
        RemoveLiquidityETHWithPermitSupportingFeeOnTransferTokens(
            RemoveLiquidityETHWithPermitSupportingFeeOnTransferTokensCall,
        ),
        RemoveLiquidityWithPermit(RemoveLiquidityWithPermitCall),
        SwapETHForExactTokens(SwapETHForExactTokensCall),
        SwapExactETHForTokens(SwapExactETHForTokensCall),
        SwapExactETHForTokensSupportingFeeOnTransferTokens(
            SwapExactETHForTokensSupportingFeeOnTransferTokensCall,
        ),
        SwapExactTokensForETH(SwapExactTokensForETHCall),
        SwapExactTokensForETHSupportingFeeOnTransferTokens(
            SwapExactTokensForETHSupportingFeeOnTransferTokensCall,
        ),
        SwapExactTokensForTokens(SwapExactTokensForTokensCall),
        SwapExactTokensForTokensSupportingFeeOnTransferTokens(
            SwapExactTokensForTokensSupportingFeeOnTransferTokensCall,
        ),
        SwapTokensForExactETH(SwapTokensForExactETHCall),
        SwapTokensForExactTokens(SwapTokensForExactTokensCall),
    }
    impl ::ethers::core::abi::AbiDecode for UniswapV2Router02Calls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <WethCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Weth(decoded));
            }
            if let Ok(decoded) = <AddLiquidityCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddLiquidity(decoded));
            }
            if let Ok(decoded) = <AddLiquidityETHCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddLiquidityETH(decoded));
            }
            if let Ok(decoded) = <FactoryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Factory(decoded));
            }
            if let Ok(decoded) = <GetAmountInCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetAmountIn(decoded));
            }
            if let Ok(decoded) = <GetAmountOutCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetAmountOut(decoded));
            }
            if let Ok(decoded) = <GetAmountsInCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetAmountsIn(decoded));
            }
            if let Ok(decoded) = <GetAmountsOutCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetAmountsOut(decoded));
            }
            if let Ok(decoded) = <QuoteCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Quote(decoded));
            }
            if let Ok(decoded) = <RemoveLiquidityCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RemoveLiquidity(decoded));
            }
            if let Ok(decoded) = <RemoveLiquidityETHCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RemoveLiquidityETH(decoded));
            }
            if let Ok(decoded) = <RemoveLiquidityETHSupportingFeeOnTransferTokensCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(
                    Self::RemoveLiquidityETHSupportingFeeOnTransferTokens(decoded),
                );
            }
            if let Ok(decoded) = <RemoveLiquidityETHWithPermitCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RemoveLiquidityETHWithPermit(decoded));
            }
            if let Ok(decoded) = <RemoveLiquidityETHWithPermitSupportingFeeOnTransferTokensCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(
                    Self::RemoveLiquidityETHWithPermitSupportingFeeOnTransferTokens(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = <RemoveLiquidityWithPermitCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RemoveLiquidityWithPermit(decoded));
            }
            if let Ok(decoded) = <SwapETHForExactTokensCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SwapETHForExactTokens(decoded));
            }
            if let Ok(decoded) = <SwapExactETHForTokensCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SwapExactETHForTokens(decoded));
            }
            if let Ok(decoded) = <SwapExactETHForTokensSupportingFeeOnTransferTokensCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(
                    Self::SwapExactETHForTokensSupportingFeeOnTransferTokens(decoded),
                );
            }
            if let Ok(decoded) = <SwapExactTokensForETHCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SwapExactTokensForETH(decoded));
            }
            if let Ok(decoded) = <SwapExactTokensForETHSupportingFeeOnTransferTokensCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(
                    Self::SwapExactTokensForETHSupportingFeeOnTransferTokens(decoded),
                );
            }
            if let Ok(decoded) = <SwapExactTokensForTokensCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SwapExactTokensForTokens(decoded));
            }
            if let Ok(decoded) = <SwapExactTokensForTokensSupportingFeeOnTransferTokensCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(
                    Self::SwapExactTokensForTokensSupportingFeeOnTransferTokens(decoded),
                );
            }
            if let Ok(decoded) = <SwapTokensForExactETHCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SwapTokensForExactETH(decoded));
            }
            if let Ok(decoded) = <SwapTokensForExactTokensCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SwapTokensForExactTokens(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for UniswapV2Router02Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Weth(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AddLiquidity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddLiquidityETH(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Factory(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetAmountIn(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetAmountOut(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetAmountsIn(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetAmountsOut(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Quote(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RemoveLiquidity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveLiquidityETH(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveLiquidityETHSupportingFeeOnTransferTokens(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveLiquidityETHWithPermit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveLiquidityETHWithPermitSupportingFeeOnTransferTokens(
                    element,
                ) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RemoveLiquidityWithPermit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SwapETHForExactTokens(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SwapExactETHForTokens(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SwapExactETHForTokensSupportingFeeOnTransferTokens(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SwapExactTokensForETH(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SwapExactTokensForETHSupportingFeeOnTransferTokens(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SwapExactTokensForTokens(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SwapExactTokensForTokensSupportingFeeOnTransferTokens(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SwapTokensForExactETH(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SwapTokensForExactTokens(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for UniswapV2Router02Calls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Weth(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddLiquidity(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddLiquidityETH(element) => ::core::fmt::Display::fmt(element, f),
                Self::Factory(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAmountIn(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAmountOut(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAmountsIn(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAmountsOut(element) => ::core::fmt::Display::fmt(element, f),
                Self::Quote(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveLiquidity(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveLiquidityETH(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RemoveLiquidityETHSupportingFeeOnTransferTokens(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RemoveLiquidityETHWithPermit(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RemoveLiquidityETHWithPermitSupportingFeeOnTransferTokens(
                    element,
                ) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveLiquidityWithPermit(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SwapETHForExactTokens(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SwapExactETHForTokens(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SwapExactETHForTokensSupportingFeeOnTransferTokens(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SwapExactTokensForETH(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SwapExactTokensForETHSupportingFeeOnTransferTokens(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SwapExactTokensForTokens(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SwapExactTokensForTokensSupportingFeeOnTransferTokens(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SwapTokensForExactETH(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SwapTokensForExactTokens(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<WethCall> for UniswapV2Router02Calls {
        fn from(value: WethCall) -> Self {
            Self::Weth(value)
        }
    }
    impl ::core::convert::From<AddLiquidityCall> for UniswapV2Router02Calls {
        fn from(value: AddLiquidityCall) -> Self {
            Self::AddLiquidity(value)
        }
    }
    impl ::core::convert::From<AddLiquidityETHCall> for UniswapV2Router02Calls {
        fn from(value: AddLiquidityETHCall) -> Self {
            Self::AddLiquidityETH(value)
        }
    }
    impl ::core::convert::From<FactoryCall> for UniswapV2Router02Calls {
        fn from(value: FactoryCall) -> Self {
            Self::Factory(value)
        }
    }
    impl ::core::convert::From<GetAmountInCall> for UniswapV2Router02Calls {
        fn from(value: GetAmountInCall) -> Self {
            Self::GetAmountIn(value)
        }
    }
    impl ::core::convert::From<GetAmountOutCall> for UniswapV2Router02Calls {
        fn from(value: GetAmountOutCall) -> Self {
            Self::GetAmountOut(value)
        }
    }
    impl ::core::convert::From<GetAmountsInCall> for UniswapV2Router02Calls {
        fn from(value: GetAmountsInCall) -> Self {
            Self::GetAmountsIn(value)
        }
    }
    impl ::core::convert::From<GetAmountsOutCall> for UniswapV2Router02Calls {
        fn from(value: GetAmountsOutCall) -> Self {
            Self::GetAmountsOut(value)
        }
    }
    impl ::core::convert::From<QuoteCall> for UniswapV2Router02Calls {
        fn from(value: QuoteCall) -> Self {
            Self::Quote(value)
        }
    }
    impl ::core::convert::From<RemoveLiquidityCall> for UniswapV2Router02Calls {
        fn from(value: RemoveLiquidityCall) -> Self {
            Self::RemoveLiquidity(value)
        }
    }
    impl ::core::convert::From<RemoveLiquidityETHCall> for UniswapV2Router02Calls {
        fn from(value: RemoveLiquidityETHCall) -> Self {
            Self::RemoveLiquidityETH(value)
        }
    }
    impl ::core::convert::From<RemoveLiquidityETHSupportingFeeOnTransferTokensCall>
    for UniswapV2Router02Calls {
        fn from(value: RemoveLiquidityETHSupportingFeeOnTransferTokensCall) -> Self {
            Self::RemoveLiquidityETHSupportingFeeOnTransferTokens(value)
        }
    }
    impl ::core::convert::From<RemoveLiquidityETHWithPermitCall>
    for UniswapV2Router02Calls {
        fn from(value: RemoveLiquidityETHWithPermitCall) -> Self {
            Self::RemoveLiquidityETHWithPermit(value)
        }
    }
    impl ::core::convert::From<
        RemoveLiquidityETHWithPermitSupportingFeeOnTransferTokensCall,
    > for UniswapV2Router02Calls {
        fn from(
            value: RemoveLiquidityETHWithPermitSupportingFeeOnTransferTokensCall,
        ) -> Self {
            Self::RemoveLiquidityETHWithPermitSupportingFeeOnTransferTokens(value)
        }
    }
    impl ::core::convert::From<RemoveLiquidityWithPermitCall>
    for UniswapV2Router02Calls {
        fn from(value: RemoveLiquidityWithPermitCall) -> Self {
            Self::RemoveLiquidityWithPermit(value)
        }
    }
    impl ::core::convert::From<SwapETHForExactTokensCall> for UniswapV2Router02Calls {
        fn from(value: SwapETHForExactTokensCall) -> Self {
            Self::SwapETHForExactTokens(value)
        }
    }
    impl ::core::convert::From<SwapExactETHForTokensCall> for UniswapV2Router02Calls {
        fn from(value: SwapExactETHForTokensCall) -> Self {
            Self::SwapExactETHForTokens(value)
        }
    }
    impl ::core::convert::From<SwapExactETHForTokensSupportingFeeOnTransferTokensCall>
    for UniswapV2Router02Calls {
        fn from(value: SwapExactETHForTokensSupportingFeeOnTransferTokensCall) -> Self {
            Self::SwapExactETHForTokensSupportingFeeOnTransferTokens(value)
        }
    }
    impl ::core::convert::From<SwapExactTokensForETHCall> for UniswapV2Router02Calls {
        fn from(value: SwapExactTokensForETHCall) -> Self {
            Self::SwapExactTokensForETH(value)
        }
    }
    impl ::core::convert::From<SwapExactTokensForETHSupportingFeeOnTransferTokensCall>
    for UniswapV2Router02Calls {
        fn from(value: SwapExactTokensForETHSupportingFeeOnTransferTokensCall) -> Self {
            Self::SwapExactTokensForETHSupportingFeeOnTransferTokens(value)
        }
    }
    impl ::core::convert::From<SwapExactTokensForTokensCall> for UniswapV2Router02Calls {
        fn from(value: SwapExactTokensForTokensCall) -> Self {
            Self::SwapExactTokensForTokens(value)
        }
    }
    impl ::core::convert::From<SwapExactTokensForTokensSupportingFeeOnTransferTokensCall>
    for UniswapV2Router02Calls {
        fn from(
            value: SwapExactTokensForTokensSupportingFeeOnTransferTokensCall,
        ) -> Self {
            Self::SwapExactTokensForTokensSupportingFeeOnTransferTokens(value)
        }
    }
    impl ::core::convert::From<SwapTokensForExactETHCall> for UniswapV2Router02Calls {
        fn from(value: SwapTokensForExactETHCall) -> Self {
            Self::SwapTokensForExactETH(value)
        }
    }
    impl ::core::convert::From<SwapTokensForExactTokensCall> for UniswapV2Router02Calls {
        fn from(value: SwapTokensForExactTokensCall) -> Self {
            Self::SwapTokensForExactTokens(value)
        }
    }
    ///Container type for all return fields from the `WETH` function with signature `WETH()` and selector `0xad5c4648`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct WethReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `addLiquidity` function with signature `addLiquidity(address,address,uint256,uint256,uint256,uint256,address,uint256)` and selector `0xe8e33700`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct AddLiquidityReturn {
        pub amount_a: ::ethers::core::types::U256,
        pub amount_b: ::ethers::core::types::U256,
        pub liquidity: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `addLiquidityETH` function with signature `addLiquidityETH(address,uint256,uint256,uint256,address,uint256)` and selector `0xf305d719`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct AddLiquidityETHReturn {
        pub amount_token: ::ethers::core::types::U256,
        pub amount_eth: ::ethers::core::types::U256,
        pub liquidity: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `factory` function with signature `factory()` and selector `0xc45a0155`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct FactoryReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getAmountIn` function with signature `getAmountIn(uint256,uint256,uint256)` and selector `0x85f8c259`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetAmountInReturn {
        pub amount_in: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getAmountOut` function with signature `getAmountOut(uint256,uint256,uint256)` and selector `0x054d50d4`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetAmountOutReturn {
        pub amount_out: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getAmountsIn` function with signature `getAmountsIn(uint256,address[])` and selector `0x1f00ca74`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetAmountsInReturn {
        pub amounts: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all return fields from the `getAmountsOut` function with signature `getAmountsOut(uint256,address[])` and selector `0xd06ca61f`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetAmountsOutReturn {
        pub amounts: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all return fields from the `quote` function with signature `quote(uint256,uint256,uint256)` and selector `0xad615dec`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct QuoteReturn {
        pub amount_b: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `removeLiquidity` function with signature `removeLiquidity(address,address,uint256,uint256,uint256,address,uint256)` and selector `0xbaa2abde`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct RemoveLiquidityReturn {
        pub amount_a: ::ethers::core::types::U256,
        pub amount_b: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `removeLiquidityETH` function with signature `removeLiquidityETH(address,uint256,uint256,uint256,address,uint256)` and selector `0x02751cec`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct RemoveLiquidityETHReturn {
        pub amount_token: ::ethers::core::types::U256,
        pub amount_eth: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `removeLiquidityETHSupportingFeeOnTransferTokens` function with signature `removeLiquidityETHSupportingFeeOnTransferTokens(address,uint256,uint256,uint256,address,uint256)` and selector `0xaf2979eb`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct RemoveLiquidityETHSupportingFeeOnTransferTokensReturn {
        pub amount_eth: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `removeLiquidityETHWithPermit` function with signature `removeLiquidityETHWithPermit(address,uint256,uint256,uint256,address,uint256,bool,uint8,bytes32,bytes32)` and selector `0xded9382a`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct RemoveLiquidityETHWithPermitReturn {
        pub amount_token: ::ethers::core::types::U256,
        pub amount_eth: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `removeLiquidityETHWithPermitSupportingFeeOnTransferTokens` function with signature `removeLiquidityETHWithPermitSupportingFeeOnTransferTokens(address,uint256,uint256,uint256,address,uint256,bool,uint8,bytes32,bytes32)` and selector `0x5b0d5984`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct RemoveLiquidityETHWithPermitSupportingFeeOnTransferTokensReturn {
        pub amount_eth: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `removeLiquidityWithPermit` function with signature `removeLiquidityWithPermit(address,address,uint256,uint256,uint256,address,uint256,bool,uint8,bytes32,bytes32)` and selector `0x2195995c`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct RemoveLiquidityWithPermitReturn {
        pub amount_a: ::ethers::core::types::U256,
        pub amount_b: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `swapETHForExactTokens` function with signature `swapETHForExactTokens(uint256,address[],address,uint256)` and selector `0xfb3bdb41`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct SwapETHForExactTokensReturn {
        pub amounts: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all return fields from the `swapExactETHForTokens` function with signature `swapExactETHForTokens(uint256,address[],address,uint256)` and selector `0x7ff36ab5`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct SwapExactETHForTokensReturn {
        pub amounts: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all return fields from the `swapExactTokensForETH` function with signature `swapExactTokensForETH(uint256,uint256,address[],address,uint256)` and selector `0x18cbafe5`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct SwapExactTokensForETHReturn {
        pub amounts: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all return fields from the `swapExactTokensForTokens` function with signature `swapExactTokensForTokens(uint256,uint256,address[],address,uint256)` and selector `0x38ed1739`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct SwapExactTokensForTokensReturn {
        pub amounts: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all return fields from the `swapTokensForExactETH` function with signature `swapTokensForExactETH(uint256,uint256,address[],address,uint256)` and selector `0x4a25d94a`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct SwapTokensForExactETHReturn {
        pub amounts: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all return fields from the `swapTokensForExactTokens` function with signature `swapTokensForExactTokens(uint256,uint256,address[],address,uint256)` and selector `0x8803dbee`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct SwapTokensForExactTokensReturn {
        pub amounts: ::std::vec::Vec<::ethers::core::types::U256>,
    }
}
