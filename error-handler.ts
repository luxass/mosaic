export default defineNitroErrorHandler((error, event) => {
  setResponseHeader(event, "Content-Type", "application/json");

  return send(event, JSON.stringify({
    path: event.path,
    timestamp: new Date().toISOString(),
    message: error.message,
    status: error.statusCode || 500,
  }));
});
