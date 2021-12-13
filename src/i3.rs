pub(crate) struct Wm {
    client: i3_ipc::I3Stream,
}

impl Wm {
    fn active_workspace(&mut self) -> Option<i32> {
        let workspaces = match self.client.get_workspaces() {
            Ok(workspaces) => workspaces,
            Err(_) => return None,
        };

        for workspace in workspaces {
            if workspace.focused {
                return Some(workspace.num);
            }
        }

        None
    }
}

impl crate::Wm for Wm {
    type Output = i3_ipc::reply::Output;

    fn new() -> crate::Result<Self> {
        use i3_ipc::Connect;

        let client = i3_ipc::I3::connect()?;

        Ok(Self { client })
    }

    fn outputs(&mut self) -> crate::Result<Vec<Self::Output>> {
        let outputs = self.client.get_outputs()?;

        Ok(outputs)
    }

    fn active_output(&mut self, outputs: &[Self::Output]) -> Option<usize> {
        let active_workspace = match self.active_workspace() {
            Some(active_workspace) => active_workspace,
            None => return None,
        };

        for (n, output) in outputs.iter().enumerate() {
            if output.current_workspace == Some(active_workspace.to_string()) {
                return Some(n);
            }
        }

        None
    }

    fn is_active(&mut self, output: &Self::Output) -> bool {
        output.active
    }

    fn select_current_workspace(&mut self, output: &Self::Output) -> crate::Result {
        let workspace = output.current_workspace.as_ref().unwrap();
        let command = format!("workspace {}", workspace);

        self.client.run_command(&command)?;

        Ok(())
    }
}
