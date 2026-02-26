EXEC_DIR := target/release
STEP0 = step0_repl
STEP1 = step1_read_print
STEP = $(STEP0)

$(EXEC_DIR)/$(STEP):
	cargo build --release
	mv $(EXEC_DIR)/akmal $(EXEC_DIR)/$(STEP)
