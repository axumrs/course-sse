import React from "react";

export default function PageTitle({
  children,
  className = "",
}: {
  children: React.ReactNode;
  className?: string;
}) {
  return <div className={`text-xl ${className}`}>{children}</div>;
}
