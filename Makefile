
# download openapi.yaml at https://raw.githubusercontent.com/igomez10/microservices/mainline/socialapp/openapi.yaml
download-openapi:
	curl https://raw.githubusercontent.com/igomez10/microservices/mainline/socialapp/openapi.yaml -o openapi.yaml

generate-rust-client:
	# delete old generated code
	rm -rf ./rust-sdk
	
	# generate new code
	openapi-generator generate -i openapi.yaml  \
	-g rust \
	-o ./rust-sdk  \
	-p packageName=rust-sdk \
	--git-user-id=igomez10 \
	--git-repo-id=learnrust \
	--additional-properties=library=reqwest

	# GENERATED CODE SUCCESSFULLY
