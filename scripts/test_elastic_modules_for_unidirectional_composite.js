const padding = 0;
const endianness /*uint8*/ = (() => {
    const num = new Uint16Array([0x0100]);
    const view = new Uint8Array(num.buffer);
    return view[0] // === 1 ? 'BE' : 'LE';
})();
const native_endianness /*boolean*/ = !endianness;
const number_of_model = 2;
const fiber_content = 0.2;
const e_for_fiber = 100.0;
const nu_for_fiber = 0.3;
const e_for_matrix = 5.0;
const nu_for_matrix = 0.2;

const args_buffer = new ArrayBuffer(48);
let args = new DataView(args_buffer);
args.setUint8(0, endianness);
args.setUint8(1, number_of_model);
args.setUint8(2, padding);
args.setUint8(3, padding);
args.setUint8(4, padding);
args.setUint8(5, padding);
args.setUint8(6, padding);
args.setUint8(7, padding);
args.setFloat64(8, fiber_content, native_endianness);
args.setFloat64(16, e_for_fiber, native_endianness);
args.setFloat64(24, nu_for_fiber, native_endianness);
args.setFloat64(32, e_for_matrix, native_endianness);
args.setFloat64(40, nu_for_matrix, native_endianness);

console.log(`endianness = ${endianness === 1 ? 'BE' : 'LE'}`);
console.log(`args_buffer = ${new Uint8Array(args_buffer)}`);
const req = new XMLHttpRequest();
req.open('POST', 'http://localhost:8080/compute/elastic_modules_for_unidirectional_composite');
req.responseType = 'arraybuffer';
req.onload = (_event) => {
    const status_code = req.status;
    console.log(`status_code = ${status_code}`);
    const arrayBuffer = req.response;
    if (arrayBuffer) {
        const resp_view = new Float64Array(arrayBuffer);
        const e1 = resp_view[0];
        const e2 = resp_view[1];
        const e3 = resp_view[2];
        const nu12 = resp_view[3];
        const nu13 = resp_view[4];
        const nu23 = resp_view[5];
        const g12 = resp_view[6];
        const g13 = resp_view[7];
        const g23 = resp_view[8];
        console.log(`e1 = ${e1}`);
        console.log(`e2 = ${e2}`);
        console.log(`e3 = ${e3}`);
        console.log(`nu12 = ${nu12}`);
        console.log(`nu13 = ${nu13}`);
        console.log(`nu23 = ${nu23}`);
        console.log(`g12 = ${g12}`);
        console.log(`g13 = ${g13}`);
        console.log(`g23 = ${g23}`);
    }
};
req.send(args_buffer)
