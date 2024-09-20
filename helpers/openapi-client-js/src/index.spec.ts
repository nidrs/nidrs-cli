import { reqArgs } from "./index";

describe('Index', () => {
  it('should get reqArgs', () => {
    const dto = {
      name: "test",
      age: 10
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
                type: "string"
              }
            },
            {
              name: "age",
              in: "query",
              required: true,
              schema: {
                type: "number"
              }
            }
          ]
        }
      }
    };

    const pathKey = "/test";
    const method = "get";

    const result = reqArgs(dto, paths, pathKey, method);

    expect(result).toEqual({
      method: "GET",
      url: "?name=test&age=10",
      body: {}
    });
  });
  
});