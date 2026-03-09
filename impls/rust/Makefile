EXEC_DIR := target/release
STEP0 = step0_repl
STEP1 = step1_read_print
STEP = $(STEP1)

$(EXEC_DIR)/$(STEP):
	rm -rf $(EXEC_DIR)
	cargo build --release
	mv $(EXEC_DIR)/akmal $(EXEC_DIR)/$(STEP)

clean:
	rm -rf $(EXEC_DIR)
