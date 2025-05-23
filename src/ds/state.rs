use super::JoystickValue;

use crate::TcpPacket;
use crate::ds::state::recv::{RecvState, TcpState};
use crate::ds::state::send::SendState;
use crate::proto::udp::inbound::types::Status;
use crate::proto::udp::outbound::types::{Alliance, Control};
use std::fmt::Debug;
use tokio::sync::RwLock;

mod recv;
mod send;

type JoystickSupplier = dyn Fn() -> Vec<Vec<JoystickValue>> + Send + Sync + 'static;
type TcpConsumer = dyn FnMut(TcpPacket) + Send + Sync + 'static;

/// The operating mode of the driver station
///
/// Normal operating mode connects to the IP specified by a team number
/// Simulation mode connects to localhost, and is activated by a connection to ::1135
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DsMode {
    Normal,
    Simulation,
}

/// The core state of the driver station, containing locks over all relevant substates
pub struct DsState {
    /// The state associated with the sending UDP socket
    send_state: RwLock<SendState>,
    /// The state associated with the receiving UDP socket
    recv_state: RwLock<RecvState>,
    /// The state associated with the TCP socket
    tcp_state: RwLock<TcpState>,
}

impl DsState {
    #[inline(always)]
    pub const fn new(alliance: Alliance) -> DsState {
        let send_state = RwLock::const_new(SendState::new(alliance));
        let recv_state = RwLock::const_new(RecvState::new());
        let tcp_state = RwLock::const_new(TcpState::new());

        DsState {
            send_state,
            recv_state,
            tcp_state,
        }
    }

    #[inline(always)]
    pub const fn send(&self) -> &RwLock<SendState> {
        &self.send_state
    }

    #[inline(always)]
    pub const fn recv(&self) -> &RwLock<RecvState> {
        &self.recv_state
    }

    #[inline(always)]
    pub const fn tcp(&self) -> &RwLock<TcpState> {
        &self.tcp_state
    }
}

/// Represents the current Mode that the robot is in. the `Mode` of the robot is considered separately from whether it is enabled or not
#[derive(Copy, Clone, Debug)]
pub enum Mode {
    Autonomous,
    Teleoperated,
    Test,
}

impl Mode {
    /// Decodes the mode of the robot from the given status byte
    #[inline]
    pub const fn from_status(status: Status) -> Option<Mode> {
        if status.contains(Status::TELEOP) {
            Some(Mode::Teleoperated)
        } else if status.contains(Status::AUTO) {
            Some(Mode::Autonomous)
        } else if status.contains(Status::TEST) {
            Some(Mode::Test)
        } else {
            None
        }
    }

    /// Converts this `Mode` into a `Control` byte that can be modified for encoding the control packet.
    #[inline(always)]
    const fn to_control(self) -> Control {
        match self {
            Mode::Teleoperated => Control::TELEOP,
            Mode::Autonomous => Control::AUTO,
            Mode::Test => Control::TEST,
        }
    }
}
