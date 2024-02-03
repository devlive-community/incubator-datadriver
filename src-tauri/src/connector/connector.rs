pub trait Connector {
    fn connect(&self);
    fn query(&self);
}
