export function uuidv4() {
    if (window?.isSecureContext) {
        return crypto.randomUUID();
    }
    const randomValue = crypto.getRandomValues(new Uint8Array(1))[0];
    if (randomValue === undefined) {
        throw new Error("Unable to generate random uuidv4");
    }
    return "10000000-1000-4000-8000-100000000000".replace(/[018]/g, (c) => (+c ^ (randomValue & (15 >> (+c / 4)))).toString(16));
}
export const clamp = (num, min, max) => Math.min(Math.max(num, min), max);
export function isEmptyString(str) {
    if (!str)
        return true;
    if (str.trim().length === 0)
        return true;
    return false;
}
export function setEmptyStringAsNullish(value) {
    if (isEmptyString(value))
        return undefined;
    return value?.trim();
}
export function uniqByKeepLast(data, key) {
    return [...new Map(data.map((x) => [key(x), x])).values()];
}
export function exhaustiveMatchingGuard(_, message) {
    throw new Error(message ?? "Should not have reached here");
}
export function toLogLevel(value) {
    const v = value?.toLowerCase() ?? "warn";
    switch (v) {
        case "debug":
            return "DEBUG";
        case "info":
            return "INFO";
        case "warn":
            return "WARN";
        case "error":
            return "ERROR";
        case "critical":
            return "CRITICAL";
        default:
            return "WARN";
    }
}
