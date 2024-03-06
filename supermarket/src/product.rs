#[derive(Debug)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub restrictions: Vec<ProductRestriction>,
}

#[derive(Debug)]
pub enum ProductRestriction {
    AgeRestriction(u8),
}
