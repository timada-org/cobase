import { Component, For } from "solid-js";
import { Route } from "@solidjs/router";
import NotFoundRoute from "@/components/NotFoundRoute";
import path from "./path";
import DataId from "./[data-id]";
import { useApi } from "@/core/api";
import { WarehouseData } from "@timada/cobase-client";
import { createQuery } from "@/utils/query";
import * as Papaparse from "papaparse";

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
      <input
        type="file"
        accept=".csv"
        onChange={(e) => {
          let [csvFile] = e.currentTarget?.files || [];

          if (!csvFile) {
            return;
          }

          let i = 0;
          Papaparse.parse<object>(csvFile, {
            header: true,
            chunkSize: 524288,
            chunk(result) {
              setTimeout(() => {
                api
                  .importData({
                    importDataWarehouseInput: { data: result.data },
                  })
                  .then(() => {});
              }, 1000 * i++);
            },
            complete() {
              console.log("compelted");
            },
          });
        }}
      />
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
