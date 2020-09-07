//! S3 error code

use hyper::StatusCode;
use std::fmt::{self, Display};

/// S3 error code enum
///
/// See [`ErrorResponses`](https://docs.aws.amazon.com/AmazonS3/latest/API/ErrorResponses.html)
#[derive(Debug, Clone, Copy)]
pub enum S3ErrorCode {
    /// Access Denied
    AccessDenied,

    /// There is a problem with your AWS account that prevents the operation from completing successfully.
    AccountProblem,

    /// All access to this Amazon S3 resource has been disabled.
    AllAccessDisabled,

    /// The email address you provided is associated with more than one account.
    AmbiguousGrantByEmailAddress,

    /// The authorization header you provided is invalid.
    AuthorizationHeaderMalformed,

    /// The Content-MD5 you specified did not match what we received.
    BadDigest,

    /// The requested bucket name is not available. The bucket namespace is shared by all users of the system. Please select a different name and try again.
    BucketAlreadyExists,

    /// The bucket you tried to create already exists, and you own it. Amazon S3 returns this error in all AWS Regions except us-east-1 (N. Virginia).
    /// For legacy compatibility, if you re-create an existing bucket that you already own in us-east-1, Amazon S3 returns <code class=\"code\">200 OK</code> and resets the bucket access control lists (ACLs).
    BucketAlreadyOwnedByYou,

    /// The bucket you tried to delete is not empty.
    BucketNotEmpty,

    /// This request does not support credentials.
    CredentialsNotSupported,

    /// Cross-location logging not allowed. Buckets in one geographic location cannot log information to a bucket in another location.
    CrossLocationLoggingProhibited,

    /// Your proposed upload is smaller than the minimum allowed object size.
    EntityTooSmall,

    /// Your proposed upload exceeds the maximum allowed object size.
    EntityTooLarge,

    /// The provided token has expired.
    ExpiredToken,

    /// Indicates that you are attempting to access a bucket from a different region than where the bucket exists. To avoid this error, use the <code class=\"code\">--region</code> option. For example: <code class=\"code\">aws s3 cp awsexample.txt s3://testbucket/ --region ap-east-1</code>.
    IllegalLocationConstraintException,

    /// Indicates that the versioning configuration specified in the request is invalid.
    IllegalVersioningConfigurationException,

    /// You did not provide the number of bytes specified by the Content-Length HTTP header.
    IncompleteBody,

    /// POST requires exactly one file upload per request.
    IncorrectNumberOfFilesInPostRequest,

    /// Inline data exceeds the maximum allowed size.
    InlineDataTooLarge,

    /// We encountered an internal error. Please try again.
    InternalError,

    /// The AWS access key ID you provided does not exist in our records.
    InvalidAccessKeyId,

    /// You must specify the Anonymous role.
    InvalidAddressingHeader,

    /// Invalid Argument
    InvalidArgument,

    /// The specified bucket is not valid.
    InvalidBucketName,

    /// The request is not valid with the current state of the bucket.
    InvalidBucketState,

    /// The Content-MD5 you specified is not valid.
    InvalidDigest,

    /// The encryption request you specified is not valid. The valid value is AES256.
    InvalidEncryptionAlgorithmError,

    /// The specified location constraint is not valid. For more information about Regions,
    /// see <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/dev/UsingBucket.html#access-bucket-intro\">How to Select a Region for Your Buckets</a>.
    InvalidLocationConstraint,

    /// The operation is not valid for the current state of the object.
    InvalidObjectState,

    /// One or more of the specified parts could not be found.
    /// The part might not have been uploaded, or the specified entity tag might not have matched the part's entity tag.
    InvalidPart,

    /// The list of parts was not in ascending order. Parts list must be specified in order by part number.
    InvalidPartOrder,

    /// All access to this object has been disabled.
    InvalidPayer,

    /// The content of the form does not meet the conditions specified in the policy document.
    InvalidPolicyDocument,

    /// The requested range cannot be satisfied.
    InvalidRange,

