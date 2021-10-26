mod validations;

use crate::lang;
use regex::Regex;
use std::io::{Read, Write};
use std::path::Path;
use std::{collections::HashMap, fs::File, path::PathBuf};

/// Alias for a set of tokens (placeholders).
pub type TokenPack = Vec<String>;
pub type TokenPackArg<'a> = &'a [String];

/// Alias for a set of values to be filled into placeholders.
pub type ValuePack = Vec<String>;
pub type ValuePackArg<'a> = &'a [String];

type DocxResult<T> = Result<T, DocxError>;

/// Error returned on failure of some of the docx-filler methods.
/// String representation should give details on what specifically went wrong.
#[derive(Debug, thiserror::Error)]
pub enum DocxError {
    #[error("IO error")]
    Io(#[from] std::io::Error),
    #[error("Zip error")]
    Zip(#[from] zip::result::ZipError),
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("Processing error: {0}")]
    Processing(String),
}

type FileMap = HashMap<String, String>;

/// Main DOCX filler / document generator.
///
/// Loads the contents of DOCX template file into memory (beware huge files).
#[derive(Debug)]
pub struct DocxTemplate {
    /// input path of the DOCX template loaded by this struct.
    input_path: PathBuf,

    /// filename/path of the DOCX contents (actual text of the DOCX document).
    target_xml: String,

    /// in-memory storage of all the DOCX contents/meta-data.
    file_data: FileMap,
}

#[allow(dead_code)] // TODO - seriously something's wrong with dead code reports!
impl DocxTemplate {
    /// Creates the new generator, loading the whole input DOCX file into memory.
    ///
    /// # Arguments
    ///
    /// * `input` - path to the file to be loaded (absolute/relative to the running app)
    ///
    /// # Errors
    ///
    /// Can return error if I/O problems are encountered during opening of the DOCX file.
    /// ZIP related errors can also be raised when reading the DOCX contents into memory.
    pub fn open(input: &Path) -> DocxResult<DocxTemplate> {
        let mut file_map: FileMap = Default::default();

        let zip_file = File::open(input)?;
        let mut zip = zip::ZipArchive::new(zip_file)?;
        for i in 0..zip.len() {
            let mut entry = zip.by_index(i)?;

            let key = String::from(entry.name());
            let mut file_buffer = String::new();
            entry.read_to_string(&mut file_buffer)?;

            file_map.insert(key, file_buffer);
        }

        Ok(DocxTemplate {
            input_path: PathBuf::from(input),
            target_xml: String::from("word/document.xml"),
            file_data: file_map,
        })
    }

    /// Get the tokens identified in the DOCX template.
    ///
    /// # Errors
    ///
    /// Can return errors if no DOCX is loaded when attempting this,
    /// or when parsing of tokens fail.
    pub fn template_tokens(&self) -> DocxResult<TokenPack> {
        let document = self
            .document_contents()
            .ok_or_else(|| DocxError::Processing(lang::tr("ui-docx-no-template")))?;

        let re = match Regex::new(r"\{\{.*?\}\}") {
            Ok(re) => re,
            Err(err) => {
                return Err(DocxError::Processing(err.to_string()));
            }
        };

        let caps = re.captures_iter(&document);

        let mut tokens: TokenPack = Default::default();
        for cap in caps {
            if let Some(token) = cap.get(0) {
                let token_str = token.as_str().to_string();
                if !tokens.contains(&token_str) {
                    tokens.push(token_str);
                }
            }
        }

        validations::validate_tokens(&tokens)?;
        Ok(tokens)
    }

    /// Get the whole textual content of the DOCX template document.
    fn document_contents(&self) -> Option<String> {
        let document = self.file_data.get(&self.target_xml);
        document.map(|content| content.to_string())
    }

