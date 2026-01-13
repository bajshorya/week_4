import React from "react";

export default async function SSR() {
  const data = await fetch("http://localhost:3000/api/data").then((res) =>
    res.json()
  );

  return (
    <div>
      <h1>SSG Page</h1>
      <p>Render type: Static Site Generation</p>
      <pre>{JSON.stringify(data, null, 2)}</pre>
      <p>Built at build time</p>
    </div>
  );
}
