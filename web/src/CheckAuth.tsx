import { Configuration, V0alpha2Api } from "@ory/kratos-client";
import { ParentComponent, createEffect, createResource, Show } from "solid-js";

const kratos = new V0alpha2Api(new Configuration({ basePath: "/kratos" }));

const removeTrailingSlash = (s: string) => s.replace(/\/$/, "");
const getUrlForFlow = (base: string, flow: string, query?: URLSearchParams) =>
  `${removeTrailingSlash(base)}/self-service/${flow}/browser${
    query ? `?${query.toString()}` : ""
  }`;

const CheckAuth: ParentComponent = (props) => {
  const [session] = createResource(() => kratos.toSession());

  createEffect(() => {
    if (!session.error || session.error.response.status !== 401) {
      return;
    }

    const initFlowUrl = getUrlForFlow(
      "/kratos",
      "login",
      new URLSearchParams({
        return_to: location.href,
      })
    );

    location.href = initFlowUrl;
  });

  return (
    <Show
      when={!session.loading && !session.error}
      fallback={<>Checking auth...</>}
    >
      {props.children}
    </Show>
  );
};

export default CheckAuth;
