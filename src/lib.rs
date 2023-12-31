// Copyright (c) 2023 d-k-bo
// SPDX-License-Identifier: GPL-3.0-or-later

//! High-level bindings for libcdio-paranoia/cdparanoia-3.
//!
//! By default, this library uses
//! [libcdio-paranoia](https://github.com/rocky/libcdio-paranoia)
//! under the hood. If you want to use the original
//! [cdparanoia-3](https://xiph.org/paranoia/) library
//! instead, you need to disable the default features and enable
//! `cdparanoia-3` instead.
//!
//! ```bash
//! cargo add cdparanoia --no-default-features --features cdparanoia-3
//! ```
//!
//! # Example
//!
//! The following example uses [hound](https://lib.rs/crates/hound) to write
//! the first track of a CD in a default drive to `/tmp/example.wav`.
//!
//! ```
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let drive = cdparanoia::Drive::find()?;
//! let mut paranoia = drive.paranoia();
//!
//! let mut writer = hound::WavWriter::create(
//!     "/tmp/example.wav",
//!     hound::WavSpec {
//!         channels: paranoia.drive().track_channels(1).unwrap_or(2).into(),
//!         sample_rate: 44100,
//!         bits_per_sample: 16,
//!         sample_format: hound::SampleFormat::Int,
//!     },
//! )?;
//!
//! for sector_result in paranoia.read_track(1)? {
//!     let sector = sector_result?;
//!     let mut writer = writer.get_i16_writer(sector.len() as u32);
//!     for sample in sector {
//!         writer.write_sample(sample);
//!     }
//!     writer.flush()?;
//! }
//! # Ok(())
//! # }
//! ```

use std::{ffi::CString, fmt::Debug, os::unix::prelude::OsStrExt, path::Path};

pub use crate::{
    error::{Error, ParanoiaError, Result},
    read::{DiscReader, Paranoia},
};

#[cfg(feature = "libcdio-paranoia")]
pub use cdio_paranoia_sys as ffi;

#[cfg(not(feature = "libcdio-paranoia"))]
pub use cdparanoia3_sys as ffi;

#[cfg(not(any(feature = "libcdio-paranoia", feature = "cdparanoia-3")))]
compile_error!(
    "Either feature \"libcdio-paranoia\" or \"cdparanoia-3\" must be enabled for this crate."
);

#[cfg(feature = "tracing")]
const MESSAGE_DEST: i32 = crate::ffi::CDDA_MESSAGE_LOGIT as i32;
#[cfg(not(feature = "tracing"))]
const MESSAGE_DEST: i32 = crate::ffi::CDDA_MESSAGE_PRINTIT as i32;

mod error;
mod read;

/// Represents a physical or virtual CD-ROM drive.
///
/// Use [`Drive::open()`] to get a default drive or [`Drive::open()`]
/// to get a specific drive with a CD-DA in it.
///
/// For reading audio data, get a [`Paranoia`] instance using the [`paranoia()`](Drive::paranoia) method.
#[derive(Debug)]
pub struct Drive {
    ptr: *mut crate::ffi::cdrom_drive,
}

impl Drop for Drive {
    fn drop(&mut self) {
        self.check_messages();
        unsafe { crate::ffi::cdda_close(self.ptr) };
    }
}

impl Drive {
    /// Open a default CD-ROM drive with a CD-DA in it.
    pub fn find() -> Result<Self> {
        let ptr = unsafe { crate::ffi::cdda_find_a_cdrom(MESSAGE_DEST, std::ptr::null_mut()) };
        if ptr.is_null() {
            return Err(Error::CantOpenDrive);
        }
        let drive = Drive { ptr };

        drive.check_messages();

        ParanoiaError::check_result(unsafe { crate::ffi::cdda_open(drive.as_ptr()) })?;

        drive.check_messages();

        Ok(drive)
    }
    /// Open a specific CD-ROM drive with a CD-DA in it.
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let path = CString::new(path.as_ref().as_os_str().as_bytes())?;
        let ptr =
            unsafe { crate::ffi::cdda_identify(path.as_ptr(), MESSAGE_DEST, std::ptr::null_mut()) };
        if ptr.is_null() {
            return Err(Error::CantOpenDrive);
        }
        let drive = Drive { ptr };

        drive.check_messages();

        ParanoiaError::check_result(unsafe { crate::ffi::cdda_open(drive.as_ptr()) })?;

        drive.check_messages();

        Ok(drive)
    }
}

impl Drive {
    /// Get a [`Paranoia`] instance for reading audio data.
    pub fn paranoia(self) -> Paranoia {
        Paranoia::new(self)
    }
}

