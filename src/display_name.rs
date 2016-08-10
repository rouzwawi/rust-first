use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Error;
use workflow;

impl Display for workflow::Workflow {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), Error> {
        write!(formatter, "Workflow: {} {}", self.component_id, self.endpoint_id)
    }
}
