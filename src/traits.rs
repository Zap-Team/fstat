pub trait FmtOutputBack {
    fn get_pretty(&self) -> String;
    fn format_fields(&self) -> String;
}
