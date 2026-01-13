"use client";
import React, { useEffect, useState } from "react";

const CSR = () => {
  const [data, setData] = useState(null);
  const [fetchTime, setFetchTime] = useState<number | null>(null);

  useEffect(() => {
    const start = Date.now();
    fetch("/api/data")
      .then((res) => res.json())
      .then((result) => {
        setData(result);
        setFetchTime(Date.now() - start);
      });
  }, []);

  return (
    <>
      <div className="bg-gray-600 p-6">
        <h1>CSR Rendering</h1>
        {!data ? (
          <p>Loading...</p>
        ) : (
          <>
            <pre>{JSON.stringify(data, null, 2)}</pre>
            <p>Client fetch time: {fetchTime} ms</p>
          </>
        )}
      </div>
    </>
  );
};

export default CSR;
