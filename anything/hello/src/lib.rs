#[macro_export]
macro_rules! header {
    ($val:literal) => {{
        format!("# {}", $val)
    }};
}

#[macro_export]
macro_rules! code_block {
    ($val:literal) => {{
        format!("```\n{}\n```", $val)
    }};

    (lang = $lang:literal, $code:literal) => {{
        format!("```{}\n{}\n```", $lang, $code)
    }};
}

#[macro_export]
macro_rules! ulist {
    ($($val:literal),+) => {{
        concat!($(" - ", $val, "\n",)+).trim_end().to_string()
    }}
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_header() {
        let val = header!("Hello");

        assert_eq!(val, "# Hello");
    }
}
