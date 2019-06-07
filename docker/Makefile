rebuild:
	docker build -f Dockerfile --no-cache -t captcha/1 .

build:
	docker build -f Dockerfile -t captcha/1 .

run:
	@echo Running  ...
	docker run -i -t -p 8080:8080 --rm --name rustcaptcha captcha/1

bash:
	@echo Running  bash ...
	docker run -i -t -p 8080:8080 --rm --name rustcaptcha --entrypoint /bin/bash captcha/1

stop:
	@echo Stopping and removing running rustcaptcha instances ...
	docker stop rustcaptcha || echo
	docker kill rustcaptcha || echo
	docker rm rustcaptcha || echo
