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

    DepositPeriodEnded = 7,
    ExceedsMaxCapacity = 8,
    InsufficientBalance = 9,
    DepositPeriodNotFinished = 10,
    InvalidUnstakeAmount = 11,
    RewardCalculationPeriodError = 12,
    MaxCapacityError = 13,
    LocktimeError = 14,
    AlreadyNotified = 15,
    UnsufficientBalance = 16,
}

impl From<Error> for ApiError {
    fn from(error: Error) -> ApiError {
        ApiError::User(error as u16)
    }
}
