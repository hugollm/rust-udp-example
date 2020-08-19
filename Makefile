.PHONY: sender receiver

sender:
	rustc sender.rs && ./sender

receiver:
	rustc receiver.rs && ./receiver
