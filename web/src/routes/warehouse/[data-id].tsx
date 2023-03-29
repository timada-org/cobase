import { useParams } from "@solidjs/router";
import { Component } from "solid-js";

const DataId: Component = () => {
  return <>Data {useParams().dataId}</>;
};

export default DataId;
