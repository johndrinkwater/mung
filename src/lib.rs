//! mung is a set of functions to demung or decode most common escapements used in data today
//!
//!

#[macro_use]
extern crate lazy_static;

extern crate encoding;
extern crate regex;
extern crate rustc_serialize;

use encoding::DecoderTrap;
use encoding::label::encoding_from_whatwg_label;

use rustc_serialize::base64::FromBase64;
use std::borrow::Cow;
use std::collections::HashMap;
use regex::{Regex, Captures};

lazy_static! {
	// TODO import/injest from https://html.spec.whatwg.org/entities.json at compile time
	//
	static ref ENTITIES: HashMap<&'static str, &'static str> = {
		let mut e = HashMap::new( );
		/* HTML 4 https://www.w3.org/TR/html4/sgml/entities.html */
		/* ISO 8859-1 */
		e.insert( "nbsp", " " );
		e.insert( "iexcl", "¡" );
		e.insert( "cent", "¢" );
		e.insert( "pound", "£" );
		e.insert( "curren", "¤" );
		e.insert( "yen", "¥" );
		e.insert( "brvbar", "¦" );
		e.insert( "sect", "§" );
		e.insert( "uml", "¨" );
		e.insert( "copy", "©" );
		e.insert( "ordf", "ª" );
		e.insert( "laquo", "«" );
		e.insert( "not", "¬" );
		e.insert( "shy", "­" );
		e.insert( "reg", "®" );
		e.insert( "macr", "¯" );
		e.insert( "deg", "°" );
		e.insert( "plusmn", "±" );
		e.insert( "sup2", "²" );
		e.insert( "sup3", "³" );
		e.insert( "acute", "´" );
		e.insert( "micro", "µ" );
		e.insert( "para", "¶" );
		e.insert( "middot", "·" );
		e.insert( "cedil", "¸" );
		e.insert( "sup1", "¹" );
		e.insert( "ordm", "º" );
		e.insert( "raquo", "»" );
		e.insert( "frac14", "¼" );
		e.insert( "frac12", "½" );
		e.insert( "frac34", "¾" );
		e.insert( "iquest", "¿" );
		e.insert( "Agrave", "À" );
		e.insert( "Aacute", "Á" );
		e.insert( "Acirc", "Â" );
		e.insert( "Atilde", "Ã" );
		e.insert( "Auml", "Ä" );
		e.insert( "Aring", "Å" );
		e.insert( "AElig", "Æ" );
		e.insert( "Ccedil", "Ç" );
		e.insert( "Egrave", "È" );
		e.insert( "Eacute", "É" );
		e.insert( "Ecirc", "Ê" );
		e.insert( "Euml", "Ë" );
		e.insert( "Igrave", "Ì" );
		e.insert( "Iacute", "Í" );
		e.insert( "Icirc", "Î" );
		e.insert( "Iuml", "Ï" );
		e.insert( "ETH", "Ð" );
		e.insert( "Ntilde", "Ñ" );
		e.insert( "Ograve", "Ò" );
		e.insert( "Oacute", "Ó" );
		e.insert( "Ocirc", "Ô" );
		e.insert( "Otilde", "Õ" );
		e.insert( "Ouml", "Ö" );
		e.insert( "times", "×" );
		e.insert( "Oslash", "Ø" );
		e.insert( "Ugrave", "Ù" );
		e.insert( "Uacute", "Ú" );
		e.insert( "Ucirc", "Û" );
		e.insert( "Uuml", "Ü" );
		e.insert( "Yacute", "Ý" );
		e.insert( "THORN", "Þ" );
		e.insert( "szlig", "ß" );
		e.insert( "agrave", "à" );
		e.insert( "aacute", "á" );
		e.insert( "acirc", "â" );
		e.insert( "atilde", "ã" );
		e.insert( "auml", "ä" );
		e.insert( "aring", "å" );
		e.insert( "aelig", "æ" );
		e.insert( "ccedil", "ç" );
		e.insert( "egrave", "è" );
		e.insert( "eacute", "é" );
		e.insert( "ecirc", "ê" );
		e.insert( "euml", "ë" );
		e.insert( "igrave", "ì" );
		e.insert( "iacute", "í" );
		e.insert( "icirc", "î" );
		e.insert( "iuml", "ï" );
		e.insert( "eth", "ð" );
		e.insert( "ntilde", "ñ" );
		e.insert( "ograve", "ò" );
		e.insert( "oacute", "ó" );
		e.insert( "ocirc", "ô" );
		e.insert( "otilde", "õ" );
		e.insert( "ouml", "ö" );
		e.insert( "divide", "÷" );
		e.insert( "oslash", "ø" );
		e.insert( "ugrave", "ù" );
		e.insert( "uacute", "ú" );
		e.insert( "ucirc", "û" );
		e.insert( "uuml", "ü" );
		e.insert( "yacute", "ý" );
		e.insert( "thorn", "þ" );
		e.insert( "yuml", "ÿ" );

		/* Symbols, mathematical symbols, Greek letters */
		e.insert( "fnof", "ƒ" );
		// Greek
		e.insert( "Alpha", "Α" );
		e.insert( "Beta", "Β" );
		e.insert( "Gamma", "Γ" );
		e.insert( "Delta", "Δ" );
		e.insert( "Epsilon", "Ε" );
		e.insert( "Zeta", "Ζ" );
		e.insert( "Eta", "Η" );
		e.insert( "Theta", "Θ" );
		e.insert( "Iota", "Ι" );
		e.insert( "Kappa", "Κ" );
		e.insert( "Lambda", "Λ" );
		e.insert( "Mu", "Μ" );
		e.insert( "Nu", "Ν" );
		e.insert( "Xi", "Ξ" );
		e.insert( "Omicron", "Ο" );
		e.insert( "Pi", "Π" );
		e.insert( "Rho", "Ρ" );
		e.insert( "Sigma", "Σ" );
		e.insert( "Tau", "Τ" );
		e.insert( "Upsilon", "Υ" );
		e.insert( "Phi", "Φ" );
		e.insert( "Chi", "Χ" );
		e.insert( "Psi", "Ψ" );
		e.insert( "Omega", "Ω" );
		e.insert( "alpha", "α" );
		e.insert( "beta", "β" );
		e.insert( "gamma", "γ" );
		e.insert( "delta", "δ" );
		e.insert( "epsilon", "ε" );
		e.insert( "zeta", "ζ" );
		e.insert( "eta", "η" );
		e.insert( "theta", "θ" );
		e.insert( "iota", "ι" );
		e.insert( "kappa", "κ" );
		e.insert( "lambda", "λ" );
		e.insert( "mu", "μ" );
		e.insert( "nu", "ν" );
		e.insert( "xi", "ξ" );
		e.insert( "omicron", "ο" );
		e.insert( "pi", "π" );
		e.insert( "rho", "ρ" );
		e.insert( "sigmaf", "ς" );
		e.insert( "sigma", "σ" );
		e.insert( "tau", "τ" );
		e.insert( "upsilon", "υ" );
		e.insert( "phi", "φ" );
		e.insert( "chi", "χ" );
		e.insert( "psi", "ψ" );
		e.insert( "omega", "ω" );
		e.insert( "thetasym", "ϑ" );
		e.insert( "upsih", "ϒ" );
		e.insert( "piv", "ϖ" );
		// General Punctuation
		e.insert( "bull", "•" );
		e.insert( "hellip", "…" );
		e.insert( "prime", "′" );
		e.insert( "Prime", "″" );
		e.insert( "oline", "‾" );
		e.insert( "frasl", "⁄" );
		// Letter like Symbols
		e.insert( "weierp", "℘" );
		e.insert( "image", "ℑ" );
		e.insert( "real", "ℜ" );
		e.insert( "trade", "™" );
		e.insert( "alefsym", "ℵ" );
		// Arrows
		e.insert( "larr", "←" );
		e.insert( "uarr", "↑" );
		e.insert( "rarr", "→" );
		e.insert( "darr", "↓" );
		e.insert( "harr", "↔" );
		e.insert( "crarr", "↵" );
		e.insert( "lArr", "⇐" );
		e.insert( "uArr", "⇑" );
		e.insert( "rArr", "⇒" );
		e.insert( "dArr", "⇓" );
		e.insert( "hArr", "⇔" );
		// Mathematical Operators
		e.insert( "forall", "∀" );
		e.insert( "part", "∂" );
		e.insert( "exist", "∃" );
		e.insert( "empty", "∅" );
		e.insert( "nabla", "∇" );
		e.insert( "isin", "∈" );
		e.insert( "notin", "∉" );
		e.insert( "ni", "∋" );
		e.insert( "prod", "∏" );
		e.insert( "sum", "∑" );
		e.insert( "minus", "−" );
		e.insert( "lowast", "∗" );
		e.insert( "radic", "√" );
		e.insert( "prop", "∝" );
		e.insert( "infin", "∞" );
		e.insert( "ang", "∠" );
		e.insert( "and", "∧" );
		e.insert( "or", "∨" );
		e.insert( "cap", "∩" );
		e.insert( "cup", "∪" );
		e.insert( "int", "∫" );
		e.insert( "there4", "∴" );
		e.insert( "sim", "∼" );
		e.insert( "cong", "≅" );
		e.insert( "asymp", "≈" );
		e.insert( "ne", "≠" );
		e.insert( "equiv", "≡" );
		e.insert( "le", "≤" );
		e.insert( "ge", "≥" );
		e.insert( "sub", "⊂" );
		e.insert( "sup", "⊃" );
		e.insert( "nsub", "⊄" );
		e.insert( "sube", "⊆" );
		e.insert( "supe", "⊇" );
		e.insert( "oplus", "⊕" );
		e.insert( "otimes", "⊗" );
		e.insert( "perp", "⊥" );
		e.insert( "sdot", "⋅" );
		// Miscellaneous Technical
		e.insert( "lceil", "⌈" );
		e.insert( "rceil", "⌉" );
		e.insert( "lfloor", "⌊" );
		e.insert( "rfloor", "⌋" );
		e.insert( "lang", "〈" );
		e.insert( "rang", "〉" );
		// Geometric Shapes
		e.insert( "loz", "◊" );
		// Miscellaneous Symbols
		e.insert( "spades", "♠" );
		e.insert( "clubs", "♣" );
		e.insert( "hearts", "♥" );
		e.insert( "diams", "♦" );

		/* markup-significant and internationalisation characters */
		e.insert( "quot", "\"" );
		e.insert( "amp", "&" );
		e.insert( "lt", "<" );
		e.insert( "gt", ">" );
		// Latin Extended-A
		e.insert( "OElig", "Œ" );
		e.insert( "oelig", "œ" );
		e.insert( "Scaron", "Š" );
		e.insert( "scaron", "š" );
		e.insert( "Yuml", "Ÿ" );
		// Spacing Modifier Letters
		e.insert( "circ", "ˆ" );
		e.insert( "tilde", "˜" );
		// General Punctuation
		e.insert( "ensp", " " );
		e.insert( "emsp", " " );
		e.insert( "thinsp", " " );
		e.insert( "zwnj", "‌" );
		e.insert( "zwj", "‍" );
		e.insert( "lrm", "‎" );
		e.insert( "rlm", "‏" );
		e.insert( "ndash", "–" );
		e.insert( "mdash", "—" );
		e.insert( "lsquo", "‘" );
		e.insert( "rsquo", "’" );
		e.insert( "sbquo", "‚" );
		e.insert( "ldquo", "“" );
		e.insert( "rdquo", "”" );
		e.insert( "bdquo", "„" );
		e.insert( "dagger", "†" );
		e.insert( "Dagger", "‡" );
		e.insert( "permil", "‰" );
		e.insert( "lsaquo", "‹" );
		e.insert( "rsaquo", "›" );
		e.insert( "euro", "€" );

		/* XML 1.0 https://www.w3.org/TR/xml/#sec-predefined-ent */
		// TODO, should XML/HTML not overlap here?
		// TODO, XML can have recursive &#38;s. ffs: <!ENTITY amp    "&#38;#38;">
		e.insert( "apos", "'" );

		/* XHTML 1.0 strictly supports &apos;, yet since HTML 4.0 user agents may not (hello IE 8), avoid */
		/* XHTML 1.0 https://www.w3.org/TR/xhtml1/#C_16 */

		/* HTML 5 … no, who thought in the age of Unicode we’d need more of those entities https://dev.w3.org/html5/html-author/charref */

		e
	};
}

