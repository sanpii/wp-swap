mod i3;

trait Wm {
    type Output;

    fn new() -> crate::Result<Self> where Self: Sized;
    fn outputs(&mut self) -> crate::Result<Vec<Self::Output>>;
    fn active_output(&mut self, outputs: &[Self::Output]) -> Option<usize>;
    fn is_active(&mut self, output: &Self::Output) -> bool;
    fn select_current_workspace(&mut self, output: &Self::Output) -> crate::Result<()>;
}

#[derive(derive_more::From, Debug)]
enum Error {
    NotFound,
    Io(std::io::Error),
    Utf8(std::str::Utf8Error),
    Json(json::Error),
}

type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()>
{
    let mut client = i3::Wm::new()?;
    let outputs = client.outputs()?;

    if let Some(mut active) = client.active_output(&outputs) {
        loop {
            active = (active + 1) % outputs.len();
            if client.is_active(&outputs[active]) == true {
                break;
            }
        }

        client.select_current_workspace(&outputs[active])?;
    }

    Ok(())
}
