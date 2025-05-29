use std::collections::HashSet;

use skrifa::{FontRef, MetadataProvider, Tag};
use ucd::Codepoint;

pub fn supported_scripts(font: &FontRef) -> HashSet<&'static str> {
    let cmap = font.charmap();
    let mut strings = HashSet::new();
    for (codepoint, _glyphid) in cmap.mappings() {
        if let Some(script) = char::from_u32(codepoint).and_then(|c| c.script()) {
            // Would you believe, no Display, no .to_string(), we just have to
            // grub around with Debug.
            let script_name = format!("{:?}", script);
            if let Some(iso_script) = unicode_to_iso(&script_name) {
                if !iso_script.starts_with("Z") {
                    strings.insert(iso_script);
                }
            } else {
                log::warn!("No ISO 15924 code for script: {}", script_name);
            }
        }
    }
    strings
}

fn unicode_to_iso(script: &str) -> Option<&'static str> {
    match script {
        "Adlam" => Some("Adlm"),
        "Caucasian_Albanian" => Some("Aghb"),
        "Ahom" => Some("Ahom"),
        "Arabic" => Some("Arab"),
        "Imperial_Aramaic" => Some("Armi"),
        "Armenian" => Some("Armn"),
        "Avestan" => Some("Avst"),
        "Balinese" => Some("Bali"),
        "Bamum" => Some("Bamu"),
        "Bassa_Vah" => Some("Bass"),
        "Batak" => Some("Batk"),
        "Bengali" => Some("Beng"),
        "Bhaiksuki" => Some("Bhks"),
        "Bopomofo" => Some("Bopo"),
        "Brahmi" => Some("Brah"),
        "Braille" => Some("Brai"),
        "Buginese" => Some("Bugi"),
        "Buhid" => Some("Buhd"),
        "Chakma" => Some("Cakm"),
        "Canadian_Aboriginal" => Some("Cans"),
        "Carian" => Some("Cari"),
        "Cham" => Some("Cham"),
        "Cherokee" => Some("Cher"),
        "Chorasmian" => Some("Chrs"),
        "Coptic" => Some("Copt"),
        "Cypro_Minoan" => Some("Cpmn"),
        "Cypriot" => Some("Cprt"),
        "Cyrillic" => Some("Cyrl"),
        "Devanagari" => Some("Deva"),
        "Dives_Akuru" => Some("Diak"),
        "Dogra" => Some("Dogr"),
        "Deseret" => Some("Dsrt"),
        "Duployan" => Some("Dupl"),
        "Egyptian_Hieroglyphs" => Some("Egyp"),
        "Elbasan" => Some("Elba"),
        "Elymaic" => Some("Elym"),
        "Ethiopic" => Some("Ethi"),
        "Garay" => Some("Gara"),
        "Georgian" => Some("Geor"),
        "Glagolitic" => Some("Glag"),
        "Gunjala_Gondi" => Some("Gong"),
        "Masaram_Gondi" => Some("Gonm"),
        "Gothic" => Some("Goth"),
        "Grantha" => Some("Gran"),
        "Greek" => Some("Grek"),
        "Gujarati" => Some("Gujr"),
        "Gurung_Khema" => Some("Gukh"),
        "Gurmukhi" => Some("Guru"),
        "Hangul" => Some("Hang"),
        "Han" => Some("Hani"),
        "Hanunoo" => Some("Hano"),
        "Hatran" => Some("Hatr"),
        "Hebrew" => Some("Hebr"),
        "Hiragana" => Some("Hira"),
        "Anatolian_Hieroglyphs" => Some("Hluw"),
        "Pahawh_Hmong" => Some("Hmng"),
        "Nyiakeng_Puachue_Hmong" => Some("Hmnp"),
        "Katakana_Or_Hiragana" => Some("Hrkt"),
        "Old_Hungarian" => Some("Hung"),
        "Old_Italic" => Some("Ital"),
        "Javanese" => Some("Java"),
        "Kayah_Li" => Some("Kali"),
        "Katakana" => Some("Kana"),
        "Kawi" => Some("Kawi"),
        "Kharoshthi" => Some("Khar"),
        "Khmer" => Some("Khmr"),
        "Khojki" => Some("Khoj"),
        "Khitan_Small_Script" => Some("Kits"),
        "Kannada" => Some("Knda"),
        "Kirat_Rai" => Some("Krai"),
        "Kaithi" => Some("Kthi"),
        "Tai_Tham" => Some("Lana"),
        "Lao" => Some("Laoo"),
        "Latin" => Some("Latn"),
        "Lepcha" => Some("Lepc"),
        "Limbu" => Some("Limb"),
        "Linear_A" => Some("Lina"),
        "Linear_B" => Some("Linb"),
        "Lisu" => Some("Lisu"),
        "Lycian" => Some("Lyci"),
        "Lydian" => Some("Lydi"),
        "Mahajani" => Some("Mahj"),
        "Makasar" => Some("Maka"),
        "Mandaic" => Some("Mand"),
        "Manichaean" => Some("Mani"),
        "Marchen" => Some("Marc"),
        "Medefaidrin" => Some("Medf"),
        "Mende_Kikakui" => Some("Mend"),
        "Meroitic_Cursive" => Some("Merc"),
        "Meroitic_Hieroglyphs" => Some("Mero"),
        "Malayalam" => Some("Mlym"),
        "Modi" => Some("Modi"),
        "Mongolian" => Some("Mong"),
        "Mro" => Some("Mroo"),
        "Meetei_Mayek" => Some("Mtei"),
        "Multani" => Some("Mult"),
        "Myanmar" => Some("Mymr"),
        "Nag_Mundari" => Some("Nagm"),
        "Nandinagari" => Some("Nand"),
        "Old_North_Arabian" => Some("Narb"),
        "Nabataean" => Some("Nbat"),
        "Newa" => Some("Newa"),
        "Nko" => Some("Nkoo"),
        "Nushu" => Some("Nshu"),
        "Ogham" => Some("Ogam"),
        "Ol_Chiki" => Some("Olck"),
        "Ol_Onal" => Some("Onao"),
        "Old_Turkic" => Some("Orkh"),
        "Oriya" => Some("Orya"),
        "Osage" => Some("Osge"),
        "Osmanya" => Some("Osma"),
        "Old_Uyghur" => Some("Ougr"),
        "Palmyrene" => Some("Palm"),
        "Pau_Cin_Hau" => Some("Pauc"),
        "Old_Permic" => Some("Perm"),
        "Phags_Pa" => Some("Phag"),
        "Inscriptional_Pahlavi" => Some("Phli"),
        "Psalter_Pahlavi" => Some("Phlp"),
        "Phoenician" => Some("Phnx"),
        "Miao" => Some("Plrd"),
        "Inscriptional_Parthian" => Some("Prti"),
        "Rejang" => Some("Rjng"),
        "Hanifi_Rohingya" => Some("Rohg"),
        "Runic" => Some("Runr"),
        "Samaritan" => Some("Samr"),
        "Old_South_Arabian" => Some("Sarb"),
        "Saurashtra" => Some("Saur"),
        "SignWriting" => Some("Sgnw"),
        "Shavian" => Some("Shaw"),
        "Sharada" => Some("Shrd"),
        "Siddham" => Some("Sidd"),
        "Khudawadi" => Some("Sind"),
        "Sinhala" => Some("Sinh"),
        "Sogdian" => Some("Sogd"),
        "Old_Sogdian" => Some("Sogo"),
        "Sora_Sompeng" => Some("Sora"),
        "Soyombo" => Some("Soyo"),
        "Sundanese" => Some("Sund"),
        "Sunuwar" => Some("Sunu"),
        "Syloti_Nagri" => Some("Sylo"),
        "Syriac" => Some("Syrc"),
        "Tagbanwa" => Some("Tagb"),
        "Takri" => Some("Takr"),
        "Tai_Le" => Some("Tale"),
        "New_Tai_Lue" => Some("Talu"),
        "Tamil" => Some("Taml"),
        "Tangut" => Some("Tang"),
        "Tai_Viet" => Some("Tavt"),
        "Telugu" => Some("Telu"),
        "Tifinagh" => Some("Tfng"),
        "Tagalog" => Some("Tglg"),
        "Thaana" => Some("Thaa"),
        "Thai" => Some("Thai"),
        "Tibetan" => Some("Tibt"),
        "Tirhuta" => Some("Tirh"),
        "Tangsa" => Some("Tnsa"),
        "Todhri" => Some("Todr"),
        "Toto" => Some("Toto"),
        "Tulu_Tigalari" => Some("Tutg"),
        "Ugaritic" => Some("Ugar"),
        "Vai" => Some("Vaii"),
        "Vithkuqi" => Some("Vith"),
        "Warang_Citi" => Some("Wara"),
        "Wancho" => Some("Wcho"),
        "Old_Persian" => Some("Xpeo"),
        "Cuneiform" => Some("Xsux"),
        "Yezidi" => Some("Yezi"),
        "Yi" => Some("Yiii"),
        "Zanabazar_Square" => Some("Zanb"),
        "Inherited" => Some("Zinh"),
        "Common" => Some("Zyyy"),
        "Unknown" => Some("Zzzz"),
        _ => None,
    }
}

pub fn iso15924_to_opentype(script: &str) -> Option<Tag> {
    let script = match script {
        "Deva" => Some(Tag::new(b"dev2")),
        "Beng" => Some(Tag::new(b"bng2")),
        "Gujr" => Some(Tag::new(b"guj2")),
        "Gurm" => Some(Tag::new(b"gur2")),
        "Knda" => Some(Tag::new(b"kan2")),
        "Taml" => Some(Tag::new(b"tam2")),
        "Telu" => Some(Tag::new(b"tel2")),
        _ => Tag::new_checked(script.to_lowercase().as_bytes()).ok(),
    };
    script
}

pub fn is_cjk_codepoint(c: char) -> bool {
    c.script().is_some_and(|s| {
        matches!(
            s,
            ucd::Script::Han
                | ucd::Script::Hiragana
                | ucd::Script::Katakana
                | ucd::Script::Bopomofo
                | ucd::Script::Hangul
                | ucd::Script::KatakanaOrHiragana
        )
    })
}
