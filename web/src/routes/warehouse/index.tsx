import { Component, createResource, For, Suspense } from "solid-js";
import { Route } from "@solidjs/router";
import NotFoundRoute from "@/components/NotFoundRoute";
import path from "./path";
import DataId from "./[data-id]";
import { useApi } from "@/core/api";

const Index: Component = () => {
  const api = useApi();
  const [listData] = createResource(
    async () => (await api.listWarehousesData()).data
  );

  // useSubscribe<Room>("rooms/+", (event) => {
  //   mutateRooms((old) => old && [...old, event.data]);
  // });

  return (
    <Suspense fallback="Loading...">
      <ul>
        <For each={listData()?.edges}>
          {(data) => <li>{JSON.stringify(data.node.data)}</li>}
        </For>
      </ul>
    </Suspense>
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
