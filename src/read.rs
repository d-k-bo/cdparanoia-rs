// Copyright (c) 2023 d-k-bo
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::{Drive, Error, Result};

/// Allows reading audio data from a CD.
#[derive(Debug)]
pub struct Paranoia {
    ptr: *mut crate::ffi::cdrom_paranoia,
    drive: Drive,
}

impl Drop for Paranoia {
    fn drop(&mut self) {
        unsafe { crate::ffi::paranoia_free(self.ptr) };
    }
}

impl Paranoia {
    pub(crate) fn new(drive: Drive) -> Self {
        let ptr = unsafe { crate::ffi::paranoia_init(drive.as_ptr()) };

        drive.check_messages();

        assert!(!ptr.is_null(), "paranoia_init should be infallible");
        Self { ptr, drive }
    }
}

impl From<Drive> for Paranoia {
    fn from(drive: Drive) -> Self {
        Self::new(drive)
    }
}

impl Paranoia {
    /// Get a reference to the underlying [`Drive`].
    pub fn drive(&self) -> &Drive {
        &self.drive
    }
}

impl Paranoia {
    /// Read audio data from a track.
    pub fn read_track(&mut self, track: u8) -> Result<DiscReader<'_>> {
        self.read_track_limited(track, 20)
    }
    /// Read audio data from a track with a custom retry count.
    pub fn read_track_limited(&mut self, track: u8, max_retries: i32) -> Result<DiscReader<'_>> {
        let first_lsn = self.drive.track_first_sector(track)?;
        let last_lsn = self.drive.track_last_sector(track)?;

        Ok(self.read_sectors_limited(first_lsn, last_lsn, max_retries))
    }
    /// Read a range of sectors.
    pub fn read_sectors(&mut self, first_lsn: u32, last_lsn: u32) -> DiscReader<'_> {
        self.read_sectors_limited(first_lsn, last_lsn, 20)
    }
    /// Read a range of sectors with a custom retry count.
    pub fn read_sectors_limited(
        &mut self,
        first_lsn: u32,
        last_lsn: u32,
        max_retries: i32,
    ) -> DiscReader<'_> {
        DiscReader::new(self, first_lsn, last_lsn, max_retries)
    }
}

impl Paranoia {
    pub fn as_ptr(&self) -> *mut crate::ffi::cdrom_paranoia {
        self.ptr
    }
}

/// Performs the actual reading of audio data.
///
/// This type implements
/// [`Iterator<Item = cdparanoia::Result<Vec<i16>>>`](#impl-Iterator-for-DiscReader<'drive,+'paranoia>)
/// which will clone the audio buffers. If you prefer to read the data
/// without cloning, you can use the [`next_sector()`](DiscReader::next_sector) method.
#[derive(Debug)]
pub struct DiscReader<'paranoia> {
    paranoia: &'paranoia mut Paranoia,
    last_lsn: u32,
    current_lsn: u32,
    max_retries: i32,
}

impl<'paranoia> DiscReader<'paranoia> {
    pub(crate) fn new(
        paranoia: &'paranoia mut Paranoia,
        first_lsn: u32,
        last_lsn: u32,
        max_retries: i32,
    ) -> Self {
        Self {
            paranoia,
            last_lsn,
            current_lsn: first_lsn,
            max_retries,
        }
    }
}

impl<'paranoia> DiscReader<'paranoia> {
    /// Read the next sector of audio data without cloning.
    pub fn next_sector(&mut self) -> Option<Result<&[i16]>> {
        if self.current_lsn == self.last_lsn {
            return None;
        }

        let data = unsafe {
            let ptr =
                crate::ffi::paranoia_read_limited(self.paranoia.as_ptr(), None, self.max_retries);

            self.paranoia.drive.check_messages();

            if ptr.is_null() {
                return Some(Err(Error::Read));
            }

            std::slice::from_raw_parts(ptr, crate::ffi::CD_FRAMEWORDS as usize)
        };
        self.current_lsn += 1;

        Some(Ok(data))
    }
}

impl<'paranoia> Iterator for DiscReader<'paranoia> {
    type Item = Result<Vec<i16>>;

    /// Read the next sector of audio data.
    ///
    /// Due to constraints of the [`Iterator`] trait, this requires cloning the data.
    fn next(&mut self) -> Option<Self::Item> {
        self.next_sector().map(|res| res.map(<[i16]>::to_vec))
    }
}
