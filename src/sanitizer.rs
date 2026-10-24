use url::Url;

/// Comprehensive list of common marketing, ad, analytics, and social tracking query parameter prefixes/names
pub const TRACKING_PARAMS: &[&str] = &[
    // Google & Analytics
    "utm_source",
    "utm_medium",
    "utm_campaign",
    "utm_term",
    "utm_content",
    "utm_id",
    "utm_name",
    "utm_reader",
    "utm_viz_id",
    "utm_pubreferrer",
    "utm_swu",
    "gclid",
    "gbraid",
    "wbraid",
    "_gl",
    "dclid",
    "gad_source",
    "gclsrc",

    // Meta (Facebook, Instagram, Messenger)
    "fbclid",
    "igshid",
    "igsh",
    "mibextid",
    "fb_action_ids",
    "fb_action_types",
    "fb_source",
    "fref",

    // Twitter / X
    "s",
    "t",
    "ref_src",
    "ref_url",
    "twclid",

    // TikTok
    "tt_medium",
    "tt_content",
    "_r",
    "_t",
    "tt_from",

    // Streaming & Media (YouTube, Spotify)
    "si",
    "feature",
    "pp",

    // E-Commerce (Amazon, AliExpress)
    "tag",
    "linkcode",
    "ref_",
    "psc",
    "spm",
    "scm",
    "pvid",
    "algo_pvid",
    "algo_expid",

    // Email, CRM, Ad networks
    "_hsenc",
    "_hsmi",
    "mc_cid",
    "mc_eid",
    "mkt_tok",
    "vero_id",
    "vero_conv",
    "yclid",
    "zanpid",
    "ref",
];

/// Result of sanitizing a URL or mixed text containing URLs
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SanitizeResult {
    pub original_url: String,
    pub sanitized_url: String,
    pub domain: String,
    pub removed_params_count: usize,
    pub removed_params: Vec<String>,
}

/// Strip tracking parameters from a standalone raw URL
pub fn sanitize_url(raw_url: &str) -> Result<SanitizeResult, String> {
    let trimmed = raw_url.trim();
    let mut parsed = Url::parse(trimmed).map_err(|e| format!("Invalid URL '{trimmed}': {e}"))?;
    let domain = parsed.host_str().unwrap_or("unknown").to_string();

    let mut removed_params = Vec::new();
    let mut kept_pairs = Vec::new();

    for (key, val) in parsed.query_pairs() {
        let is_tracker = TRACKING_PARAMS
            .iter()
            .any(|&t| key.eq_ignore_ascii_case(t) || key.to_ascii_lowercase().starts_with("utm_"));

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
        original_url: trimmed.to_string(),
        sanitized_url: parsed.to_string(),
        domain,
        removed_params_count: removed_params.len(),
        removed_params,
    })
}

/// Extracts URLs from mixed text (e.g. shared text from YouTube/Twitter containing title + link)
/// and sanitizes all detected URLs.
pub fn sanitize_text(input: &str) -> SanitizeResult {
    let trimmed = input.trim();

    // Fast-path: Check if input itself is already a standalone URL
    if let Ok(direct_res) = sanitize_url(trimmed) {
        return direct_res;
    }

    let mut result_text = trimmed.to_string();
    let mut total_removed_count = 0;
    let mut all_removed_params = Vec::new();
    let mut primary_domain = String::from("multiple");
    let mut first_sanitized_url = String::new();

    // Scan for http:// or https:// patterns
    let mut search_idx = 0;
    while let Some(start_offset) = result_text[search_idx..].find("http://")
        .or_else(|| result_text[search_idx..].find("https://"))
    {
        let abs_start = search_idx + start_offset;
        let candidate_slice = &result_text[abs_start..];

        // Find end of URL: whitespace, quotes, or trailing punctuation
        let end_offset = candidate_slice
            .find(|c: char| c.is_whitespace() || c == '"' || c == '\'' || c == '>' || c == '<' || c == ')' || c == ']')
            .unwrap_or(candidate_slice.len());

        let mut candidate_url = &candidate_slice[..end_offset];
        // Strip trailing punctuation like '.', ',', ';', '!' if attached
        while candidate_url.ends_with('.') || candidate_url.ends_with(',') || candidate_url.ends_with(';') || candidate_url.ends_with('!') {
            candidate_url = &candidate_url[..candidate_url.len() - 1];
        }

        if let Ok(clean) = sanitize_url(candidate_url) {
            if first_sanitized_url.is_empty() {
                first_sanitized_url = clean.sanitized_url.clone();
                primary_domain = clean.domain.clone();
            }
            total_removed_count += clean.removed_params_count;
            all_removed_params.extend(clean.removed_params);

            // Replace in string
            let before = &result_text[..abs_start];
            let after = &result_text[abs_start + candidate_url.len()..];
            let replacement = clean.sanitized_url;
            let new_search_idx = before.len() + replacement.len();
            result_text = format!("{before}{replacement}{after}");
            search_idx = new_search_idx;
        } else {
            search_idx = abs_start + candidate_url.len().max(1);
        }

        if search_idx >= result_text.len() {
            break;
        }
    }

    SanitizeResult {
        original_url: trimmed.to_string(),
        sanitized_url: if !first_sanitized_url.is_empty() && trimmed.split_whitespace().count() <= 1 {
            first_sanitized_url
        } else {
            result_text
        },
        domain: primary_domain,
        removed_params_count: total_removed_count,
        removed_params: all_removed_params,
    }
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

    #[test]
    fn test_sanitizes_youtube_si() {
        let input = "https://youtu.be/dQw4w9WgXcQ?si=abcdef123456";
        let res = sanitize_url(input).expect("Valid URL");
        assert_eq!(res.sanitized_url, "https://youtu.be/dQw4w9WgXcQ");
        assert_eq!(res.removed_params_count, 1);
    }

    #[test]
    fn test_sanitizes_tiktok_and_meta() {
        let input = "https://www.instagram.com/reel/xyz123/?igsh=YWJj&utm_medium=copy_link";
        let res = sanitize_url(input).expect("Valid URL");
        assert_eq!(res.sanitized_url, "https://www.instagram.com/reel/xyz123/");
        assert_eq!(res.removed_params_count, 2);
    }

    #[test]
    fn test_sanitize_text_with_leading_words() {
        let shared = "Check out this video: https://www.youtube.com/watch?v=123&si=tracking_id! It is amazing.";
        let res = sanitize_text(shared);
        assert_eq!(
            res.sanitized_url,
            "Check out this video: https://www.youtube.com/watch?v=123! It is amazing."
        );
        assert_eq!(res.removed_params_count, 1);
    }
}
