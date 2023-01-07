import { Component, createSignal, For, Match, Switch } from "solid-js";
import { useSubscribe } from "pikav/solid";
import {
  Configuration,
  CreateCommand,
  Group,
  GroupApi,
} from "@timada/cobase-client";

import {
  QueryClient,
  QueryClientProvider,
  createQuery,
  createMutation,
  useQueryClient,
} from "@tanstack/solid-query";

const groupApi = new GroupApi(new Configuration({ basePath: "/api" }));
const queryClient = new QueryClient();

const Groups: Component = () => {
  const queryClient = useQueryClient();
  const [name, setName] = createSignal("");

  const query = createQuery(
    () => ["groups"],
    async () => (await groupApi.findAll()).data
  );

  const mutation = createMutation({
    mutationFn: async (cmd: CreateCommand) => (await groupApi.create(cmd)).data,
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
  return (
    <QueryClientProvider client={queryClient}>
      <Groups />
    </QueryClientProvider>
  );
};

export default App;
