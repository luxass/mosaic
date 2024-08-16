export default defineNitroErrorHandler((error, event) => {
  setResponseHeader(event, "Content-Type", "application/json");

  return send(event, JSON.stringify({
    timestamp: new Date().toISOString(),
    message: error.message,
    status: error.status || 500,
  }));
});
