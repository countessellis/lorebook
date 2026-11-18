#![allow(dead_code)]

use env_logger::TimestampPrecision;
use log::LevelFilter;

use crate::logger::LogTarget;

// Build Constants:

pub(crate) const APP_NAME:    &str = "Lorebook";
pub(crate) const BUILD_NAME:  &str = env!("BUILD_NAME");
pub(crate) const VERSION_ID:  &str = env!("CARGO_PKG_VERSION");
pub(crate) const BUILD_TIME:  &str = env!("BUILD_TIME");
pub(crate) const BUILD_ID:    &str = env!("BUILD_ID");
pub(crate) const AUTHORS:     &str = env!("CARGO_PKG_AUTHORS");
pub(crate) const COPYRIGHT:   &str = "2026";
pub(crate) const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
pub(crate) const LICENSE:     &str = env!("CARGO_PKG_LICENSE");


// Logging Constants:
pub(crate) const DEFAULT_LOG_LEVEL:            LevelFilter                                 = LevelFilter::Debug; // LevelFilter::Info;
pub(crate) const DEFAULT_LOG_FORMAT_TIMESTAMP: Option<env_logger::fmt::TimestampPrecision> = Some(TimestampPrecision::Millis); // None;
pub(crate) const DEFAULT_LOG_FORMAT_LEVEL:     bool                                        = true; // false;
pub(crate) const DEFAULT_LOG_FORMAT_TARGET:    bool                                        = true; // false;
pub(crate) const DEFAULT_LOG_DIR:              &'static str                                = "logs";
pub(crate) const DEFAULT_LOG_FILE:             &'static str                                = "lorebook.log";
//pub(crate) const DEFAULT_LOG_TARGET:           env_logger::fmt::Target                     = env_logger::fmt::Target::Stdout;
pub(crate) const DEFAULT_LOG_TARGET:           LogTarget                                   = LogTarget::File(DEFAULT_LOG_DIR, DEFAULT_LOG_FILE);
pub(crate) const DEFAULT_LOG_NO_DEBUG_MODULES: &[&str]                                     = &[];
pub(crate) const REDACT_LOG_LIST:              &str                                        = "config/redact_list.txt";

// Path Constants:

pub const DEFAULT_CONFIG_DIR:  &str = "config";
pub const DEFAULT_CONFIG_FILE: &str = "lorebook.cfg";

// Lorebook Defaults:
pub(crate) const MODE_LIST:      &[&str] = &[ "cli","help"];
pub(crate) const DEFAULT_MODE:   &str = "cli";
pub(crate) const DEFAULT_WAIT:   u64  = 1;
pub(crate) const DEFAULT_INTRO:  &str = "  Long ago, when the dew of creation was fresh upon the fresh upon the heather, and the mists had not yet parted, and the stars in the skup were still few and bright...\n\n  Let us tell a tale together!\n";
pub(crate) const DEFAULT_PROMPT: &str = "[Lorebook] {}> ";


// Includes:
pub(crate) const SPLASH_COLOUR:          u8    = 109;
pub(crate) const SPLASH_PNG:             &[u8] = include_bytes!("../resources/lorebook.png");
pub(crate) const SPLASH_TXT:             &str  = include_str!("../resources/lorebook.txt");
pub(crate) const HELP_TEMPLATE:          &str  = include_str!("../resources/help.tpl");
