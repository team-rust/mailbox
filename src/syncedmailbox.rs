use mailbox;
use std::sync::Mutex;

struct SyncedMailbox {
    inner: Mutex<mailbox::Mailbox>,
}

impl SyncedMailbox {
    fn new() -> SyncedMailbox {
        let inner = Mutex::new(mailbox::Mailbox::new());
        SyncedMailbox{inner: inner}
    }

    fn store(&self, m: mailbox::Message) -> Result<(), String> {
        match self.inner.lock() {
            Ok(mut mailbox) => Ok(mailbox.store(m)),
            Err(_) => Err("Can't open mailbox".to_string()),
        }
    }

    fn fetch(&self) -> Result<mailbox::Message, String> {
        let mut mailbox = match self.inner.lock() {
            Ok(mailbox) => mailbox,
            Err(_) => return Err("Can't access mailbox".to_string())
        };
        mailbox.fetch().ok_or("hans".to_string())
        }
}

#[cfg(test)]
mod tests {
    use super::SyncedMailbox;
    use mailbox;

    #[test]
    fn test_store() {
        let synced_mailbox = SyncedMailbox::new();
        let stored = synced_mailbox.store(mailbox::Message::new("Matthias".to_owned(), "Anna".to_owned(), "Hello".to_owned()));
        assert!(stored.is_ok())
    }

    #[test]
    fn test_fetch() {
        let synced_mailbox = SyncedMailbox::new();
        let m = mailbox::Message::new("Matthias".to_owned(), "Anna".to_owned(), "Hello".to_owned());
        let m2 = m.clone();
        synced_mailbox.store(m).unwrap();
        assert_eq!(synced_mailbox.fetch().unwrap(), m2);
    }
}
