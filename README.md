With the help of libloading, I put together a small example for the test

1. ADD ice_library.rs to proect
2. USE:
3. let ice_library = ice_library::IceLibrary::new();
4. ice_library.init_secp256_lib();
5. let public_key_u = ice_library.privatekey_to_publickey("00000000000000000000000000000000000035c0d7234df7deb0f20cf7062444");
6. let btc_address_compress = ice_library.privatekey_to_coinaddress("00000000000000000000000000000000000035c0d7234df7deb0f20cf7062444", 0, 0, true)
