poetry run python -m grpc_tools.protoc -Iprotos --python_out=./ --grpc_python_out=./ --pyi_out=./ ./protos/rusty_slm/*.proto
