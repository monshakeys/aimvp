//! Paranoid text spacing for good readability, to automatically insert
//! whitespace between CJK (Chinese, Japanese, Korean) and half-width
//! characters (alphabetical letters, numerical digits and symbols).

use regex::Regex;
use std::borrow::Cow;
use std::sync::LazyLock;

// Chinese, Japanese, Korean
// See <http://unicode-table.com>
const CJK: &str = "\u{2e80}-\u{2eff}\u{2f00}-\u{2fdf}\u{3040}-\u{309f}\u{30a0}-\u{30ff}\u{3100}-\u{312f}\u{3200}-\u{32ff}\u{3400}-\u{4dbf}\u{4e00}-\u{9fff}\u{f900}-\u{faff}";

// Alphabets, Numbers, Symbols
const ANS: &str =
    "A-Za-z0-9`\\$%\\^&\\*\\-=\\+\\\\|/\u{00a1}-\u{00ff}\u{2022}\u{2027}\u{2150}-\u{218f}";

// Quote spacing patterns - optimized with clear documentation
static CJK_QUOTE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(&format!("([{CJK}])([\"'])"))
        .expect("CJK quote regex should compile")
});
static QUOTE_CJK: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(&format!("([\"'])([{CJK}])"))
        .expect("Quote CJK regex should compile")
});
// Remove extra spaces inside quotes, brackets, etc.
static FIX_QUOTE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new("([\"'\\(\\[\\{{<\u{201c}])(\\s*)(.+?)(\\s*)([\"'\\)\\]\\}}>\u{201d}])")
        .expect("Fix quote regex should compile")
});
// Fix single quote spacing between CJK and alphabets
static FIX_SINGLE_QUOTE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(&format!("([{CJK}])( )(')([A-Za-z])"))
        .expect("Fix single quote regex should compile")
});

// Hash spacing patterns - hashtag support
static CJK_HASH: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(&format!("([{CJK}])(#(\\S+))"))
        .expect("CJK hash regex should compile")
});
static HASH_CJK: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(&format!("((\\S+)#)([{CJK}])"))
        .expect("Hash CJK regex should compile")
});

// Operator spacing patterns - mathematical and logical operators
static CJK_OPERATOR_ANS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(&format!("([{CJK}])([\\+\\-\\*/=&\\|<>])([A-Za-z0-9])"))
        .expect("CJK operator ANS regex should compile")
});
static ANS_OPERATOR_CJK: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(&format!("([A-Za-z0-9])([\\+\\-\\*/=&\\|<>])([{CJK}])"))
        .expect("ANS operator CJK regex should compile")
});
static CJK_OPERATOR_CJK: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(&format!("([{CJK}])([\\+\\-\\*/=&\\|<>])([{CJK}])"))
        .expect("CJK operator CJK regex should compile")
});


// Bracket spacing patterns - comprehensive bracket support
static CJK_BRACKET_CJK: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(&format!(
        "([{CJK}])([\\(\\[\\{{<\u{201c}]+(.*?)[\\)\\]\\}}>\u{201d}]+)([{CJK}])"
    ))
    .expect("CJK bracket CJK regex should compile")
});
static CJK_BRACKET: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(&format!("([{CJK}])([\\(\\[\\{{<\u{201c}>])"))
        .expect("CJK bracket regex should compile")
});
static BRACKET_CJK: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(&format!("([\\)\\]\\}}>\u{201d}<])([{CJK}])"))
        .expect("Bracket CJK regex should compile")
});
// Remove extra spaces inside brackets
static FIX_BRACKET: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new("([\\(\\[\\{{<\u{201c}]+)(\\s*)(.+?)(\\s*)([\\)\\]\\}}>\u{201d}]+)")
        .expect("Fix bracket regex should compile")
});

// Symbol spacing: fix spacing between CJK and punctuation followed by ANS
static FIX_SYMBOL: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(&format!("([{CJK}])([~!;:,\\.\\?\u{2026}])([A-Za-z0-9])"))
        .expect("Fix symbol regex should compile")
});

// General CJK-ANS spacing patterns
static CJK_ANS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(&format!("([{CJK}])([{ANS}@])"))
        .expect("CJK ANS regex should compile")
});
static ANS_CJK: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(&format!("([{ANS}~!;:,\\.\\?\u{2026}])([{CJK}])"))
        .expect("ANS CJK regex should compile")
});

