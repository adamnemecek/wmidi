use std::cell::RefCell;

struct MIDIClientImpl {
    name: String,
    inputs: midir::MidiInput,
    outputs: midir::MidiOutput,
}

impl MIDIClientImpl {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            inputs: midir::MidiInput::new(name).unwrap(),
            outputs: midir::MidiOutput::new(name).unwrap(),
        }
    }

    fn connect_input<F>(
        &mut self,
        port: &midir::MidiInputPort,
        // input: usize,
        callback: F,
        port_name: &str,
    ) -> midir::MidiInputConnection<()>
    where
        F: FnMut(u64, &[u8], &mut ()) + Send + 'static,
    {
        let mut input = midir::MidiInput::new(&self.name).unwrap();
        std::mem::swap(&mut self.inputs, &mut input);
        let res = input.connect(port, port_name, callback, ());
        res.unwrap()
    }

    fn connect_output(
        &mut self,
        port: &midir::MidiOutputPort,
        port_name: &str,
    ) -> midir::MidiOutputConnection {
        let mut output = midir::MidiOutput::new(&self.name).unwrap();
        std::mem::swap(&mut self.outputs, &mut output);
        let res = output.connect(port, port_name);
        res.unwrap()
    }
}

impl std::ops::Deref for MIDIClientImpl {
    type Target = midir::MidiInput;
    fn deref(&self) -> &Self::Target {
        todo!()
    }
}
impl std::ops::DerefMut for MIDIClientImpl {
    // type Target = midir::MidiInput;
    fn deref_mut(&mut self) -> &mut Self::Target {
        todo!()
    }
}

pub(crate) struct MIDIClient {
    inner: std::rc::Rc<RefCell<MIDIClientImpl>>,
    // inner: std::pin::Pin<MIDIClientImpl>,
}

impl MIDIClient {
    pub fn new() -> Self {
        todo!()
    }

    pub(crate) fn connect_input<F>(
        &mut self,
        port: &midir::MidiInputPort,
        // input: usize,
        callback: F,
        port_name: &str,
    ) -> midir::MidiInputConnection<()>
    where
        F: FnMut(u64, &[u8], &mut ()) + Send + 'static,
    {
        self.inner
            .try_borrow_mut()
            .map(|mut x| x.connect_input(port, callback, port_name))
            .unwrap()
    }

    // pub(crate) fn connect_output(
    //     &mut self,
    // ) {
    //     todo!()
    // }
}
