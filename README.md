# unofficial-windows-message-center-rss

This is unofficial RSS of [Windows message center | Microsoft Docs](https://docs.microsoft.com/en-us/windows/release-health/windows-message-center)

[RSS](https://yumetodo.github.io/unofficial-windows-message-center-rss/feed/atom10.xml)

## Motivation

Windows message center is known as very important information source for windows application developers.

However, Microsoft doesn't provide RSS so that it's difficult to notice the update of Windows message center.

## Development Note

- Rust
- Using reqwest to get html
- Using scraper(A wrapper library of Servo, a part of implementation of Firefox)
- Serializing xml manually
  - Because there is no well-known Serde xml plugins
  - See `src/feed.rs`
    - Introducing `IntoXMLString` trait for serialize each struct
    - Using self defined macro to implement `IntoXMLString` trait easily
      - `concatenated_xml_accessor`
      - `xml_attribute_accessor_impl`
    - Using self defined macro to implement builder pattern
      - `optional_member_setter_impl`
      - `vec_member_setter_impl`
