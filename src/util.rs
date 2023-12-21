pub fn urlencode(s: impl AsRef<str>) -> String {
    url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect::<String>()
}
