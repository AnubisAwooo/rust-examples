use anyhow::Result;
use askama::Template;
use core::panic;
use proc_macro::{Delimiter, TokenStream, TokenTree};
use std::collections::{HashMap, VecDeque};

#[derive(Template, Debug)]
#[template(path = "generate.j2", escape = "none")]
pub struct GenerateContext {
    extra_types: Vec<String>,
    name: String,
    fields: Vec<Fd>,
}

/// 描述 struct 的每个 field
#[derive(Debug, Default)]
struct Fd {
    name: String,
    ty: String,
}

#[derive(Debug)]
struct Field {
    name: String,
    ty: ReadResult,
}

#[derive(Debug)]
enum ReadResult {
    Object(Vec<Field>),
    Other(String),
}
impl ReadResult {
    pub fn name(&self) -> String {
        match self {
            ReadResult::Object(_) => panic!("what a struct"),
            ReadResult::Other(v) => v.clone(),
        }
    }
}

impl GenerateContext {
    fn read_integer(input: &TokenTree) -> ReadResult {
        let stream = match input {
            TokenTree::Group(v) if v.delimiter() == Delimiter::Brace => v.stream(),
            _ => panic!("必须花括号"),
        };
        let input = stream.into_iter().collect::<Vec<_>>();

        fn check_type(ty: &TokenTree) {
            if let TokenTree::Literal(v) = ty {
                if v.to_string() == "\"integer\"" {
                    return;
                }
            }
            panic!("type 字段必须是 integer");
        }

        let mut ty = false;
        let mut format = None;

        for tokens in input.split(|t| match t {
            TokenTree::Punct(p) => p.as_char() == ',',
            _ => false,
        }) {
            let ts = tokens
                .split(|v| match v {
                    // 再用 ':' 把 &[TokenTree] 切成 [&[TokenTree], &[TokenTree]]
                    // 它们分别对应名字和类型
                    TokenTree::Punct(p) => p.as_char() == ':',
                    _ => false,
                })
                .collect::<Vec<_>>();
            if let TokenTree::Literal(literal) = &ts[0][0] {
                match literal.to_string().as_str() {
                    "\"type\"" => {
                        check_type(&ts[1][0]);
                        ty = true
                    }
                    "\"format\"" => match ts[1][0].to_string().as_str() {
                        "\"int64\"" => format = Some("i64".into()),
                        "\"int32\"" => format = Some("i32".into()),
                        v => panic!("format: {} is not support", v),
                    },
                    _ => {}
                }
            }
        }
        if !ty {
            panic!("必须包含 type 字段");
        }
        if format.is_none() {
            panic!("必须包含 format 字段");
        }
        ReadResult::Other(format.unwrap())
    }

    fn read_boolean(input: &TokenTree) -> ReadResult {
        let stream = match input {
            TokenTree::Group(v) if v.delimiter() == Delimiter::Brace => v.stream(),
            _ => panic!("必须花括号"),
        };
        let input = stream.into_iter().collect::<Vec<_>>();

        fn check_type(ty: &TokenTree) {
            if let TokenTree::Literal(v) = ty {
                if v.to_string() == "\"boolean\"" {
                    return;
                }
            }
            panic!("type 字段必须是 boolean");
        }

        let mut ty = false;

        for tokens in input.split(|t| match t {
            TokenTree::Punct(p) => p.as_char() == ',',
            _ => false,
        }) {
            let ts = tokens
                .split(|v| match v {
                    // 再用 ':' 把 &[TokenTree] 切成 [&[TokenTree], &[TokenTree]]
                    // 它们分别对应名字和类型
                    TokenTree::Punct(p) => p.as_char() == ':',
                    _ => false,
                })
                .collect::<Vec<_>>();
            if let TokenTree::Literal(literal) = &ts[0][0] {
                match literal.to_string().as_str() {
                    "\"type\"" => {
                        check_type(&ts[1][0]);
                        ty = true
                    }
                    _ => {}
                }
            }
        }
        if !ty {
            panic!("必须包含 type 字段");
        }
        ReadResult::Other("bool".into())
    }

