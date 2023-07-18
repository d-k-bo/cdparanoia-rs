// Copyright (c) 2023 d-k-bo
// SPDX-License-Identifier: GPL-3.0-or-later

use std::ffi::NulError;

use num_enum::FromPrimitive;
use num_traits::{AsPrimitive, Signed};

pub type Result<T> = std::result::Result<T, Error>;

/// The error type returned by all library functions.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("libcdio-paranoia failed to find or open a drive")]
    CantOpenDrive,
    #[error("libcdio-paranoia encountered a read error")]
    Read,
    #[error(transparent)]
    InvalidString(#[from] NulError),
    #[error(transparent)]
    Paranoia(#[from] ParanoiaError),
}

/// Error code as returned from libcdio-cdparanoia/cdparanoia-3.
#[derive(Debug, thiserror::Error, FromPrimitive)]
#[repr(u16)]
pub enum ParanoiaError {
    #[error("Unable to set CDROM to read audio mode")]
    SetToReadAudioMode = 1,
    #[error("Unable to read table of contents lead-out")]
    ReadTocLeadOut = 2,
    #[error("CDROM reporting illegal number of tracks")]
    IllegalNumberOfTracks = 3,
    #[error("Unable to read table of contents header")]
    ReadTocHeader = 4,
    #[error("Unable to read table of contents entry")]
    ReadTocEntry = 5,
    #[error("Could not read any data from drive")]
    ReadAnyData = 6,
    #[error("Unknown, unrecoverable error reading data")]
    Unknown = 7,
    #[error("Unable to identify CDROM model")]
    IdentifyModel = 8,
    #[error("CDROM reporting illegal table of contents")]
    IllegalToc = 9,
    #[error("Unaddressable sector")]
    UnaddressableSector = 10,

    #[error("Interface not supported")]
    InterfaceNotSupported = 100,
    #[error("Drive is neither a CDROM nor a WORM device")]
    InvalidDrive = 101,
    #[error("Permision denied on cdrom (ioctl) device")]
    PermissionDeniedIoctl = 102,
    #[error("Permision denied on cdrom (data) device")]
    PermissionDeniedData = 103,

    #[error("Kernel memory error")]
    KernelMemoryError = 300,

    #[error("Device not open")]
    DeviceNotOpen = 400,
    #[error("Invalid track number")]
    InvalidTrackNumber = 401,
    #[error("Track not audio data")]
    TrackNotAudioData = 402,
    #[error("No audio tracks on disc")]
    NoAudioTracks = 403,
    #[error("No medium present")]
    NoMedium = 404,
    #[error("Option not supported by drive")]
    NotSupported = 405,

    #[error("undocumented error type: {0}")]
    #[num_enum(catch_all)]
    Other(u16),
}
impl ParanoiaError {
    pub(crate) fn check_result<N>(code: N) -> std::result::Result<N, Self>
    where
        N: Signed + AsPrimitive<u16>,
    {
        if code.is_negative() {
            Err(Self::from_primitive((-code).as_()))
        } else {
            Ok(code)
        }
    }
}
