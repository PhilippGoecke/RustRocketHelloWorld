podman build --no-cache --rm --file Containerfile --tag rocket:demo .
podman run --interactive --tty --publish 8000:8000 rocket:demo
echo "browse http://localhost:8000/?name=Test"
