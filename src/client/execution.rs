use crate::response::ExecutionResponse;

pub trait ExecutionRequest {
    fn execute(&self) -> Box<dyn ExecutionResponse>;
}

struct ExecutionHandler {
    commands: Vec<Box<dyn ExecutionRequest>>,
}

impl ExecutionHandler {
    pub fn new() -> Self {
        Self { commands: vec![] }
    }
    
    pub fn enqueue(&mut self, command: Box<dyn ExecutionRequest>) {
        self.commands.push(command);
    }

    pub fn execute_all(&mut self) -> Vec<Box<dyn ExecutionResponse>> {
        // Execute commands in queue in order and collect responses
        let responses = self.commands
            .iter()
            .map(|command| command.execute())
            .collect();
        // Clear all commands 
        self.commands.clear();
        responses
    }
}