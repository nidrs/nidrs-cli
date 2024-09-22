// @ts-nocheck eslint-disable prettier-ignore
import { reqHandler, resHandler } from "@nidrs/openapi-client-js";

/* prettier-ignore */
export class UserController {
  constructor(private api: Api) {}
  async get_all(dto:any) {
    return resHandler(await this.api.request(reqHandler(dto, 'get', '/api/v1/user/', this.api.openapi)))
  }
  async create_user(dto:any) {
    return resHandler(await this.api.request(reqHandler(dto, 'post', '/api/v1/user/', this.api.openapi)))
  }
  async get_one(dto:any) {
    return resHandler(await this.api.request(reqHandler(dto, 'get', '/api/v1/user/{id}', this.api.openapi)))
  }
}
/* prettier-ignore */
export class AppController {
  constructor(private api: Api) {}
  async post_hello_world(dto:any) {
    return resHandler(await this.api.request(reqHandler(dto, 'post', '/api/v1/hello', this.api.openapi)))
  }
  async get_hello_world(dto:any) {
    return resHandler(await this.api.request(reqHandler(dto, 'get', '/api/v2/hello', this.api.openapi)))
  }
}
/* prettier-ignore */
export class Api {
  user = new UserController(this);
  app = new AppController(this);
  openapi = {"components":{},"info":{"title":"Nidrs OpenAPI","version":"v1.0"},"openapi":"3.0.3","paths":{"/api/v1/hello":{"post":{"description":"AppController::post_hello_world","responses":{},"tags":["AppController"],"x-controller":"AppController","x-router":"post_hello_world"}},"/api/v1/user/":{"get":{"description":"UserController::get_all","responses":{"200":{"content":{"text/plain":{"example":"String","schema":{"type":"object"}}},"description":""}},"tags":["UserController"],"x-controller":"UserController","x-router":"get_all"},"post":{"description":"UserController::create_user","requestBody":{"content":{"application/json":{"schema":{"properties":{"age":{"format":"int32","type":"integer"},"name":{"type":"string"}},"required":["name","age"],"type":"object"}}}},"responses":{"200":{"content":{"application/json":{"schema":{"properties":{"id":{"format":"int32","type":"integer"},"name":{"type":"string"}},"required":["id","name"],"type":"object"}}},"description":""}},"tags":["UserController"],"x-controller":"UserController","x-router":"create_user"}},"/api/v1/user/{id}":{"get":{"description":"UserController::get_one","parameters":[{"in":"path","name":"id","required":true,"schema":{"format":"int32","type":"integer"}},{"in":"query","name":"filter","required":true,"schema":{"type":"string"}},{"in":"query","name":"page","required":true,"schema":{"format":"int32","type":"integer"}},{"in":"query","name":"size","required":true,"schema":{"format":"int32","type":"integer"}}],"responses":{"200":{"content":{"text/plain":{"example":"String","schema":{"type":"object"}}},"description":""}},"tags":["UserController"],"x-controller":"UserController","x-router":"get_one"}},"/api/v2/hello":{"get":{"description":"AppController::get_hello_world","responses":{"200":{"content":{"application/json":{"schema":{"properties":{"db":{"type":"string"},"redis":{"type":"string"}},"required":["db","redis"],"type":"object"}}},"description":""}},"tags":["AppController"],"x-controller":"AppController","x-router":"get_hello_world"}}},"tags":[{"description":"Tag for UserController","name":"UserController"},{"description":"Tag for AppController","name":"AppController"}]};
  constructor(public request: any) {}
}
