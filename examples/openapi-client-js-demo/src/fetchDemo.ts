import { P, Q } from "@nidrs/openapi-client-js";
import api from "./api";

export function fetchDemo(element: HTMLButtonElement) {
  element.addEventListener("click", async () => {
    await api.user.get_all({}).then((res: any) => {
      console.log(res);
    });
    await api.user
      .get_one({
        [P("id")]: 22,
        [Q("id")]: 12,
        filter: "filter",
        page: 1,
        size: 10,
      })
      .then((res: any) => {
        console.log(res);
      });
    await api.app.get_hello_world({}).then((res: any) => {
      console.log(res);
    });
  });
}
