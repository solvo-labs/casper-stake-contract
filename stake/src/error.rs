use casper_types::ApiError;

#[repr(u16)]
#[derive(Clone, Copy)]
pub enum Error {
    FatalError = 0,
    AdminError = 1,
    AlreadyNotified = 2,
    InsufficientBalance = 3,
    WaitingNotify = 4,
    StakeIsNotStarted = 5,
    StakeIsCompleted = 6,
    AmountIsZero = 7,
    AmountLimits = 8,
    MaxCapacityError = 9,
    InvalidKey = 10,
    StillLockPeriod = 11,
    StakeAmountIsZero = 12,
    // RewardRateError = 2,
    // RewardDurationError = 3,
    // StakeAmountError = 4,
    // CannotTargetSelfUser = 5,

    // DepositPeriodEnded = 7,
    // ExceedsMaxCapacity = 8,
    // InsufficientBalance = 9,
    // DepositPeriodNotFinished = 10,
    // InvalidUnstakeAmount = 11,
    // RewardCalculationPeriodError = 12,
    // MaxCapacityError = 13,
    // LocktimeError = 14,
}

impl From<Error> for ApiError {
    fn from(error: Error) -> ApiError {
        ApiError::User(error as u16)
    }
}
