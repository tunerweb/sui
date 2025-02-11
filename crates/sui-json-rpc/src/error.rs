// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use fastcrypto::error::FastCryptoError;
use hyper::header::InvalidHeaderValue;
use jsonrpsee::core::Error as RpcError;
use jsonrpsee::types::error::CallError;
use sui_types::error::{SuiError, SuiObjectResponseError, UserInputError};
use sui_types::quorum_driver_types::QuorumDriverError;
use thiserror::Error;
use tokio::task::JoinError;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    SuiError(#[from] SuiError),

    #[error(transparent)]
    InternalError(#[from] anyhow::Error),

    #[error("Deserialization error: {0}")]
    BcsError(#[from] bcs::Error),

    #[error("Unexpected error: {0}")]
    UnexpectedError(String),

    #[error(transparent)]
    RPCServerError(#[from] jsonrpsee::core::Error),

    #[error(transparent)]
    InvalidHeaderValue(#[from] InvalidHeaderValue),

    #[error(transparent)]
    UserInputError(#[from] UserInputError),

    #[error(transparent)]
    EncodingError(#[from] eyre::Report),

    #[error(transparent)]
    TokioJoinError(#[from] JoinError),

    #[error(transparent)]
    QuorumDriverError(#[from] QuorumDriverError),

    #[error(transparent)]
    FastCryptoError(#[from] FastCryptoError),

    #[error(transparent)]
    SuiObjectResponseError(#[from] SuiObjectResponseError),
}

impl From<Error> for RpcError {
    fn from(e: Error) -> Self {
        e.to_rpc_error()
    }
}

impl Error {
    pub fn to_rpc_error(self) -> RpcError {
        match self {
            Error::UserInputError(user_input_error) => {
                RpcError::Call(CallError::InvalidParams(user_input_error.into()))
            }
            Error::SuiError(sui_error) => match sui_error {
                SuiError::TransactionNotFound { .. } | SuiError::TransactionsNotFound { .. } => {
                    RpcError::Call(CallError::InvalidParams(sui_error.into()))
                }
                _ => RpcError::Call(CallError::Failed(sui_error.into())),
            },
            _ => RpcError::Call(CallError::Failed(self.into())),
        }
    }
}
