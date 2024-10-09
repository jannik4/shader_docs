pub fn to_html(comment: &Option<String>) -> String {
    let Some(comment) = comment else {
        return String::new();
    };
    markdown::to_html(&comment)
}
