# 0.2.0

* major: rename `Captures::parse` to `parse_get` for consistency
* major: change errors to require `Display` instead of `Debug`
* major: Remove `anyhow` in favor of custom `CaptureError` type
* major: Abstract capture group names from `&'static str` to `&str`
* patch: Add documentation

## 0.1.1

Add `Captures::parse_name` for the `FromStr` + named capture combination

# 0.1.0

Initial release
