import { Component } from "solid-js";
import { Route } from "@solidjs/router";

const NotFound: Component = () => {
  return <>Not found</>;
};

const NotFoundRoute: Component = () => {
  return <Route path="*" component={NotFound} />;
};

export default NotFoundRoute;
