
pub trait Plugin {
    fn name(&self) -> String;
    fn load(&self) -> bool;
    fn messages(&self) -> Vec<String>;
    fn receive(&self, message: &str, params: Vec<String>) -> String;
}
