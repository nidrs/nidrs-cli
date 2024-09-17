import { Api } from "./openapi-def-example";

const api = new Api(() => {});

api.user.login({
  username: "hello",
  password: "12345678",
});
