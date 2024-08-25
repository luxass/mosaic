export default defineLazyEventHandler(async () => {
  const runtimeConfig = useRuntimeConfig();
  return defineEventHandler(async (event) => {
    return proxyRequest(event, `${runtimeConfig.worker}/repositories`);
  });
});
