#[derive(Debug, PartialEq, Clone)]
pub struct Message {
    sender: String,
    receiver: String,
    body: String
}

impl Message {
    pub fn new(sender: String, receiver: String, body: String) -> Message {
        Message{sender: sender, receiver: receiver, body: body}
    }
}

#[derive(Debug, PartialEq)]
pub struct Mailbox {
    messages: Vec<Message>
}

impl Mailbox {
    pub fn new() -> Mailbox {
        Mailbox{messages: vec![]}
    }

    pub fn store(&mut self, m: Message) {
        self.messages.push(m)
    }

    pub fn fetch(&mut self) -> Option<Message> {
        self.messages.last().cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::Mailbox;
    use super::Message;

    #[test]
    fn test_store() {
        let mut mailbox = Mailbox::new();
        let m = Message::new("Matthias".to_owned(), "Anna".to_owned(), "Hello".to_owned());
        assert_eq!(mailbox.messages.len(), 0);
        mailbox.store(m);
        assert_eq!(mailbox.messages.len(), 1);
    }

    #[test]
    fn test_fetch() {
        let mut mailbox = Mailbox::new();
        let m = Message::new("Matthias".to_owned(), "Anna".to_owned(), "Hello".to_owned());
        let m2 = m.clone();
        mailbox.store(m);
        assert_eq!(mailbox.fetch().unwrap(), m2);
    }
}
