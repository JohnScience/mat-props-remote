# mat-props-remote

## Testing the mutli-container application

```console
docker compose up
```

After that you can visit `http://localhost:5173/` to see the frontend and `http://localhost:8080/swagger-ui/` to see the API documentation.

## One liner for testing the backend

```console
docker build -t pew . && docker run --expose=8080 -p 8080:8080 --rm pew & docker rmi pew
```

After that you can visit `http://localhost:8080/swagger-ui/` to see the API documentation.

## One liner for testing the frontend

```console
docker build -t fr -f Dockerfile.front . && docker run --init -it -p 5173:5173 --rm fr & docker rmi fr
```

After that you can visit `http://localhost:5173/` to see the frontend. In order to stop the container, you might have to press `Ctrl+\` rather than `Ctrl+C`.
