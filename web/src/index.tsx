/* @refresh reload */
import { render } from "solid-js/web";

import "./index.css";
import App from "./App";
import Config from "./Config";

render(
  () => (
    <Config>
      <App />
    </Config>
  ),
  document.getElementById("root") as HTMLElement
);