    /// Possible cases:
    /// + Please use AWS4-HMAC-SHA256.
    /// + SOAP requests must be made over an HTTPS connection.
    /// + Amazon S3 Transfer Acceleration is not supported for buckets with non-DNS compliant names.
    /// + Amazon S3 Transfer Acceleration is not supported for buckets with periods (.) in their names.
    /// + Amazon S3 Transfer Accelerate endpoint only supports virtual style requests.
    /// + Amazon S3 Transfer Accelerate is not configured on this bucket.
    /// + Amazon S3 Transfer Accelerate is disabled on this bucket.
    /// + Amazon S3 Transfer Acceleration is not supported on this bucket. Contact AWS Support for more information.
    /// + Amazon S3 Transfer Acceleration cannot be enabled on this bucket. Contact AWS Support for more information.
    InvalidRequest,

    /// The provided security credentials are not valid.
    InvalidSecurity,

    /// The SOAP request body is invalid.
    InvalidSOAPRequest,

    /// The storage class you specified is not valid.
    InvalidStorageClass,

    /// The target bucket for logging does not exist, is not owned by you, or does not have the appropriate grants for the log-delivery group.
    InvalidTargetBucketForLogging,

    /// The provided token is malformed or otherwise invalid.
    InvalidToken,

    /// Couldn't parse the specified URI.
    InvalidURI,

    /// Your key is too long.
    KeyTooLongError,

    /// The XML you provided was not well-formed or did not validate against our published schema.
    MalformedACLError,

    /// The body of your POST request is not well-formed multipart/form-data.
    MalformedPOSTRequest,

    /// This happens when the user sends malformed XML (XML that doesn't conform to the published XSD) for the configuration. The error message is, \"The XML you provided was not well-formed or did not validate against our published schema.\"
    MalformedXML,

    /// Your request was too big.
    MaxMessageLengthExceeded,

    /// Your POST request fields preceding the upload file were too large.
    MaxPostPreDataLengthExceededError,

    /// Your metadata headers exceed the maximum allowed metadata size.
    MetadataTooLarge,

    /// The specified method is not allowed against this resource.
    MethodNotAllowed,

    /// A SOAP attachment was expected, but none were found.
    MissingAttachment,

    /// You must provide the Content-Length HTTP header.
    MissingContentLength,

    /// This happens when the user sends an empty XML document as a request. The error message is, \"Request body is empty.\"
    MissingRequestBodyError,

    /// The SOAP 1.1 request is missing a security element.
    MissingSecurityElement,

    /// Your request is missing a required header.
    MissingSecurityHeader,

    /// There is no such thing as a logging status subresource for a key.
    NoLoggingStatusForKey,

    /// The specified bucket does not exist.
    NoSuchBucket,

    /// The specified bucket does not have a bucket policy.
    NoSuchBucketPolicy,

    /// The specified key does not exist.
    NoSuchKey,

    /// The lifecycle configuration does not exist.
    NoSuchLifecycleConfiguration,

    /// The specified multipart upload does not exist. The upload ID might be invalid, or the multipart upload might have been aborted or completed.
    NoSuchUpload,

    /// Indicates that the version ID specified in the request does not match an existing version.
    NoSuchVersion,

    /// A header you provided implies functionality that is not implemented.
    NotImplemented,

    /// Your account is not signed up for the Amazon S3 service. You must sign up before you can use Amazon S3.
    NotSignedUp,

    /// A conflicting conditional operation is currently in progress against this resource. Try again.
    OperationAborted,

    /// The bucket you are attempting to access must be addressed using the specified endpoint. Send all future requests to this endpoint.
    PermanentRedirect,

    /// At least one of the preconditions you specified did not hold.
    PreconditionFailed,

    /// Temporary redirect.
    Redirect,

    /// Object restore is already in progress.
    RestoreAlreadyInProgress,

    /// Bucket POST must be of the enclosure-type multipart/form-data.
    RequestIsNotMultiPartContent,

    /// Your socket connection to the server was not read from or written to within the timeout period.
    RequestTimeout,

    /// The difference between the request time and the server's time is too large.
    RequestTimeTooSkewed,

    /// Requesting the torrent file of a bucket is not permitted.
    RequestTorrentOfBucketError,

    /// The server-side encryption configuration was not found.
    ServerSideEncryptionConfigurationNotFoundError,

