import { useQuery } from "@tanstack/react-query";
import PageTitle from "../components/PageTitle";
import { useMemo } from "react";
import NotifyDetail from "../components/NotifyDetail";

export default function NotifyListPage() {
  const { data: notificationPagination } = useQuery({
    queryKey: ["notify"],
    queryFn: () =>
      fetch(`/api/notify`, {
        headers: { "Content-Type": "application/json" },
        method: "POST",
        body: JSON.stringify({}),
      })
        .then((res) => res.json())
        .then((data) => data as Pagination<NotificationModel>),
  });

  const notifications = useMemo(
    () => notificationPagination?.data || [],
    [notificationPagination],
  );
  return (
    <>
      <PageTitle>通知列表</PageTitle>
      {notifications.length === 0 && (
        <div className="flex flex-col justify-center items-center h-36">
          没有通知
        </div>
      )}

      {notifications.length > 0 && (
        <ul className="border rounded-xl border-gray-300  my-3 overflow-hidden">
          {notifications.map((n) => (
            <li key={n.id}>
              <NotifyDetail n={n} />
            </li>
          ))}
        </ul>
      )}
    </>
  );
}
