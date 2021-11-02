pub trait Command {
    type Response;

    fn execute(&self) -> Self::Response;
}