    /// Reduce your request rate.
    ServiceUnavailable,

    /// The request signature we calculated does not match the signature you provided.
    /// Check your AWS secret access key and signing method.
    /// For more information, see <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/dev/RESTAuthentication.html\">REST Authentication</a> and <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/dev/SOAPAuthentication.html\">SOAP Authentication</a> for details.
    SignatureDoesNotMatch,

    /// Reduce your request rate.
    SlowDown,

    /// You are being redirected to the bucket while DNS updates.
    TemporaryRedirect,

    /// The provided token must be refreshed.
    TokenRefreshRequired,

    /// You have attempted to create more buckets than allowed.
    TooManyBuckets,

    /// This request does not support content.
    UnexpectedContent,

    /// The email address you provided does not match any account on record.
    UnresolvableGrantByEmailAddress,

    /// The bucket POST must contain the specified field name. If it is specified, check the order of the fields.
    UserKeyMustBeSpecified,
}

impl Display for S3ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}
impl S3ErrorCode {
    /// Returns a corresponding status code of the error code
    #[allow(clippy::match_same_arms)]
    #[must_use]
    pub const fn as_status_code(self) -> Option<StatusCode> {
        match self {
            Self::AccessDenied => Some(StatusCode::FORBIDDEN),
            Self::AccountProblem => Some(StatusCode::FORBIDDEN),
            Self::AllAccessDisabled => Some(StatusCode::FORBIDDEN),
            Self::AmbiguousGrantByEmailAddress => Some(StatusCode::BAD_REQUEST),
            Self::AuthorizationHeaderMalformed => Some(StatusCode::BAD_REQUEST),
            Self::BadDigest => Some(StatusCode::BAD_REQUEST),
            Self::BucketAlreadyExists => Some(StatusCode::CONFLICT),
            Self::BucketAlreadyOwnedByYou => Some(StatusCode::CONFLICT),
            Self::BucketNotEmpty => Some(StatusCode::CONFLICT),
            Self::CredentialsNotSupported => Some(StatusCode::BAD_REQUEST),
            Self::CrossLocationLoggingProhibited => Some(StatusCode::FORBIDDEN),
            Self::EntityTooSmall => Some(StatusCode::BAD_REQUEST),
            Self::EntityTooLarge => Some(StatusCode::BAD_REQUEST),
            Self::ExpiredToken => Some(StatusCode::BAD_REQUEST),
            Self::IllegalLocationConstraintException => Some(StatusCode::BAD_REQUEST),
            Self::IllegalVersioningConfigurationException => Some(StatusCode::BAD_REQUEST),
            Self::IncompleteBody => Some(StatusCode::BAD_REQUEST),
            Self::IncorrectNumberOfFilesInPostRequest => Some(StatusCode::BAD_REQUEST),
            Self::InlineDataTooLarge => Some(StatusCode::BAD_REQUEST),
            Self::InternalError => Some(StatusCode::INTERNAL_SERVER_ERROR),
            Self::InvalidAccessKeyId => Some(StatusCode::FORBIDDEN),
            Self::InvalidAddressingHeader => None,
            Self::InvalidArgument => Some(StatusCode::BAD_REQUEST),
            Self::InvalidBucketName => Some(StatusCode::BAD_REQUEST),
            Self::InvalidBucketState => Some(StatusCode::CONFLICT),
            Self::InvalidDigest => Some(StatusCode::BAD_REQUEST),
            Self::InvalidEncryptionAlgorithmError => Some(StatusCode::BAD_REQUEST),
            Self::InvalidLocationConstraint => Some(StatusCode::BAD_REQUEST),
            Self::InvalidObjectState => Some(StatusCode::FORBIDDEN),
            Self::InvalidPart => Some(StatusCode::BAD_REQUEST),
            Self::InvalidPartOrder => Some(StatusCode::BAD_REQUEST),
            Self::InvalidPayer => Some(StatusCode::FORBIDDEN),
            Self::InvalidPolicyDocument => Some(StatusCode::BAD_REQUEST),
            Self::InvalidRange => Some(StatusCode::RANGE_NOT_SATISFIABLE),
            Self::InvalidRequest => Some(StatusCode::BAD_REQUEST),
            Self::InvalidSecurity => Some(StatusCode::FORBIDDEN),
            Self::InvalidSOAPRequest => Some(StatusCode::BAD_REQUEST),
            Self::InvalidStorageClass => Some(StatusCode::BAD_REQUEST),
            Self::InvalidTargetBucketForLogging => Some(StatusCode::BAD_REQUEST),
            Self::InvalidToken => Some(StatusCode::BAD_REQUEST),
            Self::InvalidURI => Some(StatusCode::BAD_REQUEST),
            Self::KeyTooLongError => Some(StatusCode::BAD_REQUEST),
            Self::MalformedACLError => Some(StatusCode::BAD_REQUEST),
            Self::MalformedPOSTRequest => Some(StatusCode::BAD_REQUEST),
            Self::MalformedXML => Some(StatusCode::BAD_REQUEST),
            Self::MaxMessageLengthExceeded => Some(StatusCode::BAD_REQUEST),
            Self::MaxPostPreDataLengthExceededError => Some(StatusCode::BAD_REQUEST),
            Self::MetadataTooLarge => Some(StatusCode::BAD_REQUEST),
            Self::MethodNotAllowed => Some(StatusCode::METHOD_NOT_ALLOWED),
            Self::MissingAttachment => None,
            Self::MissingContentLength => Some(StatusCode::LENGTH_REQUIRED),
            Self::MissingRequestBodyError => Some(StatusCode::BAD_REQUEST),
            Self::MissingSecurityElement => Some(StatusCode::BAD_REQUEST),
            Self::MissingSecurityHeader => Some(StatusCode::BAD_REQUEST),
            Self::NoLoggingStatusForKey => Some(StatusCode::BAD_REQUEST),
            Self::NoSuchBucket => Some(StatusCode::NOT_FOUND),
            Self::NoSuchBucketPolicy => Some(StatusCode::NOT_FOUND),
            Self::NoSuchKey => Some(StatusCode::NOT_FOUND),
            Self::NoSuchLifecycleConfiguration => Some(StatusCode::NOT_FOUND),
            Self::NoSuchUpload => Some(StatusCode::NOT_FOUND),
            Self::NoSuchVersion => Some(StatusCode::NOT_FOUND),
            Self::NotImplemented => Some(StatusCode::NOT_IMPLEMENTED),
            Self::NotSignedUp => Some(StatusCode::FORBIDDEN),
            Self::OperationAborted => Some(StatusCode::CONFLICT),
            Self::PermanentRedirect => Some(StatusCode::MOVED_PERMANENTLY),
            Self::PreconditionFailed => Some(StatusCode::PRECONDITION_FAILED),
            Self::Redirect => Some(StatusCode::TEMPORARY_REDIRECT),
            Self::RestoreAlreadyInProgress => Some(StatusCode::CONFLICT),
            Self::RequestIsNotMultiPartContent => Some(StatusCode::BAD_REQUEST),
            Self::RequestTimeout => Some(StatusCode::BAD_REQUEST),
            Self::RequestTimeTooSkewed => Some(StatusCode::FORBIDDEN),
            Self::RequestTorrentOfBucketError => Some(StatusCode::BAD_REQUEST),
            Self::ServerSideEncryptionConfigurationNotFoundError => Some(StatusCode::BAD_REQUEST),
            Self::ServiceUnavailable => Some(StatusCode::SERVICE_UNAVAILABLE),
            Self::SignatureDoesNotMatch => Some(StatusCode::FORBIDDEN),
            Self::SlowDown => Some(StatusCode::SERVICE_UNAVAILABLE),
            Self::TemporaryRedirect => Some(StatusCode::TEMPORARY_REDIRECT),
            Self::TokenRefreshRequired => Some(StatusCode::BAD_REQUEST),
            Self::TooManyBuckets => Some(StatusCode::BAD_REQUEST),
            Self::UnexpectedContent => Some(StatusCode::BAD_REQUEST),
            Self::UnresolvableGrantByEmailAddress => Some(StatusCode::BAD_REQUEST),
            Self::UserKeyMustBeSpecified => Some(StatusCode::BAD_REQUEST),
        }
    }
}
