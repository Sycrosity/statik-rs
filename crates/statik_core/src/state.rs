use crate::prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum State {
    Handshake = 0,
    Status = 1,
    Login = 2,
    Play = 3,
}