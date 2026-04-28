import { createHashRouter } from "react-router-dom";
import { lazy } from "react";

const Home = lazy(() => import("./pages/HomePage"));
const NotifyListPage = lazy(() => import("./pages/NotifyListPage"));
const NotifyDetailPage = lazy(() => import("./pages/NotifyDetailPage"));
const Layout = lazy(() => import("./components/Layout"));

export const router = createHashRouter([
  {
    path: "/",
    element: <Layout />,
    children: [
      { index: true, element: <Home /> },
      { path: "notify", element: <NotifyListPage /> },
      { path: "notify/:id", element: <NotifyDetailPage /> },
    ],
  },
]);

export default router;
