import { Configuration, CobaseApi } from "@timada/cobase-client";
import { createContext, ParentComponent, useContext } from "solid-js";
import { useConfig } from "./config";

const Context = createContext<CobaseApi>();

export function useApi(): CobaseApi {
  return useContext(Context) as CobaseApi;
}

export const Api: ParentComponent = (props) => {
  const config = useConfig();
  const api = new CobaseApi(new Configuration({ basePath: config.api }));

  return <Context.Provider value={api}>{props.children}</Context.Provider>;
};
