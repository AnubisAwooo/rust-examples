use tokio::sync::{Semaphore, SemaphorePermit};

struct Museum {
    remaining_tickets: Semaphore,
}

#[derive(Debug)]
struct Ticket<'a> {
    permit: SemaphorePermit<'a>,
}

impl Museum {
    fn new(permits: usize) -> Self {
        Self {
            remaining_tickets: Semaphore::new(permits),
        }
    }

    fn get_ticket(&self) -> Option<Ticket> {
        match self.remaining_tickets.try_acquire() {
            Ok(permit) => Some(Ticket { permit }),
            Err(_) => None,
        }
    }

    fn remain(&self) -> usize {
        self.remaining_tickets.available_permits()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let museum = Museum::new(50);

        let _ticket = museum.get_ticket().unwrap();

        assert_eq!(museum.remain(), 49);

        let _tickets = (0..49)
            .map(|_| museum.get_ticket().unwrap())
            .collect::<Vec<_>>();

        assert_eq!(museum.remain(), 0);

        impl PartialEq for Ticket<'_> {
            fn eq(&self, _other: &Self) -> bool {
                true
            }
        }
        assert_eq!(museum.get_ticket(), None);

        impl Drop for Ticket<'_> {
            fn drop(&mut self) {
                println!("free a ticket")
            }
        }
        drop(_ticket);

        println!("by manual");

        assert!(museum.get_ticket().is_some());

        println!("before done");
    }
}
