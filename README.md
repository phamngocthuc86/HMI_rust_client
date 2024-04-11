# Rust Application for HMI Testing

This Rust application serves as a tool for HMI (Human-Machine Interface) testing, enabling users to send JSON messages to a specified server. It leverages the Tokio async runtime for efficient handling of asynchronous operations.

## How to build
  + in same linux machine which HMI is running (prallel unbuntu machine)
    run : cargo build   
  + For testing Rush on the board. you need to build this one as cross compilation (like mqtt-hmi, .....)
    and run the same way
## How to run 
   ./target/debug/hmiTcpTestTool -f msg_ota_success.txt


## NOTE You can create complex scenario using multiple sending times
  1. example 1: run script : ./run_combo_hmi_message_sample1.sh
  2. example 1: run script : ./run_combo_test_lte_sample.sh