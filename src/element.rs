trait Element {
    type Message;
    fn update(&mut self, message: Message);
    fn view(&self);
}