/// Insert whitespace between CJK and half-width characters.
/// 
/// This function applies multiple spacing rules in sequence to ensure proper
/// spacing between CJK characters and half-width characters while preserving
/// the original text when no changes are needed.
/// 
/// Optimization: Uses fast-path detection to avoid processing when unnecessary.
pub fn spacing(text: &str) -> Cow<'_, str> {
    // Fast path: Early exit if text is empty or very short
    if text.len() < 2 {
        return Cow::Borrowed(text);
    }
    
    // Fast path: Check if text contains any CJK characters
    // If no CJK characters, no spacing needed
    let has_cjk = text.chars().any(|c| {
        matches!(c,
            '\u{2e80}'..='\u{2eff}' | // CJK Radicals Supplement
            '\u{2f00}'..='\u{2fdf}' | // Kangxi Radicals  
            '\u{3040}'..='\u{309f}' | // Hiragana
            '\u{30a0}'..='\u{30ff}' | // Katakana
            '\u{3100}'..='\u{312f}' | // Bopomofo
            '\u{3200}'..='\u{32ff}' | // Enclosed CJK Letters and Months
            '\u{3400}'..='\u{4dbf}' | // CJK Unified Ideographs Extension A
            '\u{4e00}'..='\u{9fff}' | // CJK Unified Ideographs
            '\u{f900}'..='\u{faff}'   // CJK Compatibility Ideographs
        )
    });
    
    if !has_cjk {
        return Cow::Borrowed(text);
    }
    
    // Fast path: Check if text contains any ANS characters that would need spacing
    // Include operators and hash in the check
    let has_ans = text.chars().any(|c| {
        c.is_ascii_alphanumeric() || 
        matches!(c, '`' | '$' | '%' | '^' | '&' | '*' | '-' | '=' | '+' | '\\' | '|' | 
                    '/' | '@' | '~' | '!' | ';' | ':' | ',' | '.' | '?' | '\u{2026}' |
                    '<' | '>' | '#' | // Added missing operators and hash
                    '\u{00a1}'..='\u{00ff}' | '\u{2022}' | '\u{2027}' | '\u{2150}'..='\u{218f}')
    });
    
    if !has_ans {
        return Cow::Borrowed(text);
    }

    // Step 1: Apply quote spacing patterns
    let after_cjk_quote = CJK_QUOTE.replace_all(text, "$1 $2");
    let after_quote_cjk = QUOTE_CJK.replace_all(&after_cjk_quote, "$1 $2");
    let after_fix_quote = FIX_QUOTE.replace_all(&after_quote_cjk, "$1$3$5");
    let after_fix_single_quote = FIX_SINGLE_QUOTE.replace_all(&after_fix_quote, "$1$3$4");

    // Step 2: Apply hash spacing patterns
    let after_cjk_hash = CJK_HASH.replace_all(&after_fix_single_quote, "$1 $2");
    let after_hash_cjk = HASH_CJK.replace_all(&after_cjk_hash, "$1 $3");

    // Step 3: Apply operator spacing patterns (including CJK-CJK operators)
    let after_cjk_operator = CJK_OPERATOR_ANS.replace_all(&after_hash_cjk, "$1 $2 $3");
    let after_ans_operator = ANS_OPERATOR_CJK.replace_all(&after_cjk_operator, "$1 $2 $3");
    let after_cjk_operator_cjk = CJK_OPERATOR_CJK.replace_all(&after_ans_operator, "$1 $2 $3");

    // Step 4: Apply bracket spacing patterns
    let after_bracket_cjk_cjk = CJK_BRACKET_CJK.replace_all(&after_cjk_operator_cjk, "$1 $2 $4");
    let after_cjk_bracket = CJK_BRACKET.replace_all(&after_bracket_cjk_cjk, "$1 $2");
    let after_bracket_cjk = BRACKET_CJK.replace_all(&after_cjk_bracket, "$1 $2");
    let after_fix_bracket = FIX_BRACKET.replace_all(&after_bracket_cjk, "$1$3$5");

    // Step 5: Apply symbol spacing
    let after_fix_symbol = FIX_SYMBOL.replace_all(&after_fix_bracket, "$1$2 $3");

    // Step 6: Apply general CJK-ANS spacing patterns
    let after_cjk_ans = CJK_ANS.replace_all(&after_fix_symbol, "$1 $2");
    let final_result = ANS_CJK.replace_all(&after_cjk_ans, "$1 $2");

    // Optimize Cow usage: check if the text actually changed by comparing pointers
    let eq = |x: &str, y: &str| x.as_ptr() == y.as_ptr() && x.len() == y.len();
    
    match final_result {
        // If no changes were made, return borrowed original text
        Cow::Borrowed(t) if eq(t, text) => Cow::Borrowed(text),
        // Otherwise, return owned string as changes were made
        spaced => Cow::Owned(spaced.into_owned()),
    }
}
