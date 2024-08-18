export default defineLazyEventHandler(async () => {
  const runtimeConfig = useRuntimeConfig();
  return defineEventHandler(async (event) => {
    return sendRedirect(event, `${runtimeConfig.worker}/repositories`);
  });
});
