// 分页查询结果模型
struct PageResult<T> {
    items: Vec<T>,
    counts: i32,
    page: i32,
    page_size: i32,
}
impl<T> PageResult<T> {
    fn new(items: Vec<T>, counts: i32, page: i32, page_size: i32) -> Self {
        PageResult {
            items,
            counts,
            page,
            page_size,
        }
    }
}
