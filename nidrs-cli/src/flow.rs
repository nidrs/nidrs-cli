use std::any;

use metamap::Metamap;

pub trait Handler {
    fn execute(&self, payload: &mut Metamap) -> Result<(), anyhow::Error>;
    fn rollback(&self, payload: &mut Metamap) -> Result<(), anyhow::Error>;
}

pub struct Action {
    name: String,
    handler: Box<dyn Handler>,
}

impl Action {
    pub fn new<K: Into<String>, F: Handler + 'static>(name: K, handler: F) -> Self {
        Self {
            name: name.into(),
            handler: Box::new(handler),
        }
    }
}

pub struct Flow {
    name: String,
    steps: Vec<Action>,
}

impl Flow {
    pub fn new<K: Into<String>>(name: K) -> Self {
        Self {
            name: name.into(),
            steps: vec![],
        }
    }

    pub fn action(mut self, action: Action) -> Self {
        self.steps.push(action);
        self
    }

    pub fn execute(self, payload: &mut Metamap) -> Result<(), anyhow::Error> {
        for (index, step) in self.steps.iter().enumerate() {
            println!("[{}] Executing: {}", index, step.name);
            let r = step.handler.execute(payload);
            if let Err(e) = r {
                println!("[{}] Execution failed: {}", index, e);
                self.rollback(index, payload)?;
                return Err(e);
            } else {
                println!("[{}] Execution success", index);
            }
        }
        Ok(())
    }

    pub fn rollback(&self, index: usize, payload: &mut Metamap) -> Result<(), anyhow::Error> {
        for index in (0..index + 1).rev() {
            println!("[{}] Rolling: {}", index, self.steps[index].name);
            let r = self.steps[index].handler.rollback(payload);
            if r.is_err() {
                println!("[{}] Rollback failed: {}", index, r.err().unwrap());
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flow() {
        struct TestHandler;

        impl Handler for TestHandler {
            fn execute(&self, payload: &mut Metamap) -> Result<(), anyhow::Error> {
                let key: &mut String = payload.get_mut("key").unwrap();
                *key = key.to_uppercase() + "1";
                Ok(())
            }

            fn rollback(&self, payload: &mut Metamap) -> Result<(), anyhow::Error> {
                let key: &mut String = payload.get_mut("key").unwrap();
                *key = key.to_lowercase().replace("1", "");
                Ok(())
            }
        }
        struct TestHandler2;

        impl Handler for TestHandler2 {
            fn execute(&self, payload: &mut Metamap) -> Result<(), anyhow::Error> {
                let key: &mut String = payload.get_mut("key").unwrap();
                *key = key.to_string() + "2";
                Ok(())
            }

            fn rollback(&self, payload: &mut Metamap) -> Result<(), anyhow::Error> {
                let key: &mut String = payload.get_mut("key").unwrap();
                *key = key.to_lowercase().replace("2", "");
                Ok(())
            }
        }

        let flow = Flow::new("test".to_string())
            .action(Action::new("test", TestHandler))
            .action(Action::new("test2", TestHandler2));

        let mut payload = Metamap::new();
        payload.set("key", "test".to_string());
        flow.execute(&mut payload).unwrap();
        let res: &String = payload.get("key").unwrap();
        assert_eq!(res, "TEST12");
    }

    #[test]
    fn test_flow_rollback() {
        struct TestHandler;

        impl Handler for TestHandler {
            fn execute(&self, payload: &mut Metamap) -> Result<(), anyhow::Error> {
                let key: &mut String = payload.get_mut("key").unwrap();
                *key = key.to_uppercase() + "1";
                Ok(())
            }

            fn rollback(&self, payload: &mut Metamap) -> Result<(), anyhow::Error> {
                let key: &mut String = payload.get_mut("key").unwrap();
                *key = key.to_lowercase().replace("1", "");
                Ok(())
            }
        }
        struct TestHandler2;

        impl Handler for TestHandler2 {
            fn execute(&self, payload: &mut Metamap) -> Result<(), anyhow::Error> {
                let key: &mut String = payload.get_mut("key").unwrap();
                *key = key.to_string() + "2";
                Err(anyhow::anyhow!("Error"))
            }

            fn rollback(&self, payload: &mut Metamap) -> Result<(), anyhow::Error> {
                let key: &mut String = payload.get_mut("key").unwrap();
                *key = key.to_lowercase().replace("2", "");
                Ok(())
            }
        }

        let flow = Flow::new("test".to_string())
            .action(Action::new("test", TestHandler))
            .action(Action::new("test2", TestHandler2));

        let mut payload = Metamap::new();
        payload.set("key", "test".to_string());
        let r = flow.execute(&mut payload);
        let res: &String = payload.get("key").unwrap();
        assert_eq!(res, "test");
        assert!(r.is_err());
    }
}