// This demungs email headers of quoted printable escapement
/// Internal function that just decodes quoted words, for RFC 2047
fn decode_quoted_printable<'a>( s: &'a str, charset: &'a str ) -> Cow<'a, str> {

	lazy_static! {
		static ref NEED_TO_DECODE_ESCAPES: Regex = Regex::new( r"=[a-zA-Z0-9][a-zA-Z0-9]" ).unwrap( );
		static ref NEED_TO_DECODE_UNDERSCORE: Regex = Regex::new( r"_" ).unwrap( );
		static ref ESCAPES: regex::bytes::Regex = regex::bytes::Regex::new( r"((?P<encoded>(=[a-zA-Z0-9][a-zA-Z0-9]))|(?P<char>([^=])))" ).unwrap( );
	}

	if NEED_TO_DECODE_ESCAPES.is_match( &s ) ||
		NEED_TO_DECODE_UNDERSCORE.is_match( &s ) {

		// XXX Spec suggests we pass back original message, or a warning if things like Charset are unknown
		let charsetengine = encoding_from_whatwg_label( charset ).unwrap_or( encoding::all::ISO_8859_1 );
		let allo = s.to_string( ).into_bytes( );
		let allo: Vec<u8> = ESCAPES.replace_all( &allo, |cap: &regex::bytes::Captures| {

			match cap.name( "encoded" ) {
				Some( i ) => {

					let bytelist: Vec<u8> = i.as_bytes( ).chunks( 3 ).map(
						|x| u32::from_str_radix( std::str::from_utf8( &x[ 1..3 ] ).unwrap( ), 16 ).unwrap_or( 0 ) as u8
						).collect( );
					bytelist
				},
				None => {
					let character = cap.name( "char" ).unwrap( ).as_bytes( );
					if character == [95] {
						// _ → ‘ ’ // Spec says, _ should always decode to x20, whatever the charset
						vec![32]
					} else {
						character.to_vec( )
					}
				}
			}
		} ).into_owned( );

		let allo = charsetengine.decode( &allo, DecoderTrap::Replace ).unwrap_or( "�".to_string( ) );

		// TODO Make � replacement an option
		allo.into( )
	} else {
		s.into( )
	}
}


