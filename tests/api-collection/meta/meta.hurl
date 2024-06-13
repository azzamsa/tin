POST {{base_url}}
Content-Type: application/json

```graphql
{
  meta {
    build
    version
    config {
      env
      baseUrl
      port
    }
  }
}
```

HTTP/1.1 200
[Asserts]
status == 200
jsonpath "$.data.meta.build" == "unknown"
