test:
	docker build -f Dockerfile.test --no-cache=true -t test_rustcaptcha/1 .
	docker run -t --rm test_rustcaptcha/1

demo:
	@echo Stopping and removing running rustcaptcha instances ...
	docker stop rustcaptcha || echo
	docker kill rustcaptcha || echo
	docker rm rustcaptcha || echo
	@echo Running demo ...
	docker build -f Dockerfile.demo --no-cache=false -t demo_rustcaptcha/1 .
	docker run -i -t -p 8080:8080 --rm --name rustcaptcha demo_rustcaptcha/1 || echo
