Valid Regex template for parsing.

Template format

```json
{
  "A": "a",
  "custom": "{[my_key]: [my_value]}}"
}
```

A valid `[my_key]` is one of (or combination) :

1. string e.g `foo`
2. nested object key e.g `foo.bar`
3. array key e.g `foo.1`
4. combinations like `foo.1.bar`

A valid `[my_value]` is a:

1. string e.g `"bar"`
2. boolean e.g `true`
3. number e.g `1`

Example JSON Template

```json
{
  "index": "myindex",
  "body": {
    "query": {
      "match": {
        "title": "{{Foo: true}}"
      }
    },
    "facets": {
      "tags": {
        "terms": {
          "field": "tags",
          "cool": "{{foo:bar}}",
          // String is the default type for default values
          "t4": "{{foo:false}}",
          "t5": "{{Foo}}",
          // No default value
          "t5": "{{is_cool_api:true}}",
          "t6": "{{nice_name:false}}"
        }
      }
    }
  }
}
```
