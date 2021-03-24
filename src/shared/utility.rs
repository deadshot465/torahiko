pub fn create_new_followup_url(application_id: u64, token: String) -> String {
    format!(
        "https://discord.com/api/webhooks/{}/{}",
        application_id, token
    )
}
