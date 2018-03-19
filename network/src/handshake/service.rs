// Copyright 2018 Kodebox, Inc.
// This file is part of CodeChain.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.


use std::sync::Arc;

use cio::{IoError, IoService};

use super::handshake::{HandlerMessage, Handler, Handshake };
use super::super::Address;

pub struct Service {
    io_service: IoService<HandlerMessage>,
    handshake: Option<Handshake>,
}

impl Service {
    pub fn start(address: Address, bootstrap_addresses: Vec<Address>) -> Result<Self, IoError> {
        let io_service = IoService::start()?;
        io_service.register_handler(Arc::new(Handler::new(address)))?;
        for address in bootstrap_addresses {
            if let Err(err) = io_service.send_message(HandlerMessage::ConnectTo(address)) {
                info!("Cannot ConnectTo : {:?}", err);
            }
        }
        Ok(Self {
            io_service,
            handshake: None,
        })
    }
}
