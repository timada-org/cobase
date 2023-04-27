import { useSubscribe } from "pikav/solid";
import { ResourceReturn, createResource } from "solid-js";

export interface Response<T = any> {
  data: T;
}

export interface PageInfo {
  /**
   *
   * @type {string}
   * @memberof PageInfo
   */
  end_cursor?: string | null;
  /**
   *
   * @type {boolean}
   * @memberof PageInfo
   */
  has_next_page: boolean;
  /**
   *
   * @type {boolean}
   * @memberof PageInfo
   */
  has_previous_page: boolean;
  /**
   *
   * @type {string}
   * @memberof PageInfo
   */
  start_cursor?: string | null;
}

export interface QueryResult<N extends Node> {
  /**
   *
   * @type {Array<T>}
   * @memberof QueryResult
   */
  edges: Array<Edge<N>>;
  /**
   *
   * @type {PageInfo}
   * @memberof QueryResult
   */
  page_info: PageInfo;
}

export interface Edge<N extends Node> {
  /**
   *
   * @type {string}
   * @memberof Edge
   */
  cursor: string;
  /**
   *
   * @type {WarehouseData}
   * @memberof Edge
   */
  node: N;
}

export interface Node {
  /**
   *
   * @type {string}
   * @memberof Node
   */
  id: string;
}

export type ResourceFetcher<N extends Node, T extends QueryResult<N>> = (
  // eslint-disable-next-line no-unused-vars
  info?: PageInfo
) => Promise<Response<T>>;

export function createQuery<N extends Node, R = unknown>(
  topic: string,
  fetcher: ResourceFetcher<N, QueryResult<N>>
): ResourceReturn<QueryResult<N>, R> {
  const resource = createResource<QueryResult<N>, R>(async (_, old) => {
    if (old.value && !old.value.page_info.has_next_page) {
      return old.value;
    }

    const { data } = await fetcher(old.value?.page_info);

    if (!old.value) {
      return data;
    }

    return { ...data, edges: old.value.edges.concat(data.edges) };
  });

  useSubscribe<Edge<N> | Edge<N>[]>(topic, (event) => {
    if (!["added", "updated", "removed"].includes(event.name)) {
      return;
    }

    resource[1].mutate((old) => {
      if (!old) {
        return old;
      }

      if (event.name === "removed") {
        return {
          ...old,
          edges: old.edges.filter((edge) =>
            Array.isArray(event.data)
              ? !event.data.find((e) => e.node.id === edge.node.id)
              : event.data.node.id !== edge.node.id
          ),
        };
      }

      let edges = Array.from(
        (Array.isArray(event.data)
          ? [...old.edges, ...event.data]
          : [...old.edges, event.data]
        )
          .reduce((acc, o) => acc.set(o.node.id, o), new Map<string, Edge<N>>())
          .values()
      );

      if (old.page_info.has_next_page) {
        edges = edges.slice(0, old.edges.length);
      }

      return { ...old, edges };
    });
  });

  return resource;
}
