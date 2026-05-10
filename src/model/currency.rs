#[derive(toasty::Model)]
pub struct Currency {
    #[key]
    pub code: String,
    pub name: String,
    pub symbol: String,
}