/// Decodes URL character sequences that are escaped into their UTF-8 form
///
/// # Examples
///
/// Can be called with `&' str`
///
/// ```
/// use mung::decode_rfc1738;
/// let title = decode_rfc1738( "%25" );
/// ```
///
/// or `String`
///
/// ```
/// use mung::decode_rfc1738;
/// let incoming_html = "/end_point/%3Fsource%3D%2Fdata%20here".to_string( );
/// let title = decode_rfc1738( &incoming_html );
/// ```
pub fn decode_rfc1738<'a>( s: &'a str ) -> Cow<'a, str> {

	lazy_static! {
		static ref HAS_TRIPLETS: Regex = Regex::new( r"%[a-zA-Z0-9][a-zA-Z0-9]" ).unwrap( );
		static ref TRIPLETS: regex::bytes::Regex = regex::bytes::Regex::new( r"%([a-zA-Z0-9][a-zA-Z0-9])" ).unwrap( );
	}

	if HAS_TRIPLETS.is_match( &s ) {

		// TODO replace this call with a strict encoding type
		let charsetengine = encoding_from_whatwg_label( "utf-8" ).unwrap_or( encoding::all::UTF_8 );
		// TODO spec strictly says US-ASCII, so.. this is wrong :^)
		let allo = s.to_string( ).into_bytes( );
		let allo: Vec<u8> = TRIPLETS.replace_all( &allo, |cap: &regex::bytes::Captures| {

				let bytelist: Vec<u8> = cap.get( 1 ).unwrap( ).as_bytes( ).chunks( 2 ).map(
					|x| u32::from_str_radix( std::str::from_utf8( &x ).unwrap( ), 16 ).unwrap_or( 65533 ) as u8
					).collect( );
				bytelist
			} ).into_owned( );

		let allo = charsetengine.decode( &allo, DecoderTrap::Replace ).unwrap_or( "�".to_string( ) );

		// TODO Make � replacement an option
		allo.into( )
	} else {
		s.into( )
	}
}


