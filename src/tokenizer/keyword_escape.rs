use super::tokens::{RawKeyword, RawToken};
#[allow(clippy::cognitive_complexity)]
pub fn check_complicated_keyword(ident: &[u8]) -> Option<RawToken> {
    match ident.len() {
        7 if ident == br"\u0064o" => Some(RawToken::Keyword(RawKeyword::Do)),
        7 if ident == br"\u{64}o" => Some(RawToken::Keyword(RawKeyword::Do)),
        7 if ident == br"d\u006f" => Some(RawToken::Keyword(RawKeyword::Do)),
        7 if ident == br"d\u{6f}" => Some(RawToken::Keyword(RawKeyword::Do)),
        7 if ident == br"\u0069f" => Some(RawToken::Keyword(RawKeyword::If)),
        7 if ident == br"\u{69}f" => Some(RawToken::Keyword(RawKeyword::If)),
        7 if ident == br"i\u0066" => Some(RawToken::Keyword(RawKeyword::If)),
        7 if ident == br"i\u{66}" => Some(RawToken::Keyword(RawKeyword::If)),
        7 if ident == br"\u0069n" => Some(RawToken::Keyword(RawKeyword::In)),
        7 if ident == br"\u{69}n" => Some(RawToken::Keyword(RawKeyword::In)),
        7 if ident == br"i\u006e" => Some(RawToken::Keyword(RawKeyword::In)),
        7 if ident == br"i\u{6e}" => Some(RawToken::Keyword(RawKeyword::In)),
        8 if ident == br"\u0066or" => Some(RawToken::Keyword(RawKeyword::For)),
        8 if ident == br"\u{66}or" => Some(RawToken::Keyword(RawKeyword::For)),
        8 if ident == br"f\u006fr" => Some(RawToken::Keyword(RawKeyword::For)),
        8 if ident == br"f\u{6f}r" => Some(RawToken::Keyword(RawKeyword::For)),
        8 if ident == br"fo\u0072" => Some(RawToken::Keyword(RawKeyword::For)),
        8 if ident == br"fo\u{72}" => Some(RawToken::Keyword(RawKeyword::For)),
        8 if ident == br"\u006eew" => Some(RawToken::Keyword(RawKeyword::New)),
        8 if ident == br"\u{6e}ew" => Some(RawToken::Keyword(RawKeyword::New)),
        8 if ident == br"n\u0065w" => Some(RawToken::Keyword(RawKeyword::New)),
        8 if ident == br"n\u{65}w" => Some(RawToken::Keyword(RawKeyword::New)),
        8 if ident == br"ne\u0077" => Some(RawToken::Keyword(RawKeyword::New)),
        8 if ident == br"ne\u{77}" => Some(RawToken::Keyword(RawKeyword::New)),
        8 if ident == br"\u0074ry" => Some(RawToken::Keyword(RawKeyword::Try)),
        8 if ident == br"\u{74}ry" => Some(RawToken::Keyword(RawKeyword::Try)),
        8 if ident == br"t\u0072y" => Some(RawToken::Keyword(RawKeyword::Try)),
        8 if ident == br"t\u{72}y" => Some(RawToken::Keyword(RawKeyword::Try)),
        8 if ident == br"tr\u0079" => Some(RawToken::Keyword(RawKeyword::Try)),
        8 if ident == br"tr\u{79}" => Some(RawToken::Keyword(RawKeyword::Try)),
        8 if ident == br"\u0076ar" => Some(RawToken::Keyword(RawKeyword::Var)),
        8 if ident == br"\u{76}ar" => Some(RawToken::Keyword(RawKeyword::Var)),
        8 if ident == br"v\u0061r" => Some(RawToken::Keyword(RawKeyword::Var)),
        8 if ident == br"v\u{61}r" => Some(RawToken::Keyword(RawKeyword::Var)),
        8 if ident == br"va\u0072" => Some(RawToken::Keyword(RawKeyword::Var)),
        8 if ident == br"va\u{72}" => Some(RawToken::Keyword(RawKeyword::Var)),
        8 if ident == br"\u006cet" => Some(RawToken::Keyword(RawKeyword::Let)),
        8 if ident == br"\u{6c}et" => Some(RawToken::Keyword(RawKeyword::Let)),
        8 if ident == br"l\u0065t" => Some(RawToken::Keyword(RawKeyword::Let)),
        8 if ident == br"l\u{65}t" => Some(RawToken::Keyword(RawKeyword::Let)),
        8 if ident == br"le\u0074" => Some(RawToken::Keyword(RawKeyword::Let)),
        8 if ident == br"le\u{74}" => Some(RawToken::Keyword(RawKeyword::Let)),
        9 if ident == br"\u0063ase" => Some(RawToken::Keyword(RawKeyword::Case)),
        9 if ident == br"\u{63}ase" => Some(RawToken::Keyword(RawKeyword::Case)),
        9 if ident == br"c\u0061se" => Some(RawToken::Keyword(RawKeyword::Case)),
        9 if ident == br"c\u{61}se" => Some(RawToken::Keyword(RawKeyword::Case)),
        9 if ident == br"ca\u0073e" => Some(RawToken::Keyword(RawKeyword::Case)),
        9 if ident == br"ca\u{73}e" => Some(RawToken::Keyword(RawKeyword::Case)),
        9 if ident == br"cas\u0065" => Some(RawToken::Keyword(RawKeyword::Case)),
        9 if ident == br"cas\u{65}" => Some(RawToken::Keyword(RawKeyword::Case)),
        9 if ident == br"\u0074his" => Some(RawToken::Keyword(RawKeyword::This)),
        9 if ident == br"\u{74}his" => Some(RawToken::Keyword(RawKeyword::This)),
        9 if ident == br"t\u0068is" => Some(RawToken::Keyword(RawKeyword::This)),
        9 if ident == br"t\u{68}is" => Some(RawToken::Keyword(RawKeyword::This)),
        9 if ident == br"th\u0069s" => Some(RawToken::Keyword(RawKeyword::This)),
        9 if ident == br"th\u{69}s" => Some(RawToken::Keyword(RawKeyword::This)),
        9 if ident == br"thi\u0073" => Some(RawToken::Keyword(RawKeyword::This)),
        9 if ident == br"thi\u{73}" => Some(RawToken::Keyword(RawKeyword::This)),
        9 if ident == br"\u0076oid" => Some(RawToken::Keyword(RawKeyword::Void)),
        9 if ident == br"\u{76}oid" => Some(RawToken::Keyword(RawKeyword::Void)),
        9 if ident == br"v\u006fid" => Some(RawToken::Keyword(RawKeyword::Void)),
        9 if ident == br"v\u{6f}id" => Some(RawToken::Keyword(RawKeyword::Void)),
        9 if ident == br"vo\u0069d" => Some(RawToken::Keyword(RawKeyword::Void)),
        9 if ident == br"vo\u{69}d" => Some(RawToken::Keyword(RawKeyword::Void)),
        9 if ident == br"voi\u0064" => Some(RawToken::Keyword(RawKeyword::Void)),
        9 if ident == br"voi\u{64}" => Some(RawToken::Keyword(RawKeyword::Void)),
        9 if ident == br"\u0077ith" => Some(RawToken::Keyword(RawKeyword::With)),
        9 if ident == br"\u{77}ith" => Some(RawToken::Keyword(RawKeyword::With)),
        9 if ident == br"w\u0069th" => Some(RawToken::Keyword(RawKeyword::With)),
        9 if ident == br"w\u{69}th" => Some(RawToken::Keyword(RawKeyword::With)),
        9 if ident == br"wi\u0074h" => Some(RawToken::Keyword(RawKeyword::With)),
        9 if ident == br"wi\u{74}h" => Some(RawToken::Keyword(RawKeyword::With)),
        9 if ident == br"wit\u0068" => Some(RawToken::Keyword(RawKeyword::With)),
        9 if ident == br"wit\u{68}" => Some(RawToken::Keyword(RawKeyword::With)),
        9 if ident == br"\u0065num" => Some(RawToken::Keyword(RawKeyword::Enum)),
        9 if ident == br"\u{65}num" => Some(RawToken::Keyword(RawKeyword::Enum)),
        9 if ident == br"e\u006eum" => Some(RawToken::Keyword(RawKeyword::Enum)),
        9 if ident == br"e\u{6e}um" => Some(RawToken::Keyword(RawKeyword::Enum)),
        9 if ident == br"en\u0075m" => Some(RawToken::Keyword(RawKeyword::Enum)),
        9 if ident == br"en\u{75}m" => Some(RawToken::Keyword(RawKeyword::Enum)),
        9 if ident == br"enu\u006d" => Some(RawToken::Keyword(RawKeyword::Enum)),
        9 if ident == br"enu\u{6d}" => Some(RawToken::Keyword(RawKeyword::Enum)),
        9 if ident == br"\u0065lse" => Some(RawToken::Keyword(RawKeyword::Else)),
        9 if ident == br"\u{65}lse" => Some(RawToken::Keyword(RawKeyword::Else)),
        9 if ident == br"e\u006cse" => Some(RawToken::Keyword(RawKeyword::Else)),
        9 if ident == br"e\u{6c}se" => Some(RawToken::Keyword(RawKeyword::Else)),
        9 if ident == br"el\u0073e" => Some(RawToken::Keyword(RawKeyword::Else)),
        9 if ident == br"el\u{73}e" => Some(RawToken::Keyword(RawKeyword::Else)),
        9 if ident == br"els\u0065" => Some(RawToken::Keyword(RawKeyword::Else)),
        9 if ident == br"els\u{65}" => Some(RawToken::Keyword(RawKeyword::Else)),
        9 if ident == br"\u0074rue" => Some(RawToken::Boolean(true)),
        9 if ident == br"\u{74}rue" => Some(RawToken::Boolean(true)),
        9 if ident == br"t\u0072ue" => Some(RawToken::Boolean(true)),
        9 if ident == br"t\u{72}ue" => Some(RawToken::Boolean(true)),
        9 if ident == br"tr\u0075e" => Some(RawToken::Boolean(true)),
        9 if ident == br"tr\u{75}e" => Some(RawToken::Boolean(true)),
        9 if ident == br"tru\u0065" => Some(RawToken::Boolean(true)),
        9 if ident == br"tru\u{65}" => Some(RawToken::Boolean(true)),
        9 if ident == br"\u006eull" => Some(RawToken::Null),
        9 if ident == br"\u{6e}ull" => Some(RawToken::Null),
        9 if ident == br"n\u0075ll" => Some(RawToken::Null),
        9 if ident == br"n\u{75}ll" => Some(RawToken::Null),
        9 if ident == br"nu\u006cl" => Some(RawToken::Null),
        9 if ident == br"nu\u{6c}l" => Some(RawToken::Null),
        9 if ident == br"nul\u006c" => Some(RawToken::Null),
        9 if ident == br"nul\u{6c}" => Some(RawToken::Null),
        10 if ident == br"\u0061wait" => Some(RawToken::Keyword(RawKeyword::Await)),
        10 if ident == br"\u{61}wait" => Some(RawToken::Keyword(RawKeyword::Await)),
        10 if ident == br"a\u0077ait" => Some(RawToken::Keyword(RawKeyword::Await)),
        10 if ident == br"a\u{77}ait" => Some(RawToken::Keyword(RawKeyword::Await)),
        10 if ident == br"aw\u0061it" => Some(RawToken::Keyword(RawKeyword::Await)),
        10 if ident == br"aw\u{61}it" => Some(RawToken::Keyword(RawKeyword::Await)),
        10 if ident == br"awa\u0069t" => Some(RawToken::Keyword(RawKeyword::Await)),
        10 if ident == br"awa\u{69}t" => Some(RawToken::Keyword(RawKeyword::Await)),
        10 if ident == br"awai\u0074" => Some(RawToken::Keyword(RawKeyword::Await)),
        10 if ident == br"awai\u{74}" => Some(RawToken::Keyword(RawKeyword::Await)),
        10 if ident == br"\u0062reak" => Some(RawToken::Keyword(RawKeyword::Break)),
        10 if ident == br"\u{62}reak" => Some(RawToken::Keyword(RawKeyword::Break)),
        10 if ident == br"b\u0072eak" => Some(RawToken::Keyword(RawKeyword::Break)),
        10 if ident == br"b\u{72}eak" => Some(RawToken::Keyword(RawKeyword::Break)),
        10 if ident == br"br\u0065ak" => Some(RawToken::Keyword(RawKeyword::Break)),
        10 if ident == br"br\u{65}ak" => Some(RawToken::Keyword(RawKeyword::Break)),
        10 if ident == br"bre\u0061k" => Some(RawToken::Keyword(RawKeyword::Break)),
        10 if ident == br"bre\u{61}k" => Some(RawToken::Keyword(RawKeyword::Break)),
        10 if ident == br"brea\u006b" => Some(RawToken::Keyword(RawKeyword::Break)),
        10 if ident == br"brea\u{6b}" => Some(RawToken::Keyword(RawKeyword::Break)),
        10 if ident == br"\u0063atch" => Some(RawToken::Keyword(RawKeyword::Catch)),
        10 if ident == br"\u{63}atch" => Some(RawToken::Keyword(RawKeyword::Catch)),
        10 if ident == br"c\u0061tch" => Some(RawToken::Keyword(RawKeyword::Catch)),
        10 if ident == br"c\u{61}tch" => Some(RawToken::Keyword(RawKeyword::Catch)),
        10 if ident == br"ca\u0074ch" => Some(RawToken::Keyword(RawKeyword::Catch)),
        10 if ident == br"ca\u{74}ch" => Some(RawToken::Keyword(RawKeyword::Catch)),
        10 if ident == br"cat\u0063h" => Some(RawToken::Keyword(RawKeyword::Catch)),
        10 if ident == br"cat\u{63}h" => Some(RawToken::Keyword(RawKeyword::Catch)),
        10 if ident == br"catc\u0068" => Some(RawToken::Keyword(RawKeyword::Catch)),
        10 if ident == br"catc\u{68}" => Some(RawToken::Keyword(RawKeyword::Catch)),
        10 if ident == br"\u0063lass" => Some(RawToken::Keyword(RawKeyword::Class)),
        10 if ident == br"\u{63}lass" => Some(RawToken::Keyword(RawKeyword::Class)),
        10 if ident == br"c\u006cass" => Some(RawToken::Keyword(RawKeyword::Class)),
        10 if ident == br"c\u{6c}ass" => Some(RawToken::Keyword(RawKeyword::Class)),
        10 if ident == br"cl\u0061ss" => Some(RawToken::Keyword(RawKeyword::Class)),
        10 if ident == br"cl\u{61}ss" => Some(RawToken::Keyword(RawKeyword::Class)),
        10 if ident == br"cla\u0073s" => Some(RawToken::Keyword(RawKeyword::Class)),
        10 if ident == br"cla\u{73}s" => Some(RawToken::Keyword(RawKeyword::Class)),
        10 if ident == br"clas\u0073" => Some(RawToken::Keyword(RawKeyword::Class)),
        10 if ident == br"clas\u{73}" => Some(RawToken::Keyword(RawKeyword::Class)),
        10 if ident == br"\u0063onst" => Some(RawToken::Keyword(RawKeyword::Const)),
        10 if ident == br"\u{63}onst" => Some(RawToken::Keyword(RawKeyword::Const)),
        10 if ident == br"c\u006fnst" => Some(RawToken::Keyword(RawKeyword::Const)),
        10 if ident == br"c\u{6f}nst" => Some(RawToken::Keyword(RawKeyword::Const)),
        10 if ident == br"co\u006est" => Some(RawToken::Keyword(RawKeyword::Const)),
        10 if ident == br"co\u{6e}st" => Some(RawToken::Keyword(RawKeyword::Const)),
        10 if ident == br"con\u0073t" => Some(RawToken::Keyword(RawKeyword::Const)),
        10 if ident == br"con\u{73}t" => Some(RawToken::Keyword(RawKeyword::Const)),
        10 if ident == br"cons\u0074" => Some(RawToken::Keyword(RawKeyword::Const)),
        10 if ident == br"cons\u{74}" => Some(RawToken::Keyword(RawKeyword::Const)),
        10 if ident == br"\u0074hrow" => Some(RawToken::Keyword(RawKeyword::Throw)),
        10 if ident == br"\u{74}hrow" => Some(RawToken::Keyword(RawKeyword::Throw)),
        10 if ident == br"t\u0068row" => Some(RawToken::Keyword(RawKeyword::Throw)),
        10 if ident == br"t\u{68}row" => Some(RawToken::Keyword(RawKeyword::Throw)),
        10 if ident == br"th\u0072ow" => Some(RawToken::Keyword(RawKeyword::Throw)),
        10 if ident == br"th\u{72}ow" => Some(RawToken::Keyword(RawKeyword::Throw)),
        10 if ident == br"thr\u006fw" => Some(RawToken::Keyword(RawKeyword::Throw)),
        10 if ident == br"thr\u{6f}w" => Some(RawToken::Keyword(RawKeyword::Throw)),
        10 if ident == br"thro\u0077" => Some(RawToken::Keyword(RawKeyword::Throw)),
        10 if ident == br"thro\u{77}" => Some(RawToken::Keyword(RawKeyword::Throw)),
        10 if ident == br"\u0077hile" => Some(RawToken::Keyword(RawKeyword::While)),
        10 if ident == br"\u{77}hile" => Some(RawToken::Keyword(RawKeyword::While)),
        10 if ident == br"w\u0068ile" => Some(RawToken::Keyword(RawKeyword::While)),
        10 if ident == br"w\u{68}ile" => Some(RawToken::Keyword(RawKeyword::While)),
        10 if ident == br"wh\u0069le" => Some(RawToken::Keyword(RawKeyword::While)),
        10 if ident == br"wh\u{69}le" => Some(RawToken::Keyword(RawKeyword::While)),
        10 if ident == br"whi\u006ce" => Some(RawToken::Keyword(RawKeyword::While)),
        10 if ident == br"whi\u{6c}e" => Some(RawToken::Keyword(RawKeyword::While)),
        10 if ident == br"whil\u0065" => Some(RawToken::Keyword(RawKeyword::While)),
        10 if ident == br"whil\u{65}" => Some(RawToken::Keyword(RawKeyword::While)),
        10 if ident == br"\u0073uper" => Some(RawToken::Keyword(RawKeyword::Super)),
        10 if ident == br"\u{73}uper" => Some(RawToken::Keyword(RawKeyword::Super)),
        10 if ident == br"s\u0075per" => Some(RawToken::Keyword(RawKeyword::Super)),
        10 if ident == br"s\u{75}per" => Some(RawToken::Keyword(RawKeyword::Super)),
        10 if ident == br"su\u0070er" => Some(RawToken::Keyword(RawKeyword::Super)),
        10 if ident == br"su\u{70}er" => Some(RawToken::Keyword(RawKeyword::Super)),
        10 if ident == br"sup\u0065r" => Some(RawToken::Keyword(RawKeyword::Super)),
        10 if ident == br"sup\u{65}r" => Some(RawToken::Keyword(RawKeyword::Super)),
        10 if ident == br"supe\u0072" => Some(RawToken::Keyword(RawKeyword::Super)),
        10 if ident == br"supe\u{72}" => Some(RawToken::Keyword(RawKeyword::Super)),
        10 if ident == br"\u0079ield" => Some(RawToken::Keyword(RawKeyword::Yield)),
        10 if ident == br"\u{79}ield" => Some(RawToken::Keyword(RawKeyword::Yield)),
        10 if ident == br"y\u0069eld" => Some(RawToken::Keyword(RawKeyword::Yield)),
        10 if ident == br"y\u{69}eld" => Some(RawToken::Keyword(RawKeyword::Yield)),
        10 if ident == br"yi\u0065ld" => Some(RawToken::Keyword(RawKeyword::Yield)),
        10 if ident == br"yi\u{65}ld" => Some(RawToken::Keyword(RawKeyword::Yield)),
        10 if ident == br"yie\u006cd" => Some(RawToken::Keyword(RawKeyword::Yield)),
        10 if ident == br"yie\u{6c}d" => Some(RawToken::Keyword(RawKeyword::Yield)),
        10 if ident == br"yiel\u0064" => Some(RawToken::Keyword(RawKeyword::Yield)),
        10 if ident == br"yiel\u{64}" => Some(RawToken::Keyword(RawKeyword::Yield)),
        10 if ident == br"\u0066alse" => Some(RawToken::Boolean(false)),
        10 if ident == br"\u{66}alse" => Some(RawToken::Boolean(false)),
        10 if ident == br"f\u0061lse" => Some(RawToken::Boolean(false)),
        10 if ident == br"f\u{61}lse" => Some(RawToken::Boolean(false)),
        10 if ident == br"fa\u006cse" => Some(RawToken::Boolean(false)),
        10 if ident == br"fa\u{6c}se" => Some(RawToken::Boolean(false)),
        10 if ident == br"fal\u0073e" => Some(RawToken::Boolean(false)),
        10 if ident == br"fal\u{73}e" => Some(RawToken::Boolean(false)),
        10 if ident == br"fals\u0065" => Some(RawToken::Boolean(false)),
        10 if ident == br"fals\u{65}" => Some(RawToken::Boolean(false)),
        11 if ident == br"\u0064elete" => Some(RawToken::Keyword(RawKeyword::Delete)),
        11 if ident == br"\u{64}elete" => Some(RawToken::Keyword(RawKeyword::Delete)),
        11 if ident == br"d\u0065lete" => Some(RawToken::Keyword(RawKeyword::Delete)),
        11 if ident == br"d\u{65}lete" => Some(RawToken::Keyword(RawKeyword::Delete)),
        11 if ident == br"de\u006cete" => Some(RawToken::Keyword(RawKeyword::Delete)),
        11 if ident == br"de\u{6c}ete" => Some(RawToken::Keyword(RawKeyword::Delete)),
        11 if ident == br"del\u0065te" => Some(RawToken::Keyword(RawKeyword::Delete)),
        11 if ident == br"del\u{65}te" => Some(RawToken::Keyword(RawKeyword::Delete)),
        11 if ident == br"dele\u0074e" => Some(RawToken::Keyword(RawKeyword::Delete)),
        11 if ident == br"dele\u{74}e" => Some(RawToken::Keyword(RawKeyword::Delete)),
        11 if ident == br"delet\u0065" => Some(RawToken::Keyword(RawKeyword::Delete)),
        11 if ident == br"delet\u{65}" => Some(RawToken::Keyword(RawKeyword::Delete)),
        11 if ident == br"\u0072eturn" => Some(RawToken::Keyword(RawKeyword::Return)),
        11 if ident == br"\u{72}eturn" => Some(RawToken::Keyword(RawKeyword::Return)),
        11 if ident == br"r\u0065turn" => Some(RawToken::Keyword(RawKeyword::Return)),
        11 if ident == br"r\u{65}turn" => Some(RawToken::Keyword(RawKeyword::Return)),
        11 if ident == br"re\u0074urn" => Some(RawToken::Keyword(RawKeyword::Return)),
        11 if ident == br"re\u{74}urn" => Some(RawToken::Keyword(RawKeyword::Return)),
        11 if ident == br"ret\u0075rn" => Some(RawToken::Keyword(RawKeyword::Return)),
        11 if ident == br"ret\u{75}rn" => Some(RawToken::Keyword(RawKeyword::Return)),
        11 if ident == br"retu\u0072n" => Some(RawToken::Keyword(RawKeyword::Return)),
        11 if ident == br"retu\u{72}n" => Some(RawToken::Keyword(RawKeyword::Return)),
        11 if ident == br"retur\u006e" => Some(RawToken::Keyword(RawKeyword::Return)),
        11 if ident == br"retur\u{6e}" => Some(RawToken::Keyword(RawKeyword::Return)),
        11 if ident == br"\u0073witch" => Some(RawToken::Keyword(RawKeyword::Switch)),
        11 if ident == br"\u{73}witch" => Some(RawToken::Keyword(RawKeyword::Switch)),
        11 if ident == br"s\u0077itch" => Some(RawToken::Keyword(RawKeyword::Switch)),
        11 if ident == br"s\u{77}itch" => Some(RawToken::Keyword(RawKeyword::Switch)),
        11 if ident == br"sw\u0069tch" => Some(RawToken::Keyword(RawKeyword::Switch)),
        11 if ident == br"sw\u{69}tch" => Some(RawToken::Keyword(RawKeyword::Switch)),
        11 if ident == br"swi\u0074ch" => Some(RawToken::Keyword(RawKeyword::Switch)),
        11 if ident == br"swi\u{74}ch" => Some(RawToken::Keyword(RawKeyword::Switch)),
        11 if ident == br"swit\u0063h" => Some(RawToken::Keyword(RawKeyword::Switch)),
        11 if ident == br"swit\u{63}h" => Some(RawToken::Keyword(RawKeyword::Switch)),
        11 if ident == br"switc\u0068" => Some(RawToken::Keyword(RawKeyword::Switch)),
        11 if ident == br"switc\u{68}" => Some(RawToken::Keyword(RawKeyword::Switch)),
        11 if ident == br"\u0074ypeof" => Some(RawToken::Keyword(RawKeyword::TypeOf)),
        11 if ident == br"\u{74}ypeof" => Some(RawToken::Keyword(RawKeyword::TypeOf)),
        11 if ident == br"t\u0079peof" => Some(RawToken::Keyword(RawKeyword::TypeOf)),
        11 if ident == br"t\u{79}peof" => Some(RawToken::Keyword(RawKeyword::TypeOf)),
        11 if ident == br"ty\u0070eof" => Some(RawToken::Keyword(RawKeyword::TypeOf)),
        11 if ident == br"ty\u{70}eof" => Some(RawToken::Keyword(RawKeyword::TypeOf)),
        11 if ident == br"typ\u0065of" => Some(RawToken::Keyword(RawKeyword::TypeOf)),
        11 if ident == br"typ\u{65}of" => Some(RawToken::Keyword(RawKeyword::TypeOf)),
        11 if ident == br"type\u006ff" => Some(RawToken::Keyword(RawKeyword::TypeOf)),
        11 if ident == br"type\u{6f}f" => Some(RawToken::Keyword(RawKeyword::TypeOf)),
        11 if ident == br"typeo\u0066" => Some(RawToken::Keyword(RawKeyword::TypeOf)),
        11 if ident == br"typeo\u{66}" => Some(RawToken::Keyword(RawKeyword::TypeOf)),
        11 if ident == br"\u0065xport" => Some(RawToken::Keyword(RawKeyword::Export)),
        11 if ident == br"\u{65}xport" => Some(RawToken::Keyword(RawKeyword::Export)),
        11 if ident == br"e\u0078port" => Some(RawToken::Keyword(RawKeyword::Export)),
        11 if ident == br"e\u{78}port" => Some(RawToken::Keyword(RawKeyword::Export)),
        11 if ident == br"ex\u0070ort" => Some(RawToken::Keyword(RawKeyword::Export)),
        11 if ident == br"ex\u{70}ort" => Some(RawToken::Keyword(RawKeyword::Export)),
        11 if ident == br"exp\u006frt" => Some(RawToken::Keyword(RawKeyword::Export)),
        11 if ident == br"exp\u{6f}rt" => Some(RawToken::Keyword(RawKeyword::Export)),
        11 if ident == br"expo\u0072t" => Some(RawToken::Keyword(RawKeyword::Export)),
        11 if ident == br"expo\u{72}t" => Some(RawToken::Keyword(RawKeyword::Export)),
        11 if ident == br"expor\u0074" => Some(RawToken::Keyword(RawKeyword::Export)),
        11 if ident == br"expor\u{74}" => Some(RawToken::Keyword(RawKeyword::Export)),
        11 if ident == br"\u0069mport" => Some(RawToken::Keyword(RawKeyword::Import)),
        11 if ident == br"\u{69}mport" => Some(RawToken::Keyword(RawKeyword::Import)),
        11 if ident == br"i\u006dport" => Some(RawToken::Keyword(RawKeyword::Import)),
        11 if ident == br"i\u{6d}port" => Some(RawToken::Keyword(RawKeyword::Import)),
        11 if ident == br"im\u0070ort" => Some(RawToken::Keyword(RawKeyword::Import)),
        11 if ident == br"im\u{70}ort" => Some(RawToken::Keyword(RawKeyword::Import)),
        11 if ident == br"imp\u006frt" => Some(RawToken::Keyword(RawKeyword::Import)),
        11 if ident == br"imp\u{6f}rt" => Some(RawToken::Keyword(RawKeyword::Import)),
        11 if ident == br"impo\u0072t" => Some(RawToken::Keyword(RawKeyword::Import)),
        11 if ident == br"impo\u{72}t" => Some(RawToken::Keyword(RawKeyword::Import)),
        11 if ident == br"impor\u0074" => Some(RawToken::Keyword(RawKeyword::Import)),
        11 if ident == br"impor\u{74}" => Some(RawToken::Keyword(RawKeyword::Import)),
        11 if ident == br"\u0073tatic" => Some(RawToken::Keyword(RawKeyword::Static)),
        11 if ident == br"\u{73}tatic" => Some(RawToken::Keyword(RawKeyword::Static)),
        11 if ident == br"s\u0074atic" => Some(RawToken::Keyword(RawKeyword::Static)),
        11 if ident == br"s\u{74}atic" => Some(RawToken::Keyword(RawKeyword::Static)),
        11 if ident == br"st\u0061tic" => Some(RawToken::Keyword(RawKeyword::Static)),
        11 if ident == br"st\u{61}tic" => Some(RawToken::Keyword(RawKeyword::Static)),
        11 if ident == br"sta\u0074ic" => Some(RawToken::Keyword(RawKeyword::Static)),
        11 if ident == br"sta\u{74}ic" => Some(RawToken::Keyword(RawKeyword::Static)),
        11 if ident == br"stat\u0069c" => Some(RawToken::Keyword(RawKeyword::Static)),
        11 if ident == br"stat\u{69}c" => Some(RawToken::Keyword(RawKeyword::Static)),
        11 if ident == br"stati\u0063" => Some(RawToken::Keyword(RawKeyword::Static)),
        11 if ident == br"stati\u{63}" => Some(RawToken::Keyword(RawKeyword::Static)),
        11 if ident == br"\u0070ublic" => Some(RawToken::Keyword(RawKeyword::Public)),
        11 if ident == br"\u{70}ublic" => Some(RawToken::Keyword(RawKeyword::Public)),
        11 if ident == br"p\u0075blic" => Some(RawToken::Keyword(RawKeyword::Public)),
        11 if ident == br"p\u{75}blic" => Some(RawToken::Keyword(RawKeyword::Public)),
        11 if ident == br"pu\u0062lic" => Some(RawToken::Keyword(RawKeyword::Public)),
        11 if ident == br"pu\u{62}lic" => Some(RawToken::Keyword(RawKeyword::Public)),
        11 if ident == br"pub\u006cic" => Some(RawToken::Keyword(RawKeyword::Public)),
        11 if ident == br"pub\u{6c}ic" => Some(RawToken::Keyword(RawKeyword::Public)),
        11 if ident == br"publ\u0069c" => Some(RawToken::Keyword(RawKeyword::Public)),
        11 if ident == br"publ\u{69}c" => Some(RawToken::Keyword(RawKeyword::Public)),
        11 if ident == br"publi\u0063" => Some(RawToken::Keyword(RawKeyword::Public)),
        11 if ident == br"publi\u{63}" => Some(RawToken::Keyword(RawKeyword::Public)),
        12 if ident == br"\u0064\u006f" => Some(RawToken::Keyword(RawKeyword::Do)),
        12 if ident == br"\u{64}\u{6f}" => Some(RawToken::Keyword(RawKeyword::Do)),
        12 if ident == br"\u0069\u0066" => Some(RawToken::Keyword(RawKeyword::If)),
        12 if ident == br"\u{69}\u{66}" => Some(RawToken::Keyword(RawKeyword::If)),
        12 if ident == br"\u0069\u006e" => Some(RawToken::Keyword(RawKeyword::In)),
        12 if ident == br"\u{69}\u{6e}" => Some(RawToken::Keyword(RawKeyword::In)),
        12 if ident == br"\u0065xtends" => Some(RawToken::Keyword(RawKeyword::Extends)),
        12 if ident == br"\u{65}xtends" => Some(RawToken::Keyword(RawKeyword::Extends)),
        12 if ident == br"e\u0078tends" => Some(RawToken::Keyword(RawKeyword::Extends)),
        12 if ident == br"e\u{78}tends" => Some(RawToken::Keyword(RawKeyword::Extends)),
        12 if ident == br"ex\u0074ends" => Some(RawToken::Keyword(RawKeyword::Extends)),
        12 if ident == br"ex\u{74}ends" => Some(RawToken::Keyword(RawKeyword::Extends)),
        12 if ident == br"ext\u0065nds" => Some(RawToken::Keyword(RawKeyword::Extends)),
        12 if ident == br"ext\u{65}nds" => Some(RawToken::Keyword(RawKeyword::Extends)),
        12 if ident == br"exte\u006eds" => Some(RawToken::Keyword(RawKeyword::Extends)),
        12 if ident == br"exte\u{6e}ds" => Some(RawToken::Keyword(RawKeyword::Extends)),
        12 if ident == br"exten\u0064s" => Some(RawToken::Keyword(RawKeyword::Extends)),
        12 if ident == br"exten\u{64}s" => Some(RawToken::Keyword(RawKeyword::Extends)),
        12 if ident == br"extend\u0073" => Some(RawToken::Keyword(RawKeyword::Extends)),
        12 if ident == br"extend\u{73}" => Some(RawToken::Keyword(RawKeyword::Extends)),
        12 if ident == br"\u0064efault" => Some(RawToken::Keyword(RawKeyword::Default)),
        12 if ident == br"\u{64}efault" => Some(RawToken::Keyword(RawKeyword::Default)),
        12 if ident == br"d\u0065fault" => Some(RawToken::Keyword(RawKeyword::Default)),
        12 if ident == br"d\u{65}fault" => Some(RawToken::Keyword(RawKeyword::Default)),
        12 if ident == br"de\u0066ault" => Some(RawToken::Keyword(RawKeyword::Default)),
        12 if ident == br"de\u{66}ault" => Some(RawToken::Keyword(RawKeyword::Default)),
        12 if ident == br"def\u0061ult" => Some(RawToken::Keyword(RawKeyword::Default)),
        12 if ident == br"def\u{61}ult" => Some(RawToken::Keyword(RawKeyword::Default)),
        12 if ident == br"defa\u0075lt" => Some(RawToken::Keyword(RawKeyword::Default)),
        12 if ident == br"defa\u{75}lt" => Some(RawToken::Keyword(RawKeyword::Default)),
        12 if ident == br"defau\u006ct" => Some(RawToken::Keyword(RawKeyword::Default)),
        12 if ident == br"defau\u{6c}t" => Some(RawToken::Keyword(RawKeyword::Default)),
        12 if ident == br"defaul\u0074" => Some(RawToken::Keyword(RawKeyword::Default)),
        12 if ident == br"defaul\u{74}" => Some(RawToken::Keyword(RawKeyword::Default)),
        12 if ident == br"\u0066inally" => Some(RawToken::Keyword(RawKeyword::Finally)),
        12 if ident == br"\u{66}inally" => Some(RawToken::Keyword(RawKeyword::Finally)),
        12 if ident == br"f\u0069nally" => Some(RawToken::Keyword(RawKeyword::Finally)),
        12 if ident == br"f\u{69}nally" => Some(RawToken::Keyword(RawKeyword::Finally)),
        12 if ident == br"fi\u006eally" => Some(RawToken::Keyword(RawKeyword::Finally)),
        12 if ident == br"fi\u{6e}ally" => Some(RawToken::Keyword(RawKeyword::Finally)),
        12 if ident == br"fin\u0061lly" => Some(RawToken::Keyword(RawKeyword::Finally)),
        12 if ident == br"fin\u{61}lly" => Some(RawToken::Keyword(RawKeyword::Finally)),
        12 if ident == br"fina\u006cly" => Some(RawToken::Keyword(RawKeyword::Finally)),
        12 if ident == br"fina\u{6c}ly" => Some(RawToken::Keyword(RawKeyword::Finally)),
        12 if ident == br"final\u006cy" => Some(RawToken::Keyword(RawKeyword::Finally)),
        12 if ident == br"final\u{6c}y" => Some(RawToken::Keyword(RawKeyword::Finally)),
        12 if ident == br"finall\u0079" => Some(RawToken::Keyword(RawKeyword::Finally)),
        12 if ident == br"finall\u{79}" => Some(RawToken::Keyword(RawKeyword::Finally)),
        12 if ident == br"\u0070ackage" => Some(RawToken::Keyword(RawKeyword::Package)),
        12 if ident == br"\u{70}ackage" => Some(RawToken::Keyword(RawKeyword::Package)),
        12 if ident == br"p\u0061ckage" => Some(RawToken::Keyword(RawKeyword::Package)),
        12 if ident == br"p\u{61}ckage" => Some(RawToken::Keyword(RawKeyword::Package)),
        12 if ident == br"pa\u0063kage" => Some(RawToken::Keyword(RawKeyword::Package)),
        12 if ident == br"pa\u{63}kage" => Some(RawToken::Keyword(RawKeyword::Package)),
        12 if ident == br"pac\u006bage" => Some(RawToken::Keyword(RawKeyword::Package)),
        12 if ident == br"pac\u{6b}age" => Some(RawToken::Keyword(RawKeyword::Package)),
        12 if ident == br"pack\u0061ge" => Some(RawToken::Keyword(RawKeyword::Package)),
        12 if ident == br"pack\u{61}ge" => Some(RawToken::Keyword(RawKeyword::Package)),
        12 if ident == br"packa\u0067e" => Some(RawToken::Keyword(RawKeyword::Package)),
        12 if ident == br"packa\u{67}e" => Some(RawToken::Keyword(RawKeyword::Package)),
        12 if ident == br"packag\u0065" => Some(RawToken::Keyword(RawKeyword::Package)),
        12 if ident == br"packag\u{65}" => Some(RawToken::Keyword(RawKeyword::Package)),
        12 if ident == br"\u0070rivate" => Some(RawToken::Keyword(RawKeyword::Private)),
        12 if ident == br"\u{70}rivate" => Some(RawToken::Keyword(RawKeyword::Private)),
        12 if ident == br"p\u0072ivate" => Some(RawToken::Keyword(RawKeyword::Private)),
        12 if ident == br"p\u{72}ivate" => Some(RawToken::Keyword(RawKeyword::Private)),
        12 if ident == br"pr\u0069vate" => Some(RawToken::Keyword(RawKeyword::Private)),
        12 if ident == br"pr\u{69}vate" => Some(RawToken::Keyword(RawKeyword::Private)),
        12 if ident == br"pri\u0076ate" => Some(RawToken::Keyword(RawKeyword::Private)),
        12 if ident == br"pri\u{76}ate" => Some(RawToken::Keyword(RawKeyword::Private)),
        12 if ident == br"priv\u0061te" => Some(RawToken::Keyword(RawKeyword::Private)),
        12 if ident == br"priv\u{61}te" => Some(RawToken::Keyword(RawKeyword::Private)),
        12 if ident == br"priva\u0074e" => Some(RawToken::Keyword(RawKeyword::Private)),
        12 if ident == br"priva\u{74}e" => Some(RawToken::Keyword(RawKeyword::Private)),
        12 if ident == br"privat\u0065" => Some(RawToken::Keyword(RawKeyword::Private)),
        12 if ident == br"privat\u{65}" => Some(RawToken::Keyword(RawKeyword::Private)),
        13 if ident == br"\u0063ontinue" => Some(RawToken::Keyword(RawKeyword::Continue)),
        13 if ident == br"\u{63}ontinue" => Some(RawToken::Keyword(RawKeyword::Continue)),
        13 if ident == br"c\u006fntinue" => Some(RawToken::Keyword(RawKeyword::Continue)),
        13 if ident == br"c\u{6f}ntinue" => Some(RawToken::Keyword(RawKeyword::Continue)),
        13 if ident == br"co\u006etinue" => Some(RawToken::Keyword(RawKeyword::Continue)),
        13 if ident == br"co\u{6e}tinue" => Some(RawToken::Keyword(RawKeyword::Continue)),
        13 if ident == br"con\u0074inue" => Some(RawToken::Keyword(RawKeyword::Continue)),
        13 if ident == br"con\u{74}inue" => Some(RawToken::Keyword(RawKeyword::Continue)),
        13 if ident == br"cont\u0069nue" => Some(RawToken::Keyword(RawKeyword::Continue)),
        13 if ident == br"cont\u{69}nue" => Some(RawToken::Keyword(RawKeyword::Continue)),
        13 if ident == br"conti\u006eue" => Some(RawToken::Keyword(RawKeyword::Continue)),
        13 if ident == br"conti\u{6e}ue" => Some(RawToken::Keyword(RawKeyword::Continue)),
        13 if ident == br"contin\u0075e" => Some(RawToken::Keyword(RawKeyword::Continue)),
        13 if ident == br"contin\u{75}e" => Some(RawToken::Keyword(RawKeyword::Continue)),
        13 if ident == br"continu\u0065" => Some(RawToken::Keyword(RawKeyword::Continue)),
        13 if ident == br"continu\u{65}" => Some(RawToken::Keyword(RawKeyword::Continue)),
        13 if ident == br"\u0064ebugger" => Some(RawToken::Keyword(RawKeyword::Debugger)),
        13 if ident == br"\u{64}ebugger" => Some(RawToken::Keyword(RawKeyword::Debugger)),
        13 if ident == br"d\u0065bugger" => Some(RawToken::Keyword(RawKeyword::Debugger)),
        13 if ident == br"d\u{65}bugger" => Some(RawToken::Keyword(RawKeyword::Debugger)),
        13 if ident == br"de\u0062ugger" => Some(RawToken::Keyword(RawKeyword::Debugger)),
        13 if ident == br"de\u{62}ugger" => Some(RawToken::Keyword(RawKeyword::Debugger)),
        13 if ident == br"deb\u0075gger" => Some(RawToken::Keyword(RawKeyword::Debugger)),
        13 if ident == br"deb\u{75}gger" => Some(RawToken::Keyword(RawKeyword::Debugger)),
        13 if ident == br"debu\u0067ger" => Some(RawToken::Keyword(RawKeyword::Debugger)),
        13 if ident == br"debu\u{67}ger" => Some(RawToken::Keyword(RawKeyword::Debugger)),
        13 if ident == br"debug\u0067er" => Some(RawToken::Keyword(RawKeyword::Debugger)),
        13 if ident == br"debug\u{67}er" => Some(RawToken::Keyword(RawKeyword::Debugger)),
        13 if ident == br"debugg\u0065r" => Some(RawToken::Keyword(RawKeyword::Debugger)),
        13 if ident == br"debugg\u{65}r" => Some(RawToken::Keyword(RawKeyword::Debugger)),
        13 if ident == br"debugge\u0072" => Some(RawToken::Keyword(RawKeyword::Debugger)),
        13 if ident == br"debugge\u{72}" => Some(RawToken::Keyword(RawKeyword::Debugger)),
        13 if ident == br"\u0066unction" => Some(RawToken::Keyword(RawKeyword::Function)),
        13 if ident == br"\u{66}unction" => Some(RawToken::Keyword(RawKeyword::Function)),
        13 if ident == br"f\u0075nction" => Some(RawToken::Keyword(RawKeyword::Function)),
        13 if ident == br"f\u{75}nction" => Some(RawToken::Keyword(RawKeyword::Function)),
        13 if ident == br"fu\u006ection" => Some(RawToken::Keyword(RawKeyword::Function)),
        13 if ident == br"fu\u{6e}ction" => Some(RawToken::Keyword(RawKeyword::Function)),
        13 if ident == br"fun\u0063tion" => Some(RawToken::Keyword(RawKeyword::Function)),
        13 if ident == br"fun\u{63}tion" => Some(RawToken::Keyword(RawKeyword::Function)),
        13 if ident == br"func\u0074ion" => Some(RawToken::Keyword(RawKeyword::Function)),
        13 if ident == br"func\u{74}ion" => Some(RawToken::Keyword(RawKeyword::Function)),
        13 if ident == br"funct\u0069on" => Some(RawToken::Keyword(RawKeyword::Function)),
        13 if ident == br"funct\u{69}on" => Some(RawToken::Keyword(RawKeyword::Function)),
        13 if ident == br"functi\u006fn" => Some(RawToken::Keyword(RawKeyword::Function)),
        13 if ident == br"functi\u{6f}n" => Some(RawToken::Keyword(RawKeyword::Function)),
        13 if ident == br"functio\u006e" => Some(RawToken::Keyword(RawKeyword::Function)),
        13 if ident == br"functio\u{6e}" => Some(RawToken::Keyword(RawKeyword::Function)),
        14 if ident == br"\u0069nterface" => Some(RawToken::Keyword(RawKeyword::Interface)),
        14 if ident == br"\u{69}nterface" => Some(RawToken::Keyword(RawKeyword::Interface)),
        14 if ident == br"i\u006eterface" => Some(RawToken::Keyword(RawKeyword::Interface)),
        14 if ident == br"i\u{6e}terface" => Some(RawToken::Keyword(RawKeyword::Interface)),
        14 if ident == br"in\u0074erface" => Some(RawToken::Keyword(RawKeyword::Interface)),
        14 if ident == br"in\u{74}erface" => Some(RawToken::Keyword(RawKeyword::Interface)),
        14 if ident == br"int\u0065rface" => Some(RawToken::Keyword(RawKeyword::Interface)),
        14 if ident == br"int\u{65}rface" => Some(RawToken::Keyword(RawKeyword::Interface)),
        14 if ident == br"inte\u0072face" => Some(RawToken::Keyword(RawKeyword::Interface)),
        14 if ident == br"inte\u{72}face" => Some(RawToken::Keyword(RawKeyword::Interface)),
        14 if ident == br"inter\u0066ace" => Some(RawToken::Keyword(RawKeyword::Interface)),
        14 if ident == br"inter\u{66}ace" => Some(RawToken::Keyword(RawKeyword::Interface)),
        14 if ident == br"interf\u0061ce" => Some(RawToken::Keyword(RawKeyword::Interface)),
        14 if ident == br"interf\u{61}ce" => Some(RawToken::Keyword(RawKeyword::Interface)),
        14 if ident == br"interfa\u0063e" => Some(RawToken::Keyword(RawKeyword::Interface)),
        14 if ident == br"interfa\u{63}e" => Some(RawToken::Keyword(RawKeyword::Interface)),
        14 if ident == br"interfac\u0065" => Some(RawToken::Keyword(RawKeyword::Interface)),
        14 if ident == br"interfac\u{65}" => Some(RawToken::Keyword(RawKeyword::Interface)),
        14 if ident == br"\u0070rotected" => Some(RawToken::Keyword(RawKeyword::Protected)),
        14 if ident == br"\u{70}rotected" => Some(RawToken::Keyword(RawKeyword::Protected)),
        14 if ident == br"p\u0072otected" => Some(RawToken::Keyword(RawKeyword::Protected)),
        14 if ident == br"p\u{72}otected" => Some(RawToken::Keyword(RawKeyword::Protected)),
        14 if ident == br"pr\u006ftected" => Some(RawToken::Keyword(RawKeyword::Protected)),
        14 if ident == br"pr\u{6f}tected" => Some(RawToken::Keyword(RawKeyword::Protected)),
        14 if ident == br"pro\u0074ected" => Some(RawToken::Keyword(RawKeyword::Protected)),
        14 if ident == br"pro\u{74}ected" => Some(RawToken::Keyword(RawKeyword::Protected)),
        14 if ident == br"prot\u0065cted" => Some(RawToken::Keyword(RawKeyword::Protected)),
        14 if ident == br"prot\u{65}cted" => Some(RawToken::Keyword(RawKeyword::Protected)),
        14 if ident == br"prote\u0063ted" => Some(RawToken::Keyword(RawKeyword::Protected)),
        14 if ident == br"prote\u{63}ted" => Some(RawToken::Keyword(RawKeyword::Protected)),
        14 if ident == br"protec\u0074ed" => Some(RawToken::Keyword(RawKeyword::Protected)),
        14 if ident == br"protec\u{74}ed" => Some(RawToken::Keyword(RawKeyword::Protected)),
        14 if ident == br"protect\u0065d" => Some(RawToken::Keyword(RawKeyword::Protected)),
        14 if ident == br"protect\u{65}d" => Some(RawToken::Keyword(RawKeyword::Protected)),
        14 if ident == br"protecte\u0064" => Some(RawToken::Keyword(RawKeyword::Protected)),
        14 if ident == br"protecte\u{64}" => Some(RawToken::Keyword(RawKeyword::Protected)),
        15 if ident == br"\u0069nstanceof" => Some(RawToken::Keyword(RawKeyword::InstanceOf)),
        15 if ident == br"\u{69}nstanceof" => Some(RawToken::Keyword(RawKeyword::InstanceOf)),
        15 if ident == br"i\u006estanceof" => Some(RawToken::Keyword(RawKeyword::InstanceOf)),
        15 if ident == br"i\u{6e}stanceof" => Some(RawToken::Keyword(RawKeyword::InstanceOf)),
        15 if ident == br"in\u0073tanceof" => Some(RawToken::Keyword(RawKeyword::InstanceOf)),
        15 if ident == br"in\u{73}tanceof" => Some(RawToken::Keyword(RawKeyword::InstanceOf)),
        15 if ident == br"ins\u0074anceof" => Some(RawToken::Keyword(RawKeyword::InstanceOf)),
        15 if ident == br"ins\u{74}anceof" => Some(RawToken::Keyword(RawKeyword::InstanceOf)),
        15 if ident == br"inst\u0061nceof" => Some(RawToken::Keyword(RawKeyword::InstanceOf)),
        15 if ident == br"inst\u{61}nceof" => Some(RawToken::Keyword(RawKeyword::InstanceOf)),
        15 if ident == br"insta\u006eceof" => Some(RawToken::Keyword(RawKeyword::InstanceOf)),
        15 if ident == br"insta\u{6e}ceof" => Some(RawToken::Keyword(RawKeyword::InstanceOf)),
        15 if ident == br"instan\u0063eof" => Some(RawToken::Keyword(RawKeyword::InstanceOf)),
        15 if ident == br"instan\u{63}eof" => Some(RawToken::Keyword(RawKeyword::InstanceOf)),
        15 if ident == br"instanc\u0065of" => Some(RawToken::Keyword(RawKeyword::InstanceOf)),
        15 if ident == br"instanc\u{65}of" => Some(RawToken::Keyword(RawKeyword::InstanceOf)),
        15 if ident == br"instance\u006ff" => Some(RawToken::Keyword(RawKeyword::InstanceOf)),
        15 if ident == br"instance\u{6f}f" => Some(RawToken::Keyword(RawKeyword::InstanceOf)),
        15 if ident == br"instanceo\u0066" => Some(RawToken::Keyword(RawKeyword::InstanceOf)),
        15 if ident == br"instanceo\u{66}" => Some(RawToken::Keyword(RawKeyword::InstanceOf)),
        15 if ident == br"\u0069mplements" => Some(RawToken::Keyword(RawKeyword::Implements)),
        15 if ident == br"\u{69}mplements" => Some(RawToken::Keyword(RawKeyword::Implements)),
        15 if ident == br"i\u006dplements" => Some(RawToken::Keyword(RawKeyword::Implements)),
        15 if ident == br"i\u{6d}plements" => Some(RawToken::Keyword(RawKeyword::Implements)),
        15 if ident == br"im\u0070lements" => Some(RawToken::Keyword(RawKeyword::Implements)),
        15 if ident == br"im\u{70}lements" => Some(RawToken::Keyword(RawKeyword::Implements)),
        15 if ident == br"imp\u006cements" => Some(RawToken::Keyword(RawKeyword::Implements)),
        15 if ident == br"imp\u{6c}ements" => Some(RawToken::Keyword(RawKeyword::Implements)),
        15 if ident == br"impl\u0065ments" => Some(RawToken::Keyword(RawKeyword::Implements)),
        15 if ident == br"impl\u{65}ments" => Some(RawToken::Keyword(RawKeyword::Implements)),
        15 if ident == br"imple\u006dents" => Some(RawToken::Keyword(RawKeyword::Implements)),
        15 if ident == br"imple\u{6d}ents" => Some(RawToken::Keyword(RawKeyword::Implements)),
        15 if ident == br"implem\u0065nts" => Some(RawToken::Keyword(RawKeyword::Implements)),
        15 if ident == br"implem\u{65}nts" => Some(RawToken::Keyword(RawKeyword::Implements)),
        15 if ident == br"impleme\u006ets" => Some(RawToken::Keyword(RawKeyword::Implements)),
        15 if ident == br"impleme\u{6e}ts" => Some(RawToken::Keyword(RawKeyword::Implements)),
        15 if ident == br"implemen\u0074s" => Some(RawToken::Keyword(RawKeyword::Implements)),
        15 if ident == br"implemen\u{74}s" => Some(RawToken::Keyword(RawKeyword::Implements)),
        15 if ident == br"implement\u0073" => Some(RawToken::Keyword(RawKeyword::Implements)),
        15 if ident == br"implement\u{73}" => Some(RawToken::Keyword(RawKeyword::Implements)),
        18 if ident == br"\u0066\u006f\u0072" => Some(RawToken::Keyword(RawKeyword::For)),
        18 if ident == br"\u{66}\u{6f}\u{72}" => Some(RawToken::Keyword(RawKeyword::For)),
        18 if ident == br"\u006e\u0065\u0077" => Some(RawToken::Keyword(RawKeyword::New)),
        18 if ident == br"\u{6e}\u{65}\u{77}" => Some(RawToken::Keyword(RawKeyword::New)),
        18 if ident == br"\u0074\u0072\u0079" => Some(RawToken::Keyword(RawKeyword::Try)),
        18 if ident == br"\u{74}\u{72}\u{79}" => Some(RawToken::Keyword(RawKeyword::Try)),
        18 if ident == br"\u0076\u0061\u0072" => Some(RawToken::Keyword(RawKeyword::Var)),
        18 if ident == br"\u{76}\u{61}\u{72}" => Some(RawToken::Keyword(RawKeyword::Var)),
        18 if ident == br"\u006c\u0065\u0074" => Some(RawToken::Keyword(RawKeyword::Let)),
        18 if ident == br"\u{6c}\u{65}\u{74}" => Some(RawToken::Keyword(RawKeyword::Let)),
        24 if ident == br"\u0063\u0061\u0073\u0065" => Some(RawToken::Keyword(RawKeyword::Case)),
        24 if ident == br"\u{63}\u{61}\u{73}\u{65}" => Some(RawToken::Keyword(RawKeyword::Case)),
        24 if ident == br"\u0074\u0068\u0069\u0073" => Some(RawToken::Keyword(RawKeyword::This)),
        24 if ident == br"\u{74}\u{68}\u{69}\u{73}" => Some(RawToken::Keyword(RawKeyword::This)),
        24 if ident == br"\u0076\u006f\u0069\u0064" => Some(RawToken::Keyword(RawKeyword::Void)),
        24 if ident == br"\u{76}\u{6f}\u{69}\u{64}" => Some(RawToken::Keyword(RawKeyword::Void)),
        24 if ident == br"\u0077\u0069\u0074\u0068" => Some(RawToken::Keyword(RawKeyword::With)),
        24 if ident == br"\u{77}\u{69}\u{74}\u{68}" => Some(RawToken::Keyword(RawKeyword::With)),
        24 if ident == br"\u0065\u006e\u0075\u006d" => Some(RawToken::Keyword(RawKeyword::Enum)),
        24 if ident == br"\u{65}\u{6e}\u{75}\u{6d}" => Some(RawToken::Keyword(RawKeyword::Enum)),
        24 if ident == br"\u0065\u006c\u0073\u0065" => Some(RawToken::Keyword(RawKeyword::Else)),
        24 if ident == br"\u{65}\u{6c}\u{73}\u{65}" => Some(RawToken::Keyword(RawKeyword::Else)),
        24 if ident == br"\u0074\u0072\u0075\u0065" => Some(RawToken::Boolean(true)),
        24 if ident == br"\u{74}\u{72}\u{75}\u{65}" => Some(RawToken::Boolean(true)),
        24 if ident == br"\u006e\u0075\u006c\u006c" => Some(RawToken::Null),
        24 if ident == br"\u{6e}\u{75}\u{6c}\u{6c}" => Some(RawToken::Null),
        30 if ident == br"\u0061\u0077\u0061\u0069\u0074" => {
            Some(RawToken::Keyword(RawKeyword::Await))
        }
        30 if ident == br"\u{61}\u{77}\u{61}\u{69}\u{74}" => {
            Some(RawToken::Keyword(RawKeyword::Await))
        }
        30 if ident == br"\u0062\u0072\u0065\u0061\u006b" => {
            Some(RawToken::Keyword(RawKeyword::Break))
        }
        30 if ident == br"\u{62}\u{72}\u{65}\u{61}\u{6b}" => {
            Some(RawToken::Keyword(RawKeyword::Break))
        }
        30 if ident == br"\u0063\u0061\u0074\u0063\u0068" => {
            Some(RawToken::Keyword(RawKeyword::Catch))
        }
        30 if ident == br"\u{63}\u{61}\u{74}\u{63}\u{68}" => {
            Some(RawToken::Keyword(RawKeyword::Catch))
        }
        30 if ident == br"\u0063\u006c\u0061\u0073\u0073" => {
            Some(RawToken::Keyword(RawKeyword::Class))
        }
        30 if ident == br"\u{63}\u{6c}\u{61}\u{73}\u{73}" => {
            Some(RawToken::Keyword(RawKeyword::Class))
        }
        30 if ident == br"\u0063\u006f\u006e\u0073\u0074" => {
            Some(RawToken::Keyword(RawKeyword::Const))
        }
        30 if ident == br"\u{63}\u{6f}\u{6e}\u{73}\u{74}" => {
            Some(RawToken::Keyword(RawKeyword::Const))
        }
        30 if ident == br"\u0074\u0068\u0072\u006f\u0077" => {
            Some(RawToken::Keyword(RawKeyword::Throw))
        }
        30 if ident == br"\u{74}\u{68}\u{72}\u{6f}\u{77}" => {
            Some(RawToken::Keyword(RawKeyword::Throw))
        }
        30 if ident == br"\u0077\u0068\u0069\u006c\u0065" => {
            Some(RawToken::Keyword(RawKeyword::While))
        }
        30 if ident == br"\u{77}\u{68}\u{69}\u{6c}\u{65}" => {
            Some(RawToken::Keyword(RawKeyword::While))
        }
        30 if ident == br"\u0073\u0075\u0070\u0065\u0072" => {
            Some(RawToken::Keyword(RawKeyword::Super))
        }
        30 if ident == br"\u{73}\u{75}\u{70}\u{65}\u{72}" => {
            Some(RawToken::Keyword(RawKeyword::Super))
        }
        30 if ident == br"\u0079\u0069\u0065\u006c\u0064" => {
            Some(RawToken::Keyword(RawKeyword::Yield))
        }
        30 if ident == br"\u{79}\u{69}\u{65}\u{6c}\u{64}" => {
            Some(RawToken::Keyword(RawKeyword::Yield))
        }
        30 if ident == br"\u0066\u0061\u006c\u0073\u0065" => Some(RawToken::Boolean(false)),
        30 if ident == br"\u{66}\u{61}\u{6c}\u{73}\u{65}" => Some(RawToken::Boolean(false)),
        36 if &ident[0..32] == br"\u0064\u0065\u006c\u0065\u0074\u" && &ident[32..] == br"0065" => {
            Some(RawToken::Keyword(RawKeyword::Delete))
        }
        36 if &ident[0..32] == br"\u{64}\u{65}\u{6c}\u{65}\u{74}\u" && &ident[32..] == br"{65}" => {
            Some(RawToken::Keyword(RawKeyword::Delete))
        }
        36 if &ident[0..32] == br"\u0072\u0065\u0074\u0075\u0072\u" && &ident[32..] == br"006e" => {
            Some(RawToken::Keyword(RawKeyword::Return))
        }
        36 if &ident[0..32] == br"\u{72}\u{65}\u{74}\u{75}\u{72}\u" && &ident[32..] == br"{6e}" => {
            Some(RawToken::Keyword(RawKeyword::Return))
        }
        36 if &ident[0..32] == br"\u0073\u0077\u0069\u0074\u0063\u" && &ident[32..] == br"0068" => {
            Some(RawToken::Keyword(RawKeyword::Switch))
        }
        36 if &ident[0..32] == br"\u{73}\u{77}\u{69}\u{74}\u{63}\u" && &ident[32..] == br"{68}" => {
            Some(RawToken::Keyword(RawKeyword::Switch))
        }
        36 if &ident[0..32] == br"\u0074\u0079\u0070\u0065\u006f\u" && &ident[32..] == br"0066" => {
            Some(RawToken::Keyword(RawKeyword::TypeOf))
        }
        36 if &ident[0..32] == br"\u{74}\u{79}\u{70}\u{65}\u{6f}\u" && &ident[32..] == br"{66}" => {
            Some(RawToken::Keyword(RawKeyword::TypeOf))
        }
        36 if &ident[0..32] == br"\u0065\u0078\u0070\u006f\u0072\u" && &ident[32..] == br"0074" => {
            Some(RawToken::Keyword(RawKeyword::Export))
        }
        36 if &ident[0..32] == br"\u{65}\u{78}\u{70}\u{6f}\u{72}\u" && &ident[32..] == br"{74}" => {
            Some(RawToken::Keyword(RawKeyword::Export))
        }
        36 if &ident[0..32] == br"\u0069\u006d\u0070\u006f\u0072\u" && &ident[32..] == br"0074" => {
            Some(RawToken::Keyword(RawKeyword::Import))
        }
        36 if &ident[0..32] == br"\u{69}\u{6d}\u{70}\u{6f}\u{72}\u" && &ident[32..] == br"{74}" => {
            Some(RawToken::Keyword(RawKeyword::Import))
        }
        36 if &ident[0..32] == br"\u0073\u0074\u0061\u0074\u0069\u" && &ident[32..] == br"0063" => {
            Some(RawToken::Keyword(RawKeyword::Static))
        }
        36 if &ident[0..32] == br"\u{73}\u{74}\u{61}\u{74}\u{69}\u" && &ident[32..] == br"{63}" => {
            Some(RawToken::Keyword(RawKeyword::Static))
        }
        36 if &ident[0..32] == br"\u0070\u0075\u0062\u006c\u0069\u" && &ident[32..] == br"0063" => {
            Some(RawToken::Keyword(RawKeyword::Public))
        }
        36 if &ident[0..32] == br"\u{70}\u{75}\u{62}\u{6c}\u{69}\u" && &ident[32..] == br"{63}" => {
            Some(RawToken::Keyword(RawKeyword::Public))
        }
        42 if &ident[0..32] == br"\u0065\u0078\u0074\u0065\u006e\u"
            && &ident[32..] == br"0064\u0073" =>
        {
            Some(RawToken::Keyword(RawKeyword::Extends))
        }
        42 if &ident[0..32] == br"\u{65}\u{78}\u{74}\u{65}\u{6e}\u"
            && &ident[32..] == br"{64}\u{73}" =>
        {
            Some(RawToken::Keyword(RawKeyword::Extends))
        }
        42 if &ident[0..32] == br"\u0064\u0065\u0066\u0061\u0075\u"
            && &ident[32..] == br"006c\u0074" =>
        {
            Some(RawToken::Keyword(RawKeyword::Default))
        }
        42 if &ident[0..32] == br"\u{64}\u{65}\u{66}\u{61}\u{75}\u"
            && &ident[32..] == br"{6c}\u{74}" =>
        {
            Some(RawToken::Keyword(RawKeyword::Default))
        }
        42 if &ident[0..32] == br"\u0066\u0069\u006e\u0061\u006c\u"
            && &ident[32..] == br"006c\u0079" =>
        {
            Some(RawToken::Keyword(RawKeyword::Finally))
        }
        42 if &ident[0..32] == br"\u{66}\u{69}\u{6e}\u{61}\u{6c}\u"
            && &ident[32..] == br"{6c}\u{79}" =>
        {
            Some(RawToken::Keyword(RawKeyword::Finally))
        }
        42 if &ident[0..32] == br"\u0070\u0061\u0063\u006b\u0061\u"
            && &ident[32..] == br"0067\u0065" =>
        {
            Some(RawToken::Keyword(RawKeyword::Package))
        }
        42 if &ident[0..32] == br"\u{70}\u{61}\u{63}\u{6b}\u{61}\u"
            && &ident[32..] == br"{67}\u{65}" =>
        {
            Some(RawToken::Keyword(RawKeyword::Package))
        }
        42 if &ident[0..32] == br"\u0070\u0072\u0069\u0076\u0061\u"
            && &ident[32..] == br"0074\u0065" =>
        {
            Some(RawToken::Keyword(RawKeyword::Private))
        }
        42 if &ident[0..32] == br"\u{70}\u{72}\u{69}\u{76}\u{61}\u"
            && &ident[32..] == br"{74}\u{65}" =>
        {
            Some(RawToken::Keyword(RawKeyword::Private))
        }
        48 if &ident[0..32] == br"\u0063\u006f\u006e\u0074\u0069\u"
            && &ident[32..] == br"006e\u0075\u0065" =>
        {
            Some(RawToken::Keyword(RawKeyword::Continue))
        }
        48 if &ident[0..32] == br"\u{63}\u{6f}\u{6e}\u{74}\u{69}\u"
            && &ident[32..] == br"{6e}\u{75}\u{65}" =>
        {
            Some(RawToken::Keyword(RawKeyword::Continue))
        }
        48 if &ident[0..32] == br"\u0064\u0065\u0062\u0075\u0067\u"
            && &ident[32..] == br"0067\u0065\u0072" =>
        {
            Some(RawToken::Keyword(RawKeyword::Debugger))
        }
        48 if &ident[0..32] == br"\u{64}\u{65}\u{62}\u{75}\u{67}\u"
            && &ident[32..] == br"{67}\u{65}\u{72}" =>
        {
            Some(RawToken::Keyword(RawKeyword::Debugger))
        }
        48 if &ident[0..32] == br"\u0066\u0075\u006e\u0063\u0074\u"
            && &ident[32..] == br"0069\u006f\u006e" =>
        {
            Some(RawToken::Keyword(RawKeyword::Function))
        }
        48 if &ident[0..32] == br"\u{66}\u{75}\u{6e}\u{63}\u{74}\u"
            && &ident[32..] == br"{69}\u{6f}\u{6e}" =>
        {
            Some(RawToken::Keyword(RawKeyword::Function))
        }
        54 if &ident[0..32] == br"\u0069\u006e\u0074\u0065\u0072\u"
            && &ident[32..] == br"0066\u0061\u0063\u0065" =>
        {
            Some(RawToken::Keyword(RawKeyword::Interface))
        }
        54 if &ident[0..32] == br"\u{69}\u{6e}\u{74}\u{65}\u{72}\u"
            && &ident[32..] == br"{66}\u{61}\u{63}\u{65}" =>
        {
            Some(RawToken::Keyword(RawKeyword::Interface))
        }
        54 if &ident[0..32] == br"\u0070\u0072\u006f\u0074\u0065\u"
            && &ident[32..] == br"0063\u0074\u0065\u0064" =>
        {
            Some(RawToken::Keyword(RawKeyword::Protected))
        }
        54 if &ident[0..32] == br"\u{70}\u{72}\u{6f}\u{74}\u{65}\u"
            && &ident[32..] == br"{63}\u{74}\u{65}\u{64}" =>
        {
            Some(RawToken::Keyword(RawKeyword::Protected))
        }
        60 if &ident[0..32] == br"\u0069\u006e\u0073\u0074\u0061\u"
            && &ident[32..] == br"006e\u0063\u0065\u006f\u0066" =>
        {
            Some(RawToken::Keyword(RawKeyword::InstanceOf))
        }
        60 if &ident[0..32] == br"\u{69}\u{6e}\u{73}\u{74}\u{61}\u"
            && &ident[32..] == br"{6e}\u{63}\u{65}\u{6f}\u{66}" =>
        {
            Some(RawToken::Keyword(RawKeyword::InstanceOf))
        }
        60 if &ident[0..32] == br"\u0069\u006d\u0070\u006c\u0065\u"
            && &ident[32..] == br"006d\u0065\u006e\u0074\u0073" =>
        {
            Some(RawToken::Keyword(RawKeyword::Implements))
        }
        60 if &ident[0..32] == br"\u{69}\u{6d}\u{70}\u{6c}\u{65}\u"
            && &ident[32..] == br"{6d}\u{65}\u{6e}\u{74}\u{73}" =>
        {
            Some(RawToken::Keyword(RawKeyword::Implements))
        }
        _ => None,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn check_unicode_escaped_keyword() {
        let escaped_keywords = vec![
            (Some(RawToken::Keyword(RawKeyword::Yield)), r"\u0079ield"),
            (
                Some(RawToken::Keyword(RawKeyword::Private)),
                r"privat\u0065",
            ),
            (
                Some(RawToken::Keyword(RawKeyword::Static)),
                r"\u0073\u0074\u0061\u0074\u0069\u0063",
            ),
            (Some(RawToken::Keyword(RawKeyword::Yield)), r"\u{79}ield"),
            (
                Some(RawToken::Keyword(RawKeyword::Private)),
                r"privat\u{65}",
            ),
            (
                Some(RawToken::Keyword(RawKeyword::Static)),
                r"\u{73}\u{74}\u{61}\u{74}\u{69}\u{63}",
            ),
            (None, r"yield"),
        ];
        for (target, test) in escaped_keywords {
            assert_eq!(target, check_complicated_keyword(test.as_bytes()))
        }
    }
}
