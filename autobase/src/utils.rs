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

pub const KNOWN_ISO_SCRIPTS: [&str; 172] = [
    "Adlm", "Aghb", "Ahom", "Arab", "Armi", "Armn", "Avst", "Bali", "Bamu", "Bass", "Batk", "Beng",
    "Bhks", "Bopo", "Brah", "Brai", "Bugi", "Buhd", "Cakm", "Cans", "Cari", "Cham", "Cher", "Chrs",
    "Copt", "Cpmn", "Cprt", "Cyrl", "Deva", "Diak", "Dogr", "Dsrt", "Dupl", "Egyp", "Elba", "Elym",
    "Ethi", "Gara", "Geor", "Glag", "Gong", "Gonm", "Goth", "Gran", "Grek", "Gujr", "Gukh", "Guru",
    "Hang", "Hani", "Hano", "Hatr", "Hebr", "Hira", "Hluw", "Hmng", "Hmnp", "Hrkt", "Hung", "Ital",
    "Java", "Kali", "Kana", "Kawi", "Khar", "Khmr", "Khoj", "Kits", "Knda", "Krai", "Kthi", "Lana",
    "Laoo", "Latn", "Lepc", "Limb", "Lina", "Linb", "Lisu", "Lyci", "Lydi", "Mahj", "Maka", "Mand",
    "Mani", "Marc", "Medf", "Mend", "Merc", "Mero", "Mlym", "Modi", "Mong", "Mroo", "Mtei", "Mult",
    "Mymr", "Nagm", "Nand", "Narb", "Nbat", "Newa", "Nkoo", "Nshu", "Ogam", "Olck", "Onao", "Orkh",
    "Orya", "Osge", "Osma", "Ougr", "Palm", "Pauc", "Perm", "Phag", "Phli", "Phlp", "Phnx", "Plrd",
    "Prti", "Rjng", "Rohg", "Runr", "Samr", "Sarb", "Saur", "Sgnw", "Shaw", "Shrd", "Sidd", "Sind",
    "Sinh", "Sogd", "Sogo", "Sora", "Soyo", "Sund", "Sunu", "Sylo", "Syrc", "Tagb", "Takr", "Tale",
    "Talu", "Taml", "Tang", "Tavt", "Telu", "Tfng", "Tglg", "Thaa", "Thai", "Tibt", "Tirh", "Tnsa",
    "Todr", "Toto", "Tutg", "Ugar", "Vaii", "Vith", "Wara", "Wcho", "Xpeo", "Xsux", "Yezi", "Yiii",
    "Zanb", "Zinh", "Zyyy", "Zzzz",
];

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
    match script {
        "Deva" => Some(Tag::new(b"dev2")),
        "Beng" => Some(Tag::new(b"bng2")),
        "Gujr" => Some(Tag::new(b"guj2")),
        "Gurm" => Some(Tag::new(b"gur2")),
        "Knda" => Some(Tag::new(b"kan2")),
        "Taml" => Some(Tag::new(b"tam2")),
        "Telu" => Some(Tag::new(b"tel2")),
        _ => Tag::new_checked(script.to_lowercase().as_bytes()).ok(),
    }
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

