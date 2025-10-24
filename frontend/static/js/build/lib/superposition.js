export class Result {
    #ok;
    #err;
    constructor(ok, err) {
        this.#ok = ok;
        this.#err = err;
    }
    unwrap(errorFn) {
        if (this.#ok === undefined || this.#ok === null) {
            if (errorFn) {
                errorFn("Unwrapping empty result");
            }
            else {
                throw new Error("Unwrapping empty result", { cause: this.#err });
            }
        }
        return this.#ok;
    }
    unwrapOr(value) {
        if (!this.#ok) {
            return value;
        }
        return this.#ok;
    }
    unwrapElseOr(fn) {
        if (!this.#ok) {
            return fn();
        }
        return this.#ok;
    }
    unwrapErr(errorFn) {
        if (this.#err === undefined || this.#err === null) {
            if (errorFn) {
                errorFn("Unwrapping empty error");
            }
            else {
                throw new Error("Unwrapping empty error", { cause: this.#ok });
            }
        }
        return this.#err;
    }
    isOk() {
        return this.#ok !== undefined && this.#ok !== null;
    }
    isErr() {
        return this.#err !== undefined && this.#err !== null;
    }
}
export function Ok(ok) {
    return new Result(ok);
}
export function Err(err) {
    return new Result(undefined, err);
}
export const isOk = (value) => value.isOk();
export const isErr = (value) => value.isErr();
