import { Component } from "solid-js";
import { Client } from "pikav";
import { Pikav } from "pikav/solid";

import { useConfig } from "./core/config";
import { Api } from "./core/api";
import Root from "./routes";

const App: Component = () => {
  const config = useConfig();

  const pikavClient = new Client({
    url: config.pikav.url,
    api: config.pikav.api,
    namespace: "cobase",
  });

  return (
    <Api>
      <Pikav client={pikavClient}>
        <Root />
      </Pikav>
    </Api>
  );
};

export default App;
