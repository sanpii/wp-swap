#![warn(warnings)]

mod i3;

trait Wm {
    type Output;

    fn new() -> crate::Result<Self>
    where
        Self: Sized;
    fn outputs(&mut self) -> crate::Result<Vec<Self::Output>>;
    fn active_output(&mut self, outputs: &[Self::Output]) -> Option<usize>;
    fn is_active(&mut self, output: &Self::Output) -> bool;
    fn select_current_workspace(&mut self, output: &Self::Output) -> crate::Result;
}

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Utf8(#[from] std::str::Utf8Error),
}

type Result<T = ()> = std::result::Result<T, Error>;

fn main() -> Result {
    let mut client = i3::Wm::new()?;
    let outputs = client.outputs()?;

    if let Some(mut active) = client.active_output(&outputs) {
        loop {
            active = (active + 1) % outputs.len();
            if client.is_active(&outputs[active]) {
                break;
            }
        }

        client.select_current_workspace(&outputs[active])?;
    }

    Ok(())
}
