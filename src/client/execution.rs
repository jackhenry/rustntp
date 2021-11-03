use std::fmt;

use crate::response::{ExecutionResponseType};

pub trait ExecutionRequest {
    type ResponseType: fmt::Debug;
    fn execute(&self) -> Self::ResponseType;
}

pub struct ExecutionHandler {
    commands: Vec<Box<dyn ExecutionRequest<ResponseType = ExecutionResponseType>>>,
}

impl ExecutionHandler {
    pub fn new() -> Self {
        Self { commands: vec![] }
    }

    pub fn enqueue(&mut self, command: Box<dyn ExecutionRequest<ResponseType = ExecutionResponseType>>) {
        self.commands.push(command);
    }

    pub fn execute_all(&mut self) -> Vec<ExecutionResponseType> {
        // Execute commands in queue in order and collect responses
        let responses = self
            .commands
            .iter()
            .map(|command| command.execute())
            .collect();
        // Clear all commands
        self.commands.clear();
        responses
    }
}
