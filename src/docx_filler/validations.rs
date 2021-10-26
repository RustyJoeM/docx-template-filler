use super::{DocxError, TokenPackArg, ValuePack, ValuePackArg};
use crate::lang;
use std::collections::HashMap;

/// Verifies consistency of input data for a single DOCX generation.
///
/// # Arguments
///
/// * `tokens` - vector of tokens to be replaced
/// * `values` - vector of values to be filled in place of tokens
/// * `output_pattern` - output file pattern (explicit string or pattern contains tokens)
///
/// # Errors
///
/// Can return Docx::Validation on failure, with details in message.
pub fn validate_single(
    tokens: TokenPackArg,
    values: ValuePackArg,
    output_pattern: &str,
) -> Result<(), DocxError> {
    validate_tokens(tokens)?;
    validate_values(tokens, values)?;
    let filename = super::replace_tokens(output_pattern, tokens, values);
    validate_filename(&filename)?;
    Ok(())
}

/// Verifies consistency of input data for a batch of to-be generated documents
/// (one per each line in input values text).
///
/// # Arguments
///
/// * `tokens` - vector of tokens to be verified
/// * `text` - one to many lines of text - a set of values per each line for a new document to be generated
/// * `output_pattern` - output file pattern (pattern containing tokens)
///
/// # Errors
///
/// Can return Docx::Validation on failure, with details in message.
pub fn validate_batch(
    tokens: TokenPackArg,
    text: &str,
    separator: &str,
    output_pattern: &str,
) -> Result<(), DocxError> {
    validate_tokens(tokens)?;
    validate_values_multiline(text, separator, tokens)?;
    validate_filename_multiline(tokens, text, separator, output_pattern)?;
    Ok(())
}

/// Verifies token pack - checks whether it is non-empty, and does not contain duplicates.
///
/// # Arguments
///
/// * `tokens` - vector of tokens to be verified
///
/// # Errors
///
/// Can return Docx::Validation on failure, with details in message.
pub fn validate_tokens(tokens: TokenPackArg) -> Result<(), DocxError> {
    if tokens.is_empty() {
        return Err(DocxError::Validation(lang::tr("valid-no-tokens")));
    }
    let counts = tokens_counts_map(tokens);
    for (token, count) in counts {
        if count > 1 {
            let args: lang::TrArgVec = vec![("token".to_string(), token)];
            let msg = lang::tr_with_args("valid-token-duplicity", &args);
            return Err(DocxError::Validation(msg));
        }
    }
    Ok(())
}

/// Builds map with token values as keys, and number of times each token is used in the pack as a value.
fn tokens_counts_map(tokens: TokenPackArg) -> HashMap<String, u8> {
    let mut counts: HashMap<String, u8> = Default::default();
    for token in tokens.iter() {
        (*(counts.entry(token.to_string()).or_insert(0))) += 1;
    }
    counts
}

/// Verifies output file-name pattern. Checks for potential duplicities in filenames of whole input data set
/// (embedding the token values into output pattern, if such pattern used).
///
/// # Arguments
///
/// * `tokens` - vector of tokens to be replaced
/// * `text` - one to many lines of text - a set of values per each line for a new document to be generated
/// * `separator` - string to be used as a value separator on each line of input
/// * `output_pattern` - output file pattern (explicit string or pattern contains tokens)
///
/// # Errors
///
/// Can return Docx::Validation on failure, with details in message.
fn validate_filename_multiline(
    tokens: TokenPackArg,
    text: &str,
    separator: &str,
    output_pattern: &str,
) -> Result<(), DocxError> {
    let mut names: HashMap<String, bool> = Default::default();
    for line in text.lines() {
        let values = super::string_to_values(line, separator);
        validate_values(tokens, &values)?;

        let filename = super::replace_tokens(output_pattern, tokens, &values);
        validate_filename(&filename)?;

        if names.contains_key(&filename) {
            let args: lang::TrArgVec = vec![("filename".to_string(), filename)];
            let msg = lang::tr_with_args("valid-same-output-filename", &args);
            return Err(DocxError::Validation(msg));
        }
        names.insert(filename, true);
    }

    Ok(())
}

/// Validates the consistency of input sets of tokens and values.
fn validate_values(tokens: TokenPackArg, values: ValuePackArg) -> Result<(), DocxError> {
    if values.is_empty() {
        return Err(DocxError::Validation(lang::tr("valid-missing-input")));
    }

    if values.len() != tokens.len() {
        let args: lang::TrArgVec = vec![
            ("tokens".to_string(), tokens.len().to_string()),
            ("values".to_string(), values.len().to_string()),
        ];
        let msg = lang::tr_with_args("valid-count-mismatch", &args);
        return Err(DocxError::Validation(msg));
    }

    Ok(())
}

/// Verifies string to be used as a filename for generated output - whether it has .docx extension.
///
/// # Errors
///
/// Can return Docx::Validation on failure, with details in message.
fn validate_filename(filename: &str) -> Result<(), DocxError> {
    if !filename.ends_with(".docx") {
        let args: lang::TrArgVec = vec![("filename".to_string(), filename.to_string())];
        let msg = lang::tr_with_args("valid-no-docx-suffix", &args);
        return Err(DocxError::Validation(msg));
    }
    Ok(())
}

/// Verifies number of values is equal to number of tokens for each line of the input text.
///
/// # Errors
///
/// Can return Docx::Validation on failure, with details in message.
fn validate_values_multiline(
    text: &str,
    separator: &str,
    tokens: TokenPackArg,
) -> Result<(), DocxError> {
    let mut i: usize = 1;
    if text.is_empty() {
        return Err(DocxError::Validation(lang::tr("valid-missing-input")));
    }
    for inst in text.lines() {
        let values: ValuePack = super::string_to_values(inst, separator);
        if let Err(err) = validate_values(tokens, &values) {
            let args: lang::TrArgVec = vec![
                ("line".to_string(), i.to_string()),
                ("details".to_string(), err.to_string()),
            ];
            let msg = lang::tr_with_args("valid-line-mismatch", &args);
            return Err(DocxError::Validation(msg));
        };
        i += 1;
    }
    Ok(())
}
