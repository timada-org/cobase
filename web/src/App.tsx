import { Component, createSignal, For, Match, Switch } from "solid-js";
import { Provider as PikavProvider, useSubscribe } from "pikav/solid";
import { Client } from "pikav";
import { Group, CreateCommand } from "@timada/cobase-client";

import {
  QueryClient,
  QueryClientProvider,
  createQuery,
  createMutation,
  useQueryClient,
} from "@tanstack/solid-query";
import { useConfig } from "./Config";
import Api, { useApi } from "./Api";

const Groups: Component = () => {
  const api = useApi();
  const queryClient = useQueryClient();
  const [name, setName] = createSignal("");

  const query = createQuery(
    () => ["groups"],
    async () => (await api.listGroups()).data
  );

  const mutation = createMutation({
    mutationFn: async (cmd: CreateCommand) => (await api.createGroup(cmd)).data,
  });

  useSubscribe<Group>("groups/+", (event) => {
    queryClient.setQueryData<Group[]>(
      ["groups"],
      (old) => old && [...old, event.data]
    );
  });

  return (
    <div>
      <form
        onSubmit={(e) => {
          e.preventDefault();

          mutation.mutate({ name: name() });

          setName("");
        }}
      >
        <input
          type="text"
          value={name()}
          onChange={(e) => setName(e.currentTarget.value)}
          disabled={mutation.isLoading}
        />
      </form>

      <Switch>
        <Match when={query.isLoading}>
          <p>Loading...</p>
        </Match>
        <Match when={query.isError}>
          <p>Error: {(query.error as any).message}</p>
        </Match>
        <Match when={query.isSuccess}>
          <ul>
            <For each={query.data}>{(group) => <li>{group.name}</li>}</For>
          </ul>
        </Match>
      </Switch>
    </div>
  );
};

const App: Component = () => {
  const config = useConfig();

  const pikavClient = new Client({
    url: config.pikav.url,
    api: config.pikav.api,
    namespace: "cobase",
  });

  const queryClient = new QueryClient({
    defaultOptions: { queries: { staleTime: 0, refetchOnWindowFocus: false } },
  });

  return (
    <Api>
      <PikavProvider client={pikavClient}>
        <QueryClientProvider client={queryClient}>
          <Groups />
        </QueryClientProvider>
      </PikavProvider>
    </Api>
  );
};

export default App;
