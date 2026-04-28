import { useParams } from "react-router-dom";
import PageTitle from "../components/PageTitle";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { useEffect } from "react";
import NotifyDetail from "../components/NotifyDetail";

export default function NotifyDetailPage() {
  const { id } = useParams();
  const queryClient = useQueryClient();
  const { data: notification } = useQuery({
    queryKey: ["notify", id],
    queryFn: () =>
      fetch(`/api/notify/${id}`, {
        headers: {
          "content-type": "application/json",
        },
        method: "GET",
      })
        .then((resp) => resp.json())
        .then((data) => data as Notification),
  });

  const { mutate: markReadMutation } = useMutation({
    mutationKey: ["notify-make-read", id],
    mutationFn: () =>
      fetch(`/api/notify/${id}`, {
        headers: {
          "content-type": "application/json",
        },
        method: "PUT",
      })
        .then((resp) => resp.json())
        .then((data) => data as number),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["notify", id] });
    },
  });

  useEffect(() => {
    if (notification) {
      markReadMutation();
    }
  }, [notification]);
  return (
    <>
      <PageTitle>通知详情</PageTitle>

      {notification && (
        <div className="my-3">
          <NotifyDetail n={notification} showContent />
        </div>
      )}
    </>
  );
}
