import { CornerRightUp as NotifyTipIcon } from "lucide-react";
export default function HomePage() {
  return (
    <div className="flex justify-end items-center gap-x-3">
      当有新通知时，这个小铃铛将会标红
      <NotifyTipIcon />
    </div>
  );
}
