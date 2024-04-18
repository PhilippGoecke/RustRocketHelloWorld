docker build --no-cache --rm -f Containerfile -t rocket:demo .
docker run --interactive --tty -p 8000:8000 rocket:demo
echo "browse http://localhost:8000/"
