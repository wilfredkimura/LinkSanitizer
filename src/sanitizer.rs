use url::Url;

/// List of common marketing, ad, and social tracking query parameter prefixes/names
const TRACKING_PARAMS: &[&str] = &[
    "utm_source",
    "utm_medium",
    "utm_campaign",
    "utm_term",
    "utm_content",
    "fbclid",
    "gclid",
    "igshid",
    "si",          // Spotify, YouTube share id
    "ref",
    "ref_src",
    "s",           // Twitter/X tracking
    "t",           // Timestamp/tracking
    "_hsenc",      // HubSpot
    "_hsmi",
    "mc_cid",      // Mailchimp
    "mc_eid",
];

/// Result of sanitizing a URL
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SanitizeResult {
    pub original_url: String,
    pub sanitized_url: String,
    pub domain: String,
    pub removed_params_count: usize,
    pub removed_params: Vec<String>,
}

/// Strip tracking parameters from a raw URL string
pub fn sanitize_url(raw_url: &str) -> Result<SanitizeResult, String> {
    let mut parsed = Url::parse(raw_url).map_err(|e| format!("Invalid URL: {e}"))?;
    let domain = parsed.host_str().unwrap_or("unknown").to_string();

    let mut removed_params = Vec::new();
    let mut kept_pairs = Vec::new();

    for (key, val) in parsed.query_pairs() {
        let is_tracker = TRACKING_PARAMS
            .iter()
            .any(|&t| key.eq_ignore_ascii_case(t) || key.starts_with("utm_"));

        if is_tracker {
            removed_params.push(key.to_string());
        } else {
            kept_pairs.push((key.to_string(), val.to_string()));
        }
    }

    // Clear and rebuild query without trackers
    parsed.set_query(None);
    if !kept_pairs.is_empty() {
        let mut serializer = parsed.query_pairs_mut();
        for (k, v) in kept_pairs {
            serializer.append_pair(&k, &v);
        }
    }

    Ok(SanitizeResult {
        original_url: raw_url.to_string(),
        sanitized_url: parsed.to_string(),
        domain,
        removed_params_count: removed_params.len(),
        removed_params,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitizes_utm_and_fbclid() {
        let input = "https://example.com/article?title=rust&utm_source=twitter&fbclid=xyz123";
        let res = sanitize_url(input).expect("Valid URL");
        assert_eq!(res.sanitized_url, "https://example.com/article?title=rust");
        assert_eq!(res.removed_params_count, 2);
    }
}
