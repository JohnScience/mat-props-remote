import { FixedArray } from "./util";

const endianness /*uint8*/ = (() => {
    const num = new Uint16Array([0x0100]);
    const view = new Uint8Array(num.buffer);
    return view[0] // === 1 ? 'BE' : 'LE';
})();
const nativeEndianness /*boolean*/ = !endianness;

export const DEFAULT_BASE_URL = "http://localhost:8080";

export async function elasticModulesForUnidirectionalComposite(
    baseUrl: string,
    numberOfModel: number,
    fiberContent: number,
    eForFiber: number,
    nuForFiber: number,
    eForMatrix: number,
    nuForMatrix: number
): Promise<FixedArray<number, 9>> {
    const url = `${baseUrl}/compute/elastic_modules_for_unidirectional_composite`;
    const argsBuffer = new ArrayBuffer(48);
    const args = new DataView(argsBuffer); 
    args.setUint8(0, endianness);
    args.setUint8(1, numberOfModel);      
    args.setUint8(2, 0); // padding
    args.setUint8(3, 0); // padding
    args.setUint8(4, 0); // padding
    args.setUint8(5, 0); // padding
    args.setUint8(6, 0); // padding
    args.setUint8(7, 0); // padding
    args.setFloat64(8, fiberContent, nativeEndianness);
    args.setFloat64(16, eForFiber, nativeEndianness);
    args.setFloat64(24, nuForFiber, nativeEndianness);
    args.setFloat64(32, eForMatrix, nativeEndianness);
    args.setFloat64(40, nuForMatrix, nativeEndianness);

    return new Promise((resolve, reject) => {
        const req = new XMLHttpRequest();
        req.open('POST', url);
        req.responseType = 'arraybuffer';
        req.onload = () => {
            const arrayBuffer = req.response;
            if (arrayBuffer) {
                const respView = new Float64Array(arrayBuffer);
                const e1 = respView[0];
                const e2 = respView[1];
                const e3 = respView[2];
                const nu12 = respView[3];
                const nu13 = respView[4];
                const nu23 = respView[5];
                const g12 = respView[6];
                const g13 = respView[7];
                const g23 = respView[8];
                resolve([e1, e2, e3, nu12, nu13, nu23, g12, g13, g23]);
            } else {
                reject("No response");
            }
        };
        req.send(argsBuffer);
    });
}

export async function elasticModulesForHoneycomb(
    baseUrl: string,
    numberOfModel: number,
    lCellSideSize: number,
    hCellSideSize: number,
    wallThickness: number,
    angle: number,
    eForHoneycomb: number,
    nuForHoneycomb: number
): Promise<FixedArray<number, 9>> {
    const url = `${baseUrl}/compute/elastic_modules_for_honeycomb`;
    const argsBuffer = new ArrayBuffer(56);
    const args = new DataView(argsBuffer);
    args.setUint8(0, endianness);
    args.setUint8(1, numberOfModel);
    args.setUint8(2, 0); // padding
    args.setUint8(3, 0); // padding
    args.setUint8(4, 0); // padding
    args.setUint8(5, 0); // padding
    args.setUint8(6, 0); // padding
    args.setUint8(7, 0); // padding
    args.setFloat64(8, lCellSideSize, nativeEndianness);
    args.setFloat64(16, hCellSideSize, nativeEndianness);
    args.setFloat64(24, wallThickness, nativeEndianness);
    args.setFloat64(32, angle, nativeEndianness);
    args.setFloat64(40, eForHoneycomb, nativeEndianness);
    args.setFloat64(48, nuForHoneycomb, nativeEndianness);

    return new Promise((resolve, reject) => {
        const req = new XMLHttpRequest();
        req.open('POST', url);
        req.responseType = 'arraybuffer';
        req.onload = () => {
            const arrayBuffer = req.response;
            if (arrayBuffer) {
                const respView = new Float64Array(arrayBuffer);
                const e1 = respView[0];
                const e2 = respView[1];
                const e3 = respView[2];
                const nu12 = respView[3];
                const nu13 = respView[4];
                const nu23 = respView[5];
                const g12 = respView[6];
                const g13 = respView[7];
                const g23 = respView[8];
                resolve([e1, e2, e3, nu12, nu13, nu23, g12, g13, g23]);
            } else {
                reject("No response");
            }
        };
        req.send(argsBuffer);
    });
}