/// Decodes RFC 2047 encoded words into their UTF-8 form
///
/// See: Message Header Extensions for Non-ASCII Text https://tools.ietf.org/html/rfc2047
/// See also: base64 specification https://tools.ietf.org/html/rfc2045#page-24
///
/// For those unfamiliar, a base64 encoded Latin alphabet no. 1 string:
///
/// ```text
/// =?ISO-8859-1?B?SWYgeW91IGNhbiByZWFkIHRoaXMgeW91IHVuZGVyc3RhbmQgdGhlIGV4YW1wbGUu?=
/// ```
///
/// decodes to `If you can read this you understand the example.`
///
/// Known caveat of this function is that it returns as UTF-8; if you are
/// decoding into another character set, be aware you will need a further
/// conversion.
///
/// # Examples
///
/// Can be called with `&' str`
///
/// ```
/// use mung::decode_rfc2047;
/// let subject = decode_rfc2047( "Subject: =?GB2312?B?s8m5prXEsvrGt76twO0=?=" );
/// ```
pub fn decode_rfc2047<'a>( s: &'a str ) -> Cow<'a, str> {

	lazy_static! {
		static ref ENCODED_WORD: Regex = Regex::new( r"=\?(?P<charset>[^\?]*)\?(?P<encoding>[^\?]*)\?(?P<text>[^\?]*)\?=" ).unwrap( );
		// RFC 822 linear-white-space = 1*([CRLF] SPACE / HTAB)
		static ref LINEAR_WHITESPACE: Regex = Regex::new( r"\?=[\n\r\t ]+=\?" ).unwrap( );
	}
	if ENCODED_WORD.is_match( &s ) {

		let mut allo = s.to_string( );

		allo = LINEAR_WHITESPACE.replace_all( &allo, "?==?" ).into_owned( );
		allo = ENCODED_WORD.replace_all( &allo, |cap: &Captures| {
				let charset		= cap.get( 1 ).unwrap( ).as_str( ).to_lowercase( );
				let encoding	= cap.get( 2 ).unwrap( ).as_str( ).to_lowercase( );
				let encoded		= cap.get( 3 ).unwrap( ).as_str( );

				if encoding == "b" {
					let debased = match encoded.from_base64( ) {
						Ok( i ) => { i },
						_ => {
							// 6.2: display the 'encoded-word' as ordinary text
							return format!( "{}", encoded );
						}
					};

					// XXX this branch could be removed to use encoding instead? is it slower?
					if charset == "utf-8" {
						let data = String::from_utf8_lossy( &debased ).into_owned( );
						format!( "{}", data )
					} else {
						let charsetengine = encoding_from_whatwg_label( &charset ).unwrap_or( encoding::all::ISO_8859_1 );
						let replacement = charsetengine.decode( &debased, DecoderTrap::Replace ).unwrap( );
						format!( "{}", replacement )
					}
				} else if encoding == "q" {

					format!( "{}", decode_quoted_printable( encoded, &charset ) )

				} else {
					// 6.2: display the 'encoded-word' as ordinary text
					format!( "{}", encoded )
				}

			} ).into_owned( );

		// TODO Make � replacement an option
		allo.into( )
	} else {
		s.into( )
	}
}


