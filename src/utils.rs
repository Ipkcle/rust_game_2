pub trait State {
    type Input;
    fn take_input(&mut self, input: Self::Input);
}
