/* @refresh reload */
import { render } from "solid-js/web";
import { Provider } from "pikav/solid";

import "./index.css";
import App from "./App";
import CheckAuth from "./CheckAuth";
import { Client } from "pikav";

let client = new Client({ url: "/events", api: "/pikav", namespace: "cobase" });

render(
  () => (
    <Provider client={client}>
      <CheckAuth>
        <App />
      </CheckAuth>
    </Provider>
  ),
  document.getElementById("root") as HTMLElement
);
