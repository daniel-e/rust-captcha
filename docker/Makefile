build:
	docker build -f Dockerfile --no-cache=false -t captcha/1 .

run:
	@echo Running  ...
	docker run -i -t -p 8080:8080 --rm --name rustcaptcha captcha/1 /bin/bash

stop:
	@echo Stopping and removing running rustcaptcha instances ...
	docker stop rustcaptcha || echo
	docker kill rustcaptcha || echo
	docker rm rustcaptcha || echo


test:
	docker build -f Dockerfile.test --no-cache=true -t test_rustcaptcha/1 .
	docker run -t --rm test_rustcaptcha/1

demo:
	docker build -f Dockerfile.demo --no-cache=false -t demo_rustcaptcha/1 .
	docker run -i -t -p 8080:8080 --rm --name rustcaptcha demo_rustcaptcha/1 || echo
