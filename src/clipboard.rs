use copypasta_ext::prelude::*;
use copypasta_ext::x11_fork::ClipboardContext;

// ui.output().copied_text doesnt persist after close
pub fn set(input: String) {
    let mut ctx = ClipboardContext::new().unwrap();
    ctx.set_contents(input).unwrap();
}
