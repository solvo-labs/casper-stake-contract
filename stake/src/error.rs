use casper_types::ApiError;

#[repr(u16)]
#[derive(Clone, Copy)]
pub enum Error {
    FatalError = 0,
    AdminError = 1,
    RewardRateError = 2,
    RewardDurationError = 3,
    StakeAmountError = 4,
    CannotTargetSelfUser = 5,
    InvalidKey = 6,
}

impl From<Error> for ApiError {
    fn from(error: Error) -> ApiError {
        ApiError::User(error as u16)
    }
}