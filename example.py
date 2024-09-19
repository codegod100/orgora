import extism

input = open("fixtures/sample1.org", "r").read()
expected = open("fixtures/sample1.html","r").read()
manifest = {"wasm": [{"path": "target/wasm32-unknown-unknown/debug/orgora.wasm"}]}
with extism.Plugin(manifest) as plugin:
    o = plugin.call(
        "parse_with_wasm",
        input
    )
actual = o.decode("UTF-8")
print(actual)

assert(actual == expected)