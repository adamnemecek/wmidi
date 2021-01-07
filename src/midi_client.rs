use std::cell::RefCell;

struct MIDIClientImpl {
    pub(crate) inputs: midir::MidiInput,
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
        // let inputs = &mut self.inner.get_mut().inputs;
        // use std::pin::Pin;
        // let mut inputs = self.inner;
        // self.inner.connect(p)
        // inputs.connect(port, port_name, callback, ());
        // unsafe {
            // let mut_ref: Pin<&mut Self> = Pin::as_mut(&mut self);
            // Pin::get_unchecked_mut(mut_ref).slice = slice;
        // }
        todo!()
    }

    // pub(crate) fn connect_output(
    //     &mut self,
    // ) {
    //     todo!()
    // }
}
