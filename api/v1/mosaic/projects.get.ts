export default defineLazyEventHandler(async () => {
  const runtimeConfig = useRuntimeConfig();
  return defineEventHandler(async (event) => {
    // eslint-disable-next-line no-console
    console.log("event", event);
    return proxyRequest(event, `${runtimeConfig.worker}/repositories`);
  });
});
