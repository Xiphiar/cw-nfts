rm -rf ./schema/contracts
find ./contracts -maxdepth 1 -type d \( ! -wholename ../contracts \) -exec bash -c "cd '{}' && pwd && cargo schema" \;

mkdir -p ./schema/contracts
find ./contracts -maxdepth 1 -type d \( ! -wholename ../contracts \) -exec bash -c "cd '{}' && cp -r schema ../../schema/$(basename {})" \;