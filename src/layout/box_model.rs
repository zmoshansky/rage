// http://www.binvisions.com/articles/box-sizing-property-difference-content-border/

#[derive(Clone)]
pub enum BoxModel {
    Content,
    Bounding,
}
impl Default for BoxModel {fn default() -> BoxModel { BoxModel::Content }}