pub fn iso639_to_opentype(language: &str) -> Tag {
    match language {
        "aa" => Tag::new(b"AFR "),  // Afar
        "aae" => Tag::new(b"SQI "), // Arbëreshë Albanian -> Albanian
        "aao" => Tag::new(b"ARA "), // Algerian Saharan Arabic -> Arabic
        // "aaq" => 	tag: Tag::new(b"AAQ "), // Eastern Abnaki -> Eastern Abenaki
        "aat" => Tag::new(b"SQI "), // Arvanitika Albanian -> Albanian
        "ab" => Tag::new(b"ABK "),  // Abkhazian
        "aba" => Tag::new(&[0; 4]), // Abé != Abaza
        "abh" => Tag::new(b"ARA "), // Tajiki Arabic -> Arabic
        "abq" => Tag::new(b"ABA "), // Abaza
        "abs" => Tag::new(b"CPP "), // Ambonese Malay -> Creoles
        "abv" => Tag::new(b"ARA "), // Baharna Arabic -> Arabic
        "acf" => Tag::new(b"FAN "), // Saint Lucian Creole French -> French Antillean
        // "acf" => Tag::new(b"CPP "), // Saint Lucian Creole French -> Creoles
        // "ach" => 	tag: Tag::new(b"ACH "), // Acoli -> Acholi
        "acm" => Tag::new(b"ARA "), // Mesopotamian Arabic -> Arabic
        "acq" => Tag::new(b"ARA "), // Ta'izzi-Adeni Arabic -> Arabic
        "acr" => Tag::new(b"ACR "), // Achi
        // "acr" => Tag::new(b"MYN "), // Achi -> Mayan
        "acw" => Tag::new(b"ARA "), // Hijazi Arabic -> Arabic
        "acx" => Tag::new(b"ARA "), // Omani Arabic -> Arabic
        "acy" => Tag::new(b"ACY "), // Cypriot Arabic
        // "acy" => Tag::new(b"ARA "), // Cypriot Arabic -> Arabic
        "ada" => Tag::new(b"DNG "), // Adangme -> Dangme
        "adf" => Tag::new(b"ARA "), // Dhofari Arabic -> Arabic
        "adp" => Tag::new(b"DZN "), // Adap(retired code) -> Dzongkha
        // "ady" => 	tag: Tag::new(b"ADY "), // Adyghe
        "aeb" => Tag::new(b"ARA "), // Tunisian Arabic -> Arabic
        "aec" => Tag::new(b"ARA "), // Saidi Arabic -> Arabic
        "af" => Tag::new(b"AFK "),  // Afrikaans
        "afb" => Tag::new(b"ARA "), // Gulf Arabic -> Arabic
        "afk" => Tag::new(&[0; 4]), // Nanubae != Afrikaans
        "afs" => Tag::new(b"CPP "), // Afro-Seminole Creole -> Creoles
        "agu" => Tag::new(b"MYN "), // Aguacateco -> Mayan
        "agw" => Tag::new(&[0; 4]), // Kahua != Agaw
        "ahg" => Tag::new(b"AGW "), // Qimant -> Agaw
        "aht" => Tag::new(b"ATH "), // Ahtena -> Athapaskan
        "aig" => Tag::new(b"CPP "), // Antigua and Barbuda Creole English -> Creoles
        "aii" => Tag::new(b"SWA "), // Assyrian Neo-Aramaic -> Swadaya Aramaic
        // "aii" => Tag::new(b"SYR "), // Assyrian Neo-Aramaic -> Syriac
        // "aio" => 	tag: Tag::new(b"AIO "), // Aiton
        "aiw" => Tag::new(b"ARI "), // Aari
        "ajp" => Tag::new(b"ARA "), // South Levantine Arabic(retired code) -> Arabic
        "ajt" => Tag::new(b"ARA "), // Judeo-Tunisian Arabic(retired code) -> Arabic
        "ak" => Tag::new(b"AKA "),  // Akan [macrolanguage]
        "akb" => Tag::new(b"AKB "), // Batak Angkola
        // "akb" => Tag::new(b"BTK "), // Batak Angkola -> Batak
        "aln" => Tag::new(b"SQI "), // Gheg Albanian -> Albanian
        "als" => Tag::new(b"SQI "), // Tosk Albanian -> Albanian
        // "alt" => 	tag: Tag::new(b"ALT "), // Southern Altai -> Altai
        "am" => Tag::new(b"AMH "),  // Amharic
        "amf" => Tag::new(b"HBN "), // Hamer-Banna -> Hammer-Banna
        "amw" => Tag::new(b"SYR "), // Western Neo-Aramaic -> Syriac
        "an" => Tag::new(b"ARG "),  // Aragonese
        // "ang" => 	tag: Tag::new(b"ANG "), // Old English (ca. 450-1100) -> Anglo-Saxon
        "aoa" => Tag::new(b"CPP "), // Angolar -> Creoles
        "apa" => Tag::new(b"ATH "), // Apache  [collection] -> Athapaskan
        "apc" => Tag::new(b"ARA "), // Levantine Arabic -> Arabic
        "apd" => Tag::new(b"ARA "), // Sudanese Arabic -> Arabic
        "apj" => Tag::new(b"ATH "), // Jicarilla Apache -> Athapaskan
        "apk" => Tag::new(b"ATH "), // Kiowa Apache -> Athapaskan
        "apl" => Tag::new(b"ATH "), // Lipan Apache -> Athapaskan
        "apm" => Tag::new(b"ATH "), // Mescalero-Chiricahua Apache -> Athapaskan
        "apw" => Tag::new(b"ATH "), // Western Apache -> Athapaskan
        "ar" => Tag::new(b"ARA "),  // Arabic [macrolanguage]
        "arb" => Tag::new(b"ARA "), // Standard Arabic -> Arabic
        "ari" => Tag::new(&[0; 4]), // Arikara != Aari
        "ark" => Tag::new(&[0; 4]), // Arikapú != Rakhine
        "arn" => Tag::new(b"MAP "), // Mapudungun
        "arq" => Tag::new(b"ARA "), // Algerian Arabic -> Arabic
        "ars" => Tag::new(b"ARA "), // Najdi Arabic -> Arabic
        "ary" => Tag::new(b"MOR "), // Moroccan Arabic -> Moroccan
        // "ary" => Tag::new(b"ARA "), // Moroccan Arabic -> Arabic
        "arz" => Tag::new(b"ARA "), // Egyptian Arabic -> Arabic
        "as" => Tag::new(b"ASM "),  // Assamese
        // "ast" => 	tag: Tag::new(b"AST "), // Asturian
        // "ath" => 	tag: Tag::new(b"ATH "), // Athapascan  [collection] -> Athapaskan
        "atj" => Tag::new(b"RCR "), // Atikamekw -> R-Cree
        // "ats" => 	tag: Tag::new(b"ATS "), // Gros Ventre (Atsina)
        "atv" => Tag::new(b"ALT "), // Northern Altai -> Altai
        "auj" => Tag::new(b"BBR "), // Awjilah -> Berber
        "auz" => Tag::new(b"ARA "), // Uzbeki Arabic -> Arabic
        "av" => Tag::new(b"AVR "),  // Avaric -> Avar
        "avl" => Tag::new(b"ARA "), // Eastern Egyptian Bedawi Arabic -> Arabic
        // "avn" => 	tag: Tag::new(b"AVN "), // Avatime
        // "awa" => 	tag: Tag::new(b"AWA "), // Awadhi
        "ay" => Tag::new(b"AYM "),  // Aymara [macrolanguage]
        "ayc" => Tag::new(b"AYM "), // Southern Aymara -> Aymara
        "ayh" => Tag::new(b"ARA "), // Hadrami Arabic -> Arabic
        "ayl" => Tag::new(b"ARA "), // Libyan Arabic -> Arabic
        "ayn" => Tag::new(b"ARA "), // Sanaani Arabic -> Arabic
        "ayp" => Tag::new(b"ARA "), // North Mesopotamian Arabic -> Arabic
        "ayr" => Tag::new(b"AYM "), // Central Aymara -> Aymara
        "az" => Tag::new(b"AZE "),  // Azerbaijani [macrolanguage]
        "azb" => Tag::new(b"AZB "), // South Azerbaijani -> Torki
        // "azb" => Tag::new(b"AZE "), // South Azerbaijani -> Azerbaijani
        "azd" => Tag::new(b"NAH "), // Eastern Durango Nahuatl -> Nahuatl
        "azj" => Tag::new(b"AZE "), // North Azerbaijani -> Azerbaijani
        "azn" => Tag::new(b"NAH "), // Western Durango Nahuatl -> Nahuatl
        "azz" => Tag::new(b"NAH "), // Highland Puebla Nahuatl -> Nahuatl
        "ba" => Tag::new(b"BSH "),  // Bashkir
        "bad" => Tag::new(b"BAD0"), // Banda  [collection]
        "bag" => Tag::new(&[0; 4]), // Tuki != Baghelkhandi
        "bah" => Tag::new(b"CPP "), // Bahamas Creole English -> Creoles
        "bai" => Tag::new(b"BML "), // Bamileke  [collection]
        "bal" => Tag::new(b"BLI "), // Baluchi [macrolanguage]
        // "ban" => 	tag: Tag::new(b"BAN "), // Balinese
        // "bar" => 	tag: Tag::new(b"BAR "), // Bavarian
        "bau" => Tag::new(&[0; 4]), // Bada (Nigeria) != Baulé
        "bbc" => Tag::new(b"BBC "), // Batak Toba
        // "bbc" => Tag::new(b"BTK "), // Batak Toba -> Batak
        "bbj" => Tag::new(b"BML "), // Ghomálá' -> Bamileke
        "bbp" => Tag::new(b"BAD0"), // West Central Banda -> Banda
        "bbr" => Tag::new(&[0; 4]), // Girawa != Berber
        "bbz" => Tag::new(b"ARA "), // Babalia Creole Arabic(retired code) -> Arabic
        "bcc" => Tag::new(b"BLI "), // Southern Balochi -> Baluchi
        "bch" => Tag::new(&[0; 4]), // Bariai != Bench
        "bci" => Tag::new(b"BAU "), // Baoulé -> Baulé
        "bcl" => Tag::new(b"BIK "), // Central Bikol -> Bikol
        "bcq" => Tag::new(b"BCH "), // Bench
        "bcr" => Tag::new(b"ATH "), // Babine -> Athapaskan
        // "bdc" => 	tag: Tag::new(b"BDC "), // Emberá-Baudó
        // "bdy" => 	tag: Tag::new(b"BDY "), // Bandjalang
        "be" => Tag::new(b"BEL "),  // Belarusian
        "bea" => Tag::new(b"ATH "), // Beaver -> Athapaskan
        "beb" => Tag::new(b"BTI "), // Bebele -> Beti
        // "bem" => 	tag: Tag::new(b"BEM "), // Bemba (Zambia)
        "ber" => Tag::new(b"BBR "), // Berber  [collection]
        "bew" => Tag::new(b"CPP "), // Betawi -> Creoles
        "bfl" => Tag::new(b"BAD0"), // Banda-Ndélé -> Banda
        "bfq" => Tag::new(b"BAD "), // Badaga
        "bft" => Tag::new(b"BLT "), // Balti
        "bfu" => Tag::new(b"LAH "), // Gahri -> Lahuli
        "bfy" => Tag::new(b"BAG "), // Bagheli -> Baghelkhandi
        "bg" => Tag::new(b"BGR "),  // Bulgarian
        // "bgc" => 	tag: Tag::new(b"BGC "), // Haryanvi
        "bgn" => Tag::new(b"BLI "), // Western Balochi -> Baluchi
        "bgp" => Tag::new(b"BLI "), // Eastern Balochi -> Baluchi
        "bgq" => Tag::new(b"BGQ "), // Bagri
        // "bgq" => Tag::new(b"RAJ "), // Bagri -> Rajasthani
        "bgr" => Tag::new(b"QIN "), // Bawm Chin -> Chin
        "bhb" => Tag::new(b"BHI "), // Bhili
        // "bhi" => 	tag: Tag::new(b"BHI "), // Bhilali -> Bhili
        "bhk" => Tag::new(b"BIK "), // Albay Bicolano(retired code) -> Bikol
        // "bho" => 	tag: Tag::new(b"BHO "), // Bhojpuri
        "bhr" => Tag::new(b"MLG "), // Bara Malagasy -> Malagasy
        "bi" => Tag::new(b"BIS "),  // Bislama
        // "bi" => Tag::new(b"CPP "),  // Bislama -> Creoles
        // "bik" => 	tag: Tag::new(b"BIK "), // Bikol [macrolanguage]
        "bil" => Tag::new(&[0; 4]), // Bile != Bilen
        "bin" => Tag::new(b"EDO "), // Edo
        "biu" => Tag::new(b"QIN "), // Biete -> Chin
        // "bjj" => 	tag: Tag::new(b"BJJ "), // Kanauji
        "bjn" => Tag::new(b"MLY "), // Banjar -> Malay
        "bjo" => Tag::new(b"BAD0"), // Mid-Southern Banda -> Banda
        "bjq" => Tag::new(b"MLG "), // Southern Betsimisaraka Malagasy(retired code) -> Malagasy
        "bjs" => Tag::new(b"CPP "), // Bajan -> Creoles
        "bjt" => Tag::new(b"BLN "), // Balanta-Ganja -> Balante
        "bkf" => Tag::new(&[0; 4]), // Beeke != Blackfoot
        "bko" => Tag::new(b"BML "), // Kwa' -> Bamileke
        "bla" => Tag::new(b"BKF "), // Siksika -> Blackfoot
        "ble" => Tag::new(b"BLN "), // Balanta-Kentohe -> Balante
        "blg" => Tag::new(b"IBA "), // Balau(retired code) -> Iban
        "bli" => Tag::new(&[0; 4]), // Bolia != Baluchi
        "blk" => Tag::new(b"BLK "), // Pa’o Karen
        // "blk" => Tag::new(b"KRN "), // Pa'o Karen -> Karen
        "bln" => Tag::new(b"BIK "), // Southern Catanduanes Bikol -> Bikol
        "blt" => Tag::new(&[0; 4]), // Tai Dam != Balti
        "bm" => Tag::new(b"BMB "),  // Bambara (Bamanankan)
        "bmb" => Tag::new(&[0; 4]), // Bembe != Bambara (Bamanankan)
        "bml" => Tag::new(&[0; 4]), // Bomboli != Bamileke
        "bmm" => Tag::new(b"MLG "), // Northern Betsimisaraka Malagasy -> Malagasy
        "bn" => Tag::new(b"BEN "),  // Bangla
        "bo" => Tag::new(b"TIB "),  // Tibetan
        "bpd" => Tag::new(b"BAD0"), // Banda-Banda -> Banda
        "bpl" => Tag::new(b"CPP "), // Broome Pearling Lugger Pidgin -> Creoles
        "bpq" => Tag::new(b"CPP "), // Banda Malay -> Creoles
        // "bpy" => 	tag: Tag::new(b"BPY "), // Bishnupriya -> Bishnupriya Manipuri
        "bqi" => Tag::new(b"LRC "), // Bakhtiari -> Luri
        "bqk" => Tag::new(b"BAD0"), // Banda-Mbrès -> Banda
        "br" => Tag::new(b"BRE "),  // Breton
        "bra" => Tag::new(b"BRI "), // Braj -> Braj Bhasha
        "brc" => Tag::new(b"CPP "), // Berbice Creole Dutch -> Creoles
        // "brh" => 	tag: Tag::new(b"BRH "), // Brahui
        "bri" => Tag::new(&[0; 4]), // Mokpwe != Braj Bhasha
        "brm" => Tag::new(&[0; 4]), // Barambu != Burmese
        // "brx" => 	tag: Tag::new(b"BRX "), // Bodo (India)
        "bs" => Tag::new(b"BOS "),  // Bosnian
        "bsh" => Tag::new(&[0; 4]), // Kati != Bashkir
        // "bsk" => 	tag: Tag::new(b"BSK "), // Burushaski
        "btb" => Tag::new(b"BTI "), // Beti (Cameroon)(retired code)
        "btd" => Tag::new(b"BTD "), // Batak Dairi (Pakpak)
        // "btd" => Tag::new(b"BTK "), // Batak Dairi -> Batak
        "bti" => Tag::new(&[0; 4]), // Burate != Beti
        "btj" => Tag::new(b"MLY "), // Bacanese Malay -> Malay
        // "btk" => 	tag: Tag::new(b"BTK "), // Batak  [collection]
        "btm" => Tag::new(b"BTM "), // Batak Mandailing
        // "btm" => Tag::new(b"BTK "), // Batak Mandailing -> Batak
        "bto" => Tag::new(b"BIK "), // Rinconada Bikol -> Bikol
        "bts" => Tag::new(b"BTS "), // Batak Simalungun
        // "bts" => Tag::new(b"BTK "), // Batak Simalungun -> Batak
        "btx" => Tag::new(b"BTX "), // Batak Karo
        // "btx" => Tag::new(b"BTK "), // Batak Karo -> Batak
        "btz" => Tag::new(b"BTZ "), // Batak Alas-Kluet
        // "btz" => Tag::new(b"BTK "), // Batak Alas-Kluet -> Batak
        // "bug" => 	tag: Tag::new(b"BUG "), // Buginese -> Bugis
        "bum" => Tag::new(b"BTI "), // Bulu (Cameroon) -> Beti
        "bve" => Tag::new(b"MLY "), // Berau Malay -> Malay
        "bvu" => Tag::new(b"MLY "), // Bukit Malay -> Malay
        "bwe" => Tag::new(b"KRN "), // Bwe Karen -> Karen
        "bxk" => Tag::new(b"LUH "), // Bukusu -> Luyia
        "bxo" => Tag::new(b"CPP "), // Barikanchi -> Creoles
        "bxp" => Tag::new(b"BTI "), // Bebil -> Beti
        "bxr" => Tag::new(b"RBU "), // Russia Buriat -> Russian Buriat
        "byn" => Tag::new(b"BIL "), // Bilin -> Bilen
        "byv" => Tag::new(b"BYV "), // Medumba
        // "byv" => Tag::new(b"BML "), // Medumba -> Bamileke
        "bzc" => Tag::new(b"MLG "), // Southern Betsimisaraka Malagasy -> Malagasy
        "bzj" => Tag::new(b"CPP "), // Belize Kriol English -> Creoles
        "bzk" => Tag::new(b"CPP "), // Nicaragua Creole English -> Creoles
        "ca" => Tag::new(b"CAT "),  // Catalan
        "caa" => Tag::new(b"MYN "), // Chortí -> Mayan
        "cac" => Tag::new(b"MYN "), // Chuj -> Mayan
        "caf" => Tag::new(b"CRR "), // Southern Carrier -> Carrier
        // "caf" => Tag::new(b"ATH "), // Southern Carrier -> Athapaskan
        "cak" => Tag::new(b"CAK "), // Kaqchikel
        // "cak" => Tag::new(b"MYN "), // Kaqchikel -> Mayan
        // "cay" => 	tag: Tag::new(b"CAY "), // Cayuga
        // "cbg" => 	tag: Tag::new(b"CBG "), // Chimila
        "cbk" => Tag::new(b"CBK "), // Chavacano -> Zamboanga Chavacano
        // "cbk" => Tag::new(b"CPP "), // Chavacano -> Creoles
        "cbl" => Tag::new(b"QIN "), // Bualkhaw Chin -> Chin
        "ccl" => Tag::new(b"CPP "), // Cutchi-Swahili -> Creoles
        "ccm" => Tag::new(b"CPP "), // Malaccan Creole Malay -> Creoles
        "cco" => Tag::new(b"CCHN"), // Comaltepec Chinantec -> Chinantec
        "ccq" => Tag::new(b"ARK "), // Chaungtha(retired code) -> Rakhine
        "cdo" => Tag::new(b"ZHS "), // Min Dong Chinese -> Chinese, Simplified
        "ce" => Tag::new(b"CHE "),  // Chechen
        // "ceb" => 	tag: Tag::new(b"CEB "), // Cebuano
        "cek" => Tag::new(b"QIN "), // Eastern Khumi Chin -> Chin
        "cey" => Tag::new(b"QIN "), // Ekai Chin -> Chin
        "cfm" => Tag::new(b"HAL "), // Halam (Falam Chin)
        // "cfm" => Tag::new(b"QIN "), // Falam Chin -> Chin
        // "cgg" => 	tag: Tag::new(b"CGG "), // Chiga
        "ch" => Tag::new(b"CHA "),  // Chamorro
        "chf" => Tag::new(b"MYN "), // Tabasco Chontal -> Mayan
        "chg" => Tag::new(&[0; 4]), // Chagatai != Chaha Gurage
        "chh" => Tag::new(&[0; 4]), // Chinook != Chattisgarhi
        "chj" => Tag::new(b"CCHN"), // Ojitlán Chinantec -> Chinantec
        "chk" => Tag::new(b"CHK0"), // Chuukese
        "chm" => Tag::new(b"HMA "), // Mari (Russia) [macrolanguage] -> High Mari
        // "chm" => Tag::new(b"LMA "), // Mari (Russia) [macrolanguage] -> Low Mari
        "chn" => Tag::new(b"CPP "), // Chinook jargon -> Creoles
        // "cho" => 	tag: Tag::new(b"CHO "), // Choctaw
        "chp" => Tag::new(b"CHP "), // Chipewyan
        // "chp" => Tag::new(b"SAY "), // Chipewyan -> Sayisi
        // "chp" => Tag::new(b"ATH "), // Chipewyan -> Athapaskan
        "chq" => Tag::new(b"CCHN"), // Quiotepec Chinantec -> Chinantec
        // "chr" => 	tag: Tag::new(b"CHR "), // Cherokee
        // "chy" => 	tag: Tag::new(b"CHY "), // Cheyenne
        "chz" => Tag::new(b"CCHN"), // Ozumacín Chinantec -> Chinantec
        "ciw" => Tag::new(b"OJB "), // Chippewa -> Ojibway
        // "cja" => 	tag: Tag::new(b"CJA "), // Western Cham
        // "cjm" => 	tag: Tag::new(b"CJM "), // Eastern Cham
        "cjy" => Tag::new(b"ZHS "), // Jinyu Chinese -> Chinese, Simplified
        "cka" => Tag::new(b"QIN "), // Khumi Awa Chin(retired code) -> Chin
        "ckb" => Tag::new(b"KUR "), // Central Kurdish -> Kurdish
        "ckn" => Tag::new(b"QIN "), // Kaang Chin -> Chin
        "cks" => Tag::new(b"CPP "), // Tayo -> Creoles
        "ckt" => Tag::new(b"CHK "), // Chukot -> Chukchi
        "ckz" => Tag::new(b"MYN "), // Cakchiquel-Quiché Mixed Language -> Mayan
        "clc" => Tag::new(b"ATH "), // Chilcotin -> Athapaskan
        "cld" => Tag::new(b"SYR "), // Chaldean Neo-Aramaic -> Syriac
        "cle" => Tag::new(b"CCHN"), // Lealao Chinantec -> Chinantec
        "clj" => Tag::new(b"QIN "), // Laitu Chin -> Chin
        "cls" => Tag::new(b"SAN "), // Classical Sanskrit -> Sanskrit
        "clt" => Tag::new(b"QIN "), // Lautu Chin -> Chin
        // "cmi" => 	tag: Tag::new(b"CMI "), // Emberá-Chamí
        "cmn" => Tag::new(b"ZHS "), // Mandarin Chinese -> Chinese, Simplified
        "cmr" => Tag::new(b"QIN "), // Mro-Khimi Chin -> Chin
        "cnb" => Tag::new(b"QIN "), // Chinbon Chin -> Chin
        "cnh" => Tag::new(b"QIN "), // Hakha Chin -> Chin
        "cnk" => Tag::new(b"QIN "), // Khumi Chin -> Chin
        "cnl" => Tag::new(b"CCHN"), // Lalana Chinantec -> Chinantec
        "cnp" => Tag::new(b"ZHS "), // Northern Ping Chinese -> Chinese, Simplified
        "cnr" => Tag::new(b"SRB "), // Montenegrin -> Serbian
        "cnt" => Tag::new(b"CCHN"), // Tepetotutla Chinantec -> Chinantec
        "cnu" => Tag::new(b"BBR "), // Chenoua -> Berber
        "cnw" => Tag::new(b"QIN "), // Ngawn Chin -> Chin
        "co" => Tag::new(b"COS "),  // Corsican
        "coa" => Tag::new(b"MLY "), // Cocos Islands Malay -> Malay
        "cob" => Tag::new(b"MYN "), // Chicomuceltec -> Mayan
        // "coo" => 	tag: Tag::new(b"COO "), // Comox
        // "cop" => 	tag: Tag::new(b"COP "), // Coptic
        "coq" => Tag::new(b"ATH "), // Coquille -> Athapaskan
        "cpa" => Tag::new(b"CCHN"), // Palantla Chinantec -> Chinantec
        "cpe" => Tag::new(b"CPP "), // English-based creoles and pidgins [collection] -> Creoles
        "cpf" => Tag::new(b"CPP "), // French-based creoles and pidgins [collection] -> Creoles
        "cpi" => Tag::new(b"CPP "), // Chinese Pidgin English -> Creoles
        // "cpp" => 	tag: Tag::new(b"CPP "), // Portuguese-based creoles and pidgins [collection] -> Creoles
        "cpx" => Tag::new(b"ZHS "), // Pu-Xian Chinese -> Chinese, Simplified
        "cqd" => Tag::new(b"HMN "), // Chuanqiandian Cluster Miao -> Hmong
        "cqu" => Tag::new(b"QUH "), // Chilean Quechua(retired code) -> Quechua (Bolivia)
        // "cqu" => Tag::new(b"QUZ "), // Chilean Quechua(retired code) -> Quechua
        "cr" => Tag::new(b"CRE "),  // Cree [macrolanguage]
        "crh" => Tag::new(b"CRT "), // Crimean Tatar
        "cri" => Tag::new(b"CPP "), // Sãotomense -> Creoles
        "crj" => Tag::new(b"ECR "), // Southern East Cree -> Eastern Cree
        // "crj" => Tag::new(b"YCR "), // Southern East Cree -> Y-Cree
        // "crj" => Tag::new(b"CRE "), // Southern East Cree -> Cree
        "crk" => Tag::new(b"WCR "), // Plains Cree -> West-Cree
        // "crk" => Tag::new(b"YCR "), // Plains Cree -> Y-Cree
        // "crk" => Tag::new(b"CRE "), // Plains Cree -> Cree
        "crl" => Tag::new(b"ECR "), // Northern East Cree -> Eastern Cree
        // "crl" => Tag::new(b"YCR "), // Northern East Cree -> Y-Cree
        // "crl" => Tag::new(b"CRE "), // Northern East Cree -> Cree
        "crm" => Tag::new(b"MCR "), // Moose Cree
        // "crm" => Tag::new(b"LCR "), // Moose Cree -> L-Cree
        // "crm" => Tag::new(b"CRE "), // Moose Cree -> Cree
        "crp" => Tag::new(b"CPP "), // Creoles and pidgins [collection] -> Creoles
        "crr" => Tag::new(&[0; 4]), // Carolina Algonquian != Carrier
        "crs" => Tag::new(b"CPP "), // Seselwa Creole French -> Creoles
        "crt" => Tag::new(&[0; 4]), // Iyojwa'ja Chorote != Crimean Tatar
        "crx" => Tag::new(b"CRR "), // Carrier
        // "crx" => Tag::new(b"ATH "), // Carrier -> Athapaskan
        "cs" => Tag::new(b"CSY "),  // Czech
        "csa" => Tag::new(b"CCHN"), // Chiltepec Chinantec -> Chinantec
        // "csb" => 	tag: Tag::new(b"CSB "), // Kashubian
        "csh" => Tag::new(b"QIN "), // Asho Chin -> Chin
        "csj" => Tag::new(b"QIN "), // Songlai Chin -> Chin
        "csl" => Tag::new(&[0; 4]), // Chinese Sign Language != Church Slavonic
        "cso" => Tag::new(b"CCHN"), // Sochiapam Chinantec -> Chinantec
        "csp" => Tag::new(b"ZHS "), // Southern Ping Chinese -> Chinese, Simplified
        "csv" => Tag::new(b"QIN "), // Sumtu Chin -> Chin
        "csw" => Tag::new(b"NCR "), // Swampy Cree -> N-Cree
        // "csw" => Tag::new(b"NHC "), // Swampy Cree -> Norway House Cree
        // "csw" => Tag::new(b"CRE "), // Swampy Cree -> Cree
        "csy" => Tag::new(b"QIN "), // Siyin Chin -> Chin
        "ctc" => Tag::new(b"ATH "), // Chetco -> Athapaskan
        "ctd" => Tag::new(b"QIN "), // Tedim Chin -> Chin
        "cte" => Tag::new(b"CCHN"), // Tepinapa Chinantec -> Chinantec
        // "ctg" => 	tag: Tag::new(b"CTG "), // Chittagonian
        "cth" => Tag::new(b"QIN "), // Thaiphum Chin -> Chin
        "ctl" => Tag::new(b"CCHN"), // Tlacoatzintepec Chinantec -> Chinantec
        // "cto" => 	tag: Tag::new(b"CTO "), // Emberá-Catío
        "cts" => Tag::new(b"BIK "), // Northern Catanduanes Bikol -> Bikol
        // "ctt" => 	tag: Tag::new(b"CTT "), // Wayanad Chetti
        "ctu" => Tag::new(b"MYN "), // Chol -> Mayan
        "cu" => Tag::new(b"CSL "),  // Church Slavonic
        "cuc" => Tag::new(b"CCHN"), // Usila Chinantec -> Chinantec
        // "cuk" => 	tag: Tag::new(b"CUK "), // San Blas Kuna
        "cv" => Tag::new(b"CHU "),  // Chuvash
        "cvn" => Tag::new(b"CCHN"), // Valle Nacional Chinantec -> Chinantec
        "cwd" => Tag::new(b"DCR "), // Woods Cree
        // "cwd" => Tag::new(b"TCR "), // Woods Cree -> TH-Cree
        // "cwd" => Tag::new(b"CRE "), // Woods Cree -> Cree
        "cy" => Tag::new(b"WEL "),  // Welsh
        "czh" => Tag::new(b"ZHS "), // Huizhou Chinese -> Chinese, Simplified
        "czo" => Tag::new(b"ZHS "), // Min Zhong Chinese -> Chinese, Simplified
        "czt" => Tag::new(b"QIN "), // Zotung Chin -> Chin
        "da" => Tag::new(b"DAN "),  // Danish
        // "dag" => 	tag: Tag::new(b"DAG "), // Dagbani
        "dao" => Tag::new(b"QIN "), // Daai Chin -> Chin
        "dap" => Tag::new(b"NIS "), // Nisi (India)(retired code)
        // "dar" => 	tag: Tag::new(b"DAR "), // Dargwa
        // "dax" => 	tag: Tag::new(b"DAX "), // Dayi
        "dcr" => Tag::new(b"CPP "), // Negerhollands -> Creoles
        "de" => Tag::new(b"DEU "),  // German
        "den" => Tag::new(b"SLA "), // Slave (Athapascan) [macrolanguage] -> Slavey
        // "den" => Tag::new(b"ATH "), // Slave (Athapascan) [macrolanguage] -> Athapaskan
        "dep" => Tag::new(b"CPP "), // Pidgin Delaware -> Creoles
        "dgo" => Tag::new(b"DGO "), // Dogri (individual language)
        // "dgo" => Tag::new(b"DGR "), // Dogri (macrolanguage)
        "dgr" => Tag::new(b"ATH "), // Tlicho -> Athapaskan
        "dhd" => Tag::new(b"MAW "), // Dhundari -> Marwari
        // "dhg" => 	tag: Tag::new(b"DHG "), // Dhangu
        "dhv" => Tag::new(&[0; 4]), // Dehu != Divehi (Dhivehi, Maldivian) (deprecated)
        "dib" => Tag::new(b"DNK "), // South Central Dinka -> Dinka
        "dik" => Tag::new(b"DNK "), // Southwestern Dinka -> Dinka
        "din" => Tag::new(b"DNK "), // Dinka [macrolanguage]
        "dip" => Tag::new(b"DNK "), // Northeastern Dinka -> Dinka
        "diq" => Tag::new(b"DIQ "), // Dimli
        // "diq" => Tag::new(b"ZZA "), // Dimli  -> Zazaki
        "diw" => Tag::new(b"DNK "), // Northwestern Dinka -> Dinka
        "dje" => Tag::new(b"DJR "), // Zarma
        "djk" => Tag::new(b"CPP "), // Eastern Maroon Creole -> Creoles
        "djr" => Tag::new(b"DJR0"), // Djambarrpuyngu
        "dks" => Tag::new(b"DNK "), // Southeastern Dinka -> Dinka
        "dng" => Tag::new(b"DUN "), // Dungan
        // "dnj" => 	tag: Tag::new(b"DNJ "), // Dan
        "dnk" => Tag::new(&[0; 4]), // Dengka != Dinka
        "doi" => Tag::new(b"DGR "), // Dogri (macrolanguage) [macrolanguage]
        "drh" => Tag::new(b"MNG "), // Darkhat(retired code) -> Mongolian
        "dri" => Tag::new(&[0; 4]), // C'Lela != Dari
        "drw" => Tag::new(b"DRI "), // Darwazi(retired code) -> Dari
        // "drw" => Tag::new(b"FAR "), // Darwazi(retired code) -> Persian
        "dsb" => Tag::new(b"LSB "), // Lower Sorbian
        "dty" => Tag::new(b"NEP "), // Dotyali -> Nepali
        // "duj" => 	tag: Tag::new(b"DUJ "), // Dhuwal(retired code)
        "dun" => Tag::new(&[0; 4]), // Dusun Deyah != Dungan
        "dup" => Tag::new(b"MLY "), // Duano -> Malay
        "dv" => Tag::new(b"DIV "),  // Divehi (Dhivehi, Maldivian)
        // "dv" => Tag::new(b"DHV "),  // Divehi (Dhivehi, Maldivian) (deprecated)
        "dwk" => Tag::new(b"KUI "), // Dawik Kui -> Kui
        "dwu" => Tag::new(b"DUJ "), // Dhuwal
        "dwy" => Tag::new(b"DUJ "), // Dhuwaya -> Dhuwal
        "dyu" => Tag::new(b"JUL "), // Dyula -> Jula
        "dz" => Tag::new(b"DZN "),  // Dzongkha
        "dzn" => Tag::new(&[0; 4]), // Dzando != Dzongkha
        "ecr" => Tag::new(&[0; 4]), // Eteocretan != Eastern Cree
        "ee" => Tag::new(b"EWE "),  // Ewe
        // "efi" => 	tag: Tag::new(b"EFI "), // Efik
        "ekk" => Tag::new(b"ETI "), // Standard Estonian -> Estonian
        "eky" => Tag::new(b"KRN "), // Eastern Kayah -> Karen
        "el" => Tag::new(b"ELL "),  // Modern Greek (1453-) -> Greek
        "emk" => Tag::new(b"EMK "), // Eastern Maninkakan
        // "emk" => Tag::new(b"MNK "), // Eastern Maninkakan -> Maninka
        // "emp" => 	tag: Tag::new(b"EMP "), // Northern Emberá
        "emy" => Tag::new(b"MYN "), // Epigraphic Mayan -> Mayan
        "en" => Tag::new(b"ENG "),  // English
        "enb" => Tag::new(b"KAL "), // Markweeta -> Kalenjin
        "enf" => Tag::new(b"FNE "), // Forest Enets
        "enh" => Tag::new(b"TNE "), // Tundra Enets
        "eo" => Tag::new(b"NTO "),  // Esperanto
        "es" => Tag::new(b"ESP "),  // Spanish
        "esg" => Tag::new(b"GON "), // Aheri Gondi -> Gondi
        "esi" => Tag::new(b"IPK "), // North Alaskan Inupiatun -> Inupiat
        "esk" => Tag::new(b"IPK "), // Northwest Alaska Inupiatun -> Inupiat
        // "esu" => 	tag: Tag::new(b"ESU "), // Central Yupik
        "et" => Tag::new(b"ETI "),  // Estonian [macrolanguage]
        "eto" => Tag::new(b"BTI "), // Eton (Cameroon) -> Beti
        "eu" => Tag::new(b"EUQ "),  // Basque
        "euq" => Tag::new(&[0; 4]), // Basque  [collection] != Basque
        "eve" => Tag::new(b"EVN "), // Even
        "evn" => Tag::new(b"EVK "), // Evenki
        "ewo" => Tag::new(b"BTI "), // Ewondo -> Beti
        "eyo" => Tag::new(b"KAL "), // Keiyo -> Kalenjin
        "fa" => Tag::new(b"FAR "),  // Persian [macrolanguage]
        "fab" => Tag::new(b"CPP "), // Fa d'Ambu -> Creoles
        "fan" => Tag::new(b"FAN0"), // Fang (Equatorial Guinea)
        // "fan" => Tag::new(b"BTI "), // Fang (Equatorial Guinea) -> Beti
        "far" => Tag::new(&[0; 4]), // Fataleka != Persian
        "fat" => Tag::new(b"FAT "), // Fanti
        // "fat" => Tag::new(b"AKA "), // Fanti -> Akan
        "fbl" => Tag::new(b"BIK "), // West Albay Bikol -> Bikol
        "ff" => Tag::new(b"FUL "),  // Fulah [macrolanguage]
        "ffm" => Tag::new(b"FUL "), // Maasina Fulfulde -> Fulah
        "fi" => Tag::new(b"FIN "),  // Finnish
        "fil" => Tag::new(b"PIL "), // Filipino
        "fj" => Tag::new(b"FJI "),  // Fijian
        "flm" => Tag::new(b"HAL "), // Halam (Falam Chin)(retired code)
        // "flm" => Tag::new(b"QIN "), // Falam Chin(retired code) -> Chin
        "fmp" => Tag::new(b"FMP "), // Fe’fe’
        // "fmp" => Tag::new(b"BML "), // Fe'fe' -> Bamileke
        "fng" => Tag::new(b"CPP "), // Fanagalo -> Creoles
        "fo" => Tag::new(b"FOS "),  // Faroese
        // "fon" => 	tag: Tag::new(b"FON "), // Fon
        "fos" => Tag::new(&[0; 4]), // Siraya != Faroese
        "fpe" => Tag::new(b"CPP "), // Fernando Po Creole English -> Creoles
        "fr" => Tag::new(b"FRA "),  // French
        // "frc" => 	tag: Tag::new(b"FRC "), // Cajun French
        // "frp" => 	tag: Tag::new(b"FRP "), // Arpitan
        "fub" => Tag::new(b"FUL "), // Adamawa Fulfulde -> Fulah
        "fuc" => Tag::new(b"FUL "), // Pulaar -> Fulah
        "fue" => Tag::new(b"FUL "), // Borgu Fulfulde -> Fulah
        "fuf" => Tag::new(b"FTA "), // Pular -> Futa
        // "fuf" => Tag::new(b"FUL "), // Pular -> Fulah
        "fuh" => Tag::new(b"FUL "), // Western Niger Fulfulde -> Fulah
        "fui" => Tag::new(b"FUL "), // Bagirmi Fulfulde -> Fulah
        "fuq" => Tag::new(b"FUL "), // Central-Eastern Niger Fulfulde -> Fulah
        "fur" => Tag::new(b"FRL "), // Friulian
        "fuv" => Tag::new(b"FUV "), // Nigerian Fulfulde
        // "fuv" => Tag::new(b"FUL "), // Nigerian Fulfulde -> Fulah
        "fy" => Tag::new(b"FRI "), // Western Frisian -> Frisian
        "ga" => Tag::new(b"IRI "), // Irish
        // "ga" => Tag::new(b"IRT "),  // Irish -> Irish Traditional
        "gaa" => Tag::new(b"GAD "), // Ga
        "gac" => Tag::new(b"CPP "), // Mixed Great Andamanese -> Creoles
        "gad" => Tag::new(&[0; 4]), // Gaddang != Ga
        "gae" => Tag::new(&[0; 4]), // Guarequena != Scottish Gaelic
        // "gag" => 	tag: Tag::new(b"GAG "), // Gagauz
        "gal" => Tag::new(&[0; 4]), // Galolen != Galician
        "gan" => Tag::new(b"ZHS "), // Gan Chinese -> Chinese, Simplified
        "gar" => Tag::new(&[0; 4]), // Galeya != Garshuni
        "gaw" => Tag::new(&[0; 4]), // Nobonob != Garhwali
        "gax" => Tag::new(b"ORO "), // Borana-Arsi-Guji Oromo -> Oromo
        "gaz" => Tag::new(b"ORO "), // West Central Oromo -> Oromo
        "gbm" => Tag::new(b"GAW "), // Garhwali
        "gce" => Tag::new(b"ATH "), // Galice -> Athapaskan
        "gcf" => Tag::new(b"CPP "), // Guadeloupean Creole French -> Creoles
        "gcl" => Tag::new(b"CPP "), // Grenadian Creole English -> Creoles
        "gcr" => Tag::new(b"CPP "), // Guianese Creole French -> Creoles
        "gd" => Tag::new(b"GAE "),  // Scottish Gaelic
        "gda" => Tag::new(b"RAJ "), // Gade Lohar -> Rajasthani
        // "gez" => 	tag: Tag::new(b"GEZ "), // Geez
        "ggo" => Tag::new(b"GON "), // Southern Gondi(retired code) -> Gondi
        "gha" => Tag::new(b"BBR "), // Ghadamès -> Berber
        "ghc" => Tag::new(b"IRT "), // Hiberno-Scottish Gaelic -> Irish Traditional
        "ghk" => Tag::new(b"KRN "), // Geko Karen -> Karen
        "gho" => Tag::new(b"BBR "), // Ghomara -> Berber
        "gib" => Tag::new(b"CPP "), // Gibanawa -> Creoles
        // "gih" => 	tag: Tag::new(b"GIH "), // Githabul
        "gil" => Tag::new(b"GIL0"), // Kiribati (Gilbertese)
        "gju" => Tag::new(b"RAJ "), // Gujari -> Rajasthani
        "gkp" => Tag::new(b"GKP "), // Guinea Kpelle -> Kpelle (Guinea)
        // "gkp" => Tag::new(b"KPL "), // Guinea Kpelle -> Kpelle
        "gl" => Tag::new(b"GAL "),  // Galician
        "gld" => Tag::new(b"NAN "), // Nanai
        // "glk" => 	tag: Tag::new(b"GLK "), // Gilaki
        "gmz" => Tag::new(&[0; 4]), // Mgbolizhia != Gumuz
        "gn" => Tag::new(b"GUA "),  // Guarani [macrolanguage]
        "gnb" => Tag::new(b"QIN "), // Gangte -> Chin
        // "gnn" => 	tag: Tag::new(b"GNN "), // Gumatj
        "gno" => Tag::new(b"GON "), // Northern Gondi -> Gondi
        "gnw" => Tag::new(b"GUA "), // Western Bolivian Guaraní -> Guarani
        // "gog" => 	tag: Tag::new(b"GOG "), // Gogo
        "gom" => Tag::new(b"KOK "), // Goan Konkani -> Konkani
        // "gon" => 	tag: Tag::new(b"GON "), // Gondi [macrolanguage]
        "goq" => Tag::new(b"CPP "), // Gorap -> Creoles
        "gox" => Tag::new(b"BAD0"), // Gobu -> Banda
        "gpe" => Tag::new(b"CPP "), // Ghanaian Pidgin English -> Creoles
        "gro" => Tag::new(&[0; 4]), // Groma != Garo
        "grr" => Tag::new(b"BBR "), // Taznatit -> Berber
        "grt" => Tag::new(b"GRO "), // Garo
        "gru" => Tag::new(b"SOG "), // Kistane -> Sodo Gurage
        "gsw" => Tag::new(b"ALS "), // Alsatian
        "gu" => Tag::new(b"GUJ "),  // Gujarati
        "gua" => Tag::new(&[0; 4]), // Shiki != Guarani
        // "guc" => 	tag: Tag::new(b"GUC "), // Wayuu
        // "guf" => 	tag: Tag::new(b"GUF "), // Gupapuyngu
        "gug" => Tag::new(b"GUA "), // Paraguayan Guaraní -> Guarani
        "gui" => Tag::new(b"GUA "), // Eastern Bolivian Guaraní -> Guarani
        "guk" => Tag::new(b"GMZ "), // Gumuz
        "gul" => Tag::new(b"CPP "), // Sea Island Creole English -> Creoles
        "gun" => Tag::new(b"GUA "), // Mbyá Guaraní -> Guarani
        // "guz" => 	tag: Tag::new(b"GUZ "), // Gusii
        "gv" => Tag::new(b"MNX "),  // Manx
        "gwi" => Tag::new(b"ATH "), // Gwichʼin -> Athapaskan
        "gyn" => Tag::new(b"CPP "), // Guyanese Creole English -> Creoles
        "ha" => Tag::new(b"HAU "),  // Hausa
        "haa" => Tag::new(b"ATH "), // Hän -> Athapaskan
        "hae" => Tag::new(b"ORO "), // Eastern Oromo -> Oromo
        "hai" => Tag::new(b"HAI0"), // Haida [macrolanguage]
        "hak" => Tag::new(b"ZHS "), // Hakka Chinese -> Chinese, Simplified
        "hal" => Tag::new(&[0; 4]), // Halang != Halam (Falam Chin)
        "har" => Tag::new(b"HRI "), // Harari
        // "haw" => 	tag: Tag::new(b"HAW "), // Hawaiian
        "hax" => Tag::new(b"HAI0"), // Southern Haida -> Haida
        // "hay" => 	tag: Tag::new(b"HAY "), // Haya
        // "haz" => 	tag: Tag::new(b"HAZ "), // Hazaragi
        "hbn" => Tag::new(&[0; 4]), // Heiban != Hammer-Banna
        "hca" => Tag::new(b"CPP "), // Andaman Creole Hindi -> Creoles
        "hdn" => Tag::new(b"HAI0"), // Northern Haida -> Haida
        "he" => Tag::new(b"IWR "),  // Hebrew
        "hea" => Tag::new(b"HMN "), // Northern Qiandong Miao -> Hmong
        // "hei" => 	tag: Tag::new(b"HEI "), // Heiltsuk
        "hi" => Tag::new(b"HIN "), // Hindi
        // "hil" => 	tag: Tag::new(b"HIL "), // Hiligaynon
        "hji" => Tag::new(b"MLY "), // Haji -> Malay
        "hlt" => Tag::new(b"QIN "), // Matu Chin -> Chin
        "hma" => Tag::new(b"HMN "), // Southern Mashan Hmong -> Hmong
        "hmc" => Tag::new(b"HMN "), // Central Huishui Hmong -> Hmong
        "hmd" => Tag::new(b"HMD "), // Large Flowery Miao -> A-Hmao
        // "hmd" => Tag::new(b"HMN "), // Large Flowery Miao -> Hmong
        "hme" => Tag::new(b"HMN "), // Eastern Huishui Hmong -> Hmong
        "hmg" => Tag::new(b"HMN "), // Southwestern Guiyang Hmong -> Hmong
        "hmh" => Tag::new(b"HMN "), // Southwestern Huishui Hmong -> Hmong
        "hmi" => Tag::new(b"HMN "), // Northern Huishui Hmong -> Hmong
        "hmj" => Tag::new(b"HMN "), // Ge -> Hmong
        "hml" => Tag::new(b"HMN "), // Luopohe Hmong -> Hmong
        "hmm" => Tag::new(b"HMN "), // Central Mashan Hmong -> Hmong
        // "hmn" => 	tag: Tag::new(b"HMN "), // Hmong [macrolanguage]
        "hmp" => Tag::new(b"HMN "), // Northern Mashan Hmong -> Hmong
        "hmq" => Tag::new(b"HMN "), // Eastern Qiandong Miao -> Hmong
        "hmr" => Tag::new(b"QIN "), // Hmar -> Chin
        "hms" => Tag::new(b"HMN "), // Southern Qiandong Miao -> Hmong
        "hmw" => Tag::new(b"HMN "), // Western Mashan Hmong -> Hmong
        "hmy" => Tag::new(b"HMN "), // Southern Guiyang Hmong -> Hmong
        "hmz" => Tag::new(b"HMZ "), // Hmong Shua -> Hmong Shuat
        // "hmz" => Tag::new(b"HMN "), // Hmong Shua -> Hmong
        // "hnd" => 	tag: Tag::new(b"HND "), // Southern Hindko -> Hindko
        "hne" => Tag::new(b"CHH "), // Chhattisgarhi -> Chattisgarhi
        "hnj" => Tag::new(b"HMN "), // Hmong Njua -> Hmong
        "hnm" => Tag::new(b"ZHS "), // Hainanese -> Chinese, Simplified
        "hno" => Tag::new(b"HND "), // Northern Hindko -> Hindko
        "ho" => Tag::new(b"HMO "),  // Hiri Motu
        // "ho" => Tag::new(b"CPP "),  // Hiri Motu -> Creoles
        "hoc" => Tag::new(b"HO  "), // Ho
        "hoi" => Tag::new(b"ATH "), // Holikachuk -> Athapaskan
        "hoj" => Tag::new(b"HAR "), // Hadothi -> Harauti
        // "hoj" => Tag::new(b"RAJ "), // Hadothi -> Rajasthani
        "hr" => Tag::new(b"HRV "),  // Croatian
        "hra" => Tag::new(b"QIN "), // Hrangkhol -> Chin
        "hrm" => Tag::new(b"HMN "), // Horned Miao -> Hmong
        "hsb" => Tag::new(b"USB "), // Upper Sorbian
        "hsn" => Tag::new(b"ZHS "), // Xiang Chinese -> Chinese, Simplified
        "ht" => Tag::new(b"HAI "),  // Haitian (Haitian Creole)
        // "ht" => Tag::new(b"CPP "),  // Haitian -> Creoles
        "hu" => Tag::new(b"HUN "),  // Hungarian
        "huj" => Tag::new(b"HMN "), // Northern Guiyang Hmong -> Hmong
        "hup" => Tag::new(b"ATH "), // Hupa -> Athapaskan
        // "hur" => 	tag: Tag::new(b"HUR "), // Halkomelem
        "hus" => Tag::new(b"MYN "), // Huastec -> Mayan
        "hwc" => Tag::new(b"CPP "), // Hawai'i Creole English -> Creoles
        "hy" => Tag::new(b"HYE0"),  // Armenian -> Armenian East
        // "hy" => Tag::new(b"HYE "),  // Armenian
        "hyw" => Tag::new(b"HYE "), // Western Armenian -> Armenian
        "hz" => Tag::new(b"HER "),  // Herero
        "ia" => Tag::new(b"INA "),  // Interlingua (International Auxiliary Language Association)
        // "iba" => 	tag: Tag::new(b"IBA "), // Iban
        // "ibb" => 	tag: Tag::new(b"IBB "), // Ibibio
        "iby" => Tag::new(b"IJO "), // Ibani -> Ijo
        "icr" => Tag::new(b"CPP "), // Islander Creole English -> Creoles
        "id" => Tag::new(b"IND "),  // Indonesian
        // "id" => Tag::new(b"MLY "),  // Indonesian -> Malay
        "ida" => Tag::new(b"LUH "), // Idakho-Isukha-Tiriki -> Luyia
        "idb" => Tag::new(b"CPP "), // Indo-Portuguese -> Creoles
        "ie" => Tag::new(b"ILE "),  // Interlingue
        "ig" => Tag::new(b"IBO "),  // Igbo
        "igb" => Tag::new(b"EBI "), // Ebira
        "ihb" => Tag::new(b"CPP "), // Iha Based Pidgin -> Creoles
        "ii" => Tag::new(b"YIM "),  // Sichuan Yi -> Yi Modern
        "ijc" => Tag::new(b"IJO "), // Izon -> Ijo
        "ije" => Tag::new(b"IJO "), // Biseni -> Ijo
        "ijn" => Tag::new(b"IJO "), // Kalabari -> Ijo
        // "ijo" => 	tag: Tag::new(b"IJO "), // Ijo  [collection]
        "ijs" => Tag::new(b"IJO "), // Southeast Ijo -> Ijo
        "ik" => Tag::new(b"IPK "),  // Inupiaq [macrolanguage] -> Inupiat
        "ike" => Tag::new(b"INU "), // Eastern Canadian Inuktitut -> Inuktitut
        // "ike" => Tag::new(b"INUK"), // Eastern Canadian Inuktitut -> Nunavik Inuktitut
        "ikt" => Tag::new(b"INU "), // Inuinnaqtun -> Inuktitut
        // "ilo" => 	tag: Tag::new(b"ILO "), // Iloko -> Ilokano
        "in" => Tag::new(b"IND "), // Indonesian(retired code)
        // "in" => Tag::new(b"MLY "),  // Indonesian(retired code) -> Malay
        "ing" => Tag::new(b"ATH "), // Degexit'an -> Athapaskan
        "inh" => Tag::new(b"ING "), // Ingush
        "io" => Tag::new(b"IDO "),  // Ido
        "iri" => Tag::new(&[0; 4]), // Rigwe != Irish
        // "iru" => 	tag: Tag::new(b"IRU "), // Irula
        "is" => Tag::new(b"ISL "),  // Icelandic
        "ism" => Tag::new(&[0; 4]), // Masimasi != Inari Sami
        "it" => Tag::new(b"ITA "),  // Italian
        "itz" => Tag::new(b"MYN "), // Itzá -> Mayan
        "iu" => Tag::new(b"INU "),  // Inuktitut [macrolanguage]
        // "iu" => Tag::new(b"INUK"),  // Inuktitut [macrolanguage] -> Nunavik Inuktitut
        "iw" => Tag::new(b"IWR "),  // Hebrew(retired code)
        "ixl" => Tag::new(b"MYN "), // Ixil -> Mayan
        "ja" => Tag::new(b"JAN "),  // Japanese
        "jac" => Tag::new(b"MYN "), // Popti' -> Mayan
        "jak" => Tag::new(b"MLY "), // Jakun -> Malay
        "jam" => Tag::new(b"JAM "), // Jamaican Creole English -> Jamaican Creole
        // "jam" => Tag::new(b"CPP "), // Jamaican Creole English -> Creoles
        "jan" => Tag::new(&[0; 4]), // Jandai != Japanese
        "jax" => Tag::new(b"MLY "), // Jambi Malay -> Malay
        "jbe" => Tag::new(b"BBR "), // Judeo-Berber -> Berber
        "jbn" => Tag::new(b"BBR "), // Nafusi -> Berber
        // "jbo" => 	tag: Tag::new(b"JBO "), // Lojban
        // "jct" => 	tag: Tag::new(b"JCT "), // Krymchak
        // "jdt" => 	tag: Tag::new(b"JDT "), // Judeo-Tat
        "jgo" => Tag::new(b"BML "), // Ngomba -> Bamileke
        "ji" => Tag::new(b"JII "),  // Yiddish(retired code)
        "jii" => Tag::new(&[0; 4]), // Jiiddu != Yiddish
        "jkm" => Tag::new(b"KRN "), // Mobwa Karen -> Karen
        "jkp" => Tag::new(b"KRN "), // Paku Karen -> Karen
        "jud" => Tag::new(&[0; 4]), // Worodougou != Ladino
        "jul" => Tag::new(&[0; 4]), // Jirel != Jula
        "jv" => Tag::new(b"JAV "),  // Javanese
        "jvd" => Tag::new(b"CPP "), // Javindo -> Creoles
        "jw" => Tag::new(b"JAV "),  // Javanese(retired code)
        "ka" => Tag::new(b"KAT "),  // Georgian
        "kaa" => Tag::new(b"KRK "), // Karakalpak
        "kab" => Tag::new(b"KAB0"), // Kabyle
        // "kab" => Tag::new(b"BBR "), // Kabyle -> Berber
        "kac" => Tag::new(&[0; 4]), // Kachin != Kachchi
        "kam" => Tag::new(b"KMB "), // Kamba (Kenya)
        "kar" => Tag::new(b"KRN "), // Karen  [collection]
        // "kaw" => 	tag: Tag::new(b"KAW "), // Kawi (Old Javanese)
        // "kbc" => 	tag: Tag::new(b"KBC "), // Kadiwéu
        "kbd" => Tag::new(b"KAB "), // Kabardian
        "kby" => Tag::new(b"KNR "), // Manga Kanuri -> Kanuri
        "kca" => Tag::new(b"KHK "), // Khanty -> Khanty-Kazim
        // "kca" => Tag::new(b"KHS "), // Khanty -> Khanty-Shurishkar
        // "kca" => Tag::new(b"KHV "), // Khanty -> Khanty-Vakhi
        "kcn" => Tag::new(b"CPP "), // Nubi -> Creoles
        // "kde" => 	tag: Tag::new(b"KDE "), // Makonde
        "kdr" => Tag::new(b"KRM "), // Karaim
        "kdt" => Tag::new(b"KUY "), // Kuy
        "kea" => Tag::new(b"KEA "), // Kabuverdianu (Crioulo)
        // "kea" => Tag::new(b"CPP "), // Kabuverdianu -> Creoles
        "keb" => Tag::new(&[0; 4]), // Kélé != Kebena
        "kek" => Tag::new(b"KEK "), // Kekchi
        // "kek" => Tag::new(b"MYN "), // Kekchí -> Mayan
        "kex" => Tag::new(b"KKN "), // Kukna -> Kokni
        "kfa" => Tag::new(b"KOD "), // Kodava -> Kodagu
        "kfr" => Tag::new(b"KAC "), // Kachhi -> Kachchi
        "kfx" => Tag::new(b"KUL "), // Kullu Pahari -> Kulvi
        "kfy" => Tag::new(b"KMN "), // Kumaoni
        "kg" => Tag::new(b"KON0"),  // Kongo [macrolanguage]
        "kge" => Tag::new(&[0; 4]), // Komering != Khutsuri Georgian
        // "kgf" => 	tag: Tag::new(b"KGF "), // Kube
        "kha" => Tag::new(b"KSI "), // Khasi
        "khb" => Tag::new(b"XBD "), // Lü
        "khk" => Tag::new(b"MNG "), // Halh Mongolian -> Mongolian
        "khn" => Tag::new(&[0; 4]), // Khandesi != Khamti Shan(Microsoft fonts)
        "khs" => Tag::new(&[0; 4]), // Kasua != Khanty-Shurishkar
        "kht" => Tag::new(b"KHT "), // Khamti -> Khamti Shan
        // "kht" => Tag::new(b"KHN "), // Khamti -> Khamti Shan(Microsoft fonts)
        "khv" => Tag::new(&[0; 4]), // Khvarshi != Khanty-Vakhi
        // "khw" => 	tag: Tag::new(b"KHW "), // Khowar
        "ki" => Tag::new(b"KIK "),  // Kikuyu (Gikuyu)
        "kis" => Tag::new(&[0; 4]), // Kis != Kisii
        "kiu" => Tag::new(b"KIU "), // Kirmanjki
        // "kiu" => Tag::new(b"ZZA "), // Kirmanjki  -> Zazaki
        "kj" => Tag::new(b"KUA "),  // Kuanyama
        "kjb" => Tag::new(b"MYN "), // Q'anjob'al -> Mayan
        // "kjd" => 	tag: Tag::new(b"KJD "), // Southern Kiwai
        "kjh" => Tag::new(b"KHA "), // Khakas -> Khakass
        // "kjj" => 	tag: Tag::new(b"KJJ "), // Khinalugh -> Khinalug
        "kjp" => Tag::new(b"KJP "), // Pwo Eastern Karen -> Eastern Pwo Karen
        // "kjp" => Tag::new(b"KRN "), // Pwo Eastern Karen -> Karen
        "kjt" => Tag::new(b"KRN "), // Phrae Pwo Karen -> Karen
        // "kjz" => 	tag: Tag::new(b"KJZ "), // Bumthangkha
        "kk" => Tag::new(b"KAZ "),  // Kazakh
        "kkn" => Tag::new(&[0; 4]), // Kon Keu != Kokni
        "kkz" => Tag::new(b"ATH "), // Kaska -> Athapaskan
        "kl" => Tag::new(b"GRN "),  // Greenlandic
        "klm" => Tag::new(&[0; 4]), // Migum != Kalmyk
        "kln" => Tag::new(b"KAL "), // Kalenjin [macrolanguage]
        "km" => Tag::new(b"KHM "),  // Khmer
        "kmb" => Tag::new(b"MBN "), // Kimbundu -> Mbundu
        // "kmg" => 	tag: Tag::new(b"KMG "), // Kâte
        "kmn" => Tag::new(&[0; 4]), // Awtuw != Kumaoni
        "kmo" => Tag::new(&[0; 4]), // Kwoma != Komo
        "kmr" => Tag::new(b"KUR "), // Northern Kurdish -> Kurdish
        "kms" => Tag::new(&[0; 4]), // Kamasau != Komso
        "kmv" => Tag::new(b"CPP "), // Karipúna Creole French -> Creoles
        "kmw" => Tag::new(b"KMO "), // Komo (Democratic Republic of Congo)
        // "kmz" => 	tag: Tag::new(b"KMZ "), // Khorasani Turkish -> Khorasani Turkic
        "kn" => Tag::new(b"KAN "),  // Kannada
        "knc" => Tag::new(b"KNR "), // Central Kanuri -> Kanuri
        "kng" => Tag::new(b"KON0"), // Koongo -> Kongo
        "knj" => Tag::new(b"MYN "), // Western Kanjobal -> Mayan
        "knn" => Tag::new(b"KOK "), // Konkani
        "knr" => Tag::new(&[0; 4]), // Kaningra != Kanuri
        "ko" => Tag::new(b"KOR "),  // Korean
        "kod" => Tag::new(&[0; 4]), // Kodi != Kodagu
        "koh" => Tag::new(&[0; 4]), // Koyo != Korean Old Hangul
        "koi" => Tag::new(b"KOP "), // Komi-Permyak
        // "kok" => 	tag: Tag::new(b"KOK "), // Konkani  [macrolanguage]
        "kop" => Tag::new(&[0; 4]), // Waube != Komi-Permyak
        // "kos" => 	tag: Tag::new(b"KOS "), // Kosraean
        "koy" => Tag::new(b"ATH "), // Koyukon -> Athapaskan
        "koz" => Tag::new(&[0; 4]), // Korak != Komi-Zyrian
        "kpe" => Tag::new(b"KPL "), // Kpelle [macrolanguage]
        "kpl" => Tag::new(&[0; 4]), // Kpala != Kpelle
        "kpp" => Tag::new(b"KRN "), // Paku Karen(retired code) -> Karen
        "kpv" => Tag::new(b"KOZ "), // Komi-Zyrian
        "kpy" => Tag::new(b"KYK "), // Koryak
        "kqs" => Tag::new(b"KIS "), // Northern Kissi -> Kisii
        "kqy" => Tag::new(b"KRT "), // Koorete
        "kr" => Tag::new(b"KNR "),  // Kanuri [macrolanguage]
        "krc" => Tag::new(b"KAR "), // Karachay-Balkar -> Karachay
        "kri" => Tag::new(b"KRI "), // Krio
        "krk" => Tag::new(&[0; 4]), // Kerek != Karakalpak
        // "krl" => 	tag: Tag::new(b"KRL "), // Karelian
        "krm" => Tag::new(&[0; 4]), // Krim(retired code) != Karaim
        "krn" => Tag::new(&[0; 4]), // Sapo != Karen
        "krt" => Tag::new(b"KNR "), // Tumari Kanuri -> Kanuri
        "kru" => Tag::new(b"KUU "), // Kurukh
        "ks" => Tag::new(b"KSH "),  // Kashmiri
        "ksh" => Tag::new(b"KSH0"), // Kölsch -> Ripuarian
        "ksi" => Tag::new(&[0; 4]), // Krisa != Khasi
        "ksm" => Tag::new(&[0; 4]), // Kumba != Kildin Sami
        "kss" => Tag::new(b"KIS "), // Southern Kisi -> Kisii
        // "ksu" => 	tag: Tag::new(b"KSU "), // Khamyang
        "ksw" => Tag::new(b"KSW "), // S’gaw Karen
        // "ksw" => Tag::new(b"KRN "), // S'gaw Karen -> Karen
        "ktb" => Tag::new(b"KEB "), // Kambaata -> Kebena
        "ktu" => Tag::new(b"KON "), // Kituba (Democratic Republic of Congo) -> Kikongo
        "ktw" => Tag::new(b"ATH "), // Kato -> Athapaskan
        "ku" => Tag::new(b"KUR "),  // Kurdish [macrolanguage]
        "kui" => Tag::new(&[0; 4]), // Kuikúro-Kalapálo != Kui
        "kul" => Tag::new(&[0; 4]), // Kulere != Kulvi
        // "kum" => 	tag: Tag::new(b"KUM "), // Kumyk
        "kuu" => Tag::new(b"ATH "), // Upper Kuskokwim -> Athapaskan
        "kuw" => Tag::new(b"BAD0"), // Kpagua -> Banda
        "kuy" => Tag::new(&[0; 4]), // Kuuku-Ya'u != Kuy
        "kv" => Tag::new(b"KOM "),  // Komi [macrolanguage]
        "kvb" => Tag::new(b"MLY "), // Kubu -> Malay
        "kvl" => Tag::new(b"KRN "), // Kayaw -> Karen
        "kvq" => Tag::new(b"KVQ "), // Geba Karen
        // "kvq" => Tag::new(b"KRN "), // Geba Karen -> Karen
        "kvr" => Tag::new(b"MLY "), // Kerinci -> Malay
        "kvt" => Tag::new(b"KRN "), // Lahta Karen -> Karen
        "kvu" => Tag::new(b"KRN "), // Yinbaw Karen -> Karen
        "kvy" => Tag::new(b"KRN "), // Yintale Karen -> Karen
        "kw" => Tag::new(b"COR "),  // Cornish
        // "kwk" => 	tag: Tag::new(b"KWK "), // Kwak'wala -> Kwakʼwala
        "kww" => Tag::new(b"CPP "), // Kwinti -> Creoles
        "kwy" => Tag::new(b"KON0"), // San Salvador Kongo -> Kongo
        "kxc" => Tag::new(b"KMS "), // Konso -> Komso
        "kxd" => Tag::new(b"MLY "), // Brunei -> Malay
        "kxf" => Tag::new(b"KRN "), // Manumanaw Karen -> Karen
        "kxk" => Tag::new(b"KRN "), // Zayein Karen -> Karen
        "kxl" => Tag::new(b"KUU "), // Nepali Kurux(retired code) -> Kurukh
        "kxu" => Tag::new(b"KUI "), // Kui (India)(retired code)
        "ky" => Tag::new(b"KIR "),  // Kirghiz (Kyrgyz)
        "kyk" => Tag::new(&[0; 4]), // Kamayo != Koryak
        "kyu" => Tag::new(b"KYU "), // Western Kayah
        // "kyu" => Tag::new(b"KRN "), // Western Kayah -> Karen
        "la" => Tag::new(b"LAT "),  // Latin
        "lac" => Tag::new(b"MYN "), // Lacandon -> Mayan
        "lad" => Tag::new(b"JUD "), // Ladino
        "lah" => Tag::new(&[0; 4]), // Lahnda [macrolanguage] != Lahuli
        "lak" => Tag::new(&[0; 4]), // Laka (Nigeria)(retired code) != Lak
        "lam" => Tag::new(&[0; 4]), // Lamba != Lambani
        "laz" => Tag::new(&[0; 4]), // Aribwatsa != Laz
        "lb" => Tag::new(b"LTZ "),  // Luxembourgish
        "lbe" => Tag::new(b"LAK "), // Lak
        "lbj" => Tag::new(b"LDK "), // Ladakhi
        "lbl" => Tag::new(b"BIK "), // Libon Bikol -> Bikol
        "lce" => Tag::new(b"MLY "), // Loncong -> Malay
        "lcf" => Tag::new(b"MLY "), // Lubu -> Malay
        "ldi" => Tag::new(b"KON0"), // Laari -> Kongo
        "ldk" => Tag::new(&[0; 4]), // Leelau != Ladakhi
        // "lef" => 	tag: Tag::new(b"LEF "), // Lelemi
        // "lez" => 	tag: Tag::new(b"LEZ "), // Lezghian -> Lezgi
        "lg" => Tag::new(b"LUG "),  // Ganda
        "li" => Tag::new(b"LIM "),  // Limburgish
        "lif" => Tag::new(b"LMB "), // Limbu
        // "lij" => 	tag: Tag::new(b"LIJ "), // Ligurian
        "lir" => Tag::new(b"CPP "), // Liberian English -> Creoles
        // "lis" => 	tag: Tag::new(b"LIS "), // Lisu
        // "liv" => 	tag: Tag::new(b"LIV "), // Liv
        "liw" => Tag::new(b"MLY "), // Col -> Malay
        "liy" => Tag::new(b"BAD0"), // Banda-Bambari -> Banda
        // "ljp" => 	tag: Tag::new(b"LJP "), // Lampung Api -> Lampung
        "lkb" => Tag::new(b"LUH "), // Kabras -> Luyia
        // "lki" => 	tag: Tag::new(b"LKI "), // Laki
        "lko" => Tag::new(b"LUH "), // Khayo -> Luyia
        "lks" => Tag::new(b"LUH "), // Kisa -> Luyia
        "lld" => Tag::new(b"LAD "), // Ladin
        "lma" => Tag::new(&[0; 4]), // East Limba != Low Mari
        "lmb" => Tag::new(&[0; 4]), // Merei != Limbu
        "lmn" => Tag::new(b"LAM "), // Lambadi -> Lambani
        // "lmo" => 	tag: Tag::new(b"LMO "), // Lombard
        "lmw" => Tag::new(&[0; 4]), // Lake Miwok != Lomwe
        "ln" => Tag::new(b"LIN "),  // Lingala
        "lna" => Tag::new(b"BAD0"), // Langbashe -> Banda
        "lnl" => Tag::new(b"BAD0"), // South Central Banda -> Banda
        "lo" => Tag::new(b"LAO "),  // Lao
        // "lom" => 	tag: Tag::new(b"LOM "), // Loma (Liberia)
        "lou" => Tag::new(b"CPP "), // Louisiana Creole -> Creoles
        // "lpo" => 	tag: Tag::new(b"LPO "), // Lipo
        // "lrc" => 	tag: Tag::new(b"LRC "), // Northern Luri -> Luri
        "lri" => Tag::new(b"LUH "), // Marachi -> Luyia
        "lrm" => Tag::new(b"LUH "), // Marama -> Luyia
        "lrt" => Tag::new(b"CPP "), // Larantuka Malay -> Creoles
        "lsb" => Tag::new(&[0; 4]), // Burundian Sign Language != Lower Sorbian
        "lsm" => Tag::new(b"LUH "), // Saamia -> Luyia
        "lt" => Tag::new(b"LTH "),  // Lithuanian
        "ltg" => Tag::new(b"LVI "), // Latgalian -> Latvian
        "lth" => Tag::new(&[0; 4]), // Thur != Lithuanian
        "lto" => Tag::new(b"LUH "), // Tsotso -> Luyia
        "lts" => Tag::new(b"LUH "), // Tachoni -> Luyia
        "lu" => Tag::new(b"LUB "),  // Luba-Katanga
        // "lua" => 	tag: Tag::new(b"LUA "), // Luba-Lulua
        "luh" => Tag::new(b"ZHS "), // Leizhou Chinese -> Chinese, Simplified
        // "luo" => 	tag: Tag::new(b"LUO "), // Luo (Kenya and Tanzania)
        "lus" => Tag::new(b"MIZ "), // Lushai -> Mizo
        // "lus" => Tag::new(b"QIN "), // Lushai -> Chin
        // "lut" => 	tag: Tag::new(b"LUT "), // Lushootseed
        "luy" => Tag::new(b"LUH "), // Luyia [macrolanguage]
        "luz" => Tag::new(b"LRC "), // Southern Luri -> Luri
        "lv" => Tag::new(b"LVI "),  // Latvian [macrolanguage]
        "lvi" => Tag::new(&[0; 4]), // Lavi != Latvian
        "lvs" => Tag::new(b"LVI "), // Standard Latvian -> Latvian
        "lwg" => Tag::new(b"LUH "), // Wanga -> Luyia
        "lzh" => Tag::new(b"ZHT "), // Literary Chinese -> Chinese, Traditional
        "lzz" => Tag::new(b"LAZ "), // Laz
        // "mad" => 	tag: Tag::new(b"MAD "), // Madurese -> Madura
        // "mag" => 	tag: Tag::new(b"MAG "), // Magahi
        "mai" => Tag::new(b"MTH "), // Maithili
        "maj" => Tag::new(&[0; 4]), // Jalapa De Díaz Mazatec != Majang
        "mak" => Tag::new(b"MKR "), // Makasar
        "mam" => Tag::new(b"MAM "), // Mam
        // "mam" => Tag::new(b"MYN "), // Mam -> Mayan
        "man" => Tag::new(b"MNK "), // Mandingo [macrolanguage] -> Maninka
        "map" => Tag::new(&[0; 4]), // Austronesian  [collection] != Mapudungun
        "maw" => Tag::new(&[0; 4]), // Mampruli != Marwari
        "max" => Tag::new(b"MLY "), // North Moluccan Malay -> Malay
        // "max" => Tag::new(b"CPP "), // North Moluccan Malay -> Creoles
        "mbf" => Tag::new(b"CPP "), // Baba Malay -> Creoles
        "mbn" => Tag::new(&[0; 4]), // Macaguán != Mbundu
        // "mbo" => 	tag: Tag::new(b"MBO "), // Mbo (Cameroon)
        "mch" => Tag::new(&[0; 4]), // Maquiritari != Manchu
        "mcm" => Tag::new(b"CPP "), // Malaccan Creole Portuguese -> Creoles
        "mcr" => Tag::new(&[0; 4]), // Menya != Moose Cree
        "mct" => Tag::new(b"BTI "), // Mengisa -> Beti
        "mde" => Tag::new(&[0; 4]), // Maba (Chad) != Mende
        "mdf" => Tag::new(b"MOK "), // Moksha
        // "mdr" => 	tag: Tag::new(b"MDR "), // Mandar
        "mdy" => Tag::new(b"MLE "), // Male (Ethiopia)
        "men" => Tag::new(b"MDE "), // Mende (Sierra Leone)
        "meo" => Tag::new(b"MLY "), // Kedah Malay -> Malay
        // "mer" => 	tag: Tag::new(b"MER "), // Meru
        // "mev" => 	tag: Tag::new(b"MEV "), // Mano
        "mfa" => Tag::new(b"MFA "), // Pattani Malay
        // "mfa" => Tag::new(b"MLY "), // Pattani Malay -> Malay
        "mfb" => Tag::new(b"MLY "), // Bangka -> Malay
        "mfe" => Tag::new(b"MFE "), // Morisyen
        // "mfe" => Tag::new(b"CPP "), // Morisyen -> Creoles
        "mfp" => Tag::new(b"CPP "), // Makassar Malay -> Creoles
        "mg" => Tag::new(b"MLG "),  // Malagasy [macrolanguage]
        "mga" => Tag::new(b"SGA "), // Middle Irish (900-1200) -> Old Irish
        "mh" => Tag::new(b"MAH "),  // Marshallese
        "mhc" => Tag::new(b"MYN "), // Mocho -> Mayan
        "mhr" => Tag::new(b"LMA "), // Eastern Mari -> Low Mari
        "mhv" => Tag::new(b"ARK "), // Arakanese(retired code) -> Rakhine
        "mi" => Tag::new(b"MRI "),  // Maori
        "min" => Tag::new(b"MIN "), // Minangkabau
        // "min" => Tag::new(b"MLY "), // Minangkabau -> Malay
        "miz" => Tag::new(&[0; 4]), // Coatzospan Mixtec != Mizo
        "mk" => Tag::new(b"MKD "),  // Macedonian
        "mkn" => Tag::new(b"CPP "), // Kupang Malay -> Creoles
        "mkr" => Tag::new(&[0; 4]), // Malas != Makasar
        "mku" => Tag::new(b"MNK "), // Konyanka Maninka -> Maninka
        // "mkw" => 	tag: Tag::new(b"MKW "), // Kituba (Congo)
        "ml" => Tag::new(b"MAL "), // Malayalam -> Malayalam Traditional
        // "ml" => Tag::new(b"MLR "),  // Malayalam -> Malayalam Reformed
        "mle" => Tag::new(&[0; 4]), // Manambu != Male
        "mln" => Tag::new(&[0; 4]), // Malango != Malinke
        "mlq" => Tag::new(b"MLN "), // Western Maninkakan -> Malinke
        // "mlq" => Tag::new(b"MNK "), // Western Maninkakan -> Maninka
        "mlr" => Tag::new(&[0; 4]), // Vame != Malayalam Reformed
        "mmr" => Tag::new(b"HMN "), // Western Xiangxi Miao -> Hmong
        "mn" => Tag::new(b"MNG "),  // Mongolian [macrolanguage]
        "mnc" => Tag::new(b"MCH "), // Manchu
        "mnd" => Tag::new(&[0; 4]), // Mondé != Mandinka
        "mng" => Tag::new(&[0; 4]), // Eastern Mnong != Mongolian
        "mnh" => Tag::new(b"BAD0"), // Mono (Democratic Republic of Congo) -> Banda
        // "mni" => 	tag: Tag::new(b"MNI "), // Manipuri
        "mnk" => Tag::new(b"MND "), // Mandinka
        // "mnk" => Tag::new(b"MNK "), // Mandinka -> Maninka
        "mnp" => Tag::new(b"ZHS "), // Min Bei Chinese -> Chinese, Simplified
        "mns" => Tag::new(b"MAN "), // Mansi
        "mnw" => Tag::new(b"MON "), // Mon
        // "mnw" => Tag::new(b"MONT"), // Mon -> Thailand Mon
        "mnx" => Tag::new(&[0; 4]), // Manikion != Manx
        "mo" => Tag::new(b"MOL "),  // Moldavian(retired code)
        // "mo" => Tag::new(b"ROM "),  // Moldavian(retired code) -> Romanian
        "mod" => Tag::new(b"CPP "), // Mobilian -> Creoles
        // "moh" => 	tag: Tag::new(b"MOH "), // Mohawk
        "mok" => Tag::new(&[0; 4]), // Morori != Moksha
        "mop" => Tag::new(b"MYN "), // Mopán Maya -> Mayan
        "mor" => Tag::new(&[0; 4]), // Moro != Moroccan
        // "mos" => 	tag: Tag::new(b"MOS "), // Mossi
        "mpe" => Tag::new(b"MAJ "), // Majang
        "mqg" => Tag::new(b"MLY "), // Kota Bangun Kutai Malay -> Malay
        "mr" => Tag::new(b"MAR "),  // Marathi
        "mrh" => Tag::new(b"QIN "), // Mara Chin -> Chin
        "mrj" => Tag::new(b"HMA "), // Western Mari -> High Mari
        "ms" => Tag::new(b"MLY "),  // Malay  [macrolanguage]
        "msc" => Tag::new(b"MNK "), // Sankaran Maninka -> Maninka
        "msh" => Tag::new(b"MLG "), // Masikoro Malagasy -> Malagasy
        "msi" => Tag::new(b"MLY "), // Sabah Malay -> Malay
        // "msi" => Tag::new(b"CPP "), // Sabah Malay -> Creoles
        "mt" => Tag::new(b"MTS "),  // Maltese
        "mth" => Tag::new(&[0; 4]), // Munggui != Maithili
        "mtr" => Tag::new(b"MAW "), // Mewari -> Marwari
        "mts" => Tag::new(&[0; 4]), // Yora != Maltese
        "mud" => Tag::new(b"CPP "), // Mednyj Aleut -> Creoles
        "mui" => Tag::new(b"MLY "), // Musi -> Malay
        "mun" => Tag::new(&[0; 4]), // Munda  [collection] != Mundari
        "mup" => Tag::new(b"RAJ "), // Malvi -> Rajasthani
        "muq" => Tag::new(b"HMN "), // Eastern Xiangxi Miao -> Hmong
        // "mus" => 	tag: Tag::new(b"MUS "), // Creek -> Muscogee
        "mvb" => Tag::new(b"ATH "), // Mattole -> Athapaskan
        "mve" => Tag::new(b"MAW "), // Marwari (Pakistan)
        "mvf" => Tag::new(b"MNG "), // Peripheral Mongolian -> Mongolian
        "mwk" => Tag::new(b"MNK "), // Kita Maninkakan -> Maninka
        // "mwl" => 	tag: Tag::new(b"MWL "), // Mirandese
        "mwq" => Tag::new(b"QIN "), // Mün Chin -> Chin
        "mwr" => Tag::new(b"MAW "), // Marwari [macrolanguage]
        "mww" => Tag::new(b"MWW "), // Hmong Daw
        // "mww" => Tag::new(b"HMN "), // Hmong Daw -> Hmong
        "my" => Tag::new(b"BRM "),  // Burmese
        "mym" => Tag::new(b"MEN "), // Me’en
        // "myn" => 	tag: Tag::new(b"MYN "), // Mayan  [collection]
        "myq" => Tag::new(b"MNK "), // Forest Maninka(retired code) -> Maninka
        "myv" => Tag::new(b"ERZ "), // Erzya
        "mzb" => Tag::new(b"BBR "), // Tumzabt -> Berber
        // "mzn" => 	tag: Tag::new(b"MZN "), // Mazanderani
        "mzs" => Tag::new(b"CPP "), // Macanese -> Creoles
        "na" => Tag::new(b"NAU "),  // Nauru -> Nauruan
        "nag" => Tag::new(b"NAG "), // Naga Pidgin -> Naga-Assamese
        // "nag" => Tag::new(b"CPP "), // Naga Pidgin -> Creoles
        // "nah" => 	tag: Tag::new(b"NAH "), // Nahuatl  [collection]
        "nan" => Tag::new(b"ZHS "), // Min Nan Chinese -> Chinese, Simplified
        // "nap" => 	tag: Tag::new(b"NAP "), // Neapolitan
        "nas" => Tag::new(&[0; 4]), // Naasioi != Naskapi
        "naz" => Tag::new(b"NAH "), // Coatepec Nahuatl -> Nahuatl
        "nb" => Tag::new(b"NOR "),  // Norwegian Bokmål -> Norwegian
        "nch" => Tag::new(b"NAH "), // Central Huasteca Nahuatl -> Nahuatl
        "nci" => Tag::new(b"NAH "), // Classical Nahuatl -> Nahuatl
        "ncj" => Tag::new(b"NAH "), // Northern Puebla Nahuatl -> Nahuatl
        "ncl" => Tag::new(b"NAH "), // Michoacán Nahuatl -> Nahuatl
        "ncr" => Tag::new(&[0; 4]), // Ncane != N-Cree
        "ncx" => Tag::new(b"NAH "), // Central Puebla Nahuatl -> Nahuatl
        "nd" => Tag::new(b"NDB "),  // North Ndebele -> Ndebele
        "ndb" => Tag::new(&[0; 4]), // Kenswei Nsei != Ndebele
        // "ndc" => 	tag: Tag::new(b"NDC "), // Ndau
        "ndg" => Tag::new(&[0; 4]), // Ndengereko != Ndonga
        // "nds" => 	tag: Tag::new(b"NDS "), // Low Saxon
        "ne" => Tag::new(b"NEP "),  // Nepali  [macrolanguage]
        "nef" => Tag::new(b"CPP "), // Nefamese -> Creoles
        // "new" => 	tag: Tag::new(b"NEW "), // Newari
        "ng" => Tag::new(b"NDG "), // Ndonga
        // "nga" => 	tag: Tag::new(b"NGA "), // Ngbaka
        "ngl" => Tag::new(b"LMW "), // Lomwe
        "ngm" => Tag::new(b"CPP "), // Ngatik Men's Creole -> Creoles
        "ngo" => Tag::new(b"SXT "), // Ngoni(retired code) -> Sutu
        "ngr" => Tag::new(&[0; 4]), // Engdewu != Nagari
        "ngu" => Tag::new(b"NAH "), // Guerrero Nahuatl -> Nahuatl
        "nhc" => Tag::new(b"NAH "), // Tabasco Nahuatl -> Nahuatl
        "nhd" => Tag::new(b"GUA "), // Chiripá -> Guarani
        "nhe" => Tag::new(b"NAH "), // Eastern Huasteca Nahuatl -> Nahuatl
        "nhg" => Tag::new(b"NAH "), // Tetelcingo Nahuatl -> Nahuatl
        "nhi" => Tag::new(b"NAH "), // Zacatlán-Ahuacatlán-Tepetzintla Nahuatl -> Nahuatl
        "nhk" => Tag::new(b"NAH "), // Isthmus-Cosoleacaque Nahuatl -> Nahuatl
        "nhm" => Tag::new(b"NAH "), // Morelos Nahuatl -> Nahuatl
        "nhn" => Tag::new(b"NAH "), // Central Nahuatl -> Nahuatl
        "nhp" => Tag::new(b"NAH "), // Isthmus-Pajapan Nahuatl -> Nahuatl
        "nhq" => Tag::new(b"NAH "), // Huaxcaleca Nahuatl -> Nahuatl
        "nht" => Tag::new(b"NAH "), // Ometepec Nahuatl -> Nahuatl
        "nhv" => Tag::new(b"NAH "), // Temascaltepec Nahuatl -> Nahuatl
        "nhw" => Tag::new(b"NAH "), // Western Huasteca Nahuatl -> Nahuatl
        "nhx" => Tag::new(b"NAH "), // Isthmus-Mecayapan Nahuatl -> Nahuatl
        "nhy" => Tag::new(b"NAH "), // Northern Oaxaca Nahuatl -> Nahuatl
        "nhz" => Tag::new(b"NAH "), // Santa María La Alta Nahuatl -> Nahuatl
        "niq" => Tag::new(b"KAL "), // Nandi -> Kalenjin
        "nis" => Tag::new(&[0; 4]), // Nimi != Nisi
        // "niu" => 	tag: Tag::new(b"NIU "), // Niuean
        "niv" => Tag::new(b"GIL "), // Gilyak
        "njt" => Tag::new(b"CPP "), // Ndyuka-Trio Pidgin -> Creoles
        "njz" => Tag::new(b"NIS "), // Nyishi -> Nisi
        "nko" => Tag::new(&[0; 4]), // Nkonya != N’Ko
        "nkx" => Tag::new(b"IJO "), // Nkoroo -> Ijo
        "nl" => Tag::new(b"NLD "),  // Dutch
        "nla" => Tag::new(b"BML "), // Ngombale -> Bamileke
        "nle" => Tag::new(b"LUH "), // East Nyala -> Luyia
        "nln" => Tag::new(b"NAH "), // Durango Nahuatl(retired code) -> Nahuatl
        "nlv" => Tag::new(b"NAH "), // Orizaba Nahuatl -> Nahuatl
        "nn" => Tag::new(b"NYN "),  // Norwegian Nynorsk (Nynorsk, Norwegian)
        "nnh" => Tag::new(b"BML "), // Ngiemboon -> Bamileke
        "nnz" => Tag::new(b"BML "), // Nda'nda' -> Bamileke
        "no" => Tag::new(b"NOR "),  // Norwegian [macrolanguage]
        "nod" => Tag::new(b"NTA "), // Northern Thai -> Northern Tai
        // "noe" => 	tag: Tag::new(b"NOE "), // Nimadi
        // "nog" => 	tag: Tag::new(b"NOG "), // Nogai
        // "nop" => 	tag: Tag::new(b"NOP "), // Numanggang
        // "nov" => 	tag: Tag::new(b"NOV "), // Novial
        "npi" => Tag::new(b"NEP "), // Nepali
        "npl" => Tag::new(b"NAH "), // Southeastern Puebla Nahuatl -> Nahuatl
        "nqo" => Tag::new(b"NKO "), // N’Ko
        "nr" => Tag::new(b"NDB "),  // South Ndebele -> Ndebele
        "nsk" => Tag::new(b"NAS "), // Naskapi
        "nsm" => Tag::new(&[0; 4]), // Sumi Naga != Northern Sami
        // "nso" => 	tag: Tag::new(b"NSO "), // Northern Sotho
        "nsu" => Tag::new(b"NAH "), // Sierra Negra Nahuatl -> Nahuatl
        "nto" => Tag::new(&[0; 4]), // Ntomba != Esperanto
        "nue" => Tag::new(b"BAD0"), // Ngundu -> Banda
        // "nuk" => 	tag: Tag::new(b"NUK "), // Nuu-chah-nulth
        "nuu" => Tag::new(b"BAD0"), // Ngbundu -> Banda
        "nuz" => Tag::new(b"NAH "), // Tlamacazapa Nahuatl -> Nahuatl
        "nv" => Tag::new(b"NAV "),  // Navajo
        // "nv" => Tag::new(b"ATH "),  // Navajo -> Athapaskan
        "nwe" => Tag::new(b"BML "), // Ngwe -> Bamileke
        "ny" => Tag::new(b"CHI "),  // Chichewa (Chewa, Nyanja)
        "nyd" => Tag::new(b"LUH "), // Nyore -> Luyia
        // "nym" => 	tag: Tag::new(b"NYM "), // Nyamwezi
        "nyn" => Tag::new(b"NKL "), // Nyankole
        // "nza" => 	tag: Tag::new(b"NZA "), // Tigon Mbembe -> Mbembe Tigon
        "oc" => Tag::new(b"OCI "), // Occitan (post 1500)
        "oj" => Tag::new(b"OJB "), // Ojibwa [macrolanguage] -> Ojibway
        // "ojb" => 	tag: Tag::new(b"OJB "), // Northwestern Ojibwa -> Ojibway
        "ojc" => Tag::new(b"OJB "), // Central Ojibwa -> Ojibway
        "ojg" => Tag::new(b"OJB "), // Eastern Ojibwa -> Ojibway
        "ojs" => Tag::new(b"OCR "), // Severn Ojibwa -> Oji-Cree
        // "ojs" => Tag::new(b"OJB "), // Severn Ojibwa -> Ojibway
        "ojw" => Tag::new(b"OJB "), // Western Ojibwa -> Ojibway
        "okd" => Tag::new(b"IJO "), // Okodia -> Ijo
        "oki" => Tag::new(b"KAL "), // Okiek -> Kalenjin
        "okm" => Tag::new(b"KOH "), // Middle Korean (10th-16th cent.) -> Korean Old Hangul
        "okr" => Tag::new(b"IJO "), // Kirike -> Ijo
        "om" => Tag::new(b"ORO "),  // Oromo [macrolanguage]
        // "one" => 	tag: Tag::new(b"ONE "), // Oneida
        // "ono" => 	tag: Tag::new(b"ONO "), // Onondaga
        "onx" => Tag::new(b"CPP "), // Onin Based Pidgin -> Creoles
        "oor" => Tag::new(b"CPP "), // Oorlams -> Creoles
        "or" => Tag::new(b"ORI "),  // Odia  [macrolanguage]
        "orc" => Tag::new(b"ORO "), // Orma -> Oromo
        "orn" => Tag::new(b"MLY "), // Orang Kanaq -> Malay
        "oro" => Tag::new(&[0; 4]), // Orokolo != Oromo
        "orr" => Tag::new(b"IJO "), // Oruma -> Ijo
        "ors" => Tag::new(b"MLY "), // Orang Seletar -> Malay
        "ory" => Tag::new(b"ORI "), // Odia
        "os" => Tag::new(b"OSS "),  // Ossetian
        "otw" => Tag::new(b"OJB "), // Ottawa -> Ojibway
        "oua" => Tag::new(b"BBR "), // Tagargrent -> Berber
        "pa" => Tag::new(b"PAN "),  // Punjabi
        "paa" => Tag::new(&[0; 4]), // Papuan  [collection] != Palestinian Aramaic
        // "pag" => 	tag: Tag::new(b"PAG "), // Pangasinan
        "pal" => Tag::new(&[0; 4]), // Pahlavi != Pali
        // "pam" => 	tag: Tag::new(b"PAM "), // Pampanga -> Pampangan
        "pap" => Tag::new(b"PAP0"), // Papiamento -> Papiamentu
        // "pap" => Tag::new(b"CPP "), // Papiamento -> Creoles
        "pas" => Tag::new(&[0; 4]), // Papasena != Pashto
        // "pau" => 	tag: Tag::new(b"PAU "), // Palauan
        "pbt" => Tag::new(b"PAS "), // Southern Pashto -> Pashto
        "pbu" => Tag::new(b"PAS "), // Northern Pashto -> Pashto
        // "pcc" => 	tag: Tag::new(b"PCC "), // Bouyei
        // "pcd" => 	tag: Tag::new(b"PCD "), // Picard
        "pce" => Tag::new(b"PLG "), // Ruching Palaung -> Palaung
        "pck" => Tag::new(b"QIN "), // Paite Chin -> Chin
        "pcm" => Tag::new(b"CPP "), // Nigerian Pidgin -> Creoles
        // "pdc" => 	tag: Tag::new(b"PDC "), // Pennsylvania German
        "pdu" => Tag::new(b"KRN "), // Kayan -> Karen
        "pea" => Tag::new(b"CPP "), // Peranakan Indonesian -> Creoles
        "pel" => Tag::new(b"MLY "), // Pekal -> Malay
        "pes" => Tag::new(b"FAR "), // Iranian Persian -> Persian
        "pey" => Tag::new(b"CPP "), // Petjo -> Creoles
        "pga" => Tag::new(b"ARA "), // Sudanese Creole Arabic -> Arabic
        // "pga" => Tag::new(b"CPP "), // Sudanese Creole Arabic -> Creoles
        // "phk" => 	tag: Tag::new(b"PHK "), // Phake
        "pi" => Tag::new(b"PAL "),  // Pali
        "pih" => Tag::new(b"PIH "), // Pitcairn-Norfolk -> Norfolk
        // "pih" => Tag::new(b"CPP "), // Pitcairn-Norfolk -> Creoles
        "pil" => Tag::new(&[0; 4]), // Yom != Filipino
        "pis" => Tag::new(b"CPP "), // Pijin -> Creoles
        "pkh" => Tag::new(b"QIN "), // Pankhu -> Chin
        "pko" => Tag::new(b"KAL "), // Pökoot -> Kalenjin
        "pl" => Tag::new(b"PLK "),  // Polish
        "plg" => Tag::new(b"PLG0"), // Pilagá
        "plk" => Tag::new(&[0; 4]), // Kohistani Shina != Polish
        "pll" => Tag::new(b"PLG "), // Shwe Palaung -> Palaung
        "pln" => Tag::new(b"CPP "), // Palenquero -> Creoles
        "plp" => Tag::new(b"PAP "), // Palpa(retired code)
        "plt" => Tag::new(b"MLG "), // Plateau Malagasy -> Malagasy
        "pml" => Tag::new(b"CPP "), // Lingua Franca -> Creoles
        // "pms" => 	tag: Tag::new(b"PMS "), // Piemontese
        "pmy" => Tag::new(b"CPP "), // Papuan Malay -> Creoles
        // "pnb" => 	tag: Tag::new(b"PNB "), // Western Panjabi
        "poc" => Tag::new(b"MYN "), // Poqomam -> Mayan
        "poh" => Tag::new(b"POH "), // Poqomchi' -> Pocomchi
        // "poh" => Tag::new(b"MYN "), // Poqomchi' -> Mayan
        // "pon" => 	tag: Tag::new(b"PON "), // Pohnpeian
        "pov" => Tag::new(b"CPP "), // Upper Guinea Crioulo -> Creoles
        "ppa" => Tag::new(b"BAG "), // Pao(retired code) -> Baghelkhandi
        "pre" => Tag::new(b"CPP "), // Principense -> Creoles
        // "pro" => 	tag: Tag::new(b"PRO "), // Old Provençal (to 1500) -> Provençal / Old Provençal
        "prp" => Tag::new(b"GUJ "), // Parsi(retired code) -> Gujarati
        "prs" => Tag::new(b"DRI "), // Dari
        // "prs" => Tag::new(b"FAR "), // Dari -> Persian
        "ps" => Tag::new(b"PAS "),  // Pashto [macrolanguage]
        "pse" => Tag::new(b"MLY "), // Central Malay -> Malay
        "pst" => Tag::new(b"PAS "), // Central Pashto -> Pashto
        "pt" => Tag::new(b"PTG "),  // Portuguese
        "pub" => Tag::new(b"QIN "), // Purum -> Chin
        "puz" => Tag::new(b"QIN "), // Purum Naga(retired code) -> Chin
        "pwo" => Tag::new(b"PWO "), // Pwo Western Karen -> Western Pwo Karen
        // "pwo" => Tag::new(b"KRN "), // Pwo Western Karen -> Karen
        "pww" => Tag::new(b"KRN "), // Pwo Northern Karen -> Karen
        "qu" => Tag::new(b"QUZ "),  // Quechua [macrolanguage]
        "qub" => Tag::new(b"QWH "), // Huallaga Huánuco Quechua -> Quechua (Peru)
        // "qub" => Tag::new(b"QUZ "), // Huallaga Huánuco Quechua -> Quechua
        "quc" => Tag::new(b"QUC "), // K’iche’
        // "quc" => Tag::new(b"MYN "), // K'iche' -> Mayan
        "qud" => Tag::new(b"QVI "), // Calderón Highland Quichua -> Quechua (Ecuador)
        // "qud" => Tag::new(b"QUZ "), // Calderón Highland Quichua -> Quechua
        "quf" => Tag::new(b"QUZ "), // Lambayeque Quechua -> Quechua
        "qug" => Tag::new(b"QVI "), // Chimborazo Highland Quichua -> Quechua (Ecuador)
        // "qug" => Tag::new(b"QUZ "), // Chimborazo Highland Quichua -> Quechua
        "quh" => Tag::new(b"QUH "), // South Bolivian Quechua -> Quechua (Bolivia)
        // "quh" => Tag::new(b"QUZ "), // South Bolivian Quechua -> Quechua
        "quk" => Tag::new(b"QUZ "), // Chachapoyas Quechua -> Quechua
        "qul" => Tag::new(b"QUH "), // North Bolivian Quechua -> Quechua (Bolivia)
        // "qul" => Tag::new(b"QUZ "), // North Bolivian Quechua -> Quechua
        "qum" => Tag::new(b"MYN "), // Sipacapense -> Mayan
        "qup" => Tag::new(b"QVI "), // Southern Pastaza Quechua -> Quechua (Ecuador)
        // "qup" => Tag::new(b"QUZ "), // Southern Pastaza Quechua -> Quechua
        "qur" => Tag::new(b"QWH "), // Yanahuanca Pasco Quechua -> Quechua (Peru)
        // "qur" => Tag::new(b"QUZ "), // Yanahuanca Pasco Quechua -> Quechua
        "qus" => Tag::new(b"QUH "), // Santiago del Estero Quichua -> Quechua (Bolivia)
        // "qus" => Tag::new(b"QUZ "), // Santiago del Estero Quichua -> Quechua
        "quv" => Tag::new(b"MYN "), // Sacapulteco -> Mayan
        "quw" => Tag::new(b"QVI "), // Tena Lowland Quichua -> Quechua (Ecuador)
        // "quw" => Tag::new(b"QUZ "), // Tena Lowland Quichua -> Quechua
        "qux" => Tag::new(b"QWH "), // Yauyos Quechua -> Quechua (Peru)
        // "qux" => Tag::new(b"QUZ "), // Yauyos Quechua -> Quechua
        "quy" => Tag::new(b"QUZ "), // Ayacucho Quechua -> Quechua
        // "quz" => 	tag: Tag::new(b"QUZ "), // Cusco Quechua -> Quechua
        "qva" => Tag::new(b"QWH "), // Ambo-Pasco Quechua -> Quechua (Peru)
        // "qva" => Tag::new(b"QUZ "), // Ambo-Pasco Quechua -> Quechua
        "qvc" => Tag::new(b"QUZ "), // Cajamarca Quechua -> Quechua
        "qve" => Tag::new(b"QUZ "), // Eastern Apurímac Quechua -> Quechua
        "qvh" => Tag::new(b"QWH "), // Huamalíes-Dos de Mayo Huánuco Quechua -> Quechua (Peru)
        // "qvh" => Tag::new(b"QUZ "), // Huamalíes-Dos de Mayo Huánuco Quechua -> Quechua
        "qvi" => Tag::new(b"QVI "), // Imbabura Highland Quichua -> Quechua (Ecuador)
        // "qvi" => Tag::new(b"QUZ "), // Imbabura Highland Quichua -> Quechua
        "qvj" => Tag::new(b"QVI "), // Loja Highland Quichua -> Quechua (Ecuador)
        // "qvj" => Tag::new(b"QUZ "), // Loja Highland Quichua -> Quechua
        "qvl" => Tag::new(b"QWH "), // Cajatambo North Lima Quechua -> Quechua (Peru)
        // "qvl" => Tag::new(b"QUZ "), // Cajatambo North Lima Quechua -> Quechua
        "qvm" => Tag::new(b"QWH "), // Margos-Yarowilca-Lauricocha Quechua -> Quechua (Peru)
        // "qvm" => Tag::new(b"QUZ "), // Margos-Yarowilca-Lauricocha Quechua -> Quechua
        "qvn" => Tag::new(b"QWH "), // North Junín Quechua -> Quechua (Peru)
        // "qvn" => Tag::new(b"QUZ "), // North Junín Quechua -> Quechua
        "qvo" => Tag::new(b"QVI "), // Napo Lowland Quechua -> Quechua (Ecuador)
        // "qvo" => Tag::new(b"QUZ "), // Napo Lowland Quechua -> Quechua
        "qvp" => Tag::new(b"QWH "), // Pacaraos Quechua -> Quechua (Peru)
        // "qvp" => Tag::new(b"QUZ "), // Pacaraos Quechua -> Quechua
        "qvs" => Tag::new(b"QUZ "), // San Martín Quechua -> Quechua
        "qvw" => Tag::new(b"QWH "), // Huaylla Wanca Quechua -> Quechua (Peru)
        // "qvw" => Tag::new(b"QUZ "), // Huaylla Wanca Quechua -> Quechua
        "qvz" => Tag::new(b"QVI "), // Northern Pastaza Quichua -> Quechua (Ecuador)
        // "qvz" => Tag::new(b"QUZ "), // Northern Pastaza Quichua -> Quechua
        "qwa" => Tag::new(b"QWH "), // Corongo Ancash Quechua -> Quechua (Peru)
        // "qwa" => Tag::new(b"QUZ "), // Corongo Ancash Quechua -> Quechua
        "qwc" => Tag::new(b"QUZ "), // Classical Quechua -> Quechua
        "qwh" => Tag::new(b"QWH "), // Huaylas Ancash Quechua -> Quechua (Peru)
        // "qwh" => Tag::new(b"QUZ "), // Huaylas Ancash Quechua -> Quechua
        "qws" => Tag::new(b"QWH "), // Sihuas Ancash Quechua -> Quechua (Peru)
        // "qws" => Tag::new(b"QUZ "), // Sihuas Ancash Quechua -> Quechua
        "qwt" => Tag::new(b"ATH "), // Kwalhioqua-Tlatskanai -> Athapaskan
        "qxa" => Tag::new(b"QWH "), // Chiquián Ancash Quechua -> Quechua (Peru)
        // "qxa" => Tag::new(b"QUZ "), // Chiquián Ancash Quechua -> Quechua
        "qxc" => Tag::new(b"QWH "), // Chincha Quechua -> Quechua (Peru)
        // "qxc" => Tag::new(b"QUZ "), // Chincha Quechua -> Quechua
        "qxh" => Tag::new(b"QWH "), // Panao Huánuco Quechua -> Quechua (Peru)
        // "qxh" => Tag::new(b"QUZ "), // Panao Huánuco Quechua -> Quechua
        "qxl" => Tag::new(b"QVI "), // Salasaca Highland Quichua -> Quechua (Ecuador)
        // "qxl" => Tag::new(b"QUZ "), // Salasaca Highland Quichua -> Quechua
        "qxn" => Tag::new(b"QWH "), // Northern Conchucos Ancash Quechua -> Quechua (Peru)
        // "qxn" => Tag::new(b"QUZ "), // Northern Conchucos Ancash Quechua -> Quechua
        "qxo" => Tag::new(b"QWH "), // Southern Conchucos Ancash Quechua -> Quechua (Peru)
        // "qxo" => Tag::new(b"QUZ "), // Southern Conchucos Ancash Quechua -> Quechua
        "qxp" => Tag::new(b"QUZ "), // Puno Quechua -> Quechua
        "qxr" => Tag::new(b"QVI "), // Cañar Highland Quichua -> Quechua (Ecuador)
        // "qxr" => Tag::new(b"QUZ "), // Cañar Highland Quichua -> Quechua
        "qxt" => Tag::new(b"QWH "), // Santa Ana de Tusi Pasco Quechua -> Quechua (Peru)
        // "qxt" => Tag::new(b"QUZ "), // Santa Ana de Tusi Pasco Quechua -> Quechua
        "qxu" => Tag::new(b"QUZ "), // Arequipa-La Unión Quechua -> Quechua
        "qxw" => Tag::new(b"QWH "), // Jauja Wanca Quechua -> Quechua (Peru)
        // "qxw" => Tag::new(b"QUZ "), // Jauja Wanca Quechua -> Quechua
        "rag" => Tag::new(b"LUH "), // Logooli -> Luyia
        // "raj" => 	tag: Tag::new(b"RAJ "), // Rajasthani [macrolanguage]
        "ral" => Tag::new(b"QIN "), // Ralte -> Chin
        // "rar" => 	tag: Tag::new(b"RAR "), // Rarotongan
        "rbb" => Tag::new(b"PLG "), // Rumai Palaung -> Palaung
        "rbl" => Tag::new(b"BIK "), // Miraya Bikol -> Bikol
        "rcf" => Tag::new(b"CPP "), // Réunion Creole French -> Creoles
        // "rej" => 	tag: Tag::new(b"REJ "), // Rejang
        // "rhg" => 	tag: Tag::new(b"RHG "), // Rohingya
        // "ria" => 	tag: Tag::new(b"RIA "), // Riang (India)
        "rif" => Tag::new(b"RIF "), // Tarifit
        // "rif" => Tag::new(b"BBR "), // Tarifit -> Berber
        // "rit" => 	tag: Tag::new(b"RIT "), // Ritharrngu -> Ritarungo
        "rki" => Tag::new(b"ARK "), // Rakhine
        // "rkw" => 	tag: Tag::new(b"RKW "), // Arakwal
        "rm" => Tag::new(b"RMS "),  // Romansh
        "rmc" => Tag::new(b"ROY "), // Carpathian Romani -> Romany
        "rmf" => Tag::new(b"ROY "), // Kalo Finnish Romani -> Romany
        "rml" => Tag::new(b"ROY "), // Baltic Romani -> Romany
        "rmn" => Tag::new(b"ROY "), // Balkan Romani -> Romany
        "rmo" => Tag::new(b"ROY "), // Sinte Romani -> Romany
        "rms" => Tag::new(&[0; 4]), // Romanian Sign Language != Romansh
        "rmw" => Tag::new(b"ROY "), // Welsh Romani -> Romany
        "rmy" => Tag::new(b"RMY "), // Vlax Romani
        // "rmy" => Tag::new(b"ROY "), // Vlax Romani -> Romany
        "rmz" => Tag::new(b"ARK "), // Marma -> Rakhine
        "rn" => Tag::new(b"RUN "),  // Rundi
        "ro" => Tag::new(b"ROM "),  // Romanian
        "rom" => Tag::new(b"ROY "), // Romany [macrolanguage]
        "rop" => Tag::new(b"CPP "), // Kriol -> Creoles
        "rtc" => Tag::new(b"QIN "), // Rungtu Chin -> Chin
        // "rtm" => 	tag: Tag::new(b"RTM "), // Rotuman
        "ru" => Tag::new(b"RUS "),  // Russian
        "rue" => Tag::new(b"RSY "), // Rusyn
        // "rup" => 	tag: Tag::new(b"RUP "), // Aromanian
        "rw" => Tag::new(b"RUA "),  // Kinyarwanda
        "rwr" => Tag::new(b"MAW "), // Marwari (India)
        "sa" => Tag::new(b"SAN "),  // Sanskrit [macrolanguage]
        "sad" => Tag::new(&[0; 4]), // Sandawe != Sadri
        "sah" => Tag::new(b"YAK "), // Yakut -> Sakha
        "sam" => Tag::new(b"PAA "), // Samaritan Aramaic -> Palestinian Aramaic
        // "sas" => 	tag: Tag::new(b"SAS "), // Sasak
        // "sat" => 	tag: Tag::new(b"SAT "), // Santali
        "say" => Tag::new(&[0; 4]), // Saya != Sayisi
        "sc" => Tag::new(b"SRD "),  // Sardinian [macrolanguage]
        "scf" => Tag::new(b"CPP "), // San Miguel Creole French -> Creoles
        "sch" => Tag::new(b"QIN "), // Sakachep -> Chin
        "sci" => Tag::new(b"CPP "), // Sri Lankan Creole Malay -> Creoles
        "sck" => Tag::new(b"SAD "), // Sadri
        // "scn" => 	tag: Tag::new(b"SCN "), // Sicilian
        // "sco" => 	tag: Tag::new(b"SCO "), // Scots
        "scs" => Tag::new(b"SCS "), // North Slavey
        // "scs" => Tag::new(b"SLA "), // North Slavey -> Slavey
        // "scs" => Tag::new(b"ATH "), // North Slavey -> Athapaskan
        "sd" => Tag::new(b"SND "),  // Sindhi
        "sdc" => Tag::new(b"SRD "), // Sassarese Sardinian -> Sardinian
        "sdh" => Tag::new(b"KUR "), // Southern Kurdish -> Kurdish
        "sdn" => Tag::new(b"SRD "), // Gallurese Sardinian -> Sardinian
        "sds" => Tag::new(b"BBR "), // Sened -> Berber
        "se" => Tag::new(b"NSM "),  // Northern Sami
        // "see" => 	tag: Tag::new(b"SEE "), // Seneca
        "seh" => Tag::new(b"SNA "), // Sena
        "sek" => Tag::new(b"ATH "), // Sekani -> Athapaskan
        // "sel" => 	tag: Tag::new(b"SEL "), // Selkup
        "sez" => Tag::new(b"QIN "), // Senthang Chin -> Chin
        "sfm" => Tag::new(b"SFM "), // Small Flowery Miao
        // "sfm" => Tag::new(b"HMN "), // Small Flowery Miao -> Hmong
        "sg" => Tag::new(b"SGO "), // Sango
        // "sga" => 	tag: Tag::new(b"SGA "), // Old Irish (to 900)
        "sgc" => Tag::new(b"KAL "), // Kipsigis -> Kalenjin
        "sgo" => Tag::new(&[0; 4]), // Songa(retired code) != Sango
        // "sgs" => 	tag: Tag::new(b"SGS "), // Samogitian
        "sgw" => Tag::new(b"CHG "), // Sebat Bet Gurage -> Chaha Gurage
        "sh" => Tag::new(b"BOS "),  // Serbo-Croatian [macrolanguage] -> Bosnian
        // "sh" => Tag::new(b"HRV "),  // Serbo-Croatian [macrolanguage] -> Croatian
        // "sh" => Tag::new(b"SRB "),  // Serbo-Croatian [macrolanguage] -> Serbian
        "shi" => Tag::new(b"SHI "), // Tachelhit
        // "shi" => Tag::new(b"BBR "), // Tachelhit -> Berber
        "shl" => Tag::new(b"QIN "), // Shendu -> Chin
        // "shn" => 	tag: Tag::new(b"SHN "), // Shan
        "shu" => Tag::new(b"ARA "), // Chadian Arabic -> Arabic
        "shy" => Tag::new(b"BBR "), // Tachawit -> Berber
        "si" => Tag::new(b"SNH "),  // Sinhala (Sinhalese)
        "sib" => Tag::new(&[0; 4]), // Sebop != Sibe
        // "sid" => 	tag: Tag::new(b"SID "), // Sidamo
        "sig" => Tag::new(&[0; 4]), // Paasaal != Silte Gurage
        "siz" => Tag::new(b"BBR "), // Siwi -> Berber
        // "sja" => 	tag: Tag::new(b"SJA "), // Epena
        "sjc" => Tag::new(b"ZHS "), // Shaojiang Chinese -> Chinese, Simplified
        "sjd" => Tag::new(b"KSM "), // Kildin Sami
        // "sje" => 	tag: Tag::new(b"SJE "), // Pite Sami
        "sjo" => Tag::new(b"SIB "), // Xibe -> Sibe
        "sjs" => Tag::new(b"BBR "), // Senhaja De Srair -> Berber
        // "sju" => 	tag: Tag::new(b"SJU "), // Ume Sami
        "sk" => Tag::new(b"SKY "),  // Slovak
        "skg" => Tag::new(b"MLG "), // Sakalava Malagasy -> Malagasy
        "skr" => Tag::new(b"SRK "), // Saraiki
        "sks" => Tag::new(&[0; 4]), // Maia != Skolt Sami
        "skw" => Tag::new(b"CPP "), // Skepi Creole Dutch -> Creoles
        "sky" => Tag::new(&[0; 4]), // Sikaiana != Slovak
        "sl" => Tag::new(b"SLV "),  // Slovenian
        "sla" => Tag::new(&[0; 4]), // Slavic  [collection] != Slavey
        "sm" => Tag::new(b"SMO "),  // Samoan
        "sma" => Tag::new(b"SSM "), // Southern Sami
        "smd" => Tag::new(b"MBN "), // Sama(retired code) -> Mbundu
        "smj" => Tag::new(b"LSM "), // Lule Sami
        "sml" => Tag::new(&[0; 4]), // Central Sama != Somali
        "smn" => Tag::new(b"ISM "), // Inari Sami
        "sms" => Tag::new(b"SKS "), // Skolt Sami
        "smt" => Tag::new(b"QIN "), // Simte -> Chin
        "sn" => Tag::new(b"SNA0"),  // Shona
        "snb" => Tag::new(b"IBA "), // Sebuyau(retired code) -> Iban
        "snh" => Tag::new(&[0; 4]), // Shinabo(retired code) != Sinhala (Sinhalese)
        // "snk" => 	tag: Tag::new(b"SNK "), // Soninke
        "so" => Tag::new(b"SML "),  // Somali
        "sog" => Tag::new(&[0; 4]), // Sogdian != Sodo Gurage
        // "sop" => 	tag: Tag::new(b"SOP "), // Songe
        "spv" => Tag::new(b"ORI "), // Sambalpuri -> Odia
        "spy" => Tag::new(b"KAL "), // Sabaot -> Kalenjin
        "sq" => Tag::new(b"SQI "),  // Albanian [macrolanguage]
        "sr" => Tag::new(b"SRB "),  // Serbian
        "srb" => Tag::new(&[0; 4]), // Sora != Serbian
        "src" => Tag::new(b"SRD "), // Logudorese Sardinian -> Sardinian
        "srk" => Tag::new(&[0; 4]), // Serudung Murut != Saraiki
        "srm" => Tag::new(b"CPP "), // Saramaccan -> Creoles
        "srn" => Tag::new(b"CPP "), // Sranan Tongo -> Creoles
        "sro" => Tag::new(b"SRD "), // Campidanese Sardinian -> Sardinian
        // "srr" => 	tag: Tag::new(b"SRR "), // Serer
        "srs" => Tag::new(b"ATH "), // Sarsi -> Athapaskan
        "ss" => Tag::new(b"SWZ "),  // Swati
        "ssh" => Tag::new(b"ARA "), // Shihhi Arabic -> Arabic
        "ssl" => Tag::new(&[0; 4]), // Western Sisaala != South Slavey
        "ssm" => Tag::new(&[0; 4]), // Semnam != Southern Sami
        "st" => Tag::new(b"SOT "),  // Southern Sotho
        "sta" => Tag::new(b"CPP "), // Settla -> Creoles
        // "stq" => 	tag: Tag::new(b"STQ "), // Saterfriesisch -> Saterland Frisian
        // "str" => 	tag: Tag::new(b"STR "), // Straits Salish
        "stv" => Tag::new(b"SIG "), // Silt'e -> Silte Gurage
        "su" => Tag::new(b"SUN "),  // Sundanese
        // "suk" => 	tag: Tag::new(b"SUK "), // Sukuma
        "suq" => Tag::new(b"SUR "), // Suri
        "sur" => Tag::new(&[0; 4]), // Mwaghavul != Suri
        "sv" => Tag::new(b"SVE "),  // Swedish
        // "sva" => 	tag: Tag::new(b"SVA "), // Svan
        "svc" => Tag::new(b"CPP "), // Vincentian Creole English -> Creoles
        "sve" => Tag::new(&[0; 4]), // Serili != Swedish
        "sw" => Tag::new(b"SWK "),  // Swahili  [macrolanguage]
        "swb" => Tag::new(b"CMR "), // Maore Comorian -> Comorian
        "swc" => Tag::new(b"SWK "), // Congo Swahili -> Swahili
        "swh" => Tag::new(b"SWK "), // Swahili
        "swk" => Tag::new(&[0; 4]), // Malawi Sena != Swahili
        "swn" => Tag::new(b"BBR "), // Sawknah -> Berber
        "swv" => Tag::new(b"MAW "), // Shekhawati -> Marwari
        // "sxu" => 	tag: Tag::new(b"SXU "), // Upper Saxon
        "syc" => Tag::new(b"SYR "), // Classical Syriac -> Syriac
        // "syl" => 	tag: Tag::new(b"SYL "), // Sylheti
        // "syr" => 	tag: Tag::new(b"SYR "), // Syriac [macrolanguage]
        // "szl" => 	tag: Tag::new(b"SZL "), // Silesian
        "ta" => Tag::new(b"TAM "),  // Tamil
        "taa" => Tag::new(b"ATH "), // Lower Tanana -> Athapaskan
        // "tab" => 	tag: Tag::new(b"TAB "), // Tabassaran -> Tabasaran
        "taj" => Tag::new(&[0; 4]), // Eastern Tamang != Tajiki
        "taq" => Tag::new(b"TAQ "), // Tamasheq
        // "taq" => Tag::new(b"TMH "), // Tamasheq -> Tamashek
        // "taq" => Tag::new(b"BBR "), // Tamasheq -> Berber
        "tas" => Tag::new(b"CPP "), // Tay Boi -> Creoles
        "tau" => Tag::new(b"ATH "), // Upper Tanana -> Athapaskan
        // "tbv" => 	tag: Tag::new(b"TBV "), // Tobo
        "tcb" => Tag::new(b"ATH "), // Tanacross -> Athapaskan
        "tce" => Tag::new(b"ATH "), // Southern Tutchone -> Athapaskan
        "tch" => Tag::new(b"CPP "), // Turks And Caicos Creole English -> Creoles
        "tcp" => Tag::new(b"QIN "), // Tawr Chin -> Chin
        "tcs" => Tag::new(b"CPP "), // Torres Strait Creole -> Creoles
        "tcy" => Tag::new(b"TUL "), // Tulu
        "tcz" => Tag::new(b"QIN "), // Thado Chin -> Chin
        // "tdc" => 	tag: Tag::new(b"TDC "), // Emberá-Tadó
        // "tdd" => 	tag: Tag::new(b"TDD "), // Tai Nüa -> Dehong Dai
        "tdx" => Tag::new(b"MLG "), // Tandroy-Mahafaly Malagasy -> Malagasy
        "te" => Tag::new(b"TEL "),  // Telugu
        "tec" => Tag::new(b"KAL "), // Terik -> Kalenjin
        "tem" => Tag::new(b"TMN "), // Timne -> Temne
        // "tet" => 	tag: Tag::new(b"TET "), // Tetum
        "tez" => Tag::new(b"BBR "), // Tetserret -> Berber
        "tfn" => Tag::new(b"ATH "), // Tanaina -> Athapaskan
        "tg" => Tag::new(b"TAJ "),  // Tajik -> Tajiki
        "tgh" => Tag::new(b"CPP "), // Tobagonian Creole English -> Creoles
        "tgj" => Tag::new(b"NIS "), // Tagin -> Nisi
        "tgn" => Tag::new(&[0; 4]), // Tandaganon != Tongan
        "tgr" => Tag::new(&[0; 4]), // Tareng != Tigre
        "tgx" => Tag::new(b"ATH "), // Tagish -> Athapaskan
        "tgy" => Tag::new(&[0; 4]), // Togoyo != Tigrinya
        "th" => Tag::new(b"THA "),  // Thai
        // "thp" => 	tag: Tag::new(b"THP "), // Thompson
        "tht" => Tag::new(b"ATH "), // Tahltan -> Athapaskan
        "thv" => Tag::new(b"THV "), // Tahaggart Tamahaq
        // "thv" => Tag::new(b"TMH "), // Tahaggart Tamahaq -> Tamashek
        // "thv" => Tag::new(b"BBR "), // Tahaggart Tamahaq -> Berber
        "thz" => Tag::new(b"THZ "), // Tayart Tamajeq
        // "thz" => Tag::new(b"TMH "), // Tayart Tamajeq -> Tamashek
        // "thz" => Tag::new(b"BBR "), // Tayart Tamajeq -> Berber
        "ti" => Tag::new(b"TGY "),  // Tigrinya
        "tia" => Tag::new(b"BBR "), // Tidikelt Tamazight -> Berber
        "tig" => Tag::new(b"TGR "), // Tigre
        // "tiv" => 	tag: Tag::new(b"TIV "), // Tiv
        // "tjl" => 	tag: Tag::new(b"TJL "), // Tai Laing
        "tjo" => Tag::new(b"BBR "), // Temacine Tamazight -> Berber
        "tk" => Tag::new(b"TKM "),  // Turkmen
        "tkg" => Tag::new(b"MLG "), // Tesaka Malagasy -> Malagasy
        "tkm" => Tag::new(&[0; 4]), // Takelma != Turkmen
        "tl" => Tag::new(b"TGL "),  // Tagalog
        // "tli" => 	tag: Tag::new(b"TLI "), // Tlingit
        // "tly" => 	tag: Tag::new(b"TLY "), // Talysh
        "tmg" => Tag::new(b"CPP "), // Ternateño -> Creoles
        "tmh" => Tag::new(b"TMH "), // Tamashek [macrolanguage]
        // "tmh" => Tag::new(b"BBR "), // Tamashek [macrolanguage] -> Berber
        "tmn" => Tag::new(&[0; 4]), // Taman (Indonesia) != Temne
        "tmw" => Tag::new(b"MLY "), // Temuan -> Malay
        "tn" => Tag::new(b"TNA "),  // Tswana
        "tna" => Tag::new(&[0; 4]), // Tacana != Tswana
        "tne" => Tag::new(&[0; 4]), // Tinoc Kallahan(retired code) != Tundra Enets
        "tnf" => Tag::new(b"DRI "), // Tangshewi(retired code) -> Dari
        // "tnf" => Tag::new(b"FAR "), // Tangshewi(retired code) -> Persian
        "tng" => Tag::new(&[0; 4]), // Tobanga != Tonga
        "to" => Tag::new(b"TGN "),  // Tonga (Tonga Islands) -> Tongan
        "tod" => Tag::new(b"TOD0"), // Toma
        "toi" => Tag::new(b"TNG "), // Tonga (Zambia)
        "toj" => Tag::new(b"MYN "), // Tojolabal -> Mayan
        "tol" => Tag::new(b"ATH "), // Tolowa -> Athapaskan
        "tor" => Tag::new(b"BAD0"), // Togbo-Vara Banda -> Banda
        "tpi" => Tag::new(b"TPI "), // Tok Pisin
        // "tpi" => Tag::new(b"CPP "), // Tok Pisin -> Creoles
        "tr" => Tag::new(b"TRK "),  // Turkish
        "trf" => Tag::new(b"CPP "), // Trinidadian Creole English -> Creoles
        "trk" => Tag::new(&[0; 4]), // Turkic  [collection] != Turkish
        "tru" => Tag::new(b"TUA "), // Turoyo -> Turoyo Aramaic
        // "tru" => Tag::new(b"SYR "), // Turoyo -> Syriac
        "ts" => Tag::new(b"TSG "),  // Tsonga
        "tsg" => Tag::new(&[0; 4]), // Tausug != Tsonga
        // "tsj" => 	tag: Tag::new(b"TSJ "), // Tshangla
        "tt" => Tag::new(b"TAT "),  // Tatar
        "ttc" => Tag::new(b"MYN "), // Tektiteko -> Mayan
        "ttm" => Tag::new(b"ATH "), // Northern Tutchone -> Athapaskan
        "ttq" => Tag::new(b"TTQ "), // Tawallammat Tamajaq
        // "ttq" => Tag::new(b"TMH "), // Tawallammat Tamajaq -> Tamashek
        // "ttq" => Tag::new(b"BBR "), // Tawallammat Tamajaq -> Berber
        "tua" => Tag::new(&[0; 4]), // Wiarumus != Turoyo Aramaic
        "tul" => Tag::new(&[0; 4]), // Tula != Tulu
        // "tum" => 	tag: Tag::new(b"TUM "), // Tumbuka
        // "tus" => 	tag: Tag::new(b"TUS "), // Tuscarora
        "tuu" => Tag::new(b"ATH "), // Tututni -> Athapaskan
        "tuv" => Tag::new(&[0; 4]), // Turkana != Tuvin
        "tuy" => Tag::new(b"KAL "), // Tugen -> Kalenjin
        // "tvl" => 	tag: Tag::new(b"TVL "), // Tuvalu
        "tvy" => Tag::new(b"CPP "), // Timor Pidgin -> Creoles
        "tw" => Tag::new(b"TWI "),  // Twi
        // "tw" => Tag::new(b"AKA "),  // Twi -> Akan
        "txc" => Tag::new(b"ATH "), // Tsetsaut -> Athapaskan
        "txy" => Tag::new(b"MLG "), // Tanosy Malagasy -> Malagasy
        "ty" => Tag::new(b"THT "),  // Tahitian
        "tyv" => Tag::new(b"TUV "), // Tuvinian -> Tuvin
        // "tyz" => 	tag: Tag::new(b"TYZ "), // Tày
        "tzh" => Tag::new(b"MYN "), // Tzeltal -> Mayan
        "tzj" => Tag::new(b"MYN "), // Tz'utujil -> Mayan
        "tzm" => Tag::new(b"TZM "), // Central Atlas Tamazight -> Tamazight
        // "tzm" => Tag::new(b"BBR "), // Central Atlas Tamazight -> Berber
        "tzo" => Tag::new(b"TZO "), // Tzotzil
        // "tzo" => Tag::new(b"MYN "), // Tzotzil -> Mayan
        "ubl" => Tag::new(b"BIK "), // Buhi'non Bikol -> Bikol
        // "udi" => 	tag: Tag::new(b"UDI "), // Udi
        // "udm" => 	tag: Tag::new(b"UDM "), // Udmurt
        "ug" => Tag::new(b"UYG "),  // Uyghur
        "uk" => Tag::new(b"UKR "),  // Ukrainian
        "uki" => Tag::new(b"KUI "), // Kui (India)
        "uln" => Tag::new(b"CPP "), // Unserdeutsch -> Creoles
        // "umb" => 	tag: Tag::new(b"UMB "), // Umbundu
        "unr" => Tag::new(b"MUN "), // Mundari
        "ur" => Tag::new(b"URD "),  // Urdu
        "urk" => Tag::new(b"MLY "), // Urak Lawoi' -> Malay
        "usp" => Tag::new(b"MYN "), // Uspanteco -> Mayan
        "uz" => Tag::new(b"UZB "),  // Uzbek [macrolanguage]
        "uzn" => Tag::new(b"UZB "), // Northern Uzbek -> Uzbek
        "uzs" => Tag::new(b"UZB "), // Southern Uzbek -> Uzbek
        "vap" => Tag::new(b"QIN "), // Vaiphei -> Chin
        "ve" => Tag::new(b"VEN "),  // Venda
        // "vec" => 	tag: Tag::new(b"VEC "), // Venetian
        "vi" => Tag::new(b"VIT "),  // Vietnamese
        "vic" => Tag::new(b"CPP "), // Virgin Islands Creole English -> Creoles
        "vit" => Tag::new(&[0; 4]), // Viti != Vietnamese
        "vkk" => Tag::new(b"MLY "), // Kaur -> Malay
        "vkp" => Tag::new(b"CPP "), // Korlai Creole Portuguese -> Creoles
        "vkt" => Tag::new(b"MLY "), // Tenggarong Kutai Malay -> Malay
        "vls" => Tag::new(b"FLE "), // Vlaams -> Dutch (Flemish)
        "vmw" => Tag::new(b"MAK "), // Makhuwa
        "vo" => Tag::new(b"VOL "),  // Volapük
        "vro" => Tag::new(b"VRO "), // Võro
        // "vro" => Tag::new(b"ETI "), // Võro -> Estonian
        "vsn" => Tag::new(b"SAN "), // Vedic Sanskrit -> Sanskrit
        "wa" => Tag::new(b"WLN "),  // Walloon
        "wag" => Tag::new(&[0; 4]), // Wa'ema != Wagdi
        // "war" => 	tag: Tag::new(b"WAR "), // Waray (Philippines) -> Waray-Waray
        // "wbl" => 	tag: Tag::new(b"WBL "), // Wakhi
        "wbm" => Tag::new(b"WA  "), // Wa
        "wbr" => Tag::new(b"WAG "), // Wagdi
        // "wbr" => Tag::new(b"RAJ "), // Wagdi -> Rajasthani
        // "wci" => 	tag: Tag::new(b"WCI "), // Waci Gbe
        // "wdt" => 	tag: Tag::new(b"WDT "), // Wendat
        "wea" => Tag::new(b"KRN "), // Wewaw -> Karen
        "wes" => Tag::new(b"CPP "), // Cameroon Pidgin -> Creoles
        "weu" => Tag::new(b"QIN "), // Rawngtu Chin -> Chin
        "wlc" => Tag::new(b"CMR "), // Mwali Comorian -> Comorian
        "wle" => Tag::new(b"SIG "), // Wolane -> Silte Gurage
        "wlk" => Tag::new(b"ATH "), // Wailaki -> Athapaskan
        "wni" => Tag::new(b"CMR "), // Ndzwani Comorian -> Comorian
        "wo" => Tag::new(b"WLF "),  // Wolof
        "wry" => Tag::new(b"MAW "), // Merwari -> Marwari
        "wsg" => Tag::new(b"GON "), // Adilabad Gondi -> Gondi
        // "wtm" => 	tag: Tag::new(b"WTM "), // Mewati
        "wuu" => Tag::new(b"ZHS "), // Wu Chinese -> Chinese, Simplified
        "wya" => Tag::new(b"WDT "), // Wyandot(retired code) -> Wendat
        // "wya" => Tag::new(b"WYN "), // Wyandot(retired code)
        // "wyn" => 	tag: Tag::new(b"WYN "), // Wyandot
        "xal" => Tag::new(b"KLM "), // Kalmyk
        // "xal" => Tag::new(b"TOD "), // Kalmyk -> Todo
        "xan" => Tag::new(b"SEK "), // Xamtanga -> Sekota
        "xbd" => Tag::new(&[0; 4]), // Bindal != Lü
        "xh" => Tag::new(b"XHS "),  // Xhosa
        // "xjb" => 	tag: Tag::new(b"XJB "), // Minjungbal -> Minjangbal
        // "xkf" => 	tag: Tag::new(b"XKF "), // Khengkha
        "xmg" => Tag::new(b"BML "), // Mengaka -> Bamileke
        "xmm" => Tag::new(b"MLY "), // Manado Malay -> Malay
        // "xmm" => Tag::new(b"CPP "), // Manado Malay -> Creoles
        "xmv" => Tag::new(b"MLG "), // Antankarana Malagasy -> Malagasy
        "xmw" => Tag::new(b"MLG "), // Tsimihety Malagasy -> Malagasy
        "xnj" => Tag::new(b"SXT "), // Ngoni (Tanzania) -> Sutu
        "xnq" => Tag::new(b"SXT "), // Ngoni (Mozambique) -> Sutu
        "xnr" => Tag::new(b"DGR "), // Kangri -> Dogri (macrolanguage)
        // "xog" => 	tag: Tag::new(b"XOG "), // Soga
        "xpe" => Tag::new(b"XPE "), // Liberia Kpelle -> Kpelle (Liberia)
        // "xpe" => Tag::new(b"KPL "), // Liberia Kpelle -> Kpelle
        "xsl" => Tag::new(b"SSL "), // South Slavey
        // "xsl" => Tag::new(b"SLA "), // South Slavey -> Slavey
        // "xsl" => Tag::new(b"ATH "), // South Slavey -> Athapaskan
        "xst" => Tag::new(b"SIG "), // Silt'e(retired code) -> Silte Gurage
        // "xub" => 	tag: Tag::new(b"XUB "), // Betta Kurumba -> Bette Kuruma
        // "xuj" => 	tag: Tag::new(b"XUJ "), // Jennu Kurumba -> Jennu Kuruma
        "xup" => Tag::new(b"ATH "), // Upper Umpqua -> Athapaskan
        "xwo" => Tag::new(b"TOD "), // Written Oirat -> Todo
        "yaj" => Tag::new(b"BAD0"), // Banda-Yangere -> Banda
        "yak" => Tag::new(&[0; 4]), // Yakama != Sakha
        // "yao" => 	tag: Tag::new(b"YAO "), // Yao
        // "yap" => 	tag: Tag::new(b"YAP "), // Yapese
        "yba" => Tag::new(&[0; 4]), // Yala != Yoruba
        "ybb" => Tag::new(b"BML "), // Yemba -> Bamileke
        "ybd" => Tag::new(b"ARK "), // Yangbye(retired code) -> Rakhine
        "ycr" => Tag::new(b"CPP "), // Yilan Creole -> Creoles
        "ydd" => Tag::new(b"JII "), // Eastern Yiddish -> Yiddish
        // "ygp" => 	tag: Tag::new(b"YGP "), // Gepo
        "yi" => Tag::new(b"JII "),  // Yiddish [macrolanguage]
        "yih" => Tag::new(b"JII "), // Western Yiddish -> Yiddish
        "yim" => Tag::new(&[0; 4]), // Yimchungru Naga != Yi Modern
        // "yna" => 	tag: Tag::new(b"YNA "), // Aluo
        "yo" => Tag::new(b"YBA "),  // Yoruba
        "yos" => Tag::new(b"QIN "), // Yos(retired code) -> Chin
        "yua" => Tag::new(b"MYN "), // Yucateco -> Mayan
        "yue" => Tag::new(b"ZHH "), // Yue Chinese -> Chinese, Traditional, Hong Kong SAR
        // "yuf" => 	tag: Tag::new(b"YUF "), // Havasupai-Walapai-Yavapai
        // "ywq" => 	tag: Tag::new(b"YWQ "), // Wuding-Luquan Yi
        "za" => Tag::new(b"ZHA "),  // Zhuang [macrolanguage]
        "zch" => Tag::new(b"ZHA "), // Central Hongshuihe Zhuang -> Zhuang
        "zdj" => Tag::new(b"CMR "), // Ngazidja Comorian -> Comorian
        // "zea" => 	tag: Tag::new(b"ZEA "), // Zeeuws -> Zealandic
        "zeh" => Tag::new(b"ZHA "), // Eastern Hongshuihe Zhuang -> Zhuang
        "zen" => Tag::new(b"BBR "), // Zenaga -> Berber
        "zgb" => Tag::new(b"ZHA "), // Guibei Zhuang -> Zhuang
        "zgh" => Tag::new(b"ZGH "), // Standard Moroccan Tamazight
        // "zgh" => Tag::new(b"BBR "), // Standard Moroccan Tamazight -> Berber
        "zgm" => Tag::new(b"ZHA "), // Minz Zhuang -> Zhuang
        "zgn" => Tag::new(b"ZHA "), // Guibian Zhuang -> Zhuang
        "zh" => Tag::new(b"ZHS "),  // Chinese, Simplified [macrolanguage]
        "zhd" => Tag::new(b"ZHA "), // Dai Zhuang -> Zhuang
        "zhn" => Tag::new(b"ZHA "), // Nong Zhuang -> Zhuang
        "zkb" => Tag::new(b"KHA "), // Koibal(retired code) -> Khakass
        "zlj" => Tag::new(b"ZHA "), // Liujiang Zhuang -> Zhuang
        "zlm" => Tag::new(b"MLY "), // Malay
        "zln" => Tag::new(b"ZHA "), // Lianshan Zhuang -> Zhuang
        "zlq" => Tag::new(b"ZHA "), // Liuqian Zhuang -> Zhuang
        "zmi" => Tag::new(b"MLY "), // Negeri Sembilan Malay -> Malay
        "zmz" => Tag::new(b"BAD0"), // Mbandja -> Banda
        "znd" => Tag::new(&[0; 4]), // Zande  [collection] != Zande
        "zne" => Tag::new(b"ZND "), // Zande
        "zom" => Tag::new(b"QIN "), // Zou -> Chin
        "zqe" => Tag::new(b"ZHA "), // Qiubei Zhuang -> Zhuang
        "zsm" => Tag::new(b"MLY "), // Standard Malay -> Malay
        "zu" => Tag::new(b"ZUL "),  // Zulu
        "zum" => Tag::new(b"LRC "), // Kumzari -> Luri
        "zyb" => Tag::new(b"ZHA "), // Yongbei Zhuang -> Zhuang
        "zyg" => Tag::new(b"ZHA "), // Yang Zhuang -> Zhuang
        "zyj" => Tag::new(b"ZHA "), // Youjiang Zhuang -> Zhuang
        "zyn" => Tag::new(b"ZHA "), // Yongnan Zhuang -> Zhuang
        "zyp" => Tag::new(b"QIN "), // Zyphe Chin -> Chin
        // "zza" => 	tag: Tag::new(b"ZZA "), // Zazaki [macrolanguage]
        "zzj" => Tag::new(b"ZHA "), // Zuojiang Zhuang -> Zhuang
        _ => Tag::new(&[0; 4]),     // Unknown
    }
}
