use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ErrorCode {
    /// Access to this resource is denied. Check services available for your application.
    AccessDenied, 
    /// No account found matching provided id
    AccountDoesNotExist, 
    /// Session is already authorized
    AlreadyAuthorized, 
    /// The PSU does not have access to the requested account or it doesn't exist
    AspspAccountNotAccessible, 
    /// Error interacting with ASPSP
    AspspError, 
    /// Payment can not be requested from the ASPSP
    AspspPaymentNotAccessible, 
    /// PSU action is required to proceed
    AspspPsuActionRequired, 
    /// ASPSP Rate limit exceeded
    AspspRateLimitExceeded, 
    /// Timeout interacting with ASPSP
    AspspTimeout, 
    /// Authorization header is not provided
    AuthorizationNotProvided, 
    /// Session is closed
    ClosedSession, 
    /// date_from can not be in the future
    DateFromInFuture, 
    /// date_from must be provided if date_to provided
    DateToWithoutDateFrom, 
    /// Authorization code is expired
    ExpiredAuthorizationCode, 
    /// Session is expired
    ExpiredSession, 
    /// Either iban or other account identification is required
    InvalidAccountId, 
    /// Invalid host
    InvalidHost, 
    /// Invalid or expired payment provided
    InvalidPayment, 
    /// No allowed accounts added to the application
    NoAccountsAdded, 
    /// The amount value or the the number of transactions exceeds the limit
    PaymentLimitExceeded, 
    /// You can not delete a payment that is not finalized or cancelled
    PaymentNotFinalized, 
    /// Payment not found
    PaymentNotFound, 
    /// Provided PSU header contains invalid value
    PsuHeaderInvalid, 
    /// Required PSU header is not provided
    PsuHeaderNotProvided, 
    /// Redirect URI not allowed
    RedirectUriNotAllowed, 
    /// Session is revoked
    RevokedSession, 
    /// No session found matching provided id
    SessionDoesNotExist, 
    /// No transaction found matching provided id
    TransactionDoesNotExist, 
    /// Unauthorized access
    UnauthorizedAccess, 
    /// Used IP address is not authorized to access the resource
    UnauthorizedIp, 
    /// Either creditor or debtor account is not trusted
    UntrustedPaymentParty, 
    /// Webhook URI not allowed
    WebhookUriNotAllowed, 
    /// Wrong ASPSP name provided
    WrongAspspProvided, 
    /// Wrong authorization code provided
    WrongAuthorizationCode, 
    /// Wrong continuation key provided
    WrongContinuationKey, 
    /// Wrong credentials provided
    WrongCredentialsProvided, 
    /// date_from should be less than or equal date_to
    WrongDateInterval, 
    /// Wrong request parameters provided
    WrongRequestParameters, 
    /// Wrong session status
    WrongSessionStatus, 
    /// Wrong transactions period requested
    WrongTransactionsPeriod, 
}