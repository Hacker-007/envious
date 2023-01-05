use envyc_context::diagnostic_handler::DiagnosticHandler;

pub struct StdoutHandler;

impl DiagnosticHandler for StdoutHandler {
    type Output = ();

    fn handle(&self, diagnostic: String) -> Self::Output {
        println!("{}", diagnostic)
    }
}