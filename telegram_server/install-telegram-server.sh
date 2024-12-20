sudo docker build -t telegram_server .
docker run -d --restart=always --env-file .env telegram_server
docker buildx prune -f