// TODO support a list of HTML/XML ranges of entities to decode; default to all

/// Decodes HTML/XML entities into their UTF-8 form
///
/// # Examples
///
/// Can be called with `&' str`
///
/// ```
/// use mung::decode_entities;
/// let title = decode_entities( "Best &amp; the Worst of Times" );
/// ```
///
/// or `String`
///
/// ```
/// use mung::decode_entities;
/// let incoming_html = "An example of some love &heart;.".to_string( );
/// let title = decode_entities( &incoming_html );
/// ```
pub fn decode_entities<'a>( s: &'a str ) -> Cow<'a, str> {

	// &amp;amp; aren’t the only recursive escapes, we have &#38;#38; too
	lazy_static! {
		static ref HAS_ENTITIES:	Regex = Regex::new( r"&#?[a-zA-Z0-9]+;" ).unwrap( );
		static ref ENTITIES_NAME:	Regex = Regex::new( r"&([a-zA-Z0-9]+);" ).unwrap( );
		static ref ENTITIES_DEC:	Regex = Regex::new( r"&#(\d+);" ).unwrap( );
		static ref ENTITIES_HEX:	Regex = Regex::new( r"&#x([[:xdigit:]]+);" ).unwrap( );
	}

	if HAS_ENTITIES.is_match( &s ) {

		let mut allo = s.to_string( );

		while HAS_ENTITIES.is_match( &allo ) {

			allo = ENTITIES_NAME.replace_all( &allo, |cap: &Captures| {
					let origin = cap.get( 1 ).unwrap( ).as_str( );
					match ENTITIES.get( origin ) {
						Some( entity ) => format!( "{}", entity ),
						// XXX Debugging, this line needs to be replaced to output nothing, to prevent loooops
						// None => format!( "〖{}〗", origin )
						None => format!( "{}", origin )
					}
				} ).into_owned( );

			allo = ENTITIES_DEC.replace_all( &allo, |cap: &Captures| {
				format!( "{}", std::char::from_u32( cap.get( 1 ).unwrap( ).as_str(
					).parse( ).unwrap_or( 65533 ) ).unwrap_or( '�' ) ) } ).into_owned( );

			allo = ENTITIES_HEX.replace_all( &allo, |cap: &Captures| {
				format!( "{}", std::char::from_u32( u32::from_str_radix(
					cap.get( 1 ).unwrap( ).as_str( ), 16).unwrap_or( 65533
					) ).unwrap_or( '�' ) ) } ).into_owned( );
		}

		// TODO Make � replacement an option
		allo.into( )
	} else {
		s.into( )
	}
}