impl Drive {
    /// Get the logical sector number for the start of a track.
    pub fn track_first_sector(&self, track: u8) -> Result<u32> {
        #[cfg(not(feature = "libcdio-paranoia"))]
        let track = track.into();
        let lsn = ParanoiaError::check_result(unsafe {
            crate::ffi::cdda_track_firstsector(self.as_ptr(), track)
        })? as u32;
        self.check_messages();
        Ok(lsn)
    }
    /// Get the last logical sector number of a track.
    /// This is generally one less than the start of the next track.
    pub fn track_last_sector(&self, track: u8) -> Result<u32> {
        #[cfg(not(feature = "libcdio-paranoia"))]
        let track = track.into();
        let lsn = ParanoiaError::check_result(unsafe {
            crate::ffi::cdda_track_lastsector(self.as_ptr(), track)
        })? as u32;
        self.check_messages();
        Ok(lsn)
    }
    /// Get the number of tracks on the CD.
    #[allow(clippy::let_and_return)]
    pub fn tracks(&self) -> u8 {
        let tracks = unsafe { crate::ffi::cdda_tracks(self.as_ptr()) };

        self.check_messages();

        #[cfg(not(feature = "libcdio-paranoia"))]
        let tracks = tracks.try_into().unwrap();
        tracks
    }
    /// Get the track containing the given logical sector number.
    ///
    /// If the LSN is before the first track (in the pregap), 0 is returned.
    pub fn sector_track(&self, lsn: u32) -> Result<u8> {
        #[cfg(feature = "libcdio-paranoia")]
        let lsn = lsn.try_into().unwrap();
        #[cfg(not(feature = "libcdio-paranoia"))]
        let lsn = lsn.into();

        let track = ParanoiaError::check_result(unsafe {
            crate::ffi::cdda_sector_gettrack(self.as_ptr(), lsn)
        })?;

        self.check_messages();

        #[cfg(feature = "libcdio-paranoia")]
        if track as u32 == crate::ffi::cdio_track_enums::CDIO_INVALID_TRACK {
            return Err(ParanoiaError::InvalidTrackNumber.into());
        }

        Ok(track.try_into().unwrap())
    }
    /// Get the number of channels in a track.
    ///
    /// Returns `Some(2)` or `Some(4)` on success or
    /// `None` if the value could not be retrieved.
    pub fn track_channels(&self, track: u8) -> Option<u8> {
        #[cfg(not(feature = "libcdio-paranoia"))]
        let track = track.into();
        let track_channels = unsafe { crate::ffi::cdda_track_channels(self.as_ptr(), track) }
            .try_into()
            .ok();

        self.check_messages();

        track_channels
    }
    /// Check if a track is an audio track.
    pub fn track_audio(&self, track: u8) -> bool {
        #[cfg(not(feature = "libcdio-paranoia"))]
        let track = track.into();
        let track_audio = unsafe { crate::ffi::cdda_track_audiop(self.as_ptr(), track) == 1 };

        self.check_messages();

        track_audio
    }
    /// Check if a track has copy permit set.
    pub fn track_copy_permitted(&self, track: u8) -> bool {
        #[cfg(not(feature = "libcdio-paranoia"))]
        let track = track.into();
        let track_copy_permitted =
            unsafe { crate::ffi::cdda_track_copyp(self.as_ptr(), track) == 1 };

        self.check_messages();

        track_copy_permitted
    }
    /// Check if a track has linear preemphasis set.
    ///
    /// Only makes sense for audio tracks.
    pub fn track_linear_preemphasis(&self, track: u8) -> bool {
        #[cfg(not(feature = "libcdio-paranoia"))]
        let track = track.into();
        let track_linear_preemphasis =
            unsafe { crate::ffi::cdda_track_preemp(self.as_ptr(), track) == 1 };

        self.check_messages();

        track_linear_preemphasis
    }
    /// Get the first logical sector number of the first audio track.
    pub fn disc_first_sector(&self) -> Result<u32> {
        let lsn = ParanoiaError::check_result(unsafe {
            crate::ffi::cdda_disc_firstsector(self.as_ptr())
        })? as u32;

        self.check_messages();

        Ok(lsn)
    }
    /// Get the last logical sector number of the last audio track.
    pub fn disc_last_sector(&self) -> Result<u32> {
        let lsn = ParanoiaError::check_result(unsafe {
            crate::ffi::cdda_disc_lastsector(self.as_ptr())
        })? as u32;

        self.check_messages();

        Ok(lsn)
    }
}

impl Drive {
    #[inline]
    pub fn as_ptr(&self) -> *mut crate::ffi::cdrom_drive {
        self.ptr
    }
    #[cfg(not(feature = "tracing"))]
    pub(crate) fn check_messages(&self) {}
    #[cfg(feature = "tracing")]
    pub(crate) fn check_messages(&self) {
        use std::ffi::CStr;

        use tracing::{error, info};

        unsafe {
            if let Some(errorbuf) = crate::ffi::cdda_errors(self.as_ptr()).as_mut() {
                CStr::from_ptr(errorbuf)
                    .to_string_lossy()
                    .lines()
                    .for_each(|line| error!("{line}"));

                #[cfg(feature = "libcdio-paranoia")]
                crate::ffi::cdio_cddap_free_messages(errorbuf);
                #[cfg(not(feature = "libcdio-paranoia"))]
                crate::ffi::libc::free(errorbuf as *mut std::ffi::c_char as *mut std::ffi::c_void);
            }
            if let Some(messagebuf) = crate::ffi::cdda_messages(self.as_ptr()).as_mut() {
                CStr::from_ptr(messagebuf)
                    .to_string_lossy()
                    .lines()
                    .for_each(|line| info!("{line}"));

                #[cfg(feature = "libcdio-paranoia")]
                crate::ffi::cdio_cddap_free_messages(messagebuf);
                #[cfg(not(feature = "libcdio-paranoia"))]
                crate::ffi::libc::free(
                    messagebuf as *mut std::ffi::c_char as *mut std::ffi::c_void,
                );
            }
        }
    }
}
