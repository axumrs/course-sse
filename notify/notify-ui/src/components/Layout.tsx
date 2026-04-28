import { Link, Outlet } from "react-router-dom";
import Container from "./Container";
import { Bell as NotifyIcon } from "lucide-react";
import { useEffect, useMemo, useState } from "react";

export default function Layout() {
  const [_showNotify, setShowNotify] = useState(false);
  const [unreadCounter, setUnreadCounter] = useState(0);

  useEffect(() => {
    const sse = new EventSource("/sse/unread-notify-counter");
    sse.onmessage = (event: MessageEvent<number>) => {
      if (event.data) {
        setUnreadCounter(event.data);
      }
    };

    return () => {
      sse.close();
    };
  }, []);

  const hasUnreadNotify = useMemo(() => unreadCounter > 0, [unreadCounter]);

  return (
    <div
      onClick={() => {
        setShowNotify(false);
      }}
      className="h-full"
    >
      <header className="shadow">
        <Container className="flex justify-between items-center gap-x-4 p-3">
          <Link to="/" className="flex items-center gap-x-2">
            <img src="/favicon.svg" alt="logo" className="w-8 object-convert" />
            <h2 className="text-2xl">站内通知</h2>
          </Link>
          <div className="relative">
            <div className="relative">
              {/* <button
                type="button"
                onClick={(e) => {
                  e.stopPropagation();
                  setShowNotify((v) => !v);
                }}
              >
                <NotifyIcon size={24} />
              </button> */}
              <Link to="/notify">
                <NotifyIcon size={24} />
              </Link>
              {hasUnreadNotify && (
                <div className="absolute right-0 top-0  z-1">
                  <div className=" w-1 h-1 bg-red-800 rounded-full animate-ping"></div>
                </div>
              )}
            </div>

            {/* <div
              className={`absolute z-1 w-72 bg-white  rounded-md shadow -right-10  border border-gray-200 transition-all ${
                showNotify ? "opacity-100 visible" : "opacity-0 invisible"
              }`}
            >
              <div className="flex justify-between items-center gap-x-4 p-3">
                <div>通知</div>
                <div>
                  <button className="text-xs text-blue-600">全部已读</button>
                </div>
              </div>
              <div className="h-36 flex justify-center items-center border-y border-gray-200">
                没有新通知
              </div>
              <ul className="border-y border-gray-200 max-h-72 overflow-y-auto">
                <li className="relative w-full space-y-1 hover:bg-gray-50 p-3">
                  <div className="grid grid-cols-12 items-center">
                    <div className="col-span-1">
                      <div className="w-2 h-2 p-px bg-blue-500 rounded-full"></div>
                    </div>
                    <div className="col-span-11">
                      <span className="text-[10px] bg-blue-500 text-blue-100 px-1 py-0.5 rounded">
                        标签
                      </span>
                    </div>
                  </div>
                  <div className="grid grid-cols-12 items-center">
                    <div className="col-span-1"></div>
                    <div className=" col-span-11 space-y-1">
                      <div className="">🚀 实例创建成功</div>
                      <div className="truncate text-slate-600 text-sm">
                        您的实例「adfja」已创建成功！节点： 237.84.2.178
                      </div>
                      <div className="text-slate-600 text-xs">5分钟前</div>
                    </div>
                  </div>
                </li>
                <li className="relative w-full space-y-1 hover:bg-gray-50 p-3">
                  <div className="grid grid-cols-12 items-center">
                    <div className="col-span-1">
                      <div className="w-2 h-2 p-px bg-blue-500 rounded-full"></div>
                    </div>
                    <div className="col-span-11">
                      <span className="text-[10px] bg-blue-500 text-blue-100 px-1 py-0.5 rounded">
                        标签
                      </span>
                    </div>
                  </div>
                  <div className="grid grid-cols-12 items-center">
                    <div className="col-span-1"></div>
                    <div className=" col-span-11 space-y-1">
                      <div className="">🚀 实例创建成功</div>
                      <div className="truncate text-slate-600 text-sm">
                        您的实例「adfja」已创建成功！节点： 237.84.2.178
                      </div>
                      <div className="text-slate-600 text-xs">5分钟前</div>
                    </div>
                  </div>
                </li>
                <li className="relative w-full space-y-1 hover:bg-gray-50 p-3">
                  <div className="grid grid-cols-12 items-center">
                    <div className="col-span-1">
                      <div className="w-2 h-2 p-px bg-blue-500 rounded-full"></div>
                    </div>
                    <div className="col-span-11">
                      <span className="text-[10px] bg-blue-500 text-blue-100 px-1 py-0.5 rounded">
                        标签
                      </span>
                    </div>
                  </div>
                  <div className="grid grid-cols-12 items-center">
                    <div className="col-span-1"></div>
                    <div className=" col-span-11 space-y-1">
                      <div className="">🚀 实例创建成功</div>
                      <div className="truncate text-slate-600 text-sm">
                        您的实例「adfja」已创建成功！节点： 237.84.2.178
                      </div>
                      <div className="text-slate-600 text-xs">5分钟前</div>
                    </div>
                  </div>
                </li>
                <li className="relative w-full space-y-1 hover:bg-gray-50 p-3">
                  <div className="grid grid-cols-12 items-center">
                    <div className="col-span-1">
                      <div className="w-2 h-2 p-px bg-blue-500 rounded-full"></div>
                    </div>
                    <div className="col-span-11">
                      <span className="text-[10px] bg-blue-500 text-blue-100 px-1 py-0.5 rounded">
                        标签
                      </span>
                    </div>
                  </div>
                  <div className="grid grid-cols-12 items-center">
                    <div className="col-span-1"></div>
                    <div className=" col-span-11 space-y-1">
                      <div className="">🚀 实例创建成功</div>
                      <div className="truncate text-slate-600 text-sm">
                        您的实例「adfja」已创建成功！节点： 237.84.2.178
                      </div>
                      <div className="text-slate-600 text-xs">5分钟前</div>
                    </div>
                  </div>
                </li>
                <li className="relative w-full space-y-1 hover:bg-gray-50 p-3">
                  <div className="grid grid-cols-12 items-center">
                    <div className="col-span-1">
                      <div className="w-2 h-2 p-px bg-blue-500 rounded-full"></div>
                    </div>
                    <div className="col-span-11">
                      <span className="text-[10px] bg-blue-500 text-blue-100 px-1 py-0.5 rounded">
                        标签
                      </span>
                    </div>
                  </div>
                  <div className="grid grid-cols-12 items-center">
                    <div className="col-span-1"></div>
                    <div className=" col-span-11 space-y-1">
                      <div className="">🚀 实例创建成功</div>
                      <div className="truncate text-slate-600 text-sm">
                        您的实例「adfja」已创建成功！节点： 237.84.2.178
                      </div>
                      <div className="text-slate-600 text-xs">5分钟前</div>
                    </div>
                  </div>
                </li>
                <li className="relative w-full space-y-1 hover:bg-gray-50 p-3">
                  <div className="grid grid-cols-12 items-center">
                    <div className="col-span-1">
                      <div className="w-2 h-2 p-px bg-blue-500 rounded-full"></div>
                    </div>
                    <div className="col-span-11">
                      <span className="text-[10px] bg-blue-500 text-blue-100 px-1 py-0.5 rounded">
                        标签
                      </span>
                    </div>
                  </div>
                  <div className="grid grid-cols-12 items-center">
                    <div className="col-span-1"></div>
                    <div className=" col-span-11 space-y-1">
                      <div className="">🚀 实例创建成功</div>
                      <div className="truncate text-slate-600 text-sm">
                        您的实例「adfja」已创建成功！节点： 237.84.2.178
                      </div>
                      <div className="text-slate-600 text-xs">5分钟前</div>
                    </div>
                  </div>
                </li>
                <li className="relative w-full space-y-1 hover:bg-gray-50 p-3">
                  <div className="grid grid-cols-12 items-center">
                    <div className="col-span-1">
                      <div className="w-2 h-2 p-px bg-blue-500 rounded-full"></div>
                    </div>
                    <div className="col-span-11">
                      <span className="text-[10px] bg-blue-500 text-blue-100 px-1 py-0.5 rounded">
                        标签
                      </span>
                    </div>
                  </div>
                  <div className="grid grid-cols-12 items-center">
                    <div className="col-span-1"></div>
                    <div className=" col-span-11 space-y-1">
                      <div className="">🚀 实例创建成功</div>
                      <div className="truncate text-slate-600 text-sm">
                        您的实例「adfja」已创建成功！节点： 237.84.2.178
                      </div>
                      <div className="text-slate-600 text-xs">5分钟前</div>
                    </div>
                  </div>
                </li>
                <li className="relative w-full space-y-1 hover:bg-gray-50 p-3">
                  <div className="grid grid-cols-12 items-center">
                    <div className="col-span-1">
                      <div className="w-2 h-2 p-px bg-blue-500 rounded-full"></div>
                    </div>
                    <div className="col-span-11">
                      <span className="text-[10px] bg-blue-500 text-blue-100 px-1 py-0.5 rounded">
                        标签
                      </span>
                    </div>
                  </div>
                  <div className="grid grid-cols-12 items-center">
                    <div className="col-span-1"></div>
                    <div className=" col-span-11 space-y-1">
                      <div className="">🚀 实例创建成功</div>
                      <div className="truncate text-slate-600 text-sm">
                        您的实例「adfja」已创建成功！节点： 237.84.2.178
                      </div>
                      <div className="text-slate-600 text-xs">5分钟前</div>
                    </div>
                  </div>
                </li>
                <li className="relative w-full space-y-1 hover:bg-gray-50 p-3">
                  <div className="grid grid-cols-12 items-center">
                    <div className="col-span-1">
                      <div className="w-2 h-2 p-px bg-blue-500 rounded-full"></div>
                    </div>
                    <div className="col-span-11">
                      <span className="text-[10px] bg-blue-500 text-blue-100 px-1 py-0.5 rounded">
                        标签
                      </span>
                    </div>
                  </div>
                  <div className="grid grid-cols-12 items-center">
                    <div className="col-span-1"></div>
                    <div className=" col-span-11 space-y-1">
                      <div className="">🚀 实例创建成功</div>
                      <div className="truncate text-slate-600 text-sm">
                        您的实例「adfja」已创建成功！节点： 237.84.2.178
                      </div>
                      <div className="text-slate-600 text-xs">5分钟前</div>
                    </div>
                  </div>
                </li>
                <li className="relative w-full space-y-1 hover:bg-gray-50 p-3">
                  <div className="grid grid-cols-12 items-center">
                    <div className="col-span-1">
                      <div className="w-2 h-2 p-px bg-blue-500 rounded-full"></div>
                    </div>
                    <div className="col-span-11">
                      <span className="text-[10px] bg-blue-500 text-blue-100 px-1 py-0.5 rounded">
                        标签
                      </span>
                    </div>
                  </div>
                  <div className="grid grid-cols-12 items-center">
                    <div className="col-span-1"></div>
                    <div className=" col-span-11 space-y-1">
                      <div className="">🚀 实例创建成功</div>
                      <div className="truncate text-slate-600 text-sm">
                        您的实例「adfja」已创建成功！节点： 237.84.2.178
                      </div>
                      <div className="text-slate-600 text-xs">5分钟前</div>
                    </div>
                  </div>
                </li>
                <li className="relative w-full space-y-1 hover:bg-gray-50 p-3">
                  <div className="grid grid-cols-12 items-center">
                    <div className="col-span-1">
                      <div className="w-2 h-2 p-px bg-blue-500 rounded-full"></div>
                    </div>
                    <div className="col-span-11">
                      <span className="text-[10px] bg-blue-500 text-blue-100 px-1 py-0.5 rounded">
                        标签
                      </span>
                    </div>
                  </div>
                  <div className="grid grid-cols-12 items-center">
                    <div className="col-span-1"></div>
                    <div className=" col-span-11 space-y-1">
                      <div className="">🚀 实例创建成功</div>
                      <div className="truncate text-slate-600 text-sm">
                        您的实例「adfja」已创建成功！节点： 237.84.2.178
                      </div>
                      <div className="text-slate-600 text-xs">5分钟前</div>
                    </div>
                  </div>
                </li>
              </ul>
              <div className="p-3 flex justify-center">
                <Link to="/notify" className=" text-blue-600">
                  查看全部
                </Link>
              </div>
            </div> */}
          </div>
        </Container>
      </header>
      <main>
        <Container className=" p-3">
          <Outlet />
        </Container>
      </main>
    </div>
  );
}
