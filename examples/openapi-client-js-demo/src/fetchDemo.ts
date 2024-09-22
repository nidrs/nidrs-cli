import api from "./api";

export function fetchDemo(element: HTMLButtonElement) {
  element.addEventListener("click", () => {
    api.user.get_all({}).then((res) => {
      console.log(res);
    });
  });
}
