export * from "./adapt/fetchAdapt";
export * from "./errors";

import { ClientError, HttpException } from "./errors";

export function reqHandler(
  dto: any,
  method: string,
  pathKey: string,
  { paths }: any
) {
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
  const requestBodySchema =
    pathMethod.requestBody?.content?.[contentType]?.schema;

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

export function resHandler(response: any) {
  if (response.status >= 400) {
    throw new HttpException(response.statusText, response);
  }
  return response.data;
}

function extractAccept(pathMethod: any) {
  for (const responseCode in pathMethod.responses) {
    const response = pathMethod.responses[responseCode];
    for (const contentType in response.content) {
      return contentType;
    }
  }
  return undefined;
}

function extractContentType(pathMethod: any) {
  for (const contentType in pathMethod?.requestBody?.content) {
    return contentType;
  }
  return undefined;
}

function transformBodyByDto(dto: any, schema: any) {
  const body: any = {};

  // Iterate over the properties of the schema
  for (const key in schema.properties) {
    const property = schema.properties[key];

    // Check if the property is required and exists in the DTO
    if (property.required && !dto[key]) {
      throw new Error(`Missing required property: ${key}`);
    }

    // Check if the property exists in the DTO
    if (dto[key] !== undefined) {
      body[key] = dto[key];
    }
  }

  return body;
}

function transformUrlByDto(dto: any, url: string, parameters: any[] = []) {
  const parametersMap = {};

  parameters.forEach((param) => {
    if (!parametersMap[param.in]) {
      parametersMap[param.in] = [];
    }
    parametersMap[param.in].push(param);
  });

  parametersMap["path"]?.forEach((param) => {
    const paramName = param.name;
    const paramValue = dto[paramName];

    // Replace the parameter in the URL
    url = url.replace(`{${paramName}}`, encodeURIComponent(paramValue));
  });

  parametersMap["query"]?.forEach((param) => {
    const paramName = param.name;
    const paramValue = dto[paramName];

    // Append the parameter to the URL
    url += `${url.includes("?") ? "&" : "?"}${paramName}=${encodeURIComponent(
      paramValue
    )}`;
  });

  return url;
}
