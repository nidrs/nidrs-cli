import { HttpException } from "./errors";
import { reqHandler, resHandler } from "./index";

describe("Index", () => {
  it("should get reqHandler", () => {
    const dto = {
      name: "test",
      age: 10,
    };
    const paths = {
      "/test": {
        get: {
          parameters: [
            {
              name: "name",
              in: "query",
              required: true,
              schema: {
                type: "string",
              },
            },
            {
              name: "age",
              in: "query",
              required: true,
              schema: {
                type: "number",
              },
            },
          ],
        },
      },
    };

    const pathKey = "/test";
    const method = "get";

    const result = reqHandler(dto, method, pathKey, { paths });

    expect(result).toEqual({
      method: "GET",
      url: "/test?name=test&age=10",
      body: undefined,
      headers: {
        accept: undefined,
        "content-type": undefined,
      },
    });
  });

  it("should post reqHandler", () => {
    const dto = {
      name: "test",
      age: 10,
    };
    const paths = {
      "/test": {
        post: {
          requestBody: {
            content: {
              "application/json": {
                schema: {
                  type: "object",
                  properties: {
                    name: {
                      type: "string",
                    },
                    age: {
                      type: "number",
                    },
                  },
                  required: ["name", "age"],
                },
              },
            },
          },
        },
      },
    };

    const pathKey = "/test";
    const method = "post";

    const result = reqHandler(dto, method, pathKey, { paths });

    expect(result).toEqual({
      method: "POST",
      url: "/test",
      body: {
        name: "test",
        age: 10,
      },
      headers: {
        accept: undefined,
        "content-type": "application/json",
      },
    });
  });

  it("should params reqHandler", () => {
    const dto = {
      id: "iddd",
      name: "test",
      age: 10,
    };
    const paths = {
      "/test/{id}": {
        get: {
          parameters: [
            {
              name: "id",
              in: "path",
              required: true,
              schema: {
                type: "string",
              },
            },
            {
              name: "name",
              in: "query",
              required: true,
              schema: {
                type: "string",
              },
            },
            {
              name: "age",
              in: "query",
              required: true,
              schema: {
                type: "number",
              },
            },
          ],
        },
      },
    };

    const pathKey = "/test/{id}";
    const method = "get";

    const result = reqHandler(dto, method, pathKey, { paths });

    expect(result).toEqual({
      method: "GET",
      url: "/test/iddd?name=test&age=10",
      body: undefined,
      headers: {
        accept: undefined,
        "content-type": undefined,
      },
    });
  });

  it("resHandler should return data", () => {
    const response = {
      status: 200,
      data: "data",
    };

    const result = resHandler(response);

    expect(result).toEqual("data");
  });

  it("resHandler should throw error", () => {
    const response = {
      status: 400,
      statusText: "error",
    };

    expect(() => resHandler(response)).toThrow(
      new HttpException("error", response)
    );
  });
});
