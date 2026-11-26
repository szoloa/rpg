use std::collections::HashMap;

use bevy::prelude::*;
use serde::Deserialize;

/// 支持的语言列表。
pub const SUPPORTED_LANGUAGES: [Language; 2] = [Language::ZhCn, Language::EnUs];

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum Language {
    /// 简体中文
    #[default]
    ZhCn,
    /// English
    EnUs,
}

impl Language {
    /// BCP-47 语言代码
    pub fn code(self) -> &'static str {
        match self {
            Language::ZhCn => "zh-CN",
            Language::EnUs => "en-US",
        }
    }

    /// 语言自身的显示名称
    pub fn native_name(self) -> &'static str {
        match self {
            Language::ZhCn => "中文",
            Language::EnUs => "English",
        }
    }

    /// 切换到下一个受支持语言
    pub fn next(self) -> Self {
        let index = SUPPORTED_LANGUAGES
            .iter()
            .position(|language| *language == self)
            .unwrap_or(0);
        SUPPORTED_LANGUAGES[(index + 1) % SUPPORTED_LANGUAGES.len()]
    }
}

/// 多语言资源。持有当前语言和所有翻译表。
#[derive(Resource, Debug)]
pub struct I18n {
    language: Language,
    tables: HashMap<Language, HashMap<String, String>>,
}

impl Default for I18n {
    fn default() -> Self {
        Self::new(Language::default())
    }
}

impl I18n {
    pub fn new(language: Language) -> Self {
        let mut tables = HashMap::new();
        tables.insert(
            Language::ZhCn,
            load_table(include_str!("../assets/i18n/zh_cn.csv")),
        );
        tables.insert(
            Language::EnUs,
            load_table(include_str!("../assets/i18n/en_us.csv")),
        );

        Self { language, tables }
    }

    pub fn language(&self) -> Language {
        self.language
    }

    /// 设置当前语言。资源变化会自动触发 `is_changed()`，供 UI 刷新。
    pub fn set_language(&mut self, language: Language) {
        self.language = language;
    }

    /// 翻译一个 key。key 不存在时直接返回 key 本身，便于发现漏配。
    pub fn tr(&self, key: &str) -> String {
        self.tr_or(key, key)
    }

    /// 翻译一个 key，并允许提供兜底文本。
    pub fn tr_or(&self, key: &str, fallback: &str) -> String {
        self.tables
            .get(&self.language)
            .and_then(|table| table.get(key))
            .cloned()
            .unwrap_or_else(|| fallback.to_string())
    }

    /// 翻译并用 `{{name}}` 形式的占位符替换参数。
    pub fn tr_args(&self, key: &str, args: &[(&str, String)]) -> String {
        let mut text = self.tr(key);
        for (name, value) in args {
            text = text.replace(&format!("{{{{{name}}}}}"), value);
        }
        text
    }

    /// 当前语言是否存在某个 key（测试和调试用）。
    pub fn has_key(&self, key: &str) -> bool {
        self.tables
            .get(&self.language)
            .is_some_and(|table| table.contains_key(key))
    }
}

#[derive(Debug, Deserialize)]
struct TranslationRecord {
    key: String,
    text: String,
}

fn load_table(csv_data: &str) -> HashMap<String, String> {
    let mut reader = csv::Reader::from_reader(csv_data.as_bytes());
    let mut table = HashMap::new();

    for result in reader.deserialize::<TranslationRecord>() {
        match result {
            Ok(record) => {
                table.insert(record.key, record.text);
            }
            Err(error) => warn!("跳过一条无效翻译数据: {error}"),
        }
    }

    table
}

pub struct I18nPlugin;

impl Plugin for I18nPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<I18n>()
            .add_systems(Update, toggle_language);
    }
}

/// F1 在支持的语言之间切换。
fn toggle_language(keyboard: Res<ButtonInput<KeyCode>>, mut i18n: ResMut<I18n>) {
    if !keyboard.just_pressed(KeyCode::F1) {
        return;
    }

    let next = i18n.language().next();
    i18n.set_language(next);
    info!(
        "{}",
        i18n.tr_args(
            "i18n.switched",
            &[("language", next.native_name().to_string())]
        )
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn both_languages_define_the_same_keys() {
        let i18n = I18n::default();

        let zh_keys: std::collections::BTreeSet<_> = i18n
            .tables
            .get(&Language::ZhCn)
            .expect("中文表必须存在")
            .keys()
            .cloned()
            .collect();
        let en_keys: std::collections::BTreeSet<_> = i18n
            .tables
            .get(&Language::EnUs)
            .expect("英文表必须存在")
            .keys()
            .cloned()
            .collect();

        assert_eq!(zh_keys, en_keys);
        assert!(!zh_keys.is_empty());
    }

    #[test]
    fn switches_language_and_replaces_placeholders() {
        let mut i18n = I18n::default();

        let zh = i18n.tr_args("combat.hit", &[("enemy", "史莱姆".to_string())]);
        assert!(zh.contains("史莱姆"));
        assert!(!zh.contains("{{enemy}}"));

        i18n.set_language(Language::EnUs);
        assert_eq!(i18n.language(), Language::EnUs);
        assert_eq!(i18n.tr("class.mage.name"), "Mage");
        assert_eq!(i18n.language().next(), Language::ZhCn);
    }
}
