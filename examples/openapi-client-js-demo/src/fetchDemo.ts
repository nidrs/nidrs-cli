import api from "./api";

export function fetchDemo(element: HTMLButtonElement) {
  element.addEventListener("click", async () => {
    await api.user.get_all({}).then((res) => {
      console.log(res);
    });
    await api.app.get_hello_world({}).then((res) => {
      console.log(res);
    });
  });
}
