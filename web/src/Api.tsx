import { Configuration, UserApi } from "@timada/cobase-client";
import { createContext, ParentComponent, useContext } from "solid-js";
import { useConfig } from "./Config";

const Context = createContext<UserApi>();

export function useApi(): UserApi {
  return useContext(Context) as UserApi;
}

const Api: ParentComponent = (props) => {
  const config = useConfig();
  const api = new UserApi(new Configuration({ basePath: config.api }));

  return <Context.Provider value={api}>{props.children}</Context.Provider>;
};

export default Api;
