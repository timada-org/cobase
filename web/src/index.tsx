/* @refresh reload */
import { render } from "solid-js/web";
import { Router } from "@solidjs/router";

import "./index.css";
import App from "./App";
import { Config } from "./core/config";

render(
  () => (
    <Config>
      <Router>
        <App />
      </Router>
    </Config>
  ),
  document.getElementById("root") as HTMLElement
);