    fn read_string(
        name: &str,
        input: &TokenTree,
        extra: &mut HashMap<String, String>,
    ) -> ReadResult {
        let stream = match input {
            TokenTree::Group(v) if v.delimiter() == Delimiter::Brace => v.stream(),
            _ => panic!("必须花括号"),
        };
        let input = stream.into_iter().collect::<Vec<_>>();

        fn check_type(ty: &TokenTree) {
            if let TokenTree::Literal(v) = ty {
                if v.to_string() == "\"string\"" {
                    return;
                }
            }
            panic!("type 字段必须是 string");
        }

        let mut ty = false;
        let mut format = None;
        for tokens in input.split(|t| match t {
            TokenTree::Punct(p) => p.as_char() == ',',
            _ => false,
        }) {
            let ts = tokens
                .split(|v| match v {
                    // 再用 ':' 把 &[TokenTree] 切成 [&[TokenTree], &[TokenTree]]
                    // 它们分别对应名字和类型
                    TokenTree::Punct(p) => p.as_char() == ':',
                    _ => false,
                })
                .collect::<Vec<_>>();
            if let TokenTree::Literal(literal) = &ts[0][0] {
                match literal.to_string().as_str() {
                    "\"type\"" => {
                        check_type(&ts[1][0]);
                        ty = true
                    }
                    "\"format\"" => match ts[1][0].to_string().as_str() {
                        "\"date-time\"" => format = Some("String".into()),
                        v => panic!("format: {} is not support", v),
                    },
                    "\"enum\"" => {
                        let names = match &ts[1][0] {
                            TokenTree::Group(v) if v.delimiter() == Delimiter::Bracket => v
                                .stream()
                                .into_iter()
                                .filter(|v| match v {
                                    TokenTree::Literal(_) => true,
                                    _ => false,
                                })
                                .map(|v| {
                                    let n = v
                                        .to_string()
                                        .trim_end_matches("\"")
                                        .trim_start_matches("\"")
                                        .to_string();

                                    let n = format!("{}{}", &n[0..1].to_uppercase(), &n[1..]);
                                    n
                                })
                                .collect::<Vec<_>>(),
                            _ => panic!("必须是 中括号"),
                        };
                        dbg!(&names);
                        let n = format!("{}Enum", name);
                        if !extra.contains_key(&n) {
                            let de = format!(
                                "pub enum {} {{\n{}\n}}",
                                n,
                                names
                                    .iter()
                                    .map(|n| format!("\t{},", n))
                                    .collect::<Vec<_>>()
                                    .join("\n")
                            );
                            extra.insert(n.clone(), de);
                        }
                        format = Some(n);
                    }
                    _ => {}
                }
            }
        }
        if !ty {
            panic!("必须包含 type 字段");
        }
        if format.is_none() {
            panic!("必须包含 format 字段");
        }
        ReadResult::Other(format.unwrap())
    }

    fn read_object(input: &TokenTree, mut extra: &mut HashMap<String, String>) -> ReadResult {
        let stream = match input {
            TokenTree::Group(v) if v.delimiter() == Delimiter::Brace => v.stream(),
            _ => panic!("必须花括号"),
        };
        let input = stream.into_iter().collect::<Vec<_>>();

        fn check_type(ty: &TokenTree) {
            if let TokenTree::Literal(v) = ty {
                if v.to_string() == "\"object\"" {
                    return;
                }
            }
            panic!("type 字段必须是 object");
        }

        fn read_props(props: &TokenTree, mut extra: &mut HashMap<String, String>) -> Vec<Field> {
            let stream = match props {
                TokenTree::Group(v) if v.delimiter() == Delimiter::Brace => v.stream(),
                _ => panic!("properties 字段必须花括号"),
            }
            .into_iter()
            .collect::<Vec<_>>();
            let props = stream
                .split(|v| match v {
                    TokenTree::Punct(p) => p.as_char() == ',',
                    _ => false,
                })
                .map(|v| {
                    v.split(|v| match v {
                        TokenTree::Punct(p) => p.as_char() == ':',
                        _ => false,
                    })
                    .collect::<Vec<_>>()
                })
                .map(|ts| {
                    let name = match &ts[0][0] {
                        TokenTree::Literal(v) => v
                            .to_string()
                            .trim_end_matches("\"")
                            .trim_start_matches("\"")
                            .to_string(),
                        _ => panic!("必须是字符串"),
                    };
                    let n = format!("{}{}", &name[0..1].to_uppercase(), &name[1..]);
                    let ty = GenerateContext::read_type(&n, &ts[1][0], &mut extra);
                    Field { name, ty }
                })
                .collect::<Vec<_>>();
            props
        }

        let mut ty = false;
        let mut props = Vec::new();

        for tokens in input.split(|t| match t {
            TokenTree::Punct(p) => p.as_char() == ',',
            _ => false,
        }) {
            let ts = tokens
                .split(|v| match v {
                    // 再用 ':' 把 &[TokenTree] 切成 [&[TokenTree], &[TokenTree]]
                    // 它们分别对应名字和类型
                    TokenTree::Punct(p) => p.as_char() == ':',
                    _ => false,
                })
                .collect::<Vec<_>>();
            if let TokenTree::Literal(literal) = &ts[0][0] {
                match literal.to_string().as_str() {
                    "\"type\"" => {
                        check_type(&ts[1][0]);
                        ty = true
                    }
                    "\"properties\"" => {
                        props = read_props(&ts[1][0], &mut extra);
                    }
                    _ => {}
                }
            }
        }
        if !ty {
            panic!("必须包含 type 字段");
        }
        ReadResult::Object(props)
    }

