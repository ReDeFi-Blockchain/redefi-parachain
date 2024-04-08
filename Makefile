.PHONY: _help
_help:
	@echo "Used to generate stubs for contract(s) defined in native (via evm-coder). See Makefile for details."

NATIVE_FUNGIBLE_EVM_STUBS=./pallets/balances-adapter/src/stubs


NativeFungible.sol:
	PACKAGE=pallet-balances-adapter NAME=eth::gen_impl OUTPUT=$(NATIVE_FUNGIBLE_EVM_STUBS)/$@ ./.maintain/scripts/generate_sol.sh

NativeFungible: NativeFungible.sol
	INPUT=$(NATIVE_FUNGIBLE_EVM_STUBS)/$< OUTPUT=$(NATIVE_FUNGIBLE_EVM_STUBS)/NativeFungible.raw ./.maintain/scripts/compile_stub.sh