import { useEffect, useState } from "react";

export default function App() {
  const [dateList, setDateList] = useState<string[]>([]);
  useEffect(() => {
    const eventSource = new EventSource("/api/sse");
    eventSource.onmessage = (event: MessageEvent<string>) => {
      setDateList((list) => [event.data, ...list]);
      console.log(event.data);
    };
    return () => eventSource.close();
  }, []);
  return (
    <div className="container mx-auto p-3 my-3 space-y-6">
      <h1 className="text-3xl">SSE：服务器推送时间</h1>
      <div className="outline-0 ring ring-gray-400 rounded w-xl h-72 overflow-y-auto px-2 py-1 font-mono">
        {dateList
          .map((d) => new Date(d))
          .map((date, idx) => (
            <div key={idx}>{date.toLocaleString()}</div>
          ))}
      </div>
    </div>
  );
}
