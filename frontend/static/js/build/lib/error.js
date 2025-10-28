const errorType = [
    "UnknownError",
    "UntagedError",
    "FetchError",
    "JsonDeserializeError",
    "ApiError",
    "ApiModelError",
    "ValidationError",
    "CriticalError",
    // "ParseError",
];
function stringToNumber(value) {
    const n = Number(value);
    return Number.isNaN(n) ? null : n;
}
function constructApiError(err) {
    if (typeof err !== "object" || err === null) {
        return {
            error: "-1",
            errorCode: -1,
            href: "",
            httpCode: 418,
            title: "",
        };
    }
    const obj = {};
    if ("error" in err)
        obj.error = err?.error?.toString();
    if ("errorCode" in err)
        stringToNumber(err?.errorCode);
    if ("href" in err)
        err?.href?.toString();
    if ("httpCode" in err)
        stringToNumber(err?.httpCode);
    if ("title" in err)
        err?.title?.toString();
    return {
        error: obj.error ?? "-1",
        errorCode: obj.errorCode ?? -1,
        href: obj.href ?? "",
        httpCode: obj.httpCode ?? 418,
        title: obj.title ?? "",
    };
}
// const errorSchema = pipe(
//   object(
//     {
//       success: literal(false, "success should be boolean false"),
//       type: union(
//         errorType.map(v => literal(v)),
//         "invalid error type",
//       ),
//       message: string("message should be string"),
//       name: optional(string("name should be string")),
//       cause: optional(unknown()),
//       messages: pipe(
//         array(
//           string("messages should be string"),
//           "messages should be an array of string",
//         ),
//         minLength(1, "messages array should atleast contain one element of string"),
//       ),
//       extra: record(
//         string("extra object key shoud be string"),
//         unknown(),
//         "extra must by of type Record<string, unknown>",
//       ),
//     },
//   ),
// );
class CustomError extends Error {
    _tag;
    // readonly messages: [string, ...string[]];
    // extra: Record<string, unknown>;
    constructor(tag, message) {
        super(message);
        this._tag = tag;
    }
}
// export class GenericError extends CustomError {
//   constructor(extra?: Record<string, unknown>, messages?: [string, ...string[]]) {
//     super("GenericError", extra, messages ?? ["Generic Error"]);
//   }
// }
export class UnknowError extends CustomError {
    error;
    constructor(error) {
        let message = "Unknow Message";
        if (typeof error === "object" &&
            error !== null &&
            "message" in error &&
            typeof error.message === "string")
            message = error.message;
        super("UnknownError", message);
        if (error instanceof Error) {
            this.error = error;
            return;
        }
        this.error = new Error(message, { cause: error });
    }
}
export class TagedError extends CustomError {
    error;
    constructor(error) {
        let message = "Unknow Message";
        if (typeof error === "object" &&
            error !== null &&
            "message" in error &&
            typeof error.message === "string")
            message = error.message;
        super(error._tag, message);
        if (error instanceof Error) {
            this.error = error;
            return;
        }
        this.error = new Error(message, { cause: error });
    }
}
export class UntagedError extends CustomError {
    error;
    constructor(error) {
        super("UntagedError", error.message);
        this.error = error;
    }
}
export class FetchError extends CustomError {
    error;
    constructor(error) {
        super("FetchError", error.message);
        this.error = error;
    }
    static fromUnknownError(error) {
        return new FetchError(error.error);
    }
}
export class JsonDeserializeError extends CustomError {
    error;
    constructor(error) {
        super("JsonDeserializeError", error.message);
        this.error = error;
    }
    static fromUnknownError(error) {
        return new FetchError(error.error);
    }
}
export class ApiError extends CustomError {
    error;
    constructor(error) {
        const apiError = constructApiError(error);
        super("ApiError", apiError.error);
        this.error = apiError;
    }
}
export class ValidationError extends CustomError {
    validationError;
    constructor(validationError, tag = "ValidationError", message) {
        super(tag, message ?? "Validation Error");
        this.validationError = validationError;
    }
}
export class ApiModelError extends CustomError {
    validationError;
    constructor(extra, message) {
        super("ApiModelError", message ?? "Api Model Error");
        this.validationError = extra;
    }
}
export class CriticalError extends CustomError {
    constructor(message) {
        super("CriticalError", message);
    }
}
