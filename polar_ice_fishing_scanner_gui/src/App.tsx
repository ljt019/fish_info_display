import {
  HashRouter as Router,
  Routes as RoutesPrimitive,
  Route,
} from "react-router-dom";
import { Index } from "./screens/index";
import TitleBar from "@/components/titlebar";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";

type Route = {
  path: string;
  element: JSX.Element;
};

const routes: Route[] = [
  {
    path: "/",
    element: <Index />,
  },
];

function Routes({ routes }: { routes: Route[] }) {
  return (
    <RoutesPrimitive>
      {routes.map((route) => (
        <Route
          key={route.path}
          path={route.path}
          element={<Layout>{route.element}</Layout>}
        />
      ))}
    </RoutesPrimitive>
  );
}

function Layout({ children }: { children: JSX.Element }) {
  return (
    <>
      <div className="overflow-hidden h-screen">{children}</div>
    </>
  );
}

const queryClient = new QueryClient();

function App() {
  return (
    <Router>
      <QueryClientProvider client={queryClient}>
        <Routes routes={routes} />
      </QueryClientProvider>
    </Router>
  );
}

export default App;
