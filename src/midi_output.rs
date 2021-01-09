use crate::{
    MIDIClient,
    MIDIPort,
    MIDIPortConnectionState,
    MIDIPortDeviceState,
    MIDIPortKind,
};

pub type RcRefCell<T> = std::rc::Rc<std::cell::RefCell<T>>;

fn RcRefCell<T>(v: T) -> RcRefCell<T> {
    std::rc::Rc::new(std::cell::RefCell::new(v))
}

enum MIDIOutputImpl {
    Port(RcRefCell<midir::MidiOutputPort>),
    Connection(
        RcRefCell<midir::MidiOutputPort>,
        RcRefCell<midir::MidiOutputConnection>,
    ),
}

impl MIDIOutputImpl {
    fn open(self, port_name: &str, output: midir::MidiOutput) -> Self {
        match self {
            Self::Port(port) => {
                let conn = output.connect(&port.borrow(), port_name).unwrap();
                Self::Connection(port, RcRefCell(conn))
            }
            Self::Connection(port, conn) => Self::Connection(port, conn),
        }
    }

    fn close(self) -> Self {
        match self {
            Self::Connection(port, conn) => {
                // let _output = conn.get_mut().close();
                // conn.close();
                // conn.clone().borrow().close();
                todo!();

                Self::Port(port)
            }
            Self::Port(port) => Self::Port(port),
        }
    }

    fn send(&mut self, message: &[u8]) {
        match self {
            Self::Connection(_, conn) => {
                conn.borrow_mut().send(message);
                // conn.send(message);
            }
            _ => todo!(),
        }
    }

    fn connection(&self) -> MIDIPortConnectionState {
        match self {
            Self::Port(_) => MIDIPortConnectionState::Closed,
            Self::Connection(_, _) => MIDIPortConnectionState::Open,
        }
    }
}

// #[]
pub struct MIDIOutput {
    client: MIDIClient,
    name: String,
    imp: MIDIOutputImpl, // inner: midir::MidiOutput,
                         // connection: Option<midir::MidiOutputConnection>
}

impl MIDIOutput {
    fn new(inner: midir::MidiOutput, connection: midir::MidiOutputConnection) -> Self {
        todo!()
    }

    // func send<S: Sequence>(_ data: S, offset: Timestamp = 0) -> MIDIOutput where S.Iterator.Element == UInt8
    pub fn send(&self, message: &[u8]) {
        // self.imp.open();
        todo!()
    }

    fn open1(&mut self, output: midir::MidiOutput) {
        // let output: midir::MidiOutput = todo!();
        // self.imp = self.imp.open(&self.name, output);
        // todo!()
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
        &self.name
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
        // if self.connection.is_none() {
        //     MIDIPortConnectionState::Closed
        // } else {
        //     MIDIPortConnectionState::Open
        // }
        self.imp.connection()
    }

    /// open the port, is called implicitly when MIDIInput's onMIDIMessage is set or MIDIOutputs' send is called
    fn open(&mut self) {
        let output: midir::MidiOutput = todo!();
        self.imp = self.imp.open(&self.name, output);
        // todo!()
    }

    /// closes the port
    fn close(&mut self) {
        // self.imp = self.imp.close()
        todo!()
    }
}
