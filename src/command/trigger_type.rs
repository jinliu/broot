/// This rather vague enum might be precised or removed. It
///  serves today to characterize whether a verb execution
///  comes from the input or not (in this case the input is
///  consumed and cleared when the verb is executed).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TriggerType {
    /// the verb was typed in the input and user has hit enter.
    Input,
    /// probably a key shortcut
    Other,
}
