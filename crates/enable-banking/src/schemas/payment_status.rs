use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PaymentStatus {
    /// Settlement on the creditor's account has been completed.
    #[serde(rename = "ACCC")]
    AcceptedCreditSettlementCompleted,
    /// Preceding check of technical validation was successful. Customer profile check was also successful.
    #[serde(rename = "ACCP")]
    AcceptedCustomerProfile,
    /// Cancellation is accepted.
    #[serde(rename = "ACCR")]
    AcceptedCancellationRequest,
    /// Request is accepted.
    #[serde(rename = "ACPT")]
    Accepted,
    /// Settlement on the debtor's account has been completed.
    #[serde(rename = "ACSC")]
    AcceptedSettlementCompleted,
    /// All preceding checks such as technical validation and customer profile were successful. Dynamic risk assessment is now also successful and therefore the Payment Request has been accepted for execution.
    #[serde(rename = "ACSP")]
    AcceptedSettlementInProcess,
    /// Authentication and syntactical and semantical validation are successful.
    #[serde(rename = "ACTC")]
    AcceptedTechnicalValidation,
    /// Instruction is accepted but a change will be made, such as date or remittance not sent.
    #[serde(rename = "ACWC")]
    AcceptedWithChange,
    /// Payment instruction included in the credit transfer is accepted without being posted to the creditor's account.
    #[serde(rename = "ACWP")]
    AcceptedWithoutPosting,
    /// Payment is cancelled.
    #[serde(rename = "CNCL")]
    PaymentCancelled,
    /// There is no cancellation process ongoing.
    #[serde(rename = "NULL")]
    NoCancellationProcess,
    /// Cancellation is partially accepted.
    #[serde(rename = "PACR")]
    PartiallyAcceptedCancellationRequest,
    /// A number of transactions have been accepted, whereas another number of transactions have not yet achieved 'accepted' status.
    #[serde(rename = "PART")]
    PartiallyAccepted,
    /// Cancellation request is pending.
    #[serde(rename = "PDCR")]
    PendingCancellationRequest,
    /// Payment request or individual transaction included in the Payment Request is pending. Further checks and status update will be performed.
    #[serde(rename = "PDNG")]
    Pending,
    /// Payment initiation has been received by the receiving agent.
    #[serde(rename = "RCVD")]
    Received,
    /// Cancellation request is rejected.
    #[serde(rename = "RJCR")]
    RejectedCancellationRequest,
    /// Payment request has been rejected.
    #[serde(rename = "RJCT")]
    Rejected,
}
