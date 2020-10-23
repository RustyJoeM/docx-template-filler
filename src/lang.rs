use std::{collections::HashMap, sync::RwLock};

use unic_langid::{langid, LanguageIdentifier};

use fluent_templates::{fluent_bundle::FluentValue, static_loader, Loader};

#[derive(Debug, Clone)]
pub enum SupportedLanguage {
    EnglishUs,
    Czech,
    Russian,
}

use SupportedLanguage as sl;

impl SupportedLanguage {
    /// Returns collection of all the supported languages.
    pub fn collection() -> Vec<SupportedLanguage> {
        vec![sl::EnglishUs, sl::Czech, sl::Russian]
    }

    /// Returns "localized" textual representation of the enum value.
    pub fn to_name(&self) -> String {
        match self {
            sl::EnglishUs => "English".to_string(),
            sl::Czech => "Česky".to_string(),
            sl::Russian => "Русский".to_string(),
        }
    }

    /// Create enum value form the input name, or None on invalid input.
    pub fn from_string(string: &str) -> Option<Self> {
        for lang in SupportedLanguage::collection().iter() {
            if lang.to_name() == string {
                return Some(lang.clone());
            }
        }
        None
    }

    fn to_lang_code(&self) -> String {
        match self {
            sl::EnglishUs => "en-US".to_string(),
            sl::Czech => "cz".to_string(),
            sl::Russian => "ru".to_string(),
        }
    }
}

lazy_static! {
    static ref LANG_IDS: HashMap<String, LanguageIdentifier> = {
        let mut m = HashMap::new();
        m.insert(sl::EnglishUs.to_lang_code(), langid!("en-US"));
        m.insert(sl::Czech.to_lang_code(), langid!("cz"));
        m.insert(sl::Russian.to_lang_code(), langid!("ru"));
        m
    };
    static ref CURRENT_LANGUAGE: RwLock<SupportedLanguage> = RwLock::new(sl::EnglishUs);
}

static_loader! {
    static LOCALES = {
        locales: "./locales",
        fallback_language: "en-US",
        // Optional: A fluent resource that is shared with every locale.
        // core_locales: "./locales/core.ftl",
        customise: |bundle| bundle.set_use_isolating(false),
    };
}

/// Sets the target language to be used for all the further tr()/tr_with_args() calls for text translations.
pub fn set_current_lang(lang: SupportedLanguage) {
    let mut current = CURRENT_LANGUAGE.write().unwrap();
    *current = lang;
}

/// Returns translation of a plain text specified by current language setting and the input message id.
pub fn tr(msg_id: &str) -> String {
    translate(msg_id, None)
}

/// Type alias for more complex creation of translation arguments.
pub type TrArg = (String, String);

/// Vector of arguments to be used for translated message.
pub type TrArgVec = Vec<TrArg>;

/// Final type for argument map used as direct input for fluent-templates.
type ArgsMap<'m> = HashMap<String, FluentValue<'m>>;

/// Returns translation of a text augmented with arguments, specified by current language setting and the input message id.
pub fn tr_with_args(msg_id: &str, args: &TrArgVec) -> String {
    let arg_map = args_to_map(args);
    translate(msg_id, Some(&arg_map))
}

/// Translates the input text to the currently set language.
fn translate(text_id: &str, args: Option<&ArgsMap>) -> String {
    let lang_code = current_lang().to_lang_code();
    if let Some(li) = LANG_IDS.get(&lang_code) {
        let translated = &*LOCALES.lookup_complete(&li, text_id, args);
        return translated.to_string();
    }
    text_id.to_string()
}

// Returns currently set language.
fn current_lang() -> SupportedLanguage {
    CURRENT_LANGUAGE.read().unwrap().clone()
}

/// Creates args map for the string translation.
fn args_to_map<'a, T>(params: &'a Vec<(String, T)>) -> ArgsMap
where
    T: Into<FluentValue<'a>> + Clone,
{
    let mut map: ArgsMap = HashMap::new();
    for (k, v) in params {
        let key = k.to_string();
        let value: FluentValue = v.to_owned().into();
        map.insert(key, value);
    }
    map
}
