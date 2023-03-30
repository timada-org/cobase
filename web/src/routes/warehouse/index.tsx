import { Component, createResource, For } from "solid-js";
import { Route } from "@solidjs/router";
import NotFoundRoute from "@/components/NotFoundRoute";
import path from "./path";
import DataId from "./[data-id]";
import { useApi } from "@/core/api";
import {
  EdgeWarehouseData,
  QueryResultWarehouseData,
} from "@timada/cobase-client";
import { useSubscribe } from "pikav/solid";

const Index: Component = () => {
  const api = useApi();
  const [query, { mutate, refetch }] = createResource<QueryResultWarehouseData>(
    async (_, old) => {
      if (old.value && !old.value.page_info.has_next_page) {
        return old.value;
      }

      const { data } = await api.listWarehousesData(
        40,
        old.value?.page_info.end_cursor as string | undefined
      );

      if (!old.value) {
        return data;
      }

      return { ...data, edges: old.value.edges.concat(data.edges) };
    }
  );

  useSubscribe<EdgeWarehouseData[]>("warehouses/+", (event) => {
    if (event.name !== "data-imported") {
      return;
    }

    mutate((old) => {
      if (!old) {
        return old;
      }

      let edges = Array.from(
        [...old.edges, ...event.data]
          .reduce(
            (acc, o) => acc.set(o.node.id, o),
            new Map<string, EdgeWarehouseData>()
          )
          .values()
      );

      if (old.page_info.has_next_page) {
        edges = edges.slice(0, old.edges.length);
      }

      return { ...old, edges };
    });
  });

  return (
    <>
      <ul>
        <For each={query()?.edges}>
          {(data) => <li>{JSON.stringify(data.node.data)}</li>}
        </For>
      </ul>

      <button disabled={query.loading} onClick={async () => await refetch()}>
        Load more
      </button>
    </>
  );
};

const WarehouseRoute: Component = () => {
  return (
    <Route path="/warehouse">
      <Route path={path.index} component={Index} />
      <Route path={path.dataId()} component={DataId} />
      <NotFoundRoute />
    </Route>
  );
};

export default WarehouseRoute;
