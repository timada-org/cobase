import { Configuration, GroupApi } from "@timada/cobase-client";
import { createContext, ParentComponent, useContext } from "solid-js";
import { useConfig } from "./Config";

interface ApiContext {
  group: GroupApi;
}

const Context = createContext<ApiContext>();

export function useApi(): ApiContext {
  return useContext(Context) as ApiContext;
}

const Api: ParentComponent = (props) => {
  const config = useConfig();
  const groupApi = new GroupApi(new Configuration({ basePath: config.api }));
  const apiContext = { group: groupApi };

  return (
    <Context.Provider value={apiContext}>{props.children}</Context.Provider>
  );
};

export default Api;
