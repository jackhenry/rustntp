
use crate::{config::ClientConfig, response::{ExecutionResponseType}};

pub type ExecuteFnPtr = fn(config: &ClientConfig) -> ExecutionResponseType; 

pub trait ExecutionCreator {
    fn create_executable() -> ExecuteFnPtr;
}

pub struct ExecutionRequest {
    pub handler: ExecuteFnPtr,
}

pub struct ExecutionHandler<'a> {
    config: &'a ClientConfig,
    commands: Vec<ExecutionRequest>,
}

impl<'a> ExecutionHandler<'a> {
    pub fn from(config: &'a ClientConfig) -> Self {
        Self {
            config,
            commands: vec![] 
        }
    }

    pub fn enqueue(&mut self, command: ExecutionRequest) {
        self.commands.push(command);
    }

    pub fn execute_all(&mut self) -> Vec<ExecutionResponseType> {
        // Execute commands in queue in order and collect responses
        let responses = self
            .commands
            .iter()
            .map(|command| (command.handler)(self.config))
            .collect();
        // Clear all commands
        self.commands.clear();
        responses
    }
}
