import "./style.css";
import logo from "/logo.jpg";
import { fetchDemo } from "./fetchDemo.ts";

window.eval("window.DEBUG = true");

document.querySelector<HTMLDivElement>("#app")!.innerHTML = `
  <div>
    <a href="https://github.com/nidrs/nidrs" target="_blank">
      <img src="${logo}" class="logo" alt="Vite logo" />
    </a>
    <h1>Nidrs Openapi Client Demo</h1>
    <div class="card">
      <button id="fetcher" type="button">Send Request</button>
    </div>
    <p class="read-the-docs">
     Please open the console.
    </p>
  </div>
`;

fetchDemo(document.querySelector<HTMLButtonElement>("#fetcher")!);
