#!/bin/bash
pfexec cargo test add_server &
sleep 2
pfexec cargo test add_client
pfexec cargo test add_client_ptr

pfexec cargo test add_slice_server &
sleep 2
pfexec cargo test add_slice_client

wait
