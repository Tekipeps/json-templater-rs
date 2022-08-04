Valid Regex template for parsing.

format
{"A": a", "custom": "{[`key`]: [`value`]}}"

A valid `key` is one of (or combination) :

1. string e.g `foo`
2. nested object key e.g `foo.bar`
3. array key e.g `foo.1`
4. combinations like `foo.1.bar`

A valid `value` is a:

1. string e.g `"bar"`
2. boolean e.g `true`
3. number e.g `1`
