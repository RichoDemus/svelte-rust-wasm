export const from_wasm = async function load() {
    const {greet} = await import("../pkg/index.js")
        // .then(obj => {
        //     const value = obj.greet();
        //     console.log("from js:",value);
        //     return obj;
        // })
        .catch(console.error);

    console.log("also js:", greet());
    return greet();
};

// load();
