#[derive(Debug, PartialEq)]
pub enum FatError {
    DiskErr = 1,
    IntErr = 2,
    NotReady = 3,
    NoFile = 4,
    NoPath = 5,
    InvalidName = 6,
    Denied = 7,
    Exist = 8,
    InvalidObject = 9,
    WriteProtected = 10,
    InvalidDrive = 11,
    NotEnabled = 12,
    NoFileSystem = 13,
    MkfsAborted = 14,
    Timeout = 15,
    Locked = 16,
    NotEnoughCore = 17,
    TooManyOpenFiles = 18,
    InvalidParameter = 19,
}

impl From<u32> for FatError {
    fn from(value: u32) -> Self {
        match value {
            1 => FatError::DiskErr,
            2 => FatError::IntErr,
            3 => FatError::NotReady,
            4 => FatError::NoFile,
            5 => FatError::NoPath,
            6 => FatError::InvalidName,
            7 => FatError::Denied,
            8 => FatError::Exist,
            9 => FatError::InvalidObject,
            10 => FatError::WriteProtected,
            11 => FatError::InvalidDrive,
            12 => FatError::NotEnabled,
            13 => FatError::NoFileSystem,
            14 => FatError::MkfsAborted,
            15 => FatError::Timeout,
            16 => FatError::Locked,
            17 => FatError::NotEnoughCore,
            18 => FatError::TooManyOpenFiles,
            19 => FatError::InvalidParameter,
            _ => FatError::InvalidParameter, // Default case
        }
    }
}
