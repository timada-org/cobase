import { createSignal, getOwner, runWithOwner } from "solid-js";

export type Action<T, U> = [
  { value?: U; pending: T[]; state: ActionState },
  // eslint-disable-next-line no-unused-vars
  (vars: T) => Promise<U>
];

export type ActionState = "idle" | "pending";

export function createAction<T, U = void>(
  // eslint-disable-next-line no-unused-vars
  fn: (args: T) => Promise<U>
): Action<T, U> {
  const [pending, setPending] = createSignal<T[]>([]);
  const [value, setValue] = createSignal<U>();
  const owner = getOwner();
  const lookup = new Map();
  let count = 0;
  function mutate(variables: T) {
    const p = fn(variables);
    const reqId = ++count;
    lookup.set(p, variables);
    setPending(Array.from(lookup.values()));
    p.then((data) => {
      lookup.delete(p);
      const v = Array.from(lookup.values());
      setPending(v);
      if (reqId === count) setValue(() => data);
      return data;
    }).catch((err) =>
      runWithOwner(owner, () => {
        throw err;
      })
    );
    return p;
  }
  return [
    {
      get value() {
        return value();
      },
      get pending() {
        return pending();
      },
      get state() {
        return pending().length ? "pending" : "idle";
      },
    },
    mutate,
  ];
}
