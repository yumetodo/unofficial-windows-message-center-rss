use std::fmt::Display;

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
macro_rules! xml_accessor_impl {
    ($name:ident, $access_name:ident) => {
        pub fn $access_name(&self) -> String {
            self.$name.to_xml_str(stringify!($name))
        }
    };
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
pub struct Person {
    name: String,
    uri: Option<String>,
    email: Option<String>,
}
impl Person {
    xml_accessor_impl!(name, get_name_as_xml);
    xml_accessor_impl!(name, get_uri_as_xml);
    xml_accessor_impl!(name, get_email_as_xml);
    pub fn new<S: Into<String>>(name: S) -> Self {
        Person {
            name: name.into(),
            uri: None,
            email: None,
        }
    }
    pub fn uri<S: Into<String>>(self, uri: S) -> Self {
        Person {
            uri: Some(uri.into()),
            ..self
        }
    }
    pub fn email<S: Into<String>>(self, email: S) -> Self {
        Person {
            email: Some(email.into()),
            ..self
        }
    }
}
impl IntoXMLString for Person {
    fn to_xml_str(&self, var_name: &str) -> String {
        let value = format!(
            "{}{}{}",
            self.get_name_as_xml(),
            self.get_uri_as_xml(),
            self.get_email_as_xml()
        );
        to_xml_str(&value, var_name)
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
pub struct Feed {
    id: String,
    title: String,
    updated: String,
}
impl Feed {
    pub fn new(id: String, title: String, updated: String) -> Self {
        Feed { id, title, updated }
    }
}
