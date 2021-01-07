use crate::{
    MIDIPort,
    MIDIPortConnectionState,
    MIDIPortDeviceState,
    MIDIPortKind,
};

enum MIDIOutputImpl {
    Port(midir::MidiOutputPort),
    Connection(midir::MidiOutputConnection),
}

impl MIDIOutputImpl {
    fn connect(mut self, port_name: &str, output: midir::MidiOutput) {
        match self {
            Self::Port(port) => {
                let conn = output.connect(&port, port_name).unwrap();
                self = Self::Connection(conn);
            }
            _ => { }
        }
    }
}

pub struct MIDIOutput {
    inner: midir::MidiOutput,
    connection: Option<midir::MidiOutputConnection>,
}

impl MIDIOutput {
    fn new(inner: midir::MidiOutput, connection: midir::MidiOutputConnection) -> Self {
        todo!()
    }

    // func send<S: Sequence>(_ data: S, offset: Timestamp = 0) -> MIDIOutput where S.Iterator.Element == UInt8
    pub fn send(&self, message: &[u8]) {
        todo!()
    }

    fn close(&mut self) {

    }
}

impl MIDIPort for MIDIOutput {
    fn id(&self) -> i32 {
        todo!()
    }

    fn manufacturer(&self) -> &str {
        todo!()
    }

    fn name(&self) -> &str {
        todo!()
    }

    /// .input (for MIDIInput) or .output (for MIDIOutput)
    fn kind(&self) -> MIDIPortKind {
        MIDIPortKind::Output
    }

    fn version(&self) -> u32 {
        todo!()
    }

    /// .connected | .disconnected,
    /// indicates if the port's endpoint is connected or not
    fn state(&self) -> MIDIPortDeviceState {
        todo!()
    }

    /// .open | .closed
    fn connection(&self) -> MIDIPortConnectionState {
        if self.connection.is_none() {
            MIDIPortConnectionState::Closed
        } else {
            MIDIPortConnectionState::Open
        }
    }

    /// open the port, is called implicitly when MIDIInput's onMIDIMessage is set or MIDIOutputs' send is called
    fn open(&mut self) {
        todo!()
    }

    /// closes the port
    fn close(&mut self) {
        todo!()
    }
}
