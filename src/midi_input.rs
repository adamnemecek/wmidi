use crate::{
    MIDIClient,
    MIDIPort,
    MIDIPortConnectionState,
    MIDIPortDeviceState,
    MIDIPortKind,
};

#[derive(Clone, Copy)]
pub struct MIDIPacket {}

unsafe impl Send for MIDIPacket {}
unsafe impl Sync for MIDIPacket {}

type OnMIDIMessage = std::sync::Arc<dyn Fn(MIDIPacket) -> ()>;
pub struct MIDIInput {
    client: MIDIClient,
    inner: midir::MidiInput,
    connection: Option<midir::MidiInputConnection<()>>,
    // var onMIDIMessage: ((MIDIPacket) -> ())? = nil { get set }
    on_midi_message: Option<OnMIDIMessage>,
}

impl MIDIInput {
    fn new(client: MIDIClient, port: midir::MidiInput) -> Self {
        todo!()
    }

    pub fn set_on_midi_message(&mut self, on_midi_message: Option<OnMIDIMessage>) {
        self.on_midi_message = on_midi_message;
    }
}

impl MIDIPort for MIDIInput {
    fn id(&self) -> i32 {
        // midir::MidiInputConnection
        todo!()
    }

    fn manufacturer(&self) -> &str {
        // self.inner.manu
        todo!()
    }

    fn name(&self) -> &str {
        todo!()
    }

    /// .input (for MIDIInput) or .output (for MIDIOutput)
    fn kind(&self) -> MIDIPortKind {
        MIDIPortKind::Input
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
        todo!()
    }

    /// open the port, is called implicitly when MIDIInput's onMIDIMessage is set or MIDIOutputs' send is called
    fn open(&mut self) {
        match self.connection {
            None => {
                // self.
                // let z = self.client.connect_input();
                // let on_midi_message = self.on_midi_message.unwrap().clone();
                self.connection = Some(self.client.connect_input(
                    0,
                    |x, b, z| {
                        // on_midi_message(MIDIPacket{})
                    },
                    "cz",
                ));
            }
            _ => {}
        }
        todo!()
    }

    /// closes the port
    fn close(&mut self) {
        todo!()
    }
}
