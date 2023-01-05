pub trait DiagnosticHandler {
    type Output;

    fn handle(&self, diagnostic: String) -> Self::Output;
}