export async function thermalConductivityForUnidirectionalComposite(
    baseUrl: string,
    numberOfModel: number,
    fibreContent: number,
    kForFiber: number,
    kForMatrix: number
): Promise<FixedArray<number, 3>> {
    const url = `${baseUrl}/compute/thermal_conductivity_for_unidirectional_composite`;
    
    const argsBuffer = new ArrayBuffer(32);
    const args = new DataView(argsBuffer); 
    args.setUint8(0, endianness);
    args.setUint8(1, numberOfModel);      
    args.setUint8(2, 0); // padding
    args.setUint8(3, 0); // padding
    args.setUint8(4, 0); // padding
    args.setUint8(5, 0); // padding
    args.setUint8(6, 0); // padding
    args.setUint8(7, 0); // padding
    args.setFloat64(8, fibreContent, nativeEndianness);
    args.setFloat64(16, kForFiber, nativeEndianness);
    args.setFloat64(24, kForMatrix, nativeEndianness);

    return new Promise((resolve, reject) => {
        const req = new XMLHttpRequest();
        req.open('POST', url);
        req.responseType = 'arraybuffer';
        req.onload = () => {
            const arrayBuffer = req.response;
            if (arrayBuffer) {
                const respView = new Float64Array(arrayBuffer);
                const k1 = respView[0];
                const k2 = respView[1];
                const k3 = respView[2];
                resolve([k1,k2,k3]);
            } else {
                reject("No response");
            }
        };
        req.send(argsBuffer);
    });
}

export async function thermalExpansionForUnidirectionalComposite(
    baseUrl: string,
    numberOfModel: number,
    fibreContent: number,
    eForFiber: number,
    nuForFiber: number,
    alphaForFiber: number,
    eForMatrix: number,
    nuForMatrix: number,
    alphaForMatrix: number
): Promise<FixedArray<number, 3>> {
    const url = `${baseUrl}/compute/thermal_expansion_for_unidirectional_composite`;
    
    const argsBuffer = new ArrayBuffer(64);
    const args = new DataView(argsBuffer); 
    args.setUint8(0, endianness);
    args.setUint8(1, numberOfModel);      
    args.setUint8(2, 0); // padding
    args.setUint8(3, 0); // padding
    args.setUint8(4, 0); // padding
    args.setUint8(5, 0); // padding
    args.setUint8(6, 0); // padding
    args.setUint8(7, 0); // padding
    args.setFloat64(8, fibreContent, nativeEndianness);
    args.setFloat64(16, eForFiber, nativeEndianness);
    args.setFloat64(24, nuForFiber, nativeEndianness);
    args.setFloat64(32, alphaForFiber, nativeEndianness);
    args.setFloat64(40, eForMatrix, nativeEndianness);
    args.setFloat64(48, nuForMatrix, nativeEndianness);
    args.setFloat64(56, alphaForMatrix, nativeEndianness);

    return new Promise((resolve, reject) => {
        const req = new XMLHttpRequest();
        req.open('POST', url);
        req.responseType = 'arraybuffer';
        req.onload = () => {
            const arrayBuffer = req.response;
            if (arrayBuffer) {
                const respView = new Float64Array(arrayBuffer);
                const alpha1 = respView[0];
                const alpha2 = respView[1];
                const alpha3 = respView[2];
                resolve([alpha1,alpha2,alpha3]);
            } else {
                reject("No response");
            }
        };
        req.send(argsBuffer);
    });
}

export async function thermalExpansionForHoneycomb(
    baseUrl: string,
    numberOfModel: number,
    lCellSideSize: number,
    hCellSideSize: number,
    wallThickness: number,
    angle: number,
    alphaForHoneycomb: number
): Promise<FixedArray<number, 3>> {
    const url = `${baseUrl}/compute/thermal_expansion_for_honeycomb`;
    
    const argsBuffer = new ArrayBuffer(48);
    const args = new DataView(argsBuffer); 
    args.setUint8(0, endianness);
    args.setUint8(1, numberOfModel);      
    args.setUint8(2, 0); // padding
    args.setUint8(3, 0); // padding
    args.setUint8(4, 0); // padding
    args.setUint8(5, 0); // padding
    args.setUint8(6, 0); // padding
    args.setUint8(7, 0); // padding
    args.setFloat64(8, lCellSideSize, nativeEndianness);
    args.setFloat64(16, hCellSideSize, nativeEndianness);
    args.setFloat64(24, wallThickness, nativeEndianness);
    args.setFloat64(32, angle, nativeEndianness);
    args.setFloat64(40, alphaForHoneycomb, nativeEndianness);

    return new Promise((resolve, reject) => {
        const req = new XMLHttpRequest();
        req.open('POST', url);
        req.responseType = 'arraybuffer';
        req.onload = () => {
            const arrayBuffer = req.response;
            if (arrayBuffer) {
                const respView = new Float64Array(arrayBuffer);
                const alpha1 = respView[0];
                const alpha2 = respView[1];
                const alpha3 = respView[2];
                resolve([alpha1,alpha2,alpha3]);
            } else {
                reject("No response");
            }
        };
        req.send(argsBuffer);
    });
}
