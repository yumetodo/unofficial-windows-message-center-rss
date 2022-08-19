use fmt::Display;
use std::fmt;

fn to_xml_str<T: Display>(value: &T, var_name: &str) -> String {
    format!("<{}>{}</{}>", var_name, value, var_name)
}
trait IntoXMLString<T = (), U = ()> {
    fn to_xml_str(&self, var_name: &str) -> String;
}
impl<T: Display> IntoXMLString<T, T> for T {
    fn to_xml_str(&self, var_name: &str) -> String {
        to_xml_str(self, var_name)
    }
}
impl<T: Display> IntoXMLString<Option<T>, T> for Option<T> {
    fn to_xml_str(&self, var_name: &str) -> String {
        if let Some(v) = self {
            to_xml_str(&v, var_name)
        } else {
            String::new()
        }
    }
}
impl<T: IntoXMLString> IntoXMLString for Option<T> {
    fn to_xml_str(&self, var_name: &str) -> String {
        if let Some(v) = self {
            v.to_xml_str(var_name)
        } else {
            String::new()
        }
    }
}
impl<T: IntoXMLString> IntoXMLString for Vec<T> {
    fn to_xml_str(&self, var_name: &str) -> String {
        let mut ret = String::new();
        for e in self {
            ret += &e.to_xml_str(var_name);
        }
        ret
    }
}
macro_rules! concatenated_xml_accessor {
    ($($name:ident),*) => {
        pub fn as_concatenated_xml(&self) -> String {
            let mut ret = String::new();
            $(
                ret += &self.$name.to_xml_str(stringify!($name));
            )*
            ret
        }
    }
}
macro_rules! xml_attribute_accessor_impl {
    ($( $name:ident ),*) => {
        pub fn as_xml_attributes(&self) -> String {
            let mut capacity: usize = 0;
            $(
                capacity += self.$name.as_deref().map_or(0, |s| s.len());
            )*
            let mut ret = String::with_capacity(capacity * 2);
            $(
                if let Some(v) = self.$name.as_deref() {
                    if !ret.is_empty() {
                        ret += " ";
                    }
                    ret += &format!(r#"{}="{}""#, stringify!($name).trim_end_matches('_'), v);
                }
            )*
            ret
        }
    };
}
macro_rules! optional_member_setter_impl {
    ($struct_name:ident, $( $name:ident : $into_type:ident),*) => {
        $(
            pub fn $name<S: Into<$into_type>>(self, $name: S) -> Self {
                $struct_name {
                    $name: Some($name.into()),
                    ..self
                }
            }
        )*
    }
}
macro_rules! vec_member_setter_impl {
    ($struct_name:ident, $( $name:ident: $elem_type:ident ),*) => {
        $(
            pub fn $name(self, $name: Vec<$elem_type>) -> Self {
                $struct_name {
                    $name: $name,
                    ..self
                }
            }
        )*
    }
}
pub struct Person {
    name: String,
    uri: Option<String>,
    email: Option<String>,
}
impl Person {
    pub fn new<S: Into<String>>(name: S) -> Self {
        Person {
            name: name.into(),
            uri: None,
            email: None,
        }
    }
    optional_member_setter_impl!(Person, uri: String, email: String);
    concatenated_xml_accessor!(name, uri, email);
}
impl IntoXMLString for Person {
    fn to_xml_str(&self, var_name: &str) -> String {
        to_xml_str(&self.as_concatenated_xml(), var_name)
    }
}
#[derive(Default)]
pub struct Link {
    href: Option<String>,
    rel: Option<String>,
    type_: Option<String>,
    hreflang: Option<String>,
    title: Option<String>,
}
impl Link {
    xml_attribute_accessor_impl!(href, rel, type_, hreflang, title);
}
impl IntoXMLString for Link {
    fn to_xml_str(&self, var_name: &str) -> String {
        format!("<{} {} />", var_name, self.as_xml_attributes())
    }
}
pub struct HTMLText {
    text: String,
}
impl HTMLText {
    pub fn new(s: &str) -> Self {
        HTMLText {
            text: String::from(html_escape::encode_text(s)),
        }
    }
}
impl From<&str> for HTMLText {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}
impl IntoXMLString for HTMLText {
    fn to_xml_str(&self, var_name: &str) -> String {
        format!(r#"<{} type="html">{}</{}>"#, var_name, self.text, var_name)
    }
}

pub struct Entry {
    id: String,
    title: String,
    updated: String,
    author: Vec<Person>,
    content: Option<HTMLText>,
    link: Option<Link>,
    summary: Option<HTMLText>,
}
impl Entry {
    pub fn new(id: String, title: String, updated: String) -> Self {
        Entry {
            id,
            title,
            updated,
            author: Default::default(),
            content: None,
            link: None,
            summary: None,
        }
    }
    optional_member_setter_impl!(Entry, content: HTMLText, link: Link, summary: HTMLText);
    vec_member_setter_impl!(Entry, author: Person);
    concatenated_xml_accessor!(id, title, updated, author, content, link, summary);
}
impl IntoXMLString for Entry {
    fn to_xml_str(&self, var_name: &str) -> String {
        to_xml_str(&self.as_concatenated_xml(), var_name)
    }
}
pub struct Feed {
    id: String,
    title: String,
    updated: String,
    author: Vec<Person>,
    link: Option<Link>,
    entry: Vec<Entry>,
}
impl Feed {
    pub fn new(id: String, title: String, updated: String) -> Self {
        Feed {
            id,
            title,
            updated,
            author: Default::default(),
            link: None,
            entry: Default::default(),
        }
    }
    optional_member_setter_impl!(Feed, link: Link);
    vec_member_setter_impl!(Feed, author: Person, entry: Entry);
    concatenated_xml_accessor!(id, title, updated, author, link, entry);
}
impl IntoXMLString for Feed {
    fn to_xml_str(&self, var_name: &str) -> String {
        to_xml_str(&self.as_concatenated_xml(), var_name)
    }
}
