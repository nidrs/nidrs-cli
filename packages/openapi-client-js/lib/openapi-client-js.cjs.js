'use strict';

var env = {
    get DEBUG() {
        return window.DEBUG ?? false;
    },
};

function fetchAdapt(fetchFn) {
    return async (requestOptions) => {
        if (env.DEBUG)
            console.log("[FetchAdapt] request:", requestOptions);
        const { method, url, body, headers } = requestOptions;
        const accept = headers.accept;
        const response = await fetchFn(url, {
            method,
            body: body ? JSON.stringify(body) : undefined,
            headers,
        });
        const resHeaders = {};
        response.headers.forEach((value, key) => {
            resHeaders[key] = value;
        });
        const res = {
            status: response.status,
            statusText: response.statusText,
            headers: resHeaders,
            data: accept === "application/json"
                ? await response.json()
                : await response.text(),
        };
        if (env.DEBUG)
            console.log("[FetchAdapt] response:", res);
        return res;
    };
}

class ClientError extends Error {
    payload;
    constructor(message, payload) {
        super(message);
        this.payload = payload;
    }
}
class HttpException extends Error {
    payload;
    constructor(message, payload) {
        super(message);
        this.payload = payload;
    }
}

/**
 * API 接口文件，最终会被 `nid openapi` 命令生成的代码覆盖
 */
class Api {
    request;
    constructor(request) {
        this.request = request;
    }
}

/**
 * Query params injection
 * @param key
 * @returns
 */
function Q(key) {
    return `q/${key}`;
}
/**
 * Path params injection
 * @param key
 * @returns
 */
function P(key) {
    return `p/${key}`;
}
/**
 * Body params injection
 * @param key
 * @returns
 */
function B(key) {
    return `b/${key}`;
}

function reqHandler(dto, method, pathKey, { paths }) {
    let url = "";
    let body = undefined;
    // Find the path in the openapi paths object
    const path = paths[pathKey];
    // Check if the path exists
    if (!path) {
        throw new ClientError(`Path not found: ${pathKey}`, paths);
    }
    // Find the method in the path object
    const pathMethod = path[method];
    // Check if the method exists
    if (!pathMethod) {
        throw new ClientError(`Method not found: ${method}`, path);
    }
    // Get the URL from the path
    url = transformUrlByDto(dto, pathKey, pathMethod?.parameters);
    const contentType = extractContentType(pathMethod);
    // Get the request body schema from the path
    const requestBodySchema = pathMethod.requestBody?.content?.[contentType]?.schema;
    // Check if the request body schema exists
    if (requestBodySchema) {
        // Validate and transform the DTO based on the request body schema
        body = transformBodyByDto(dto, requestBodySchema);
    }
    return {
        method: method.toUpperCase(),
        url: url,
        body: body,
        headers: {
            accept: extractAccept(pathMethod),
            "content-type": contentType,
        },
    };
}
function resHandler(response) {
    if (response.status >= 400) {
        throw new HttpException(response.statusText, response);
    }
    return response.data;
}
function extractAccept(pathMethod) {
    for (const responseCode in pathMethod.responses) {
        const response = pathMethod.responses[responseCode];
        for (const contentType in response.content) {
            return contentType;
        }
    }
    return undefined;
}
function extractContentType(pathMethod) {
    for (const contentType in pathMethod?.requestBody?.content) {
        return contentType;
    }
    return undefined;
}
function transformBodyByDto(dto, schema) {
    const body = {};
    // Iterate over the properties of the schema
    for (const key in schema.properties) {
        const property = schema.properties[key];
        // Check if the property is required and exists in the DTO
        if (property.required && !dto[key]) {
            throw new Error(`Missing required property: ${key}`);
        }
        // Check if the property exists in the DTO
        body[key] = dto[key] ?? dto[B(key)];
    }
    return body;
}
function transformUrlByDto(dto, url, parameters = []) {
    const parametersMap = {};
    parameters.forEach((param) => {
        if (!parametersMap[param.in]) {
            parametersMap[param.in] = [];
        }
        parametersMap[param.in].push(param);
    });
    parametersMap["path"]?.forEach((param) => {
        const paramName = param.name;
        const paramValue = dto[paramName] ?? dto[P(paramName)];
        // Replace the parameter in the URL
        url = url.replace(`{${paramName}}`, encodeURIComponent(paramValue));
    });
    parametersMap["query"]?.forEach((param) => {
        const paramName = param.name;
        const paramValue = dto[paramName] ?? dto[Q(paramName)];
        // Append the parameter to the URL
        url += `${url.includes("?") ? "&" : "?"}${paramName}=${encodeURIComponent(paramValue)}`;
    });
    return url;
}

exports.Api = Api;
exports.B = B;
exports.ClientError = ClientError;
exports.HttpException = HttpException;
exports.P = P;
exports.Q = Q;
exports.fetchAdapt = fetchAdapt;
exports.reqHandler = reqHandler;
exports.resHandler = resHandler;
//# sourceMappingURL=openapi-client-js.cjs.js.map
