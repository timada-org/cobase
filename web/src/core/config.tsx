import {
  createContext,
  createResource,
  ErrorBoundary,
  ParentComponent,
  Show,
  Suspense,
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

const Component: ParentComponent = (props) => {
  const [config] = createResource<AppConfig>(() =>
    fetch(`${import.meta.env.BASE_URL}config.json`).then((response) =>
      response.json()
    )
  );

  return (
    <Show when={config()}>
      <Context.Provider value={config() as AppConfig}>
        {props.children}
      </Context.Provider>
    </Show>
  );
};

export const Config: ParentComponent = (props) => {
  return (
    <ErrorBoundary fallback="">
      <Suspense>
        <Component children={props.children} />
      </Suspense>
    </ErrorBoundary>
  );
};