    /// Generates a single DOCX file from the loaded template.
    /// Replaces all the tokens/placeholders with the corresponding input values.
    /// This method can be used repeatedly to generate multiple output files with various input tokens/values.
    ///
    /// # Arguments
    ///
    /// * `tokens` - vector of tokens to be replaced
    /// * `values` - vector of values to be filled in place of tokens
    /// * `output_pattern` - output file pattern (explicit string or pattern contains tokens)
    ///
    /// # Errors
    ///
    /// Can return errors on inconsistent input data or other internal problems (see error message for details).
    pub fn build_docx(
        &self,
        tokens: TokenPackArg,
        values: ValuePackArg,
        output_pattern: &str,
    ) -> DocxResult<()> {
        validations::validate_single(tokens, values, output_pattern)?;
        self.data_to_docx(tokens, values, output_pattern)?;
        Ok(())
    }

    /// Common executive method for processing one docx file generation form the loaded template.
    ///
    /// # Arguments
    ///
    /// * `tokens` - vector of tokens to be replaced
    /// * `values` - vector of values to be filled in place of tokens
    /// * `output_pattern` - output file pattern (explicit string or pattern contains tokens)
    ///
    /// # Errors
    ///
    /// Can return errors on inconsistent input data or other internal problems (see error message for details).
    fn data_to_docx(
        &self,
        tokens: TokenPackArg,
        values: ValuePackArg,
        output_pattern: &str,
    ) -> DocxResult<()> {
        let out_str = replace_tokens(output_pattern, tokens, values);

        let out_path = PathBuf::from(&out_str);
        if out_path.exists() {
            let args: lang::TrArgVec = vec![("filename".to_string(), out_str)];
            let msg = lang::tr_with_args("docx-filler-fail-overwrite", &args);
            return Err(DocxError::Processing(msg));
        }

        let zip_file = File::create(out_path)?;
        let mut zip = zip::ZipWriter::new(zip_file);

        let options = zip::write::FileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated)
            .unix_permissions(0o755);

        for (file_name, file_content) in self.file_data.iter() {
            zip.start_file(file_name, options)?;
            zip.write_all(file_content.as_bytes())?;
        }

        let orig_document = self
            .document_contents()
            .ok_or_else(|| DocxError::Processing(lang::tr("docx-filler-fail-load")))?;

        let updated_document = replace_tokens(&orig_document, tokens, values);
        zip.start_file(&self.target_xml, options)?;
        zip.write_all(updated_document.as_bytes())?;
        zip.finish()?;

        Ok(())
    }

    /// Generates batch of DOCX files form  the loaded template, one per each line of values in the input text.
    ///
    /// # Arguments
    ///
    /// * `tokens` - vector of tokens to be replaced
    /// * `text` - one to many lines of text - a set of values per each line for a new document to be generated
    /// * `output_pattern` - output file pattern (explicit string or pattern contains tokens)
    ///
    /// # Errors
    ///
    /// Can return error on failure, with details in the error message.
    pub fn build_docx_batch(
        &self,
        tokens: TokenPackArg,
        text: &str, // TODO change into some line iterator?
        separator: &str,
        output_pattern: &str,
    ) -> DocxResult<()> {
        validations::validate_batch(tokens, text, separator, output_pattern)?;

        for line in text.lines() {
            let values = string_to_values(line, separator);
            self.data_to_docx(tokens, &values, output_pattern)?;
        }

        Ok(())
    }
}

/// Fill in the input string with specified set of tokens and values.
fn replace_tokens(input: &str, tokens: TokenPackArg, values: ValuePackArg) -> String {
    assert_eq!(tokens.len(), values.len());
    let mut output: String = input.to_string();
    for i in 0..tokens.len() {
        output = output.replace(&tokens[i], &values[i]);
    }
    output
}

/// Parse the input string into set of values.
fn string_to_values(input: &str, separator: &str) -> ValuePack {
    let values: ValuePack = input
        .split(separator)
        .map(|x| x.trim().to_string())
        .collect();
    values
}
