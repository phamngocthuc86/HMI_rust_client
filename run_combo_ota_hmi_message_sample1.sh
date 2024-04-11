#!/bin/bash
echo "Sample using HMI Testing Tool script"

HMI_TEST_TOOL_BINARY_LOCATION="./target/debug/hmiTcpTestTool"

$HMI_TEST_TOOL_BINARY_LOCATION -f msg_ota_downloading.txt
sleep 3

$HMI_TEST_TOOL_BINARY_LOCATION -f msg_ota_installing1.txt
sleep 3

$HMI_TEST_TOOL_BINARY_LOCATION -f msg_ota_installing2.txt
sleep 3

$HMI_TEST_TOOL_BINARY_LOCATION -f msg_ota_success.txt
sleep 3

echo "Sample using HMI Testing Tool script FINISHED SAMPLE"

# $HMI_TEST_TOOL_BINARY_LOCATION -f msg_ota_success.txt