#[cfg( test )]
mod tests {
	use super::*;

	#[test]
	fn test_decode_entities( ) {

		assert_eq!( decode_entities( "" ),				"" );
		assert_eq!( decode_entities( "test." ),			"test." );
		assert_eq!( decode_entities( "&amp;" ),			"&" );
		assert_eq!( decode_entities( "&amp;amp;" ),		"&" );
		assert_eq!( decode_entities( "&#38;" ),			"&" );
		assert_eq!( decode_entities( "&#38;#38;" ),		"&" );
		assert_eq!( decode_entities( "&amp;lt;" ),		"<" );
		assert_eq!( decode_entities( "&#8800;" ),		"≠" );
		assert_eq!( decode_entities( "&amp;#8800;" ),	"≠" );
		assert_eq!( decode_entities( "&#x2665;" ),		"♥" );
		assert_eq!( decode_entities( "&amp;#x2665;" ),	"♥" );
		assert_eq!( decode_entities( "&#x9999999;" ),	"�" );

	}

	#[test]
	fn test_decode_rfc1738( ) {
		assert_eq!( decode_rfc1738( "%25" ),	"%" );

		// From https://www.w3.org/International/O-URL-code.html
		assert_eq!( decode_rfc1738( "Fran%c3%a7ois" ),	"François" );

		// From PSN Store URLs
		assert_eq!( decode_rfc1738( "assassin%27s-creed-chronicles-china" ),	"assassin\'s-creed-chronicles-china" );
		assert_eq!( decode_rfc1738( "assassin%e2%80%99s-creed-chronicles-russia" ),	"assassin’s-creed-chronicles-russia" );

		// Our doc
		assert_eq!( decode_rfc1738( "/end_point/%3Fsource%3D%2Fdata%20here" ),	"/end_point/?source=/data here" );
	}

	#[test]
	fn test_decode_entities_failure( ) {

		assert_ne!( decode_entities( "" ), " " );
		assert_ne!( decode_entities( "&amp;" ), "&amp;" );
		assert_ne!( decode_entities( "&amp;amp;" ), "&amp;amp;" );
		assert_ne!( decode_entities( "&#38;" ), "&#38;" );
		assert_ne!( decode_entities( "&#38;#38;" ), "&#38;" );
		assert_ne!( decode_entities( "&#38;#38;" ), "&#38;#38;" );
		assert_ne!( decode_entities( "&amp;lt;" ), "&amp;lt;" );
		assert_eq!( decode_entities( "&fred;" ), "fred" );
		assert_ne!( decode_entities( "&#8800;" ), "&#8800;" );
		assert_ne!( decode_entities( "&amp;#8800;" ), "&amp;#8800;" );
		assert_ne!( decode_entities( "&#x2665;" ), "&#x2665;" );
		assert_ne!( decode_entities( "&#x9999999;" ), "&#x9999999;" );
	}


