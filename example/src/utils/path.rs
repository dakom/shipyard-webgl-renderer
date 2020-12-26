cfg_if::cfg_if! {
    if #[cfg(debug_assertions)] {
        pub fn media_url(path: &str) -> String {
            format!("/media/{}", path)
        }
    } else { 
        pub fn media_url(path: &str) -> String {
            format!("/awsm-renderer/media/{}", path)
        }
    } 
}