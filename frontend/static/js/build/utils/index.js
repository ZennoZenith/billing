import { UnknowError } from "../lib/error.js";
import { Err, Ok } from "../lib/superposition.js";
export function sleep(ms) {
    return new Promise((resolve) => setTimeout(resolve, ms));
}
export async function catchError(promise) {
    try {
        const data = await promise;
        return Ok(data);
    }
    catch (error) {
        return Err(new UnknowError(error));
    }
}
// Disabled because of any type
// eslint-disable-next-line
// biome-ignore lint/suspicious/noExplicitAny: no explanation
export function catchErrorSync(fn, ...args) {
    try {
        const data = fn(...args);
        return Ok(data);
    }
    catch (error) {
        return Err(new UnknowError(error));
    }
}
