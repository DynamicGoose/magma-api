use crate::App;

pub trait Module {
    fn setup(&self, app: &mut App);
}

#[cfg(test)]
mod tests {}
