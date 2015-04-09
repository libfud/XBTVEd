pub trait Action {
    fn apply(&self, buffer: &mut super::gui::EdBuffer);
    fn reverse(&self, buffer: &mut super::gui::EdBuffer);
}

pub struct ChangeName {
    old: String,
    new: String
}

impl Action for ChangeName {
    fn apply(&self, buffer: &mut super::gui::EdBuffer) {
        buffer.set_name(&self.new);
    }

    fn reverse(&self, buffer: &mut super::gui::EdBuffer) {
        buffer.set_name(&self.old);
    }
}

impl ChangeName {
    pub fn new(orig: &str, novo: &str) -> Box<Action> {
        let changename = ChangeName {
            old: orig.to_string(),
            new: novo.to_string()
        };
        Box::new(changename)
    }
}
