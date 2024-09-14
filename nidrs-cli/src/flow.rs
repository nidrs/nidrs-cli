use std::any;

#[derive(Debug)]
pub struct HandlerError(Box<dyn any::Any>, anyhow::Error);

pub trait Handler {
    fn run(&self, payload: Box<dyn any::Any>) -> Result<Box<dyn any::Any>, HandlerError>;
    fn rollback(&self, payload: Box<dyn any::Any>) -> Result<Box<dyn any::Any>, HandlerError>;
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

    pub fn run(
        self,
        mut payload: Option<Box<dyn any::Any>>,
    ) -> Result<Option<Box<dyn any::Any>>, anyhow::Error> {
        for (index, step) in self.steps.iter().enumerate() {
            println!("[{}] Running: {}", index, step.name);
            let r = step.handler.run(payload.take().unwrap());
            if let Err(e) = r {
                println!("[{}] Error: {}", index, e.1);
                payload = Some(e.0);
                return self.rollback(index, payload);
            } else if let Ok(v) = r {
                payload = Some(v);
            }
        }
        Ok(payload)
    }

    pub fn rollback(
        &self,
        index: usize,
        mut payload: Option<Box<dyn any::Any>>,
    ) -> Result<Option<Box<dyn any::Any>>, anyhow::Error> {
        for index in (0..index + 1).rev() {
            println!("[{}] Rolling: {}", index, self.steps[index].name);
            let res = self.steps[index]
                .handler
                .rollback(payload.take().unwrap())
                .unwrap();
            payload = Some(res);
        }
        Ok(payload)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flow() {
        struct TestHandler;

        impl Handler for TestHandler {
            fn run(&self, payload: Box<dyn any::Any>) -> Result<Box<dyn any::Any>, HandlerError> {
                let payload = payload.downcast::<String>().unwrap();
                let payload = payload.to_uppercase() + "1";
                Ok(Box::new(payload))
            }

            fn rollback(
                &self,
                payload: Box<dyn any::Any>,
            ) -> Result<Box<dyn any::Any>, HandlerError> {
                let payload = payload.downcast::<String>().unwrap();
                let payload = payload.to_lowercase();
                Ok(Box::new(payload))
            }
        }
        struct TestHandler2;

        impl Handler for TestHandler2 {
            fn run(&self, payload: Box<dyn any::Any>) -> Result<Box<dyn any::Any>, HandlerError> {
                let payload = payload.downcast::<String>().unwrap();
                let payload = payload.to_uppercase() + "2";
                Ok(Box::new(payload))
            }

            fn rollback(
                &self,
                payload: Box<dyn any::Any>,
            ) -> Result<Box<dyn any::Any>, HandlerError> {
                let payload = payload.downcast::<String>().unwrap();
                let payload = payload.to_lowercase();
                Ok(Box::new(payload))
            }
        }

        let flow = Flow::new("test".to_string())
            .action(Action {
                name: "test".to_string(),
                handler: Box::new(TestHandler),
            })
            .action(Action {
                name: "test2".to_string(),
                handler: Box::new(TestHandler2),
            });

        let payload = Some(Box::new("test".to_string()) as Box<dyn any::Any>);
        let res = flow.run(payload).unwrap().unwrap();
        let res = *res.downcast::<String>().unwrap();
        assert_eq!(res, "TEST12".to_string());
    }

    #[test]
    fn test_flow_rollback() {
        struct TestHandler;

        impl Handler for TestHandler {
            fn run(&self, payload: Box<dyn any::Any>) -> Result<Box<dyn any::Any>, HandlerError> {
                let payload = payload.downcast::<String>().unwrap();
                let payload = payload.to_uppercase() + "1";
                Ok(Box::new(payload))
            }

            fn rollback(
                &self,
                payload: Box<dyn any::Any>,
            ) -> Result<Box<dyn any::Any>, HandlerError> {
                let payload = payload.downcast::<String>().unwrap();
                let payload = payload.to_lowercase();
                let payload = payload.replace("1", "");
                Ok(Box::new(payload))
            }
        }
        struct TestHandler2;

        impl Handler for TestHandler2 {
            fn run(&self, payload: Box<dyn any::Any>) -> Result<Box<dyn any::Any>, HandlerError> {
                Err(HandlerError(payload, anyhow::anyhow!("test error")))
            }

            fn rollback(
                &self,
                payload: Box<dyn any::Any>,
            ) -> Result<Box<dyn any::Any>, HandlerError> {
                let payload = payload.downcast::<String>().unwrap();
                let payload = payload.to_lowercase();
                Ok(Box::new(payload))
            }
        }

        let flow = Flow::new("test".to_string())
            .action(Action::new("test", TestHandler))
            .action(Action::new("test2", TestHandler2));

        let payload = Some(Box::new("test".to_string()) as Box<dyn any::Any>);
        let res = flow.run(payload).unwrap().unwrap();
        let res = *res.downcast::<String>().unwrap();
        assert_eq!(res, "test".to_string());
    }
}
