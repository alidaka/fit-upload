setup:
	./setup.sh

push:
	git push
	git push github

unsetup:
	sudo systemctl disable fit-upload.service
	rm --force fit-upload.service
	rm --force systemd.log
