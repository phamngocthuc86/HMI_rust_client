#!/bin/bash
echo "Sample using HMI Testing Tool script"

HMI_TEST_TOOL_BINARY_LOCATION="./target/debug/hmiTcpTestTool"

$HMI_TEST_TOOL_BINARY_LOCATION -f msg_lte_signal_level_Minus_1.txt
sleep 3

$HMI_TEST_TOOL_BINARY_LOCATION -f msg_lte_signal_level_0.txt
sleep 3

$HMI_TEST_TOOL_BINARY_LOCATION -f msg_lte_signal_level_1.txt
sleep 3

$HMI_TEST_TOOL_BINARY_LOCATION -f msg_lte_signal_level_2.txt
sleep 3

echo "Sample using HMI Testing Tool script FINISHED SAMPLE"

# $HMI_TEST_TOOL_BINARY_LOCATION -f msg_ota_success.txt