    fn read_type(name: &str, input: &TokenTree, extra: &mut HashMap<String, String>) -> ReadResult {
        let stream = match &input {
            TokenTree::Group(v) if v.delimiter() == Delimiter::Brace => v.stream(),
            _ => panic!("必须花括号"),
        }
        .into_iter()
        .collect::<Vec<_>>();

        let ty = stream
            .split(|v| match v {
                TokenTree::Punct(p) => p.as_char() == ',',
                _ => false,
            })
            .map(|v| {
                v.split(|v| match v {
                    TokenTree::Punct(p) => p.as_char() == ':',
                    _ => false,
                })
                .collect::<Vec<_>>()
            })
            .find_map(|ts| {
                if let TokenTree::Literal(v) = &ts[0][0] {
                    if v.to_string() == "\"type\"" {
                        return Some(
                            ts[1][0]
                                .to_string()
                                .trim_end_matches("\"")
                                .trim_start_matches("\"")
                                .to_string(),
                        );
                    }
                }
                None
            });

        if let None = ty {
            panic!("必须包含 type 字段");
        }
        let ty = ty.unwrap();
        match ty.as_str() {
            "object" => Self::read_object(input, extra),
            "integer" => Self::read_integer(input),
            "string" => Self::read_string(name, input, extra),
            "boolean" => Self::read_boolean(input),
            v => {
                panic!("type: {} is not support", v)
            }
        }
    }

    /// 从 TokenStream 中提取信息，构建 BuilderContext
    fn new(input: TokenStream) -> Self {
        let mut input = input.into_iter().collect::<VecDeque<_>>();
        // 1. 必须是花括号
        let input = input.pop_front().unwrap();

        let mut extra = HashMap::new();
        let ty = Self::read_type("", &input, &mut extra);
        let fields = match ty {
            ReadResult::Object(fields) => fields
                .iter()
                .map(|f| Fd {
                    name: f.name.to_owned(),
                    ty: f.ty.name(),
                })
                .collect::<Vec<_>>(),
            _ => panic!("必须是 object"),
        };

        let name = {
            let stream = match &input {
                TokenTree::Group(v) if v.delimiter() == Delimiter::Brace => v.stream(),
                _ => panic!("必须花括号"),
            }
            .into_iter()
            .collect::<Vec<_>>();
            stream
                .split(|v| match v {
                    TokenTree::Punct(p) => p.as_char() == ',',
                    _ => false,
                })
                .map(|s| {
                    s.split(|v| match v {
                        TokenTree::Punct(p) => p.as_char() == ':',
                        _ => false,
                    })
                    .collect::<Vec<_>>()
                })
                .find_map(|ts| {
                    if let TokenTree::Literal(v) = &ts[0][0] {
                        if v.to_string() == "\"xml\"" {
                            let input = &ts[1][0];
                            let stream = match &input {
                                TokenTree::Group(v) if v.delimiter() == Delimiter::Brace => {
                                    v.stream()
                                }
                                _ => panic!("必须花括号"),
                            }
                            .into_iter()
                            .collect::<Vec<_>>();
                            return stream
                                .split(|v| match v {
                                    TokenTree::Punct(p) => p.as_char() == ',',
                                    _ => false,
                                })
                                .map(|s| {
                                    s.split(|v| match v {
                                        TokenTree::Punct(p) => p.as_char() == ':',
                                        _ => false,
                                    })
                                    .collect::<Vec<_>>()
                                })
                                .find_map(|ts| {
                                    if let TokenTree::Literal(v) = &ts[0][0] {
                                        if v.to_string() == "\"name\"" {
                                            return Some(
                                                ts[1][0]
                                                    .to_string()
                                                    .trim_end_matches("\"")
                                                    .trim_start_matches("\"")
                                                    .to_string(),
                                            );
                                        }
                                    }
                                    None
                                });
                        }
                    }
                    None
                })
        };

        Self {
            extra_types: extra.into_iter().map(|(_, v)| v).collect::<Vec<_>>(),
            name: name.unwrap(),
            fields,
        }
    }

    /// 把模板渲染成字符串代码
    pub fn render(input: TokenStream) -> Result<String> {
        let template = Self::new(input);
        println!("{:#?}", template);
        Ok(template.render()?)
    }
}
