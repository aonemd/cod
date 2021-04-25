pub trait Store<'a, T> {
    fn new(source: Option<&'a str>) -> Self;
    fn read(self) -> T;
    fn write(self, data: String);
    fn get_source(&self) -> String;
}
