// Copyright (c) 2023 d-k-bo
// SPDX-License-Identifier: GPL-3.0-or-later

//! Low-level bindings for libcdio-paranoia.
//!
//! This crate provides function/type aliases for compatibility with
//! [cdparanoia3-sys](https://lib.rs/crates/cdparanoia3-sys), but the function
//! signatures sometimes expect a different integer type.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub use self::paranoia_cdda_enums_t::{
    CDDA_MESSAGE_FORGETIT, CDDA_MESSAGE_LOGIT, CDDA_MESSAGE_PRINTIT, CD_FRAMESAMPLES, MAXTRK,
};

#[rustfmt::skip]
pub use self::{
    cdio_cddap_find_a_cdrom         as cdda_find_a_cdrom,
    cdio_cddap_identify             as cdda_identify,
    cdio_cddap_version              as cdda_version,
    cdio_cddap_speed_set            as cdda_speed_set,
    cdio_cddap_verbose_set          as cdda_verbose_set,
    cdio_cddap_messages             as cdda_messages,
    cdio_cddap_errors               as cdda_errors,
    cdio_cddap_close                as cdda_close,
    cdio_cddap_open                 as cdda_open,
    cdio_cddap_read                 as cdda_read,
    cdio_cddap_read_timed           as cdda_read_timed,
    cdio_cddap_track_firstsector    as cdda_track_firstsector,
    cdio_cddap_track_lastsector     as cdda_track_lastsector,
    cdio_cddap_tracks               as cdda_tracks,
    cdio_cddap_sector_gettrack      as cdda_sector_gettrack,
    cdio_cddap_track_channels       as cdda_track_channels,
    cdio_cddap_track_audiop         as cdda_track_audiop,
    cdio_cddap_track_copyp          as cdda_track_copyp,
    cdio_cddap_track_preemp         as cdda_track_preemp,
    cdio_cddap_disc_firstsector     as cdda_disc_firstsector,
    cdio_cddap_disc_lastsector      as cdda_disc_lastsector,
    cdrom_drive_t                   as cdrom_drive,
};

#[rustfmt::skip]
pub use self::{
    cdrom_paranoia_t                as cdrom_paranoia,
    cdio_paranoia_version           as paranoia_version,
    cdio_paranoia_init              as paranoia_init,
    cdio_paranoia_free              as paranoia_free,
    cdio_paranoia_modeset           as paranoia_modeset,
    cdio_paranoia_seek              as paranoia_seek,
    cdio_paranoia_read              as paranoia_read,
    cdio_paranoia_read_limited      as paranoia_read_limited,
    cdio_paranoia_overlapset        as paranoia_overlapset,
    cdio_paranoia_set_range         as paranoia_set_range,
    cdio_paranoia_cachemodel_size   as paranoia_cachemodel_size,
};
