// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-server: server component model.
//!
//! Real implementation: server_items, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServerItem {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServerError {
    EmptyServerItemName,
    DuplicateServerItem(String),
    EmptyEventSubject,
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyServerItemName => write!(f, "server_item name must not be empty"),
            Self::DuplicateServerItem(id) => write!(f, "duplicate server_item {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for ServerError {}

pub struct Server {
    pub name: String,
    server_items: BTreeMap<String, ServerItem>,
    events: BTreeMap<String, Event>,
}

impl Server {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            server_items: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_server_item(&mut self, item: ServerItem) -> Result<(), ServerError> {
        if item.name.trim().is_empty() {
            return Err(ServerError::EmptyServerItemName);
        }
        let id = item.id.clone();
        if self.server_items.contains_key(&id) {
            return Err(ServerError::DuplicateServerItem(id.clone()));
        }
        self.server_items.insert(id, item);
        Ok(())
    }

    pub fn server_item_count(&self) -> usize {
        self.server_items.len()
    }

    pub fn get_server_item(&self, id: &str) -> Option<&ServerItem> {
        self.server_items.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), ServerError> {
        if event.subject.trim().is_empty() {
            return Err(ServerError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_server_item(id: &str, name: &str) -> ServerItem {
        ServerItem {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = Server::new("Server");
        assert_eq!(m.name, "Server");
        assert_eq!(m.server_item_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_server_item() {
        let mut m = Server::new("Server");
        m.add_server_item(sample_server_item("a1", "Alpha")).unwrap();
        assert_eq!(m.server_item_count(), 1);
        assert!(m.get_server_item("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = Server::new("Server");
        assert!(m.add_server_item(sample_server_item("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = Server::new("Server");
        m.add_server_item(sample_server_item("x", "One")).unwrap();
        assert!(m.add_server_item(sample_server_item("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = Server::new("Server");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
