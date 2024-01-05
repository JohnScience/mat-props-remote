# mat-props (no wasmtime)

## One liner for testing

```console
docker build -t pew . && docker run --expose=8080 -p 8080:8080 --rm pew & docker rmi pew
```

After that you can visit `http://localhost:8080/swagger-ui/` to see the API documentation.
