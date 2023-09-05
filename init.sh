#!/bin/bash
sqlx db create 
sqlx migrate run 
cargo run --release -- --bind 0.0.0.0:3000
