#!/bin/sh
# Make sure this file has LF line endings and not CRLF line endings so it can run on linux.

cd ~/Documents/G3Tech/Server/LolDraft

docker load < lol-draft.tar

docker compose down

docker compose up