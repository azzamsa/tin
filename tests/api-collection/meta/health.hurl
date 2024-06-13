POST {{base_url}}
Content-Type: application/json

```graphql
{
  health {
    status
  }
}
```

HTTP/1.1 200
[Asserts]
status == 200
jsonpath "$.data.health.status" == "running"


