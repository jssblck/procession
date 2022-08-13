use assert_matches::assert_matches;
use proptest::proptest;

use super::*;

proptest! {
    #[test]
    fn parses_valid_redis(input in r#"redis://\w+:[0-6499]/[0-15]"#) {
        assert_equivalent_parse(&input);
        assert_matches!(parse_redis_url(&input), Ok(_));
    }

    #[test]
    fn parses_valid_rediss(input in r#"rediss://\w+:[0-6499]/[0-15]"#) {
        assert_equivalent_parse(&input);
        assert_matches!(parse_redis_url(&input), Ok(_));
    }

    #[test]
    fn parses_valid_redis_unix(input in r#"redis\+unix://\w+:[0-6499]/[0-15]"#) {
        assert_equivalent_parse(&input);
        assert_matches!(parse_redis_url(&input), Ok(_));
    }

    #[test]
    fn parses_valid_unix(input in r#"unix://\w+:[0-6499]/[0-15]"#) {
        assert_equivalent_parse(&input);
        assert_matches!(parse_redis_url(&input), Ok(_));
    }

    #[test]
    fn parses_redis_urls_equivalently(input in "\\PC*") {
        assert_equivalent_parse(&input);
    }
}

#[track_caller]
fn assert_equivalent_parse(input: &str) {
    let redis_parse = redis::parse_redis_url(input);
    let local_parse = parse_redis_url(input);
    match redis_parse {
        Some(_) => assert_matches!(local_parse, Ok(_)),
        None => assert_matches!(local_parse, Err(_)),
    }
}
