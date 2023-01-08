import {
  createContext,
  createResource,
  Match,
  ParentComponent,
  Switch,
  useContext,
} from "solid-js";

interface AppConfigPikav {
  url: string;
  api: string;
}

interface AppConfig {
  api: string;
  pikav: AppConfigPikav;
}

const Context = createContext<AppConfig>({
  api: "",
  pikav: {
    url: "string",
    api: "string",
  },
});

export function useConfig(): AppConfig {
  return useContext(Context);
}

const Config: ParentComponent = (props) => {
  const [config] = createResource<AppConfig>(() =>
    fetch(`${import.meta.env.BASE_URL}config.json`).then((response) =>
      response.json()
    )
  );

  return (
    <Switch>
      <Match when={config.loading}>
        <p>Initialize...</p>
      </Match>
      <Match when={config.error}>
        <p>Error: {config.error.message}</p>
      </Match>
      <Match when={config()}>
        <Context.Provider value={config() as AppConfig}>
          {props.children}
        </Context.Provider>
      </Match>
    </Switch>
  );
};

export default Config;
