pub enum HandshakePacket {
    Handshake {
        protocol_version: i32,
        server_address: String,
        server_port: u16,
        next_state: NextState,
    },
}

pub enum StatusPacket {
    Request,
    Ping { payload: u64 },
}

pub enum LoginPacket {
    LoginStart {
        name: String,
    },
    EncryptionResponse {
        shared_secret: Vec<u8>,
        verify_token: Vec<u8>,
    },
}

pub enum PlayPacket {
    ClientSettings {
        locale: String,
        view_distance: u8,
        chat_mode: i32,
        chat_colors: bool,
        displayed_skin_parts: u8,
        main_hand: i32,
    },
}

pub enum NextState {
    Status,
    Login,
}