	#[test]
	fn test_decode_rfc2047( ) {

		assert_eq!( decode_rfc2047( "Subject: =?utf-8?B?dGVzdA==?= testing" ),
									"Subject: test testing" );
		// These examples were pulled out of my Inbox before reaching the accolades of deletion
		// Apologies if they contain distasteful remarks —please report if they do— as I need to find good example content in wild.
		assert_eq!( decode_rfc2047( "Subject: =?utf-8?B?0J/Qu9Cw0YHRgtC40LrQvtCy0YvQtSBv0LrQvdCwINC/0L4g0YHQv9C10YbRhtC10L1l?=" ),
									"Subject: Пластиковые oкна по спецценe" );
		assert_eq!( decode_rfc2047( "Subject: =?utf-8?B?am9objrlpoLkvZXpmY3kvY7ph4fotK3nianlk4HnmoTlupPlrZjmiJDmnKzvvJ9k?==?utf-8?B?eXRtbw==?=" ),
									"Subject: john:如何降低采购物品的库存成本？dytmo" );
		assert_eq!( decode_rfc2047( "Subject: =?UTF-8?B?RG91Y2V1cnMgZCdoaXZlciA6IHRvdXQgw6AgLTQwJQ==?=" ),
									"Subject: Douceurs d'hiver : tout à -40%" );
		assert_eq!( decode_rfc2047( "Subject: =?utf-8?B?MDM6MjQ6MTfjgI485pyJPjzlj5E+POelqD485Luj?=
										=?utf-8?B?Pjzplos+44CP4piPOjEzNjQyMzUyNTMw5YiYKw==?=
										=?utf-8?B?UToyODU4MzcyMjY=?=" ),
									"Subject: 03:24:17『<有><发><票><代><開>』☏:13642352530刘+Q:285837226" );
		assert_eq!( decode_rfc2047( "Subject: =?GB2312?B?Mc67s7W85Nb3yM61xLncwO3QxLXDLMDPsOW3/sHL?=" ),
									"Subject: 1位车间主任的管理心得,老板服了" );
		assert_eq!( decode_rfc2047( "Subject: =?GB2312?B?s8m5prXEsvrGt76twO0=?=" ),
									"Subject: 成功的产品经理" );
		assert_eq!( decode_rfc2047( "Subject: =?big5?B?rPyw6q3suMu1UqdRpGggxenF57ZXr8WpyrdSpc2soSCw06t+OKfpIK5NwFw3p+k=?=zatb" ),
									"Subject: 美國原裝犀利士 體驗超級性愛生活 商品8折 套餐7折zatb" );
		assert_eq!( decode_rfc2047( "Subject: ?Big5?" ), "Subject: ?Big5?" );
		assert_eq!( decode_rfc2047( "From: =?utf-8?B?5YWa6Z2S5peL?= <rjunskde@tecye.net>" ),
									"From: 党青旋 <rjunskde@tecye.net>" );
		assert_eq!( decode_rfc2047( "From: =?utf-8?B?ItCX0JXQnNCb0K/QndCVIg==?= <8ac5d01@americamel.net>" ),
									"From: \"ЗЕМЛЯНЕ\" <8ac5d01@americamel.net>" );

		// examples direct from RFC https://tools.ietf.org/html/rfc2047
		assert_eq!( decode_rfc2047( "From: Nathaniel Borenstein <nsb@thumper.bellcore.com> (=?iso-8859-8?b?7eXs+SDv4SDp7Oj08A==?=)" ),
									"From: Nathaniel Borenstein <nsb@thumper.bellcore.com> (םולש ןב ילטפנ)" );
		assert_eq!( decode_rfc2047( "From: =?US-ASCII?Q?Keith_Moore?= <moore@cs.utk.edu>" ),
									"From: Keith Moore <moore@cs.utk.edu>" );
		assert_eq!( decode_rfc2047( "To: =?ISO-8859-1?Q?Keld_J=F8rn_Simonsen?= <keld@dkuug.dk>" ),
		// XXX removed this issue 2017-01-11: we currently *fail* for most quoted printable cases, because Rust converts int→char into UTF-8 codepoints, not keeping them as octets
		//							"To: Keld JÃ¸rn Simonsen <keld@dkuug.dk>" ); // it outputs this. To those unfamiliar, this is ø double encoded
									"To: Keld Jørn Simonsen <keld@dkuug.dk>" );
		assert_eq!( decode_rfc2047( "CC: =?ISO-8859-1?Q?Andr=E9?= Pirard <PIRARD@vm1.ulg.ac.be>" ),
									"CC: André Pirard <PIRARD@vm1.ulg.ac.be>" );
		assert_eq!( decode_rfc2047( "From: =?ISO-8859-1?Q?Patrik_F=E4ltstr=F6m?= <paf@nada.kth.se>" ),
									"From: Patrik Fältström <paf@nada.kth.se>" );
		assert_eq!( decode_rfc2047( "From: =?UTF-8?Q?Patrik_F=C3=A4ltstr=C3=B6m?= <paf@nada.kth.se>" ),
									"From: Patrik Fältström <paf@nada.kth.se>" );

		// example from search https://lists.debian.org/debian-chinese-big5/2002/09/msg00164.html
		assert_eq!( decode_rfc2047( "Subject: =?big5?Q?=A4=A3=AA=E1=BF=FA=A5u=AA=E1=B4X=A4=C0=AE=C9=B6=A1?=" ),
									"Subject: 不花錢只花幾分時間" );

		// From a spam email...
		assert_eq!( decode_rfc2047( "From: =?utf-8?Q?=E9=A6=99=E6=B8=AF=E8=BF=9B=E5=8F=A3?= <hk_import4@163.com>" ), "From: 香港进口 <hk_import4@163.com>" );
	}

	#[test]
	fn test_decode_rfc2047_whitespace( ) {

		assert_eq!( decode_rfc2047( "Subject: =?ISO-8859-1?B?SWYgeW91IGNhbiByZWFkIHRoaXMgeW8=?=
    =?ISO-8859-2?B?dSB1bmRlcnN0YW5kIHRoZSBleGFtcGxlLg==?=" ),
									"Subject: If you can read this you understand the example." );
		assert_eq!( decode_rfc2047( "(=?ISO-8859-1?Q?a?=)" ),						"(a)" );
		assert_eq!( decode_rfc2047( "(=?ISO-8859-1?Q?a?= b)" ),						"(a b)" );
		assert_eq!( decode_rfc2047( "(=?ISO-8859-1?Q?a?= =?ISO-8859-1?Q?b?=)" ),	"(ab)" );
		assert_eq!( decode_rfc2047( "(=?ISO-8859-1?Q?a?=  =?ISO-8859-1?Q?b?=)" ),	"(ab)" );
		assert_eq!( decode_rfc2047( "(=?ISO-8859-1?Q?a?=
             =?ISO-8859-1?Q?b?=)" ), "(ab)" ); // spaces
		assert_eq!( decode_rfc2047( "(=?ISO-8859-1?Q?a?=
			=?ISO-8859-1?Q?b?=)" ), "(ab)" ); // tabs
		assert_eq!( decode_rfc2047( "(=?ISO-8859-1?Q?a_b?=)" ), "(a b)" );
		assert_eq!( decode_rfc2047( "(=?ISO-8859-1?Q?a?= =?ISO-8859-1?Q?_b?=)" ),	"(a b)" );
	}

	// TODO make this test correctly test failures
	#[test]
	#[should_panic( expected = "assertion failed" )]
	fn test_decode_rfc2047_failure( ) {

		assert_eq!( decode_rfc2047( "=?utf-8?b?¢?= failure to decode base64" ), "failure to decode base64" );
		assert_eq!( decode_rfc2047( "=?utf-8?z?dGVzdA==?= unknown encoding" ), "unknown encoding" );
		assert_eq!( decode_rfc2047( "=?zalgo-he-comes?q?=AF?= unknown charset" ), "unknown charset" );
	}

	use super::decode_quoted_printable;

	#[test]
	fn test_decode_rfc2047_quoted_printable( ) {
		assert_eq!( decode_quoted_printable( "=0D=0A", "utf-8" ),	"\r\n" );
		assert_eq!( decode_quoted_printable( "=0d=0a", "utf-8" ),	"\r\n" );
		assert_eq!( decode_quoted_printable( "=F8", "ISO-8859-1" ),	"ø" );
		assert_eq!( decode_quoted_printable( "=F8", "ISO-8859-8" ),	"ר" );
		assert_eq!( decode_quoted_printable( "_",	"UTF-8" ),		" " );
		assert_eq!( decode_quoted_printable( "a",	"ISO-8859-1" ),	"a" );
		assert_eq!( decode_quoted_printable( "a_b", "ISO-8859-1" ),	"a b" );
		assert_eq!( decode_quoted_printable( "dog", "ISO-8859-1" ),	"dog" );
		assert_eq!( decode_quoted_printable( "___", "ISO-8859-1" ),	"   " );
	}

}
