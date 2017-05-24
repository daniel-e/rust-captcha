build:
	docker build -f Dockerfile --no-cache=false -t captcha/1 .

run:
	@echo Running  ...
	docker run -i -t -p 8080:8080 --rm --name rustcaptcha captcha/1

bash:
	@echo Running  bash ...
	docker run -i -t -p 8080:8080 --rm --name rustcaptcha captcha/1 /bin/bash

stop:
	@echo Stopping and removing running rustcaptcha instances ...
	docker stop rustcaptcha || echo
	docker kill rustcaptcha || echo
	docker rm rustcaptcha || echo
