import api from "./api";

export function fetchDemo(element: HTMLButtonElement) {
  element.addEventListener("click", async () => {
    await api.user.get_all({}).then((res: any) => {
      console.log(res);
    });
    await api.user
      .get_one({
        ["query(id)"]: 22,
        ["path(id)"]: 12,
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
