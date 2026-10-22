//! The web dashboard: the same numbers, as JSON and a small page, behind a secret key.

#![allow(unused_imports)]

use std::net::{Ipv4Addr, SocketAddr};
use std::sync::{Arc, Mutex};

use axum::Json;
use axum::Router;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};
use axum::routing::get;
use serde::{Deserialize, Serialize};

use crate::stat::ALL;
use crate::*;
