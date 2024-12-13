sudo doccker build -t telegram_server .
docker run -d --restart=always --env-file .env telegram_server
