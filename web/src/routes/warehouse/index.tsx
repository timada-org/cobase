import { Component, For } from "solid-js";
import { Route } from "@solidjs/router";
import NotFoundRoute from "@/components/NotFoundRoute";
import path from "./path";
import DataId from "./[data-id]";
import { useApi } from "@/core/api";
import { WarehouseData } from "@timada/cobase-client";
import { createQuery } from "@/utils/query";

const Index: Component = () => {
  const api = useApi();
  const [query, { refetch }] = createQuery<WarehouseData>(
    "warehouses/+",
    (pageInfo) =>
      api.listWarehousesData({
        first: 40,
        after: pageInfo?.end_cursor as string | undefined,
      })
  );

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
