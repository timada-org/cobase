import { Component, lazy } from "solid-js";
import { Routes, Route, A } from "@solidjs/router";
import path from "./path";
import NotFoundRoute from "@/components/NotFoundRoute";

const WarehouseRoute = lazy(() => import("./warehouse"));

const Index: Component = () => {
  return <>Home</>;
};

const Root: Component = () => {
  return (
    <>
      <nav>
        <A href={path.index}>Home | </A>
        <A href={path.warehouse.index}>Warehouse</A>
      </nav>
      <Routes>
        <Route path={path.index} component={Index} />
        <WarehouseRoute />
        <NotFoundRoute />
      </Routes>
    </>
  );
};

export default Root;
