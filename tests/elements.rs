extern crate xmlparser as xml;

#[macro_use] mod token;
use token::*;

test!(element_01, "<a/>",
    Token::ElementStart("", "a", 0..2),
    Token::ElementEnd(ElementEnd::Empty, 2..4)
);

test!(element_02, "<a></a>",
    Token::ElementStart("", "a", 0..2),
    Token::ElementEnd(ElementEnd::Open, 2..3),
    Token::ElementEnd(ElementEnd::Close("", "a"), 3..7)
);

test!(element_03, "  \t  <a/>   \n ",
    Token::ElementStart("", "a", 5..7),
    Token::ElementEnd(ElementEnd::Empty, 7..9)
);

test!(element_04, "  \t  <b><a/></b>   \n ",
    Token::ElementStart("", "b", 5..7),
    Token::ElementEnd(ElementEnd::Open, 7..8),
    Token::ElementStart("", "a", 8..10),
    Token::ElementEnd(ElementEnd::Empty, 10..12),
    Token::ElementEnd(ElementEnd::Close("", "b"), 12..16)
);

test!(element_06, "<俄语 լեզու=\"ռուսերեն\">данные</俄语>",
    Token::ElementStart("", "俄语", 0..7),
    Token::Attribute("", "լեզու", "ռուսերեն", 8..37),
    Token::ElementEnd(ElementEnd::Open, 37..38),
    Token::Text("данные", 38..50),
    Token::ElementEnd(ElementEnd::Close("", "俄语"), 50..59)
);

test!(element_07, "<svg:circle></svg:circle>",
    Token::ElementStart("svg", "circle", 0..11),
    Token::ElementEnd(ElementEnd::Open, 11..12),
    Token::ElementEnd(ElementEnd::Close("svg", "circle"), 12..25)
);

test!(element_08, "<:circle/>",
    Token::ElementStart("", "circle", 0..8),
    Token::ElementEnd(ElementEnd::Empty, 8..10)
);

test!(element_err_01, "<>",
    Token::Error("invalid token 'Element Start' at 1:1 cause invalid name token".to_string())
);

test!(element_err_02, "</",
    Token::Error("unexpected token 'Element Close' at 1:1".to_string())
);

test!(element_err_03, "</a",
    Token::Error("unexpected token 'Element Close' at 1:1".to_string())
);

test!(element_err_04, "<a x='test' /",
    Token::ElementStart("", "a", 0..2),
    Token::Attribute("", "x", "test", 3..11),
    Token::Error("invalid token 'Attribute' at 1:13 cause unexpected end of stream".to_string())
);

test!(element_err_05, "<<",
    Token::Error("invalid token 'Element Start' at 1:1 cause invalid name token".to_string())
);

test!(element_err_06, "< a",
    Token::Error("invalid token 'Element Start' at 1:1 cause invalid name token".to_string())
);

test!(element_err_07, "< ",
    Token::Error("invalid token 'Element Start' at 1:1 cause invalid name token".to_string())
);

test!(element_err_08, "<&#x9;",
    Token::Error("invalid token 'Element Start' at 1:1 cause invalid name token".to_string())
);

test!(element_err_09, "<a></a></a>",
    Token::ElementStart("", "a", 0..2),
    Token::ElementEnd(ElementEnd::Open, 2..3),
    Token::ElementEnd(ElementEnd::Close("", "a"), 3..7),
    Token::Error("unexpected token 'Element Close' at 1:8".to_string())
);

test!(element_err_10, "<a/><a/>",
    Token::ElementStart("", "a", 0..2),
    Token::ElementEnd(ElementEnd::Empty, 2..4),
    Token::Error("unexpected token 'Element Start' at 1:5".to_string())
);

test!(element_err_11, "<a></br/></a>",
    Token::ElementStart("", "a", 0..2),
    Token::ElementEnd(ElementEnd::Open, 2..3),
    Token::Error("invalid token 'Element Close' at 1:4 cause expected '>' not '/' at 1:8".to_string())
);

test!(element_err_12, "<svg:/>",
    Token::Error("invalid token 'Element Start' at 1:1 cause invalid name token".to_string())
);

test!(element_err_13, "\
<root>
</root>
</root>",
    Token::ElementStart("", "root", 0..5),
    Token::ElementEnd(ElementEnd::Open, 5..6),
    Token::Text("\n", 6..7),
    Token::ElementEnd(ElementEnd::Close("", "root"), 7..14),
    Token::Error("unexpected token 'Element Close' at 3:1".to_string())
);


test!(attribute_01, "<a ax=\"test\"/>",
    Token::ElementStart("", "a", 0..2),
    Token::Attribute("", "ax", "test", 3..12),
    Token::ElementEnd(ElementEnd::Empty, 12..14)
);

test!(attribute_02, "<a ax='test'/>",
    Token::ElementStart("", "a", 0..2),
    Token::Attribute("", "ax", "test", 3..12),
    Token::ElementEnd(ElementEnd::Empty, 12..14)
);

test!(attribute_03, "<a b='test1' c=\"test2\"/>",
    Token::ElementStart("", "a", 0..2),
    Token::Attribute("", "b", "test1", 3..12),
    Token::Attribute("", "c", "test2", 13..22),
    Token::ElementEnd(ElementEnd::Empty, 22..24)
);

test!(attribute_04, "<a b='\"test1\"' c=\"'test2'\"/>",
    Token::ElementStart("", "a", 0..2),
    Token::Attribute("", "b", "\"test1\"", 3..14),
    Token::Attribute("", "c", "'test2'", 15..26),
    Token::ElementEnd(ElementEnd::Empty, 26..28)
);

test!(attribute_05, "<c a=\"test1' c='test2\" b='test1\" c=\"test2'/>",
    Token::ElementStart("", "c", 0..2),
    Token::Attribute("", "a", "test1' c='test2", 3..22),
    Token::Attribute("", "b", "test1\" c=\"test2", 23..42),
    Token::ElementEnd(ElementEnd::Empty, 42..44)
);

test!(attribute_06, "<c   a   =    'test1'     />",
    Token::ElementStart("", "c", 0..2),
    Token::Attribute("", "a", "test1", 5..21),
    Token::ElementEnd(ElementEnd::Empty, 26..28)
);

test!(attribute_07, "<c q:a='b'/>",
    Token::ElementStart("", "c", 0..2),
    Token::Attribute("q", "a", "b", 3..10),
    Token::ElementEnd(ElementEnd::Empty, 10..12)
);

test!(attribute_err_01, "<c az=test>",
    Token::ElementStart("", "c", 0..2),
    Token::Error("invalid token 'Attribute' at 1:3 cause expected quote mark not 't' at 1:7".to_string())
);

test!(attribute_err_02, "<c a>",
    Token::ElementStart("", "c", 0..2),
    Token::Error("invalid token 'Attribute' at 1:3 cause expected \'=\' not \'>\' at 1:5".to_string())
);

test!(attribute_err_03, "<c a/>",
    Token::ElementStart("", "c", 0..2),
    Token::Error("invalid token 'Attribute' at 1:3 cause expected '=' not '/' at 1:5".to_string())
);

test!(attribute_err_04, "<c a='b' q/>",
    Token::ElementStart("", "c", 0..2),
    Token::Attribute("", "a", "b", 3..8),
    Token::Error("invalid token 'Attribute' at 1:10 cause expected '=' not '/' at 1:11".to_string())
);

test!(attribute_err_05, "<c a='<'/>",
    Token::ElementStart("", "c", 0..2),
    Token::Error("invalid token 'Attribute' at 1:3 cause attribute value with '<' character is not allowed".to_string())
);
