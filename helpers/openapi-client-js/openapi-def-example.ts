export class UserController {
  api: Api;
  constructor(api: Api) {
    this.api = api;
  }

  login(dto: any) {
    return this.api.request(
      reqArgsDto(dto, this.api.openapi.paths, "/api/v1/user/", "post")
    );
  }
}

export class Api {
  readonly request: any;
  readonly openapi = {
    paths: {},
  };
  readonly user = new UserController(this);

  constructor(request: any) {
    this.request = request;
  }
}

function reqArgsDto(dto: any, paths: any, pathKey: string, method: string) {
  let url = "";
  let body = {};

  // Find the path in the openapi paths object
  const path = paths[pathKey];

  // Check if the path exists
  if (path) {
    // Find the method in the path object
    const pathMethod = path[method];

    // Check if the method exists
    if (pathMethod) {
      // Get the URL from the path
      url = transformUrlByDto(dto, pathMethod.parameters);

      // Get the request body schema from the path
      const requestBodySchema =
        pathMethod.requestBody?.content?.["application/json"]?.schema;

      // Check if the request body schema exists
      if (requestBodySchema) {
        // Validate and transform the DTO based on the request body schema
        body = transformBodyByDto(dto, requestBodySchema);
      }
    }
  }

  return {
    method: method.toUpperCase(),
    url: url,
    body: body,
  };
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

function transformUrlByDto(dto: any, parameters: any[]) {
  let url = "";
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
