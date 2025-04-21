pub trait Table {
    fn table_name(&self) -> &'static str;
}
