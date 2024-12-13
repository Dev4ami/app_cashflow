sudo docker build -t actix_web_server .
docker run -d --restart=always -p 8080:8080 --env-file .env actix_web_server
