import dayjs from "dayjs";
import relativeTime from "dayjs/plugin/relativeTime";
import { Link } from "react-router-dom";
dayjs.extend(relativeTime);

export default function NotifyDetail({
  n,
  showContent,
}: {
  n: NotificationModel;
  showContent?: boolean;
}) {
  return (
    <div className="relative w-full space-y-1 hover:bg-gray-50 p-3 last:border-0 border-b border-gray-300">
      <div className="flex justify-start items-center gap-x-4">
        {n.is_read === false && (
          <div className="shrink-0 w-2">
            <div className="w-2 h-2 p-px bg-purple-600 rounded-full"></div>
          </div>
        )}
        <div className="grow">
          <span className="text-[10px] bg-purple-600 text-purple-100 px-1 py-0.5 rounded">
            {n.kind}
          </span>
        </div>
      </div>
      <div className="flex justify-start items-center gap-x-4">
        <div className="shrink-0 w-2"></div>
        <div className="grow space-y-1">
          <div className="">
            {showContent ? (
              <>{n.title}</>
            ) : (
              <Link to={`/notify/${n.id}`} className="text-blue-500">
                {n.title}
              </Link>
            )}
          </div>
          {showContent && (
            <div className="truncate text-slate-600 text-sm">{n.content}</div>
          )}
          <div className="text-slate-600 text-xs">
            {dayjs(n.created_at).fromNow()}
          </div>
        </div>
      </div>
    </div>
  );
}
