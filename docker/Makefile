all:
	docker build -f Dockerfile -t captcha/1 .

rebuild:
	docker build -f Dockerfile --no-cache -t captcha/1 .

run:
	@echo Running  ...
	docker run -i -t -p 8000:8000 --rm --name rustcaptcha captcha/1

bash:
	@echo Running  bash ...
	docker run -i -t --dns 8.8.8.8 -p 8000:8000 -v /home:/host --rm --name rustcaptcha --entrypoint /bin/bash captcha/1

stop:
	@echo Stopping and removing running rustcaptcha instances ...
	docker stop rustcaptcha || echo
	docker kill rustcaptcha || echo
	docker rm rustcaptcha || echo
