use std::collections::HashMap;
use once_cell::sync::Lazy;

#[derive(Debug, PartialEq)]
pub enum Convert {
    Aesthetic,
    Super,
    Flip,
    Italic,
}

static ALPHABET: &str = " ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789()";

static AESTHETIC: Lazy<HashMap<char, char>> = Lazy::new(|| {
    let mut m = HashMap::new();
    static CHARSET: &str = " ＡＢＣＤＥＦＧＨＩＪＫＬＭＮＯＰＱＲＳＴＵＶＷＸＹＺａｂｃｄｅｆｇｈｉｊｋｌｍｎｏｐｑｒｓｔｕｖｗｘｙｚ０１２３４５６７８９";
    for (key, value) in ALPHABET.chars().zip(CHARSET.chars()) {
        m.insert(key, value);
    }
    m
});

static SUPER: Lazy<HashMap<char, char>> = Lazy::new(|| {
    let mut m = HashMap::new();
    static CHARSET: &str = " ᴬᴮᶜᴰᴱᶠᴳᴴᴵᴶᴷᴸᴹᴺᴼᴾqᴿˢᵀᵁⱽᵂˣʸᶻᵃᵇᶜᵈᵉᶠᵍʰⁱʲᵏˡᵐⁿᵒᵖqʳˢᵗᵘᵛʷˣʸᶻ⁰¹²³⁴⁵⁶⁷⁸⁹⁽⁾";
    for (key, value) in ALPHABET.chars().zip(CHARSET.chars()) {
        m.insert(key, value);
    }
    m
});

static ITALIC: Lazy<HashMap<char, char>> = Lazy::new(|| {
    let mut m = HashMap::new();
    static CHARSET: &str = " 𝓐𝓑𝓒𝓓𝓔𝓕𝓖𝓗𝓘𝓙𝓚𝓛𝓜𝓝𝓞𝓟𝓠𝓡𝓢𝓣𝓤𝓥𝓦𝓧𝓨𝓩𝓪𝓫𝓬𝓭𝓮𝓯𝓰𝓱𝓲𝓳𝓴𝓵𝓶𝓷𝓸𝓹𝓺𝓻𝓼𝓽𝓾𝓿𝔀𝔁𝔂𝔃";
    for (key, value) in ALPHABET.chars().zip(CHARSET.chars()) {
        m.insert(key, value);
    }
    m
});


static FLIP: Lazy<HashMap<char, char>> = Lazy::new(|| {
    let mut m = HashMap::new();
    let charset = [('0','0'),('6','9'),('8','8'),('9','6'),('^','ᵥ'),('∀','A'),('B','ꓭ'),('Ɔ','C'),('D','ꓷ'),('Ǝ','E'),('Ⅎ','F'),('פ','G'),('H','H'),('I','I'),('ſ','J'),('ʞ','K'),('˥','L'),('W','M'),('N','N'),('O','O'),('Ԁ','P'),('Q','Ὸ'),('R','ꓤ'),('S','S'),('┴','T'),('∩','U'),('Λ','V'),('M','W'),('X','X'),('⅄','Y'),('Z','Z'),('a','ɐ'),('b','q'),('c','ɔ'),('d','p'),('e','ǝ'),('f','ɟ'),('g','ƃ'),('h','ɥ'),('i','ı'),('j','ɾ'),('k','ʞ'),('l','ן'),('m','ɯ'),('n','u'),('o','o'),('p','d'),('q','b'),('r','ɹ'),('s','s'),('t','ʇ'),('u','n'),('v','ʌ'),('w','ʍ'),('x','x'),('y','ʎ'),('z','z'),('Ɩ','1'),('ᄅ','2'),('Ɛ','3'),('ㄣ','4'),('ϛ','5'),('ㄥ','7'),('¿','?'),('¡','!'),('[',']'),('(',')'),('{','}'),('\'',','),('.','˙'),('<','>'),('♥','♠'),('#','♯')];
    for (key, value) in charset.iter() {
        m.insert(*key, *value);
        m.insert(*value, *key);
    }
    m
});


impl Convert {
    pub fn convert(&self, text: &str) -> String {
        match self {
            Convert::Aesthetic => {
                text.chars()
                    .map(|ch| AESTHETIC.get(&ch).unwrap_or(&'?'))
                    .collect::<String>()
            },
            Convert::Flip => {
                text.chars()
                    .rev()
                    .map(|ch| FLIP.get(&ch).unwrap_or(&' '))
                    .collect::<String>()
            },
            Convert::Super => {
                text.chars()
                    .map(|ch| SUPER.get(&ch).unwrap_or(&'?'))
                    .collect::<String>()
            },
            Convert::Italic => {
                text.chars()
                    .map(|ch| ITALIC.get(&ch).unwrap_or(&'?'))
                    .collect::<String>()
            },
        }
    }
}
