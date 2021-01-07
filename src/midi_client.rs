use std::cell::RefCell;

struct MIDIClientImpl {
    inputs: midir::MidiInput,
    outputs: midir::MidiOutput,
}

impl MIDIClientImpl {
    fn new(name: &str) -> Self {
        Self {
            inputs: midir::MidiInput::new(name).unwrap(),
            outputs: midir::MidiOutput::new(name).unwrap(),
        }
    }
}

#[derive(Clone)]
pub(crate) struct MIDIClient {
    inner: std::rc::Rc<RefCell<MIDIClientImpl>>,
}

impl MIDIClient {
    pub fn new() -> Self {
        todo!()
    }

    pub(crate) fn connect_input<F>(
        &mut self,
        // port: &midir::MidiInputPort,
        input: usize,
        callback: F,
        port_name: &str,
    ) -> midir::MidiInputConnection<()>
    where
        F: FnMut(u64, &[u8], &mut ()) + Send + 'static,
    {
        // let mut inputs = self.inner.borrow_mut().take();
        // inputs.connect(port, port_name, callback, ());
        todo!()
    }

    // pub(crate) fn connect_output(
    //     &mut self,
    // ) {
    //     todo!()
    // }
}
