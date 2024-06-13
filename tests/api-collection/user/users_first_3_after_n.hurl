POST {{base_url}}
Content-Type: application/json

```graphql
{
  users(
    first: 2
    after: "Q3Vyc29yOjAxN2YyYTlmLWVjNmMtYzk1ZS01MGVhLWUzZWZkMWIxMjZhNw"
  ) {
    edges {
      cursor
      node {
        id
        name
      }
    }
    pageInfo {
      hasNextPage
      hasPreviousPage
      startCursor
      endCursor
    }
  }
}
```

HTTP/1.1 200
[Asserts]
status == 200


