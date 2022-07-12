// program specific errors

use solana_program::program_error::ProgramError;
use borsh::{ BorshSerialize, BorshDeserialize };

#[derive(Clone, Debug, Eq, PartialEq, BorshSerialize, BorshDeserialize)]
pub enum CustomError {
    InstructionUnpackError,
    InstructionPackError,
    NotRentExempt,
    AlreadyInitialized,
    Uninitialized,
    InvalidNFTDataKey,
    InvalidEditionKey,
    UpdateAuthorityIncorrect,
    UpdateAuthorityIsNotSigner,
    NotMintAuthority,
    InvalidMintAuthority,
    NameTooLong,
    SymbolTooLong,
    UriTooLong,
    UpdateAuthorityMustBeEqualToNFTDataAuthorityAndSigner,
    MintMismatch,
    EditionsMustHaveExactlyOneToken,
    MaxEditionsMintedAlready,
    TokenMintToFailed,
    MasterRecordMismatch,
    DestinationMintMismatch,
    EditionAlreadyMinted,
    PrintingMintDecimalsShouldBeZero,
    OneTimePrintingAuthorizationMintDecimalsShouldBeZero,
    EditionMintDecimalsShouldBeZero,
    TokenBurnFailed,
    TokenAccountOneTimeAuthMintMismatch,
    DerivedKeyInvalid,
    PrintingMintMismatch,
    OneTimePrintingAuthMintMismatch,
    TokenAccountMintMismatch,
    TokenAccountMintMismatchV2,
    NotEnoughTokens,
    PrintingMintAuthorizationAccountMismatch,
    AuthorizationTokenAccountOwnerMismatch,
    Disabled,
    CreatorsTooLong,
    CreatorsMustBeAtleastOne,
    MustBeOneOfCreators,
    NoCreatorsPresentOnNFTData,
    CreatorNotFound,
    InvalidBasisPoints,
    PrimarySaleCanOnlyBeFlippedToTrue,
    OwnerMismatch,
    NoBalanceInAccountForAuthorization,
    ShareTotalMustBe100,
    ReservationExists,
    ReservationDoesNotExist,
    ReservationNotSet,
    ReservationAlreadyMade,
    BeyondMaxAddressSize,
    NumericalOverflowError,
    ReservationBreachesMaximumSupply,
    AddressNotInReservation,
    CannotVerifyAnotherCreator,
    CannotUnverifyAnotherCreator,
    SpotMismatch,
    IncorrectOwner,
    PrintingWouldBreachMaximumSupply,
    DataIsImmutable,
    DuplicateCreatorAddress,
    ReservationSpotsRemainingShouldMatchTotalSpotsAtStart,
    InvalidTokenProgram,
    DataTypeMismatch,
    BeyondAlottedAddressSize,
    ReservationNotComplete,
    TriedToReplaceAnExistingReservation,
    InvalidOperation,
    InvalidOwner,
    PrintingMintSupplyMustBeZeroForConversion,
    OneTimeAuthMintSupplyMustBeZeroForConversion,
    InvalidEditionIndex,
    ReservationArrayShouldBeSizeOne,
}

impl From<CustomError> for ProgramError {
    fn from(e: CustomError) -> Self {
        ProgramError::Custom(e as u32)
    }
}