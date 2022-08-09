pub fn escape(mrkd: String) -> String {
    mrkd.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
