export function warehousePath(basePath: string = "/") {
  return {
    index: basePath,
    dataId: (id: string = ":dataId") => `${basePath}${id}`,
  };
}

export default warehousePath();
