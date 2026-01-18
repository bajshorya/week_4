export async function GET() {
  const start = Date.now();
  await new Promise((r) => setTimeout(r, 3000));
  return Response.json({
    message: "response",
    timestamp: new Date().toISOString(),
    executionTime: Date.now() - start,
  });
}
